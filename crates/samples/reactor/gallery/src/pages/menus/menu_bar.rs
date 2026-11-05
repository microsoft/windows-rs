use crate::controls::*;
use windows_reactor::*;

pub struct MenuBarPage {
    last_click: String,
}

impl Component for MenuBarPage {
    type Message = String;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            last_click: "(none)".to_string(),
        }
    }

    fn update(&mut self, item: String, _context: &ComponentContext<Self>) {
        self.last_click = item;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        let callback = context.callback(|key: Key| key_label(&key));
        page_content(
            "MenuBar",
            "A horizontal bar hosting drop-down menus.",
            [KeyedView::new(
                "basic-menu-bar",
                sample_card(
                    "Basic MenuBar",
                    StackPanel::new().spacing(8.0).children((
                        MenuBar::new().items((
                            MenuBarItem::new().title("File").menu(Menu::new(
                                [
                                    MenuItem::item("new", "New"),
                                    MenuItem::item("open", "Open"),
                                    MenuItem::item("save", "Save"),
                                ],
                                callback.clone(),
                            )),
                            MenuBarItem::new().title("Edit").menu(Menu::new(
                                [
                                    MenuItem::item("undo", "Undo"),
                                    MenuItem::item("cut", "Cut"),
                                    MenuItem::item("copy", "Copy"),
                                    MenuItem::item("paste", "Paste"),
                                ],
                                callback,
                            )),
                        )),
                        TextBlock::new()
                            .text(format!("Last clicked: {}", self.last_click))
                            .opacity(0.6),
                    )),
                ),
            )],
        )
    }
}
