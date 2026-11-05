use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Range {
    #[serde(with = "super::number")]
    pub min: u64,
    #[serde(with = "super::number")]
    pub max: u64,
}

impl Default for Range {
    fn default() -> Self {
        Self {
            min: 0,
            max: 1 << 20,
        }
    }
}
