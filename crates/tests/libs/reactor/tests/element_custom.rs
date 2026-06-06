//! Tests for `CustomElement` — the open/closed extension hatch that lets
//! third-party code add widgets without modifying the core `Element` enum.
//!
//! See [`docs/roadmap.md`](../../../../docs/roadmap.md) item 1a-ii.
//!
//! These tests double as the reference implementation pattern for anyone
//! writing their own `CustomElement`. The recommended pattern:
//!
//! 1. Implement `as_any(&self) -> &dyn Any { self }`.
//! 2. Implement `eq_dyn` by downcasting via `other.as_any().downcast_ref::<Self>()`.
//! 3. Implement `update` the same way for prop diffing.
//! 4. Make the type `Clone` so `clone_dyn` is one line.

use std::any::Any;
use std::cell::Cell;
use std::rc::Rc;

use windows_reactor::core::backend::{
    Backend, ControlId, ControlKind, Op, Prop, PropValue, RecordingBackend,
};
use windows_reactor::core::custom::{CustomElement, CustomElementHandle};
use windows_reactor::core::element::{Element, Orientation, StackPanel, TextBlock};
use windows_reactor::core::reconciler::Reconciler;

fn noop() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

fn mount(el: Element) -> (Reconciler<RecordingBackend>, Option<ControlId>) {
    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, &el, None, noop());
    (r, id)
}

fn into_element<C: CustomElement>(c: C) -> Element {
    Element::Custom(CustomElementHandle::new(c))
}

// ---------------------------------------------------------------------------
// Test fixture: a `BadgeText` widget defined entirely outside the core crate.
//
// It composes a TextBlock control with a formatted "{label} ({count})" label.
// Lives in a sibling test crate so this also documents the externally-defined
// widget pattern.
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct BadgeText {
    key: Option<String>,
    label: String,
    count: i32,
    destroy_counter: Rc<Cell<u32>>,
}

impl BadgeText {
    fn new(label: &str, count: i32) -> Self {
        Self {
            key: None,
            label: label.into(),
            count,
            destroy_counter: Rc::new(Cell::new(0)),
        }
    }
    fn with_key(mut self, k: &str) -> Self {
        self.key = Some(k.into());
        self
    }
    fn rendered(&self) -> String {
        format!("{} ({})", self.label, self.count)
    }
}

impl CustomElement for BadgeText {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn kind_name(&self) -> &'static str {
        "BadgeText"
    }
    fn key(&self) -> Option<&str> {
        self.key.as_deref()
    }
    fn eq_dyn(&self, other: &dyn CustomElement) -> bool {
        other.as_any().downcast_ref::<BadgeText>().is_some_and(|o| {
            // destroy_counter is a test-fixture concern, not a logical
            // prop — exclude it from equality.
            self.key == o.key && self.label == o.label && self.count == o.count
        })
    }
    fn clone_dyn(&self) -> Box<dyn CustomElement> {
        Box::new(self.clone())
    }
    fn mount(&self, backend: &mut dyn Backend) -> ControlId {
        let id = backend.create(ControlKind::TextBlock);
        backend.set_prop(id, Prop::Text, &PropValue::Str(self.rendered()));
        id
    }
    fn update(&self, prev: &dyn CustomElement, id: ControlId, backend: &mut dyn Backend) {
        let prev = prev
            .as_any()
            .downcast_ref::<BadgeText>()
            .expect("reconciler guarantees prev has same type as self");
        if prev.rendered() != self.rendered() {
            backend.set_prop(id, Prop::Text, &PropValue::Str(self.rendered()));
        }
    }
    fn before_destroy(&self, _id: ControlId, _backend: &mut dyn Backend) {
        self.destroy_counter.set(self.destroy_counter.get() + 1);
    }
}

/// A second, distinct CustomElement type — used to verify that swapping
/// to a different `type_id` triggers a full unmount + remount.
#[derive(Clone)]
struct AltCustom;
impl CustomElement for AltCustom {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn kind_name(&self) -> &'static str {
        "AltCustom"
    }
    fn eq_dyn(&self, other: &dyn CustomElement) -> bool {
        other.as_any().is::<AltCustom>()
    }
    fn clone_dyn(&self) -> Box<dyn CustomElement> {
        Box::new(self.clone())
    }
    fn mount(&self, backend: &mut dyn Backend) -> ControlId {
        backend.create(ControlKind::Border)
    }
    fn update(&self, _prev: &dyn CustomElement, _id: ControlId, _backend: &mut dyn Backend) {}
}

#[derive(Clone)]
struct KeyedBadge {
    key: String,
    label: String,
    count: i32,
}

impl KeyedBadge {
    fn rendered(&self) -> String {
        format!("{} ({})", self.label, self.count)
    }
}

impl CustomElement for KeyedBadge {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn kind_name(&self) -> &'static str {
        "KeyedBadge"
    }
    fn key(&self) -> Option<&str> {
        Some(&self.key)
    }
    fn eq_dyn(&self, other: &dyn CustomElement) -> bool {
        other
            .as_any()
            .downcast_ref::<KeyedBadge>()
            .is_some_and(|o| self.key == o.key && self.label == o.label && self.count == o.count)
    }
    fn clone_dyn(&self) -> Box<dyn CustomElement> {
        Box::new(self.clone())
    }
    fn mount(&self, backend: &mut dyn Backend) -> ControlId {
        let id = backend.create(ControlKind::TextBlock);
        backend.set_prop(id, Prop::Text, &PropValue::Str(self.rendered()));
        id
    }
    fn update(&self, prev: &dyn CustomElement, id: ControlId, backend: &mut dyn Backend) {
        let prev = prev.as_any().downcast_ref::<KeyedBadge>().unwrap();
        if prev.rendered() != self.rendered() {
            backend.set_prop(id, Prop::Text, &PropValue::Str(self.rendered()));
        }
    }
}

fn keyed_badge(key: &str, label: &str, count: i32) -> Element {
    into_element(KeyedBadge {
        key: key.into(),
        label: label.into(),
        count,
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn custom_mount_records_create_and_initial_prop() {
    let (r, id) = mount(into_element(BadgeText::new("Inbox", 3)));
    let id = id.expect("Custom mount produces an id");
    let ops = &r.backend.ops;
    assert_eq!(
        ops.len(),
        2,
        "expect Create + SetProp(TextBlock), got {ops:?}"
    );
    match &ops[0] {
        Op::Create { id: got, kind } => {
            assert_eq!(*got, id);
            assert_eq!(*kind, ControlKind::TextBlock);
        }
        other => panic!("first op must be Create, got {other:?}"),
    }
    match &ops[1] {
        Op::SetProp {
            id: got,
            prop,
            value,
        } => {
            assert_eq!(*got, id);
            assert_eq!(*prop, Prop::Text);
            assert_eq!(*value, PropValue::Str("Inbox (3)".into()));
        }
        other => panic!("second op must be SetProp(TextBlock), got {other:?}"),
    }
}

#[test]
fn custom_update_with_unchanged_props_emits_zero_ops() {
    let initial = into_element(BadgeText::new("Inbox", 3));
    let next = into_element(BadgeText::new("Inbox", 3));

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, &initial, None, noop()).unwrap();
    let baseline = r.backend.ops.len();
    r.reconcile(Some(&initial), &next, Some(id), noop())
        .unwrap();
    let new_ops = &r.backend.ops[baseline..];
    assert!(
        new_ops.is_empty(),
        "eq_dyn-equal Custom update should emit zero ops; got {new_ops:?}"
    );
}

#[test]
fn custom_update_with_changed_props_emits_only_the_changed_prop() {
    let initial = into_element(BadgeText::new("Inbox", 3));
    let next = into_element(BadgeText::new("Inbox", 4));

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, &initial, None, noop()).unwrap();
    let baseline = r.backend.ops.len();
    r.reconcile(Some(&initial), &next, Some(id), noop())
        .unwrap();

    let new_ops: Vec<&Op> = r.backend.ops[baseline..].iter().collect();
    assert_eq!(
        new_ops.len(),
        1,
        "exactly one set_prop expected, got {new_ops:?}"
    );
    match new_ops[0] {
        Op::SetProp {
            prop: Prop::Text,
            value: PropValue::Str(s),
            ..
        } => {
            assert_eq!(s, "Inbox (4)");
        }
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn custom_type_change_unmounts_old_and_mounts_new() {
    let initial = into_element(BadgeText::new("Inbox", 3));
    let next = into_element(AltCustom);

    let mut r = Reconciler::new(RecordingBackend::new());
    let old_id = r.reconcile(None, &initial, None, noop()).unwrap();
    let baseline = r.backend.ops.len();

    let new_id = r
        .reconcile(Some(&initial), &next, Some(old_id), noop())
        .unwrap();

    assert_ne!(
        new_id, old_id,
        "different CustomElement types must produce a fresh id"
    );

    let new_ops = &r.backend.ops[baseline..];
    let destroyed = new_ops
        .iter()
        .any(|op| matches!(op, Op::Destroy { id } if *id == old_id));
    let created_alt = new_ops.iter().any(|op| {
        matches!(
            op,
            Op::Create { id, kind: ControlKind::Border } if *id == new_id
        )
    });
    assert!(destroyed, "old custom must be destroyed; ops: {new_ops:?}");
    assert!(
        created_alt,
        "new custom must be created with its own ControlKind; ops: {new_ops:?}"
    );
}

#[test]
fn custom_before_destroy_runs_exactly_once_on_unmount() {
    let badge = BadgeText::new("Inbox", 3);
    let counter = badge.destroy_counter.clone();
    let initial = into_element(badge);
    let replacement = into_element(AltCustom);

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, &initial, None, noop()).unwrap();
    assert_eq!(counter.get(), 0, "no destroy yet");

    // Type-change forces unmount of the BadgeText.
    r.reconcile(Some(&initial), &replacement, Some(id), noop())
        .unwrap();
    assert_eq!(counter.get(), 1, "before_destroy must fire exactly once");
}

#[test]
fn custom_before_destroy_uses_latest_handle_after_updates() {
    // After an update, the reconciler should retain the *new* handle, so
    // before_destroy fires on the latest version's destroy_counter.
    let initial_badge = BadgeText::new("Inbox", 3);
    let old_counter = initial_badge.destroy_counter.clone();
    let initial = into_element(initial_badge);

    let updated_badge = BadgeText::new("Inbox", 4);
    let new_counter = updated_badge.destroy_counter.clone();
    let updated = into_element(updated_badge);

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, &initial, None, noop()).unwrap();
    r.reconcile(Some(&initial), &updated, Some(id), noop())
        .unwrap();

    // Now swap to a different custom type → unmount.
    let replacement = into_element(AltCustom);
    r.reconcile(Some(&updated), &replacement, Some(id), noop())
        .unwrap();

    assert_eq!(
        new_counter.get(),
        1,
        "before_destroy should fire on the latest handle"
    );
    assert_eq!(
        old_counter.get(),
        0,
        "before_destroy should NOT fire on the pre-update handle"
    );
}

#[test]
fn custom_lives_alongside_built_in_widgets_inside_a_stack() {
    let stack = StackPanel {
        orientation: Orientation::Vertical,
        children: vec![
            Element::TextBlock(TextBlock::new("title")),
            into_element(BadgeText::new("Inbox", 5)),
            Element::TextBlock(TextBlock::new("after")),
        ],
        ..Default::default()
    };
    let mut r = Reconciler::new(RecordingBackend::new());
    let parent = r
        .reconcile(None, &Element::StackPanel(stack), None, noop())
        .unwrap();

    let kids = r.backend.children_of(parent).to_vec();
    assert_eq!(kids.len(), 3);
}

#[test]
fn custom_keyed_reorder_inside_stack_reuses_controls() {
    let initial = Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        children: vec![
            keyed_badge("a", "A", 1),
            keyed_badge("b", "B", 2),
            keyed_badge("c", "C", 3),
        ],
        ..Default::default()
    });
    let updated = Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        children: vec![
            keyed_badge("c", "C", 3),
            keyed_badge("a", "A", 1),
            keyed_badge("b", "B", 2),
        ],
        ..Default::default()
    });

    let mut r = Reconciler::new(RecordingBackend::new());
    let parent = r.reconcile(None, &initial, None, noop()).unwrap();
    let before_kids = r.backend.children_of(parent).to_vec();
    assert_eq!(before_kids.len(), 3);

    let baseline = r.backend.ops.len();
    r.reconcile(Some(&initial), &updated, Some(parent), noop())
        .unwrap();
    let after_kids = r.backend.children_of(parent).to_vec();

    let mut sorted_before = before_kids;
    sorted_before.sort_by_key(|c| c.get());
    let mut sorted_after = after_kids;
    sorted_after.sort_by_key(|c| c.get());
    assert_eq!(
        sorted_before, sorted_after,
        "keyed reorder must reuse the same Custom-backed controls"
    );

    let new_ops = &r.backend.ops[baseline..];
    let destroyed: Vec<_> = new_ops
        .iter()
        .filter(|o| matches!(o, Op::Destroy { .. }))
        .collect();
    assert!(
        destroyed.is_empty(),
        "no destroys during keyed reorder; got {destroyed:?}"
    );
    let created: Vec<_> = new_ops
        .iter()
        .filter(|o| matches!(o, Op::Create { .. }))
        .collect();
    assert!(
        created.is_empty(),
        "no creates during keyed reorder; got {created:?}"
    );
}

#[test]
fn custom_kind_name_is_user_supplied() {
    let e = into_element(BadgeText::new("x", 0));
    assert_eq!(e.kind_name(), "BadgeText");
}

#[test]
fn custom_key_is_user_supplied() {
    let e = into_element(BadgeText::new("x", 0).with_key("k1"));
    assert_eq!(e.key(), Some("k1"));
}

#[test]
fn custom_clone_dyn_produces_equal_handle() {
    let h1 = CustomElementHandle::new(BadgeText::new("x", 1));
    let h2 = h1.clone();
    assert_eq!(h1, h2, "cloned CustomElementHandle compares equal");
}

#[test]
fn custom_default_type_id_uses_any() {
    // Default type_id() impl should match Any::type_id of the concrete type.
    let badge = BadgeText::new("x", 0);
    let any_tid = (&badge as &dyn Any).type_id();
    assert_eq!((&badge as &dyn CustomElement).type_id(), any_tid);
}
