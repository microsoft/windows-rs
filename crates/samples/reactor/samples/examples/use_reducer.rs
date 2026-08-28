#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Default)]
struct CounterState {
    count: i32,
}

#[derive(Clone, Copy)]
enum Action {
    Decrement,
    Increment,
    Reset,
}

fn reducer(state: &mut CounterState, action: Action) {
    match action {
        Action::Decrement => state.count -= 1,
        Action::Increment => state.count += 1,
        Action::Reset => *state = CounterState::default(),
    }
}

struct UseReducerSample {
    state: CounterState,
}

impl Component for UseReducerSample {
    type Message = Action;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            state: CounterState::default(),
        }
    }

    fn update(&mut self, action: Action, _context: &ComponentContext<Self>) {
        reducer(&mut self.state, action);
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("UseReducer");
        StackPanel::new().spacing(8.0).children((
            TextBlock::new()
                .text(format!("count = {}", self.state.count))
                .font_size(24.0),
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(8.0)
                .children((
                    Button::new()
                        .on_click(context.message(Action::Decrement))
                        .content("-"),
                    Button::new()
                        .on_click(context.message(Action::Increment))
                        .content("+"),
                    Button::new()
                        .on_click(context.message(Action::Reset))
                        .content("reset"),
                )),
        ))
    }
}

fn main() {
    App::run_component::<UseReducerSample>(()).unwrap();
}
