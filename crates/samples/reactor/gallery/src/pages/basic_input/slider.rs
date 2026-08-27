use crate::controls::*;
use windows_reactor::*;

pub struct SliderPage {
    volume: f64,
    brightness: f64,
    temperature: f64,
}

#[derive(Clone)]
pub enum Message {
    Volume(f64),
    Brightness(f64),
    Temperature(f64),
}

impl Component for SliderPage {
    type Message = Message;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            volume: 35.0,
            brightness: 60.0,
            temperature: 21.0,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Volume(value) => self.volume = value,
            Message::Brightness(value) => self.brightness = value,
            Message::Temperature(value) => self.temperature = value,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "Slider",
            "Select a value from a range with touch-friendly input.",
            [
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Basic Slider",
                        StackPanel::new().spacing(8.0).children((
                            Slider::new()
                                .minimum(0.0)
                                .maximum(100.0)
                                .step_frequency(1.0)
                                .value(self.volume)
                                .on_value_changed(context.callback(Message::Volume)),
                            TextBlock::new()
                                .text(format!("Volume: {:.0}%", self.volume))
                                .opacity(0.6),
                        )),
                        "Slider::new().minimum(0.0).maximum(100.0).value(volume)",
                    ),
                ),
                KeyedView::new(
                    "vertical",
                    sample_card(
                        "Vertical Slider",
                        StackPanel::new()
                            .orientation(Orientation::Horizontal)
                            .spacing(16.0)
                            .children((
                                Slider::new()
                                    .minimum(0.0)
                                    .maximum(100.0)
                                    .step_frequency(5.0)
                                    .value(self.brightness)
                                    .orientation(Orientation::Vertical)
                                    .height(140.0)
                                    .on_value_changed(context.callback(Message::Brightness)),
                                TextBlock::new()
                                    .text(format!("Brightness: {:.0}%", self.brightness)),
                            )),
                        "Slider::new().orientation(Orientation::Vertical).height(140.0)",
                    ),
                ),
                KeyedView::new(
                    "temperature",
                    sample_card(
                        "Range with Value Label",
                        StackPanel::new().spacing(8.0).children((
                            Slider::new()
                                .minimum(16.0)
                                .maximum(30.0)
                                .step_frequency(0.5)
                                .value(self.temperature)
                                .on_value_changed(context.callback(Message::Temperature)),
                            TextBlock::new()
                                .text(format!("Target: {:.1} C", self.temperature))
                                .opacity(0.6),
                        )),
                        "Slider::new().minimum(16.0).maximum(30.0).step_frequency(0.5)",
                    ),
                ),
            ],
        )
    }
}
