#![windows_subsystem = "windows"]

use windows_reactor::*;

struct SliderSample {
    volume: f64,
}

impl Component for SliderSample {
    type Message = f64;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { volume: 35.0 }
    }

    fn update(&mut self, message: f64, _context: &ComponentContext<Self>) {
        self.volume = message;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("Slider");
        StackPanel::new().spacing(8.0).max_width(320.0).children((
            Slider::new()
                .minimum(0.0)
                .maximum(100.0)
                .value(self.volume)
                .step_frequency(1.0)
                .on_value_changed(context.callback(|value| value))
                .slots([SlotView::new(
                    SliderSlot::Header,
                    TextBlock::new().text("Volume"),
                )]),
            TextBlock::new().text(format!("Volume = {:.0}", self.volume)),
            Slider::new()
                .minimum(0.0)
                .maximum(100.0)
                .value(50.0)
                .orientation(Orientation::Vertical)
                .height(120.0)
                .slots([SlotView::new(
                    SliderSlot::Header,
                    TextBlock::new().text("Vertical"),
                )]),
            Slider::new()
                .minimum(0.0)
                .maximum(100.0)
                .value(50.0)
                .is_enabled(false)
                .slots([SlotView::new(
                    SliderSlot::Header,
                    TextBlock::new().text("Disabled"),
                )]),
        ))
    }
}

fn main() {
    App::run_component::<SliderSample>(()).unwrap();
}
