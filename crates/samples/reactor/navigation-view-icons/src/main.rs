use std::rc::Rc;

use windows_reactor::*;

struct NavigationIconsSample {
    page: Rc<str>,
}

impl Component for NavigationIconsSample {
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
                    .icon(symbol),
            )
        };
        let content = match self.page.as_ref() {
            "home" => "Welcome home!",
            "settings" => "Settings page",
            "mail" => "Mail inbox",
            "people" => "Contacts",
            _ => "Unknown page",
        };

        context.window_title("NavigationViewIcons");
        NavigationView::new()
            .is_settings_visible(false)
            .on_selected_tag_changed(context.forward())
            .keyed_menu_items([
                item("home", "Home", Symbol::Home),
                item("mail", "Mail", Symbol::Mail),
                item("people", "People", Symbol::People),
                item("settings", "Settings", Symbol::Setting),
            ])
            .content(content)
            .into()
    }
}

fn main() {
    App::run_component::<NavigationIconsSample>(()).unwrap();
}
