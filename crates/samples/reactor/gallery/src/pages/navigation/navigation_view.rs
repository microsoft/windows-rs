use crate::controls::*;
use windows_reactor::*;

pub struct NavigationViewPage {
    selected: String,
    top_selected: String,
}

#[derive(Clone)]
pub enum Message {
    Left(Option<String>),
    Top(Option<String>),
}

fn item(tag: &str, label: &str, icon: Symbol, selected: bool) -> KeyedView {
    KeyedView::new(
        tag,
        NavigationViewItem::new()
            .tag(tag)
            .is_selected(selected)
            .selects_on_invoked(true)
            .slots([
                SlotView::new(
                    NavigationViewItemSlot::Content,
                    TextBlock::new().text(label),
                ),
                SlotView::new(NavigationViewItemSlot::Icon, SymbolIcon::new().symbol(icon)),
            ]),
    )
}

impl Component for NavigationViewPage {
    type Message = Message;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            selected: "home".to_string(),
            top_selected: "overview".to_string(),
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Left(Some(tag)) => self.selected = tag,
            Message::Top(Some(tag)) => self.top_selected = tag,
            Message::Left(None) | Message::Top(None) => {}
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let left_body = match self.selected.as_str() {
            "browse" => "Browse page content",
            "settings" => "Settings page content",
            _ => "Home page content",
        };
        let top_body = match self.top_selected.as_str() {
            "documents" => "Documents area",
            "downloads" => "Downloads area",
            _ => "Overview area",
        };
        page_content(
            "NavigationView",
            "A side or top navigation pane for app-level navigation.",
            [
                KeyedView::new(
                    "left",
                    sample_card(
                        "Left-Pane NavigationView",
                        NavigationView::new()
                            .pane_title("Navigation demo")
                            .is_settings_visible(false)
                            .height(300.0)
                            .on_selected_tag_changed(context.callback(Message::Left))
                            .slots([
                                SlotView::collection(
                                    NavigationViewSlot::MenuItems,
                                    [
                                        item("home", "Home", Symbol::Home, self.selected == "home"),
                                        item(
                                            "browse",
                                            "Browse",
                                            Symbol::Find,
                                            self.selected == "browse",
                                        ),
                                        item(
                                            "settings",
                                            "Settings",
                                            Symbol::Setting,
                                            self.selected == "settings",
                                        ),
                                    ],
                                ),
                                SlotView::new(
                                    NavigationViewSlot::Content,
                                    Border::new()
                                        .padding(20.0)
                                        .content(TextBlock::new().text(left_body)),
                                ),
                            ]),
                        "NavigationView::new().slots([menu_items, content])",
                    ),
                ),
                KeyedView::new(
                    "top",
                    sample_card(
                        "Top-Mode NavigationView",
                        NavigationView::new()
                            .pane_display_mode(NavigationViewPaneDisplayMode::Top)
                            .is_settings_visible(false)
                            .height(200.0)
                            .on_selected_tag_changed(context.callback(Message::Top))
                            .slots([
                                SlotView::collection(
                                    NavigationViewSlot::MenuItems,
                                    [
                                        item(
                                            "overview",
                                            "Overview",
                                            Symbol::Home,
                                            self.top_selected == "overview",
                                        ),
                                        item(
                                            "documents",
                                            "Documents",
                                            Symbol::Edit,
                                            self.top_selected == "documents",
                                        ),
                                        item(
                                            "downloads",
                                            "Downloads",
                                            Symbol::Download,
                                            self.top_selected == "downloads",
                                        ),
                                    ],
                                ),
                                SlotView::new(
                                    NavigationViewSlot::Content,
                                    Border::new()
                                        .padding(20.0)
                                        .content(TextBlock::new().text(top_body)),
                                ),
                            ]),
                        "NavigationView::new().pane_display_mode(NavigationViewPaneDisplayMode::Top)",
                    ),
                ),
            ],
        )
    }
}
