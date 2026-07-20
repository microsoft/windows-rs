use crate::bindings;

/// The kinds of work a [`CompositionScopedBatch`] can track for completion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BatchKind {
    /// Track key-frame and expression animations.
    Animation,
    /// Track effect loads.
    Effect,
    /// Track infinitely-repeating animations.
    InfiniteAnimation,
    /// Track all animations, including infinite ones.
    AllAnimations,
}

impl From<BatchKind> for bindings::CompositionBatchTypes {
    fn from(kind: BatchKind) -> Self {
        match kind {
            BatchKind::Animation => Self::Animation,
            BatchKind::Effect => Self::Effect,
            BatchKind::InfiniteAnimation => Self::InfiniteAnimation,
            BatchKind::AllAnimations => Self::AllAnimations,
        }
    }
}

/// Groups the animations started while it is open so they can be sealed
/// together with [`end`](Self::end).
///
/// Create the batch, start the animations, then call [`end`](Self::end) to seal
/// it so no later work is added to the group.
pub struct CompositionScopedBatch(pub(crate) bindings::CompositionScopedBatch);

impl CompositionScopedBatch {
    /// Seals the batch. No further work started after this call is tracked by
    /// the batch.
    pub fn end(&self) {
        self.0.End().unwrap();
    }
}
