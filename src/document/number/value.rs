use alloc::string::ToString;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
};

pub(super) struct Number(pub u64);
impl Serialize for Number {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.0.to_string())
        } else {
            serializer.serialize_u64(self.0)
        }
    }
}
impl<'de> Deserialize<'de> for Number {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            deserializer.deserialize_any(Self(0))
        } else {
            u64::deserialize(deserializer).map(Self)
        }
    }
}
impl<'de> Visitor<'de> for Number {
    type Value = Self;
    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("an unsigned integer or quoted decimal/0x hexadecimal u64")
    }
    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self, E> {
        Ok(Self(value))
    }
    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self, E> {
        u64::try_from(value).map(Self).map_err(E::custom)
    }
    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self, E> {
        let parsed = match value.strip_prefix("0x") {
            Some(hex) => u64::from_str_radix(hex, 16),
            None => value.parse(),
        };
        parsed.map(Self).map_err(E::custom)
    }
}
