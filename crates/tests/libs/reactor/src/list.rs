use super::*;

pub(super) struct ListFixture {
    keys: Vec<&'static str>,
    selectable: bool,
    selected: Option<&'static str>,
}

#[derive(Clone)]
pub(super) enum Message {
    Reverse,
    Insert,
    Remove,
    InsertFirst,
    EnableSelection,
    Clicked(Option<usize>),
    Promote(&'static str),
}

impl Component for ListFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            keys: vec!["alpha", "beta", "rich"],
            selectable: false,
            selected: None,
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Reverse => self.keys.reverse(),
            Message::Insert => self.keys.insert(1, "delta"),
            Message::Remove => self.keys.retain(|key| *key != "beta"),
            Message::InsertFirst => self.keys.insert(0, "gamma"),
            Message::EnableSelection => self.selectable = true,
            Message::Clicked(Some(index)) => {
                let key = self.keys[index];
                // Promote after a delay, as a background load would. Promoting immediately does not crash.
                context.spawn_background(move |_| {
                    std::thread::sleep(std::time::Duration::from_millis(300));
                    Message::Promote(key)
                });
            }
            Message::Clicked(None) => {}
            Message::Promote(key) => {
                self.keys.retain(|item| *item != key);
                self.keys.insert(0, key);
                self.selected = Some(key);
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let items = self.keys.iter().map(|&key| {
            let item = DataItem::new(key, title(key));
            if key == "rich" {
                item.content(
                    TextBlock::new()
                        .automation_id("rich-content")
                        .text("Rich content"),
                )
            } else {
                item
            }
        });
        let selectable = ListView::new()
            .automation_id("selectable")
            .selection_mode(if self.selectable {
                ListViewSelectionMode::Single
            } else {
                ListViewSelectionMode::None
            })
            .selected_index(self.selectable.then_some(2))
            .items(["one", "two", "three"].map(|key| DataItem::new(key, title(key))));
        context.window_title("Reactor UI fixture: list");
        context.window_visuals(WindowVisuals::new().client_size(600.0, 700.0));
        StackPanel::new()
            .margin(20.0)
            .spacing(8.0)
            .children((
                ListView::new()
                    .automation_id("mounted")
                    .selection_mode(ListViewSelectionMode::Single)
                    .selected_index(Some(1))
                    .items(["first", "second"].map(|key| DataItem::new(key, title(key)))),
                ListView::new()
                    .automation_id("items")
                    .selected_index(
                        self.selected
                            .and_then(|selected| self.keys.iter().position(|key| *key == selected)),
                    )
                    .on_selection_changed(context.callback(Message::Clicked))
                    .items(items),
                selectable,
                Button::new()
                    .automation_id("reverse")
                    .content("Reverse")
                    .on_click(context.message(Message::Reverse)),
                Button::new()
                    .automation_id("insert")
                    .content("Insert")
                    .on_click(context.message(Message::Insert)),
                Button::new()
                    .automation_id("remove")
                    .content("Remove")
                    .on_click(context.message(Message::Remove)),
                Button::new()
                    .automation_id("insert-first")
                    .content("Insert first")
                    .on_click(context.message(Message::InsertFirst)),
                Button::new()
                    .automation_id("enable-selection")
                    .content("Enable selection")
                    .on_click(context.message(Message::EnableSelection)),
            ))
            .into()
    }
}

fn title(key: &str) -> String {
    let mut title = key.to_string();
    title[..1].make_ascii_uppercase();
    title
}
