use windows_reactor::core::rich_text::{RichText, RichTextInline, RichTextParagraph, RichTextRun};

#[test]
fn run_plain_creates_default_styled_run() {
    let r = RichTextRun::plain("hello");
    assert_eq!(r.text, "hello");
    assert!(!r.is_bold);
    assert!(!r.is_italic);
    assert!(r.font_family.is_none());
}

#[test]
fn paragraph_holds_inlines_in_order() {
    let p = RichTextParagraph::new(vec![
        RichTextInline::Run(RichTextRun::plain("a")),
        RichTextInline::LineBreak,
        RichTextInline::Run(RichTextRun::plain("b")),
    ]);
    assert_eq!(p.inlines.len(), 3);
    assert!(matches!(p.inlines[1], RichTextInline::LineBreak));
}

#[test]
fn rich_text_builder_chain() {
    let rt = RichText::single_paragraph(vec![RichTextInline::Run(RichTextRun::plain("x"))])
        .font_size(20.0)
        .selectable()
        .wrap();
    assert_eq!(rt.font_size, Some(20.0));
    assert!(rt.is_text_selection_enabled);
    assert!(rt.text_wrapping_wrap);
    assert_eq!(rt.paragraphs.len(), 1);
}
