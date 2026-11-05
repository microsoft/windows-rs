use crate::controls::*;
use windows_reactor::*;

pub struct GridViewPage {
    selected: Option<usize>,
}

impl Component for GridViewPage {
    type Message = Option<usize>;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self { selected: None }
    }

    fn update(&mut self, selected: Option<usize>, _context: &ComponentContext<Self>) {
        self.selected = selected;
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
                            .on_selection_changed(context.forward())
                            .keyed_items(items.iter().map(|item| {
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
                            })),
                        TextBlock::new().text(label).opacity(0.6),
                    )),
                ),
            )],
        )
    }
}
