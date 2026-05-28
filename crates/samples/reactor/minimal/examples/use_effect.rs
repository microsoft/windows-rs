//! Minimal sample for the `cx.use_effect` hook.
//!
//! The effect copies `count` into a `use_ref` cell whenever `count`
//! changes. Toggling an unrelated flag forces a rerender without
//! changing the effect's deps, so the "last observed" value stays put.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> impl Into<Element> {
    let (count, set_count) = cx.use_state(0_i32);
    let (flag, set_flag) = cx.use_state(false);

    let last_seen = cx.use_ref(0_i32);
    let last_seen_for_effect = last_seen.clone();

    cx.use_effect((count,), move || {
        *last_seen_for_effect.borrow_mut() = count;
    });

    let dec = {
        let s = set_count.clone();
        move || s.call(count - 1)
    };
    let inc = move || set_count.call(count + 1);
    let toggle = move || set_flag.call(!flag);

    vstack((
        text_block(format!("count = {count}"))
            .font_size(24.0)
            .bold(),
        text_block(format!("use_effect last observed: {}", *last_seen.borrow())),
        hstack((
            button("-").on_click(dec),
            button("+").on_click(inc),
            // Rerenders without changing the effect's deps.
            button("toggle unrelated state").on_click(toggle),
        ))
        .spacing(8.0),
        text_block(format!("unrelated flag = {flag}"))
            .font_size(12.0)
            .opacity(0.7),
    ))
    .spacing(8.0)
}

fn main() -> Result<()> {
    App::new().title("use_effect").render(app)
}
