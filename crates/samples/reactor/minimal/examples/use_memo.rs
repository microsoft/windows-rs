//! Minimal sample for the `cx.use_memo` hook.
//!
//! Memoizes `factorial(n)` so toggling unrelated state re-renders
//! without recomputing. A recompute counter makes the caching visible.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (n, set_n) = cx.use_state(3_i32);
    let (show_hint, set_hint) = cx.use_state(false);

    let recomputes = cx.use_ref(0_u32);
    let recomputes_for_memo = recomputes.clone();

    let factorial = cx.use_memo((n,), move || {
        *recomputes_for_memo.borrow_mut() += 1;
        (1..=n as i64).product::<i64>()
    });

    let dec = {
        let s = set_n.clone();
        move || s.call((n - 1).max(0))
    };
    let inc = {
        let s = set_n;
        // 20! fits in i64; 21! overflows.
        move || s.call((n + 1).min(20))
    };
    let toggle_hint = move || set_hint.call(!show_hint);

    vstack((
        text_block(format!("n = {n},  factorial(n) = {factorial}")).font_size(18.0),
        text_block(format!(
            "memo factory ran {} time(s) so far",
            *recomputes.borrow()
        ))
        .font_size(12.0)
        .opacity(0.7),
        hstack((
            button("-").on_click(dec),
            button("+").on_click(inc),
            // Rerenders without changing memo deps.
            button("toggle unrelated state").on_click(toggle_hint),
        ))
        .spacing(8.0),
        if show_hint {
            text_block(
                "Toggling this state rerenders, \
                     but the memo was skipped — same deps.",
            )
            .opacity(0.7)
            .into()
        } else {
            group(vec![])
        },
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    App::new().title("use_memo").render(app)
}
