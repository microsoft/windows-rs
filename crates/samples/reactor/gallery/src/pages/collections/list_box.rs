use crate::controls::*;
use windows_reactor::*;

pub struct ListBoxPage {
    selected: Option<usize>,
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
        Self { selected: None }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Selected(tag) => {
                self.selected = tag.and_then(|tag| tag.parse().ok());
            }
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        let label = self
            .selected
            .and_then(|index| FRUITS.get(index))
            .copied()
            .unwrap_or("(none)");
        let items = FRUITS.into_iter().enumerate().map(|(index, name)| {
            let tag = index.to_string();
            KeyedView::new(
                tag.clone(),
                ListBoxItem::new()
                    .tag(tag)
                    .is_selected(self.selected == Some(index))
                    .content(name),
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
                            .content(name),
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
                                .collection_slot(ListBoxSlot::Items, items),
                            TextBlock::new()
                                .text(format!("Selected: {label}"))
                                .opacity(0.6),
                        )),
                        r#"ListBox::new().on_selected_tag_changed(...)
    .collection_slot(ListBoxSlot::Items, items)"#,
                    ),
                ),
                KeyedView::new(
                    "disabled-list-box",
                    sample_card(
                        "Disabled ListBox",
                        ListBox::new()
                            .is_enabled(false)
                            .collection_slot(ListBoxSlot::Items, disabled_items),
                        r#"ListBox::new().is_enabled(false)
    .collection_slot(ListBoxSlot::Items, items)"#,
                    ),
                ),
            ],
        )
    }
}
