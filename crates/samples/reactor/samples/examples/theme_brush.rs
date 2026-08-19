#![windows_subsystem = "windows"]

use windows_reactor::{Border, Element, RenderCx, TextBlock, ThemeBrush, Thickness, vstack};

fn app(_cx: &mut RenderCx<'_>) -> Element {
    let swatch = |label: &str, background: ThemeBrush, foreground: ThemeBrush| {
        Border::new(
            TextBlock::new(label)
                .padding(Thickness::uniform(10.0))
                .font_size(13.0)
                .foreground(foreground)
                .build(),
        )
        .background(background)
        .padding(Thickness::uniform(4.0))
        .min_width(200.0)
        .build()
    };

    vstack(
        6.0,
        [
            swatch(
                "Accent / AccentText",
                ThemeBrush::Accent,
                ThemeBrush::AccentText,
            ),
            swatch(
                "Card / Primary text",
                ThemeBrush::CardBackground,
                ThemeBrush::PrimaryText,
            ),
            swatch(
                "SystemCritical background / foreground",
                ThemeBrush::SystemCriticalBackground,
                ThemeBrush::SystemCritical,
            ),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Theme Brush", app)
}
