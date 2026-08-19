#![windows_subsystem = "windows"]

use windows_reactor::{Button, Element, FontWeight, RenderCx, TextBlock, hstack, vstack};

#[derive(Clone, Default)]
struct CounterState {
    count: i32,
}

enum Action {
    Increment,
    Decrement,
    Reset,
}

fn reducer(mut state: CounterState, action: Action) -> CounterState {
    match action {
        Action::Increment => state.count += 1,
        Action::Decrement => state.count -= 1,
        Action::Reset => state = CounterState::default(),
    }
    state
}

fn app(cx: &mut RenderCx<'_>) -> Element {
    let (state, dispatch) = cx.use_reducer(CounterState::default, reducer);
    let increment = dispatch.clone();
    let decrement = dispatch.clone();

    vstack(
        8.0,
        [
            TextBlock::new(format!("count = {}", state.count))
                .font_size(24.0)
                .font_weight(FontWeight::BOLD)
                .build(),
            hstack(
                8.0,
                [
                    Button::new("-")
                        .on_click(move || decrement.call(Action::Decrement))
                        .build(),
                    Button::new("+")
                        .on_click(move || increment.call(Action::Increment))
                        .build(),
                    Button::new("Reset")
                        .on_click(move || dispatch.call(Action::Reset))
                        .build(),
                ],
            ),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("UseReducer", app)
}
