use std::rc::Rc;
use windows_reactor::*;

struct TabViewAddButtonSample {
    tabs: Vec<Tab>,
    next_id: u32,
    selected: i32,
}

#[derive(Clone)]
struct Tab {
    id: u32,
    label: String,
}

#[derive(Clone)]
enum Message {
    Selected(i32),
    Add,
    Close(String),
    Reordered(Rc<Vec<String>>),
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
            selected: 0,
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
                self.selected = self.tabs.len() as i32 - 1;
            }
            Message::Close(key) => {
                self.tabs.retain(|tab| tab.id.to_string() != key);
                self.selected = if self.tabs.is_empty() {
                    -1
                } else {
                    self.selected.min(self.tabs.len() as i32 - 1).max(0)
                };
            }
            Message::Reordered(order) => {
                if order.len() == self.tabs.len()
                    && order.iter().all(|key| {
                        self.tabs
                            .iter()
                            .any(|candidate| candidate.id.to_string() == *key)
                    })
                {
                    self.tabs.sort_by_key(|tab| {
                        order
                            .iter()
                            .position(|candidate| *candidate == tab.id.to_string())
                            .unwrap()
                    });
                }
            }
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("TabView Add Button");
        let items = self.tabs.iter().map(|tab| {
            let key = tab.id.to_string();
            KeyedView::new(
                key.clone(),
                TabViewItem::new()
                    .header(tab.label.as_str())
                    .tag(key)
                    .is_closable(true)
                    .content(
                        Border::new()
                            .padding(Thickness::uniform(12.0))
                            .content(TextBlock::new().text(format!("Content for {}", tab.label))),
                    ),
            )
        });

        StackPanel::new().spacing(8.0).children((
            TabView::new()
                .selected_index(self.selected)
                .is_add_tab_button_visible(true)
                .on_selection_changed(context.callback(Message::Selected))
                .on_add_tab_button_click(context.message(Message::Add))
                .on_close_requested(context.callback(Message::Close))
                .on_reordered(context.callback(Message::Reordered))
                .slots([SlotView::collection(TabViewSlot::TabItems, items)]),
            TextBlock::new().text(format!(
                "selected = {}, total tabs = {}",
                self.selected,
                self.tabs.len()
            )),
        ))
    }
}

fn main() {
    App::run_component::<TabViewAddButtonSample>(()).unwrap();
}
