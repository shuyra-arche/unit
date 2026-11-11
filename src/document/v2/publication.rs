use crate::document::{Permit, Rights};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Publication<S> {
    pub name: S,
    #[serde(with = "crate::document::number")]
    pub mark: u64,
    pub rights: Rights,
    pub permit: Permit,
}
