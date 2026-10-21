use windows_reactor::*;

struct TabViewAddButtonSample {
    tabs: Vec<Tab>,
    next_id: u32,
    selected: Option<usize>,
}

#[derive(Clone)]
struct Tab {
    id: u32,
    label: String,
}

#[derive(Clone)]
enum Message {
    Selected(Option<usize>),
    Add,
    Close(Key),
    Reordered(Vec<Key>),
}

impl Component for TabViewAddButtonSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            tabs: vec![
                Tab {
                    id: 1,
                    label: "Tab 1".to_string(),
                },
                Tab {
                    id: 2,
                    label: "Tab 2".to_string(),
                },
            ],
            next_id: 3,
            selected: Some(0),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Selected(index) => self.selected = index,
            Message::Add => {
                self.tabs.push(Tab {
                    id: self.next_id,
                    label: format!("Tab {}", self.next_id),
                });
                self.next_id += 1;
                self.selected = Some(self.tabs.len() - 1);
            }
            Message::Close(key) => {
                self.tabs.retain(|tab| Key::from(tab.id) != key);
                self.selected = self
                    .selected
                    .map(|selected| selected.min(self.tabs.len().saturating_sub(1)))
                    .filter(|_| !self.tabs.is_empty());
            }
            Message::Reordered(order) => {
                let selected = self.selected.map(|index| self.tabs[index].id);
                self.tabs
                    .sort_by_key(|tab| order.iter().position(|key| *key == Key::from(tab.id)));
                self.selected =
                    selected.and_then(|id| self.tabs.iter().position(|tab| tab.id == id));
            }
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("TabView Add Button");
        let items = self.tabs.iter().map(|tab| {
            keyed(
                tab.id,
                TabViewItem::new()
                    .header(tab.label.as_str())
                    .is_closable(true)
                    .content(
                        Border::new()
                            .padding(Thickness::uniform(12.0))
                            .content(format!("Content for {}", tab.label)),
                    ),
            )
        });

        StackPanel::new()
            .spacing(8.0)
            .children((
                TabView::new()
                    .selected_index(self.selected)
                    .is_add_tab_button_visible(true)
                    .on_selection_changed(context.callback(Message::Selected))
                    .on_add_tab_button_click(context.message(Message::Add))
                    .on_close_requested(context.callback(Message::Close))
                    .on_reordered(context.callback(Message::Reordered))
                    .keyed_tab_items(items),
                format!(
                    "selected = {:?}, total tabs = {}",
                    self.selected,
                    self.tabs.len()
                ),
            ))
            .into()
    }
}

fn main() {
    App::run_component::<TabViewAddButtonSample>(()).unwrap();
}
