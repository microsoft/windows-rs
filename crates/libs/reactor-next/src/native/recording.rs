use crate::runtime::{Command, CommitReceipt, NativeRuntime, RuntimeError};

#[derive(Default)]
pub(crate) struct RecordingRuntime {
    batches: usize,
}

impl NativeRuntime for RecordingRuntime {
    fn apply(&mut self, commands: &[Command]) -> Result<CommitReceipt, RuntimeError> {
        self.batches += 1;
        Ok(CommitReceipt {
            command_count: commands.len(),
        })
    }
}

#[test]
fn applies_empty_batch() {
    let mut runtime = RecordingRuntime::default();
    let receipt = runtime.apply(&[]).unwrap();

    assert_eq!(receipt.command_count, 0);
    assert_eq!(runtime.batches, 1);
}
