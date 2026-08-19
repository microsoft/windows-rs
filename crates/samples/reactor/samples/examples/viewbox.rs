#![windows_subsystem = "windows"]

use windows_reactor::{
    Border, Color, Element, FontWeight, RenderCx, Stretch, TextBlock, Thickness, Viewbox, vstack,
};

fn card(label: &str) -> Element {
    Border::new(
        TextBlock::new(label)
            .font_size(24.0)
            .font_weight(FontWeight::BOLD)
            .build(),
    )
    .background(Color::rgb(80, 130, 200))
    .padding(Thickness::uniform(12.0))
    .width(200.0)
    .height(80.0)
    .build()
}

pub fn app(_cx: &mut RenderCx<'_>) -> Element {
    vstack(
        8.0,
        [
            TextBlock::new("Uniform").build(),
            Viewbox::new(card("200 x 80"))
                .stretch(Stretch::Uniform)
                .width(300.0)
                .height(140.0)
                .build(),
            TextBlock::new("Fill").build(),
            Viewbox::new(card("200 x 80"))
                .stretch(Stretch::Fill)
                .width(300.0)
                .height(140.0)
                .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Viewbox", app)
}
