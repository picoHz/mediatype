use super::parse::*;
use std::{
    borrow::Cow,
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
};

/// A media-type parameter value.
///
/// A valid value has the following requirements:
///
/// - Allowed characters are alphabets, numbers and `!#$&-^_.+%*'`.
/// - The value can not be empty.
#[derive(Debug, Copy, Clone)]
pub struct Value<'a>(&'a str);

impl<'a> Value<'a> {
    /// Constructs a `Value`.
    ///
    /// If the string is not valid as a value, returns `None`.
    pub fn new(s: &'a str) -> Option<Self> {
        if is_restricted_str(s) {
            Some(Self(s))
        } else {
            None
        }
    }

    /// Returns the underlying string.
    pub fn unquoted_str(&self) -> Cow<'_, str> {
        Cow::Borrowed(self.0)
    }

    pub(crate) const fn new_unchecked(s: &'a str) -> Self {
        Self(s)
    }
}

impl<'a> fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.unquoted_str())
    }
}

impl<'a> PartialEq for Value<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.unquoted_str() == other.unquoted_str()
    }
}

impl<'a> Eq for Value<'a> {}

impl<'a> PartialOrd for Value<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Value<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.unquoted_str().cmp(&other.unquoted_str())
    }
}

impl<'a> Hash for Value<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.unquoted_str().hash(state);
    }
}

impl<'a, T> PartialEq<T> for Value<'a>
where
    T: AsRef<str>,
{
    fn eq(&self, other: &T) -> bool {
        self.unquoted_str() == other.as_ref()
    }
}

impl<'a, T> PartialOrd<T> for Value<'a>
where
    T: AsRef<str>,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        Some(self.unquoted_str().as_ref().cmp(other.as_ref()))
    }
}
