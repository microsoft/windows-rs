use windows_reactor::*;

struct SelectorBarSample {
    selected: String,
}

impl Component for SelectorBarSample {
    type Message = Option<String>;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            selected: "Recent".to_string(),
        }
    }

    fn update(&mut self, selected: Option<String>, _context: &ComponentContext<Self>) {
        if let Some(selected) = selected {
            self.selected = selected;
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let item = |text: &'static str, symbol| {
            let item = SelectorBarItem::new()
                .text(text)
                .is_selected(self.selected == text);
            let item = match symbol {
                Some(symbol) => item.slots([SlotView::new(
                    SelectorBarItemSlot::Icon,
                    SymbolIcon::new().symbol(symbol),
                )]),
                None => item.into(),
            };
            KeyedView::new(text, item)
        };

        context.window_title("SelectorBar");
        StackPanel::new().spacing(12.0).children((
            SelectorBar::new()
                .on_selected_text_changed(context.callback(std::convert::identity))
                .slots([SlotView::collection(
                    SelectorBarSlot::Items,
                    [
                        item("Recent", None),
                        item("Shared", Some(Symbol::People)),
                        item("Favorites", Some(Symbol::Favorite)),
                    ],
                )]),
            TextBlock::new().text(format!("Selected: {}", self.selected)),
        ))
    }
}

fn main() {
    App::run_component::<SelectorBarSample>(()).unwrap();
}
