//! Sample for the `cx.use_callback` hook.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (rerenders, set_rerenders) = cx.use_state(0_u32);

    let fires = cx.use_ref(0_u32);
    let fires_for_cb = fires.clone();

    let on_fire: Callback<()> = cx.use_callback((), move |()| {
        *fires_for_cb.borrow_mut() += 1;
    });

    let fire_a = {
        let cb = on_fire.clone();
        move || cb.invoke(())
    };
    let fire_b = {
        let cb = on_fire;
        move || cb.invoke(())
    };
    let rerender = move || set_rerenders.call(rerenders + 1);

    vstack((
        text_block(format!("callback fired {} time(s)", *fires.borrow())).font_size(18.0),
        text_block(format!("forced rerenders = {rerenders}"))
            .font_size(12.0)
            .opacity(0.7),
        hstack((
            button("Fire (A)").on_click(fire_a),
            button("Fire (B)").on_click(fire_b),
            // Rerenders without changing callback deps.
            button("Force rerender").on_click(rerender),
        ))
        .spacing(8.0),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("UseCallback", app)
}
