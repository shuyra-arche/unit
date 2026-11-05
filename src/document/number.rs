pub mod optional;
pub mod sequence;
mod value;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use value::Number;

pub fn serialize<S: Serializer>(value: &u64, serializer: S) -> Result<S::Ok, S::Error> {
    Number(*value).serialize(serializer)
}
pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
    Number::deserialize(deserializer).map(|number| number.0)
}
