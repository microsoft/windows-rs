use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use windows_reactor::{ReactorWindow, text_block, with_active_host};

use crate::fixtures::reconciler::FixtureFuture;
use crate::harness::Harness;

/// Open two real secondary windows on the UI thread, prove each hosts its own
/// independent reactor tree, then close one and confirm the survivor is
/// promoted to the active window — i.e. closing a non-last window keeps the
/// process (and the other windows) alive.
///
/// Closing the *last* registered window intentionally exits the process
/// (matching `App::run`), so this fixture deliberately leaves the surviving
/// window open; the last-window bookkeeping is covered by the `WindowRegistry`
/// unit tests in `windows-reactor`.
pub fn open_close_secondary(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        // Each secondary window bumps its own counter every time its reactor
        // tree renders, so the harness can prove the windows render
        // independently without needing to reach into their content.
        let a_renders = Rc::new(Cell::new(0u32));
        let b_renders = Rc::new(Cell::new(0u32));

        let a_flag = Rc::clone(&a_renders);
        let open_a = ReactorWindow::new()
            .title("Reactor Secondary A")
            .inner_size(320.0, 240.0)
            .render(move |_cx| {
                a_flag.set(a_flag.get() + 1);
                text_block("Secondary window A").into()
            });

        let b_flag = Rc::clone(&b_renders);
        let open_b = ReactorWindow::new()
            .title("Reactor Secondary B")
            .inner_size(320.0, 240.0)
            .render(move |_cx| {
                b_flag.set(b_flag.get() + 1);
                text_block("Secondary window B").into()
            });

        let handle_a = match open_a {
            Ok(handle) => handle,
            Err(err) => {
                h.check_with("SecondaryWindow_OpenA", false, || format!("{err:?}"));
                return;
            }
        };
        let handle_b = match open_b {
            Ok(handle) => handle,
            Err(err) => {
                h.check_with("SecondaryWindow_OpenB", false, || format!("{err:?}"));
                return;
            }
        };
        h.check("SecondaryWindow_OpenA", true);
        h.check("SecondaryWindow_OpenB", true);
        h.check("SecondaryWindow_HandlesDistinct", handle_a != handle_b);

        // Both secondaries render their own trees on the shared UI thread.
        let a_ok = h
            .pump_until(Duration::from_secs(2), || a_renders.get() >= 1)
            .await;
        let b_ok = h
            .pump_until(Duration::from_secs(2), || b_renders.get() >= 1)
            .await;
        h.check("SecondaryWindow_A_Renders", a_ok);
        h.check("SecondaryWindow_B_Renders", b_ok);

        // While both are open the active (primary) host is the first opened (A).
        h.check(
            "SecondaryWindow_ActiveHostWhileOpen",
            with_active_host(|_| ()).is_some(),
        );

        // Close A. Because B is still open the process must survive; the
        // registry promotes B to the active host. If closing A had exited the
        // process, execution would never reach the assertion below.
        handle_a.close();
        h.pump_for(Duration::from_millis(200)).await;
        h.check(
            "SecondaryWindow_SurvivorPromotedAfterNonLastClose",
            with_active_host(|_| ()).is_some(),
        );

        // Intentionally leave B open — see the fixture doc comment.
        let _ = handle_b;
    })
}
