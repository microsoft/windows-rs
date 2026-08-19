#![windows_subsystem = "windows"]

use windows_reactor::{Color, ColorPicker, Element, RenderCx, TextBlock, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let color = cx.use_state(|| Color::argb(255, 0, 120, 215));
    let current = color.value();

    vstack(
        8.0,
        [
            ColorPicker::new(current, move |value| {
                color.set(value);
            })
            .build(),
            TextBlock::new(format!(
                "ARGB: ({}, {}, {}, {})",
                current.a, current.r, current.g, current.b
            ))
            .build(),
            TextBlock::new(format!(
                "Hex: #{:02X}{:02X}{:02X}",
                current.r, current.g, current.b
            ))
            .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("ColorPicker", app)
}
