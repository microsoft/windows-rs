#![windows_subsystem = "windows"]

use windows_reactor::*;

fn field(label: &str, text: TextBlock) -> View {
    StackPanel::new()
        .spacing(4.0)
        .children((
            label,
            Border::new()
                .border_thickness(Thickness::uniform(1.0))
                .border_brush(Color::rgb(128, 128, 128))
                .padding(Thickness::uniform(8.0))
                .content(text),
        ))
        .into()
}

fn main() {
    App::run(
        Border::new().padding(Thickness::uniform(16.0)).content(
            StackPanel::new().spacing(12.0).children((
                "Resize the window to compare text and control alignment.",
                field(
                    "Left text in a stretched TextBlock",
                    TextBlock::new()
                        .text("0123456789\nshort")
                        .text_alignment(TextAlignment::Left),
                ),
                field(
                    "Centered text in a stretched TextBlock",
                    TextBlock::new()
                        .text("0123456789\nshort")
                        .text_alignment(TextAlignment::Center),
                ),
                field(
                    "Right text in a stretched TextBlock",
                    TextBlock::new()
                        .text("0123456789\nshort")
                        .text_alignment(TextAlignment::Right),
                ),
                field(
                    "Centered control with left-aligned text",
                    TextBlock::new()
                        .text("0123456789\nshort")
                        .text_alignment(TextAlignment::Left)
                        .horizontal_alignment(HorizontalAlignment::Center),
                ),
            )),
        ),
    )
    .unwrap();
}
