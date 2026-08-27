use crate::controls::*;
use windows_reactor::*;

pub struct MenuBarPage {
    last_click: String,
}

#[derive(Clone)]
pub enum Message {
    ItemClicked(String),
}

impl Component for MenuBarPage {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            last_click: "(none)".to_string(),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::ItemClicked(label) => self.last_click = label,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        let callback = context.callback(Message::ItemClicked);
        page_content(
            "MenuBar",
            "A horizontal bar hosting drop-down menus.",
            [KeyedView::new(
                "basic-menu-bar",
                sample_card(
                    "Basic MenuBar",
                    StackPanel::new().spacing(8.0).children((
                        MenuBar::new().slots([SlotView::collection(
                            MenuBarSlot::Items,
                            [
                                KeyedView::new(
                                    "file",
                                    MenuBarItem::new().title("File").menu(Menu::new(
                                        [
                                            MenuItem::item("new", "New"),
                                            MenuItem::item("open", "Open"),
                                            MenuItem::item("save", "Save"),
                                        ],
                                        callback.clone(),
                                    )),
                                ),
                                KeyedView::new(
                                    "edit",
                                    MenuBarItem::new().title("Edit").menu(Menu::new(
                                        [
                                            MenuItem::item("undo", "Undo"),
                                            MenuItem::item("cut", "Cut"),
                                            MenuItem::item("copy", "Copy"),
                                            MenuItem::item("paste", "Paste"),
                                        ],
                                        callback,
                                    )),
                                ),
                            ],
                        )]),
                        TextBlock::new()
                            .text(format!("Last clicked: {}", self.last_click))
                            .opacity(0.6),
                    )),
                    r#"MenuBar::new().slots([SlotView::collection(MenuBarSlot::Items, [...])])"#,
                ),
            )],
        )
    }
}
