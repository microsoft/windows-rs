use crate::controls::*;
use windows_reactor::*;

pub struct ColorPickerPage {
    color: Color,
}

impl Component for ColorPickerPage {
    type Message = Color;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            color: Color::rgb(50, 120, 200),
        }
    }

    fn update(&mut self, color: Color, _: &ComponentContext<Self>) {
        self.color = color;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "ColorPicker",
            "A control that lets a user pick a color.",
            [
                KeyedView::new(
                    "full",
                    sample_card(
                        "Full ColorPicker",
                        StackPanel::new().spacing(8.0).children((
                            ColorPicker::new()
                                .color(self.color)
                                .is_alpha_enabled(true)
                                .on_color_changed(context.callback(std::convert::identity)),
                            TextBlock::new()
                                .text(format!(
                                    "ARGB({}, {}, {}, {})",
                                    self.color.a, self.color.r, self.color.g, self.color.b
                                ))
                                .opacity(0.6),
                        )),
                        "ColorPicker::new().color(color).on_color_changed(handler)",
                    ),
                ),
                KeyedView::new(
                    "minimal",
                    sample_card(
                        "Minimal ColorPicker",
                        ColorPicker::new()
                            .color(Color::rgb(200, 50, 50))
                            .is_hex_input_visible(false)
                            .is_color_channel_text_input_visible(false),
                        "ColorPicker::new().is_hex_input_visible(false)",
                    ),
                ),
            ],
        )
    }
}
