//! Explicit declaration intent. Kernel handle identity and live authorization
//! remain the responsibility of the selected runtime consumer.
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

macro_rules! operations {
    ($($name:ident { $($operation:ident),+ $(,)? });+ $(;)?) => {$ (
        #[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
        #[serde(rename_all = "snake_case")]
        pub enum $name { $($operation),+ }
    )+};
}
operations! {
    Creation { Flat, Nest, Clone, Share, Sign };
    Clone { Push, Pull, Peek, Delete };
    Share { Open, Shut, Associate, Delete };
    Sign { Listen, Trigger, Route, Consume, Delete };
    Flat { Inspect, Transfer, Associate, Share, Delete, Block };
    Nest { Inspect, Reserve, Release, Attach, Transfer, Publish, Retract, Protect, Delete };
    Memo { Inspect, Insert, Remove, Publish, Retract, Protect, Delete };
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Operations {
    None(Vec<Creation>),
    Clone(Vec<Clone>),
    Share(Vec<Share>),
    Sign(Vec<Sign>),
    Flat(Vec<Flat>),
    Nest(Vec<Nest>),
    Memo(Vec<Memo>),
}
impl Operations {
    fn valid(&self) -> bool {
        fn distinct<T: PartialEq>(values: &[T]) -> bool {
            !values.is_empty()
                && values
                    .iter()
                    .enumerate()
                    .all(|(i, value)| !values[..i].contains(value))
        }
        match self {
            Self::None(values) => distinct(values),
            Self::Clone(values) => distinct(values),
            Self::Share(values) => distinct(values),
            Self::Sign(values) => distinct(values),
            Self::Flat(values) => distinct(values),
            Self::Nest(values) => distinct(values),
            Self::Memo(values) => distinct(values),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Window {
    #[default]
    Inherit,
    Pages {
        #[serde(with = "super::number")]
        offset: u64,
        #[serde(with = "super::number")]
        count: u64,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Rights {
    pub operations: Operations,
    #[serde(default)]
    pub modifiable: bool,
    #[serde(default)]
    pub distributable: bool,
    #[serde(default)]
    pub window: Window,
}
impl Rights {
    pub fn valid(&self) -> bool {
        if !self.operations.valid()
            || self.modifiable
                && matches!(self.operations, Operations::None(_) | Operations::Sign(_))
        {
            return false;
        }
        match self.window {
            Window::Inherit => true,
            Window::Pages { offset, count } => {
                matches!(
                    self.operations,
                    Operations::Share(_) | Operations::Flat(_) | Operations::Memo(_)
                ) && count != 0
                    && offset
                        .checked_add(count)
                        .and_then(|n| n.checked_mul(4096))
                        .is_some()
            }
        }
    }
}
