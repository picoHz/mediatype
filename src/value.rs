use super::parse::*;
use std::{cmp::Ordering, fmt};

/// A MediaType parameter value.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Value<'a>(pub(crate) &'a str);

impl<'a> Value<'a> {
    pub fn new(s: &'a str) -> Option<Self> {
        if is_restricted_str(s) {
            Some(Self(s))
        } else {
            None
        }
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

impl<'a> PartialEq<String> for Value<'a> {
    fn eq(&self, other: &String) -> bool {
        self.0 == other
    }
}

impl<'a> PartialOrd<String> for Value<'a> {
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        Some(self.0.cmp(other))
    }
}

impl<'a> PartialEq<str> for Value<'a> {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl<'a> PartialOrd<str> for Value<'a> {
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        Some(self.0.cmp(other))
    }
}
