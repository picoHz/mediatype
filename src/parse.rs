use super::name::*;
use std::{error, fmt, num::NonZeroU16, ops::Range};

const INPUT_LENGTH_HARD_LIMIT: usize = std::u16::MAX as usize - 1;
const TYPE_NAME_LENGTH_HARD_LIMIT: usize = 127;

#[derive(Debug, Clone)]
pub(crate) struct Indices {
    ty: NonZeroU16,
    subty: NonZeroU16,
    suffix: u16,
    params: Box<[[u16; 4]]>,
}

impl Indices {
    pub const fn ty(&self) -> Range<usize> {
        0..self.ty.get() as _
    }

    pub const fn subty(&self) -> Range<usize> {
        (self.ty.get() + 1) as _..(self.ty.get() + 1 + self.subty.get()) as _
    }

    pub const fn suffix(&self) -> Option<Range<usize>> {
        if self.suffix > 0 {
            Some(
                (self.ty.get() + 1 + self.subty.get() + 1) as _
                    ..(self.ty.get() + 1 + self.subty.get() + 1 + self.suffix) as _,
            )
        } else {
            None
        }
    }

    pub fn params(&self) -> &[[u16; 4]] {
        &self.params
    }

    pub fn parse(s: &str) -> Result<(Self, usize), ParseError> {
        if s.len() > INPUT_LENGTH_HARD_LIMIT {
            return Err(ParseError::TooLongInput);
        }

        let (ty, right) = match s.split_once('/') {
            Some(pair) => pair,
            _ => return Err(ParseError::InvalidTypeName),
        };

        if !is_restricted_name(ty) {
            return Err(ParseError::InvalidTypeName);
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
            return Err(ParseError::InvalidSubtypeName);
        }

        if !suffix.is_empty() && !is_restricted_name(&suffix[1..]) {
            return Err(ParseError::InvalidSuffix);
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
                *v += params_start as u16;
            }
        }
        params.sort_unstable_by_key(|&[start, end, _, _]| Name(&s[start as usize..end as usize]));

        for window in params.windows(2) {
            let key_a = Name(&s[window[0][0] as usize..window[0][1] as usize]);
            let key_b = Name(&s[window[1][0] as usize..window[1][1] as usize]);
            if key_a == key_b {
                return Err(ParseError::DuplicatedParamKeys);
            }
        }

        Ok((
            Self {
                ty: NonZeroU16::new(ty.len() as _).unwrap(),
                subty: NonZeroU16::new(subty.len() as _).unwrap(),
                suffix: suffix.len() as _,
                params: params.into_boxed_slice(),
            },
            params_start + params_len,
        ))
    }
}

fn is_restricted_name(s: &str) -> bool {
    s.len() <= TYPE_NAME_LENGTH_HARD_LIMIT
        && s.starts_with(char::is_alphanumeric)
        && is_restricted_seq(s)
}

fn is_restricted_seq(s: &str) -> bool {
    s.chars().all(is_restricted_char)
}

fn is_restricted_char(c: char) -> bool {
    c.is_alphanumeric()
        || matches!(
            c,
            '!' | '#' | '$' | '&' | '-' | '^' | '_' | '.' | '+' | '%' | '*' | '\''
        )
}

fn parse_params(s: &str) -> Result<(Vec<[u16; 4]>, usize), ParseError> {
    let mut vec = Vec::new();
    let mut offset = 0;
    let mut len = 0;
    for (i, param) in s.trim_end_matches(' ').split_terminator(';').enumerate() {
        let empty = param.chars().all(|c| c == ' ');
        if i > 0 && empty {
            return Err(ParseError::InvalidParams);
        }
        if !empty {
            let (key, value) = parse_param(param)?;
            vec.push([
                (offset + key.start) as u16,
                (offset + key.end) as u16,
                (offset + value.start) as u16,
                (offset + value.end) as u16,
            ]);
            len = offset + value.end;
        }
        offset += param.len() + 1;
    }
    Ok((vec, len))
}

fn parse_param(s: &str) -> Result<(Range<usize>, Range<usize>), ParseError> {
    if let Some((key, value)) = s.split_once('=') {
        let key_trimmed = key.trim_start_matches(' ').len();
        let key_start = key.len() - key_trimmed;
        let key_range = key_start..key_start + key_trimmed;
        if !is_restricted_name(&s[key_range.clone()]) {
            return Err(ParseError::InvalidParamKey);
        }

        let value_trimmed = value.trim_end_matches(' ').len();
        let value_end = key.len() + 1 + value_trimmed;
        let value = &s[value_end - value_trimmed..value_end];

        let value_range = if value.len() > 1 && value.starts_with('"') && value.ends_with('"') {
            value_end - value_trimmed + 1..value_end - 1
        } else {
            value_end - value_trimmed..value_end
        };
        if !is_restricted_seq(&s[value_range.clone()]) {
            return Err(ParseError::InvalidParamValue);
        }

        Ok((key_range, value_range))
    } else {
        Err(ParseError::InvalidParams)
    }
}

/// Parsing error.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ParseError {
    TooLongInput,
    InvalidTypeName,
    InvalidSubtypeName,
    InvalidSuffix,
    InvalidParams,
    InvalidParamKey,
    InvalidParamValue,
    DuplicatedParamKeys,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::TooLongInput => "Too long input",
            Self::InvalidTypeName => "Invalid type name",
            Self::InvalidSubtypeName => "Invalid subtype name",
            Self::InvalidSuffix => "Invalid suffix",
            Self::InvalidParams => "Invalid params",
            Self::InvalidParamKey => "Invalid param key",
            Self::InvalidParamValue => "Invalid param value",
            Self::DuplicatedParamKeys => "Duplicated param keys",
        };
        f.write_str(msg)
    }
}

impl error::Error for ParseError {}
