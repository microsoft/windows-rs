use super::super::*;

#[test]
fn rich_text_paragraphs_mount_update_and_clear() {
    let initial = RichText::single_paragraph([
        RichTextInline::Run(RichTextRun {
            text: "Bold".to_string(),
            is_bold: true,
            ..Default::default()
        }),
        RichTextInline::LineBreak,
        RichTextInline::Hyperlink(RichTextHyperlink {
            text: "Link".to_string(),
            uri: "https://github.com/microsoft/windows-rs".to_string(),
        }),
    ]);
    let replacement = RichText::new([
        RichTextParagraph::new([RichTextInline::Run(RichTextRun::plain("First"))]),
        RichTextParagraph::new([RichTextInline::Run(RichTextRun {
            text: "Second".to_string(),
            is_italic: true,
            ..Default::default()
        })]),
    ]);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(RichTextBlock::new().paragraphs(initial.clone()).into())
        .unwrap();
    let root = pump.root().unwrap();
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::RichTextBlockBlocks),
        Some(&PropertyValue::RichText(initial))
    );

    pump.update(RichTextBlock::new().paragraphs(replacement.clone()).into())
        .unwrap();
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::RichTextBlockBlocks),
        Some(&PropertyValue::RichText(replacement))
    );

    pump.update(RichTextBlock::new().into()).unwrap();
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::RichTextBlockBlocks),
        None
    );
}
