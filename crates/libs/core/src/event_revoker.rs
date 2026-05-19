use super::*;
use core::ffi::c_void;

/// A handle that automatically revokes an event registration when dropped.
///
/// Obtained by calling an event-registration method generated with the
/// `--auto-events` bindgen option. The registration is revoked when the
/// `EventRevoker` is dropped, or explicitly via [`EventRevoker::revoke`].
///
/// # Drop behaviour
///
/// When an `EventRevoker` is dropped without calling [`revoke`], it performs a
/// best-effort revocation: errors from the underlying `Remove*` call are
/// silently discarded. Use [`revoke`] when you need to observe errors.
///
/// [`revoke`]: EventRevoker::revoke
pub struct EventRevoker<I: Interface> {
    source: I,
    token: Option<i64>,
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
            token: Some(token),
            remove,
        }
    }

    /// Explicitly revoke the event registration and return any error.
    ///
    /// After this call the registration is cancelled. Calling `revoke` when
    /// the token has already been consumed (e.g. by a previous `revoke` call
    /// or by the `Drop` impl) is a no-op that returns `Ok(())`.
    pub fn revoke(mut self) -> Result<()> {
        if let Some(token) = self.token.take() {
            unsafe { (self.remove)(self.source.as_raw(), token).ok() }
        } else {
            Ok(())
        }
    }
}

impl<I: Interface> Drop for EventRevoker<I> {
    fn drop(&mut self) {
        if let Some(token) = self.token.take() {
            // Best-effort: discard errors silently (Drop cannot return Result).
            unsafe {
                let _ = (self.remove)(self.source.as_raw(), token);
            }
        }
    }
}
