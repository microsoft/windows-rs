#![windows_subsystem = "windows"]

use windows_reactor::*;

struct WindowSample {
    size: WindowSize,
}

impl Component for WindowSample {
    type Message = WindowSize;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            size: WindowSize {
                width: 800.0,
                height: 600.0,
            },
        }
    }

    fn update(&mut self, size: WindowSize, _context: &ComponentContext<Self>) {
        self.size = size;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Sample");
        context.window_visuals(
            WindowVisuals::new()
                .backdrop(WindowBackdrop::Mica)
                .client_size(800.0, 600.0)
                .constraints(WindowConstraints {
                    min_width: Some(400.0),
                    min_height: Some(300.0),
                    max_width: Some(1200.0),
                    max_height: Some(900.0),
                }),
        );
        let on_window_size = context.callback(|size| size);
        context.on_window_size(on_window_size);

        Grid::new().children([
            arrow(
                "\u{1F881}",
                HorizontalAlignment::Center,
                VerticalAlignment::Top,
                Thickness::xy(0.0, -8.0),
            ),
            arrow(
                "\u{1F883}",
                HorizontalAlignment::Center,
                VerticalAlignment::Bottom,
                Thickness::xy(0.0, -5.0),
            ),
            arrow(
                "\u{1F880}",
                HorizontalAlignment::Left,
                VerticalAlignment::Center,
                Thickness::xy(-2.0, 0.0),
            ),
            arrow(
                "\u{1F882}",
                HorizontalAlignment::Right,
                VerticalAlignment::Center,
                Thickness::xy(-2.0, 0.0),
            ),
            TextBlock::new()
                .text(format!(
                    "({}, {})",
                    self.size.width as i32, self.size.height as i32
                ))
                .font_size(24.0)
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center)
                .into(),
        ])
    }
}

fn arrow(
    text: &'static str,
    horizontal: HorizontalAlignment,
    vertical: VerticalAlignment,
    margin: Thickness,
) -> View {
    TextBlock::new()
        .text(text)
        .font_size(24.0)
        .horizontal_alignment(horizontal)
        .vertical_alignment(vertical)
        .margin(margin)
        .into()
}

fn main() {
    App::run_component::<WindowSample>(()).unwrap();
}
