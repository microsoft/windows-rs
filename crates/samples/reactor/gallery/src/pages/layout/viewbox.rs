use crate::controls::*;
use windows_reactor::*;

pub struct ViewboxPage {
    size: f64,
}

impl Component for ViewboxPage {
    type Message = f64;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self { size: 120.0 }
    }

    fn update(&mut self, size: f64, _: &ComponentContext<Self>) {
        self.size = size;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "Viewbox",
            "Scales a child to fit the available space.",
            [KeyedView::new(
                "resizable",
                sample_card(
                    "Resizable Content",
                    StackPanel::new().spacing(8.0).children((
                        Viewbox::new()
                            .width(self.size)
                            .height(100.0)
                            .stretch(Stretch::Uniform)
                            .slot(
                                ViewboxSlot::Child,
                                Border::new()
                                    .width(200.0)
                                    .height(100.0)
                                    .background(Color::rgb(0, 120, 212))
                                    .corner_radius(8.0)
                                    .content(TextBlock::new().text("200 x 100").font_size(24.0)),
                            ),
                        Slider::new()
                            .minimum(60.0)
                            .maximum(220.0)
                            .value(self.size)
                            .on_value_changed(context.callback(std::convert::identity)),
                    )),
                    r#"Viewbox::new()
    .width(viewport_width)
    .height(100.0)
    .stretch(Stretch::Uniform)
    .slot(ViewboxSlot::Child, content)"#,
                ),
            )],
        )
    }
}
