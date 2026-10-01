use std::rc::Rc;

use windows_reactor::*;

struct NavigationPaneSample {
    page: Rc<str>,
}

impl Component for NavigationPaneSample {
    type Message = Option<Rc<str>>;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            page: "home".into(),
        }
    }

    fn update(&mut self, page: Option<Rc<str>>, _context: &ComponentContext<Self>) {
        if let Some(page) = page {
            self.page = page;
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let item = |tag: &'static str, label: &'static str, symbol| {
            KeyedView::new(
                tag,
                NavigationViewItem::new()
                    .tag(tag)
                    .is_selected(self.page.as_ref() == tag)
                    .content(label)
                    .icon(SymbolIcon::new().symbol(symbol)),
            )
        };
        let body = match self.page.as_ref() {
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
            .keyed_menu_items([
                item("home", "Home", Symbol::Home),
                item("docs", "Documents", Symbol::Document),
            ])
            .keyed_footer_menu_items([item("settings", "Settings", Symbol::Setting)])
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
