#![windows_subsystem = "windows"]

use windows_reactor::{CalendarSelectionMode, CalendarView, Element, RenderCx, TextBlock, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let selected = cx.use_state(Vec::new);
    let count = cx.use_state(|| 0_u32);
    let update_selected = selected.clone();
    let update_count = count.clone();

    vstack(
        8.0,
        [
            CalendarView::new(selected.value(), move |dates| {
                update_selected.set(dates);
                update_count.set(update_count.value() + 1);
            })
            .selection_mode(CalendarSelectionMode::Multiple)
            .build(),
            TextBlock::new(format!("Selection changed {} time(s)", count.value())).build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("CalendarView", app)
}
