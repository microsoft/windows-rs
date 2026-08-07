#![windows_subsystem = "windows"]

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    let source = format!(
        "file:///{}/examples/image.svg",
        env!("CARGO_MANIFEST_DIR").replace('\\', "/"),
    );

    vstack((
        text_block("The source SVG has a 120x60 view box."),
        text_block("As an ImageIcon it stays inside the standard 20-DIP icon box."),
        button("SVG image icon").icon(Icon::image(source.clone())),
        button("").icon(Icon::image(source.clone())),
        text_block("The same source in an Image control:"),
        Image::new(source).width(120.0).height(60.0),
    ))
    .spacing(12.0)
    .padding(Thickness::uniform(16.0))
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("ImageIconSize", app)
}
