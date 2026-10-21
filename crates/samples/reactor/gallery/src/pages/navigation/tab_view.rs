use crate::controls::*;
use windows_reactor::*;

pub struct TabViewPage {
    basic_selected: Option<usize>,
    basic_tabs: Vec<u32>,
    dynamic_selected: Option<usize>,
    dynamic_tabs: Vec<u32>,
    next_tab: u32,
}

#[derive(Clone)]
pub enum Message {
    BasicSelected(Option<usize>),
    CloseBasic(Key),
    ReorderBasic(Vec<Key>),
    CloseDynamic(Key),
    DynamicSelected(Option<usize>),
    Add,
    Remove,
}

fn tabs(tabs: &[u32], closable: bool) -> Vec<KeyedView> {
    tabs.iter()
        .map(|&index| {
            keyed(
                index,
                TabViewItem::new()
                    .header(format!("Tab {index}"))
                    .is_closable(closable)
                    .content(format!("Content of tab {index}")),
            )
        })
        .collect()
}

fn remove_tab(tabs: &mut Vec<u32>, selected: &mut Option<usize>, key: &Key) {
    let Some(index) = tabs.iter().position(|&tab| Key::from(tab) == *key) else {
        return;
    };
    tabs.remove(index);
    *selected = match *selected {
        _ if tabs.is_empty() => None,
        Some(current) if index < current => Some(current - 1),
        Some(current) if index == current => Some(index.min(tabs.len() - 1)),
        current => current,
    };
}

impl Component for TabViewPage {
    type Message = Message;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            basic_selected: Some(0),
            basic_tabs: vec![1, 2, 3],
            dynamic_selected: Some(0),
            dynamic_tabs: vec![1, 2, 3],
            next_tab: 4,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::BasicSelected(value) => self.basic_selected = value,
            Message::CloseBasic(key) => {
                remove_tab(&mut self.basic_tabs, &mut self.basic_selected, &key);
            }
            Message::ReorderBasic(order) => {
                let selected = self.basic_selected.map(|index| self.basic_tabs[index]);
                self.basic_tabs
                    .sort_by_key(|&tab| order.iter().position(|key| *key == Key::from(tab)));
                self.basic_selected =
                    selected.and_then(|id| self.basic_tabs.iter().position(|&tab| tab == id));
            }
            Message::CloseDynamic(key) => {
                remove_tab(&mut self.dynamic_tabs, &mut self.dynamic_selected, &key);
            }
            Message::DynamicSelected(value) => self.dynamic_selected = value,
            Message::Add => {
                self.dynamic_tabs.push(self.next_tab);
                self.dynamic_selected = Some(self.dynamic_tabs.len() - 1);
                self.next_tab += 1;
            }
            Message::Remove => {
                if self.dynamic_tabs.len() > 1
                    && let Some(selected) = self.dynamic_selected
                {
                    let key = Key::from(self.dynamic_tabs[selected]);
                    remove_tab(&mut self.dynamic_tabs, &mut self.dynamic_selected, &key);
                }
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "TabView",
            "A control that displays closable, rearrangeable tabs.",
            [
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Basic TabView",
                        TabView::new()
                            .selected_index(self.basic_selected)
                            .can_reorder_tabs(true)
                            .height(200.0)
                            .on_selection_changed(context.callback(Message::BasicSelected))
                            .on_close_requested(context.callback(Message::CloseBasic))
                            .on_reordered(context.callback(Message::ReorderBasic))
                            .keyed_tab_items(tabs(&self.basic_tabs, true)),
                    ),
                ),
                KeyedView::new(
                    "dynamic",
                    sample_card(
                        "Dynamic Tabs",
                        StackPanel::new().spacing(8.0).children((
                            TabView::new()
                                .selected_index(self.dynamic_selected)
                                .height(180.0)
                                .on_selection_changed(context.callback(Message::DynamicSelected))
                                .on_close_requested(context.callback(Message::CloseDynamic))
                                .keyed_tab_items(tabs(&self.dynamic_tabs, true)),
                            StackPanel::new()
                                .orientation(Orientation::Horizontal)
                                .spacing(8.0)
                                .children((
                                    Button::new()
                                        .on_click(context.message(Message::Add))
                                        .content("Add tab"),
                                    Button::new()
                                        .is_enabled(self.dynamic_tabs.len() > 1)
                                        .on_click(context.message(Message::Remove))
                                        .content("Remove tab"),
                                )),
                        )),
                    ),
                ),
                KeyedView::new(
                    "fixed",
                    sample_card(
                        "Non-closable Tabs",
                        TabView::new()
                            .height(150.0)
                            .keyed_tab_items(tabs(&[1, 2], false)),
                    ),
                ),
            ],
        )
    }
}
