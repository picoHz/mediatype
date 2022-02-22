use super::{error::*, media_type_buf::*, name::*, parse::*, value::*};
use std::{
    borrow::Cow,
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    mem,
};

/// A borrowed MediaType.
///
/// ```
/// use mediatype::{names::*, MediaType, Value};
///
/// let mut multipart = MediaType::new(MULTIPART, FORM_DATA);
///
/// let boundary = Value::new("dyEV84n7XNJ").unwrap();
/// multipart.set_param(BOUNDARY, boundary);
/// assert_eq!(
///     multipart.to_string(),
///     "multipart/form-data; boundary=dyEV84n7XNJ"
/// );
///
/// multipart.subty = RELATED;
/// assert_eq!(
///     multipart.to_string(),
///     "multipart/related; boundary=dyEV84n7XNJ"
/// );
///
/// const IMAGE_SVG: MediaType = MediaType::from_parts(IMAGE, SVG, Some(XML), None);
/// let svg = MediaType::parse("IMAGE/SVG+XML").unwrap();
/// assert_eq!(svg, IMAGE_SVG);
/// ```
#[derive(Debug, Clone)]
pub struct MediaType<'a> {
    /// Top-level type.
    pub ty: Name<'a>,

    /// Subtype.
    pub subty: Name<'a>,

    /// Optional suffix.
    pub suffix: Option<Name<'a>>,

    params: Cow<'a, [(Name<'a>, Value<'a>)]>,
}

impl<'a> MediaType<'a> {
    /// Constructs a `MediaType` from a top-level type and a subtype.
    /// ```
    /// # use mediatype::{names::*, MediaType};
    /// const IMAGE_PNG: MediaType = MediaType::new(IMAGE, PNG);
    /// assert_eq!(IMAGE_PNG, MediaType::parse("image/png").unwrap());
    /// ```
    pub const fn new(ty: Name<'a>, subty: Name<'a>) -> Self {
        Self {
            ty,
            subty,
            suffix: None,
            params: Cow::Borrowed(&[]),
        }
    }

    /// Constructs a `MediaType` with an optional suffix and parameters.
    ///
    /// This function accepts only one parameter
    /// because key duplication is not detectable at compile-time.
    ///
    /// ```
    /// # use mediatype::{names::*, values::*, MediaType};
    /// const IMAGE_SVG: MediaType =
    ///     MediaType::from_parts(IMAGE, SVG, Some(XML), Some(&[(CHARSET, UTF_8)]));
    /// assert_eq!(
    ///     IMAGE_SVG,
    ///     MediaType::parse("image/svg+xml; charset=UTF-8").unwrap()
    /// );
    /// ```
    pub const fn from_parts(
        ty: Name<'a>,
        subty: Name<'a>,
        suffix: Option<Name<'a>>,
        param: Option<&'a [(Name<'a>, Value<'a>); 1]>,
    ) -> Self {
        let params: &[(Name, Value)] = if let Some(param) = param { param } else { &[] };
        Self {
            ty,
            subty,
            suffix,
            params: Cow::Borrowed(params),
        }
    }

    /// Constructs a `MediaType` from `str` without copying the string.
    pub fn parse<'s: 'a>(s: &'s str) -> Result<Self, MediaTypeError> {
        let (indices, _) = Indices::parse(s)?;
        let params = indices
            .params()
            .iter()
            .map(|param| {
                (
                    Name::new_unchecked(&s[param[0] as usize..param[1] as usize]),
                    Value::new_unchecked(&s[param[2] as usize..param[3] as usize]),
                )
            })
            .collect();
        Ok(Self {
            ty: Name::new_unchecked(&s[indices.ty()]),
            subty: Name::new_unchecked(&s[indices.subty()]),
            suffix: indices.suffix().map(|range| Name::new_unchecked(&s[range])),
            params: Cow::Owned(params),
        })
    }

    /// Returns the parameters.
    ///
    /// The parameters are alphabetically sorted by their keys.
    pub fn params(&self) -> &[(Name, Value)] {
        &self.params
    }

    /// Gets the parameter value by its key.
    pub fn get_param(&self, key: Name) -> Option<Value> {
        self.params
            .binary_search_by_key(&key, |(key, _)| *key)
            .ok()
            .map(|index| self.params[index].1)
    }

    /// Sets a parameter value.
    ///
    /// If the parameter is already set, replaces it with a new value and
    /// returns the old value.
    pub fn set_param<'k: 'a, 'v: 'a>(&mut self, key: Name<'k>, value: Value<'v>) -> Option<Value> {
        if let Ok(index) = self
            .params
            .binary_search_by_key(&Name::new_unchecked(key.as_str()), |(key, _)| *key)
        {
            Some(mem::replace(&mut self.params.to_mut()[index].1, value))
        } else {
            let params = self.params.to_mut();
            params.push((key, value));
            params.sort_unstable_by_key(|&(key, _)| key);
            None
        }
    }

    /// Removes and returns a parameter value by its key.
    pub fn remove_param(&mut self, key: Name) -> Option<Value> {
        self.params
            .binary_search_by_key(&key, |(key, _)| *key)
            .ok()
            .map(|index| self.params.to_mut().remove(index).1)
    }

    /// Removes all parameters.
    pub fn clear_params(&mut self) {
        if !self.params.is_empty() {
            self.params.to_mut().clear();
        }
    }
}

impl<'a> fmt::Display for MediaType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.ty, self.subty)?;
        if let Some(suffix) = self.suffix {
            write!(f, "+{}", suffix)?;
        }
        for (key, value) in &*self.params {
            write!(f, "; {}={}", key, value)?;
        }
        Ok(())
    }
}

impl<'a> From<&'a MediaTypeBuf> for MediaType<'a> {
    fn from(t: &'a MediaTypeBuf) -> Self {
        t.to_ref()
    }
}

impl<'a> PartialEq for MediaType<'a> {
    fn eq(&self, other: &MediaType) -> bool {
        self.ty == other.ty
            && self.subty == other.subty
            && self.suffix == other.suffix
            && self.params().eq(other.params())
    }
}

impl<'a> Eq for MediaType<'a> {}

impl<'a> PartialOrd for MediaType<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for MediaType<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.ty.cmp(&other.ty) {
            Ordering::Equal => (),
            ne => return ne,
        }
        match self.subty.cmp(&other.subty) {
            Ordering::Equal => (),
            ne => return ne,
        }
        match self.suffix.cmp(&other.suffix) {
            Ordering::Equal => (),
            ne => return ne,
        }
        self.params().cmp(other.params())
    }
}

impl<'a> Hash for MediaType<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ty.hash(state);
        self.subty.hash(state);
        self.suffix.hash(state);
        for param in self.params() {
            param.hash(state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{names::*, values::*};

    #[test]
    fn to_string() {
        assert_eq!(MediaType::new(TEXT, PLAIN).to_string(), "text/plain");
        assert_eq!(
            MediaType::from_parts(IMAGE, SVG, Some(XML), None).to_string(),
            "image/svg+xml"
        );
        assert_eq!(
            MediaType::from_parts(TEXT, PLAIN, None, Some(&[(CHARSET, UTF_8)])).to_string(),
            "text/plain; charset=UTF-8"
        );
        assert_eq!(
            MediaType::from_parts(IMAGE, SVG, Some(XML), Some(&[(CHARSET, UTF_8)])).to_string(),
            "image/svg+xml; charset=UTF-8"
        );
    }

    #[test]
    fn get_param() {
        assert_eq!(MediaType::new(TEXT, PLAIN).get_param(CHARSET), None);
        assert_eq!(
            MediaType::from_parts(TEXT, PLAIN, None, Some(&[(CHARSET, UTF_8)])).get_param(CHARSET),
            Some(UTF_8)
        );
        assert_eq!(
            MediaType::parse("image/svg+xml; charset=UTF-8; HELLO=WORLD")
                .unwrap()
                .get_param(Name::new("hello").unwrap()),
            Some(Value::new("WORLD").unwrap())
        );
    }

    #[test]
    fn set_param() {
        let mut media_type = MediaType::from_parts(TEXT, PLAIN, None, Some(&[(CHARSET, UTF_8)]));
        let lower_utf8 = Value::new("utf-8").unwrap();
        assert_eq!(media_type.set_param(CHARSET, lower_utf8), Some(UTF_8));
        assert_eq!(media_type.to_string(), "text/plain; charset=utf-8");

        let alice = Name::new("ALICE").unwrap();
        let bob = Value::new("bob").unwrap();
        assert_eq!(media_type.set_param(alice, bob), None);
        assert_eq!(
            media_type.to_string(),
            "text/plain; ALICE=bob; charset=utf-8"
        );
    }

    #[test]
    fn remove_param() {
        assert_eq!(MediaType::new(TEXT, PLAIN).remove_param(CHARSET), None);

        let mut media_type = MediaType::from_parts(TEXT, PLAIN, None, Some(&[(CHARSET, UTF_8)]));
        assert_eq!(media_type.remove_param(CHARSET), Some(UTF_8));
        assert_eq!(media_type.remove_param(CHARSET), None);
        assert_eq!(media_type.to_string(), "text/plain");

        let mut media_type = MediaType::parse("image/svg+xml; charset=UTF-8; HELLO=WORLD").unwrap();
        assert_eq!(
            media_type.remove_param(Name::new("hello").unwrap()),
            Some(Value::new("WORLD").unwrap())
        );
        assert_eq!(media_type.remove_param(Name::new("hello").unwrap()), None);
        assert_eq!(media_type.to_string(), "image/svg+xml; charset=UTF-8");
    }

    #[test]
    fn clear_params() {
        let mut media_type = MediaType::parse("image/svg+xml; charset=UTF-8; HELLO=WORLD").unwrap();
        media_type.clear_params();
        assert_eq!(media_type.to_string(), "image/svg+xml");
    }

    #[test]
    fn cmp() {
        assert_eq!(
            MediaType::parse("text/plain").unwrap(),
            MediaType::parse("TEXT/PLAIN").unwrap()
        );
        assert_eq!(
            MediaType::parse("image/svg+xml; charset=UTF-8").unwrap(),
            MediaType::parse("IMAGE/SVG+XML; CHARSET=UTF-8").unwrap()
        );
        assert_eq!(
            MediaType::parse("image/svg+xml; hello=WORLD; charset=UTF-8").unwrap(),
            MediaType::parse("IMAGE/SVG+XML; CHARSET=UTF-8; HELLO=WORLD").unwrap()
        );
    }
}
