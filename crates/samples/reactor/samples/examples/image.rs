#![windows_subsystem = "windows"]

use windows_reactor::*;

fn main() {
    sample_reactor_controls::run("Image", || {
        let bitmap = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/image.png");
        let svg = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/image.svg");

        StackPanel::new().spacing(8.0).children((
            "PNG",
            Image::new()
                .width(120.0)
                .height(60.0)
                .source_file(&bitmap)
                .unwrap(),
            "PNG from encoded bytes",
            Image::new()
                .width(120.0)
                .height(60.0)
                .source_data(EncodedImage::from_static(include_bytes!("image.png"))),
            "SVG",
            Image::new()
                .width(120.0)
                .height(60.0)
                .source_file(&svg)
                .unwrap(),
            "Uniform (default)",
            Image::new()
                .width(120.0)
                .height(60.0)
                .source_file(&bitmap)
                .unwrap(),
            "UniformToFill",
            Image::new()
                .stretch(Stretch::UniformToFill)
                .width(120.0)
                .height(60.0)
                .source_file(&bitmap)
                .unwrap(),
            "Fill",
            Image::new()
                .stretch(Stretch::Fill)
                .width(120.0)
                .height(60.0)
                .source_file(&bitmap)
                .unwrap(),
            "None",
            Image::new()
                .stretch(Stretch::None)
                .width(120.0)
                .height(60.0)
                .source_file(&bitmap)
                .unwrap(),
        ))
    })
    .unwrap();
}
