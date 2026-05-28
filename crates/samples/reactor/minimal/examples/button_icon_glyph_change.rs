//! Demonstrates that a button's text is preserved when its icon changes dynamically.
//!
//! Without the ButtonIcon glyph-change fix, updating the icon would nest
//! StackPanels or lose the text label entirely. Click the button to toggle
//! the icon between Favorite (♥) and Save (💾) — the "Toggle Icon" label
//! should remain visible throughout.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> impl Into<Element> {
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
}

fn main() -> Result<()> {
    App::new().title("Button Icon — Dynamic Glyph").render(app)
}
