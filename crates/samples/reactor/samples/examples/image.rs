use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    let bitmap = format!(
        "file:///{}/examples/image.png",
        env!("CARGO_MANIFEST_DIR").replace('\\', "/"),
    );
    let svg = format!(
        "file:///{}/examples/image.svg",
        env!("CARGO_MANIFEST_DIR").replace('\\', "/"),
    );

    vstack((
        text_block("PNG"),
        Image::new(&bitmap).width(120.0).height(60.0),
        text_block("SVG"),
        Image::new(&svg).width(120.0).height(60.0),
        text_block("Uniform (default)"),
        Image::new_with_uri(&bitmap).width(120.0).height(60.0),
        text_block("UniformToFill"),
        Image::new_with_uri(&bitmap)
            .stretch(Stretch::UniformToFill)
            .width(120.0)
            .height(60.0),
        text_block("Fill"),
        Image::new_with_uri(&bitmap)
            .stretch(Stretch::Fill)
            .width(120.0)
            .height(60.0),
        text_block("None"),
        Image::new_with_uri(&bitmap)
            .stretch(Stretch::None)
            .width(120.0)
            .height(60.0),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("Image", app)
}
