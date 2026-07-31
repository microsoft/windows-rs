use super::*;

// CreateDispatcherQueueController is absent from the Win32 metadata. The WinRT
// runtime class can create only a dedicated-thread queue.
#[repr(C)]
struct DispatcherQueueOptions {
    size: u32,
    thread_type: i32,
    apartment_type: i32,
}

// DISPATCHERQUEUE_THREAD_TYPE::DQTYPE_THREAD_CURRENT
const DQTYPE_THREAD_CURRENT: i32 = 2;
// DISPATCHERQUEUE_THREAD_APARTMENTTYPE::DQTAT_COM_ASTA also initializes COM.
const DQTAT_COM_ASTA: i32 = 1;

windows_core::link!("coremessaging.dll" "system" fn CreateDispatcherQueueController(options: DispatcherQueueOptions, controller: *mut *mut core::ffi::c_void) -> windows_core::HRESULT);

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
