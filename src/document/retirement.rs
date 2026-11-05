use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Retirement<S> {
    pub provider: S,
    pub namespace: u8,
    pub face: S,
}
