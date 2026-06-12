//! Selftest fixtures for React-style hooks beyond `use_state`.
//! Covers: use_reducer, use_reducer_fn, use_memo, use_callback,
//! use_effect, use_ref, and use_context.

use std::cell::RefCell;
use std::rc::Rc;

use windows_reactor::Component;
use windows_reactor::Context;
use windows_reactor::Element;
use windows_reactor::RenderCx;
use windows_reactor::{ElementExt, button, text_block};
use windows_reactor::vstack;

use crate::fixtures::reconciler::{FixtureFuture, cc};
use crate::harness::Harness;

// ── use_reducer ─────────────────────────────────────────────────────────

pub fn use_reducer_increment(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (count, update) = cx.use_reducer(0i32);
            let inc = {
                let update = update.clone();
                move || update.call(Box::new(|n| n + 1))
            };
            let dec = {
                let update = update;
                move || update.call(Box::new(|n| n - 1))
            };
            vstack((
                text_block(format!("reducer={count}")),
                button("Inc").on_click(inc),
                button("Dec").on_click(dec),
            ))
            .into()
        }));
        h.render().await;
        h.check(
            "Hook_UseReducer_InitialZero",
            h.find_text("reducer=0").is_some(),
        );

        let _ = h.click_button("Inc");
        h.render().await;
        let _ = h.click_button("Inc");
        h.render().await;
        h.check(
            "Hook_UseReducer_AfterTwoIncrements",
            h.find_text("reducer=2").is_some(),
        );

        let _ = h.click_button("Dec");
        h.render().await;
        h.check(
            "Hook_UseReducer_AfterDecrement",
            h.find_text("reducer=1").is_some(),
        );
    })
}

// ── use_reducer_fn ──────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
enum CountAction {
    Add(i32),
    Reset,
}

pub fn use_reducer_fn_actions(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (count, dispatch) = cx.use_reducer_fn(
                |state: i32, action: CountAction| match action {
                    CountAction::Add(n) => state + n,
                    CountAction::Reset => 0,
                },
                10i32,
            );
            let add5 = {
                let d = dispatch.clone();
                move || d.call(CountAction::Add(5))
            };
            let reset = {
                let d = dispatch;
                move || d.call(CountAction::Reset)
            };
            vstack((
                text_block(format!("rfn={count}")),
                button("Add5").on_click(add5),
                button("Reset").on_click(reset),
            ))
            .into()
        }));
        h.render().await;
        h.check("Hook_UseReducerFn_Initial", h.find_text("rfn=10").is_some());

        let _ = h.click_button("Add5");
        h.render().await;
        h.check(
            "Hook_UseReducerFn_AfterAdd",
            h.find_text("rfn=15").is_some(),
        );

        let _ = h.click_button("Reset");
        h.render().await;
        h.check(
            "Hook_UseReducerFn_AfterReset",
            h.find_text("rfn=0").is_some(),
        );
    })
}

// ── use_memo ────────────────────────────────────────────────────────────

pub fn use_memo_caches(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        // use_memo should only recompute when deps change.
        // We track call count via an Rc<Cell>.
        let call_count: Rc<std::cell::Cell<u32>> = Rc::new(std::cell::Cell::new(0));
        let cc_clone = call_count.clone();

        h.mount(cc(move |cx| {
            let cc_inner = cc_clone.clone();
            let (n, set) = cx.use_state(5i32);
            let (unrelated, set_unrelated) = cx.use_state(0i32);

            let doubled = cx.use_memo(n, || {
                cc_inner.set(cc_inner.get() + 1);
                n * 2
            });
            let bump_unrelated = {
                let set_unrelated = set_unrelated;
                move || set_unrelated.call(unrelated + 1)
            };
            let change_n = move || set.call(n + 1);
            vstack((
                text_block(format!("memo={doubled}")),
                text_block(format!("unrelated={unrelated}")),
                button("ChangeN").on_click(change_n),
                button("BumpUnrelated").on_click(bump_unrelated),
            ))
            .into()
        }));
        h.render().await;
        h.check("Hook_UseMemo_Initial", h.find_text("memo=10").is_some());

        let count_after_initial = call_count.get();

        // Bump unrelated state — memo should NOT recompute
        let _ = h.click_button("BumpUnrelated");
        h.render().await;
        h.check(
            "Hook_UseMemo_NotRecomputedOnUnrelatedChange",
            call_count.get() == count_after_initial,
        );

        // Change n — memo SHOULD recompute
        let _ = h.click_button("ChangeN");
        h.render().await;
        h.check("Hook_UseMemo_Recomputed", h.find_text("memo=12").is_some());
        h.check(
            "Hook_UseMemo_CallCountIncreased",
            call_count.get() > count_after_initial,
        );
    })
}

// ── use_ref ─────────────────────────────────────────────────────────────

pub fn use_ref_persists(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let render_count = cx.use_ref(0u32);
            // Increment on every render
            *render_count.borrow_mut() += 1;
            let count = *render_count.borrow();

            let (trigger, set) = cx.use_state(0i32);
            let rerender = move || set.call(trigger + 1);
            vstack((
                text_block(format!("renders={count}")),
                button("Rerender").on_click(rerender),
            ))
            .into()
        }));
        h.render().await;
        h.check(
            "Hook_UseRef_InitialRender",
            h.find_text("renders=1").is_some(),
        );

        let _ = h.click_button("Rerender");
        h.render().await;
        h.check("Hook_UseRef_Persists", h.find_text("renders=2").is_some());

        let _ = h.click_button("Rerender");
        h.render().await;
        h.check(
            "Hook_UseRef_Accumulates",
            h.find_text("renders=3").is_some(),
        );
    })
}

// ── use_effect ──────────────────────────────────────────────────────────

pub fn use_effect_fires(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        let effect_log: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
        let log_clone = effect_log.clone();

        h.mount(cc(move |cx| {
            let log = log_clone.clone();
            let (n, set) = cx.use_state(0i32);

            cx.use_effect(n, {
                move || {
                    log.borrow_mut().push(format!("effect-{n}"));
                }
            });

            let bump = move || set.call(n + 1);
            vstack((text_block(format!("n={n}")), button("Bump").on_click(bump))).into()
        }));
        h.render().await;
        h.check(
            "Hook_UseEffect_FiredOnMount",
            effect_log.borrow().contains(&"effect-0".to_string()),
        );

        let _ = h.click_button("Bump");
        h.render().await;
        h.check(
            "Hook_UseEffect_FiredOnChange",
            effect_log.borrow().contains(&"effect-1".to_string()),
        );

        // Bump again without changing — just trigger a rerender via same button
        let _ = h.click_button("Bump");
        h.render().await;
        h.check(
            "Hook_UseEffect_FiredAgainOnNewDep",
            effect_log.borrow().contains(&"effect-2".to_string()),
        );
    })
}

// ── use_callback ────────────────────────────────────────────────────────

pub fn use_callback_stable(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (count, set) = cx.use_state(0i32);
            let (unrelated, set_unrelated) = cx.use_state(0i32);

            let cb = cx.use_callback(count, move |_: ()| {
                set.call(count + 1);
            });

            let fire = {
                let cb = cb;
                move || cb.invoke(())
            };
            let bump_unrelated = {
                let set_unrelated = set_unrelated;
                move || set_unrelated.call(unrelated + 1)
            };
            vstack((
                text_block(format!("cb-count={count}")),
                text_block(format!("cb-unrelated={unrelated}")),
                button("Fire").on_click(fire),
                button("Unrelated").on_click(bump_unrelated),
            ))
            .into()
        }));
        h.render().await;
        h.check(
            "Hook_UseCallback_Initial",
            h.find_text("cb-count=0").is_some(),
        );

        let _ = h.click_button("Fire");
        h.render().await;
        h.check(
            "Hook_UseCallback_AfterFire",
            h.find_text("cb-count=1").is_some(),
        );
    })
}

// ── use_context ─────────────────────────────────────────────────────────

static THEME_CTX: std::sync::LazyLock<Context<String>> =
    std::sync::LazyLock::new(|| Context::new("light".to_string()));

struct ContextConsumer;
impl Component for ContextConsumer {
    fn render(&self, _: &(), cx: &mut RenderCx) -> Element {
        let theme = cx.use_context(&THEME_CTX);
        text_block(format!("theme={theme}")).into()
    }
}

pub fn use_context_reads_default(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_cx| {
            // No provider — should get the default value "light"
            windows_reactor::component(ContextConsumer, ())
        }));
        h.render().await;
        h.check(
            "Hook_UseContext_ReadsDefault",
            h.find_text("theme=light").is_some(),
        );
    })
}

pub fn use_context_reads_provided(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_cx| {
            // Provide "dark" — consumer should see it
            windows_reactor::component(ContextConsumer, ())
                .provide(&THEME_CTX, "dark".to_string())
        }));
        h.render().await;
        h.check(
            "Hook_UseContext_ReadsProvided",
            h.find_text("theme=dark").is_some(),
        );
    })
}
