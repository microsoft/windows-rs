#![windows_subsystem = "windows"]

use windows_reactor::{Element, RenderCx, TextBlock, ToggleButton, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let bold = cx.use_state(|| false);
    let italic = cx.use_state(|| false);
    let is_bold = bold.value();
    let is_italic = italic.value();

    let style_label = match (is_bold, is_italic) {
        (true, true) => "Bold + Italic",
        (true, false) => "Bold",
        (false, true) => "Italic",
        (false, false) => "Normal",
    };

    vstack(
        8.0,
        [
            ToggleButton::new("Bold", is_bold, move |value| {
                bold.set(value);
            })
            .build(),
            ToggleButton::new("Italic", is_italic, move |value| {
                italic.set(value);
            })
            .build(),
            TextBlock::new(format!("Style: {style_label}")).build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("ToggleButton", app)
}
