use crate::{Construction, Dependency, Ending, Ready, Retirement};
use alloc::vec::Vec;

pub struct ControlConfig {
    pub creation: crate::document::Creation,
    pub after: Option<Vec<Dependency>>,
    pub ending: Ending,
    pub ready: Ready,
    pub construction: Vec<Construction>,
    pub retirement: Vec<Retirement>,
    pub supervision: Option<&'static str>,
    pub groups: Vec<&'static str>,
    pub guarded: bool,
}
