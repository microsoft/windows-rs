use crate::bindings;
use windows_core::Result;

/// Owns a dispatcher queue on the current thread.
///
/// The lifted composition engine requires a `DispatcherQueue` to be present on
/// the thread that creates and drives a [`Compositor`](crate::Compositor). This
/// controller creates that queue and keeps it alive — hold it for as long as the
/// compositor is in use, and pump the thread's message loop so the queue (and
/// the compositor's off-thread animations) run.
///
/// Like the compositor itself, this requires the Windows App SDK runtime to be
/// bootstrapped (see [`Compositor::new`](crate::Compositor::new)).
pub struct DispatcherQueueController(
    #[expect(
        dead_code,
        reason = "held only to keep the dispatcher queue alive on this thread"
    )]
    bindings::DispatcherQueueController,
);

impl DispatcherQueueController {
    /// Creates a dispatcher queue on the current thread. Fails if the thread
    /// already has one.
    pub fn create_on_current_thread() -> Result<Self> {
        Ok(Self(
            bindings::DispatcherQueueController::CreateOnCurrentThread()?,
        ))
    }
}
