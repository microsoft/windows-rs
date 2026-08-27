use crate::controls::*;
use windows_reactor::*;

pub struct TabViewPage {
    basic_selected: i32,
    basic_tabs: Vec<u32>,
    dynamic_selected: i32,
    dynamic_tabs: Vec<u32>,
    next_tab: u32,
}

#[derive(Clone)]
pub enum Message {
    BasicSelected(i32),
    CloseBasic(String),
    CloseDynamic(String),
    DynamicSelected(i32),
    Add,
    Remove,
}

fn tabs(tabs: &[u32], closable: bool) -> Vec<KeyedView> {
    tabs.iter()
        .map(|index| {
            let tag = format!("tab-{index}");
            KeyedView::new(
                tag.clone(),
                TabViewItem::new()
                    .tag(tag)
                    .header(format!("Tab {index}"))
                    .is_closable(closable)
                    .content(TextBlock::new().text(format!("Content of tab {index}"))),
            )
        })
        .collect()
}

fn remove_tab(tabs: &mut Vec<u32>, selected: &mut i32, tag: &str) {
    let Some(index) = tabs.iter().position(|tab| tag == format!("tab-{tab}")) else {
        return;
    };
    tabs.remove(index);
    if tabs.is_empty() {
        *selected = -1;
    } else if index < *selected as usize {
        *selected -= 1;
    } else if index == *selected as usize {
        *selected = (index.min(tabs.len() - 1)) as i32;
    }
}

impl Component for TabViewPage {
    type Message = Message;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            basic_selected: 0,
            basic_tabs: vec![1, 2, 3],
            dynamic_selected: 0,
            dynamic_tabs: vec![1, 2, 3],
            next_tab: 4,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::BasicSelected(value) => self.basic_selected = value,
            Message::CloseBasic(tag) => {
                remove_tab(&mut self.basic_tabs, &mut self.basic_selected, &tag);
            }
            Message::CloseDynamic(tag) => {
                remove_tab(&mut self.dynamic_tabs, &mut self.dynamic_selected, &tag);
            }
            Message::DynamicSelected(value) => self.dynamic_selected = value,
            Message::Add => {
                self.dynamic_tabs.push(self.next_tab);
                self.dynamic_selected = self.dynamic_tabs.len() as i32 - 1;
                self.next_tab += 1;
            }
            Message::Remove => {
                if self.dynamic_tabs.len() > 1 {
                    let tag = format!("tab-{}", self.dynamic_tabs[self.dynamic_selected as usize]);
                    remove_tab(&mut self.dynamic_tabs, &mut self.dynamic_selected, &tag);
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
                            .slots([SlotView::collection(
                                TabViewSlot::TabItems,
                                tabs(&self.basic_tabs, true),
                            )]),
                        "TabView::new().selected_index(index).slots([tab_items])",
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
                                .slots([SlotView::collection(
                                    TabViewSlot::TabItems,
                                    tabs(&self.dynamic_tabs, true),
                                )]),
                            StackPanel::new()
                                .orientation(Orientation::Horizontal)
                                .spacing(8.0)
                                .children((
                                    Button::new()
                                        .on_click(context.message(Message::Add))
                                        .content(TextBlock::new().text("Add tab")),
                                    Button::new()
                                        .is_enabled(self.dynamic_tabs.len() > 1)
                                        .on_click(context.message(Message::Remove))
                                        .content(TextBlock::new().text("Remove tab")),
                                )),
                        )),
                        "Build keyed TabViewItem values from component state.",
                    ),
                ),
                KeyedView::new(
                    "fixed",
                    sample_card(
                        "Non-closable Tabs",
                        TabView::new().height(150.0).slots([SlotView::collection(
                            TabViewSlot::TabItems,
                            tabs(&[1, 2], false),
                        )]),
                        "TabViewItem::new().is_closable(false)",
                    ),
                ),
            ],
        )
    }
}
