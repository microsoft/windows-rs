//! Sample for the `RichEditBox` widget.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (text, set_text) = cx.use_state(String::new());

    let on_changed = move |v: String| set_text.call(v);

    vstack((
        rich_edit_box(String::new())
            .header("Rich Editor")
            .placeholder("Type rich text here…")
            .on_changed(on_changed)
            .height(200.0),
        text_block(format!("Plain text: {text}")),
        rich_edit_box("Read-only content.".to_string())
            .header("Read Only")
            .read_only()
            .height(100.0),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("RichEditBox", app)
}
