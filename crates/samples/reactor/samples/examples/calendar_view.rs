//! Sample for the `CalendarView` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0_u32);

    let bump = move || set_count.call(count + 1);

    vstack((
        calendar_view().on_selected_dates_changed(bump),
        text_block(format!("Selection changed {count} time(s)")),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("CalendarView", app)
}
