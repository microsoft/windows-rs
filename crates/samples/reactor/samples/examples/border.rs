#![windows_subsystem = "windows"]

use windows_reactor::{Border, Color, Element, FontWeight, RenderCx, TextBlock, Thickness, vstack};

pub fn app(_cx: &mut RenderCx<'_>) -> Element {
    vstack(
        8.0,
        [
            Border::new(
                TextBlock::new("Boxed text")
                    .foreground(Color::rgb(255, 255, 255))
                    .build(),
            )
            .background(Color::rgb(60, 100, 180))
            .padding(Thickness::uniform(12.0))
            .build(),
            Border::new(
                TextBlock::new("Margined + width-capped")
                    .font_weight(FontWeight::BOLD)
                    .foreground(Color::rgb(255, 255, 255))
                    .build(),
            )
            .background(Color::rgb(80, 140, 90))
            .padding(Thickness::xy(16.0, 8.0))
            .margin(Thickness::uniform(4.0))
            .max_width(280.0)
            .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Border", app)
}
