//! Minimal sample for the `cx.use_async_state` hook.
//!
//! `AsyncSetState` is `Send + Sync` — call it from any thread and the
//! value write + rerender are auto-marshalled back to the UI thread.

use std::thread;
use std::time::Duration;

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_async_state(0_i32);
    let (busy, set_busy) = cx.use_async_state(false);

    let bump = {
        let set_count = set_count;
        let set_busy = set_busy;
        move || {
            set_busy.call(true);
            let set_count = set_count.clone();
            let set_busy = set_busy.clone();
            let current = count;
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(500));
                set_count.call(current + 1);
                set_busy.call(false);
            });
        }
    };

    vstack((
        text_block(format!("count = {count}"))
            .font_size(24.0)
            .bold(),
        text_block(if busy {
            "working off the UI thread…".to_string()
        } else {
            "idle".to_string()
        })
        .font_size(12.0)
        .opacity(0.7),
        button("Bump (off-thread)").on_click(bump).enabled(!busy),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    let _bootstrap_handle = windows_reactor::bootstrap::initialize()?;
    App::new().title("async_state").render(app)
}
