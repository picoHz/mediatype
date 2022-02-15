use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
};

/// A MediaType name.
#[derive(Debug, Copy, Clone)]
pub struct Name<'a>(pub(crate) &'a str);

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

impl<'a> PartialEq for Name<'a> {
    fn eq(&self, other: &Name) -> bool {
        self.0.eq_ignore_ascii_case(other.0)
    }
}

impl<'a> Eq for Name<'a> {}

impl<'a> PartialOrd for Name<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Name<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .to_ascii_lowercase()
            .cmp(&other.0.to_ascii_lowercase())
    }
}

impl<'a> Hash for Name<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_ascii_lowercase().hash(state);
    }
}

impl<'a> PartialEq<String> for Name<'a> {
    fn eq(&self, other: &String) -> bool {
        self.0.eq_ignore_ascii_case(other)
    }
}

impl<'a> PartialOrd<String> for Name<'a> {
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        Some(self.0.cmp(other))
    }
}

impl<'a> PartialEq<str> for Name<'a> {
    fn eq(&self, other: &str) -> bool {
        self.0.eq_ignore_ascii_case(other)
    }
}

impl<'a> PartialOrd<str> for Name<'a> {
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        Some(self.0.cmp(other))
    }
}
