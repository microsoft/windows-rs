use crate::controls::*;
use windows_reactor::*;

pub struct MenuFlyoutPage {
    last_action: String,
    last_format: String,
}

impl Component for MenuFlyoutPage {
    type Message = (String, String);
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            last_action: "(none)".to_string(),
            last_format: "(none)".to_string(),
        }
    }

    fn update(&mut self, message: (String, String), _context: &ComponentContext<Self>) {
        let (which, value) = message;
        match which.as_str() {
            "action" => self.last_action = value,
            "format" => self.last_format = value,
            _ => {}
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
                                context.callback(|label| ("action".to_string(), label)),
                            )),
                            TextBlock::new()
                                .text(format!("Last action: {}", self.last_action))
                                .opacity(0.6),
                        )),
                        r#"Button::new()
    .content("Open Menu")
    .menu(Menu::new(
        [
            MenuItem::item("cut", "Cut"),
            MenuItem::item("copy", "Copy"),
            MenuItem::item("paste", "Paste"),
        ],
        context.callback(|label| ("action".to_string(), label)),
    ))"#,
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
                                context.callback(|label| ("format".to_string(), label)),
                            )),
                            TextBlock::new()
                                .text(format!("Last format: {}", self.last_format))
                                .opacity(0.6),
                        )),
                        r#"Button::new()
    .content("Format")
    .menu(Menu::new(
        [
            MenuItem::item("bold", "Bold"),
            MenuItem::item("italic", "Italic"),
            MenuItem::separator("separator"),
            MenuItem::item("underline", "Underline"),
            MenuItem::item("strikethrough", "Strikethrough"),
        ],
        context.callback(|label| ("format".to_string(), label)),
    ))"#,
                    ),
                ),
            ],
        )
    }
}
