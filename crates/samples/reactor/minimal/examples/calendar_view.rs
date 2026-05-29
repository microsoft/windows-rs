//! Minimal sample for the `CalendarView` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> impl Into<Element> {
    let (count, set_count) = cx.use_state(0_u32);

    let bump = move || set_count.call(count + 1);

    vstack((
        calendar_view().today_highlighted(true).on_changed(bump),
        text_block(format!("Selection changed {count} time(s)")),
    ))
    .spacing(8.0)
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
