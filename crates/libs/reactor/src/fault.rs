//! Fatal boundary for panics that would otherwise cross a WinUI callback.

use std::panic::AssertUnwindSafe;

use super::diagnostics;

/// Run `f` and abort after reporting any panic under `context`.
pub(crate) fn abort_on_panic<T>(context: &'static str, f: impl FnOnce() -> T) -> T {
    match std::panic::catch_unwind(AssertUnwindSafe(f)) {
        Ok(value) => value,
        Err(payload) => abort(context, &*payload),
    }
}

/// Report an explicit failure from deferred best-effort work that cannot return
/// its `Result` to the caller.
pub(crate) fn report(context: &'static str, message: String) {
    diagnostics::emit(&format!("windows_reactor: {context} failed: {message}"));
}

#[cold]
fn abort(context: &'static str, payload: &(dyn std::any::Any + Send)) -> ! {
    let message = diagnostics::format_panic_payload(payload);
    diagnostics::emit(&format!(
        "windows_reactor: {context} panicked: {message}; aborting"
    ));
    std::process::abort()
}
