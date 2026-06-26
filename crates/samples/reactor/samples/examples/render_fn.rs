//! Sample for `App::render` — just a render function.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0_i32);

    vstack((
        text_block(format!("Count: {count}")).font_size(24.0).bold(),
        hstack((
            button("-").on_click({
                let set_count = set_count.clone();
                move || set_count.call(count - 1)
            }),
            button("+").on_click(move || set_count.call(count + 1)),
        ))
        .spacing(8.0),
    ))
    .spacing(12.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("RenderFn", app)
}
