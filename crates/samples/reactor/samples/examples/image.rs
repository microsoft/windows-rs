#![windows_subsystem = "windows"]

use windows_reactor::{Element, Image, ImageSource, RenderCx, Stretch, TextBlock, vstack};

pub fn app(_cx: &mut RenderCx<'_>) -> Element {
    let bitmap = format!(
        "file:///{}/examples/image.png",
        env!("CARGO_MANIFEST_DIR").replace('\\', "/"),
    );
    let svg = format!(
        "file:///{}/examples/image.svg",
        env!("CARGO_MANIFEST_DIR").replace('\\', "/"),
    );

    vstack(
        8.0,
        [
            TextBlock::new("PNG").build(),
            Image::new(ImageSource::bitmap(bitmap.as_str()))
                .width(120.0)
                .height(60.0)
                .build(),
            TextBlock::new("SVG").build(),
            Image::new(ImageSource::svg(svg))
                .width(120.0)
                .height(60.0)
                .build(),
            TextBlock::new("Uniform (default)").build(),
            Image::new(ImageSource::bitmap(bitmap.as_str()))
                .width(120.0)
                .height(60.0)
                .build(),
            TextBlock::new("UniformToFill").build(),
            Image::new(ImageSource::bitmap(bitmap.as_str()))
                .stretch(Stretch::UniformToFill)
                .width(120.0)
                .height(60.0)
                .build(),
            TextBlock::new("Fill").build(),
            Image::new(ImageSource::bitmap(bitmap.as_str()))
                .stretch(Stretch::Fill)
                .width(120.0)
                .height(60.0)
                .build(),
            TextBlock::new("None").build(),
            Image::new(ImageSource::bitmap(bitmap))
                .stretch(Stretch::None)
                .width(120.0)
                .height(60.0)
                .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Image", app)
}
