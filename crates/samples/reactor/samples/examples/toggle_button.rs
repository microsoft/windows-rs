//! Sample for the `ToggleButton` widget.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (is_bold, set_bold) = cx.use_state(false);
    let (is_italic, set_italic) = cx.use_state(false);

    let on_bold = move |v: bool| set_bold.call(v);
    let on_italic = move |v: bool| set_italic.call(v);

    let style_label = match (is_bold, is_italic) {
        (true, true) => "Bold + Italic",
        (true, false) => "Bold",
        (false, true) => "Italic",
        (false, false) => "Normal",
    };

    vstack((
        toggle_button("Bold", is_bold).on_checked(on_bold),
        toggle_button("Italic", is_italic).on_checked(on_italic),
        text_block(format!("Style: {style_label}")),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("ToggleButton", app)
}
