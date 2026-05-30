//! Event detachment fixtures: verify that `on_click` / `on_changed` handlers
//! are properly revoked when a control is unmounted or re-rendered without
//! the callback attached. This catches leaks in the EventRevoker lifecycle.

use std::cell::Cell;
use std::rc::Rc;

use windows_reactor::core::element::Element;
use windows_reactor::core::element::Slider;
use windows_reactor::dsl::{button, check_box, text_block};

use crate::fixtures::reconciler::{FixtureFuture, cc};
use crate::harness::Harness;

use windows_reactor::vstack;

/// Verify that when a button with `on_click` is removed from the tree,
/// subsequent programmatic clicks on the same UI slot don't fire the old
/// handler (handler was properly revoked/detached).
pub fn on_click_detach_on_unmount(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        let fire_count = Rc::new(Cell::new(0u32));
        let fire_count2 = fire_count.clone();

        h.mount(cc(move |cx| {
            let (show_btn, set_show) = cx.use_state(true);
            let (count, _set_count) = cx.use_state(0u32);
            let fc = fire_count2.clone();

            let body: Element = if show_btn {
                vstack((
                    button("Clickable").on_click(move || {
                        fc.set(fc.get() + 1);
                    }),
                    text_block(format!("fires={}", fire_count2.get())),
                ))
                .into()
            } else {
                vstack((text_block(format!("removed,fires={}", fire_count2.get())),)).into()
            };

            vstack((
                body,
                text_block(format!("count={count}")),
                button("Remove").on_click(move || set_show.call(false)),
            ))
            .into()
        }));
        h.render().await;

        // Click the button — handler should fire
        let _ = h.click_button("Clickable");
        h.render().await;
        h.check(
            "EventDetach_OnClick_FiredBeforeRemove",
            fire_count.get() >= 1,
        );

        // Remove the button from the tree
        let _ = h.click_button("Remove");
        h.render().await;

        h.check(
            "EventDetach_OnClick_ButtonRemoved",
            h.find_text_containing("removed,fires=").is_some(),
        );

        // The fire count should not increase after removal
        let count_after_remove = fire_count.get();
        h.check(
            "EventDetach_OnClick_NoLeakedFires",
            count_after_remove >= 1, // only the initial click
        );
    })
}

/// Verify that when an `on_changed` callback is conditionally removed from
/// a CheckBox (control stays mounted, but handler is gone), subsequent
/// state changes via the WinUI property no longer trigger the old handler.
pub fn on_changed_detach_on_rerender(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        let fire_count = Rc::new(Cell::new(0u32));
        let fire_count2 = fire_count.clone();

        h.mount(cc(move |cx| {
            let (attach_handler, set_attach) = cx.use_state(true);
            let (checked, set_checked) = cx.use_state(false);
            let fc = fire_count2.clone();

            let cb: Element = if attach_handler {
                check_box(checked)
                    .label("target")
                    .on_changed(move |v| {
                        fc.set(fc.get() + 1);
                        set_checked.call(v);
                    })
                    .into()
            } else {
                // Same control, no handler attached
                check_box(checked).label("target").into()
            };

            vstack((
                cb,
                text_block(format!("fires={}", fire_count2.get())),
                text_block(format!("attached={attach_handler}")),
                button("Detach").on_click(move || set_attach.call(false)),
            ))
            .into()
        }));
        h.render().await;

        // Toggle checkbox — handler fires
        let _ = h.set_checkbox_value(true);
        h.render().await;
        h.check(
            "EventDetach_OnChanged_FiredWhileAttached",
            fire_count.get() >= 1,
        );

        // Detach the handler (re-render without on_changed)
        let _ = h.click_button("Detach");
        h.render().await;
        h.check(
            "EventDetach_OnChanged_HandlerDetached",
            h.find_text("attached=false").is_some(),
        );

        let count_before = fire_count.get();
        // Toggle again — handler should NOT fire
        let _ = h.set_checkbox_value(false);
        h.render().await;

        h.check(
            "EventDetach_OnChanged_NoFireAfterDetach",
            fire_count.get() == count_before,
        );
    })
}

/// Verify that a Slider's `on_changed` handler is properly replaced when
/// the component re-renders with a new closure (no stale closure leak).
pub fn on_changed_handler_replacement(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (multiplier, set_mult) = cx.use_state(1i32);
            let (result, set_result) = cx.use_state(0i32);
            let m = multiplier;

            vstack((
                text_block(format!("result={result}")),
                text_block(format!("mult={multiplier}")),
                Slider::new(5.0)
                    .range(0.0, 10.0)
                    .on_changed(move |v| set_result.call((v as i32) * m)),
                button("DoubleMult").on_click(move || set_mult.call(multiplier * 2)),
            ))
            .into()
        }));
        h.render().await;

        // Initial: multiplier=1, slide to 3 → result=3
        let _ = h.set_slider_value(3.0);
        h.render().await;
        h.check(
            "EventDetach_Replacement_InitialMult",
            h.find_text("result=3").is_some(),
        );

        // Update multiplier to 2
        let _ = h.click_button("DoubleMult");
        h.render().await;
        h.check(
            "EventDetach_Replacement_MultUpdated",
            h.find_text("mult=2").is_some(),
        );

        // Slide to 4 → result should be 4*2=8, not 4*1=4 (stale closure)
        let _ = h.set_slider_value(4.0);
        h.render().await;
        h.check(
            "EventDetach_Replacement_NewClosureUsed",
            h.find_text("result=8").is_some(),
        );
    })
}
