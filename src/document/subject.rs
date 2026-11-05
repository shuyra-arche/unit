use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Subject {
    #[serde(with = "super::number")]
    pub authority: u64,
    #[serde(with = "super::number")]
    pub principal: u64,
    #[serde(with = "super::number::sequence")]
    pub coalitions: Vec<u64>,
}
