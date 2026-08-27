#![windows_subsystem = "windows"]

use windows_reactor::*;

fn main() {
    sample_reactor_controls::run("Image", || {
        let bitmap = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/image.png");
        let svg = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/image.svg");

        StackPanel::new().spacing(8.0).children((
            TextBlock::new().text("PNG"),
            Image::new()
                .width(120.0)
                .height(60.0)
                .source_file(&bitmap)
                .unwrap(),
            TextBlock::new().text("SVG"),
            Image::new()
                .width(120.0)
                .height(60.0)
                .source_file(&svg)
                .unwrap(),
            TextBlock::new().text("Uniform (default)"),
            Image::new()
                .width(120.0)
                .height(60.0)
                .source_file(&bitmap)
                .unwrap(),
            TextBlock::new().text("UniformToFill"),
            Image::new()
                .stretch(Stretch::UniformToFill)
                .width(120.0)
                .height(60.0)
                .source_file(&bitmap)
                .unwrap(),
            TextBlock::new().text("Fill"),
            Image::new()
                .stretch(Stretch::Fill)
                .width(120.0)
                .height(60.0)
                .source_file(&bitmap)
                .unwrap(),
            TextBlock::new().text("None"),
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
