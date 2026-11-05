#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Exit {
    pub cause: super::Cause,
    pub reason: u64,
}

impl From<env::TaskExit> for Exit {
    fn from(exit: env::TaskExit) -> Self {
        Self {
            cause: match exit.cause {
                env::ExitCause::Reap => super::Cause::Reap,
                env::ExitCause::Slay => super::Cause::Slay,
                env::ExitCause::Cascade => super::Cause::Cascade,
                env::ExitCause::Fault => super::Cause::Fault,
            },
            reason: exit.reason as u64,
        }
    }
}
