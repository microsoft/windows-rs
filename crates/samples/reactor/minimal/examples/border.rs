//! Minimal sample for the `Border` element.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> impl Into<Element> {
    vstack((
        border(text_block("Boxed text").foreground(Color::rgb(255, 255, 255)))
            .background(Color::rgb(60, 100, 180))
            .padding(Thickness::uniform(12.0)),
        border(
            text_block("Margined + width-capped")
                .bold()
                .foreground(Color::rgb(255, 255, 255)),
        )
        .background(Color::rgb(80, 140, 90))
        .padding(Thickness::xy(16.0, 8.0))
        .margin(Thickness::uniform(4.0))
        .max_width(280.0),
    ))
    .spacing(8.0)
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
