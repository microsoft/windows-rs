//! Sample for the `cx.use_ref` hook.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (clicks, set_clicks) = cx.use_state(0_u32);

    let render_count = cx.use_ref(0_u64);
    *render_count.borrow_mut() += 1;
    let renders = *render_count.borrow();

    let bump = move || set_clicks.call(clicks + 1);

    vstack((
        text_block(format!("clicks (use_state) = {clicks}")).font_size(18.0),
        text_block(format!("renders (use_ref)  = {renders}")).font_size(18.0),
        button("Click me").on_click(bump),
        text_block(
            "The ref counter increments every render; the state counter \
                 only on button click. Ref mutation never schedules a rerender.",
        )
        .font_size(12.0)
        .opacity(0.7),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("UseRef", app)
}
