#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Cause {
    Reap,
    Slay,
    Cascade,
    Fault,
}
