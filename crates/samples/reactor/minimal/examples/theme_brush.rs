//! Minimal sample for theme-brush bindings.
//!
//! `ThemeRef::Accent`, `CardBackground`, etc. resolve against the live
//! XAML resource dictionary and track light/dark switches.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> impl Into<Element> {
    let swatch = |label: &str, bg: ThemeRef, fg: ThemeRef| -> Element {
        border(
            text_block(label)
                .font_size(13.0)
                .foreground(fg)
                .padding(Thickness::uniform(10.0)),
        )
        .background(bg)
        .padding(Thickness::uniform(4.0))
        .min_width(200.0)
        .into()
    };

    vstack((
        swatch(
            "Accent / AccentText",
            ThemeRef::Accent,
            ThemeRef::AccentText,
        ),
        swatch(
            "Card / Primary text",
            ThemeRef::CardBackground,
            ThemeRef::PrimaryText,
        ),
        swatch(
            "SystemCritical background / foreground",
            ThemeRef::SystemCriticalBackground,
            ThemeRef::SystemCritical,
        ),
    ))
    .spacing(6.0)
    .max_width(420.0)
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
