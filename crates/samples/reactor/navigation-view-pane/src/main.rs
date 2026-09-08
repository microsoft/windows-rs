use windows_reactor::*;

struct NavigationPaneSample {
    page: String,
}

impl Component for NavigationPaneSample {
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
                    .content(label)
                    .icon(SymbolIcon::new().symbol(symbol)),
            )
        };
        let body = match self.page.as_str() {
            "docs" => "Documents page",
            "settings" => "Settings page",
            _ => "Home page",
        };

        context.window_title("NavigationView pane");
        NavigationView::new()
            .pane_display_mode(NavigationViewPaneDisplayMode::Left)
            .pane_title("Account")
            .open_pane_length(400.0)
            .is_settings_visible(false)
            .on_selected_tag_changed(context.forward())
            .menu_items([
                item("home", "Home", Symbol::Home),
                item("docs", "Documents", Symbol::Document),
            ])
            .footer_menu_items([item("settings", "Settings", Symbol::Setting)])
            .content(body)
            .pane_footer(
                Button::new()
                    .on_click(|| println!("signed out"))
                    .content("Sign out"),
            )
            .into()
    }
}

fn main() {
    App::run_component::<NavigationPaneSample>(()).unwrap();
}
