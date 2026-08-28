use crate::controls::*;
use windows_reactor::*;

pub struct ButtonPage {
    basic_output: String,
    accent_output: String,
    subtle_output: String,
    link_output: String,
}

#[derive(Clone)]
pub enum Message {
    Basic,
    Accent,
    Subtle,
    Link,
}

impl Component for ButtonPage {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            basic_output: String::from("Ready"),
            accent_output: String::from("Ready"),
            subtle_output: String::from("Ready"),
            link_output: String::from("Ready"),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Basic => self.basic_output = String::from("Saved!"),
            Message::Accent => self.accent_output = String::from("Confirmed!"),
            Message::Subtle => self.subtle_output = String::from("Cancelled."),
            Message::Link => self.link_output = String::from("Link clicked."),
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "Button",
            "A button that responds to user clicks.",
            [
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Basic Button",
                        StackPanel::new().spacing(8.0).children((
                            Button::new()
                                .on_click(context.message(Message::Basic))
                                .content("Save"),
                            TextBlock::new()
                                .text(self.basic_output.clone())
                                .opacity(0.6),
                        )),
                        r#"Button::new().on_click(context.message(Message::Basic)).content("Save")"#,
                    ),
                ),
                KeyedView::new(
                    "disabled",
                    sample_card(
                        "Disabled Button",
                        Button::new().is_enabled(false).content("Can't Click"),
                        r#"Button::new().is_enabled(false).content("Can't Click")"#,
                    ),
                ),
                KeyedView::new(
                    "accent",
                    sample_card(
                        "Accent style",
                        StackPanel::new().spacing(8.0).children((
                            Button::new()
                                .style(ButtonStyle::Accent)
                                .on_click(context.message(Message::Accent))
                                .content("Confirm"),
                            TextBlock::new()
                                .text(self.accent_output.clone())
                                .opacity(0.6),
                        )),
                        r#"Button::new().style(ButtonStyle::Accent).on_click(handler).content("Confirm")"#,
                    ),
                ),
                KeyedView::new(
                    "subtle",
                    sample_card(
                        "Subtle style",
                        StackPanel::new().spacing(8.0).children((
                            Button::new()
                                .style(ButtonStyle::Subtle)
                                .on_click(context.message(Message::Subtle))
                                .content("Cancel"),
                            TextBlock::new()
                                .text(self.subtle_output.clone())
                                .opacity(0.6),
                        )),
                        r#"Button::new().style(ButtonStyle::Subtle).on_click(handler).content("Cancel")"#,
                    ),
                ),
                KeyedView::new(
                    "text-link",
                    sample_card(
                        "Text-link style",
                        StackPanel::new().spacing(8.0).children((
                            StackPanel::new()
                                .orientation(Orientation::Horizontal)
                                .spacing(4.0)
                                .children((
                                    TextBlock::new().text("Need help?").opacity(0.6),
                                    Button::new()
                                        .style(ButtonStyle::TextLink)
                                        .on_click(context.message(Message::Link))
                                        .content("Learn more"),
                                )),
                            TextBlock::new().text(self.link_output.clone()).opacity(0.6),
                        )),
                        r#"Button::new().style(ButtonStyle::TextLink).on_click(handler).content("Learn more")"#,
                    ),
                ),
            ],
        )
    }
}
