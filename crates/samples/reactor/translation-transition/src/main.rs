use std::time::Duration;
use windows_reactor::*;

struct Sample {
    open: bool,
}

impl Component for Sample {
    type Input = ();
    type Message = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self { open: false }
    }

    fn update(&mut self, _: (), _: &ComponentContext<Self>) {
        self.open = !self.open;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Translation Transition");
        StackPanel::new()
            .spacing(16.0)
            .children((
                Button::new()
                    .content(if self.open { "Close" } else { "Open" })
                    .on_click(context.callback(|_| ())),
                Border::new()
                    .width(280.0)
                    .horizontal_alignment(HorizontalAlignment::Left)
                    .padding(Thickness::uniform(20.0))
                    .background(Color::rgb(70, 130, 200))
                    .translation(if self.open { 0.0 } else { -280.0 }, 0.0)
                    .translation_transition(Duration::from_millis(600))
                    .content(
                        TextBlock::new()
                            .text("Click again to reverse.")
                            .foreground(Color::rgb(0, 0, 0)),
                    ),
            ))
            .into()
    }
}

fn main() {
    App::run_component::<Sample>(()).unwrap();
}
