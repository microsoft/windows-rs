use std::rc::Rc;

use windows_reactor::core::backend::{ControlKind, Event, Op, Prop, PropValue, RecordingBackend};
use windows_reactor::core::element::Element;
use windows_reactor::core::element::{
    CheckBox, NumberBox, RadioButton, RatingControl, RepeatButton, RichEditBox, Slider, TextBox,
    ToggleButton, ToggleSwitch,
};
use windows_reactor::core::reconciler::Reconciler;

fn mount(el: &Element) -> Reconciler<RecordingBackend> {
    let mut r = Reconciler::new(RecordingBackend::new());
    r.reconcile(None, el, None, Rc::new(|| {}));
    r
}

fn first_create(
    r: &Reconciler<RecordingBackend>,
) -> (ControlKind, windows_reactor::core::backend::ControlId) {
    r.backend
        .ops
        .iter()
        .find_map(|op| match op {
            Op::Create { id, kind } => Some((*kind, *id)),
            _ => None,
        })
        .expect("no Create op")
}

#[test]
fn toggle_switch_mounts_with_is_on_and_content() {
    let el: Element = ToggleSwitch::new(true)
        .on_content("On")
        .off_content("Off")
        .header("Notifications")
        .into();
    let r = mount(&el);
    let (kind, id) = first_create(&r);
    assert_eq!(kind, ControlKind::ToggleSwitch);

    let got_is_on = r.backend.ops.iter().any(|op| {
        matches!(op, Op::SetProp { id: oid, prop: Prop::IsOn, value: PropValue::Bool(true) } if *oid == id)
    });
    assert!(got_is_on, "IsOn not set");

    let header_set = r.backend.ops.iter().any(|op| {
        matches!(op, Op::SetProp { prop: Prop::Header, value: PropValue::Str(s), .. } if s == "Notifications")
    });
    assert!(header_set, "Header not set");
}

#[test]
fn toggle_switch_attaches_toggled_event() {
    use std::cell::Cell;

    let fired = Rc::new(Cell::new(None::<bool>));
    let fired_c = Rc::clone(&fired);
    let el: Element = ToggleSwitch::new(false)
        .on_toggled(move |v| fired_c.set(Some(v)))
        .into();
    let r = mount(&el);

    let (_, id) = first_create(&r);
    r.backend.fire_bool(id, Event::Toggled, true);
    assert_eq!(fired.get(), Some(true));
}

#[test]
fn toggle_switch_update_diffs_is_on() {
    let old: Element = ToggleSwitch::new(false).into();
    let new: Element = ToggleSwitch::new(true).into();

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, &old, None, Rc::new(|| {})).unwrap();
    r.backend.clear_ops();
    r.reconcile(Some(&old), &new, Some(id), Rc::new(|| {}));

    let changes: Vec<&Op> = r
        .backend
        .ops
        .iter()
        .filter(|op| {
            matches!(
                op,
                Op::SetProp {
                    prop: Prop::IsOn,
                    ..
                }
            )
        })
        .collect();
    assert_eq!(changes.len(), 1, "expected one IsOn set on update");
}

#[test]
fn slider_mounts_with_range_and_value() {
    let el: Element = Slider::new(42.0).range(0.0, 100.0).step(2.0).into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::Slider);

    let mut saw_min = false;
    let mut saw_max = false;
    let mut saw_val = false;
    let mut saw_step = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::Minimum, PropValue::F64(0.0)) => saw_min = true,
                (Prop::Maximum, PropValue::F64(100.0)) => saw_max = true,
                (Prop::Value, PropValue::F64(42.0)) => saw_val = true,
                (Prop::Step, PropValue::F64(2.0)) => saw_step = true,
                _ => {}
            }
        }
    }
    assert!(saw_min && saw_max && saw_val && saw_step);
}

#[test]
fn slider_value_changed_event_routes_through_fire_f64() {
    use std::cell::Cell;

    let fired = Rc::new(Cell::new(None::<f64>));
    let fired_c = Rc::clone(&fired);
    let el: Element = Slider::new(0.0)
        .on_value_changed(move |v| fired_c.set(Some(v)))
        .into();
    let r = mount(&el);

    let (_, id) = first_create(&r);
    r.backend.fire_f64(id, Event::ValueChanged, 17.5);
    assert_eq!(fired.get(), Some(17.5));
}

#[test]
fn radio_button_mounts_with_group_and_label() {
    let el: Element = RadioButton::new("Option A")
        .group("my-group")
        .checked(true)
        .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::RadioButton);

    let mut saw_label = false;
    let mut saw_group = false;
    let mut saw_checked = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::Content, PropValue::Str(s)) if s == "Option A" => saw_label = true,
                (Prop::GroupName, PropValue::Str(s)) if s == "my-group" => saw_group = true,
                (Prop::IsChecked, PropValue::Bool(true)) => saw_checked = true,
                _ => {}
            }
        }
    }
    assert!(saw_label);
    assert!(saw_group);
    assert!(saw_checked);
}

#[test]
fn number_box_mounts_with_value_and_range() {
    let el: Element = NumberBox::new(7.0).range(0.0, 10.0).header("Count").into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::NumberBox);

    let mut saw_val = false;
    let mut saw_header = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::Value, PropValue::F64(7.0)) => saw_val = true,
                (Prop::Header, PropValue::Str(s)) if s == "Count" => saw_header = true,
                _ => {}
            }
        }
    }
    assert!(saw_val);
    assert!(saw_header);
}

#[test]
fn check_box_mounts_with_checked_content_and_disabled() {
    let el: Element = CheckBox::new(true).content("Accept").enabled(false).into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::CheckBox);

    let mut saw_checked = false;
    let mut saw_content = false;
    let mut saw_enabled = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::IsChecked, PropValue::Bool(true)) => saw_checked = true,
                (Prop::Content, PropValue::Str(s)) if s == "Accept" => saw_content = true,
                (Prop::IsEnabled, PropValue::Bool(false)) => saw_enabled = true,
                _ => {}
            }
        }
    }
    assert!(saw_checked && saw_content && saw_enabled);
}

#[test]
fn check_box_checked_event_routes_bool_value() {
    use std::cell::Cell;

    let fired = Rc::new(Cell::new(None::<bool>));
    let fired_c = Rc::clone(&fired);
    let el: Element = CheckBox::new(false)
        .on_checked(move |v| fired_c.set(Some(v)))
        .into();
    let r = mount(&el);

    let (_, id) = first_create(&r);
    r.backend.fire_bool(id, Event::Checked, true);
    assert_eq!(fired.get(), Some(true));
}

#[test]
fn text_box_mounts_with_value_placeholder_header_and_accepts_return() {
    let el: Element = TextBox::new("initial")
        .placeholder_text("Type...")
        .header("Name")
        .accepts_return(true)
        .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::TextBox);

    let mut saw_value = false;
    let mut saw_placeholder = false;
    let mut saw_header = false;
    let mut saw_accepts_return = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::Value, PropValue::Str(s)) if s == "initial" => saw_value = true,
                (Prop::PlaceholderText, PropValue::Str(s)) if s == "Type..." => {
                    saw_placeholder = true;
                }
                (Prop::Header, PropValue::Str(s)) if s == "Name" => saw_header = true,
                (Prop::AcceptsReturn, PropValue::Bool(true)) => saw_accepts_return = true,
                _ => {}
            }
        }
    }
    assert!(saw_value && saw_placeholder && saw_header && saw_accepts_return);
}

#[test]
fn text_box_text_changed_event_routes_string_value() {
    let fired = Rc::new(std::cell::RefCell::new(None::<String>));
    let fired_c = Rc::clone(&fired);
    let el: Element = TextBox::new("")
        .on_text_changed(move |s| *fired_c.borrow_mut() = Some(s))
        .into();
    let r = mount(&el);

    let (_, id) = first_create(&r);
    r.backend
        .fire_string(id, Event::TextChanged, "new".to_string());
    assert_eq!(&*fired.borrow(), &Some("new".to_string()));
}

#[test]
fn toggle_button_mounts_with_content_checked_and_disabled() {
    let el: Element = ToggleButton::new("Bold", true).enabled(false).into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::ToggleButton);

    let mut saw_content = false;
    let mut saw_checked = false;
    let mut saw_enabled = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::Content, PropValue::Str(s)) if s == "Bold" => saw_content = true,
                (Prop::IsChecked, PropValue::Bool(true)) => saw_checked = true,
                (Prop::IsEnabled, PropValue::Bool(false)) => saw_enabled = true,
                _ => {}
            }
        }
    }
    assert!(saw_content && saw_checked && saw_enabled);
}

#[test]
fn toggle_button_checked_event_routes_bool_value() {
    use std::cell::Cell;

    let fired = Rc::new(Cell::new(None::<bool>));
    let fired_c = Rc::clone(&fired);
    let el: Element = ToggleButton::new("Bold", true)
        .on_checked(move |v| fired_c.set(Some(v)))
        .into();
    let r = mount(&el);

    let (_, id) = first_create(&r);
    r.backend.fire_bool(id, Event::Checked, false);
    assert_eq!(fired.get(), Some(false));
}

#[test]
fn rating_control_mounts_with_value_max_rating_and_caption() {
    let el: Element = RatingControl::new(3.5)
        .max_rating(10)
        .caption("Rate")
        .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::RatingControl);

    let mut saw_value = false;
    let mut saw_max = false;
    let mut saw_caption = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::Value, PropValue::F64(v)) if (*v - 3.5).abs() < f64::EPSILON => {
                    saw_value = true;
                }
                (Prop::MaxRating, PropValue::I32(10)) => saw_max = true,
                (Prop::Caption, PropValue::Str(s)) if s == "Rate" => saw_caption = true,
                _ => {}
            }
        }
    }
    assert!(saw_value && saw_max && saw_caption);
}

#[test]
fn repeat_button_mounts_with_content_delay_and_interval() {
    let el: Element = RepeatButton::new("Hold").delay(500).interval(100).into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::RepeatButton);

    let mut saw_content = false;
    let mut saw_delay = false;
    let mut saw_interval = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::Content, PropValue::Str(s)) if s == "Hold" => saw_content = true,
                (Prop::Delay, PropValue::I32(500)) => saw_delay = true,
                (Prop::Interval, PropValue::I32(100)) => saw_interval = true,
                _ => {}
            }
        }
    }
    assert!(saw_content && saw_delay && saw_interval);
}

#[test]
fn repeat_button_click_event_fires() {
    use std::cell::Cell;

    let fired = Rc::new(Cell::new(0));
    let fired_c = Rc::clone(&fired);
    let el: Element = RepeatButton::new("Hold")
        .on_click(move || fired_c.set(fired_c.get() + 1))
        .into();
    let r = mount(&el);

    let (_, id) = first_create(&r);
    r.backend.fire(id, Event::Click);
    assert_eq!(fired.get(), 1);
}

#[test]
fn rich_edit_box_mounts_with_text_placeholder_header_and_read_only() {
    let el: Element = RichEditBox::new("text")
        .placeholder_text("Enter...")
        .header("Doc")
        .read_only()
        .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::RichEditBox);

    let mut saw_text = false;
    let mut saw_placeholder = false;
    let mut saw_header = false;
    let mut saw_read_only = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::Text, PropValue::Str(s)) if s == "text" => saw_text = true,
                (Prop::PlaceholderText, PropValue::Str(s)) if s == "Enter..." => {
                    saw_placeholder = true;
                }
                (Prop::Header, PropValue::Str(s)) if s == "Doc" => saw_header = true,
                (Prop::IsReadOnly, PropValue::Bool(true)) => saw_read_only = true,
                _ => {}
            }
        }
    }
    assert!(saw_text && saw_placeholder && saw_header && saw_read_only);
}
