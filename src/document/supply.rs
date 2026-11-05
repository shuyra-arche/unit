use super::SupplyBinding;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Supply<S> {
    #[serde(default)]
    pub bindings: Vec<SupplyBinding<S>>,
}
