//! Live WinUI coverage for refactored `try_universal_prop` paths and related
//! helpers: min/max sizing, canvas z_index, relative-panel alignment, and
//! background/foreground/padding set+clear transitions.
//!
//! Most of these cannot read back values from WinUI (the selftest bindings
//! are minimal), so the tests verify:
//! 1. Mount with the prop set succeeds (COM call doesn't fail/panic).
//! 2. Prop transitions (set → clear → set) succeed without errors.
//! 3. No `windows-reactor:` diagnostic warnings are emitted.

use windows_reactor::{Canvas, Color, Element, RelativePanel};
use windows_reactor::{ElementExt, button, text_block};

use crate::fixtures::reconciler::{FixtureFuture, cc};
use crate::harness::Harness;

use windows_reactor::vstack;

/// Mount a TextBlock with min/max constraints, then toggle them off and back
/// on. Verifies the COM calls succeed on a real FrameworkElement.
pub fn min_max_sizing(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        let cap = h.capture_stderr();
        h.mount(cc(|cx| {
            let (constrained, set) = cx.use_state(true);
            let tb = if constrained {
                text_block("constrained")
                    .min_width(50.0)
                    .max_width(400.0)
                    .min_height(20.0)
                    .max_height(200.0)
            } else {
                text_block("unconstrained")
            };
            vstack((
                tb,
                button("Toggle").on_click(move || set.call(!constrained)),
            ))
            .into()
        }));
        h.render().await;

        h.check(
            "UniversalProp_MinMax_MountConstrained",
            h.find_text("constrained").is_some(),
        );

        let _ = h.click_button("Toggle");
        h.render().await;
        h.check(
            "UniversalProp_MinMax_ClearConstraints",
            h.find_text("unconstrained").is_some(),
        );

        let _ = h.click_button("Toggle");
        h.render().await;
        h.check(
            "UniversalProp_MinMax_RestoreConstraints",
            h.find_text("constrained").is_some(),
        );

        h.check_no_reactor_warnings("UniversalProp_MinMax_NoWarnings", &cap.finish());
    })
}

/// Mount a Canvas child with canvas_z_index set. Verifies the static
/// Canvas.SetZIndex COM call succeeds.
pub fn canvas_z_index(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        let cap = h.capture_stderr();
        h.mount(cc(|_| {
            Canvas::new([
                text_block("back").canvas_z_index(0),
                text_block("front").canvas_z_index(10),
            ])
            .into()
        }));
        h.render().await;

        h.check(
            "UniversalProp_CanvasZIndex_BackPresent",
            h.find_text("back").is_some(),
        );
        h.check(
            "UniversalProp_CanvasZIndex_FrontPresent",
            h.find_text("front").is_some(),
        );
        h.check_no_reactor_warnings("UniversalProp_CanvasZIndex_NoWarnings", &cap.finish());
    })
}

/// Mount a RelativePanel with children using alignment attached props.
/// Verifies the static RelativePanel.SetAlign*WithPanel COM calls succeed.
pub fn relative_panel_alignment(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        let cap = h.capture_stderr();
        h.mount(cc(|_| {
            let left: Element = text_block("L").into();
            let right: Element = text_block("R").into();
            let top: Element = text_block("T").into();
            let bottom: Element = text_block("B").into();
            let hc: Element = text_block("HC").into();
            let vc: Element = text_block("VC").into();
            RelativePanel::new([
                left.relative_align_left(),
                right.relative_align_right(),
                top.relative_align_top(),
                bottom.relative_align_bottom(),
                hc.relative_align_h_center(),
                vc.relative_align_v_center(),
            ])
            .into()
        }));
        h.render().await;

        h.check(
            "UniversalProp_RelativeAlign_ChildrenPresent",
            h.find_text("L").is_some() && h.find_text("R").is_some(),
        );
        h.check_no_reactor_warnings("UniversalProp_RelativeAlign_NoWarnings", &cap.finish());
    })
}

/// Mount with background/foreground/padding set, then clear them, then
/// restore them. Verifies the set/unset COM calls on IControl/IPanel/IBorder
/// don't fail. Uses a Button since it implements IControl (which supports
/// Background, Foreground, and Padding).
pub fn background_foreground_padding_transition(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        let cap = h.capture_stderr();
        h.mount(cc(|cx| {
            let (styled, set) = cx.use_state(true);
            let btn = if styled {
                button("styled")
                    .background(Color {
                        a: 255,
                        r: 200,
                        g: 0,
                        b: 0,
                    })
                    .foreground(Color {
                        a: 255,
                        r: 255,
                        g: 255,
                        b: 255,
                    })
                    .padding(10.0)
            } else {
                button("plain")
            };
            vstack((btn, button("Toggle").on_click(move || set.call(!styled)))).into()
        }));
        h.render().await;

        h.check(
            "UniversalProp_BgFgPad_MountStyled",
            h.find_button("styled").is_some(),
        );

        let _ = h.click_button("Toggle");
        h.render().await;
        h.check(
            "UniversalProp_BgFgPad_ClearStyle",
            h.find_button("plain").is_some(),
        );

        let _ = h.click_button("Toggle");
        h.render().await;
        h.check(
            "UniversalProp_BgFgPad_RestoreStyle",
            h.find_button("styled").is_some(),
        );

        h.check_no_reactor_warnings("UniversalProp_BgFgPad_NoWarnings", &cap.finish());
    })
}

/// Mount with opacity set, then clear it. Verifies the UIElement.Opacity COM
/// call succeeds in both directions.
pub fn opacity_transition(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        let cap = h.capture_stderr();
        h.mount(cc(|cx| {
            let (faded, set) = cx.use_state(true);
            let tb = if faded {
                text_block("faded").opacity(0.5)
            } else {
                text_block("full")
            };
            vstack((tb, button("Toggle").on_click(move || set.call(!faded)))).into()
        }));
        h.render().await;

        h.check(
            "UniversalProp_Opacity_MountFaded",
            h.find_text("faded").is_some(),
        );

        let _ = h.click_button("Toggle");
        h.render().await;
        h.check(
            "UniversalProp_Opacity_ClearOpacity",
            h.find_text("full").is_some(),
        );

        h.check_no_reactor_warnings("UniversalProp_Opacity_NoWarnings", &cap.finish());
    })
}
