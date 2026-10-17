use super::*;

pub(super) struct WindowFixture {
    resized: bool,
    icon: bool,
    observed: WindowSize,
}

#[derive(Clone, Copy)]
pub(super) enum Message {
    Resize(bool),
    Icon(bool),
    Size(WindowSize),
}

impl Component for WindowFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            resized: false,
            icon: false,
            observed: WindowSize::default(),
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Resize(resized) => self.resized = resized,
            Message::Icon(icon) => self.icon = icon,
            Message::Size(size) => self.observed = size,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.on_window_size(context.callback(Message::Size));
        let (title, width, height) = if self.resized {
            ("Reactor UI fixture: window updated", 640.0, 400.0)
        } else {
            ("Reactor UI fixture: window", 480.0, 320.0)
        };
        let mut visuals = WindowVisuals::new().client_size(width, height);
        if self.icon {
            visuals = visuals.icon(concat!(env!("CARGO_MANIFEST_DIR"), "\\icon.ico"));
        }
        context.window_visuals(visuals);
        context.window_title(title);
        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                Button::new()
                    .automation_id("resize")
                    .content("Update title and size")
                    .on_click(context.message(Message::Resize(true))),
                Button::new()
                    .automation_id("reset")
                    .content("Restore title and size")
                    .on_click(context.message(Message::Resize(false))),
                Button::new()
                    .automation_id("set-icon")
                    .content("Set icon")
                    .on_click(context.message(Message::Icon(true))),
                Button::new()
                    .automation_id("clear-icon")
                    .content("Clear icon")
                    .on_click(context.message(Message::Icon(false))),
                TextBlock::new().automation_id("size").text(format!(
                    "Size {:.3} x {:.3}",
                    self.observed.width, self.observed.height
                )),
            ))
            .into()
    }
}
