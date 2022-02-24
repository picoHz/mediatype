use std::{error, fmt};

/// Media-type format error.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MediaTypeError {
    /// The top-level type name is not valid.
    InvalidTypeName,
    /// The subtype name is not valid.
    InvalidSubtypeName,
    /// The suffix name is not valid.
    InvalidSuffix,
    /// The parameter syntax is not valid.
    InvalidParams,
    /// An invalid parameter name is detected.
    InvalidParamName,
    /// An invalid parameter value is detected.
    InvalidParamValue,
}

impl fmt::Display for MediaTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::InvalidTypeName => "Invalid type name",
            Self::InvalidSubtypeName => "Invalid subtype name",
            Self::InvalidSuffix => "Invalid suffix",
            Self::InvalidParams => "Invalid params",
            Self::InvalidParamName => "Invalid param name",
            Self::InvalidParamValue => "Invalid param value",
        };
        f.write_str(msg)
    }
}

impl error::Error for MediaTypeError {}
