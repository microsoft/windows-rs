#![windows_subsystem = "windows"]

use windows_reactor::{Button, Element, Icon, IconSymbol, RenderCx, TextBlock, vstack};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let toggled = cx.use_state(|| false);
    let current = toggled.value();
    let icon = if current {
        IconSymbol::SAVE
    } else {
        IconSymbol::FAVORITE
    };
    let status = if current { "Save" } else { "Favorite" };

    vstack(
        12.0,
        [
            Button::new("Toggle Icon")
                .icon(Icon::symbol(icon))
                .on_click(move || {
                    toggled.set(!current);
                })
                .build(),
            TextBlock::new(format!("Current icon: {status}"))
                .opacity(0.6)
                .build(),
            TextBlock::new("Click the button - the icon should change but the label stays.")
                .opacity(0.4)
                .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("ButtonIconGlyphChange", app)
}
