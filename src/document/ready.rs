use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Ready {
    #[default]
    Spawn,
    Signal(#[serde(with = "super::number")] u64),
}
