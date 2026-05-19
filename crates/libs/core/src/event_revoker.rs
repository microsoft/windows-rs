use super::*;
use core::ffi::c_void;

/// A handle that automatically revokes an event registration when dropped.
///
/// Obtained by calling an event-registration method generated with the
/// `--minimal` bindgen option. The registration is revoked when the
/// `EventRevoker` is dropped.
///
/// Call [`into_token`] to take back the raw token and prevent the automatic
/// revocation, which is useful for interoperating with code that manages
/// registration tokens directly.
///
/// [`into_token`]: EventRevoker::into_token
pub struct EventRevoker<I: Interface> {
    source: I,
    token: i64,
    remove: unsafe extern "system" fn(*mut c_void, i64) -> HRESULT,
}

impl<I: Interface> EventRevoker<I> {
    #[doc(hidden)]
    pub fn new(
        source: I,
        token: i64,
        remove: unsafe extern "system" fn(*mut c_void, i64) -> HRESULT,
    ) -> Self {
        Self {
            source,
            token,
            remove,
        }
    }

    /// Consumes the revoker and returns the raw registration token without
    /// revoking the event handler.
    ///
    /// After this call the automatic revocation on drop is cancelled. The
    /// caller is responsible for passing the returned token to the
    /// corresponding `Remove*` method when the handler is no longer needed.
    pub fn into_token(self) -> i64 {
        let mut this = core::mem::ManuallyDrop::new(self);
        let token = this.token;
        // Release the source interface reference without calling Remove*.
        unsafe { core::ptr::drop_in_place(&mut this.source) };
        token
    }
}

impl<I: Interface> Drop for EventRevoker<I> {
    fn drop(&mut self) {
        // Best-effort: discard errors silently (Drop cannot return Result).
        unsafe {
            let _ = (self.remove)(self.source.as_raw(), self.token);
        }
    }
}
