//! Demonstrates `cx.use_resource` — fetch data on a background thread
//! with automatic loading/error/ready states and dep-driven refetch.

use std::thread;
use std::time::Duration;

use windows_reactor::*;

fn fetch_page(page: i32) -> std::result::Result<Vec<String>, String> {
    thread::sleep(Duration::from_secs(1));
    Ok((0..5)
        .map(|i| format!("Item {} (page {})", page * 5 + i + 1, page + 1))
        .collect())
}

fn app(cx: &mut RenderCx) -> Element {
    let (page, set_page) = cx.use_state(0_i32);

    let items = cx.use_resource(fetch_page, page);

    let next = {
        let set_page = set_page.clone();
        move || set_page.call(page + 1)
    };

    let prev = {
        let set_page = set_page;
        move || {
            if page > 0 {
                set_page.call(page - 1);
            }
        }
    };

    let content: Element = match &items {
        Resource::Loading => ProgressRing::indeterminate().into(),
        Resource::Ready(data) => vstack(
            data.iter()
                .map(|item| -> Element { text_block(item).into() })
                .collect::<Vec<Element>>(),
        )
        .into(),
        Resource::Error(e) => text_block(format!("Error: {e}")).into(),
        _ => text_block("...").into(),
    };

    vstack((
        text_block(format!("Page {}", page + 1)).font_size(24.0),
        content,
        hstack((
            button("Previous").enabled(page > 0).on_click(prev),
            button("Next").on_click(next),
        )),
    ))
    .spacing(12.0)
    .into()
}

fn main() -> Result<()> {
    App::new().title("use_resource").render(app)
}
