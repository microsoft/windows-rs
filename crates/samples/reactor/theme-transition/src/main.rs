#![windows_subsystem = "windows"]

use windows_reactor::*;

const CARD_WIDTH: f64 = 96.0;
const CARD_HEIGHT: f64 = 72.0;

struct ThemeTransitionSample {
    alternate: bool,
}

impl Component for ThemeTransitionSample {
    type Message = ();
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self { alternate: false }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
        self.alternate = !self.alternate;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Theme Transition");

        let positions = if self.alternate {
            [(250.0, 150.0), (50.0, 150.0), (250.0, 30.0), (50.0, 30.0)]
        } else {
            [(50.0, 30.0), (250.0, 30.0), (50.0, 150.0), (250.0, 150.0)]
        };
        let colors = [
            Color::rgb(210, 70, 70),
            Color::rgb(70, 130, 210),
            Color::rgb(70, 170, 110),
            Color::rgb(180, 110, 210),
        ];
        let cards =
            positions
                .into_iter()
                .zip(colors)
                .enumerate()
                .map(|(index, ((left, top), color))| {
                    KeyedView::new(
                        index,
                        Border::new()
                            .width(CARD_WIDTH)
                            .height(CARD_HEIGHT)
                            .horizontal_alignment(HorizontalAlignment::Left)
                            .vertical_alignment(VerticalAlignment::Top)
                            .margin(Thickness::new(left, top, 0.0, 0.0))
                            .transitions([ThemeTransition::Reposition])
                            .background(color)
                            .corner_radius(12.0)
                            .content(
                                TextBlock::new()
                                    .text(format!("Card {}", index + 1))
                                    .font_size(18.0)
                                    .horizontal_alignment(HorizontalAlignment::Center)
                                    .vertical_alignment(VerticalAlignment::Center),
                            ),
                    )
                });

        Border::new().padding(24.0).content(
            StackPanel::new().spacing(16.0).children((
                "The same native elements move to new Grid layout positions.",
                Button::new()
                    .on_click(context.forward())
                    .content("Move cards"),
                Border::new()
                    .background(Color::rgb(32, 64, 48))
                    .content(Grid::new().width(400.0).height(260.0).keyed_children(cards)),
            )),
        )
    }
}

fn main() {
    App::run_component::<ThemeTransitionSample>(()).unwrap();
}
