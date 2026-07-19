use crate::bindings;
use windows_core::{Interface, Result};

// CreateDispatcherQueueController (dispatcherqueue.h / CoreMessaging) is not part
// of this repo's Win32 metadata, so it is declared here directly. It is the only
// way to stand up a dispatcher queue on the *current* thread — the system
// `DispatcherQueueController` runtime class only offers a dedicated-thread
// factory.
#[repr(C)]
struct DispatcherQueueOptions {
    size: u32,
    thread_type: i32,
    apartment_type: i32,
}

// DISPATCHERQUEUE_THREAD_TYPE::DQTYPE_THREAD_CURRENT
const DQTYPE_THREAD_CURRENT: i32 = 2;
// DISPATCHERQUEUE_THREAD_APARTMENTTYPE::DQTAT_COM_ASTA — initializes this thread's
// apartment, so callers need no separate CoInitialize.
const DQTAT_COM_ASTA: i32 = 1;

windows_core::link!("coremessaging.dll" "system" fn CreateDispatcherQueueController(options: DispatcherQueueOptions, controller: *mut *mut core::ffi::c_void) -> windows_core::HRESULT);

/// Owns a dispatcher queue on the current thread.
///
/// The composition engine requires a `DispatcherQueue` to be present on the
/// thread that creates and drives a [`Compositor`](crate::Compositor). This
/// controller creates that queue and keeps it alive — hold it for as long as the
/// compositor is in use, and pump the thread's message loop so the queue (and
/// the compositor's off-thread animations) run.
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
        let options = DispatcherQueueOptions {
            size: size_of::<DispatcherQueueOptions>() as u32,
            thread_type: DQTYPE_THREAD_CURRENT,
            apartment_type: DQTAT_COM_ASTA,
        };

        unsafe {
            let mut controller = core::ptr::null_mut();
            CreateDispatcherQueueController(options, &mut controller).ok()?;
            Ok(Self(bindings::DispatcherQueueController::from_raw(
                controller,
            )))
        }
    }
}
