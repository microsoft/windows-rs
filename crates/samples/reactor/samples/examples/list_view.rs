use windows_reactor::*;

enum Message {
    Select(Option<usize>),
    SelectMode(Option<usize>),
    Reorder(Vec<String>),
}

struct ListViewSample {
    selected: Option<usize>,
    mode_index: Option<usize>,
    items: Vec<String>,
}

impl Component for ListViewSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            selected: None,
            mode_index: Some(1),
            items: ["Red", "Green", "Blue", "Yellow", "Magenta"]
                .map(str::to_string)
                .to_vec(),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Select(index) => self.selected = index,
            Message::SelectMode(index) => self.mode_index = index,
            Message::Reorder(items) => self.items = items,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("ListView");
        let modes = [
            ListViewSelectionMode::None,
            ListViewSelectionMode::Single,
            ListViewSelectionMode::Multiple,
            ListViewSelectionMode::Extended,
        ];
        let mode_names = ["None", "Single", "Multiple", "Extended"];
        let label = self
            .selected
            .and_then(|index| self.items.get(index))
            .map_or("(none)", String::as_str);

        StackPanel::new().spacing(8.0).max_width(320.0).children((
            "Selection Mode:",
            ListView::new()
                .height(120.0)
                .selected_index(self.mode_index)
                .on_selection_changed(context.callback(Message::SelectMode))
                .collection_slot(
                    ListViewSlot::Items,
                    mode_names.into_iter().map(|name| {
                        KeyedView::new(
                            name,
                            ListViewItem::new().tag(name).content(
                                Border::new()
                                    .padding(Thickness::xy(12.0, 4.0))
                                    .content(name),
                            ),
                        )
                    }),
                ),
            "Items (drag to reorder):",
            ListView::new()
                .height(180.0)
                .selected_index(self.selected)
                .selection_mode(modes[self.mode_index.unwrap_or_default()])
                .can_drag_items(true)
                .can_reorder_items(true)
                .allow_drop(true)
                .on_selection_changed(context.callback(Message::Select))
                .on_reordered(context.callback(Message::Reorder))
                .collection_slot(
                    ListViewSlot::Items,
                    self.items.iter().map(|item| {
                        KeyedView::new(
                            item.clone(),
                            ListViewItem::new().tag(item).content(
                                Border::new()
                                    .padding(Thickness::xy(12.0, 6.0))
                                    .content(item.as_str()),
                            ),
                        )
                    }),
                ),
            format!(
                "selected_index = {:?} ({label}) | mode = {:?}",
                self.selected,
                modes[self.mode_index.unwrap_or_default()]
            ),
        ))
    }
}

fn main() {
    App::run_component::<ListViewSample>(()).unwrap();
}
