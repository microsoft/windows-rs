//! Minimal sample for the `Image` element.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    let source = format!(
        "file:///{}/examples/image.png",
        env!("CARGO_MANIFEST_DIR").replace('\\', "/"),
    );

    vstack((
        text_block("Uniform (default)"),
        Image::new(&source).width(120.0).height(60.0),
        text_block("UniformToFill"),
        Image::new(&source)
            .stretch(ImageStretch::UniformToFill)
            .width(120.0)
            .height(60.0),
        text_block("Fill"),
        Image::new(&source)
            .stretch(ImageStretch::Fill)
            .width(120.0)
            .height(60.0),
        text_block("None"),
        Image::new(&source)
            .stretch(ImageStretch::None)
            .width(120.0)
            .height(60.0),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
