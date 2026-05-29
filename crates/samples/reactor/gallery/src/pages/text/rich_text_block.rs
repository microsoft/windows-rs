use crate::controls::*;
use windows_reactor::core::{RichTextBlock, RichTextInline, RichTextParagraph, RichTextRun};
use windows_reactor::*;

pub fn rich_text_block_page(_: &(), cx: &mut RenderCx) -> Element {
    let (font_size, set_font_size) = cx.use_state(14.0);

    page_content(
            "RichTextBlock",
            "Displays formatted, read-only rich text with multiple paragraphs and inline styles.",
            vec![
                sample_card(
                    "Basic RichTextBlock",
                    vstack((
                        RichTextBlock::single_paragraph(vec![RichTextInline::Run(
                            RichTextRun::plain(
                                "This is a simple rich text block displaying read-only content.",
                            ),
                        )])
                        .font_size(font_size),
                        Slider::new(font_size)
                            .range(10.0, 28.0)
                            .header("Font Size")
                            .on_changed({
                                let set_font_size = set_font_size;
                                move |v: f64| set_font_size.call(v)
                            }),
                    ))
                    .spacing(12.0)
                    ,
                    r#"RichTextBlock::single_paragraph(vec![
    RichTextInline::Run(RichTextRun::plain("Hello world")),
]).font_size(14.0)"#,
                ),
                sample_card(
                    "Structured Rich Text",
                    RichTextBlock {
                        paragraphs: vec![
                            RichTextParagraph::new(vec![
                                RichTextInline::Run(RichTextRun {
                                    text: "Bold introduction. ".into(),
                                    is_bold: true,
                                    ..Default::default()
                                }),
                                RichTextInline::Run(RichTextRun::plain(
                                    "Followed by normal text.",
                                )),
                            ]),
                            RichTextParagraph::new(vec![
                                RichTextInline::Run(RichTextRun {
                                    text: "Italic emphasis ".into(),
                                    is_italic: true,
                                    ..Default::default()
                                }),
                                RichTextInline::Run(RichTextRun::plain("mixed with ")),
                                RichTextInline::Run(RichTextRun {
                                    text: "bold".into(),
                                    is_bold: true,
                                    ..Default::default()
                                }),
                                RichTextInline::Run(RichTextRun::plain(".")),
                            ]),
                            RichTextParagraph::new(vec![RichTextInline::Run(RichTextRun::plain(
                                "A third paragraph with different content to show block-level formatting.",
                            ))]),
                        ],
                        ..Default::default()
                    }
                    ,
                    r#"RichTextBlock {
    paragraphs: vec![
        RichTextParagraph::new(vec![
            RichTextInline::Run(RichTextRun { text: "Bold".into(), is_bold: true, .. }),
            RichTextInline::Run(RichTextRun::plain("normal")),
        ]),
        RichTextParagraph::new(vec![
            RichTextInline::Run(RichTextRun { text: "Italic".into(), is_italic: true, .. }),
        ]),
    ],
    ..Default::default()
}"#,
                ),
                sample_card(
                    "Selectable Text",
                    RichTextBlock::single_paragraph(vec![RichTextInline::Run(
                        RichTextRun::plain(
                            "This text is selectable. Try highlighting it with your mouse.",
                        ),
                    )])
                    .selectable()
                    .wrap()
                    ,
                    r#"RichTextBlock::single_paragraph(vec![...])
    .selectable()
    .wrap()"#,
                ),
            ],
        )
}
