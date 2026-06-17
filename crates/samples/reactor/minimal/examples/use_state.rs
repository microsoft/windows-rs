//! Sample for the `cx.use_state` hook.

use windows_reactor::*;

fn main() -> Result<()> {
    reactor_minimal::run("UseState", app)
}

fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0);
    let click = move || set_count.call(count + 1);

    vstack((
        button("Click").on_click(click),
        text_block(format!("count = {count}"))
            .font_size(18.0)
            .bold(),
    ))
    .into()
}
