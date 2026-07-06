//! Sample for `TextBox` background / border customization (issue 4671).
//!
//! Shows three variants:
//!   1. A default TextBox (for comparison).
//!   2. A thick colored border via `border_brush` + `border_thickness`.
//!   3. A borderless, transparent input (e.g. a chat/search bar) using
//!      `background(Color::transparent())` + `border_thickness(uniform(0.0))`.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (text, set_text) = cx.use_state(String::new());

    vstack((
        text_block("1. Default TextBox"),
        text_box(text.clone())
            .placeholder_text("Default style")
            .on_text_changed(set_text.clone()),
        text_block("2. Custom border (brush + thickness)"),
        text_box(text.clone())
            .placeholder_text("Thick blue border")
            .border_brush(Color::rgb(60, 120, 220))
            .border_thickness(Thickness::uniform(2.0))
            .on_text_changed(set_text.clone()),
        text_block("3. Borderless + transparent (chat/search bar)"),
        text_box(text)
            .placeholder_text("Type a message…")
            .background(Color::transparent())
            .border_thickness(Thickness::uniform(0.0))
            .on_text_changed(set_text),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("TextBox border", app)
}
