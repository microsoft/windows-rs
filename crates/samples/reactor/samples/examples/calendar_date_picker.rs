#![windows_subsystem = "windows"]

use windows_reactor::{CalendarDatePicker, DateTime, Element, RenderCx, TextBlock, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let selected = cx.use_state(|| None::<DateTime>);
    let current = selected.value();
    let label = current.map_or_else(
        || "Pick a date to see days from today".to_string(),
        |value| {
            let now = DateTime::now();
            match value.checked_duration_since(now) {
                Some(span) => {
                    let days = span.whole_days();
                    match days.cmp(&0) {
                        std::cmp::Ordering::Greater => format!("{days} day(s) from now"),
                        std::cmp::Ordering::Less => format!("{} day(s) ago", days.abs()),
                        std::cmp::Ordering::Equal => "That's today!".to_string(),
                    }
                }
                None => "Date too far away to compute".to_string(),
            }
        },
    );

    vstack(
        8.0,
        [
            CalendarDatePicker::new(current, move |value| {
                selected.set(value);
            })
            .header("Select a date")
            .placeholder_text("Choose...")
            .build(),
            TextBlock::new(label).build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("CalendarDatePicker", app)
}
