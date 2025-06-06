use super::parse::*;
use alloc::{borrow::Cow, fmt, string::String};
use core::{
    cmp::Ordering,
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
    #[must_use]
    pub fn new(s: &'a str) -> Option<Self> {
        if is_restricted_name(s) {
            Some(Self(s))
        } else {
            None
        }
    }

    /// Returns the underlying string.
    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        self.0
    }

    /// The maximum byte length of a name.
    pub const MAX_LENGTH: usize = 127;

    /// Constructs a `Name` without validation.
    pub const fn new_unchecked(s: &'a str) -> Self {
        Self(s)
    }
}

impl fmt::Display for Name<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl AsRef<str> for Name<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl PartialEq for Name<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(other.as_ref())
    }
}

impl Eq for Name<'_> {}

impl PartialOrd for Name<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Name<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .chars()
            .map(|c| c.to_ascii_lowercase())
            .cmp(other.0.chars().map(|c| c.to_ascii_lowercase()))
    }
}

impl Hash for Name<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_ascii_lowercase().hash(state);
    }
}

impl PartialEq<String> for Name<'_> {
    fn eq(&self, other: &String) -> bool {
        self.eq(other.as_str())
    }
}

impl PartialOrd<String> for Name<'_> {
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        self.partial_cmp(other.as_str())
    }
}

impl PartialEq<&String> for Name<'_> {
    fn eq(&self, other: &&String) -> bool {
        self.eq(other.as_str())
    }
}

impl PartialOrd<&String> for Name<'_> {
    fn partial_cmp(&self, other: &&String) -> Option<Ordering> {
        self.partial_cmp(other.as_str())
    }
}

impl PartialEq<str> for Name<'_> {
    fn eq(&self, other: &str) -> bool {
        self.0.eq_ignore_ascii_case(other)
    }
}

impl PartialOrd<str> for Name<'_> {
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        Some(
            self.0
                .chars()
                .map(|c| c.to_ascii_lowercase())
                .cmp(other.chars().map(|c| c.to_ascii_lowercase())),
        )
    }
}

impl PartialEq<&str> for Name<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.eq(*other)
    }
}

impl PartialOrd<&str> for Name<'_> {
    fn partial_cmp(&self, other: &&str) -> Option<Ordering> {
        self.partial_cmp(*other)
    }
}

impl PartialEq<Cow<'_, str>> for Name<'_> {
    fn eq(&self, other: &Cow<'_, str>) -> bool {
        self.eq(other.as_ref())
    }
}

impl PartialOrd<Cow<'_, str>> for Name<'_> {
    fn partial_cmp(&self, other: &Cow<'_, str>) -> Option<Ordering> {
        self.partial_cmp(other.as_ref())
    }
}

impl PartialEq<&Cow<'_, str>> for Name<'_> {
    fn eq(&self, other: &&Cow<'_, str>) -> bool {
        self.eq(other.as_ref())
    }
}

impl PartialOrd<&Cow<'_, str>> for Name<'_> {
    fn partial_cmp(&self, other: &&Cow<'_, str>) -> Option<Ordering> {
        self.partial_cmp(other.as_ref())
    }
}
