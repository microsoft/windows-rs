use super::*;

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
    /// Registers a callback for when all tracked work has completed.
    ///
    /// The returned revoker keeps the callback registered. Dropping it before
    /// completion cancels the subscription.
    pub fn on_completed(&self, handler: impl Fn() + 'static) -> Result<windows_core::EventRevoker> {
        self.0.Completed(move |_, _| handler())
    }

    /// Seals the batch. No further work started after this call is tracked by
    /// the batch.
    pub fn end(&self) {
        self.0.End().unwrap();
    }
}
