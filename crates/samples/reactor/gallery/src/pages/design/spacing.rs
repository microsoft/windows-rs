use crate::controls::page_content;
use windows_reactor::*;

pub fn spacing_page(_: &(), _cx: &mut RenderCx) -> impl Into<Element> {
    let spacings = [
        ("XXSmall", 2.0),
        ("XSmall", 4.0),
        ("Small", 8.0),
        ("Medium", 12.0),
        ("Large", 16.0),
        ("XLarge", 24.0),
        ("XXLarge", 32.0),
        ("XXXLarge", 48.0),
    ];

    let samples: Vec<Element> = spacings
        .iter()
        .map(|(name, size)| spacing_row(name, *size))
        .collect();

    page_content(
        "Spacing",
        "Standard spacing values used to create consistent layouts in WinUI 3.",
        vec![vstack(samples).spacing(8.0).into()],
    )
}

fn spacing_row(name: &str, size: f64) -> Element {
    let bar = border(Element::Empty)
        .background(Color::rgb(0, 120, 212))
        .width(size)
        .height(24.0)
        .corner_radius(4.0);

    hstack((
        text_block(format!("{name} ({size}px)"))
            .width(180.0)
            .vertical_alignment(VerticalAlignment::Center),
        bar,
    ))
    .spacing(12.0)
    .into()
}
