use windows_reactor::*;

fn main() {
    sample_reactor_controls::run("RichText", || {
        let mixed = RichText::single_paragraph([
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
            RichTextInline::LineBreak,
            RichTextInline::Run(RichTextRun::plain(
                "This continuation lives in the same paragraph but on a new visual line.",
            )),
        ]);
        let multi = RichText::new([
            RichTextParagraph::new([RichTextInline::Run(RichTextRun::plain("First paragraph."))]),
            RichTextParagraph::new([RichTextInline::Run(RichTextRun::plain("Second paragraph."))]),
        ]);

        StackPanel::new().spacing(8.0).children((
            "Single paragraph with mixed inlines:",
            RichTextBlock::new()
                .paragraphs(mixed)
                .font_size(14.0)
                .is_text_selection_enabled(true)
                .text_wrapping(TextWrapping::Wrap),
            "Multi-paragraph block:",
            RichTextBlock::new().paragraphs(multi),
        ))
    })
    .unwrap();
}
