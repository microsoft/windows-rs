//! Sample for the `TimePicker` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (label, set_label) = cx.use_state(String::from("No time picked"));

    let on_time = move |ts: TimeSpan| {
        let hours = ts.whole_hours();
        let minutes = ts.whole_minutes() % 60;
        set_label.call(format!("Picked: {hours:02}:{minutes:02}"));
    };

    vstack((
        time_picker()
            .header("Pick a time")
            .minute_increment(15)
            .on_selected_time_changed(on_time),
        text_block(&*label),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("TimePicker", app)
}
