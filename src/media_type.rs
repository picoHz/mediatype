use super::{error::*, media_type_buf::*, name::*, params::*, parse::*, value::*};
use std::{
    borrow::Cow,
    collections::BTreeMap,
    fmt,
    hash::{Hash, Hasher},
};

/// A borrowed media type.
///
/// ```
/// use mediatype::{names::*, MediaType, Value, WriteParams};
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
/// const IMAGE_SVG: MediaType = MediaType::from_parts(IMAGE, SVG, Some(XML), &[]);
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

    /// Parameters.
    pub params: Cow<'a, [(Name<'a>, Value<'a>)]>,
}

impl<'a> MediaType<'a> {
    /// Constructs a `MediaType` from a top-level type and a subtype.
    /// ```
    /// # use mediatype::{names::*, MediaType};
    /// const IMAGE_PNG: MediaType = MediaType::new(IMAGE, PNG);
    /// assert_eq!(IMAGE_PNG, MediaType::parse("image/png").unwrap());
    /// ```
    #[must_use]
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
    /// ```
    /// # use mediatype::{names::*, values::*, MediaType};
    /// const IMAGE_SVG: MediaType = MediaType::from_parts(IMAGE, SVG, Some(XML), &[(CHARSET, UTF_8)]);
    /// assert_eq!(
    ///     IMAGE_SVG,
    ///     MediaType::parse("image/svg+xml; charset=UTF-8").unwrap()
    /// );
    /// ```
    #[must_use]
    pub const fn from_parts(
        ty: Name<'a>,
        subty: Name<'a>,
        suffix: Option<Name<'a>>,
        params: &'a [(Name<'a>, Value<'a>)],
    ) -> Self {
        Self {
            ty,
            subty,
            suffix,
            params: Cow::Borrowed(params),
        }
    }

    pub(crate) const fn from_parts_unchecked(
        ty: Name<'a>,
        subty: Name<'a>,
        suffix: Option<Name<'a>>,
        params: Cow<'a, [(Name<'a>, Value<'a>)]>,
    ) -> Self {
        Self {
            ty,
            subty,
            suffix,
            params,
        }
    }

    /// Constructs a `MediaType` from `str` without copying the string.
    ///
    /// # Errors
    ///
    /// Returns an error if the string fails to be parsed.
    pub fn parse<'s: 'a>(s: &'s str) -> Result<Self, MediaTypeError> {
        let (indices, _) = Indices::parse(s)?;
        let params = indices
            .params()
            .iter()
            .map(|param| {
                (
                    Name::new_unchecked(&s[param[0]..param[1]]),
                    Value::new_unchecked(&s[param[2]..param[3]]),
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

    /// Returns a [`MediaType`] without parameters.
    ///
    /// ```
    /// # use mediatype::{names::*, values::*, MediaType};
    /// const IMAGE_SVG: MediaType = MediaType::from_parts(IMAGE, SVG, Some(XML), &[(CHARSET, UTF_8)]);
    /// assert_eq!(
    ///     IMAGE_SVG.essence(),
    ///     MediaType::parse("image/svg+xml").unwrap()
    /// );
    /// ```
    ///
    /// [`MadiaType`]: ./struct.MediaType.html
    #[must_use]
    pub const fn essence(&self) -> MediaType<'_> {
        MediaType::from_parts(self.ty, self.subty, self.suffix, &[])
    }
}

impl<'a> ReadParams for MediaType<'a> {
    fn params(&self) -> Params {
        Params::from_slice(&self.params)
    }

    fn get_param(&self, name: Name) -> Option<Value> {
        self.params
            .iter()
            .rev()
            .find(|&&param| name == param.0)
            .map(|&(_, value)| value)
    }
}

impl<'a> WriteParams<'a> for MediaType<'a> {
    fn set_param<'n: 'a, 'v: 'a>(&mut self, name: Name<'n>, value: Value<'v>) {
        self.remove_params(name);
        let params = self.params.to_mut();
        params.push((name, value));
    }

    fn remove_params(&mut self, name: Name) {
        let key_exists = self.params.iter().any(|&param| name == param.0);
        if key_exists {
            self.params.to_mut().retain(|&param| name != param.0);
        }
    }

    fn clear_params(&mut self) {
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
        for (name, value) in &*self.params {
            write!(f, "; {}={}", name, value)?;
        }
        Ok(())
    }
}

impl<'a> From<&'a MediaTypeBuf> for MediaType<'a> {
    fn from(t: &'a MediaTypeBuf) -> Self {
        t.to_ref()
    }
}

impl<'a, 'b> PartialEq<MediaType<'b>> for MediaType<'a> {
    fn eq(&self, other: &MediaType<'b>) -> bool {
        self.ty == other.ty
            && self.subty == other.subty
            && self.suffix == other.suffix
            && self.params().collect::<BTreeMap<_, _>>()
                == other.params().collect::<BTreeMap<_, _>>()
    }
}

impl<'a> Eq for MediaType<'a> {}

impl<'a> PartialEq<MediaTypeBuf> for MediaType<'a> {
    fn eq(&self, other: &MediaTypeBuf) -> bool {
        self.ty == other.ty()
            && self.subty == other.subty()
            && self.suffix == other.suffix()
            && self.params().collect::<BTreeMap<_, _>>()
                == other.params().collect::<BTreeMap<_, _>>()
    }
}

impl<'a> PartialEq<&MediaTypeBuf> for MediaType<'a> {
    fn eq(&self, other: &&MediaTypeBuf) -> bool {
        self == *other
    }
}

impl<'a> Hash for MediaType<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ty.hash(state);
        self.subty.hash(state);
        self.suffix.hash(state);
        self.params()
            .collect::<BTreeMap<_, _>>()
            .into_iter()
            .collect::<Vec<_>>()
            .hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{names::*, values::*};
    use std::collections::hash_map::DefaultHasher;
    use std::str::FromStr;

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    #[test]
    fn to_string() {
        assert_eq!(MediaType::new(_STAR, _STAR).to_string(), "*/*");
        assert_eq!(MediaType::new(TEXT, PLAIN).to_string(), "text/plain");
        assert_eq!(
            MediaType::from_parts(IMAGE, SVG, Some(XML), &[]).to_string(),
            "image/svg+xml"
        );
        assert_eq!(
            MediaType::from_parts(TEXT, PLAIN, None, &[(CHARSET, UTF_8)]).to_string(),
            "text/plain; charset=UTF-8"
        );
        assert_eq!(
            MediaType::from_parts(IMAGE, SVG, Some(XML), &[(CHARSET, UTF_8)]).to_string(),
            "image/svg+xml; charset=UTF-8"
        );
    }

    #[test]
    fn get_param() {
        assert_eq!(MediaType::new(TEXT, PLAIN).get_param(CHARSET), None);
        assert_eq!(
            MediaType::from_parts(TEXT, PLAIN, None, &[(CHARSET, UTF_8)]).get_param(CHARSET),
            Some(UTF_8)
        );
        assert_eq!(
            MediaType::parse("image/svg+xml; charset=UTF-8; HELLO=WORLD; HELLO=world")
                .unwrap()
                .get_param(Name::new("hello").unwrap()),
            Some(Value::new("world").unwrap())
        );
    }

    #[test]
    fn set_param() {
        let mut media_type = MediaType::from_parts(TEXT, PLAIN, None, &[(CHARSET, UTF_8)]);
        let lower_utf8 = Value::new("utf-8").unwrap();
        media_type.set_param(CHARSET, lower_utf8);
        assert_eq!(media_type.to_string(), "text/plain; charset=utf-8");

        let alice = Name::new("ALICE").unwrap();
        let bob = Value::new("bob").unwrap();
        media_type.set_param(alice, bob);
        media_type.set_param(alice, bob);

        assert_eq!(
            media_type.to_string(),
            "text/plain; charset=utf-8; ALICE=bob"
        );
    }

    #[test]
    fn remove_params() {
        let mut media_type = MediaType::from_parts(TEXT, PLAIN, None, &[(CHARSET, UTF_8)]);
        media_type.remove_params(CHARSET);
        assert_eq!(media_type.to_string(), "text/plain");

        let mut media_type =
            MediaType::parse("image/svg+xml; hello=WORLD; charset=UTF-8; HELLO=WORLD").unwrap();
        media_type.remove_params(Name::new("hello").unwrap());
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
            MediaType::parse("IMAGE/SVG+XML; HELLO=WORLD; CHARSET=UTF-8").unwrap()
        );
        assert_eq!(
            MediaType::from_parts(
                IMAGE,
                SVG,
                Some(XML),
                &[(CHARSET, US_ASCII), (CHARSET, UTF_8)]
            ),
            MediaTypeBuf::from_str("image/svg+xml; charset=UTF-8").unwrap(),
        );

        const TEXT_PLAIN: MediaType = MediaType::from_parts(TEXT, PLAIN, None, &[]);
        let text_plain = MediaType::parse("text/plain").unwrap();
        assert_eq!(text_plain.essence(), TEXT_PLAIN);
    }

    #[test]
    fn hash() {
        assert_eq!(
            calculate_hash(&MediaType::parse("text/plain").unwrap()),
            calculate_hash(&MediaType::parse("TEXT/PLAIN").unwrap())
        );
        assert_eq!(
            calculate_hash(&MediaType::parse("image/svg+xml; charset=UTF-8").unwrap()),
            calculate_hash(&MediaType::parse("IMAGE/SVG+XML; CHARSET=UTF-8").unwrap())
        );
        assert_eq!(
            calculate_hash(&MediaType::parse("image/svg+xml; hello=WORLD; charset=UTF-8").unwrap()),
            calculate_hash(&MediaType::parse("IMAGE/SVG+XML; HELLO=WORLD; CHARSET=UTF-8").unwrap())
        );
        assert_eq!(
            calculate_hash(&MediaType::from_parts(
                IMAGE,
                SVG,
                Some(XML),
                &[(CHARSET, UTF_8)]
            )),
            calculate_hash(&MediaType::from_parts(
                IMAGE,
                SVG,
                Some(XML),
                &[(CHARSET, US_ASCII), (CHARSET, UTF_8)]
            )),
        );
    }
}
