use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Arguments {
    pub min: u8,
    pub max: u8,
}

impl Default for Arguments {
    fn default() -> Self {
        Self { min: 0, max: 64 }
    }
}
