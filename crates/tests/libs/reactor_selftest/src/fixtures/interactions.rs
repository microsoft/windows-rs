//! End-to-end interaction fixtures: drive each input control through the
//! real WinUI event surface and verify the reactor's widget callbacks
//! fires and the next render reflects the new state. These complement the
//! purely-structural `mount_*` fixtures, which only assert initial render.

use windows_core::Interface as _;

use windows_reactor::SymbolGlyph;
use windows_reactor::core::element::Element;
use windows_reactor::core::element::{ComboBox, PasswordBox, RadioButtons, Slider, ToggleSwitch};
use windows_reactor::dsl::{ElementExt, button, check_box, text_block, text_box};
use windows_reactor::vstack;

use crate::fixtures::reconciler::{FixtureFuture, cc};
use crate::harness::Harness;

pub fn checkbox_toggles_state(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (checked, set) = cx.use_state(false);
            vstack((
                text_block(format!("checked={checked}")),
                check_box(checked)
                    .content("agree")
                    .on_checked(move |v| set.call(v)),
            ))
            .into()
        }));
        h.render().await;
        h.check(
            "Interaction_CheckBox_InitialUnchecked",
            h.find_text("checked=false").is_some(),
        );

        let _ = h.set_checkbox_value(true);
        h.render().await;
        h.check(
            "Interaction_CheckBox_AfterToggle",
            h.find_text("checked=true").is_some(),
        );
    })
}

pub fn toggle_switch_changes_state(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (on, set) = cx.use_state(false);
            vstack((
                text_block(format!("on={on}")),
                ToggleSwitch::new(on).on_toggled(move |v| set.call(v)),
            ))
            .into()
        }));
        h.render().await;
        h.check(
            "Interaction_ToggleSwitch_InitialOff",
            h.find_text("on=false").is_some(),
        );

        let _ = h.set_toggle_switch_value(true);
        h.render().await;
        h.check(
            "Interaction_ToggleSwitch_AfterToggle",
            h.find_text("on=true").is_some(),
        );
    })
}

pub fn slider_value_changes_state(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (value, set) = cx.use_state(10.0_f64);
            vstack((
                text_block(format!("value={}", value as i32)),
                Slider::new(value)
                    .range(0.0, 100.0)
                    .on_value_changed(move |v| set.call(v)),
            ))
            .into()
        }));
        h.render().await;
        h.check(
            "Interaction_Slider_InitialValue",
            h.find_text("value=10").is_some(),
        );

        let _ = h.set_slider_value(73.0);
        h.render().await;
        h.check(
            "Interaction_Slider_AfterValueChange",
            h.find_text("value=73").is_some(),
        );
    })
}

pub fn text_field_changes_state(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (text, set) = cx.use_state(String::new());
            vstack((
                text_block(format!("typed='{text}'")),
                text_box(text).on_text_changed(move |v| set.call(v)),
            ))
            .into()
        }));
        h.render().await;
        h.check(
            "Interaction_TextField_InitialEmpty",
            h.find_text("typed=''").is_some(),
        );

        let _ = h.set_text_field_value("hello");
        h.render().await;
        h.check(
            "Interaction_TextField_AfterTyping",
            h.find_text("typed='hello'").is_some(),
        );
    })
}

pub fn button_disabled_to_enabled_prop_update(h: Harness) -> FixtureFuture {
    // Exercises the IsEnabled→Unset fallback path (disabled→enabled transition).
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (enabled, set) = cx.use_state(false);
            let mut b = button("Action");
            if !enabled {
                b = b.enabled(false);
            }
            vstack((b, button("Enable").on_click(move || set.call(true)))).into()
        }));
        h.render().await;

        let initially_disabled = h
            .find_button("Action")
            .and_then(|btn| {
                btn.cast::<crate::bindings::IControl>()
                    .ok()
                    .and_then(|c| c.get_IsEnabled().ok())
            })
            .unwrap_or(true);
        h.check(
            "Interaction_PropUpdate_InitiallyDisabled",
            !initially_disabled,
        );

        let _ = h.click_button("Enable");
        h.render().await;

        let now_enabled = h
            .find_button("Action")
            .and_then(|btn| {
                btn.cast::<crate::bindings::IControl>()
                    .ok()
                    .and_then(|c| c.get_IsEnabled().ok())
            })
            .unwrap_or(false);
        h.check("Interaction_PropUpdate_NowEnabled", now_enabled);
    })
}

pub fn pool_churn_grow_shrink_grow(h: Harness) -> FixtureFuture {
    // Exercises element pool: shrink to zero then regrow to verify recycle.
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (count, set) = cx.use_state(4i32);
            let inc = {
                let set = set.clone();
                move || set.call(count + 1)
            };
            let dec = {
                let set = set.clone();
                move || set.call((count - 1).max(0))
            };
            let zero = {
                let set = set;
                move || set.call(0)
            };
            let items: Vec<Element> = (0..count)
                .map(|i| {
                    text_block(format!("Pool#{i}"))
                        .with_key(format!("pool-{i}"))
                        .into()
                })
                .collect();
            vstack((
                windows_reactor::hstack((
                    button("Add").on_click(inc),
                    button("Remove").on_click(dec),
                    button("Empty").on_click(zero),
                ))
                .spacing(8.0),
                vstack(items).spacing(2.0),
            ))
            .spacing(8.0)
            .into()
        }));
        h.render().await;
        h.check(
            "Interaction_PoolChurn_InitialFour",
            h.find_text("Pool#0").is_some() && h.find_text("Pool#3").is_some(),
        );

        let _ = h.click_button("Empty");
        h.render().await;
        h.check(
            "Interaction_PoolChurn_EmptiedToZero",
            h.find_text("Pool#0").is_none(),
        );

        let _ = h.click_button("Add");
        h.render().await;
        let _ = h.click_button("Add");
        h.render().await;
        let _ = h.click_button("Add");
        h.render().await;
        h.check(
            "Interaction_PoolChurn_RegrowAfterEmpty",
            h.find_text("Pool#0").is_some()
                && h.find_text("Pool#1").is_some()
                && h.find_text("Pool#2").is_some(),
        );

        let _ = h.click_button("Remove");
        h.render().await;
        let _ = h.click_button("Add");
        h.render().await;
        h.check(
            "Interaction_PoolChurn_ShrinkThenGrow",
            h.find_text("Pool#2").is_some(),
        );
    })
}

pub fn password_box_changes_state(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (value, set) = cx.use_state(String::new());
            vstack((
                text_block(format!("pwd-len={}", value.len())),
                PasswordBox::new()
                    .value(value)
                    .on_password_changed(move |v| set.call(v)),
            ))
            .into()
        }));
        h.render().await;
        h.check(
            "Interaction_PasswordBox_InitialEmpty",
            h.find_text("pwd-len=0").is_some(),
        );

        let _ = h.set_password_box_value("hunter22");
        h.render().await;
        h.check(
            "Interaction_PasswordBox_AfterChange",
            h.find_text("pwd-len=8").is_some(),
        );
    })
}

pub fn radio_buttons_change_selection(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (idx, set) = cx.use_state(0i32);
            vstack((
                text_block(format!("radio-idx={idx}")),
                RadioButtons::new(["Email", "SMS", "None"])
                    .selected_index(idx)
                    .on_selection_changed(move |i| set.call(i)),
            ))
            .into()
        }));
        // RadioButtons children materialize over several dispatcher turns.
        h.render_until("RadioButton children to materialize", |h| {
            h.count_controls::<crate::bindings::RadioButton>() >= 3
        })
        .await;

        let initial = h
            .find_text_containing("radio-idx=")
            .and_then(|tb| {
                tb.cast::<crate::bindings::ITextBlock>()
                    .ok()?
                    .get_Text()
                    .ok()
            })
            .unwrap_or_else(|| "<no radio-idx= text block found>".into());
        h.check_eq(
            "Interaction_RadioButtons_InitialZero",
            "radio-idx=0".to_string(),
            initial,
        );

        // Try programmatic selection — this is known to fail reliably
        // because WinUI RadioButtons.SelectionChanged only fires for real
        // input events. Silence errors and fall through to the SKIP path.
        let _ = h.set_radio_buttons_selected_index(2);
        // RadioButtons doesn't reliably fire SelectionChanged for programmatic input.
        let arrived = h
            .render_until_quiet("radio-idx=2 to appear", |h| {
                h.find_text("radio-idx=2").is_some()
            })
            .await;

        if !arrived {
            h.check_skip(
                "Interaction_RadioButtons_AfterChange",
                "programmatic RadioButtons selection not supported in-process",
            );
            return;
        }

        let after = h
            .find_text_containing("radio-idx=")
            .and_then(|tb| {
                tb.cast::<crate::bindings::ITextBlock>()
                    .ok()?
                    .get_Text()
                    .ok()
            })
            .unwrap_or_else(|| "<no radio-idx= text block found>".into());
        let h2 = h.clone();
        h.check_with(
            "Interaction_RadioButtons_AfterChange",
            after == "radio-idx=2",
            move || {
                format!(
                    "expected radio-idx=2 but rendered text was {after:?}; visual tree:\n{}",
                    h2.dump_tree()
                )
            },
        );
    })
}

pub fn combo_box_changes_selection(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (idx, set) = cx.use_state(-1i32);
            vstack((
                text_block(format!("combo-idx={idx}")),
                ComboBox::new(["Red", "Green", "Blue"])
                    .selected_index(idx)
                    .on_selection_changed(move |i| set.call(i)),
            ))
            .into()
        }));
        h.render().await;
        h.check(
            "Interaction_ComboBox_InitialUnset",
            h.find_text("combo-idx=-1").is_some(),
        );

        let _ = h.set_combo_box_selected_index(1);
        h.render().await;
        h.check(
            "Interaction_ComboBox_AfterChange",
            h.find_text("combo-idx=1").is_some(),
        );
    })
}

/// Verify that updating a button's label preserves the SymbolIcon when the
/// button was created with `.icon(...)`. Without the ButtonContent fix the
/// entire StackPanel (icon + text) would be replaced by a bare TextBlock.
pub fn button_icon_label_preserved(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (count, set) = cx.use_state(0u32);
            vstack((
                button(format!("Clicked {count}"))
                    .icon(SymbolGlyph::Favorite)
                    .on_click(move || set.call(count + 1)),
                text_block(format!("count={count}")),
            ))
            .into()
        }));
        h.render().await;

        // Initial: icon should be present.
        let has_icon_before = !h
            .find_all::<crate::bindings::SymbolIcon>(&|_| true)
            .is_empty();
        h.check("Interaction_ButtonIcon_InitialIconPresent", has_icon_before);
        h.check(
            "Interaction_ButtonIcon_InitialLabel",
            h.find_text("Clicked 0").is_some(),
        );

        // Click — label changes to "Clicked 1".
        let btn = h
            .find_all::<crate::bindings::Button>(&|_| true)
            .into_iter()
            .next()
            .unwrap();
        let peer = crate::bindings::ButtonAutomationPeer::CreateInstanceWithOwner(&btn).unwrap();
        let pat = peer
            .cast::<crate::bindings::IAutomationPeer>()
            .unwrap()
            .GetPattern(crate::bindings::PatternInterface::Invoke)
            .unwrap();
        let invoke: crate::bindings::IInvokeProvider = pat.cast().unwrap();
        invoke.Invoke().unwrap();
        h.render().await;

        // After click: icon must still be present and label updated.
        let has_icon_after = !h
            .find_all::<crate::bindings::SymbolIcon>(&|_| true)
            .is_empty();
        h.check(
            "Interaction_ButtonIcon_IconPreservedAfterLabelChange",
            has_icon_after,
        );
        h.check(
            "Interaction_ButtonIcon_LabelUpdated",
            h.find_text("Clicked 1").is_some(),
        );
    })
}

/// Verify that changing a button's icon glyph preserves the text label.
pub fn button_icon_glyph_change_preserves_text(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (toggled, set) = cx.use_state(false);
            let icon = if toggled {
                SymbolGlyph::Save
            } else {
                SymbolGlyph::Favorite
            };
            vstack((
                button("Action")
                    .icon(icon)
                    .on_click(move || set.call(!toggled)),
                text_block(format!("toggled={toggled}")),
            ))
            .into()
        }));
        h.render().await;

        // Initial state: icon is Favorite (57619), text label present.
        let icons_before = h.find_all::<crate::bindings::SymbolIcon>(&|_| true);
        h.check(
            "Interaction_ButtonIconGlyph_InitialIconPresent",
            icons_before.len() == 1,
        );
        let initial_symbol = icons_before[0]
            .cast::<crate::bindings::ISymbolIcon>()
            .unwrap()
            .get_Symbol()
            .unwrap();
        h.check(
            "Interaction_ButtonIconGlyph_InitialIsFavorite",
            initial_symbol == crate::bindings::Symbol(57619),
        );
        h.check(
            "Interaction_ButtonIconGlyph_InitialTextPresent",
            h.find_text("Action").is_some(),
        );

        // Click — icon changes from Favorite to Save.
        let btn = h
            .find_all::<crate::bindings::Button>(&|_| true)
            .into_iter()
            .next()
            .unwrap();
        let peer = crate::bindings::ButtonAutomationPeer::CreateInstanceWithOwner(&btn).unwrap();
        let pat = peer
            .cast::<crate::bindings::IAutomationPeer>()
            .unwrap()
            .GetPattern(crate::bindings::PatternInterface::Invoke)
            .unwrap();
        let invoke: crate::bindings::IInvokeProvider = pat.cast().unwrap();
        invoke.Invoke().unwrap();
        h.render().await;

        // After click: icon should be Save (57605), text still "Action",
        // and exactly one SymbolIcon (no nested panels).
        let icons_after = h.find_all::<crate::bindings::SymbolIcon>(&|_| true);
        h.check(
            "Interaction_ButtonIconGlyph_StillOneIcon",
            icons_after.len() == 1,
        );
        let new_symbol = icons_after[0]
            .cast::<crate::bindings::ISymbolIcon>()
            .unwrap()
            .get_Symbol()
            .unwrap();
        h.check(
            "Interaction_ButtonIconGlyph_ChangedToSave",
            new_symbol == crate::bindings::Symbol(57605),
        );
        h.check(
            "Interaction_ButtonIconGlyph_TextPreserved",
            h.find_text("Action").is_some(),
        );
    })
}
