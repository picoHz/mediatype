use super::parse::*;
use std::{
    borrow::Cow,
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    iter,
};

/// A media-type parameter value.
///
/// The constructor accepts both an unquoted string and a quoted string like `"Hello Wold!"`.
/// If the string is not quoted, the allowed characters are
/// alphabets, numbers and `!#$&-^_.+%*'`.
///
/// ```
/// # use mediatype::Value;
/// let utf8 = Value::new("UTF-8").unwrap();
/// let utf8_quoted = Value::new("\"UTF-8\"").unwrap();
/// assert_eq!(utf8, utf8_quoted);
/// assert_eq!(utf8_quoted.as_str(), "\"UTF-8\"");
/// assert_eq!(utf8_quoted.unquoted_str(), "UTF-8");
///
/// let double_quoted = Value::new("\" \\\" \"").unwrap();
/// assert_eq!(double_quoted.as_str(), "\" \\\" \"");
/// assert_eq!(double_quoted.unquoted_str(), " \" ");
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Value<'a>(&'a str);

impl<'a> Value<'a> {
    /// Constructs a `Value`.
    ///
    /// If the string is not valid as a value, returns `None`.
    #[must_use]
    pub fn new(s: &'a str) -> Option<Self> {
        if let Some(quoted) = s.strip_prefix('\"') {
            if quoted.len() > 1 && parse_quoted_value(quoted) == Ok(quoted.len()) {
                return Some(Self(s));
            }
        } else if is_restricted_str(s) {
            return Some(Self(s));
        }
        None
    }

    /// Returns the underlying string.
    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        self.0
    }

    /// Returns the unquoted string.
    #[must_use]
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

    /// Generates a quoted string if necessary.
    ///
    /// ```
    /// # use mediatype::Value;
    /// assert_eq!(Value::quote("UTF-8"), "UTF-8");
    /// assert_eq!(Value::quote("Hello world!"), "\"Hello world!\"");
    /// assert_eq!(
    ///     Value::quote(r#" "What's wrong?" "#),
    ///     "\" \\\"What's wrong\\?\\\" \""
    /// );
    /// ```
    #[must_use]
    pub fn quote(s: &str) -> Cow<'_, str> {
        if is_restricted_str(s) {
            Cow::Borrowed(s)
        } else {
            let inner = s.chars().flat_map(|c| {
                if is_restricted_char(c) || c == ' ' {
                    vec![c]
                } else {
                    vec!['\\', c]
                }
            });
            let quoted = iter::once('"').chain(inner).chain(iter::once('"'));
            Cow::Owned(quoted.collect())
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

impl<'a> PartialEq<String> for Value<'a> {
    fn eq(&self, other: &String) -> bool {
        self.eq(other.as_str())
    }
}

impl<'a> PartialOrd<String> for Value<'a> {
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        self.partial_cmp(other.as_str())
    }
}

impl<'a> PartialEq<&String> for Value<'a> {
    fn eq(&self, other: &&String) -> bool {
        self.eq(other.as_str())
    }
}

impl<'a> PartialOrd<&String> for Value<'a> {
    fn partial_cmp(&self, other: &&String) -> Option<Ordering> {
        self.partial_cmp(other.as_str())
    }
}

impl<'a> PartialEq<str> for Value<'a> {
    fn eq(&self, other: &str) -> bool {
        self.unquoted_str() == other
    }
}

impl<'a> PartialOrd<str> for Value<'a> {
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        Some(self.unquoted_str().as_ref().cmp(other))
    }
}

impl<'a> PartialEq<&str> for Value<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.eq(*other)
    }
}

impl<'a> PartialOrd<&str> for Value<'a> {
    fn partial_cmp(&self, other: &&str) -> Option<Ordering> {
        self.partial_cmp(*other)
    }
}

impl<'a> PartialEq<Cow<'_, str>> for Value<'a> {
    fn eq(&self, other: &Cow<'_, str>) -> bool {
        self.eq(other.as_ref())
    }
}

impl<'a> PartialOrd<Cow<'_, str>> for Value<'a> {
    fn partial_cmp(&self, other: &Cow<'_, str>) -> Option<Ordering> {
        self.partial_cmp(other.as_ref())
    }
}

impl<'a> PartialEq<&Cow<'_, str>> for Value<'a> {
    fn eq(&self, other: &&Cow<'_, str>) -> bool {
        self.eq(other.as_ref())
    }
}

impl<'a> PartialOrd<&Cow<'_, str>> for Value<'a> {
    fn partial_cmp(&self, other: &&Cow<'_, str>) -> Option<Ordering> {
        self.partial_cmp(other.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unquoted_str() {
        assert_eq!(Value::new("\"\\a\\\\\"").unwrap().unquoted_str(), "a\\");
        assert_eq!(Value::new("\"\\\"\"").unwrap().unquoted_str(), "\"");
        assert_eq!(Value::new("\"\\a\\b\\c\"").unwrap().unquoted_str(), "abc");
        assert_eq!(
            Value::new("\" \\\"What's wrong\\?\\\" \"")
                .unwrap()
                .unquoted_str(),
            r#" "What's wrong?" "#
        );
    }
}
