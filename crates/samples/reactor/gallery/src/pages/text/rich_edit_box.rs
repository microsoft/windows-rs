use crate::controls::*;
use windows_reactor::*;

pub struct RichEditBoxPage {
    text: String,
}

impl Component for RichEditBoxPage {
    type Message = String;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            text: String::new(),
        }
    }

    fn update(&mut self, text: String, _: &ComponentContext<Self>) {
        self.text = text;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "RichEditBox",
            "A rich text editing control with formatting support.",
            [
                KeyedView::new(
                    "editor",
                    sample_card(
                        "Basic RichEditBox",
                        StackPanel::new().spacing(8.0).children((
                            RichEditBox::new()
                                .text(&self.text)
                                .placeholder_text("Start typing...")
                                .height(200.0)
                                .on_text_changed(context.forward())
                                .slot(RichEditBoxSlot::Header, "Document"),
                            TextBlock::new()
                                .text(if self.text.is_empty() {
                                    "No changes yet".to_string()
                                } else {
                                    format!("Modified - {} characters", self.text.chars().count())
                                })
                                .opacity(0.6),
                        )),
                        "RichEditBox::new().text(value).on_text_changed(handler)",
                    ),
                ),
                KeyedView::new(
                    "read-only",
                    sample_card(
                        "Read-only RichEditBox",
                        RichEditBox::new()
                            .text("This rich text content is read-only.")
                            .is_read_only(true)
                            .height(100.0),
                        "RichEditBox::new().is_read_only(true)",
                    ),
                ),
            ],
        )
    }
}
