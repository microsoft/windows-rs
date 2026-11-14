use super::*;
use reactor_process_monitor::details_view::{Column, DetailsView, Input, Rows};

#[derive(Clone, PartialEq)]
struct Source {
    keys: Rc<Vec<Key>>,
    tick: u32,
}

impl Rows for Source {
    fn len(&self) -> usize {
        self.keys.len()
    }
    fn key(&self, index: usize) -> Key {
        self.keys[index].clone()
    }
    fn cell(&self, index: usize, column: usize) -> String {
        if column == 0 {
            format!("Row {}", self.keys[index].as_str().unwrap())
        } else {
            format!("{}", self.tick + index as u32)
        }
    }
}

pub(super) struct Fixture {
    source: Source,
    selected: Option<Key>,
    revision: u64,
    sort: Option<usize>,
    timer: ComponentTimer,
}

#[derive(Clone)]
pub(super) enum Message {
    Select(Key),
    Sort(usize),
    Remove,
    Tick,
}

impl Component for Fixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), context: &ComponentContext<Self>) -> Self {
        Self {
            source: Source {
                keys: Rc::new((0..10_000).map(|i| Key::from(i.to_string())).collect()),
                tick: 0,
            },
            selected: None,
            revision: 0,
            sort: None,
            timer: context
                .set_local_timeout(std::time::Duration::from_millis(100), || Message::Tick),
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Select(key) => self.selected = Some(key),
            Message::Sort(column) => {
                Rc::make_mut(&mut self.source.keys).reverse();
                self.revision += 1;
                self.sort = Some(column);
            }
            Message::Remove => {
                let selected = self.selected.take();
                Rc::make_mut(&mut self.source.keys).retain(|key| Some(key) != selected.as_ref());
                self.revision += 1;
            }
            Message::Tick => {
                self.source.tick += 1;
                self.timer = context
                    .set_local_timeout(std::time::Duration::from_millis(100), || Message::Tick);
            }
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: canvas-details");
        context.window_visuals(WindowVisuals::new().client_size(600.0, 420.0));
        Grid::new()
            .rows([GridLength::Auto, GridLength::STAR])
            .children((
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .children((
                        TextBlock::new().automation_id("selected").text(
                            self.selected
                                .as_ref()
                                .and_then(Key::as_str)
                                .unwrap_or("None"),
                        ),
                        TextBlock::new()
                            .automation_id("sorted")
                            .text(format!("{:?}", self.sort)),
                        Button::new()
                            .automation_id("remove")
                            .content("Remove selected")
                            .on_click(context.message(Message::Remove)),
                    )),
                Border::new()
                    .grid_row(1)
                    .content(View::component::<DetailsView<Source>>(Input {
                        automation_id: Rc::from("custom-details"),
                        name: Rc::from("Custom details fixture"),
                        rows: self.source.clone(),
                        order_revision: self.revision,
                        columns: Rc::from([
                            Column {
                                title: "Name",
                                width: 180.0,
                                right_aligned: false,
                            },
                            Column {
                                title: "Value",
                                width: 180.0,
                                right_aligned: true,
                            },
                        ]),
                        selected: self.selected.clone(),
                        sort: self.sort.map(|column| (column, true)),
                        scheme: ColorScheme::Light,
                        on_select: context.callback(Message::Select),
                        on_sort: context.callback(Message::Sort),
                    })),
            ))
            .into()
    }
}
