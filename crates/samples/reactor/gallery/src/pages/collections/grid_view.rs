use crate::controls::*;
use windows_reactor::*;

pub struct GridViewPage {
    selected: Option<usize>,
}

#[derive(Clone)]
pub enum Message {
    Selected(Option<usize>),
}

impl Component for GridViewPage {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self { selected: None }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Selected(index) => self.selected = index,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        let items: Vec<String> = (1..=12).map(|i| format!("Item {i}")).collect();
        let label = self.selected.map_or_else(
            || "No selection".to_string(),
            |index| format!("Selected: Item {}", index + 1),
        );

        page_content(
            "GridView",
            "Displays items in a horizontally wrapping grid.",
            [KeyedView::new(
                "selectable-grid-view",
                sample_card(
                    "Selectable GridView",
                    StackPanel::new().spacing(8.0).children((
                        GridView::new()
                            .height(300.0)
                            .selected_index(self.selected)
                            .on_selection_changed(context.callback(Message::Selected))
                            .collection_slot(
                                GridViewSlot::Items,
                                items.iter().map(|item| {
                                    KeyedView::new(
                                        item.clone(),
                                        GridViewItem::new().tag(item).content(
                                            Border::new().padding(16.0).corner_radius(4.0).content(
                                                TextBlock::new()
                                                    .text(item)
                                                    .font_weight(FontWeight::BOLD),
                                            ),
                                        ),
                                    )
                                }),
                            ),
                        TextBlock::new().text(label).opacity(0.6),
                    )),
                    r#"GridView::new()
    .selected_index(selected)
    .on_selection_changed(handler)
    .collection_slot(GridViewSlot::Items, selectable_items)"#,
                ),
            )],
        )
    }
}
