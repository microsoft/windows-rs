#![windows_subsystem = "windows"]

use windows_reactor::*;

fn main() {
    sample_reactor_controls::run("ThemeBrush", || {
        let swatch = |label, background, foreground| {
            Border::new()
                .background(background)
                .padding(Thickness::uniform(4.0))
                .min_width(200.0)
                .content(
                    Border::new().padding(Thickness::uniform(10.0)).content(
                        TextBlock::new()
                            .text(label)
                            .font_size(13.0)
                            .foreground(foreground),
                    ),
                )
        };

        StackPanel::new().spacing(6.0).max_width(420.0).children((
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
        ))
    })
    .unwrap();
}
