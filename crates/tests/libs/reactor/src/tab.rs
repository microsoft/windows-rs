use super::*;

pub(super) struct TabFixture {
    tabs: Vec<u32>,
    next: u32,
    closed: Vec<Key>,
    reorders: u32,
}

#[derive(Clone)]
pub(super) enum Message {
    Add,
    Reverse,
    Close(Key),
    Reordered(Vec<Key>),
}

impl Component for TabFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            tabs: vec![1, 2, 3],
            next: 4,
            closed: Vec::new(),
            reorders: 0,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Add => {
                self.tabs.insert(1, self.next);
                self.next += 1;
            }
            Message::Reverse => self.tabs.reverse(),
            Message::Close(key) => {
                self.tabs.retain(|tab| Key::from(*tab) != key);
                self.closed.push(key);
            }
            Message::Reordered(order) => {
                self.reorders += 1;
                self.tabs
                    .sort_by_key(|tab| order.iter().position(|key| *key == Key::from(*tab)));
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let tabs = self.tabs.iter().map(|&tab| {
            keyed(
                tab,
                TabViewItem::new()
                    .automation_id(format!("tab-{tab}"))
                    .header(format!("Tab {tab}"))
                    .is_closable(true),
            )
        });
        let closed = self
            .closed
            .iter()
            .map(|key| {
                self.closed_tab(key)
                    .map_or_else(|| "?".to_string(), |tab| tab.to_string())
            })
            .collect::<Vec<_>>()
            .join(",");
        context.window_title("Reactor UI fixture: tab");
        StackPanel::new()
            .margin(20.0)
            .spacing(8.0)
            .children((
                TabView::new()
                    .automation_id("tabs")
                    .can_reorder_tabs(true)
                    .on_close_requested(context.callback(Message::Close))
                    .on_reordered(context.callback(Message::Reordered))
                    .keyed_tab_items(tabs),
                TextBlock::new().automation_id("order").text(
                    self.tabs
                        .iter()
                        .map(u32::to_string)
                        .collect::<Vec<_>>()
                        .join(","),
                ),
                TextBlock::new().automation_id("closed").text(closed),
                TextBlock::new()
                    .automation_id("reorders")
                    .text(self.reorders.to_string()),
                Button::new()
                    .automation_id("add")
                    .content("Add")
                    .on_click(context.message(Message::Add)),
                Button::new()
                    .automation_id("reverse")
                    .content("Reverse")
                    .on_click(context.message(Message::Reverse)),
            ))
            .into()
    }
}

impl TabFixture {
    fn closed_tab(&self, key: &Key) -> Option<u32> {
        (1..self.next).find(|tab| Key::from(*tab) == *key)
    }
}
