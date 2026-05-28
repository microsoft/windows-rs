//! Control update fixtures: verify that changing a prop on an already-mounted
//! control correctly diffs and applies to the WinUI backend without
//! re-creating the underlying XAML control (identity preservation).

use windows_core::Interface;

use windows_reactor::core::element::{ProgressBar, Slider};
use windows_reactor::dsl::{button, text_block, ElementExt};

use crate::bindings::{ProgressBar as XamlProgressBar, Slider as XamlSlider};

use crate::fixtures::reconciler::{cc, FixtureFuture};
use crate::harness::Harness;

use windows_reactor::vstack;

/// Verify that updating `text_block(...)` text re-renders without re-creating
/// the TextBlock control (identity preserved across prop change).
pub fn text_block_update(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (label, set) = cx.use_state("Hello".to_string());
            vstack((
                text_block(label).with_key("tb"),
                button("Update").on_click(move || set.call("World".to_string())),
            ))
            .into()
        }));
        h.render().await;

        let tb1 = h.find_text("Hello");
        h.check("PropUpdate_TextBlock_Initial", tb1.is_some());

        let _ = h.click_button("Update");
        h.render().await;

        let tb2 = h.find_text("World");
        h.check("PropUpdate_TextBlock_Updated", tb2.is_some());
        h.check(
            "PropUpdate_TextBlock_OldGone",
            h.find_text("Hello").is_none(),
        );

        // Verify same TextBlock instance was reused (prop diff, not recreate)
        let same = match (tb1, tb2) {
            (Some(a), Some(b)) => identity_eq(&a, &b),
            _ => false,
        };
        h.check("PropUpdate_TextBlock_IdentityPreserved", same);
    })
}

/// Verify that `Slider::value` changes are applied via prop diff.
pub fn slider_value_update(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (val, set) = cx.use_state(25.0f64);
            vstack((
                text_block(format!("val={val}")),
                Slider::new(val).range(0.0, 100.0),
                button("Set75").on_click(move || set.call(75.0)),
            ))
            .into()
        }));
        h.render().await;

        h.check(
            "PropUpdate_Slider_InitialValue",
            h.find_text("val=25").is_some(),
        );

        // Capture the Slider instance before update
        let slider1 = h.find_all::<XamlSlider>(&|_| true);
        let _ = h.click_button("Set75");
        h.render().await;

        h.check(
            "PropUpdate_Slider_UpdatedValue",
            h.find_text("val=75").is_some(),
        );

        // Verify slider value propagated to WinUI
        let sliders = h.find_all::<XamlSlider>(&|_| true);
        let winui_val = sliders
            .first()
            .and_then(|s| s.cast::<crate::bindings::IRangeBase>().ok())
            .and_then(|rb| rb.get_Value().ok())
            .unwrap_or(0.0);
        h.check_eq("PropUpdate_Slider_WinUIValue", 75.0, winui_val);

        // Identity preserved
        let same = match (slider1.first(), sliders.first()) {
            (Some(a), Some(b)) => identity_eq(a, b),
            _ => false,
        };
        h.check("PropUpdate_Slider_IdentityPreserved", same);
    })
}

/// Verify that `ProgressBar::value` changes are applied via prop diff.
pub fn progress_bar_value_update(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (val, set) = cx.use_state(10.0f64);
            vstack((
                text_block(format!("pv={val}")),
                ProgressBar::new(val).range(0.0, 100.0),
                button("Set90").on_click(move || set.call(90.0)),
            ))
            .into()
        }));
        h.render().await;

        h.check(
            "PropUpdate_ProgressBar_Initial",
            h.find_text("pv=10").is_some(),
        );

        let pb1 = h.find_all::<XamlProgressBar>(&|_| true);

        let _ = h.click_button("Set90");
        h.render().await;

        h.check(
            "PropUpdate_ProgressBar_Updated",
            h.find_text("pv=90").is_some(),
        );

        let pbs = h.find_all::<XamlProgressBar>(&|_| true);
        let winui_val = pbs
            .first()
            .and_then(|p| p.cast::<crate::bindings::IRangeBase>().ok())
            .and_then(|rb| rb.get_Value().ok())
            .unwrap_or(0.0);
        h.check_eq("PropUpdate_ProgressBar_WinUIValue", 90.0, winui_val);

        let same = match (pb1.first(), pbs.first()) {
            (Some(a), Some(b)) => identity_eq(a, b),
            _ => false,
        };
        h.check("PropUpdate_ProgressBar_IdentityPreserved", same);
    })
}

/// Verify that `Slider::range` (min/max) changes propagate without
/// re-creating the control. Since `IRangeBase::Maximum()` getter may not
/// be in the minimal binding set, we verify via the rendered text label.
pub fn slider_range_update(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (wide, set) = cx.use_state(false);
            let (min, max) = if wide { (0.0, 1000.0) } else { (0.0, 100.0) };
            vstack((
                text_block(format!("max={max}")),
                Slider::new(50.0).range(min, max),
                button("Widen").on_click(move || set.call(true)),
            ))
            .into()
        }));
        h.render().await;

        h.check(
            "PropUpdate_SliderRange_Initial",
            h.find_text("max=100").is_some(),
        );

        let slider1 = h.find_all::<XamlSlider>(&|_| true);

        let _ = h.click_button("Widen");
        h.render().await;

        h.check(
            "PropUpdate_SliderRange_Updated",
            h.find_text("max=1000").is_some(),
        );

        // Identity preserved (same control reused, not recreated)
        let sliders = h.find_all::<XamlSlider>(&|_| true);
        let same = match (slider1.first(), sliders.first()) {
            (Some(a), Some(b)) => identity_eq(a, b),
            _ => false,
        };
        h.check("PropUpdate_SliderRange_IdentityPreserved", same);
    })
}

/// Verify that Button `is_enabled` prop changes propagate.
pub fn button_enabled_update(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (enabled, set) = cx.use_state(false);
            vstack((
                text_block(format!("enabled={enabled}")),
                button("Target").enabled(enabled),
                button("Enable").on_click(move || set.call(true)),
            ))
            .into()
        }));
        h.render().await;

        h.check(
            "PropUpdate_ButtonEnabled_InitialDisabled",
            h.find_text("enabled=false").is_some(),
        );

        let _ = h.click_button("Enable");
        h.render().await;

        h.check(
            "PropUpdate_ButtonEnabled_Updated",
            h.find_text("enabled=true").is_some(),
        );

        // Verify WinUI IsEnabled on the "Target" button
        let target = h.find_button("Target");
        let is_enabled = target
            .as_ref()
            .and_then(|b| b.cast::<crate::bindings::IControl>().ok())
            .and_then(|c| c.get_IsEnabled().ok())
            .unwrap_or(false);
        h.check("PropUpdate_ButtonEnabled_WinUIValue", is_enabled);
    })
}

fn identity_eq<T: Interface, U: Interface>(a: &T, b: &U) -> bool {
    use windows_core::IUnknown;
    let Ok(ua) = a.cast::<IUnknown>() else {
        return false;
    };
    let Ok(ub) = b.cast::<IUnknown>() else {
        return false;
    };
    ua.as_raw() == ub.as_raw()
}
