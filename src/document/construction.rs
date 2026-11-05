use super::{Arguments, IdentitySelection, Owner, Range};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(
    deny_unknown_fields,
    bound(deserialize = "S: Deserialize<'de> + Default")
)]
pub struct Construction<S> {
    pub unit: Option<S>,
    #[serde(default)]
    pub owner: Owner<S>,
    #[serde(default)]
    pub identity: IdentitySelection,
    #[serde(default)]
    pub arguments: Arguments,
    #[serde(default)]
    pub stack: Range,
    #[serde(default, with = "super::number")]
    pub max_image: u64,
}
