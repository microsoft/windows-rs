use windows_canvas::*;
use windows_reactor::*;

pub fn run(title: &'static str, draw: fn(&DrawContext) -> Result<()>) -> Result<()> {
    App::run_component::<Sample>((title, draw, false))
}

pub fn run_animated(title: &'static str, draw: fn(&DrawContext) -> Result<()>) -> Result<()> {
    App::run_component::<Sample>((title, draw, true))
}

type Input = (&'static str, fn(&DrawContext) -> Result<()>, bool);

struct Sample;

impl Component for Sample {
    type Input = Input;
    type Message = ();

    fn create(_input: &Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Input, context: &mut ViewContext<Self>) -> View {
        context.window_title(input.0);
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));
        if input.2 {
            animated_canvas(input.1)
        } else {
            canvas(input.1)
        }
    }
}
