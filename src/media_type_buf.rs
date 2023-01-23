use super::{error::*, media_type::*, name::*, params::*, parse::*, value::*};
use std::{
    borrow::Cow,
    collections::BTreeMap,
    fmt,
    hash::{Hash, Hasher},
    str::FromStr,
};

/// An owned and immutable media type.
///
/// ```
/// use mediatype::{names::*, values::*, MediaType, MediaTypeBuf, ReadParams};
///
/// let text_plain: MediaTypeBuf = "text/plain; charset=UTF-8".parse().unwrap();
/// assert_eq!(text_plain.get_param(CHARSET).unwrap(), UTF_8);
///
/// let mut text_markdown: MediaType = text_plain.to_ref();
/// text_markdown.subty = MARKDOWN;
/// assert_eq!(text_markdown.to_string(), "text/markdown; charset=UTF-8");
/// ```
#[derive(Debug, Clone)]
pub struct MediaTypeBuf {
    data: Box<str>,
    indices: Indices,
}

impl MediaTypeBuf {
    /// Constructs a `MediaTypeBuf` from a top-level type and a subtype.
    #[must_use]
    pub fn new(ty: Name, subty: Name) -> Self {
        Self::from_string(format!("{}/{}", ty, subty)).expect("`ty` and `subty` should be valid")
    }

    /// Constructs a `MediaTypeBuf` with an optional suffix and parameters.
    #[must_use]
    pub fn from_parts(
        ty: Name,
        subty: Name,
        suffix: Option<Name>,
        params: &[(Name, Value)],
    ) -> Self {
        use std::fmt::Write;
        let mut s = String::new();
        write!(s, "{}/{}", ty, subty).expect("`ty` and `subty` should be valid");
        if let Some(suffix) = suffix {
            write!(s, "+{}", suffix).unwrap();
        }
        for (name, value) in params {
            write!(s, "; {}={}", name, value).unwrap();
        }
        Self::from_string(s).expect("all values should be valid")
    }

    /// Constructs a `MediaTypeBuf` from [`String`].
    ///
    /// Unlike [`FromStr::from_str`], this function takes the ownership of [`String`]
    /// instead of making a new copy.
    ///
    /// # Errors
    ///
    /// Returns an error if the string fails to be parsed.
    ///
    /// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
    /// [`FromStr::from_str`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
    pub fn from_string(mut s: String) -> Result<Self, MediaTypeError> {
        let (indices, len) = Indices::parse(&s)?;
        s.truncate(len);
        Ok(Self {
            data: s.into(),
            indices,
        })
    }

    /// Returns the top-level type.
    #[must_use]
    pub fn ty(&self) -> Name {
        Name::new_unchecked(&self.data[self.indices.ty()])
    }

    /// Returns the subtype.
    #[must_use]
    pub fn subty(&self) -> Name {
        Name::new_unchecked(&self.data[self.indices.subty()])
    }

    /// Returns the suffix.
    #[must_use]
    pub fn suffix(&self) -> Option<Name> {
        self.indices
            .suffix()
            .map(|range| Name::new_unchecked(&self.data[range]))
    }

    /// Returns a [`MediaType`] without parameters.
    ///
    /// ```
    /// # use mediatype::MediaTypeBuf;
    /// # use std::str::FromStr;
    /// let media_type: MediaTypeBuf = "image/svg+xml; charset=UTF-8".parse().unwrap();
    /// assert_eq!(media_type.essence().to_string(), "image/svg+xml");
    /// ```
    ///
    /// [`MadiaType`]: ./struct.MediaType.html
    #[must_use]
    pub fn essence(&self) -> MediaType<'_> {
        MediaType::from_parts(self.ty(), self.subty(), self.suffix(), &[])
    }

    /// Returns the underlying string.
    #[must_use]
    pub const fn as_str(&self) -> &str {
        &self.data
    }

    /// Returns the canonicalized `MediaTypeBuf`.
    ///
    /// All strings except parameter values will be converted to lowercase.
    ///
    /// ```
    /// # use mediatype::MediaTypeBuf;
    /// let media_type: MediaTypeBuf = "IMAGE/SVG+XML;  CHARSET=UTF-8;  ".parse().unwrap();
    /// assert_eq!(
    ///     media_type.canonicalize().to_string(),
    ///     "image/svg+xml; charset=UTF-8"
    /// );
    /// ```
    #[must_use]
    pub fn canonicalize(&self) -> Self {
        use std::fmt::Write;
        let mut s = String::with_capacity(self.data.len());
        write!(
            s,
            "{}/{}",
            self.ty().as_str().to_ascii_lowercase(),
            self.subty().as_str().to_ascii_lowercase()
        )
        .unwrap();
        if let Some(suffix) = self.suffix() {
            write!(s, "+{}", suffix.as_str().to_ascii_lowercase())
                .expect("`write` should not fail on a `String`");
        }
        for (name, value) in self.params() {
            write!(s, "; {}={}", name.as_str().to_ascii_lowercase(), value)
                .expect("`write` should not fail on a `String`");
        }
        s.shrink_to_fit();
        Self::from_string(s).expect("all values should be valid")
    }

    /// Constructs a `MediaType` from `self`.
    #[must_use]
    pub fn to_ref(&self) -> MediaType {
        let params = self.params().collect::<Vec<_>>();
        let params = if params.is_empty() {
            Cow::Borrowed([].as_slice())
        } else {
            Cow::Owned(params)
        };
        MediaType::from_parts_unchecked(self.ty(), self.subty(), self.suffix(), params)
    }
}

impl ReadParams for MediaTypeBuf {
    fn params(&self) -> Params {
        Params::from_indices(&self.data, &self.indices)
    }

    fn get_param(&self, name: Name) -> Option<Value> {
        self.indices
            .params()
            .iter()
            .rev()
            .find(|&&[start, end, _, _]| {
                name == Name::new_unchecked(&self.data[start..end])
            })
            .map(|&[_, _, start, end]| {
                Value::new_unchecked(&self.data[start..end])
            })
    }
}

impl FromStr for MediaTypeBuf {
    type Err = MediaTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (indices, len) = Indices::parse(s)?;
        Ok(Self {
            data: s[..len].into(),
            indices,
        })
    }
}

impl From<MediaType<'_>> for MediaTypeBuf {
    fn from(t: MediaType) -> Self {
        Self::from_string(t.to_string()).expect("`t` should be valid")
    }
}

impl From<&MediaType<'_>> for MediaTypeBuf {
    fn from(t: &MediaType) -> Self {
        Self::from_string(t.to_string()).expect("`t` should be valid")
    }
}

impl AsRef<str> for MediaTypeBuf {
    fn as_ref(&self) -> &str {
        &self.data
    }
}

impl PartialEq for MediaTypeBuf {
    fn eq(&self, other: &Self) -> bool {
        self.ty() == other.ty()
            && self.subty() == other.subty()
            && self.suffix() == other.suffix()
            && self.params().collect::<BTreeMap<_, _>>()
                == other.params().collect::<BTreeMap<_, _>>()
    }
}

impl Eq for MediaTypeBuf {}

impl PartialEq<MediaType<'_>> for MediaTypeBuf {
    fn eq(&self, other: &MediaType) -> bool {
        self.ty() == other.ty
            && self.subty() == other.subty
            && self.suffix() == other.suffix
            && self.params().collect::<BTreeMap<_, _>>()
                == other.params().collect::<BTreeMap<_, _>>()
    }
}

impl PartialEq<&MediaType<'_>> for MediaTypeBuf {
    fn eq(&self, other: &&MediaType) -> bool {
        self == *other
    }
}

impl PartialEq<MediaType<'_>> for &MediaTypeBuf {
    fn eq(&self, other: &MediaType) -> bool {
        *self == other
    }
}

impl fmt::Display for MediaTypeBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.ty(), self.subty())?;
        if let Some(suffix) = self.suffix() {
            write!(f, "+{}", suffix)?;
        }
        for (name, value) in self.params() {
            write!(f, "; {}={}", name, value)?;
        }
        Ok(())
    }
}

impl Hash for MediaTypeBuf {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ty().hash(state);
        self.subty().hash(state);
        self.suffix().hash(state);
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
    use crate::{media_type, names::*, values::*};
    use std::collections::hash_map::DefaultHasher;

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    #[test]
    fn from_parts() {
        assert_eq!(
            MediaTypeBuf::from_parts(IMAGE, SVG, Some(XML), &[(CHARSET, UTF_8)]).to_string(),
            "image/svg+xml; charset=UTF-8"
        );
    }

    #[test]
    fn get_param() {
        assert_eq!(
            MediaTypeBuf::from_str("image/svg+xml")
                .unwrap()
                .get_param(CHARSET),
            None
        );
        assert_eq!(
            MediaTypeBuf::from_str("image/svg+xml; charset=UTF-8")
                .unwrap()
                .get_param(CHARSET),
            Some(UTF_8)
        );
        assert_eq!(
            MediaTypeBuf::from_str("image/svg+xml; charset=UTF-8; HELLO=WORLD; HELLO=world")
                .unwrap()
                .get_param(Name::new("hello").unwrap()),
            Some(Value::new("world").unwrap())
        );
    }

    #[test]
    fn essence() {
        assert_eq!(
            MediaTypeBuf::from_str("image/svg+xml")
                .unwrap()
                .essence()
                .to_string(),
            "image/svg+xml"
        );
        assert_eq!(
            MediaTypeBuf::from_str("image/svg+xml;  ")
                .unwrap()
                .essence()
                .to_string(),
            "image/svg+xml"
        );
        assert_eq!(
            MediaTypeBuf::from_str("image/svg+xml; charset=UTF-8")
                .unwrap()
                .essence()
                .to_string(),
            "image/svg+xml"
        );
        assert_eq!(
            MediaTypeBuf::from_str("image/svg+xml  ; charset=UTF-8")
                .unwrap()
                .essence()
                .to_string(),
            "image/svg+xml"
        );
    }

    #[test]
    fn canonicalize() {
        assert_eq!(
            MediaTypeBuf::from_str("IMAGE/SVG+XML;         CHARSET=UTF-8;     ")
                .unwrap()
                .canonicalize()
                .to_string(),
            "image/svg+xml; charset=UTF-8"
        );
    }

    #[test]
    fn cmp() {
        assert_eq!(
            MediaTypeBuf::from_str("text/plain").unwrap(),
            MediaTypeBuf::from_str("TEXT/PLAIN").unwrap()
        );
        assert_eq!(
            MediaTypeBuf::from_str("image/svg+xml; charset=UTF-8").unwrap(),
            MediaTypeBuf::from_str("IMAGE/SVG+XML; CHARSET=UTF-8").unwrap()
        );
        assert_eq!(
            MediaTypeBuf::from_str("image/svg+xml; hello=WORLD; charset=UTF-8").unwrap(),
            MediaTypeBuf::from_str("IMAGE/SVG+XML; HELLO=WORLD; CHARSET=UTF-8").unwrap()
        );
        assert_eq!(
            MediaTypeBuf::from_str("image/svg+xml; charset=UTF-8").unwrap(),
            MediaType::from_parts(
                IMAGE,
                SVG,
                Some(XML),
                &[(CHARSET, US_ASCII), (CHARSET, UTF_8)]
            ),
        );
        assert_eq!(
            &MediaTypeBuf::from_str("image/svg+xml").unwrap(),
            media_type!(IMAGE / SVG + XML)
        );
    }

    #[test]
    fn hash() {
        assert_eq!(
            calculate_hash(&MediaTypeBuf::from_str("text/plain").unwrap()),
            calculate_hash(&MediaTypeBuf::from_str("TEXT/PLAIN").unwrap())
        );
        assert_eq!(
            calculate_hash(&MediaTypeBuf::from_str("image/svg+xml; charset=UTF-8").unwrap()),
            calculate_hash(&MediaTypeBuf::from_str("IMAGE/SVG+XML; CHARSET=UTF-8").unwrap())
        );
        assert_eq!(
            calculate_hash(
                &MediaTypeBuf::from_str("image/svg+xml; hello=WORLD; charset=UTF-8").unwrap()
            ),
            calculate_hash(
                &MediaTypeBuf::from_str("IMAGE/SVG+XML; HELLO=WORLD; CHARSET=UTF-8").unwrap()
            )
        );
        assert_eq!(
            calculate_hash(&MediaTypeBuf::from_str("image/svg+xml; charset=UTF-8").unwrap()),
            calculate_hash(
                &MediaTypeBuf::from_str("image/svg+xml; charset=US-ASCII; charset=UTF-8").unwrap()
            ),
        );
    }
}
