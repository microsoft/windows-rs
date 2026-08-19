#![windows_subsystem = "windows"]

use windows_reactor::{
    Button, Element, Icon, Image, ImageSource, RenderCx, StackPanel, TextBlock, Thickness,
};

fn app(_cx: &mut RenderCx<'_>) -> Element {
    let source = format!(
        "file:///{}/examples/image.svg",
        env!("CARGO_MANIFEST_DIR").replace('\\', "/"),
    );

    StackPanel::new([
        TextBlock::new("The source SVG has a 120x60 view box.").build(),
        TextBlock::new("As an ImageIcon it stays inside the standard 20-DIP icon box.").build(),
        Button::new("SVG image icon")
            .icon(Icon::image(ImageSource::svg(source.as_str())))
            .build(),
        Button::new("")
            .icon(Icon::image(ImageSource::svg(source.as_str())))
            .build(),
        TextBlock::new("The same source in an Image control:").build(),
        Image::new(ImageSource::svg(source))
            .width(120.0)
            .height(60.0)
            .build(),
    ])
    .spacing(12.0)
    .padding(Thickness::uniform(16.0))
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("ImageIconSize", app)
}
