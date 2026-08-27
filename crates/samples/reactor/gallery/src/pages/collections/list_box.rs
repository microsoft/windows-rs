use crate::controls::*;
use windows_reactor::*;

pub struct ListBoxPage {
    selected: i32,
}

#[derive(Clone)]
pub enum Message {
    Selected(Option<String>),
}

const FRUITS: [&str; 5] = ["Apple", "Banana", "Cherry", "Date", "Elderberry"];

impl Component for ListBoxPage {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self { selected: -1 }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Selected(tag) => {
                self.selected = tag.and_then(|tag| tag.parse().ok()).unwrap_or(-1);
            }
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        let label = if self.selected >= 0 {
            FRUITS.get(self.selected as usize).copied().unwrap_or("?")
        } else {
            "(none)"
        };
        let items = FRUITS.into_iter().enumerate().map(|(index, name)| {
            let tag = index.to_string();
            KeyedView::new(
                tag.clone(),
                ListBoxItem::new()
                    .tag(tag)
                    .is_selected(self.selected == index as i32)
                    .content(TextBlock::new().text(name)),
            )
        });
        let disabled_items =
            ["Read", "Only", "Items"]
                .into_iter()
                .enumerate()
                .map(|(index, name)| {
                    let tag = index.to_string();
                    KeyedView::new(
                        tag.clone(),
                        ListBoxItem::new()
                            .tag(tag)
                            .is_selected(index == 0)
                            .content(TextBlock::new().text(name)),
                    )
                });

        page_content(
            "ListBox",
            "A list of selectable items presented inline.",
            [
                KeyedView::new(
                    "basic-list-box",
                    sample_card(
                        "Basic ListBox",
                        StackPanel::new().spacing(8.0).children((
                            ListBox::new()
                                .on_selected_tag_changed(context.callback(Message::Selected))
                                .slots([SlotView::collection(ListBoxSlot::Items, items)]),
                            TextBlock::new()
                                .text(format!("Selected: {label}"))
                                .opacity(0.6),
                        )),
                        r#"ListBox::new().on_selected_tag_changed(...)
    .slots([SlotView::collection(ListBoxSlot::Items, items)])"#,
                    ),
                ),
                KeyedView::new(
                    "disabled-list-box",
                    sample_card(
                        "Disabled ListBox",
                        ListBox::new()
                            .is_enabled(false)
                            .slots([SlotView::collection(ListBoxSlot::Items, disabled_items)]),
                        r#"ListBox::new().is_enabled(false)
    .slots([SlotView::collection(ListBoxSlot::Items, items)])"#,
                    ),
                ),
            ],
        )
    }
}
