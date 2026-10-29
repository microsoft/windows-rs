#![windows_subsystem = "windows"]

use std::time::Duration;
use windows_reactor::*;

struct Transitions {
    toggled: bool,
}

fn swatch(text: &str) -> Border {
    Border::new()
        .width(200.0)
        .horizontal_alignment(HorizontalAlignment::Left)
        .padding(Thickness::uniform(12.0))
        .background(Color::rgb(70, 130, 200))
        .corner_radius(CornerRadius::uniform(8.0))
        .content(
            TextBlock::new()
                .text(text)
                .foreground(Color::rgb(255, 255, 255)),
        )
}

impl Component for Transitions {
    type Message = ();
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self { toggled: false }
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
        self.toggled = !self.toggled;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Transitions");
        let toggled = self.toggled;

        // ThemeTransition::Reposition animates the same native elements to new layout slots.
        let cards = [Color::rgb(210, 70, 70), Color::rgb(70, 170, 110)]
            .into_iter()
            .enumerate()
            .map(|(index, color)| {
                let left = if (index == 0) == toggled { 220.0 } else { 0.0 };
                keyed(
                    index,
                    Border::new()
                        .width(96.0)
                        .height(48.0)
                        .horizontal_alignment(HorizontalAlignment::Left)
                        .margin(Thickness::new(left, 0.0, 0.0, 0.0))
                        .transitions([ThemeTransition::Reposition])
                        .background(color)
                        .corner_radius(CornerRadius::uniform(8.0)),
                )
            });

        let mut children: Vec<View> = vec![
            Button::new()
                .on_click(context.forward())
                .content("Toggle")
                .into(),
            swatch("Opacity")
                .opacity(if toggled { 0.2 } else { 1.0 })
                .opacity_transition(Duration::from_secs(1))
                .into(),
            swatch("Scale")
                .scale(if toggled { 1.3 } else { 1.0 })
                .scale_transition(Duration::from_secs(1))
                .into(),
            swatch("Translation (click again to reverse)")
                .translation(if toggled { 120.0 } else { 0.0 }, 0.0)
                .translation_transition(Duration::from_millis(600))
                .into(),
            Grid::new()
                .width(320.0)
                .height(48.0)
                .horizontal_alignment(HorizontalAlignment::Left)
                .keyed_children(cards)
                .into(),
        ];

        // An exit transition keeps the removed element visible until its animation completes.
        if !toggled {
            children.push(
                swatch("Exit")
                    .exit_transition(ExitTransition::fade(Duration::from_millis(600)))
                    .into(),
            );
        }

        Border::new()
            .padding(Thickness::uniform(24.0))
            .content(StackPanel::new().spacing(16.0).children(children))
            .into()
    }
}

fn main() {
    App::run_component::<Transitions>(()).unwrap();
}
