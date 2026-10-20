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

pub(super) struct PlacementFixture {
    changed: bool,
    observed: Option<WindowPlacement>,
}

#[derive(Clone, Copy)]
pub(super) enum PlacementMessage {
    ChangeInitial,
    Observed(WindowPlacement),
}

impl Component for PlacementFixture {
    type Input = ();
    type Message = PlacementMessage;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            changed: false,
            observed: None,
        }
    }

    fn update(&mut self, message: PlacementMessage, _: &ComponentContext<Self>) {
        match message {
            PlacementMessage::ChangeInitial => self.changed = true,
            PlacementMessage::Observed(placement) => self.observed = Some(placement),
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let placement = if self.changed {
            WindowPlacement {
                x: 240,
                y: 220,
                width: 760,
                height: 580,
                maximized: false,
            }
        } else {
            WindowPlacement {
                x: 80,
                y: 100,
                width: 640,
                height: 480,
                maximized: false,
            }
        };
        let observed = match self.observed {
            Some(value) => format!(
                "Placement {} {}; {} x {}; maximized {}",
                value.x, value.y, value.width, value.height, value.maximized
            ),
            None => "Waiting for placement".to_owned(),
        };
        context.window_title("Reactor UI fixture: window-placement");
        context.window_visuals(
            WindowVisuals::new()
                .client_size(320.0, 240.0)
                .initial_placement(placement),
        );
        context.on_window_placement(context.callback(PlacementMessage::Observed));
        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                TextBlock::new().automation_id("placement").text(observed),
                TextBlock::new()
                    .automation_id("state")
                    .text(format!("Changed initial placement: {}", self.changed)),
                Button::new()
                    .automation_id("change-initial")
                    .content("Change initial placement")
                    .on_click(context.message(PlacementMessage::ChangeInitial)),
            ))
            .into()
    }
}
