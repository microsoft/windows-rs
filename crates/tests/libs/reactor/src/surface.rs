use super::*;

#[derive(Clone, Copy, Debug)]
pub(super) enum Surface {
    Info,
    Tip,
}

pub(super) struct SurfaceFixture {
    info: bool,
    tip: bool,
    adopt: bool,
    events: Vec<String>,
}

#[derive(Clone)]
pub(super) enum Message {
    Open(Surface, bool),
    Closed(Surface),
    Action,
    Reject,
}

impl Component for SurfaceFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            info: false,
            tip: false,
            adopt: true,
            events: Vec::new(),
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Open(Surface::Info, open) => self.info = open,
            Message::Open(Surface::Tip, open) => self.tip = open,
            Message::Closed(surface) => {
                self.events.push(format!("{surface:?} closed"));
                if self.adopt {
                    match surface {
                        Surface::Info => self.info = false,
                        Surface::Tip => self.tip = false,
                    }
                }
            }
            Message::Action => self.events.push("Tip action".to_string()),
            Message::Reject => self.adopt = false,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: surface");
        context.window_visuals(WindowVisuals::new().client_size(600.0, 500.0));
        let button = |id: &str, message| {
            Button::new()
                .automation_id(id)
                .content(id)
                .on_click(context.message(message))
        };
        StackPanel::new()
            .margin(20.0)
            .spacing(8.0)
            .children((
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .children((
                        button("show-info", Message::Open(Surface::Info, true)),
                        button("hide-info", Message::Open(Surface::Info, false)),
                        button("show-tip", Message::Open(Surface::Tip, true)),
                        button("hide-tip", Message::Open(Surface::Tip, false)),
                        button("reject", Message::Reject),
                    )),
                TextBlock::new()
                    .automation_id("state")
                    .text(format!("Info {}; tip {}", self.info, self.tip)),
                TextBlock::new()
                    .automation_id("events")
                    .text(format!("Events [{}]", self.events.join(", "))),
                InfoBar::new()
                    .automation_id("info")
                    .title("Info")
                    .message("Message")
                    .is_closable(true)
                    .is_open(self.info)
                    .on_closed(context.message(Message::Closed(Surface::Info))),
                TeachingTip::new()
                    .automation_id("tip")
                    .title("Tip")
                    .is_open(self.tip)
                    .action_button_content("Act")
                    .close_button_content("Dismiss")
                    .on_action_button_click(context.message(Message::Action))
                    .on_closed(context.message(Message::Closed(Surface::Tip))),
            ))
            .into()
    }
}
