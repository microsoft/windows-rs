use windows_reactor::*;

struct NavigationIconsSample {
    page: String,
}

impl Component for NavigationIconsSample {
    type Message = Option<String>;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            page: "home".to_string(),
        }
    }

    fn update(&mut self, page: Option<String>, _context: &ComponentContext<Self>) {
        if let Some(page) = page {
            self.page = page;
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let item = |tag, label, symbol| {
            KeyedView::new(
                tag,
                NavigationViewItem::new()
                    .tag(tag)
                    .is_selected(self.page == tag)
                    .slots([
                        SlotView::new(
                            NavigationViewItemSlot::Content,
                            TextBlock::new().text(label),
                        ),
                        SlotView::new(
                            NavigationViewItemSlot::Icon,
                            SymbolIcon::new().symbol(symbol),
                        ),
                    ]),
            )
        };
        let content = match self.page.as_str() {
            "home" => "Welcome home!",
            "settings" => "Settings page",
            "mail" => "Mail inbox",
            "people" => "Contacts",
            _ => "Unknown page",
        };

        context.window_title("NavigationViewIcons");
        NavigationView::new()
            .is_settings_visible(false)
            .on_selected_tag_changed(context.callback(std::convert::identity))
            .slots([
                SlotView::collection(
                    NavigationViewSlot::MenuItems,
                    [
                        item("home", "Home", Symbol::Home),
                        item("mail", "Mail", Symbol::Mail),
                        item("people", "People", Symbol::People),
                        item("settings", "Settings", Symbol::Setting),
                    ],
                ),
                SlotView::new(NavigationViewSlot::Content, TextBlock::new().text(content)),
            ])
    }
}

fn main() {
    App::run_component::<NavigationIconsSample>(()).unwrap();
}
