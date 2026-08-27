use crate::controls::*;
use windows_reactor::*;

pub struct CanvasPage {
    x: f64,
    y: f64,
}

#[derive(Clone)]
pub enum Message {
    X(f64),
    Y(f64),
}

impl Component for CanvasPage {
    type Message = Message;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self { x: 100.0, y: 80.0 }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::X(value) => self.x = value,
            Message::Y(value) => self.y = value,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "Canvas",
            "Absolute positioning of child elements.",
            [KeyedView::new(
                "position",
                sample_card(
                    "Adjustable Position",
                    StackPanel::new().spacing(8.0).children((
                        Canvas::new().width(320.0).height(200.0).children((
                            TextBlock::new()
                                .text("Fixed")
                                .canvas_left(10.0)
                                .canvas_top(10.0),
                            Border::new()
                                .background(ThemeBrush::CardBackground)
                                .padding(8.0)
                                .corner_radius(4.0)
                                .canvas_left(self.x)
                                .canvas_top(self.y)
                                .content("Move me"),
                        )),
                        Slider::new()
                            .minimum(0.0)
                            .maximum(250.0)
                            .value(self.x)
                            .on_value_changed(context.callback(Message::X)),
                        Slider::new()
                            .minimum(0.0)
                            .maximum(160.0)
                            .value(self.y)
                            .on_value_changed(context.callback(Message::Y)),
                        TextBlock::new()
                            .text(format!("Position: {:.0}, {:.0}", self.x, self.y))
                            .opacity(0.6),
                    )),
                    "Canvas::new().children((child.canvas_left(x).canvas_top(y),))",
                ),
            )],
        )
    }
}
