use super::Subject;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentitySelection {
    Caller,
    #[default]
    Selected,
    Exact(Subject),
}
