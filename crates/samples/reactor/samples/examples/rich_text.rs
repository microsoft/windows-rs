#![windows_subsystem = "windows"]

use windows_reactor::{
    Element, RenderCx, RichTextBlock, RichTextHyperlink, RichTextInline, RichTextParagraph,
    RichTextRun, StackPanel, TextBlock, Thickness,
};

pub fn app(_cx: &mut RenderCx<'_>) -> Element {
    let mixed = RichTextBlock::single_paragraph([
        RichTextInline::Run(RichTextRun::plain("Plain, ")),
        RichTextInline::Run(RichTextRun {
            text: "bold".into(),
            bold: true,
            ..Default::default()
        }),
        RichTextInline::Run(RichTextRun::plain(", ")),
        RichTextInline::Run(RichTextRun {
            text: "italic".into(),
            italic: true,
            ..Default::default()
        }),
        RichTextInline::Run(RichTextRun::plain(", and a ")),
        RichTextInline::Hyperlink(RichTextHyperlink {
            text: "link".into(),
            uri: "https://github.com/microsoft/windows-rs".into(),
        }),
        RichTextInline::Run(RichTextRun::plain(" all in one paragraph.")),
        RichTextInline::LineBreak,
        RichTextInline::Run(RichTextRun::plain(
            "This continuation is on a new visual line.",
        )),
    ])
    .font_size(14.0)
    .selectable(true)
    .wrap(true)
    .automation_id("mixed-text")
    .build();

    let multi = RichTextBlock::new([
        RichTextParagraph::new([RichTextInline::Run(RichTextRun::plain("First paragraph."))]),
        RichTextParagraph::new([RichTextInline::Run(RichTextRun::plain("Second paragraph."))]),
    ])
    .automation_id("multi-text")
    .build();

    StackPanel::new([
        TextBlock::new("Single paragraph with mixed inlines:").build(),
        mixed,
        TextBlock::new("Multi-paragraph block:").build(),
        multi,
    ])
    .spacing(8.0)
    .padding(Thickness::uniform(16.0))
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("RichText", app)
}
