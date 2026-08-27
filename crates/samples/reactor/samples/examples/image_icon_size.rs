#![windows_subsystem = "windows"]

use windows_reactor::*;

fn main() {
    sample_reactor_controls::run("ImageIconSize", || {
        let source = format!(
            "file:///{}/examples/image.svg",
            env!("CARGO_MANIFEST_DIR").replace('\\', "/"),
        );
        let icon = |label: &'static str| {
            let image = ImageIcon::new()
                .width(20.0)
                .height(20.0)
                .source(source.clone())
                .unwrap();
            if label.is_empty() {
                Button::new().content(image)
            } else {
                Button::new().content(
                    StackPanel::new()
                        .orientation(Orientation::Horizontal)
                        .spacing(6.0)
                        .children((image, TextBlock::new().text(label))),
                )
            }
        };

        Border::new().padding(Thickness::uniform(16.0)).content(
            StackPanel::new().spacing(12.0).children((
                TextBlock::new().text("The source SVG has a 120x60 view box."),
                TextBlock::new()
                    .text("As an ImageIcon it stays inside the standard 20-DIP icon box."),
                icon("SVG image icon"),
                icon(""),
                TextBlock::new().text("The same source in an Image control:"),
                Image::new()
                    .width(120.0)
                    .height(60.0)
                    .source(source)
                    .unwrap(),
            )),
        )
    })
    .unwrap();
}
