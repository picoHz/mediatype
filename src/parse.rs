use super::{error::*, name::*};
use std::{num::NonZeroU8, ops::Range};

#[derive(Debug, Clone)]
pub(crate) struct Indices {
    ty: NonZeroU8,
    subty: NonZeroU8,
    suffix: u8,
    params: Box<[[usize; 4]]>,
}

impl Indices {
    pub const fn ty(&self) -> Range<usize> {
        0..self.ty.get() as _
    }

    pub const fn subty(&self) -> Range<usize> {
        let start = self.ty.get() as usize + 1;
        let end = start + self.subty.get() as usize;
        start..end
    }

    pub const fn suffix(&self) -> Option<Range<usize>> {
        let start = self.ty.get() as usize + 1 + self.subty.get() as usize + 1;
        let end = start + self.suffix as usize;
        if start < end {
            Some(start..end)
        } else {
            None
        }
    }

    pub fn params(&self) -> &[[usize; 4]] {
        &self.params
    }

    pub fn parse(s: &str) -> Result<(Self, usize), MediaTypeError> {
        // ty.len() + '/' + subty.len() + '+' + suffix.len()
        const MAX_ESSENCE_LENGTH: usize = Name::MAX_LENGTH * 3 + 2;

        let (ty, right) = match s[..MAX_ESSENCE_LENGTH.min(s.len())].split_once('/') {
            Some(pair) => pair,
            _ => return Err(MediaTypeError::InvalidTypeName),
        };

        if !is_restricted_name(ty) {
            return Err(MediaTypeError::InvalidTypeName);
        }

        let suffix_end = right
            .find(|c: char| !is_restricted_char(c))
            .unwrap_or(right.len());
        let suffix_start = right[..suffix_end].rfind('+');

        let (subty, suffix) = if let Some(suffix_start) = suffix_start {
            (&right[..suffix_start], &right[suffix_start + 1..suffix_end])
        } else {
            (&right[..suffix_end], "")
        };

        if !is_restricted_name(subty) {
            return Err(MediaTypeError::InvalidSubtypeName);
        }

        if !suffix.is_empty() && !is_restricted_name(&suffix[1..]) {
            return Err(MediaTypeError::InvalidSuffix);
        }

        let sub_end = ty.len() + 1 + subty.len();
        let params_start = sub_end
            + if suffix.is_empty() {
                0
            } else {
                suffix.len() + 1
            };

        let (mut params, params_len) = parse_params(&s[params_start as usize..])?;
        for elem in params.iter_mut() {
            for v in elem.iter_mut() {
                *v += params_start;
            }
        }
        params.sort_unstable_by_key(|&[start, end, _, _]| {
            Name::new_unchecked(&s[start as usize..end as usize])
        });

        for window in params.windows(2) {
            let key_a = Name::new_unchecked(&s[window[0][0] as usize..window[0][1] as usize]);
            let key_b = Name::new_unchecked(&s[window[1][0] as usize..window[1][1] as usize]);
            if key_a == key_b {
                return Err(MediaTypeError::DuplicatedParamKeys);
            }
        }

        Ok((
            Self {
                ty: NonZeroU8::new(ty.len() as _).unwrap(),
                subty: NonZeroU8::new(subty.len() as _).unwrap(),
                suffix: suffix.len() as _,
                params: params.into_boxed_slice(),
            },
            params_start + params_len,
        ))
    }
}

#[cfg(test)]
fn parse_to_string(s: &str) -> Result<String, MediaTypeError> {
    use std::fmt::Write;

    let mut out = String::new();
    let (indices, _) = Indices::parse(s)?;

    write!(out, "{}/{}", &s[indices.ty()], &s[indices.subty()]).unwrap();
    if let Some(suffix) = indices.suffix() {
        write!(out, "+{}", &s[suffix]).unwrap();
    }
    for param in indices.params() {
        write!(
            out,
            "; {}={}",
            &s[param[0] as usize..param[1] as usize],
            &s[param[2] as usize..param[3] as usize]
        )
        .unwrap();
    }

    Ok(out)
}

pub fn is_restricted_name(s: &str) -> bool {
    s.len() <= Name::MAX_LENGTH && s.starts_with(char::is_alphanumeric) && is_restricted_str(s)
}

pub fn is_restricted_str(s: &str) -> bool {
    s.chars().all(is_restricted_char)
}

fn is_restricted_char(c: char) -> bool {
    c.is_alphanumeric()
        || matches!(
            c,
            '!' | '#' | '$' | '&' | '-' | '^' | '_' | '.' | '+' | '%' | '*' | '\''
        )
}

fn is_ows(c: char) -> bool {
    c == ' ' || c == '\t'
}

fn parse_params(s: &str) -> Result<(Vec<[usize; 4]>, usize), MediaTypeError> {
    let mut vec = Vec::new();
    let mut offset = 0;
    let mut len = 0;

    while let Some((key, value)) = parse_param(&s[offset..])? {
        vec.push([
            offset + key.start,
            offset + key.end,
            offset + value.start,
            offset + value.end,
        ]);
        len = offset + value.end;
        offset += value.end;
    }

    Ok((vec, len))
}

type ParamRange = (Range<usize>, Range<usize>);

fn parse_param(s: &str) -> Result<Option<ParamRange>, MediaTypeError> {
    let (ows, right) = match s.split_once(';') {
        Some((ows, right)) if ows.chars().all(is_ows) && right.chars().all(is_ows) => {
            return Ok(None)
        }
        Some((ows, right)) if ows.chars().all(is_ows) => (ows, right),
        _ if s.chars().all(is_ows) => return Ok(None),
        _ => return Err(MediaTypeError::InvalidParams),
    };

    let (key, value) = match right.split_once('=') {
        Some(pair) => pair,
        _ => return Err(MediaTypeError::InvalidParams),
    };

    let key_trimmed = key.trim_start_matches(is_ows).len();
    let key_start = ows.len() + 1 + key.len() - key_trimmed;
    let key_range = key_start..key_start + key_trimmed;
    if !is_restricted_name(&s[key_range.clone()]) {
        return Err(MediaTypeError::InvalidParamKey);
    }

    let value_start = key_range.end + 1;
    if let Some(value) = value.strip_prefix('\"') {
        let value_end = value_start + parse_quoted_value(value)? + 1;
        let value_range = value_start..value_end;
        Ok(Some((key_range, value_range)))
    } else {
        let value_end = value_start
            + value
                .chars()
                .take_while(|&c| is_restricted_char(c))
                .map(char::len_utf8)
                .sum::<usize>();
        let value_range = value_start..value_end;
        Ok(Some((key_range, value_range)))
    }
}

fn parse_quoted_value(s: &str) -> Result<usize, MediaTypeError> {
    let mut len = 0;
    let mut escaped = false;
    for c in s.chars() {
        len += c.len_utf8();
        match c {
            _ if escaped => {
                escaped = false;
            }
            '\\' => {
                escaped = true;
            }
            '"' => return Ok(len),
            '\n' => return Err(MediaTypeError::InvalidParamValue),
            _ => (),
        }
    }
    Err(MediaTypeError::InvalidParamValue)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(parse_to_string("text/plain"), Ok("text/plain".into()));
        assert_eq!(parse_to_string("text/plain;"), Ok("text/plain".into()));
        assert_eq!(parse_to_string("image/svg+xml"), Ok("image/svg+xml".into()));
        assert_eq!(
            parse_to_string("image/svg+xml;"),
            Ok("image/svg+xml".into())
        );
        assert_eq!(
            parse_to_string("image/svg+xml; charset=UTF-8"),
            Ok("image/svg+xml; charset=UTF-8".into())
        );
        assert_eq!(
            parse_to_string("image/svg+xml; charset=UTF-8;"),
            Ok("image/svg+xml; charset=UTF-8".into())
        );
        assert_eq!(
            parse_to_string("image/svg+xml    ; charset=UTF-8   "),
            Ok("image/svg+xml; charset=UTF-8".into())
        );
        assert_eq!(
            parse_to_string("image/svg+xml; charset=\"UTF-8\""),
            Ok("image/svg+xml; charset=\"UTF-8\"".into())
        );
        assert_eq!(
            parse_to_string("image/svg+xml; charset=\"UT\\\"F-8\""),
            Ok("image/svg+xml; charset=\"UT\\\"F-8\"".into())
        );
        assert_eq!(
            parse_to_string("multipart/form-data ; boundary=--boundary13234"),
            Ok("multipart/form-data; boundary=--boundary13234".into())
        );

        let s = "text/plain";
        let long_str = format!("{};{}", s, " ".repeat(std::u16::MAX as usize - 2 - s.len()));
        assert_eq!(parse_to_string(&long_str), Ok("text/plain".into()));

        let long_name = "a".repeat(Name::MAX_LENGTH);
        let long_str = format!("{}/{}+{}", long_name, long_name, long_name);
        assert_eq!(parse_to_string(&long_str), Ok(long_str));
    }

    #[test]
    fn parse_error() {
        assert_eq!(parse_to_string(""), Err(MediaTypeError::InvalidTypeName));
        assert_eq!(
            parse_to_string("textplain"),
            Err(MediaTypeError::InvalidTypeName)
        );
        assert_eq!(
            parse_to_string("text//plain"),
            Err(MediaTypeError::InvalidSubtypeName)
        );
        assert_eq!(
            parse_to_string(" text/plain"),
            Err(MediaTypeError::InvalidTypeName)
        );
        assert_eq!(
            parse_to_string("text/plain; charsetUTF-8"),
            Err(MediaTypeError::InvalidParams)
        );
        assert_eq!(
            parse_to_string("text/plain;;"),
            Err(MediaTypeError::InvalidParams)
        );
        assert_eq!(
            parse_to_string("text/plain;;;"),
            Err(MediaTypeError::InvalidParams)
        );
        assert_eq!(
            parse_to_string("text/plain; charset=UTF-8; charset=UTF-8"),
            Err(MediaTypeError::DuplicatedParamKeys)
        );
        assert_eq!(
            parse_to_string("text/plain; charset=\"UTF-8"),
            Err(MediaTypeError::InvalidParamValue)
        );
        assert_eq!(
            parse_to_string("text/plain; charset==UTF-8"),
            Err(MediaTypeError::InvalidParams)
        );
        assert_eq!(
            parse_to_string("text/plain; \r\n charset=UTF-8;"),
            Err(MediaTypeError::InvalidParamKey)
        );

        let long_str = format!("{}/plain", "t".repeat(std::u16::MAX as usize));
        assert_eq!(
            parse_to_string(&long_str),
            Err(MediaTypeError::InvalidTypeName)
        );
    }
}
