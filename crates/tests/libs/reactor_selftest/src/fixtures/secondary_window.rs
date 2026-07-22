use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use windows_reactor::{ReactorWindow, WindowHandle, text_block, with_active_host};

use crate::fixtures::reconciler::FixtureFuture;
use crate::harness::Harness;

/// End-to-end coverage for the secondary-window feature on the real WinUI
/// backend: opening independent top-level windows (both via the [`ReactorWindow`]
/// builder and via the `use_open_window` hook from inside a component), proving
/// each hosts its own reactor tree, and closing them.
///
/// The fixture opens an *anchor* window that stays open for its whole duration.
/// Closing the last registered window exits the process (matching `App::run`),
/// which would tear down the self-test harness itself, so the anchor keeps the
/// registry non-empty and lets the fixture run full open+close cycles on every
/// other window without tripping that exit. The last-window bookkeeping itself
/// is covered by the `WindowRegistry` unit tests in `windows-reactor`; the
/// anchor is intentionally left open when the fixture returns.
pub fn open_close_secondary(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        // Anchor window — kept open for the whole fixture so the registry never
        // drains to empty (which would exit the process).
        let anchor = match ReactorWindow::new()
            .title("Reactor Anchor")
            .inner_size(320.0, 240.0)
            .render(|_cx| text_block("Anchor window").into())
        {
            Ok(handle) => handle,
            Err(err) => {
                h.check_with("SecondaryWindow_OpenAnchor", false, || format!("{err:?}"));
                return;
            }
        };
        h.check("SecondaryWindow_OpenAnchor", true);
        h.check(
            "SecondaryWindow_ActiveHostAfterAnchor",
            with_active_host(|_| ()).is_some(),
        );

        // --- Independent secondary window via the ReactorWindow builder ---
        // A per-window render counter proves the tree renders independently
        // without reaching into the window's content.
        let b_renders = Rc::new(Cell::new(0u32));
        let b_flag = Rc::clone(&b_renders);
        let handle_b = match ReactorWindow::new()
            .title("Reactor Secondary B")
            .inner_size(320.0, 240.0)
            .render(move |_cx| {
                b_flag.set(b_flag.get() + 1);
                text_block("Secondary window B").into()
            }) {
            Ok(handle) => handle,
            Err(err) => {
                h.check_with("SecondaryWindow_OpenB", false, || format!("{err:?}"));
                return;
            }
        };
        h.check("SecondaryWindow_OpenB", true);
        h.check("SecondaryWindow_HandlesDistinct", anchor != handle_b);

        let b_ok = h
            .pump_until(Duration::from_secs(2), || b_renders.get() >= 1)
            .await;
        h.check("SecondaryWindow_B_Renders", b_ok);

        // Closing B (a non-last window) keeps the process alive; the anchor
        // remains the active host.
        handle_b.close();
        h.pump_for(Duration::from_millis(200)).await;
        h.check(
            "SecondaryWindow_ActiveHostAfterNonLastClose",
            with_active_host(|_| ()).is_some(),
        );

        // --- Secondary window opened via the use_open_window hook ---
        // A launcher component grabs an opener from its RenderCx and, on mount,
        // opens a child window whose reactor tree renders independently. This
        // exercises the documented in-component path (open a window from within
        // another component's render/effect), not just the free builder.
        let child_renders = Rc::new(Cell::new(0u32));
        let child_handle: Rc<RefCell<Option<WindowHandle>>> = Rc::new(RefCell::new(None));

        let child_flag = Rc::clone(&child_renders);
        let child_handle_slot = Rc::clone(&child_handle);
        let launcher = ReactorWindow::new()
            .title("Reactor Launcher")
            .inner_size(320.0, 240.0)
            .render(move |cx| {
                let opener = cx.use_open_window();
                let child_flag = Rc::clone(&child_flag);
                let child_handle_slot = Rc::clone(&child_handle_slot);
                cx.use_effect((), move || {
                    let opened = opener.render(move |_cx| {
                        child_flag.set(child_flag.get() + 1);
                        text_block("Hook-opened child").into()
                    });
                    if let Ok(handle) = opened {
                        *child_handle_slot.borrow_mut() = Some(handle);
                    }
                });
                text_block("Launcher window").into()
            });

        let launcher = match launcher {
            Ok(handle) => handle,
            Err(err) => {
                h.check_with("SecondaryWindow_OpenLauncher", false, || format!("{err:?}"));
                return;
            }
        };
        h.check("SecondaryWindow_OpenLauncher", true);

        // The hook-opened child window mounts and renders its own tree.
        let child_ok = h
            .pump_until(Duration::from_secs(2), || child_renders.get() >= 1)
            .await;
        h.check("SecondaryWindow_HookChildRenders", child_ok);

        // Tear the hook windows back down to just the anchor. Because the anchor
        // is still open these closes never trip the last-window exit, so the
        // fixture exercises the full open -> render -> close lifecycle.
        match *child_handle.borrow() {
            Some(child) => {
                h.check(
                    "SecondaryWindow_HookChildDistinct",
                    child != launcher && child != anchor,
                );
                child.close();
            }
            None => h.check("SecondaryWindow_HookChildDistinct", false),
        }
        launcher.close();
        h.pump_for(Duration::from_millis(200)).await;
        h.check(
            "SecondaryWindow_ActiveHostAfterHookClose",
            with_active_host(|_| ()).is_some(),
        );

        // Intentionally leave the anchor open — see the fixture doc comment.
        let _ = anchor;
    })
}
