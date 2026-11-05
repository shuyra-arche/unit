use super::Permit;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Publication<S> {
    pub name: S,
    #[serde(with = "super::number")]
    pub mark: u64,
    pub view: u32,
    pub permit: Permit,
}
