use super::*;

pub(super) struct WindowFixture {
    resized: Option<bool>,
    icon: bool,
    observed: WindowSize,
}

#[derive(Clone, Copy, PartialEq)]
pub(super) enum WindowMode {
    Native,
    Custom,
    CustomPolicy,
}

#[derive(Clone, Copy)]
pub(super) enum Message {
    Resize(bool),
    Icon(bool),
    Size(WindowSize),
}

impl Component for WindowFixture {
    type Input = WindowMode;
    type Message = Message;

    fn create(_: &WindowMode, _: &ComponentContext<Self>) -> Self {
        Self {
            resized: None,
            icon: false,
            observed: WindowSize::default(),
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Resize(resized) => self.resized = Some(resized),
            Message::Icon(icon) => self.icon = icon,
            Message::Size(size) => self.observed = size,
        }
    }

    fn view(&self, mode: &WindowMode, context: &mut ViewContext<Self>) -> View {
        context.on_window_size(context.callback(Message::Size));
        let resized = self.resized.unwrap_or(false);
        let (width, height) = if resized {
            (640.0, 400.0)
        } else {
            (480.0, 320.0)
        };
        let fixture = match mode {
            WindowMode::Native => "window",
            WindowMode::Custom => "window-frame",
            WindowMode::CustomPolicy => "window-frame-policy",
        };
        let suffix = if resized { " updated" } else { "" };
        let title = format!("Reactor UI fixture: {fixture}{suffix}");
        let mut visuals = WindowVisuals::new();
        if *mode != WindowMode::CustomPolicy || self.resized.is_some() {
            visuals = visuals.client_size(width, height);
        }
        if self.icon {
            visuals = visuals.icon(concat!(env!("CARGO_MANIFEST_DIR"), "\\icon.ico"));
        }
        context.window_visuals(visuals);
        let content = StackPanel::new().margin(20.0).spacing(12.0).children((
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
        ));
        if *mode != WindowMode::Native {
            context.window_frame(title, content)
        } else {
            context.window_title(title);
            content.into()
        }
    }
}
