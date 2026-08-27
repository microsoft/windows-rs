use std::rc::Rc;
use windows_reactor::*;

enum Message {
    Select(i32),
    SelectMode(i32),
    Reorder(Rc<Vec<String>>),
}

struct ListViewSample {
    selected: i32,
    mode_index: i32,
    items: Vec<String>,
}

impl Component for ListViewSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            selected: -1,
            mode_index: 1,
            items: ["Red", "Green", "Blue", "Yellow", "Magenta"]
                .map(str::to_string)
                .to_vec(),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Select(index) => self.selected = index,
            Message::SelectMode(index) => self.mode_index = index,
            Message::Reorder(items) => self.items.clone_from(items.as_ref()),
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
        let label = usize::try_from(self.selected)
            .ok()
            .and_then(|index| self.items.get(index))
            .map_or("(none)", String::as_str);

        StackPanel::new().spacing(8.0).max_width(320.0).children((
            TextBlock::new().text("Selection Mode:"),
            ListView::new()
                .height(120.0)
                .selected_index(self.mode_index)
                .on_selection_changed(context.callback(Message::SelectMode))
                .slots([SlotView::collection(
                    ListViewSlot::Items,
                    mode_names.into_iter().map(|name| {
                        KeyedView::new(
                            name,
                            ListViewItem::new().tag(name).content(
                                Border::new()
                                    .padding(Thickness::xy(12.0, 4.0))
                                    .content(TextBlock::new().text(name)),
                            ),
                        )
                    }),
                )]),
            TextBlock::new().text("Items (drag to reorder):"),
            ListView::new()
                .height(180.0)
                .selected_index(self.selected)
                .selection_mode(modes[self.mode_index as usize])
                .can_drag_items(true)
                .can_reorder_items(true)
                .allow_drop(true)
                .on_selection_changed(context.callback(Message::Select))
                .on_reordered(context.callback(Message::Reorder))
                .slots([SlotView::collection(
                    ListViewSlot::Items,
                    self.items.iter().map(|item| {
                        KeyedView::new(
                            item.clone(),
                            ListViewItem::new().tag(item).content(
                                Border::new()
                                    .padding(Thickness::xy(12.0, 6.0))
                                    .content(TextBlock::new().text(item)),
                            ),
                        )
                    }),
                )]),
            TextBlock::new().text(format!(
                "selected_index = {} ({label}) | mode = {:?}",
                self.selected, modes[self.mode_index as usize]
            )),
        ))
    }
}

fn main() {
    App::run_component::<ListViewSample>(()).unwrap();
}
