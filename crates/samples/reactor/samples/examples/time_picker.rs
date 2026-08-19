#![windows_subsystem = "windows"]

use windows_reactor::{Element, RenderCx, TextBlock, TimePicker, TimeSpan, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let selected = cx.use_state(|| None::<TimeSpan>);
    let current = selected.value();
    let label = current.map_or_else(
        || "No time picked".to_string(),
        |time| {
            let hours = time.whole_hours();
            let minutes = time.whole_minutes() % 60;
            format!("Picked: {hours:02}:{minutes:02}")
        },
    );

    vstack(
        8.0,
        [
            TimePicker::new(current, move |value| {
                selected.set(value);
            })
            .header("Pick a time")
            .minute_increment(15)
            .build(),
            TextBlock::new(label).build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("TimePicker", app)
}
