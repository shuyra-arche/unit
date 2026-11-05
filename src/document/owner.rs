use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Owner<S> {
    Caller,
    #[default]
    Selected,
    Task(#[serde(with = "super::number")] u64),
    Unit(S),
}
