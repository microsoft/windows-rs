//! Minimal sample for the `DatePicker` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> impl Into<Element> {
    let (label, set_label) = cx.use_state(String::from("No date picked"));

    let on_date = move |dt: DateTime| {
        set_label.call(format!("Picked: {dt}"));
    };

    vstack((
        date_picker().header("Pick a date").on_changed(on_date),
        text_block(&*label),
    ))
    .spacing(8.0)
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
