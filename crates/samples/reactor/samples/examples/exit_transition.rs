#![windows_subsystem = "windows"]

use std::time::Duration;
use windows_reactor::*;

struct ExitTransitionSample {
    visible: bool,
}

impl Component for ExitTransitionSample {
    type Message = ();
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self { visible: true }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
        self.visible = !self.visible;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Exit Transition");
        let card = self.visible.then(|| {
            Border::new()
                .padding(Thickness::uniform(24.0))
                .background(Color::rgb(32, 96, 160))
                .corner_radius(CornerRadius::uniform(12.0))
                .exit_transition(ExitTransition::fade(Duration::from_millis(600)))
                .content(
                    TextBlock::new()
                        .text("This visual remains visible while its exit animation completes.")
                        .font_size(18.0),
                )
        });

        let content = StackPanel::new().spacing(16.0).children((
            Button::new()
                .on_click(context.message(()))
                .content(TextBlock::new().text(if self.visible { "Remove" } else { "Restore" })),
            card.unwrap_or_else(View::empty),
        ));
        Border::new()
            .padding(Thickness::uniform(24.0))
            .content(content)
    }
}

fn main() {
    App::run_component::<ExitTransitionSample>(()).unwrap();
}
