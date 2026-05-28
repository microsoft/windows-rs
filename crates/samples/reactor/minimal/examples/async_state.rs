//! Minimal sample for the `cx.use_async_state` hook (gap H5,
//! off-UI-thread setter auto-marshal).
//!
//! Clicking the button spawns a background `std::thread::spawn`, sleeps
//! briefly to simulate work, and then calls the setter. The
//! `AsyncSetState` is `Send + Sync`, so it crosses into the thread
//! freely; on call, the value write + rerender request are auto-marshalled
//! back onto the UI thread via the host's `UiMarshaller`.
//!
//! The button is disabled while `busy` is `true`, so the captured
//! `count` snapshot can't go stale between rapid clicks — each click is
//! only re-enabled after the previous spawn's `set_count(current + 1)`
//! has already been marshalled back and the component has re-rendered
//! with the new value.

use std::thread;
use std::time::Duration;

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> impl Into<Element> {
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
                // Off the UI thread: simulate some background work.
                thread::sleep(Duration::from_millis(500));
                // These setters auto-marshal back to the UI thread.
                // set_count fires first so the next render sees the
                // new value before the button is re-enabled.
                set_count.call(current + 1);
                set_busy.call(false);
            });
        }
    };

    let mut bump_button = button("Bump (off-thread)").on_click(bump);
    if busy {
        bump_button = bump_button.enabled(false);
    }

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
        bump_button,
    ))
    .spacing(8.0)
}

fn main() -> Result<()> {
    App::new().title("async_state").render(app)
}
