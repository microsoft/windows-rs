#![windows_subsystem = "windows"]

use windows_reactor::*;

struct ColorSchemeSample {
    scheme: ColorScheme,
}

impl Component for ColorSchemeSample {
    type Message = ColorScheme;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            scheme: ColorScheme::Light,
        }
    }

    fn update(&mut self, scheme: ColorScheme, _context: &ComponentContext<Self>) {
        self.scheme = scheme;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Color scheme");
        let on_color_scheme = context.callback(|scheme| scheme);
        context.on_color_scheme(on_color_scheme);
        let is_dark = self.scheme == ColorScheme::Dark;
        let label = if is_dark { "Dark" } else { "Light" };

        let content = StackPanel::new().spacing(8.0).children((
            TextBlock::new()
                .text(format!("Dark theme: {is_dark}"))
                .font_size(20.0)
                .font_weight(FontWeight::BOLD),
            TextBlock::new()
                .text(format!("Current color scheme: {label}"))
                .font_size(16.0),
            TextBlock::new()
                .text(if is_dark {
                    "Dark color branch"
                } else {
                    "Light color branch"
                })
                .font_size(14.0)
                .foreground(ThemeBrush::PrimaryText),
        ));
        Grid::new().children((content,))
    }
}

fn main() {
    App::run_component::<ColorSchemeSample>(()).unwrap();
}
