use super::*;
use std::time::Duration;

pub(super) struct ScaleFixture {
    scale: Option<[f64; 2]>,
    resized: bool,
    attached: bool,
    transition: bool,
}

#[derive(Clone)]
pub(super) enum Message {
    Scale(Option<[f64; 2]>),
    Resize,
    Attach,
    Transition,
}

impl Component for ScaleFixture {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            scale: Some([0.5, 0.5]),
            resized: false,
            attached: true,
            transition: false,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Scale(value) => self.scale = value,
            Message::Resize => self.resized = !self.resized,
            Message::Attach => self.attached = !self.attached,
            Message::Transition => self.transition = !self.transition,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Reactor UI fixture: scale");
        context.window_visuals(WindowVisuals::new().client_size(640.0, 480.0));
        let action = |id: &str, message| {
            Button::new()
                .automation_id(id)
                .content(id)
                .on_click(context.message(message))
        };
        let content: View = if self.attached {
            let mut marker = Border::new()
                .width(if self.resized { 240.0 } else { 160.0 })
                .height(if self.resized { 160.0 } else { 120.0 })
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center)
                .background(Color::rgb(255, 0, 0));
            if let Some(scale) = self.scale {
                marker = marker.scale(scale[0], scale[1]);
            }
            if self.transition {
                marker = marker.scale_transition(Duration::from_secs(3));
            }
            marker.into()
        } else {
            StackPanel::new().into()
        };
        StackPanel::new()
            .margin(20.0)
            .spacing(12.0)
            .children((
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .children((
                        action("half", Message::Scale(Some([0.5, 0.5]))),
                        action("vertical", Message::Scale(Some([1.0, 0.5]))),
                        action("horizontal", Message::Scale(Some([0.5, 1.0]))),
                        action("zero", Message::Scale(Some([1.0, 0.0]))),
                        action("zero-x", Message::Scale(Some([0.0, 1.0]))),
                        action("full", Message::Scale(Some([1.0, 1.0]))),
                    )),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .children((
                        action("clear", Message::Scale(None)),
                        action("resize", Message::Resize),
                        action("attach", Message::Attach),
                        action("transition", Message::Transition),
                    )),
                Border::new()
                    .automation_id("bounds")
                    .automation_name("Scale bounds")
                    .is_tab_stop(true)
                    .horizontal_alignment(HorizontalAlignment::Left)
                    .width(240.0)
                    .height(160.0)
                    .background(Color::rgb(255, 255, 255))
                    .content(content),
                TextBlock::new()
                    .automation_id("transition-state")
                    .text(self.transition.to_string()),
                TextBlock::new()
                    .automation_id("attached-state")
                    .text(self.attached.to_string()),
            ))
            .into()
    }
}
