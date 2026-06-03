//! Button text preserved when its icon glyph changes dynamically.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (toggled, set_toggled) = cx.use_state(false);
    let icon = if toggled {
        SymbolGlyph::Save
    } else {
        SymbolGlyph::Favorite
    };
    let status = if toggled { "Save" } else { "Favorite" };

    vstack((
        Button::new("Toggle Icon").icon(icon).on_click({
            let set_toggled = set_toggled;
            move || set_toggled.call(!toggled)
        }),
        text_block(format!("Current icon: {status}")).opacity(0.6),
        text_block("Click the button — the icon should change but the label stays.").opacity(0.4),
    ))
    .spacing(12.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("ButtonIconGlyphChange", app)
}
