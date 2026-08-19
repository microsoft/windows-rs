#![windows_subsystem = "windows"]

use std::thread;
use std::time::Duration;

use windows_reactor::{
    Button, CancellationToken, Element, ProgressRing, RenderCx, Resource, TextBlock, hstack, vstack,
};

fn load_page(cancel: CancellationToken, page: i32) -> windows_core::Result<Vec<String>> {
    for _ in 0..40 {
        if cancel.is_cancelled() {
            return Err(windows_core::Error::new(
                windows_core::HRESULT(0x80004004_u32 as i32),
                "resource load cancelled",
            ));
        }
        thread::sleep(Duration::from_millis(25));
    }

    Ok((0..5)
        .map(|index| format!("Item {} (page {})", page * 5 + index + 1, page + 1))
        .collect())
}

fn app(cx: &mut RenderCx<'_>) -> Element {
    let page = cx.use_state(|| 0_i32);
    let current = page.value();
    let items = cx.use_resource(current, load_page);

    let content = match items {
        Resource::Loading => ProgressRing::indeterminate().build(),
        Resource::Ready(items) => vstack(
            4.0,
            items
                .iter()
                .map(|item| TextBlock::new(item).build())
                .collect::<Vec<_>>(),
        ),
        Resource::Failed(error) => TextBlock::new(format!("Error: {error}")).build(),
    };

    let previous = page.clone();
    let next = page;
    vstack(
        12.0,
        [
            TextBlock::new(format!("Page {}", current + 1))
                .font_size(24.0)
                .build(),
            content,
            hstack(
                8.0,
                [
                    Button::new("Previous")
                        .on_click(move || {
                            previous.update(|value| *value -= 1);
                        })
                        .enabled(current > 0)
                        .build(),
                    Button::new("Next")
                        .on_click(move || {
                            next.update(|value| *value += 1);
                        })
                        .build(),
                ],
            ),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("UseResource", app)
}
