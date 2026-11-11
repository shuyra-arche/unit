use crate::document::Rights;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SupplyBinding<S> {
    pub source: S,
    pub resource: S,
    #[serde(with = "crate::document::number")]
    pub slot: u64,
    pub rights: Rights,
}
