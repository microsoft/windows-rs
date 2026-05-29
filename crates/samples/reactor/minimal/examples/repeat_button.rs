//! Minimal sample for the `RepeatButton` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0_i32);

    let set_inc = set_count.clone();
    let increment = move || set_inc.call(count + 1);
    let decrement = move || set_count.call(count - 1);

    vstack((
        text_block(format!("Count: {count}")),
        repeat_button("+1 (hold to repeat)")
            .on_click(increment)
            .delay(300)
            .interval(50),
        repeat_button("-1 (hold to repeat)")
            .on_click(decrement)
            .delay(300)
            .interval(50),
        repeat_button("Disabled").enabled(false),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
