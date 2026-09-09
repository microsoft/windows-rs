#![windows_subsystem = "windows"]

use windows_reactor::*;
use windows_sys::{Win32::*, core::w};

#[derive(Clone, Copy)]
enum Message {
    Confirm,
    Answered(i32),
}

struct MessageBoxSample {
    status: String,
}

impl Component for MessageBoxSample {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            status: "No answer yet".to_string(),
        }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Confirm => {
                self.status = "Waiting for an answer...".to_string();
                if !context.run_window(|window| {
                    let answer = unsafe {
                        MessageBoxW(
                            window.as_raw(),
                            w!("Continue with this operation?"),
                            w!("Confirm"),
                            (MB_YESNO | MB_ICONQUESTION) as u32,
                        )
                    };
                    Message::Answered(answer)
                }) {
                    self.status = "Another window operation is pending".to_string();
                }
            }
            Message::Answered(IDYES) => {
                self.status = "You chose Yes".to_string();
            }
            Message::Answered(IDNO) => {
                self.status = "You chose No".to_string();
            }
            Message::Answered(_) => {
                self.status = "MessageBoxW failed".to_string();
            }
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Native message box");

        StackPanel::new().spacing(8.0).children((
            Button::new()
                .on_click(context.message(Message::Confirm))
                .content("Show message box"),
            self.status.clone(),
        ))
    }
}

fn main() {
    App::run_component::<MessageBoxSample>(()).unwrap();
}
