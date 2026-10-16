#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Clone, Copy)]
enum Message {
    Value(usize, Option<f64>),
    Step,
    Options,
}

struct NumberBoxSample {
    values: [Option<f64>; 3],
    step: f64,
    custom: bool,
}

impl Component for NumberBoxSample {
    type Input = ();
    type Message = Message;

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            values: [Some(10.0); 3],
            step: 0.25,
            custom: true,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Value(index, value) => self.values[index] = value,
            Message::Step => self.step = if self.step == 0.25 { 2.0 } else { 0.25 },
            Message::Options => self.custom = !self.custom,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("NumberBox options");
        context.window_visuals(WindowVisuals::new().client_size(440.0, 420.0));
        let inputs = [
            ("Hidden", NumberBoxSpinButtonPlacementMode::Hidden),
            ("Inline", NumberBoxSpinButtonPlacementMode::Inline),
            ("Compact", NumberBoxSpinButtonPlacementMode::Compact),
        ]
        .into_iter()
        .enumerate()
        .map(|(index, (label, mode))| {
            let mut input = NumberBox::new()
                .header(label)
                .automation_name(label)
                .minimum(0.0)
                .maximum(20.0)
                .value(self.values[index])
                .on_value_changed(context.callback(move |value| Message::Value(index, value)));
            if self.custom {
                input = input
                    .small_change(self.step)
                    .spin_button_placement_mode(mode);
            }
            input.into()
        })
        .collect::<Vec<View>>();
        Border::new()
            .padding(Thickness::uniform(16.0))
            .content(
                StackPanel::new().spacing(12.0).children((
                    "Focus a number and use Up/Down, or use its spinner buttons.",
                    StackPanel::new().spacing(8.0).children(inputs),
                    Button::new()
                        .content(format!("Custom step: {}", self.step))
                        .on_click(context.message(Message::Step)),
                    Button::new()
                        .content(if self.custom {
                            "Use native defaults"
                        } else {
                            "Use custom options"
                        })
                        .on_click(context.message(Message::Options)),
                )),
            )
            .into()
    }
}

fn main() {
    App::run_component::<NumberBoxSample>(()).unwrap();
}
