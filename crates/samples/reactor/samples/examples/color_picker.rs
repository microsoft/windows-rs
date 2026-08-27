use windows_reactor::*;

struct ColorPickerSample {
    color: Color,
}

impl Component for ColorPickerSample {
    type Message = Color;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            color: Color::argb(255, 0, 120, 215),
        }
    }

    fn update(&mut self, color: Color, _context: &ComponentContext<Self>) {
        self.color = color;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("ColorPicker");
        let Color { a, r, g, b } = self.color;

        StackPanel::new().spacing(8.0).children((
            ColorPicker::new()
                .color(self.color)
                .is_alpha_enabled(true)
                .is_hex_input_visible(true)
                .is_color_slider_visible(true)
                .is_color_channel_text_input_visible(true)
                .on_color_changed(context.callback(std::convert::identity)),
            TextBlock::new().text(format!("ARGB: ({a}, {r}, {g}, {b})")),
            TextBlock::new().text(format!("Hex: #{r:02X}{g:02X}{b:02X}")),
        ))
    }
}

fn main() {
    App::run_component::<ColorPickerSample>(()).unwrap();
}
