//! End-to-end interaction fixtures: drive each input control through the
//! real WinUI event surface and verify the reactor's widget callbacks
//! fires and the next render reflects the new state. These complement the
//! purely-structural `mount_*` fixtures, which only assert initial render.

use std::time::Duration;

use windows_core::Interface as _;

use windows_reactor::AnimationConfig;
use windows_reactor::Element;
use windows_reactor::Icon;
use windows_reactor::NavViewItem;
use windows_reactor::NavigationView;
use windows_reactor::Symbol;
use windows_reactor::vstack;
use windows_reactor::{ComboBox, PasswordBox, RadioButtons, Slider, ToggleSwitch};
use windows_reactor::{KeyExt, VisualExt, button, check_box, text_block, text_box};

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
    // Exercises the `IsEnabled` unset fallback.
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
                    .and_then(|c| c.IsEnabled().ok())
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
                    .and_then(|c| c.IsEnabled().ok())
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
            .and_then(|tb| tb.cast::<crate::bindings::ITextBlock>().ok()?.Text().ok())
            .unwrap_or_else(|| "<no radio-idx= text block found>".into());
        h.check_eq(
            "Interaction_RadioButtons_InitialZero",
            "radio-idx=0".to_string(),
            initial,
        );

        // Programmatic selection is expected to fail reliably.
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
            .and_then(|tb| tb.cast::<crate::bindings::ITextBlock>().ok()?.Text().ok())
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
                    .icon(Symbol::Favorite)
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
                Symbol::Save
            } else {
                Symbol::Favorite
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
            .Symbol()
            .unwrap();
        h.check(
            "Interaction_ButtonIconGlyph_InitialIsFavorite",
            initial_symbol == crate::bindings::Symbol(57619),
        );
        h.check(
            "Interaction_ButtonIconGlyph_InitialTextPresent",
            h.find_text("Action").is_some(),
        );

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
            .Symbol()
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

/// Verify that the non-`Symbol` [`Icon`](windows_reactor::Icon) kinds construct
/// and attach the intended WinUI `IconElement` subclasses.
pub fn button_icon_subclasses(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_cx| {
            vstack((
                button("Starred").icon(Icon::font("\u{E734}")),
                button("Raster").icon(Icon::image("ms-appx:///Assets/logo.png")),
                button("Vector").icon(Icon::image("ms-appx:///Assets/logo.svg")),
                button("Monochrome bitmap")
                    .icon(Icon::bitmap_icon("ms-appx:///Assets/logo.png", true)),
                button("Color bitmap").icon(Icon::bitmap_icon("ms-appx:///Assets/logo.png", false)),
                button("Path").icon(Icon::path("F1 M 0,8 L 6,14 L 16,2 L 14,0 L 6,10 L 2,6 Z")),
                NavigationView::new(
                    [
                        NavViewItem::new("Bitmap item")
                            .icon(Icon::bitmap_icon("ms-appx:///Assets/logo.png", false)),
                        NavViewItem::new("Path item").icon(Icon::path("F1 M 0,0 L 12,0 L 6,12 Z")),
                    ],
                    text_block("Navigation icon host"),
                )
                .settings_visible(false),
            ))
            .into()
        }));
        h.render().await;

        let font_icons = h.find_all::<crate::bindings::FontIcon>(&|_| true);
        h.check(
            "Interaction_ButtonIcon_FontIconCreated",
            font_icons.len() == 1,
        );

        let image_icons = h.find_all::<crate::bindings::ImageIcon>(&|_| true);
        h.check(
            "Interaction_ButtonIcon_ImageIconsCreated",
            image_icons.len() == 2,
        );
        let image_icons_constrained = image_icons.iter().all(|icon| {
            icon.cast::<crate::bindings::IFrameworkElement>()
                .ok()
                .is_some_and(|element| {
                    element.MaxWidth().ok() == Some(20.0) && element.MaxHeight().ok() == Some(20.0)
                })
        });
        h.check(
            "Interaction_ButtonIcon_ImageIconsConstrained",
            image_icons_constrained,
        );

        let sources: Vec<_> = image_icons
            .iter()
            .map(|icon| icon.Source().unwrap())
            .collect();
        let bitmap_sources = sources
            .iter()
            .filter(|source| source.cast::<crate::bindings::BitmapImage>().is_ok())
            .count();
        h.check(
            "Interaction_ButtonIcon_BitmapSourceCreated",
            bitmap_sources == 1,
        );
        let svg_sources = sources
            .iter()
            .filter(|source| source.cast::<crate::bindings::SvgImageSource>().is_ok())
            .count();
        h.check("Interaction_ButtonIcon_SvgSourceCreated", svg_sources == 1);

        let bitmap_icons = h.find_all::<crate::bindings::BitmapIcon>(&|_| true);
        h.check(
            "Interaction_ButtonIcon_BitmapIconsCreated",
            bitmap_icons.len() >= 2,
        );
        let monochrome_modes: Vec<_> = bitmap_icons
            .iter()
            .map(|icon| icon.ShowAsMonochrome().unwrap())
            .collect();
        h.check(
            "Interaction_ButtonIcon_BitmapModesApplied",
            monochrome_modes.contains(&false) && monochrome_modes.contains(&true),
        );

        let path_icons = h.find_all::<crate::bindings::PathIcon>(&|_| true);
        h.check(
            "Interaction_ButtonIcon_PathIconCreated",
            !path_icons.is_empty(),
        );
        h.check(
            "Interaction_ButtonIcon_PathDataParsed",
            path_icons.first().is_some_and(|icon| icon.Data().is_ok()),
        );

        let navigation = h
            .find_all::<crate::bindings::NavigationView>(&|_| true)
            .into_iter()
            .next()
            .unwrap();
        let items = navigation.MenuItems().unwrap();
        let bitmap_item: crate::bindings::NavigationViewItem =
            items.GetAt(0).unwrap().cast().unwrap();
        let path_item: crate::bindings::NavigationViewItem =
            items.GetAt(1).unwrap().cast().unwrap();
        h.check(
            "Interaction_NavigationViewItem_CustomIconsCreated",
            bitmap_item
                .Icon()
                .is_ok_and(|icon| icon.cast::<crate::bindings::BitmapIcon>().is_ok())
                && path_item
                    .Icon()
                    .is_ok_and(|icon| icon.cast::<crate::bindings::PathIcon>().is_ok()),
        );
    })
}

pub fn element_exit_transition(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (visible, set_visible) = cx.use_state(true);
            let (transition_enabled, set_transition_enabled) = cx.use_state(true);
            let child: Element = if visible {
                let mut child = button("Animated child");
                if transition_enabled {
                    child = child.transition(
                        Some(AnimationConfig::fade_in(Duration::from_millis(100))),
                        Some(AnimationConfig::fade_out(Duration::from_millis(800))),
                    );
                }
                child.into()
            } else {
                Element::Empty
            };

            vstack((
                button("Toggle child transition")
                    .on_click(move || set_transition_enabled.call(!transition_enabled)),
                button("Remove animated child").on_click(move || set_visible.call(false)),
                child,
            ))
            .into()
        }));
        h.render().await;

        let _ = h.click_button("Toggle child transition");
        h.render().await;
        h.check(
            "Interaction_ExitTransition_ClearedWithoutRemoval",
            h.find_button("Animated child").is_some(),
        );

        let _ = h.click_button("Toggle child transition");
        h.render().await;
        let _ = h.click_button("Remove animated child");
        h.render().await;
        h.check(
            "Interaction_ExitTransition_LogicalRemovalImmediate",
            h.find_button("Animated child").is_none(),
        );
    })
}

/// Verify that removing a button's icon (`Some(Icon)` -> `None`) clears
/// the previously-applied `IconElement` and restores the plain text label. This
/// exercises the `(Prop::Icon, PropValue::Unset, Handle::Button)` backend arm.
pub fn button_icon_removal_clears_icon(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (removed, set) = cx.use_state(false);
            let mut b = button("Action").on_click(move || set.call(!removed));
            if !removed {
                b = b.icon(Symbol::Favorite);
            }
            vstack((b, text_block(format!("removed={removed}")))).into()
        }));
        h.render().await;

        // Initial state: one SymbolIcon plus the "Action" text label.
        h.check(
            "Interaction_ButtonIconRemoval_InitialIconPresent",
            h.find_all::<crate::bindings::SymbolIcon>(&|_| true).len() == 1,
        );
        h.check(
            "Interaction_ButtonIconRemoval_InitialTextPresent",
            h.find_text("Action").is_some(),
        );

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

        // After removal: no SymbolIcon remains, and the text label is preserved.
        h.check(
            "Interaction_ButtonIconRemoval_IconCleared",
            h.find_all::<crate::bindings::SymbolIcon>(&|_| true)
                .is_empty(),
        );
        h.check(
            "Interaction_ButtonIconRemoval_TextPreserved",
            h.find_text("Action").is_some(),
        );
    })
}
