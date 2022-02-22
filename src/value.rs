use super::parse::*;
use std::{cmp::Ordering, fmt};

/// A media-type parameter value.
///
/// A valid value has the following requirements:
///
/// - Allowed characters are alphabets, numbers and `!#$&-^_.+%*'`.
/// - The value can not be empty.
#[derive(Debug, Copy, Clone, Hash)]
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
    pub fn as_str(&self) -> &str {
        self.0
    }

    pub(crate) const fn new_unchecked(s: &'a str) -> Self {
        Self(s)
    }
}

impl<'a> fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl<'a> AsRef<str> for Value<'a> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl<'a> Eq for Value<'a> {}

impl<'a> Ord for Value<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'a, T> PartialEq<T> for Value<'a>
where
    T: AsRef<str>,
{
    fn eq(&self, other: &T) -> bool {
        self.0.eq_ignore_ascii_case(other.as_ref())
    }
}

impl<'a, T> PartialOrd<T> for Value<'a>
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
