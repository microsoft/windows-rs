use std::cell::Cell;
use std::rc::Rc;

use windows_reactor::core::backend::{
    Backend, ControlId, ControlKind, Event, EventHandler, Op, Prop, PropValue, RecordingBackend,
};

#[test]
fn recording_backend_assigns_sequential_ids() {
    let mut b = RecordingBackend::new();
    let a = b.create(ControlKind::TextBlock);
    let c = b.create(ControlKind::Button);
    assert_eq!(a.get(), 1);
    assert_eq!(c.get(), 2);
}

#[test]
fn set_prop_is_logged_verbatim() {
    let mut b = RecordingBackend::new();
    let id = b.create(ControlKind::TextBlock);
    b.set_prop(id, Prop::Text, &PropValue::Str("hi".into()));
    assert_eq!(b.ops.len(), 2);
    match &b.ops[1] {
        Op::SetProp {
            id: got,
            prop,
            value,
        } => {
            assert_eq!(*got, id);
            assert_eq!(*prop, Prop::Text);
            assert_eq!(*value, PropValue::Str("hi".into()));
        }
        other => panic!("expected SetProp, got {other:?}"),
    }
}

#[test]
fn append_remove_replace_child_maintains_structure() {
    let mut b = RecordingBackend::new();
    let parent = b.create(ControlKind::StackPanel);
    let a = b.create(ControlKind::TextBlock);
    let c = b.create(ControlKind::TextBlock);
    let d = b.create(ControlKind::TextBlock);

    b.append_child(parent, a);
    b.append_child(parent, c);
    assert_eq!(b.children_of(parent), &[a, c]);

    b.replace_child(parent, 0, d);
    assert_eq!(b.children_of(parent), &[d, c]);

    b.remove_child(parent, 1);
    assert_eq!(b.children_of(parent), &[d]);
}

#[test]
fn destroy_deletes_children_map_entry() {
    let mut b = RecordingBackend::new();
    let parent = b.create(ControlKind::StackPanel);
    let a = b.create(ControlKind::TextBlock);
    b.append_child(parent, a);
    assert_eq!(b.children_of(parent).len(), 1);
    b.destroy(parent);
    assert_eq!(b.children_of(parent).len(), 0);
}

#[test]
fn attached_event_handler_can_be_fired() {
    let mut b = RecordingBackend::new();
    let id = b.create(ControlKind::Button);
    let fired = Rc::new(Cell::new(0_i32));
    let fired_c = Rc::clone(&fired);
    b.attach_event(
        id,
        Event::Click,
        EventHandler::from_fn(move || fired_c.set(fired_c.get() + 1)),
    );
    b.fire(id, Event::Click);
    b.fire(id, Event::Click);
    assert_eq!(fired.get(), 2);
}

#[test]
fn live_control_count_reflects_create_destroy() {
    let mut b = RecordingBackend::new();
    let a = b.create(ControlKind::TextBlock);
    let _c = b.create(ControlKind::TextBlock);
    assert_eq!(b.live_control_count(), 2);
    b.destroy(a);
    assert_eq!(b.live_control_count(), 1);
}

#[test]
fn control_id_display_includes_hash_prefix() {
    let id = ControlId::new(7);
    assert_eq!(format!("{id}"), "#7");
}

#[test]
#[should_panic(expected = "ControlId must be non-zero")]
fn control_id_zero_panics() {
    let _ = ControlId::new(0);
}
