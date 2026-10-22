use super::*;

pub(super) struct ListFixture {
    keys: Vec<&'static str>,
    selectable: bool,
}

#[derive(Clone)]
pub(super) enum Message {
    Reverse,
    Insert,
    Remove,
    InsertFirst,
    EnableSelection,
}

impl Component for ListFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            keys: vec!["alpha", "beta", "rich"],
            selectable: false,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Reverse => self.keys.reverse(),
            Message::Insert => self.keys.insert(1, "delta"),
            Message::Remove => self.keys.retain(|key| *key != "beta"),
            Message::InsertFirst => self.keys.insert(0, "gamma"),
            Message::EnableSelection => self.selectable = true,
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
                ListView::new().automation_id("items").items(items),
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
