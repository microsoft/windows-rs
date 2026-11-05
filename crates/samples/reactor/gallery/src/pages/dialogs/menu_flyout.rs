use crate::controls::*;
use windows_reactor::*;

pub struct MenuFlyoutPage {
    last_action: String,
    last_format: String,
}

pub enum Message {
    Action(String),
    Format(String),
}

impl Component for MenuFlyoutPage {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            last_action: "(none)".to_string(),
            last_format: "(none)".to_string(),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Action(value) => self.last_action = value,
            Message::Format(value) => self.last_format = value,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "MenuFlyout",
            "A flyout that displays a list of menu commands.",
            [
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Basic MenuFlyout",
                        StackPanel::new().spacing(8.0).children((
                            Button::new().content("Open Menu").menu(Menu::new(
                                [
                                    MenuItem::item("cut", "Cut"),
                                    MenuItem::item("copy", "Copy"),
                                    MenuItem::item("paste", "Paste"),
                                ],
                                context.callback(|key: Key| Message::Action(key_label(&key))),
                            )),
                            TextBlock::new()
                                .text(format!("Last action: {}", self.last_action))
                                .opacity(0.6),
                        )),
                    ),
                ),
                KeyedView::new(
                    "separators",
                    sample_card(
                        "MenuFlyout with Separators",
                        StackPanel::new().spacing(8.0).children((
                            Button::new().content("Format").menu(Menu::new(
                                [
                                    MenuItem::item("bold", "Bold"),
                                    MenuItem::item("italic", "Italic"),
                                    MenuItem::separator("separator"),
                                    MenuItem::item("underline", "Underline"),
                                    MenuItem::item("strikethrough", "Strikethrough"),
                                ],
                                context.callback(|key: Key| Message::Format(key_label(&key))),
                            )),
                            TextBlock::new()
                                .text(format!("Last format: {}", self.last_format))
                                .opacity(0.6),
                        )),
                    ),
                ),
            ],
        )
    }
}
