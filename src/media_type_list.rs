use super::{error::*, media_type::*, parse::*};

/// A comma-separated list of `MediaType`s used in the HTTP `Accept` header. ([RFC 7231](https://www.rfc-editor.org/rfc/rfc7231#section-5.3.2))
///
/// ```
/// use mediatype::{MediaType, MediaTypeList};
///
/// let mut list = MediaTypeList::new(
///     "text/html, application/xhtml+xml, application/xml;q=0.9, */*;q=0.8",
/// );
/// assert_eq!(list.next(), Some(MediaType::parse("text/html")));
/// assert_eq!(list.next(), Some(MediaType::parse("application/xhtml+xml")));
/// assert_eq!(list.next(), Some(MediaType::parse("application/xml;q=0.9")));
/// assert_eq!(list.next(), Some(MediaType::parse("*/*;q=0.8")));
/// assert_eq!(list.next(), None);
///
/// // Commas can be used in a quoted string.
/// let mut list = MediaTypeList::new("text/html; message=\"Hello, world!\"");
/// assert_eq!(list.next(), Some(MediaType::parse("text/html; message=\"Hello, world!\"")));
/// assert_eq!(list.next(), None);
/// ```
pub struct MediaTypeList<'a>(&'a str);

impl<'a> MediaTypeList<'a> {
    /// Constructs a `MediaTypeList`.
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }
}

impl<'a> Iterator for MediaTypeList<'a> {
    type Item = Result<MediaType<'a>, MediaTypeError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.0.find(|c| !is_ows(c)) {
            self.0 = &self.0[index..];
        } else {
            return None;
        }
        if self.0.is_empty() {
            return None;
        }
        let mut end = 0;
        let mut quoted = false;
        let mut escaped = false;
        while let Some(c) = self.0.as_bytes().get(end) {
            if escaped {
                escaped = false;
            } else {
                match c {
                    b'"' => quoted = !quoted,
                    b'\\' if quoted => escaped = true,
                    b',' if !quoted => break,
                    _ => (),
                }
            }
            end += 1;
        }
        let madia_type = MediaType::parse(&self.0[..end]);
        let end = self.0.as_bytes().len().min(end + 1);
        self.0 = &self.0[end..];
        Some(madia_type)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.0.matches(',').count() + 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::ReadParams;

    #[test]
    fn empty() {
        let mut list = MediaTypeList::new("");
        assert_eq!(list.next(), None);
        let mut list = MediaTypeList::new("   \t   ");
        assert_eq!(list.next(), None);
    }

    #[test]
    fn invalid() {
        let mut list = MediaTypeList::new(",,,");
        assert_eq!(list.next(), Some(MediaType::parse("")));
        assert_eq!(list.next(), Some(MediaType::parse("")));
        assert_eq!(list.next(), Some(MediaType::parse("")));
        assert_eq!(list.next(), None);
    }

    #[test]
    fn simple() {
        let mut list = MediaTypeList::new("text/html");
        assert_eq!(list.next(), Some(MediaType::parse("text/html")));
        assert_eq!(list.next(), None);
        let mut list = MediaTypeList::new("image/*");
        assert_eq!(list.next(), Some(MediaType::parse("image/*")));
        assert_eq!(list.next(), None);
        let mut list = MediaTypeList::new("*/*");
        assert_eq!(list.next(), Some(MediaType::parse("*/*")));
        assert_eq!(list.next(), None);
    }

    #[test]
    fn list() {
        let mut list = MediaTypeList::new("text/html, image/*, */*");
        assert_eq!(list.next(), Some(MediaType::parse("text/html")));
        assert_eq!(list.next(), Some(MediaType::parse("image/*")));
        assert_eq!(list.next(), Some(MediaType::parse("*/*")));
        assert_eq!(list.next(), None);
    }

    #[test]
    fn params() {
        let mut list = MediaTypeList::new(
            "text/html, application/xhtml+xml, application/xml;q=0.9, */*;q=0.8",
        );
        assert_eq!(list.next(), Some(MediaType::parse("text/html")));
        assert_eq!(list.next(), Some(MediaType::parse("application/xhtml+xml")));
        assert_eq!(list.next(), Some(MediaType::parse("application/xml;q=0.9")));
        assert_eq!(list.next(), Some(MediaType::parse("*/*;q=0.8")));
        assert_eq!(list.next(), None);
    }

    #[test]
    fn quoted_params() {
        let mut list = MediaTypeList::new("text/html; message=\"Hello, world!\", application/xhtml+xml; message=\"Hello, world?\"");

        let media_type = list.next();
        assert_eq!(
            media_type,
            Some(MediaType::parse("text/html; message=\"Hello, world!\""))
        );
        assert_eq!(
            media_type
                .unwrap()
                .unwrap()
                .params()
                .next()
                .unwrap()
                .1
                .unquoted_str(),
            "Hello, world!"
        );

        let media_type = list.next();
        assert_eq!(
            media_type,
            Some(MediaType::parse(
                "application/xhtml+xml; message=\"Hello, world?\""
            ))
        );
        assert_eq!(
            media_type
                .unwrap()
                .unwrap()
                .params()
                .next()
                .unwrap()
                .1
                .unquoted_str(),
            "Hello, world?"
        );

        assert_eq!(list.next(), None);
    }

    #[test]
    fn escaped_params() {
        let mut list = MediaTypeList::new("image/svg+xml; charset=\"UT\\\"F-8\"");
        assert_eq!(
            list.next(),
            Some(MediaType::parse("image/svg+xml; charset=\"UT\\\"F-8\""))
        );
        assert_eq!(list.next(), None);
    }

    #[test]
    fn size_hint() {
        let list = MediaTypeList::new("");
        assert_eq!(list.size_hint(), (0, Some(1)));

        let list = MediaTypeList::new(
            "text/html, application/xhtml+xml, application/xml;q=0.9, */*;q=0.8",
        );
        assert_eq!(list.size_hint(), (0, Some(4)));

        let list = MediaTypeList::new("text/html; message=\"Hello, world!\"");
        assert_eq!(list.size_hint(), (0, Some(2)));
    }
}
