use super::*;

/// Owns a dispatcher queue on the current thread.
///
/// Keep it alive and pump the thread's message loop while using a
/// [`Compositor`](crate::Compositor) created on that thread.
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
        let options = bindings::DispatcherQueueOptions {
            dwSize: size_of::<bindings::DispatcherQueueOptions>() as u32,
            threadType: bindings::DQTYPE_THREAD_CURRENT,
            apartmentType: bindings::DQTAT_COM_ASTA,
        };

        unsafe {
            let mut controller = core::ptr::null_mut();
            bindings::CreateDispatcherQueueController(options, &mut controller).ok()?;
            Ok(Self(bindings::DispatcherQueueController::from_raw(
                controller,
            )))
        }
    }
}
