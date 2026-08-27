#![windows_subsystem = "windows"]

use windows_reactor::*;

struct ListBoxSample {
    selected: i32,
}

impl Component for ListBoxSample {
    type Message = Option<String>;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { selected: -1 }
    }

    fn update(&mut self, tag: Self::Message, _context: &ComponentContext<Self>) {
        self.selected = tag.and_then(|tag| tag.parse().ok()).unwrap_or(-1);
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("ListBox");
        let items = ["Apple", "Banana", "Cherry", "Date", "Elderberry"]
            .into_iter()
            .enumerate()
            .map(|(index, label)| {
                let tag = index.to_string();
                KeyedView::new(
                    tag.clone(),
                    ListBoxItem::new()
                        .tag(tag)
                        .is_selected(self.selected == index as i32)
                        .content(TextBlock::new().text(label)),
                )
            });
        let label = if self.selected >= 0 {
            format!("Selected index: {}", self.selected)
        } else {
            "No selection".to_string()
        };

        StackPanel::new().spacing(8.0).children((
            ListBox::new()
                .on_selected_tag_changed(context.callback(|tag| tag))
                .slots([SlotView::collection(ListBoxSlot::Items, items)]),
            TextBlock::new().text(label),
        ))
    }
}

fn main() {
    App::run_component::<ListBoxSample>(()).unwrap();
}
