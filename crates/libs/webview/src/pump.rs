use super::*;
use std::cell::Cell;
use std::rc::Rc;

/// A one-shot result slot shared between an asynchronous WebView2 completion
/// handler and the [`wait`] pump that drives it.
pub(crate) type Slot<T> = Rc<Cell<Option<Result<T>>>>;

pub(crate) fn slot<T>() -> Slot<T> {
    Rc::new(Cell::new(None))
}

/// Builds a completion handler that stores its result in `slot` for [`wait`].
pub(crate) fn slot_handler<T: 'static>(slot: &Slot<T>) -> impl FnOnce(Result<T>) + 'static {
    let slot = slot.clone();
    move |result| slot.set(Some(result))
}

/// Pumps the calling thread's message loop until `slot` is filled, then returns
/// its result.
///
/// WebView2 delivers completion handlers by posting to the message loop, so a
/// synchronous wrapper has to keep dispatching messages until the handler runs.
/// This must only be called on the UI thread that owns the WebView2 objects.
pub(crate) fn wait<T>(slot: &Slot<T>) -> Result<T> {
    let mut message = MSG::default();

    loop {
        if let Some(value) = slot.take() {
            return value;
        }

        unsafe {
            match GetMessageW(&mut message, std::ptr::null_mut(), 0, 0).0 {
                -1 => return Err(Error::from_thread()),
                0 => return Err(Error::empty()),
                _ => {
                    let _ = TranslateMessage(&message);
                    DispatchMessageW(&message);
                }
            }
        }
    }
}
