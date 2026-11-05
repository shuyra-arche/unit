#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Event<S> {
    Started { system: u64 },
    Bound { unit: S, task: u64, team: u64 },
    Ready { task: u64 },
    Exited { task: u64, exit: super::Exit },
    Retired { task: u64 },
    Complete { system: u64, reason: u64 },
}
