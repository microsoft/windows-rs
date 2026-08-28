use windows_reactor::*;

struct GridViewSample {
    items: Vec<String>,
}

impl Component for GridViewSample {
    type Message = Vec<String>;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            items: ["Red", "Green", "Blue", "Yellow", "Magenta", "Cyan"]
                .map(str::to_string)
                .to_vec(),
        }
    }

    fn update(&mut self, items: Vec<String>, _context: &ComponentContext<Self>) {
        self.items = items;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("GridView");
        GridView::new()
            .height(220.0)
            .can_drag_items(true)
            .can_reorder_items(true)
            .allow_drop(true)
            .on_reordered(context.forward())
            .collection_slot(
                GridViewSlot::Items,
                self.items.iter().map(|item| {
                    KeyedView::new(
                        item.clone(),
                        GridViewItem::new().tag(item).content(
                            Border::new()
                                .background(Color::rgb(220, 230, 245))
                                .padding(Thickness::uniform(10.0))
                                .width(110.0)
                                .height(70.0)
                                .content(TextBlock::new().text(item).font_size(12.0)),
                        ),
                    )
                }),
            )
    }
}

fn main() {
    App::run_component::<GridViewSample>(()).unwrap();
}
