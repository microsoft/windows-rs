use std::time::Duration;

use windows_reactor::*;

struct ScaleTransition {
    big: bool,
}

impl Component for ScaleTransition {
    type Message = ();
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self { big: false }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
        self.big = !self.big;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Scale Transition");
        let swatch = Border::new()
            .background(Color::rgb(70, 130, 200))
            .padding(Thickness::uniform(20.0))
            .scale(if self.big { 1.3 } else { 1.0 })
            .scale_transition(Duration::from_secs(1))
            .max_width(280.0)
            .content(
                TextBlock::new()
                    .text("Animated content")
                    .font_size(18.0)
                    .foreground(Color::rgb(255, 255, 255)),
            );

        Border::new().padding(Thickness::uniform(16.0)).content(
            StackPanel::new().spacing(12.0).children((
                TextBlock::new().text("Toggle to drive scale through an implicit transition."),
                Button::new().on_click(context.callback(|_| ())).content(
                    TextBlock::new().text(if self.big { "Scale down" } else { "Scale up" }),
                ),
                swatch,
            )),
        )
    }
}

fn main() {
    App::run_component::<ScaleTransition>(()).unwrap();
}
