use crate::controls::*;
use windows_reactor::*;

pub struct RichTextBlockPage {
    font_size: f64,
}

impl Component for RichTextBlockPage {
    type Message = f64;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self { font_size: 14.0 }
    }

    fn update(&mut self, font_size: f64, _: &ComponentContext<Self>) {
        self.font_size = font_size;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let structured = RichText::new([
            RichTextParagraph::new([
                RichTextInline::Run(RichTextRun {
                    text: "Bold introduction. ".to_string(),
                    is_bold: true,
                    ..Default::default()
                }),
                RichTextInline::Run(RichTextRun::plain("Followed by normal text.")),
            ]),
            RichTextParagraph::new([
                RichTextInline::Run(RichTextRun {
                    text: "Italic emphasis ".to_string(),
                    is_italic: true,
                    ..Default::default()
                }),
                RichTextInline::Run(RichTextRun::plain("in a second paragraph.")),
            ]),
        ]);
        page_content(
            "RichTextBlock",
            "Displays formatted, read-only rich text with multiple paragraphs.",
            [
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Basic RichTextBlock",
                        StackPanel::new().spacing(12.0).children((
                            RichTextBlock::new()
                                .paragraphs(RichText::single_paragraph([RichTextInline::Run(
                                    RichTextRun::plain("This is a simple rich text block."),
                                )]))
                                .font_size(self.font_size),
                            Slider::new()
                                .minimum(10.0)
                                .maximum(28.0)
                                .value(self.font_size)
                                .on_value_changed(context.forward()),
                        )),
                        "RichTextBlock::new().paragraphs(rich_text).font_size(size)",
                    ),
                ),
                KeyedView::new(
                    "structured",
                    sample_card(
                        "Structured Rich Text",
                        RichTextBlock::new()
                            .paragraphs(structured)
                            .text_wrapping(TextWrapping::Wrap),
                        "RichText::new([RichTextParagraph::new(inlines), ...])",
                    ),
                ),
                KeyedView::new(
                    "selectable",
                    sample_card(
                        "Selectable Text",
                        RichTextBlock::new()
                            .paragraphs(RichText::single_paragraph([RichTextInline::Run(
                                RichTextRun::plain("This text is selectable. Try highlighting it."),
                            )]))
                            .is_text_selection_enabled(true),
                        "RichTextBlock::new().is_text_selection_enabled(true)",
                    ),
                ),
            ],
        )
    }
}
