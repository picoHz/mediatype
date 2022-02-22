use super::parse::*;
use std::{borrow::Cow, cmp::Ordering, fmt};

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

    /// Returns the unquoted string.
    pub fn unquoted_str(&self) -> Cow<'_, str> {
        if self.0.starts_with('"') {
            let inner = &self.0[1..self.0.len() - 1];
            if inner.contains('\\') {
                let mut s = String::with_capacity(inner.len());
                let mut quoted = false;
                for c in inner.chars() {
                    match c {
                        _ if quoted => {
                            quoted = false;
                            s.push(c);
                        }
                        '\\' => {
                            quoted = true;
                        }
                        _ => {
                            s.push(c);
                        }
                    }
                }
                Cow::Owned(s)
            } else {
                Cow::Borrowed(inner)
            }
        } else {
            Cow::Borrowed(self.0)
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unquoted_str() {
        assert_eq!(Value::new_unchecked("\"\\a\\\\\"").unquoted_str(), "a\\");
        assert_eq!(Value::new_unchecked("\"\\\"\"").unquoted_str(), "\"");
        assert_eq!(Value::new_unchecked("\"\\a\\b\\c\"").unquoted_str(), "abc");
    }
}
