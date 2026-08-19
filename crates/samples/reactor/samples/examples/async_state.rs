#![windows_subsystem = "windows"]

use std::thread;
use std::time::Duration;

use windows_reactor::{Button, Element, RenderCx, TextBlock, vstack};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let (count, set_count) = cx.use_async_state(0_i32);
    let (busy, set_busy) = cx.use_async_state(false);

    vstack(
        8.0,
        [
            TextBlock::new(format!("count = {count}"))
                .font_size(24.0)
                .build(),
            TextBlock::new(if busy {
                "working off the UI thread..."
            } else {
                "idle"
            })
            .font_size(12.0)
            .opacity(0.7)
            .build(),
            Button::new("Bump (off-thread)")
                .on_click(move || {
                    set_busy.set(true);
                    let set_count = set_count.clone();
                    let set_busy = set_busy.clone();
                    thread::spawn(move || {
                        thread::sleep(Duration::from_millis(500));
                        set_count.set(count + 1);
                        set_busy.set(false);
                    });
                })
                .enabled(!busy)
                .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Async State", app)
}
