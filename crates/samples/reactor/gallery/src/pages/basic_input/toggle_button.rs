use crate::controls::*;
use windows_reactor::*;

pub struct ToggleButtonPage {
    bold: bool,
    italic: bool,
}

#[derive(Clone)]
pub enum Message {
    Bold(bool),
    Italic(bool),
}

impl Component for ToggleButtonPage {
    type Message = Message;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            bold: false,
            italic: false,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Bold(value) => self.bold = value,
            Message::Italic(value) => self.italic = value,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "ToggleButton",
            "A button that toggles between two states.",
            [
                KeyedView::new(
                    "formatting",
                    sample_card(
                        "Text Formatting Toggles",
                        StackPanel::new().spacing(8.0).children((
                            StackPanel::new()
                                .orientation(Orientation::Horizontal)
                                .spacing(8.0)
                                .children((
                                    ToggleButton::new()
                                        .is_checked(self.bold)
                                        .on_is_checked_changed(context.callback(Message::Bold))
                                        .content("Bold"),
                                    ToggleButton::new()
                                        .is_checked(self.italic)
                                        .on_is_checked_changed(context.callback(Message::Italic))
                                        .content("Italic"),
                                )),
                            TextBlock::new()
                                .text(format!("Bold: {}, Italic: {}", self.bold, self.italic))
                                .opacity(0.6),
                        )),
                        "ToggleButton::new().is_checked(value).on_is_checked_changed(handler)",
                    ),
                ),
                KeyedView::new(
                    "disabled",
                    sample_card(
                        "Disabled ToggleButton",
                        ToggleButton::new()
                            .is_checked(true)
                            .is_enabled(false)
                            .content("Locked"),
                        "ToggleButton::new().is_checked(true).is_enabled(false)",
                    ),
                ),
            ],
        )
    }
}
