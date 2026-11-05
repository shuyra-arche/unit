use super::{Construction, Creation, Ending, Ready, Retirement};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(
    deny_unknown_fields,
    bound(deserialize = "S: Deserialize<'de> + Default")
)]
pub struct Control<S> {
    #[serde(default)]
    pub creation: Creation,
    #[serde(default)]
    pub requires: Vec<S>,
    #[serde(default)]
    pub after: Vec<S>,
    #[serde(default)]
    pub ready: Ready,
    #[serde(default)]
    pub ending: Ending,
    #[serde(default)]
    pub supervision: Option<S>,
    #[serde(default)]
    pub construction: Vec<Construction<S>>,
    #[serde(default)]
    pub retirement: Vec<Retirement<S>>,
    #[serde(default)]
    pub groups: Vec<S>,
    #[serde(default)]
    pub guarded: bool,
}
