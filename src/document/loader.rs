use super::Space;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Loader<S> {
    pub image: S,
    #[serde(default)]
    pub space: Space,
    #[serde(default, with = "super::number::sequence")]
    pub args: Vec<u64>,
    #[serde(default, with = "super::number")]
    pub stack: u64,
}
