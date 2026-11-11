#![windows_subsystem = "windows"]

mod bindings {
    #![allow(
        non_snake_case,
        non_camel_case_types,
        non_upper_case_globals,
        clippy::all
    )]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod load;

use load::Load;
use windows_reactor::*;

struct MaxCpu {
    load: Load,
    always_on_top: bool,
}

#[derive(Clone, Copy)]
enum Message {
    Processors(usize),
    AlwaysOnTop(bool),
}

impl Component for MaxCpu {
    type Input = usize;
    type Message = Message;

    fn create(processors: &usize, _: &ComponentContext<Self>) -> Self {
        Self {
            load: Load::new(*processors),
            always_on_top: true,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Processors(count) => self.load.set(count),
            Message::AlwaysOnTop(value) => self.always_on_top = value,
        }
    }

    fn view(&self, processors: &usize, context: &mut ViewContext<Self>) -> View {
        context.window_visuals(
            WindowVisuals::new()
                .client_size(440.0, 240.0)
                .backdrop(WindowBackdrop::Acrylic)
                .always_on_top(self.always_on_top)
                .minimizable(false)
                .constraints(WindowConstraints {
                    min_width: Some(380.0),
                    min_height: Some(240.0),
                    ..Default::default()
                }),
        );
        let content =
            StackPanel::new().margin(24.0).spacing(16.0).children((
                TextBlock::new()
                    .automation_id("status")
                    .font_size(20.0)
                    .font_weight(FontWeight::SEMI_BOLD)
                    .text(format!(
                        "Stressing processors: {} of {processors}",
                        self.load.active()
                    )),
                Slider::new()
                    .automation_id("processors")
                    .automation_name("Processors to stress")
                    .minimum(0.0)
                    .maximum(*processors as f64)
                    .step_frequency(1.0)
                    .value(self.load.active() as f64)
                    .on_value_changed(
                        context.callback(|value: f64| Message::Processors(value.round() as usize)),
                    ),
                CheckBox::new()
                    .automation_id("always-on-top")
                    .content("Always on top")
                    .is_checked(self.always_on_top)
                    .on_is_checked_changed(context.callback(|value: Option<bool>| {
                        Message::AlwaysOnTop(value.unwrap_or(false))
                    })),
            ));
        context.window_frame("Max CPU", content)
    }
}

fn main() {
    let processors = std::thread::available_parallelism().unwrap().get();
    App::run_component::<MaxCpu>(processors).unwrap();
}
