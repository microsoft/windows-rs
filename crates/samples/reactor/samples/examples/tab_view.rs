use windows_reactor::*;

struct TabViewSample {
    tabs: Vec<(&'static str, &'static str)>,
    selected: Option<usize>,
}

#[derive(Clone)]
enum Message {
    Selected(Option<usize>),
    Close(String),
    Reordered(Vec<String>),
}

impl Component for TabViewSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            tabs: vec![
                ("overview", "Overview"),
                ("badges", "Badges"),
                ("notice", "Notice"),
            ],
            selected: Some(0),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Selected(index) => self.selected = index,
            Message::Close(key) => {
                self.tabs.retain(|(k, _)| *k != key);
                self.selected = self
                    .selected
                    .map(|selected| selected.min(self.tabs.len().saturating_sub(1)))
                    .filter(|_| !self.tabs.is_empty());
            }
            Message::Reordered(order) => {
                if order.len() == self.tabs.len()
                    && order
                        .iter()
                        .all(|key| self.tabs.iter().any(|(candidate, _)| candidate == key))
                {
                    self.tabs.sort_by_key(|(key, _)| {
                        order.iter().position(|candidate| candidate == key).unwrap()
                    });
                }
            }
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("TabView");
        let items = self.tabs.iter().map(|(key, header)| {
            let closable = *key != "overview";
            KeyedView::new(
                *key,
                TabViewItem::new()
                    .header(*header)
                    .tag(*key)
                    .is_closable(closable)
                    .content(
                        Border::new()
                            .padding(Thickness::uniform(12.0))
                            .content(format!("Tab content - {header}")),
                    ),
            )
        });

        StackPanel::new().spacing(8.0).children((
            TabView::new()
                .selected_index(self.selected)
                .can_reorder_tabs(true)
                .on_selection_changed(context.callback(Message::Selected))
                .on_close_requested(context.callback(Message::Close))
                .on_reordered(context.callback(Message::Reordered))
                .collection_slot(TabViewSlot::TabItems, items),
            format!(
                "selected_index = {:?}, tabs remaining = {}",
                self.selected,
                self.tabs.len()
            ),
        ))
    }
}

fn main() {
    App::run_component::<TabViewSample>(()).unwrap();
}
