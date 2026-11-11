use super::Publication;
use crate::document::{Permit, Placement, Scope};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(
    deny_unknown_fields,
    bound(deserialize = "S: Deserialize<'de> + Default")
)]
pub struct Namespace<S> {
    pub key: S,
    #[serde(default)]
    pub placement: Placement,
    #[serde(default)]
    pub path: S,
    #[serde(default)]
    pub scope: Scope<S>,
    pub ops: u16,
    #[serde(default)]
    pub permit: Option<Permit>,
    #[serde(default, with = "crate::document::number::optional")]
    pub source_mark: Option<u64>,
    #[serde(default)]
    pub bindings: Vec<Publication<S>>,
}
