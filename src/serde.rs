#![cfg(feature = "serde")]

use super::{media_type::*, media_type_buf::*};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl<'a> Serialize for MediaType<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MediaType<'de> {
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
        let s: &str = Deserialize::deserialize(deserializer)?;
        s.parse().map_err(Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn serde() {
        let original = MediaTypeBuf::from_str("image/svg+xml; charset=UTF-8;").unwrap();
        let json = serde_json::to_string(&original).unwrap();
        let decoded: MediaType = serde_json::from_str(&json).unwrap();
        assert_eq!(original, decoded);
    }
}
