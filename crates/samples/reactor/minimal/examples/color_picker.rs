//! Minimal sample for the `ColorPicker` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> impl Into<Element> {
    let (color, set_color) = cx.use_state((255_u8, 0_u8, 120_u8, 215_u8));

    let update = move |argb: (u8, u8, u8, u8)| set_color.call(argb);

    let (a, r, g, b) = color;

    vstack((
        color_picker(ColorArgb::with_alpha(a, r, g, b))
            .alpha_enabled(true)
            .on_changed(update),
        text_block(format!("ARGB: ({a}, {r}, {g}, {b})")),
        text_block(format!("Hex: #{r:02X}{g:02X}{b:02X}")),
    ))
    .spacing(8.0)
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
