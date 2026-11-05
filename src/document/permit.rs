use super::IdentitySelector;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Permit {
    Public,
    Bound,
    Identity(IdentitySelector),
    Opener(#[serde(with = "super::number")] u64),
}

impl Permit {
    pub fn valid(&self) -> bool {
        match self {
            Self::Public | Self::Bound => true,
            Self::Identity(selector) => selector.valid(),
            Self::Opener(entry) => usize::try_from(*entry).is_ok(),
        }
    }
}
