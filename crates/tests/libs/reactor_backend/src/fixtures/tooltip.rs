//! Selftest fixtures for the `Modifiers::tooltip` slot (gap M1).
//!
//! Mounts a `Button` with `.tooltip("hi")` and asserts that the WinUI
//! attached property `Microsoft.UI.Xaml.Controls.ToolTipService.ToolTip`
//! on the realized control returns the expected `IInspectable` string.
//! Programmatic read of an attached property is a synchronous call, so
//! these fixtures use the regular `Harness::check` (no SKIPs needed).

use crate::bindings;
use windows_reactor::core::element::{Element, Tooltip, TooltipPlacement};
use windows_reactor::dsl::{button, ElementExt};
use windows_reactor::vstack;

use crate::fixtures::reconciler::{cc, FixtureFuture};
use crate::harness::Harness;

fn tooltip_string_for_button(h: &Harness, label: &str) -> Option<String> {
    use windows_core::Interface;
    let btn = h.find_button(label)?;
    let dep = btn.cast::<bindings::DependencyObject>().ok()?;
    let insp = bindings::ToolTipService::GetToolTip(&dep).ok()?;
    let pv = insp
        .cast::<windows_reference::IReference<windows_core::HSTRING>>()
        .ok()?;
    Some(pv.Value().ok()?.to_string_lossy())
}

fn tooltip_placement_for_button(h: &Harness, label: &str) -> Option<bindings::PlacementMode> {
    use windows_core::Interface;
    let btn = h.find_button(label)?;
    let dep = btn.cast::<bindings::DependencyObject>().ok()?;
    bindings::ToolTipService::GetPlacement(&dep).ok()
}

pub fn tooltip_text(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| button("Hover").tooltip("hi").into()));
        h.render().await;
        let got = tooltip_string_for_button(&h, "Hover");
        h.check("Tooltip_Text_AppliedToButton", got.as_deref() == Some("hi"));
    })
}

pub fn tooltip_clear_on_update(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        // First render: tooltip = "first".
        h.mount(cc(|_| button("Hover").tooltip("first").into()));
        h.render().await;
        h.check(
            "Tooltip_Clear_Initial",
            tooltip_string_for_button(&h, "Hover").as_deref() == Some("first"),
        );

        // Re-render without the modifier — the WinUI attached property
        // would otherwise persist across re-renders; the reconciler
        // must emit a `SetTooltip(None)` so the slot is cleared.
        h.mount(cc(|_| {
            let el: Element = button("Hover").into();
            el
        }));
        h.render().await;
        h.check(
            "Tooltip_Clear_AfterRemove",
            tooltip_string_for_button(&h, "Hover").is_none(),
        );
    })
}

pub fn tooltip_placement(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_| {
            vstack((
                button("Hover").tooltip_with(Tooltip::text("tip").placement(TooltipPlacement::Top)),
            ))
            .into()
        }));
        h.render().await;
        let got = tooltip_placement_for_button(&h, "Hover");
        h.check(
            "Tooltip_Placement_Top",
            got == Some(bindings::PlacementMode::Top),
        );
    })
}
