//! Minimal sample for the `cx.use_reducer_fn` hook.

use windows_reactor::*;

#[derive(Clone, PartialEq, Default)]
struct CounterState {
    count: i32,
}

enum Action {
    Increment,
    Decrement,
    Reset,
}

fn reducer(state: CounterState, action: Action) -> CounterState {
    match action {
        Action::Increment => CounterState {
            count: state.count + 1,
        },
        Action::Decrement => CounterState {
            count: state.count - 1,
        },
        Action::Reset => CounterState::default(),
    }
}

fn app(cx: &mut RenderCx) -> Element {
    let (state, dispatch) = cx.use_reducer_fn(reducer, CounterState::default());

    let inc = {
        let d = dispatch.clone();
        move || d.call(Action::Increment)
    };
    let dec = {
        let d = dispatch.clone();
        move || d.call(Action::Decrement)
    };
    let reset = move || dispatch.call(Action::Reset);

    vstack((
        text_block(format!("count = {}", state.count))
            .font_size(24.0)
            .bold(),
        hstack((
            button("-").on_click(dec),
            button("+").on_click(inc),
            button("reset").on_click(reset),
        ))
        .spacing(8.0),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    let _bootstrap_handle = windows_reactor::bootstrap::initialize()?;
    App::new().title("use_reducer").render(app)
}
