use std::rc::Rc;

use windows_reactor::*;

struct IconElementsSample {
    page: Rc<str>,
}

impl Component for IconElementsSample {
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
        let image = format!(
            "file:///{}/image.svg",
            env!("CARGO_MANIFEST_DIR").replace('\\', "/"),
        );
        let bitmap = format!(
            "file:///{}/image.png",
            env!("CARGO_MANIFEST_DIR").replace('\\', "/"),
        );
        let app_icon = Icon::image_uri(image).unwrap();
        let item = |tag: &'static str, label: &'static str, icon: Icon| {
            KeyedView::new(
                tag,
                NavigationViewItem::new()
                    .tag(tag)
                    .is_selected(self.page.as_ref() == tag)
                    .content(label)
                    .icon(icon),
            )
        };
        let content = match self.page.as_ref() {
            "home" => "Symbol icon (SymbolIcon).",
            "starred" => "Font-glyph icon (FontIcon).",
            "repo" => "SVG image icon (ImageIcon).",
            "bitmap" => "Foreground-tinted bitmap mask (BitmapIcon).",
            "path" => "Vector path data (PathIcon).",
            _ => "Unknown page",
        };

        context.window_title("IconElements");
        let navigation = NavigationView::new()
            .is_settings_visible(false)
            .on_selected_tag_changed(context.forward())
            .keyed_menu_items([
                item("home", "Home", Icon::symbol(Symbol::Home)),
                item("starred", "Starred", Icon::font("\u{E734}")),
                item("repo", "Repository", app_icon.clone()),
                item("bitmap", "Bitmap mask", Icon::bitmap(bitmap, true).unwrap()),
                item(
                    "path",
                    "Path",
                    Icon::path("F1 M 0,8 L 6,14 L 16,2 L 14,0 L 6,10 L 2,6 Z"),
                ),
            ])
            .content(content);

        Grid::new()
            .rows([GridLength::Auto, GridLength::STAR])
            .children((
                TitleBar::new()
                    .title("Icon slots")
                    .icon(app_icon)
                    .left_header(TextBlock::new().text("Left header"))
                    .content(
                        AutoSuggestBox::new()
                            .placeholder_text("Search...")
                            .query_icon(Icon::font("\u{E721}")),
                    ),
                navigation.grid_row(1),
            ))
            .into()
    }
}

fn main() {
    App::run_component::<IconElementsSample>(()).unwrap();
}
