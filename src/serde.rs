#![cfg(feature = "serde")]

use super::{media_type::*, media_type_buf::*};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;

impl<'a> Serialize for MediaType<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for MediaType<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let s: &str = Deserialize::deserialize(deserializer)?;
        MediaType::parse(s).map_err(Error::custom)
    }
}

impl Serialize for MediaTypeBuf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MediaTypeBuf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let s: Cow<str> = Deserialize::deserialize(deserializer)?;
        Self::from_string(s.into_owned()).map_err(Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::str::FromStr;

    #[test]
    fn serde() {
        let original = MediaTypeBuf::from_str("image/svg+xml; charset=UTF-8;").unwrap();
        let json = serde_json::to_string(&original).unwrap();
        let decoded: MediaType = serde_json::from_str(&json).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn serde_from_value() {
        let original = MediaTypeBuf::from_str("image/svg+xml; charset=UTF-8;").unwrap();
        let value = Value::String(original.as_str().into());
        let decoded: MediaTypeBuf = serde_json::from_value(value).unwrap();
        assert_eq!(original, decoded);
    }
}
