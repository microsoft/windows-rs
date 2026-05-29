//! Minimal sample for the `TextBox` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> impl Into<Element> {
    let (name, set_name) = cx.use_state(String::new());
    let (notes, set_notes) = cx.use_state(String::new());

    let update_name = move |v: String| set_name.call(v);
    let update_notes = move |v: String| set_notes.call(v);

    vstack((
        text_box(name.clone())
            .header("Display name")
            .placeholder("Type your name…")
            .on_changed(update_name),
        text_block(format!(
            "Hello, {}!",
            if name.is_empty() {
                "stranger"
            } else {
                name.as_str()
            }
        )),
        text_box(notes)
            .header("Notes")
            .placeholder("Write something longer…")
            .multiline()
            .height(100.0)
            .on_changed(update_notes),
        text_box("read-only").header("Disabled").enabled(false),
    ))
    .spacing(8.0)
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
