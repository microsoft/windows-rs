#![windows_subsystem = "windows"]

use windows_reactor::{Element, FontWeight, RenderCx, TextBlock, vstack};

pub fn app(_cx: &mut RenderCx<'_>) -> Element {
    vstack(
        8.0,
        [
            TextBlock::new("Title - 28 pixels, semibold")
                .font_size(28.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .build(),
            TextBlock::new("Subtitle - 20 pixels, semibold")
                .font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .build(),
            TextBlock::new("Body large - 18 pixels")
                .font_size(18.0)
                .build(),
            TextBlock::new("Body strong - 14 pixels, semibold")
                .font_size(14.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .build(),
            TextBlock::new("Body - 14 pixels").font_size(14.0).build(),
            TextBlock::new("Caption - 12 pixels")
                .font_size(12.0)
                .build(),
            TextBlock::new("Custom light weight")
                .font_weight(FontWeight::LIGHT)
                .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Type Ramp", app)
}
