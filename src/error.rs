use std::{error, fmt};

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
