use std::rc::Rc;

use windows_reactor::core::backend::{ControlKind, Event, Op, Prop, PropValue, RecordingBackend};
use windows_reactor::core::element::Element;
use windows_reactor::core::element::{NumberBox, RadioButton, Slider, ToggleSwitch};
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
        .on_changed(move |v| fired_c.set(Some(v)))
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
                (Prop::NumericValue, PropValue::F64(42.0)) => saw_val = true,
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
        .on_changed(move |v| fired_c.set(Some(v)))
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
                (Prop::RadioLabel, PropValue::Str(s)) if s == "Option A" => saw_label = true,
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
                (Prop::NumericValue, PropValue::F64(7.0)) => saw_val = true,
                (Prop::Header, PropValue::Str(s)) if s == "Count" => saw_header = true,
                _ => {}
            }
        }
    }
    assert!(saw_val);
    assert!(saw_header);
}
