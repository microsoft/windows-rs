#![windows_subsystem = "windows"]

use std::result::Result as StdResult;
use std::time::Duration;

use windows_reactor::*;

fn save_data(name: &str) -> StdResult<String, String> {
    std::thread::sleep(Duration::from_millis(800));
    if name.is_empty() {
        Err("Name cannot be empty".to_string())
    } else {
        Ok(format!("Saved '{name}' successfully"))
    }
}

enum SaveState {
    Idle,
    Loading,
    Success(String),
    Error(String),
    Rejected,
}

struct UseMutationSample {
    name: String,
    save_state: SaveState,
    task: Option<ComponentTask>,
}

#[derive(Clone)]
enum Message {
    NameChanged(String),
    Save,
    SaveEmpty,
    Completed(StdResult<String, String>),
    Rejected,
}

impl Component for UseMutationSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            name: "Hello".into(),
            save_state: SaveState::Idle,
            task: None,
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::NameChanged(name) => self.name = name,
            Message::Save | Message::SaveEmpty if matches!(self.save_state, SaveState::Loading) => {
            }
            Message::Save => {
                self.save_state = SaveState::Loading;
                let name = self.name.clone();
                self.task = Some(context.spawn_background_with_rejection(
                    move |_| Message::Completed(save_data(&name)),
                    Message::Rejected,
                ));
            }
            Message::SaveEmpty => {
                self.save_state = SaveState::Loading;
                self.task = Some(context.spawn_background_with_rejection(
                    move |_| Message::Completed(save_data("")),
                    Message::Rejected,
                ));
            }
            Message::Completed(Ok(message)) => {
                drop(self.task.take());
                self.save_state = SaveState::Success(message);
            }
            Message::Completed(Err(error)) => {
                drop(self.task.take());
                self.save_state = SaveState::Error(error);
            }
            Message::Rejected => {
                drop(self.task.take());
                self.save_state = SaveState::Rejected;
            }
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("UseMutation");
        let loading = matches!(self.save_state, SaveState::Loading);
        let status: View = match &self.save_state {
            SaveState::Idle => TextBlock::new().text("Ready to save").into(),
            SaveState::Loading => ProgressRing::new().is_indeterminate(true).into(),
            SaveState::Success(message) => TextBlock::new().text(message).into(),
            SaveState::Error(error) => TextBlock::new().text(format!("Error: {error}")).into(),
            SaveState::Rejected => TextBlock::new()
                .text("Error: background task rejected")
                .into(),
        };

        StackPanel::new().spacing(12.0).children((
            TextBlock::new().text("use_mutation Demo").font_size(24.0),
            TextBox::new()
                .text(self.name.clone())
                .on_text_changed(context.callback(Message::NameChanged))
                .slots([SlotView::new(
                    TextBoxSlot::Header,
                    TextBlock::new().text("Name"),
                )]),
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .children((
                    Button::new()
                        .is_enabled(!loading)
                        .on_click(context.message(Message::Save))
                        .content(TextBlock::new().text("Save")),
                    Button::new()
                        .is_enabled(!loading)
                        .on_click(context.message(Message::SaveEmpty))
                        .content(TextBlock::new().text("Save Empty (error)")),
                )),
            status,
        ))
    }
}

fn main() {
    App::run_component::<UseMutationSample>(()).unwrap();
}
