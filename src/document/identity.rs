use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Identity<S> {
    #[serde(default)]
    pub aliases: Vec<S>,
    #[serde(default)]
    pub grants: Vec<u8>,
}
