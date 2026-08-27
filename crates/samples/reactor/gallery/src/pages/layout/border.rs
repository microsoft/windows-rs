use crate::controls::*;
use windows_reactor::*;

pub struct BorderPage {
    radius: f64,
    thick: bool,
}

#[derive(Clone)]
pub enum Message {
    Radius(f64),
    Thick(bool),
}

impl Component for BorderPage {
    type Message = Message;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            radius: 8.0,
            thick: false,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Radius(value) => self.radius = value,
            Message::Thick(value) => self.thick = value,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "Border",
            "A container that draws a border around its child element.",
            [
                KeyedView::new(
                    "dynamic",
                    sample_card(
                        "Dynamic Border",
                        StackPanel::new().spacing(12.0).children((
                            Border::new()
                                .border_brush(Color::rgb(60, 100, 180))
                                .border_thickness(if self.thick { 4.0 } else { 1.0 })
                                .padding(16.0)
                                .corner_radius(self.radius)
                                .content("Adjustable border"),
                            Slider::new()
                                .minimum(0.0)
                                .maximum(32.0)
                                .value(self.radius)
                                .on_value_changed(context.callback(Message::Radius)),
                            ToggleSwitch::new()
                                .is_on(self.thick)
                                .on_toggled(context.callback(Message::Thick))
                                .slots([SlotView::new(ToggleSwitchSlot::Header, "Thick border")]),
                        )),
                        "Border::new().corner_radius(radius).border_thickness(thickness)",
                    ),
                ),
                KeyedView::new(
                    "colored",
                    sample_card(
                        "Colored Border",
                        Border::new()
                            .background(Color::rgb(60, 100, 180))
                            .padding(16.0)
                            .corner_radius(4.0)
                            .content("Styled border"),
                        "Border::new().background(color).corner_radius(4.0)",
                    ),
                ),
            ],
        )
    }
}
