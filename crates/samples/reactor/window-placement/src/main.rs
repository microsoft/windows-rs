#![windows_subsystem = "windows"]

use windows_reactor::*;

struct PlacementSample {
    placement: Option<WindowPlacement>,
}

#[derive(Clone, Copy)]
enum Message {
    Placement(WindowPlacement),
    OpenCopy,
}

impl Component for PlacementSample {
    type Input = Option<WindowPlacement>;
    type Message = Message;

    fn create(_: &Self::Input, _: &ComponentContext<Self>) -> Self {
        Self { placement: None }
    }

    fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
        match message {
            Message::Placement(placement) => self.placement = Some(placement),
            Message::OpenCopy => {
                assert!(context.open_window::<Self>(self.placement));
            }
        }
    }

    fn view(&self, saved: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let mut visuals = WindowVisuals::new()
            .client_size(640.0, 360.0)
            .initial_position(ScreenPoint::new(100, 100));
        if let Some(saved) = saved {
            visuals = visuals.initial_placement(*saved);
        }
        context.window_visuals(visuals);
        context.on_window_placement(context.callback(Message::Placement));

        let status = self.placement.map_or_else(
            || "Waiting for placement".to_string(),
            |placement| {
                format!(
                    "Restored bounds: ({}, {}) {} x {} physical pixels\nMaximized: {}",
                    placement.x,
                    placement.y,
                    placement.width,
                    placement.height,
                    placement.maximized,
                )
            },
        );
        context.window_frame(
            "Window placement",
            StackPanel::new().margin(24.0).spacing(12.0).children((
                "Move, resize, or maximize this window.",
                "Open a copy at its current placement, without a startup jump.",
                TextBlock::new().text(status),
                Button::new()
                    .is_enabled(self.placement.is_some())
                    .on_click(context.message(Message::OpenCopy))
                    .content("Open a restored copy"),
            )),
        )
    }
}

fn main() {
    App::run_component::<PlacementSample>(None).unwrap();
}
