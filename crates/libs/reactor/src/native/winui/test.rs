use super::*;
use windows_core::EventRevoker;

pub(crate) fn native_window_handle(window: &Window) -> windows_core::Result<isize> {
    let mut hwnd = std::ptr::null_mut();
    unsafe {
        window
            .cast::<IWindowNative>()?
            .WindowHandle(&mut hwnd)
            .ok()?;
    }
    Ok(hwnd as isize)
}

pub fn subscribe_live_rendering<F>(rendering: F) -> windows_core::Result<EventRevoker>
where
    F: Fn() + 'static,
{
    CompositionTarget::Rendering(move |_, _| {
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(&rendering)).is_err() {
            std::process::abort();
        }
    })
}

pub fn schedule_live_test_exit(success: bool) -> windows_core::Result<()> {
    let dispatcher = DispatcherQueue::GetForCurrentThread()?;
    let handler = DispatcherQueueHandler::new(move || {
        std::process::exit(i32::from(!success));
    });
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Low, &handler)? {
        Ok(())
    } else {
        std::process::exit(1);
    }
}
