#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CommitReceipt {
    pub(crate) command_count: usize,
}

#[derive(Debug)]
pub(crate) struct RuntimeError;

pub(crate) trait NativeRuntime {
    fn apply(&mut self, commands: &[Command]) -> Result<CommitReceipt, RuntimeError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Command {}
