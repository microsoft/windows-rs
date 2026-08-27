use windows_reactor::*;

struct IconElementsSample {
    page: String,
}

impl Component for IconElementsSample {
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
        let image = format!(
            "file:///{}/examples/image.svg",
            env!("CARGO_MANIFEST_DIR").replace('\\', "/"),
        );
        let bitmap = format!(
            "file:///{}/examples/image.png",
            env!("CARGO_MANIFEST_DIR").replace('\\', "/"),
        );
        let item = |tag, label, icon: View| {
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
                        SlotView::new(NavigationViewItemSlot::Icon, icon),
                    ]),
            )
        };
        let content = match self.page.as_str() {
            "home" => "Symbol icon (SymbolIcon).",
            "starred" => "Font-glyph icon (FontIcon).",
            "repo" => "SVG image icon (ImageIcon).",
            "bitmap" => "Foreground-tinted bitmap mask (BitmapIcon).",
            "path" => "Vector path data (PathIcon).",
            _ => "Unknown page",
        };

        context.window_title("IconElements");
        NavigationView::new()
            .is_settings_visible(false)
            .on_selected_tag_changed(context.callback(std::convert::identity))
            .slots([
                SlotView::collection(
                    NavigationViewSlot::MenuItems,
                    [
                        item(
                            "home",
                            "Home",
                            SymbolIcon::new().symbol(Symbol::Home).into(),
                        ),
                        item(
                            "starred",
                            "Starred",
                            FontIcon::new().glyph("\u{E734}").into(),
                        ),
                        item(
                            "repo",
                            "Repository",
                            ImageIcon::new().source(image).unwrap().into(),
                        ),
                        item(
                            "bitmap",
                            "Bitmap mask",
                            BitmapIcon::new()
                                .uri_source(bitmap)
                                .unwrap()
                                .show_as_monochrome(true)
                                .into(),
                        ),
                        item(
                            "path",
                            "Path",
                            PathIcon::new()
                                .data("F1 M 0,8 L 6,14 L 16,2 L 14,0 L 6,10 L 2,6 Z")
                                .into(),
                        ),
                    ],
                ),
                SlotView::new(NavigationViewSlot::Content, TextBlock::new().text(content)),
            ])
    }
}

fn main() {
    App::run_component::<IconElementsSample>(()).unwrap();
}
