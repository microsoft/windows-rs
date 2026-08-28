#![windows_subsystem = "windows"]

use std::result::Result as StdResult;
use std::time::Duration;

use windows_reactor::*;

fn fetch_page(page: i32) -> StdResult<Vec<String>, String> {
    std::thread::sleep(Duration::from_secs(1));
    Ok((0..5)
        .map(|index| format!("Item {} (page {})", page * 5 + index + 1, page + 1))
        .collect())
}

enum ResourceState {
    Loading,
    Ready(Vec<String>),
    Error(String),
    Rejected,
}

struct UseResourceSample {
    page: i32,
    state: ResourceState,
    task: Option<ComponentTask>,
}

#[derive(Clone)]
enum Message {
    Previous,
    Next,
    Completed {
        page: i32,
        result: StdResult<Vec<String>, String>,
    },
    Rejected {
        page: i32,
    },
}

impl UseResourceSample {
    fn load(&mut self, page: i32, context: &ComponentContext<Self>) {
        if let Some(task) = self.task.take() {
            task.cancel();
        }
        self.page = page;
        self.state = ResourceState::Loading;
        self.task = Some(context.spawn_background_with_rejection(
            move |_| Message::Completed {
                page,
                result: fetch_page(page),
            },
            Message::Rejected { page },
        ));
    }
}

impl Component for UseResourceSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        let mut sample = Self {
            page: 0,
            state: ResourceState::Loading,
            task: None,
        };
        sample.load(0, context);
        sample
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Previous if self.page > 0 => self.load(self.page - 1, context),
            Message::Next => self.load(self.page + 1, context),
            Message::Completed { page, result } if page == self.page => {
                self.task = None;
                self.state = match result {
                    Ok(items) => ResourceState::Ready(items),
                    Err(error) => ResourceState::Error(error),
                };
            }
            Message::Rejected { page } if page == self.page => {
                self.task = None;
                self.state = ResourceState::Rejected;
            }
            Message::Previous | Message::Completed { .. } | Message::Rejected { .. } => {}
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("UseResource");
        let content: View = match &self.state {
            ResourceState::Loading => ProgressRing::new().is_indeterminate(true).into(),
            ResourceState::Ready(items) => StackPanel::new().keyed_children(
                items
                    .iter()
                    .map(|item| KeyedView::new(item.clone(), item.as_str())),
            ),
            ResourceState::Error(error) => format!("Error: {error}").into(),
            ResourceState::Rejected => "Error: background task rejected".into(),
        };

        StackPanel::new().spacing(12.0).children((
            TextBlock::new()
                .text(format!("Page {}", self.page + 1))
                .font_size(24.0),
            content,
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .children((
                    Button::new()
                        .is_enabled(self.page > 0)
                        .on_click(context.message(Message::Previous))
                        .content("Previous"),
                    Button::new()
                        .on_click(context.message(Message::Next))
                        .content("Next"),
                )),
        ))
    }
}

fn main() {
    App::run_component::<UseResourceSample>(()).unwrap();
}
