use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentitySelector {
    Exact {
        #[serde(with = "super::number")]
        authority: u64,
        #[serde(with = "super::number")]
        slot: u64,
    },
    DescendantOf {
        #[serde(with = "super::number")]
        authority: u64,
        #[serde(with = "super::number")]
        slot: u64,
    },
    MemberOf {
        #[serde(with = "super::number")]
        authority: u64,
        #[serde(with = "super::number")]
        slot: u64,
    },
}

impl IdentitySelector {
    pub fn valid(&self) -> bool {
        let authority = match self {
            Self::Exact { authority, .. }
            | Self::DescendantOf { authority, .. }
            | Self::MemberOf { authority, .. } => *authority,
        };
        authority != 0 && usize::try_from(authority).is_ok()
    }
}
