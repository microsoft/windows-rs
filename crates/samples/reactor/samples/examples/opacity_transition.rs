use std::time::Duration;

use windows_reactor::*;

struct OpacityTransition {
    visible: bool,
}

impl Component for OpacityTransition {
    type Message = ();
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self { visible: true }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
        self.visible = !self.visible;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Opacity Transition");
        let swatch = Border::new()
            .background(Color::rgb(70, 130, 200))
            .padding(Thickness::uniform(20.0))
            .opacity(if self.visible { 1.0 } else { 0.2 })
            .opacity_transition(Duration::from_secs(1))
            .max_width(280.0)
            .content(
                TextBlock::new()
                    .text("Animated content")
                    .font_size(18.0)
                    .foreground(Color::rgb(255, 255, 255)),
            );

        Border::new().padding(Thickness::uniform(16.0)).content(
            StackPanel::new().spacing(12.0).children((
                "Toggle to drive opacity through an implicit transition.",
                Button::new()
                    .on_click(context.callback(|_| ()))
                    .content(if self.visible { "Fade out" } else { "Fade in" }),
                swatch,
            )),
        )
    }
}

fn main() {
    App::run_component::<OpacityTransition>(()).unwrap();
}
