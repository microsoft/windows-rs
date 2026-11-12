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
mod memory;

use load::Load;
use memory::Memory;
use windows_reactor::*;

struct MaxCpu {
    load: Load,
    memory: Memory,
    memory_target: usize,
    memory_allocated: usize,
    memory_error: Option<String>,
    always_on_top: bool,
}

enum Message {
    Processors(usize),
    Memory(usize),
    MemoryStatus(usize, Option<String>),
    AlwaysOnTop(bool),
}

impl Component for MaxCpu {
    type Input = usize;
    type Message = Message;

    fn create(processors: &usize, context: &ComponentContext<Self>) -> Self {
        let completion = context.completion();
        Self {
            load: Load::new(*processors),
            memory: Memory::new(move |blocks, error| {
                _ = completion
                    .clone()
                    .complete(Message::MemoryStatus(blocks, error));
            }),
            memory_target: 0,
            memory_allocated: 0,
            memory_error: None,
            always_on_top: true,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Processors(count) => self.load.set(count),
            Message::Memory(blocks) => {
                self.memory_target = blocks;
                self.memory_error = None;
                self.memory.set(blocks);
            }
            Message::MemoryStatus(blocks, error) => {
                self.memory_allocated = blocks;
                self.memory_error = error;
            }
            Message::AlwaysOnTop(value) => self.always_on_top = value,
        }
    }

    fn view(&self, processors: &usize, context: &mut ViewContext<Self>) -> View {
        let memory_detail = self
            .memory_error
            .as_deref()
            .unwrap_or("0-16 GB in 256 MB steps. Lower to release memory.");
        context.window_visuals(
            WindowVisuals::new()
                .client_size(440.0, 340.0)
                .backdrop(WindowBackdrop::Acrylic)
                .always_on_top(self.always_on_top)
                .minimizable(false)
                .constraints(WindowConstraints {
                    min_width: Some(440.0),
                    min_height: Some(340.0),
                    max_height: Some(340.0),
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
                TextBlock::new()
                    .automation_id("memory-status")
                    .font_size(20.0)
                    .font_weight(FontWeight::SEMI_BOLD)
                    .text(format!(
                        "Memory: {} / {} MB",
                        self.memory_allocated * memory::BLOCK_MB,
                        self.memory_target * memory::BLOCK_MB,
                    )),
                Slider::new()
                    .automation_id("memory")
                    .automation_name("Memory to allocate in MB")
                    .minimum(0.0)
                    .maximum((memory::MAX_BLOCKS * memory::BLOCK_MB) as f64)
                    .step_frequency(memory::BLOCK_MB as f64)
                    .value((self.memory_target * memory::BLOCK_MB) as f64)
                    .on_value_changed(context.callback(|value: f64| {
                        Message::Memory((value / memory::BLOCK_MB as f64).round() as usize)
                    })),
                TextBlock::new()
                    .automation_id("memory-detail")
                    .text_wrapping(TextWrapping::Wrap)
                    .max_lines(2)
                    .text_trimming(TextTrimming::CharacterEllipsis)
                    .text(memory_detail)
                    .tooltip(memory_detail),
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
