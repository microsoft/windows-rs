//! Central fault boundary for panics that cross a WinUI callback boundary.
//!
//! Reactor closures run behind `extern "system"` WinUI delegates that cannot
//! unwind: a panic reaching one aborts the process. This module catches panics
//! at the reactor-owned entry points (event handlers, timers, the render pass)
//! and routes them to a developer-supplied handler installed via
//! [`App::on_fault`](crate::App::on_fault), defaulting to log-and-continue.
//!
//! The catch is *context-aware*: callbacks invoked during a render pass are left
//! to propagate so that [`error_boundary`](crate::error_boundary) can recover the
//! affected subtree, while callbacks invoked outside render (events, timer ticks)
//! are caught and reported directly.

use std::cell::{Cell, RefCell};
use std::panic::AssertUnwindSafe;
use std::rc::Rc;

use super::diagnostics;

thread_local! {
    static IN_RENDER: Cell<bool> = const { Cell::new(false) };
    static HANDLER: RefCell<Option<Rc<dyn Fn(&Fault)>>> = const { RefCell::new(None) };
}

/// Information about a panic caught at a reactor callback boundary.
pub struct Fault {
    /// Where the fault was caught, e.g. `"event handler"`, `"render"`, `"timer"`.
    pub context: &'static str,
    /// The panic message.
    pub message: String,
}

/// Install the per-thread fault handler. Called by [`App::on_fault`](crate::App::on_fault)
/// on the UI thread before the first render.
pub(crate) fn set_handler<F: Fn(&Fault) + 'static>(handler: F) {
    HANDLER.with(|slot| *slot.borrow_mut() = Some(Rc::new(handler)));
}

/// Run `f`, catching a panic and routing it to the fault handler.
///
/// During a render pass this is a no-op wrapper: the panic is allowed to
/// propagate so [`error_boundary`](crate::error_boundary) (or [`render_scope`])
/// can handle it. Outside render it catches the panic and reports it under
/// `context`.
pub(crate) fn catch<F: FnOnce()>(context: &'static str, f: F) {
    if IN_RENDER.with(Cell::get) {
        f();
        return;
    }
    if let Err(payload) = std::panic::catch_unwind(AssertUnwindSafe(f)) {
        dispatch(context, &*payload);
    }
}

/// Run the render pass `f` with the render guard set, catching any panic that
/// escapes an [`error_boundary`](crate::error_boundary) and routing it to the
/// fault handler under the `"render"` context.
pub(crate) fn render_scope<F: FnOnce()>(f: F) {
    let previous = IN_RENDER.replace(true);
    let result = std::panic::catch_unwind(AssertUnwindSafe(f));
    IN_RENDER.set(previous);
    if let Err(payload) = result {
        dispatch("render", &*payload);
    }
}

/// Report an explicit failure (not a panic) to the fault handler. Used for
/// deferred best-effort work that runs behind a WinUI callback and therefore
/// cannot return its `Result` to the caller (e.g. applying the window icon,
/// backdrop, or presenter during `activate`).
pub(crate) fn report(context: &'static str, message: String) {
    deliver(&Fault { context, message });
}

fn dispatch(context: &'static str, payload: &(dyn std::any::Any + Send)) {
    deliver(&Fault {
        context,
        message: diagnostics::format_panic_payload(payload),
    });
}

fn deliver(fault: &Fault) {
    let handler = HANDLER.with(|slot| slot.borrow().clone());
    match handler {
        Some(handler) => {
            let _ = std::panic::catch_unwind(AssertUnwindSafe(|| handler(fault)));
        }
        None => diagnostics::emit(&format!(
            "windows_reactor: {} fault: {}",
            fault.context, fault.message
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    fn install_recorder() -> Rc<RefCell<Vec<(&'static str, String)>>> {
        let log: Rc<RefCell<Vec<(&'static str, String)>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        set_handler(move |fault: &Fault| {
            sink.borrow_mut()
                .push((fault.context, fault.message.clone()));
        });
        log
    }

    #[test]
    fn catch_outside_render_routes_to_handler() {
        let log = install_recorder();
        catch("event handler", || panic!("boom"));
        let entries = log.borrow();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].0, "event handler");
        assert_eq!(entries[0].1, "boom");
    }

    #[test]
    fn catch_returns_normally_without_panic() {
        let log = install_recorder();
        let mut ran = false;
        catch("event handler", || ran = true);
        assert!(ran);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn catch_during_render_defers_to_render_scope() {
        let log = install_recorder();
        render_scope(|| {
            catch("event handler", || panic!("nested"));
        });
        let entries = log.borrow();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].0, "render");
        assert_eq!(entries[0].1, "nested");
        assert!(!IN_RENDER.with(Cell::get));
    }

    #[test]
    fn render_scope_resets_guard_after_panic() {
        let _log = install_recorder();
        render_scope(|| panic!("render boom"));
        assert!(!IN_RENDER.with(Cell::get));
    }

    #[test]
    fn a_panicking_handler_does_not_escape() {
        set_handler(|_| panic!("handler explodes"));
        catch("event handler", || panic!("boom"));
    }
}
