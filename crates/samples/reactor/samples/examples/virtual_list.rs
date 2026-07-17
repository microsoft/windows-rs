//! Sample demonstrating UI virtualization for a large `list_view`.
//!
//! The list holds 5,000 rows, yet WinUI only ever realizes the handful of item
//! containers that are actually on screen. The reactor's reconciler mirrors that
//! recycling: row content is created on demand as containers scroll into view and
//! released as they scroll out. Drag-to-reorder is mirrored back into state via
//! `on_reorder`, so a reordered row survives the next render even though the
//! realized element for it may have been recycled.

use windows_reactor::*;

const COUNT: usize = 5_000;

fn app(cx: &mut RenderCx) -> Element {
    let (items, update_items) = cx.use_reducer(
        (0..COUNT)
            .map(|i| format!("Item {i:04}"))
            .collect::<Vec<_>>(),
    );

    vstack((
        text_block(format!("{} items, drag any row to reorder", items.len())).bold(),
        list_view(items, |s, idx| {
            let shade = if idx % 2 == 0 { 245 } else { 230 };
            border(text_block(s.clone()).margin(Thickness::xy(12.0, 6.0)))
                .background(Color::rgb(shade, shade, 250))
        })
        .with_key_selector(|s| s.clone())
        .can_drag_items(true)
        .can_reorder_items(true)
        .allow_drop(true)
        .on_reorder(move |order: Vec<usize>| {
            // Derive the next order from the previous state so the callback
            // doesn't capture a per-render snapshot of all 5,000 rows.
            update_items.call(move |prev| order.iter().map(|i| prev[*i].clone()).collect());
        })
        .height(400.0),
    ))
    .spacing(8.0)
    .max_width(360.0)
    .into()
}

fn main() -> Result<()> {
    bootstrap()?;
    App::new().title("Sample").render(app)
}
