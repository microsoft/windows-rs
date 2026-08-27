#![windows_subsystem = "windows"]

use windows_reactor::*;

struct NavigationViewSample {
    page: String,
}

impl Component for NavigationViewSample {
    type Message = Option<String>;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            page: "home".into(),
        }
    }

    fn update(&mut self, page: Option<String>, _context: &ComponentContext<Self>) {
        if let Some(page) = page
            && matches!(page.as_str(), "home" | "settings" | "about")
        {
            self.page = page;
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("NavigationView");
        let items = [
            ("home", "Home"),
            ("settings", "Settings"),
            ("about", "About"),
        ]
        .into_iter()
        .map(|(tag, label)| {
            KeyedView::new(
                tag,
                NavigationViewItem::new()
                    .tag(tag)
                    .is_selected(self.page == tag)
                    .slots([SlotView::new(
                        NavigationViewItemSlot::Content,
                        TextBlock::new().text(label),
                    )]),
            )
        });
        let body = match self.page.as_str() {
            "settings" => TextBlock::new().text("Settings page"),
            "about" => TextBlock::new().text("About page"),
            _ => TextBlock::new().text("Home page"),
        };

        NavigationView::new()
            .pane_display_mode(NavigationViewPaneDisplayMode::Left)
            .pane_title("Demo")
            .is_settings_visible(false)
            .on_selected_tag_changed(context.callback(|page| page))
            .slots([
                SlotView::collection(NavigationViewSlot::MenuItems, items),
                SlotView::new(NavigationViewSlot::Content, body),
                SlotView::new(
                    NavigationViewSlot::Header,
                    TextBlock::new().text(format!("page: {}", self.page)),
                ),
            ])
    }
}

fn main() {
    App::run_component::<NavigationViewSample>(()).unwrap();
}
