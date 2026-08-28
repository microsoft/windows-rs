use crate::controls::*;
use windows_reactor::*;

pub struct TextBoxPage {
    text: String,
    notes: String,
}

#[derive(Clone)]
pub enum Message {
    Text(String),
    Notes(String),
}

impl Component for TextBoxPage {
    type Message = Message;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            text: String::new(),
            notes: "Hello\nWorld".to_string(),
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Text(value) => self.text = value,
            Message::Notes(value) => self.notes = value,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "TextBox",
            "A single-line or multi-line text input field.",
            [
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Basic TextBox",
                        StackPanel::new().spacing(8.0).children((
                            TextBox::new()
                                .text(&self.text)
                                .placeholder_text("Type here...")
                                .on_text_changed(context.callback(Message::Text))
                                .slot(TextBoxSlot::Header, "Name"),
                            TextBlock::new()
                                .text(format!("Characters: {}", self.text.len()))
                                .opacity(0.6),
                        )),
                        "TextBox::new().text(value).on_text_changed(handler)",
                    ),
                ),
                KeyedView::new(
                    "multiline",
                    sample_card(
                        "Multi-line TextBox",
                        TextBox::new()
                            .text(&self.notes)
                            .accepts_return(true)
                            .text_wrapping(TextWrapping::Wrap)
                            .height(120.0)
                            .on_text_changed(context.callback(Message::Notes)),
                        "TextBox::new().accepts_return(true).text_wrapping(TextWrapping::Wrap)",
                    ),
                ),
                KeyedView::new(
                    "disabled",
                    sample_card(
                        "Disabled TextBox",
                        TextBox::new().text("Read-only content").is_enabled(false),
                        "TextBox::new().text(content).is_enabled(false)",
                    ),
                ),
            ],
        )
    }
}
