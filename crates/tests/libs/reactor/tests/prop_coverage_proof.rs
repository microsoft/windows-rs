//! Proof that optional props correctly emit `PropValue::Unset` through
//! the reconciler when they are removed from a widget's configuration.
//!
//! Each test mounts a widget with optional props set, then reconciles to
//! the bare version and asserts the expected Unset ops appear.

use std::rc::Rc;

use rustc_hash::FxHashSet;
use windows_reactor::core::backend::{Op, Prop, PropValue, RecordingBackend};
use windows_reactor::core::element::Element;
use windows_reactor::core::reconciler::Reconciler;

fn noop_rr() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

/// Mount full, reconcile to bare, return the set of props that got Unset.
fn unset_props_on_transition(full: Element, bare: Element) -> FxHashSet<Prop> {
    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, &full, None, noop_rr());
    r.backend.clear_ops();
    let _ = r.reconcile(Some(&full), &bare, id, noop_rr());

    r.backend
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::SetProp {
                prop,
                value: PropValue::Unset,
                ..
            } => Some(*prop),
            _ => None,
        })
        .collect()
}

/// Assert that all expected props got Unset.
fn assert_unsets(widget_name: &str, full: Element, bare: Element, expected: &[Prop]) {
    let actual = unset_props_on_transition(full, bare);
    let mut missing: Vec<&Prop> = expected.iter().filter(|p| !actual.contains(p)).collect();
    missing.sort_by_key(|p| format!("{p:?}"));
    assert!(
        missing.is_empty(),
        "{widget_name}: expected Unset for {missing:?} but only got {actual:?}"
    );
}

// ─── Tests ───────────────────────────────────────────────────────────────────

use windows_reactor::core::element::*;

#[test]
fn textblock_optional_props_unset() {
    let full: Element = TextBlock {
        content: "hello".into(),
        font_size: Some(20.0),
        font_weight: Some(700),
        wrap_text: true,
        is_text_selection_enabled: true,
        ..TextBlock::default()
    }
    .into();
    let bare: Element = TextBlock::new("hello").into();
    assert_unsets(
        "TextBlock",
        full,
        bare,
        &[
            Prop::FontSize,
            Prop::FontWeight,
            Prop::TextWrappingWrap,
            Prop::IsTextSelectionEnabled,
        ],
    );
}

#[test]
fn button_is_enabled_unset() {
    let full: Element = Button {
        label: "click".into(),
        is_enabled: false,
        ..Button::default()
    }
    .into();
    let bare: Element = Button::new("click").into();
    assert_unsets("Button", full, bare, &[Prop::IsEnabled]);
}

#[test]
fn checkbox_optional_props_unset() {
    let full: Element = CheckBox {
        label: Some("opt".into()),
        is_checked: true,
        is_enabled: false,
        ..CheckBox::default()
    }
    .into();
    // bare keeps is_checked=true so only label and IsEnabled should unset
    let bare: Element = CheckBox::new(true).into();
    assert_unsets(
        "CheckBox",
        full,
        bare,
        &[Prop::CheckBoxLabel, Prop::IsEnabled],
    );
}

#[test]
fn textbox_optional_props_unset() {
    let full: Element = TextBox {
        value: "hi".into(),
        placeholder: Some("type...".into()),
        header: Some("Name".into()),
        accepts_return: true,
        text_wrapping_wrap: true,
        is_enabled: false,
        ..TextBox::default()
    }
    .into();
    let bare: Element = TextBox::new("hi").into();
    assert_unsets(
        "TextBox",
        full,
        bare,
        &[
            Prop::Placeholder,
            Prop::Header,
            Prop::AcceptsReturn,
            Prop::TextWrappingWrap,
            Prop::IsEnabled,
        ],
    );
}

#[test]
fn textbox_is_enabled_default_coverage() {
    // TextBox::new sets is_enabled=true, TextBox::default sets false.
    // When is_enabled goes true→true no Unset is emitted (always present).
    // This test just verifies the disabled→enabled transition emits Unset.
    let full: Element = TextBox {
        value: "x".into(),
        is_enabled: false,
        ..TextBox::default()
    }
    .into();
    let bare: Element = TextBox::new("x").into();
    assert_unsets("TextBox(is_enabled)", full, bare, &[Prop::IsEnabled]);
}

#[test]
fn toggle_switch_optional_props_unset() {
    let full: Element = ToggleSwitch {
        is_on: true,
        is_enabled: false,
        header: Some("Toggle".into()),
        on_content: Some("Yes".into()),
        off_content: Some("No".into()),
        ..ToggleSwitch::default()
    }
    .into();
    let bare: Element = ToggleSwitch::new(true).into();
    assert_unsets(
        "ToggleSwitch",
        full,
        bare,
        &[
            Prop::Header,
            Prop::OnContent,
            Prop::OffContent,
            Prop::IsEnabled,
        ],
    );
}

#[test]
fn slider_optional_props_unset() {
    let full: Element = Slider {
        value: 50.0,
        minimum: 10.0,
        maximum: 200.0,
        is_enabled: false,
        header: Some("Volume".into()),
        vertical: true,
        ..Slider::default()
    }
    .into();
    let bare: Element = Slider::default().into();
    assert_unsets(
        "Slider",
        full,
        bare,
        &[Prop::Header, Prop::Orientation, Prop::IsEnabled],
    );
}

#[test]
fn number_box_optional_props_unset() {
    let full: Element = NumberBox {
        value: 42.0,
        minimum: Some(0.0),
        maximum: Some(100.0),
        header: Some("Count".into()),
        is_enabled: false,
        ..NumberBox::default()
    }
    .into();
    let bare: Element = NumberBox::default().into();
    assert_unsets(
        "NumberBox",
        full,
        bare,
        &[Prop::Minimum, Prop::Maximum, Prop::Header, Prop::IsEnabled],
    );
}

#[test]
fn progress_bar_no_optional_unset() {
    // ProgressBar always emits all props (value, min, max, isIndeterminate).
    // None are truly optional in the bindings sense.
    let full: Element = ProgressBar {
        value: 50.0,
        maximum: 200.0,
        is_indeterminate: true,
        ..ProgressBar::default()
    }
    .into();
    let bare: Element = ProgressBar::default().into();
    let unsets = unset_props_on_transition(full, bare);
    assert!(
        unsets.is_empty(),
        "ProgressBar: unexpected Unsets: {unsets:?}"
    );
}

#[test]
fn progress_ring_no_optional_unset() {
    // ProgressRing always emits all props (like ProgressBar).
    let full: Element = ProgressRing {
        value: 75.0,
        maximum: 150.0,
        is_active: false,
        is_indeterminate: true,
        ..ProgressRing::default()
    }
    .into();
    let bare: Element = ProgressRing::default().into();
    let unsets = unset_props_on_transition(full, bare);
    assert!(
        unsets.is_empty(),
        "ProgressRing: unexpected Unsets: {unsets:?}"
    );
}

#[test]
fn radio_button_optional_props_unset() {
    let full: Element = RadioButton {
        label: Some("opt1".into()),
        is_checked: true,
        is_enabled: false,
        group_name: Some("grp".into()),
        ..RadioButton::default()
    }
    .into();
    let bare: Element = RadioButton::new("opt1").into();
    assert_unsets(
        "RadioButton",
        full,
        bare,
        &[Prop::GroupName, Prop::IsEnabled],
    );
}

#[test]
fn hyperlink_button_optional_props_unset() {
    let full: Element = HyperlinkButton {
        label: "link".into(),
        navigate_uri: Some("https://example.com".into()),
        is_enabled: false,
        ..HyperlinkButton::default()
    }
    .into();
    let bare: Element = HyperlinkButton::new("link").into();
    assert_unsets(
        "HyperlinkButton",
        full,
        bare,
        &[Prop::NavigateUri, Prop::IsEnabled],
    );
}

#[test]
fn expander_header_unset() {
    let full: Element = Expander {
        header: Some(ExpanderHeader::Text("Details".into())),
        is_expanded: true,
        ..Expander::default()
    }
    .into();
    // bare: no header (is_expanded is always emitted, just value change)
    let bare: Element = Expander {
        is_expanded: true,
        ..Expander::default()
    }
    .into();
    assert_unsets("Expander", full, bare, &[Prop::Header]);
}

#[test]
fn info_bar_optional_props_unset() {
    let full: Element = InfoBar {
        title: Some("Alert".into()),
        message: Some("Something happened".into()),
        is_open: true,
        ..InfoBar::default()
    }
    .into();
    let bare: Element = InfoBar::default().into();
    assert_unsets(
        "InfoBar",
        full,
        bare,
        &[Prop::InfoBarTitle, Prop::InfoBarMessage],
    );
}

#[test]
fn info_badge_always_emits_value() {
    // InfoBadge always emits InfoBadgeValue (even -1 default).
    // Changing from Some(5) → None just changes value, no Unset.
    // This test verifies the widget mounts successfully.
    let full: Element = InfoBadge {
        value: Some(5),
        ..InfoBadge::default()
    }
    .into();
    let bare: Element = InfoBadge::default().into();
    let unsets = unset_props_on_transition(full, bare);
    assert!(
        unsets.is_empty(),
        "InfoBadge: unexpected Unsets: {unsets:?}"
    );
}

#[test]
fn password_box_optional_props_unset() {
    let full: Element = PasswordBox {
        value: "secret".into(),
        placeholder: Some("enter password".into()),
        header: Some("Password".into()),
        is_enabled: false,
        ..PasswordBox::default()
    }
    .into();
    let bare: Element = PasswordBox::new().value("secret").into();
    assert_unsets(
        "PasswordBox",
        full,
        bare,
        &[Prop::Placeholder, Prop::Header, Prop::IsEnabled],
    );
}

#[test]
fn combo_box_optional_props_unset() {
    let full: Element = ComboBox {
        items: vec!["a".into(), "b".into()],
        selected_index: 0,
        header: Some("Pick".into()),
        placeholder: Some("select...".into()),
        is_enabled: false,
        ..ComboBox::default()
    }
    .into();
    let bare: Element = ComboBox::new::<[&str; 0], &str>([]).into();
    assert_unsets(
        "ComboBox",
        full,
        bare,
        &[Prop::Header, Prop::Placeholder, Prop::IsEnabled],
    );
}

#[test]
fn rating_control_optional_props_unset() {
    let full: Element = RatingControl {
        value: 3.5,
        max_rating: Some(10),
        is_read_only: true,
        caption: Some("Rate this".into()),
        ..RatingControl::default()
    }
    .into();
    let bare: Element = RatingControl::default().into();
    assert_unsets(
        "RatingControl",
        full,
        bare,
        &[Prop::MaxRating, Prop::IsReadOnly, Prop::RatingCaption],
    );
}
