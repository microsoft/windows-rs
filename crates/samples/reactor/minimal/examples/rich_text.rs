//! Sample for the `RichTextBlock` element.

use windows_reactor::*;
use windows_reactor::{
    RichTextBlock, RichTextHyperlink, RichTextInline, RichTextParagraph, RichTextRun,
};

fn app(_cx: &mut RenderCx) -> Element {
    let mixed = RichTextBlock::single_paragraph(vec![
        RichTextInline::Run(RichTextRun::plain("Plain, ")),
        RichTextInline::Run(RichTextRun {
            text: "bold".to_string(),
            is_bold: true,
            ..Default::default()
        }),
        RichTextInline::Run(RichTextRun::plain(", ")),
        RichTextInline::Run(RichTextRun {
            text: "italic".to_string(),
            is_italic: true,
            ..Default::default()
        }),
        RichTextInline::Run(RichTextRun::plain(", and a ")),
        RichTextInline::Hyperlink(RichTextHyperlink {
            text: "link".to_string(),
            uri: "https://github.com/microsoft/windows-rs".to_string(),
        }),
        RichTextInline::Run(RichTextRun::plain(" all in one paragraph.")),
        // Soft break — same paragraph, new visual line.
        RichTextInline::LineBreak,
        RichTextInline::Run(RichTextRun::plain(
            "This continuation lives in the same paragraph but on a new visual line.",
        )),
    ])
    .font_size(14.0)
    .selectable()
    .wrap();

    let multi = RichTextBlock {
        paragraphs: vec![
            RichTextParagraph::new(vec![RichTextInline::Run(RichTextRun::plain(
                "First paragraph.",
            ))]),
            RichTextParagraph::new(vec![RichTextInline::Run(RichTextRun::plain(
                "Second paragraph.",
            ))]),
        ],
        ..RichTextBlock::new()
    };

    vstack((
        text_block("Single paragraph with mixed inlines:"),
        mixed,
        text_block("Multi-paragraph block:"),
        multi,
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("RichText", app)
}
