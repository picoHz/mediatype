use super::parse::*;
use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
};

/// A media-type name.
///
/// A valid name has the following requirements:
///
/// - The first character must be an alphabet or a number.
/// - The subsequent characters must be alphabets, numbers or `!#$&-^_.+%*'`.
/// - Length of the name can not exceed 127 bytes.
#[derive(Debug, Copy, Clone)]
pub struct Name<'a>(&'a str);

impl<'a> Name<'a> {
    /// Constructs a `Name`.
    ///
    /// If the string is not valid as a name, returns `None`.
    pub fn new(s: &'a str) -> Option<Self> {
        if is_restricted_name(s) {
            Some(Self(s))
        } else {
            None
        }
    }

    /// Returns the underlying string.
    pub fn as_str(&self) -> &str {
        self.0
    }

    /// The maximum byte length of a name.
    pub const MAX_LENGTH: usize = 127;

    pub(crate) const fn new_unchecked(s: &'a str) -> Self {
        Self(s)
    }
}

impl<'a> fmt::Display for Name<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl<'a> AsRef<str> for Name<'a> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl<'a> Eq for Name<'a> {}

impl<'a> Ord for Name<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'a> Hash for Name<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_ascii_lowercase().hash(state);
    }
}

impl<'a, T> PartialEq<T> for Name<'a>
where
    T: AsRef<str>,
{
    fn eq(&self, other: &T) -> bool {
        self.0.eq_ignore_ascii_case(other.as_ref())
    }
}

impl<'a, T> PartialOrd<T> for Name<'a>
where
    T: AsRef<str>,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        Some(
            self.0
                .to_ascii_lowercase()
                .cmp(&other.as_ref().to_ascii_lowercase()),
        )
    }
}
