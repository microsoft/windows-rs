use windows_reactor::*;

struct MenuBarSample {
    last_click: String,
}

impl Component for MenuBarSample {
    type Message = String;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            last_click: "(none)".to_string(),
        }
    }

    fn update(&mut self, message: String, _context: &ComponentContext<Self>) {
        self.last_click = message;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("MenuBar");
        let callback = context.callback(std::convert::identity);
        StackPanel::new().spacing(12.0).children((
            MenuBar::new().slots([SlotView::collection(
                MenuBarSlot::Items,
                [
                    KeyedView::new(
                        "file",
                        MenuBarItem::new().title("File").menu(Menu::new(
                            [
                                MenuItem::item("new", "New"),
                                MenuItem::item("open", "Open"),
                                MenuItem::separator("file-separator-1"),
                                MenuItem::submenu(
                                    "recent",
                                    "Recent",
                                    [
                                        MenuItem::item("doc1", "doc1.txt"),
                                        MenuItem::item("doc2", "doc2.txt"),
                                    ],
                                ),
                                MenuItem::separator("file-separator-2"),
                                MenuItem::item("exit", "Exit"),
                            ],
                            callback.clone(),
                        )),
                    ),
                    KeyedView::new(
                        "edit",
                        MenuBarItem::new().title("Edit").menu(Menu::new(
                            [
                                MenuItem::item("cut", "Cut"),
                                MenuItem::item("copy", "Copy"),
                                MenuItem::item("paste", "Paste"),
                            ],
                            callback.clone(),
                        )),
                    ),
                    KeyedView::new(
                        "help",
                        MenuBarItem::new().title("Help").menu(Menu::new(
                            [MenuItem::item("about", "About")],
                            callback.clone(),
                        )),
                    ),
                ],
            )]),
            DropDownButton::new()
                .content(TextBlock::new().text("Actions"))
                .menu(Menu::new(
                    [
                        MenuItem::item("action-a", "Action A"),
                        MenuItem::item("action-b", "Action B"),
                        MenuItem::separator("action-separator"),
                        MenuItem::submenu(
                            "more",
                            "More",
                            [
                                MenuItem::item("action-c", "Action C"),
                                MenuItem::item("action-d", "Action D"),
                            ],
                        ),
                    ],
                    callback,
                )),
            TextBlock::new().text(format!("Last clicked: {}", self.last_click)),
        ))
    }
}

fn main() {
    App::run_component::<MenuBarSample>(()).unwrap();
}
