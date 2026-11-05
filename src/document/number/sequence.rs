use super::value::Number;
use alloc::vec::Vec;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
pub fn serialize<S: Serializer>(value: &[u64], serializer: S) -> Result<S::Ok, S::Error> {
    value
        .iter()
        .copied()
        .map(Number)
        .collect::<Vec<_>>()
        .serialize(serializer)
}
pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u64>, D::Error> {
    Vec::<Number>::deserialize(deserializer)
        .map(|values| values.into_iter().map(|number| number.0).collect())
}
