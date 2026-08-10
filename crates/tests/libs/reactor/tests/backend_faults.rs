use std::cell::Cell;
use std::panic::AssertUnwindSafe;
use std::rc::Rc;

use test_reactor::{BackendOperation, Op, RecordingBackend};
use windows_reactor::{
    Button, Component, Context, ControlKind, Element, Expander, KeyExt, Pivot, PivotItem,
    ProvideExt, Reconciler, RenderCx, SplitView, TabItem, TabView, component, error_boundary,
    swap_chain_panel, text_block, vstack,
};

fn rerender() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

fn mount_case(operation: BackendOperation) -> Element {
    match operation {
        BackendOperation::Create => text_block("create").into(),
        BackendOperation::SetProp => text_block("property").into(),
        BackendOperation::AttachEvent => Button::new("event").on_click(|| {}).into(),
        BackendOperation::AppendChild => vstack((text_block("child"),)).into(),
        BackendOperation::SetHeaderElement => Expander::new(text_block("body"))
            .header_content(text_block("header"))
            .into(),
        BackendOperation::SetPaneElement => SplitView::new(text_block("content"))
            .pane(text_block("pane"))
            .into(),
        _ => panic!("operation is not an ordinary mount failure point"),
    }
}

fn mount_operations() -> [BackendOperation; 6] {
    [
        BackendOperation::Create,
        BackendOperation::SetProp,
        BackendOperation::AttachEvent,
        BackendOperation::AppendChild,
        BackendOperation::SetHeaderElement,
        BackendOperation::SetPaneElement,
    ]
}

fn first_property_after_create(element: &Element, kind: ControlKind) -> usize {
    let mut reconciler = Reconciler::new(RecordingBackend::new());
    reconciler.reconcile(None, element, None, rerender());

    let mut properties = 0;
    let mut found = false;
    for operation in &reconciler.backend.ops {
        match operation {
            Op::Create {
                kind: created_kind, ..
            } if *created_kind == kind => found = true,
            Op::SetProp { .. } => {
                properties += 1;
                if found {
                    return properties;
                }
            }
            _ => {}
        }
    }
    panic!("{kind:?} was not followed by a property operation");
}

#[test]
fn failed_widget_mounts_roll_back_all_owned_state() {
    for operation in mount_operations() {
        let mut backend = RecordingBackend::new();
        backend.fail_next(operation);
        let mut reconciler = Reconciler::new(backend);
        let element = mount_case(operation);

        let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
            reconciler.reconcile(None, &element, None, rerender())
        }));

        assert!(result.is_err(), "{operation:?} did not fail");
        reconciler.assert_consistent();
        reconciler.backend.assert_consistent();
        assert_eq!(
            reconciler.backend.live_control_count(),
            0,
            "{operation:?} leaked a native control"
        );
    }
}

#[test]
fn error_boundaries_recover_after_failed_widget_mounts() {
    for operation in mount_operations() {
        let mut backend = RecordingBackend::new();
        backend.fail_next(operation);
        let mut reconciler = Reconciler::new(backend);
        let element = error_boundary(mount_case(operation), |_| text_block("fallback").into());

        assert!(
            reconciler
                .reconcile(None, &element, None, rerender())
                .is_some(),
            "{operation:?} did not mount the fallback"
        );
        reconciler.assert_consistent();
        reconciler.backend.assert_consistent();
        assert_eq!(
            reconciler.backend.live_control_count(),
            1,
            "{operation:?} retained part of the failed subtree"
        );

        reconciler.unmount_root();
        reconciler.backend.assert_consistent();
        assert_eq!(reconciler.backend.live_control_count(), 0);
    }
}

#[test]
fn failed_tab_and_pivot_item_mounts_roll_back_direct_native_children() {
    let cases = [
        (
            TabView::new([TabItem::new("tab", text_block("content"))]).into(),
            ControlKind::TabViewItem,
        ),
        (
            Pivot::new([PivotItem::new("item", text_block("content"))]).into(),
            ControlKind::PivotItem,
        ),
    ];

    for (element, kind) in cases {
        let occurrence = first_property_after_create(&element, kind);
        let mut backend = RecordingBackend::new();
        backend.fail_on(BackendOperation::SetProp, occurrence);
        let mut reconciler = Reconciler::new(backend);

        let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
            reconciler.reconcile(None, &element, None, rerender())
        }));

        assert!(result.is_err(), "{kind:?} property did not fail");
        reconciler.assert_consistent();
        reconciler.backend.assert_consistent();
        assert_eq!(
            reconciler.backend.live_control_count(),
            0,
            "{kind:?} leaked a native control"
        );
    }
}

struct PendingEffect {
    setups: Rc<Cell<u32>>,
    cleanups: Rc<Cell<u32>>,
}

impl Component for PendingEffect {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let setups = Rc::clone(&self.setups);
        let cleanups = Rc::clone(&self.cleanups);
        cx.use_effect_with_cleanup((), move || {
            setups.set(setups.get() + 1);
            Some(move || cleanups.set(cleanups.get() + 1))
        });
        text_block("pending effect").into()
    }
}

#[test]
fn failed_component_mount_cleans_up_effects_before_fallback() {
    let setups = Rc::new(Cell::new(0));
    let cleanups = Rc::new(Cell::new(0));
    let component = component(
        PendingEffect {
            setups: Rc::clone(&setups),
            cleanups: Rc::clone(&cleanups),
        },
        (),
    );
    let element = error_boundary(component, |_| text_block("fallback").into());
    let mut backend = RecordingBackend::new();
    backend.fail_next(BackendOperation::SetProp);
    let mut reconciler = Reconciler::new(backend);

    assert!(
        reconciler
            .reconcile(None, &element, None, rerender())
            .is_some()
    );

    assert_eq!(setups.get(), 1);
    assert_eq!(cleanups.get(), 1);
    reconciler.assert_consistent();
    reconciler.backend.assert_consistent();
    assert_eq!(reconciler.backend.live_control_count(), 1);
}

#[derive(Clone, PartialEq)]
struct UpdateProps {
    text: &'static str,
    handler: bool,
    revision: u8,
}

struct UpdateEffect {
    setups: Rc<Cell<u32>>,
    cleanups: Rc<Cell<u32>>,
}

impl Component<UpdateProps> for UpdateEffect {
    fn render(&self, props: &UpdateProps, cx: &mut RenderCx) -> Element {
        let setups = Rc::clone(&self.setups);
        let cleanups = Rc::clone(&self.cleanups);
        cx.use_effect_with_cleanup((), move || {
            setups.set(setups.get() + 1);
            Some(move || cleanups.set(cleanups.get() + 1))
        });
        let button = Button::new(props.text);
        if props.handler {
            button.on_click(|| {}).into()
        } else {
            button.into()
        }
    }
}

fn update_component(
    props: UpdateProps,
    setups: &Rc<Cell<u32>>,
    cleanups: &Rc<Cell<u32>>,
) -> Element {
    component(
        UpdateEffect {
            setups: Rc::clone(setups),
            cleanups: Rc::clone(cleanups),
        },
        props,
    )
}

#[test]
fn error_boundary_discards_failed_component_updates_and_runs_cleanup() {
    let cases = [
        (
            BackendOperation::SetProp,
            UpdateProps {
                text: "old",
                handler: false,
                revision: 0,
            },
            UpdateProps {
                text: "new",
                handler: false,
                revision: 1,
            },
        ),
        (
            BackendOperation::AttachEvent,
            UpdateProps {
                text: "button",
                handler: true,
                revision: 0,
            },
            UpdateProps {
                text: "button",
                handler: true,
                revision: 1,
            },
        ),
        (
            BackendOperation::DetachEvent,
            UpdateProps {
                text: "button",
                handler: true,
                revision: 0,
            },
            UpdateProps {
                text: "button",
                handler: false,
                revision: 1,
            },
        ),
    ];

    for (operation, old_props, new_props) in cases {
        let setups = Rc::new(Cell::new(0));
        let cleanups = Rc::new(Cell::new(0));
        let old = error_boundary(update_component(old_props, &setups, &cleanups), |_| {
            text_block("fallback").into()
        });
        let new = error_boundary(update_component(new_props, &setups, &cleanups), |_| {
            text_block("fallback").into()
        });
        let mut reconciler = Reconciler::new(RecordingBackend::new());
        reconciler.reconcile(None, &old, None, rerender());
        reconciler.backend.fail_next(operation);

        assert!(
            reconciler
                .reconcile(Some(&old), &new, None, rerender())
                .is_some(),
            "{operation:?} did not mount the fallback"
        );

        assert_eq!(setups.get(), 1, "{operation:?} reran the stable effect");
        assert_eq!(
            cleanups.get(),
            1,
            "{operation:?} did not run cleanup for the discarded component"
        );
        assert_eq!(reconciler.debug_logical_node_count(), 1);
        reconciler.assert_consistent();
        reconciler.backend.assert_consistent();

        reconciler.unmount_root();
        assert_eq!(cleanups.get(), 1);
        reconciler.backend.assert_consistent();
        assert_eq!(reconciler.backend.live_control_count(), 0);
    }
}

#[test]
fn failed_component_update_retains_ownership_for_teardown() {
    let setups = Rc::new(Cell::new(0));
    let cleanups = Rc::new(Cell::new(0));
    let old = update_component(
        UpdateProps {
            text: "old",
            handler: false,
            revision: 0,
        },
        &setups,
        &cleanups,
    );
    let new = update_component(
        UpdateProps {
            text: "new",
            handler: false,
            revision: 1,
        },
        &setups,
        &cleanups,
    );
    let mut reconciler = Reconciler::new(RecordingBackend::new());
    reconciler.reconcile(None, &old, None, rerender());
    reconciler.backend.fail_next(BackendOperation::SetProp);

    let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
        reconciler.reconcile(Some(&old), &new, None, rerender())
    }));

    assert!(result.is_err());
    reconciler.assert_consistent();
    reconciler.backend.assert_consistent();
    reconciler.unmount_root();
    assert_eq!(setups.get(), 1);
    assert_eq!(cleanups.get(), 1);
    reconciler.backend.assert_consistent();
    assert_eq!(reconciler.backend.live_control_count(), 0);
}

#[test]
fn failed_provider_update_retains_ownership_for_teardown() {
    let context = Context::new(0_u32);
    let old = text_block("old").provide(&context, 1);
    let new = text_block("new").provide(&context, 2);
    let mut reconciler = Reconciler::new(RecordingBackend::new());
    reconciler.reconcile(None, &old, None, rerender());
    reconciler.backend.fail_next(BackendOperation::SetProp);

    let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
        reconciler.reconcile(Some(&old), &new, None, rerender())
    }));

    assert!(result.is_err());
    reconciler.assert_consistent();
    reconciler.backend.assert_consistent();
    reconciler.unmount_root();
    reconciler.backend.assert_consistent();
    assert_eq!(reconciler.backend.live_control_count(), 0);
}

#[test]
fn error_boundary_discards_failed_root_replacement() {
    let old = error_boundary(text_block("old"), |_| text_block("fallback").into());
    let new = error_boundary(Button::new("new"), |_| text_block("fallback").into());
    let mut reconciler = Reconciler::new(RecordingBackend::new());
    reconciler.reconcile(None, &old, None, rerender());
    reconciler.backend.fail_next(BackendOperation::Create);

    assert!(
        reconciler
            .reconcile(Some(&old), &new, None, rerender())
            .is_some()
    );

    assert_eq!(reconciler.backend.live_control_count(), 1);
    assert_eq!(reconciler.debug_logical_node_count(), 1);
    reconciler.assert_consistent();
    reconciler.backend.assert_consistent();
    reconciler.unmount_root();
    reconciler.backend.assert_consistent();
    assert_eq!(reconciler.backend.live_control_count(), 0);
}

#[test]
fn error_boundary_discards_failed_nested_child_replacement() {
    let old = error_boundary(vstack((text_block("old"),)), |_| {
        text_block("fallback").into()
    });
    let new = error_boundary(vstack((Button::new("new"),)), |_| {
        text_block("fallback").into()
    });
    let mut reconciler = Reconciler::new(RecordingBackend::new());
    reconciler.reconcile(None, &old, None, rerender());
    reconciler.backend.fail_next(BackendOperation::Create);

    assert!(
        reconciler
            .reconcile(Some(&old), &new, None, rerender())
            .is_some()
    );

    assert_eq!(reconciler.backend.live_control_count(), 1);
    assert_eq!(reconciler.debug_logical_node_count(), 1);
    reconciler.assert_consistent();
    reconciler.backend.assert_consistent();
    reconciler.unmount_root();
    reconciler.backend.assert_consistent();
    assert_eq!(reconciler.backend.live_control_count(), 0);
}

#[derive(Clone, PartialEq)]
struct ReplacementProps(bool);

struct ReplacementEffect {
    cleanups: Rc<Cell<u32>>,
}

impl Component<ReplacementProps> for ReplacementEffect {
    fn render(&self, props: &ReplacementProps, cx: &mut RenderCx) -> Element {
        let cleanups = Rc::clone(&self.cleanups);
        cx.use_effect_with_cleanup((), move || Some(move || cleanups.set(cleanups.get() + 1)));
        if props.0 {
            Button::new("new").into()
        } else {
            text_block("old").into()
        }
    }
}

#[test]
fn error_boundary_cleans_component_after_failed_output_replacement() {
    let cleanups = Rc::new(Cell::new(0));
    let old = error_boundary(
        component(
            ReplacementEffect {
                cleanups: Rc::clone(&cleanups),
            },
            ReplacementProps(false),
        ),
        |_| text_block("fallback").into(),
    );
    let new = error_boundary(
        component(
            ReplacementEffect {
                cleanups: Rc::clone(&cleanups),
            },
            ReplacementProps(true),
        ),
        |_| text_block("fallback").into(),
    );
    let mut reconciler = Reconciler::new(RecordingBackend::new());
    reconciler.reconcile(None, &old, None, rerender());
    reconciler.backend.fail_next(BackendOperation::Create);

    assert!(
        reconciler
            .reconcile(Some(&old), &new, None, rerender())
            .is_some()
    );

    assert_eq!(cleanups.get(), 1);
    assert_eq!(reconciler.backend.live_control_count(), 1);
    assert_eq!(reconciler.debug_logical_node_count(), 1);
    reconciler.assert_consistent();
    reconciler.backend.assert_consistent();
    reconciler.unmount_root();
    assert_eq!(cleanups.get(), 1);
    reconciler.backend.assert_consistent();
    assert_eq!(reconciler.backend.live_control_count(), 0);
}

#[test]
fn strict_unmount_still_rejects_an_already_removed_control() {
    let element: Element = text_block("strict").into();
    let mut reconciler = Reconciler::new(RecordingBackend::new());
    let id = reconciler
        .reconcile(None, &element, None, rerender())
        .unwrap();
    reconciler.unmount(id);

    let result = std::panic::catch_unwind(AssertUnwindSafe(|| reconciler.unmount(id)));

    assert!(result.is_err());
    reconciler.backend.assert_consistent();
    assert_eq!(reconciler.backend.live_control_count(), 0);
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum CollectionChange {
    Append,
    Insert,
    Replace,
    Move,
    Remove,
}

impl CollectionChange {
    fn backend_operation(self) -> BackendOperation {
        match self {
            Self::Append => BackendOperation::AppendChild,
            Self::Insert => BackendOperation::InsertChild,
            Self::Replace => BackendOperation::ReplaceChild,
            Self::Move => BackendOperation::MoveChild,
            Self::Remove => BackendOperation::RemoveChild,
        }
    }
}

#[derive(Clone, PartialEq)]
struct CollectionProps {
    change: CollectionChange,
    updated: bool,
}

struct CollectionEffect {
    cleanups: Rc<Cell<u32>>,
}

impl Component<CollectionProps> for CollectionEffect {
    fn render(&self, props: &CollectionProps, cx: &mut RenderCx) -> Element {
        let cleanups = Rc::clone(&self.cleanups);
        cx.use_effect_with_cleanup((), move || Some(move || cleanups.set(cleanups.get() + 1)));

        let children: Vec<Element> = match (props.change, props.updated) {
            (CollectionChange::Append, false) => vec![text_block("a").into()],
            (CollectionChange::Append, true) => {
                vec![text_block("a").into(), text_block("b").into()]
            }
            (CollectionChange::Insert, false) => {
                vec![
                    text_block("a").with_key("a").into(),
                    text_block("c").with_key("c").into(),
                ]
            }
            (CollectionChange::Insert, true) => vec![
                text_block("a").with_key("a").into(),
                text_block("b").with_key("b").into(),
                text_block("c").with_key("c").into(),
            ],
            (CollectionChange::Replace, false) => vec![text_block("old").into()],
            (CollectionChange::Replace, true) => vec![Button::new("new").into()],
            (CollectionChange::Move, false) => vec![
                text_block("a").with_key("a").into(),
                text_block("b").with_key("b").into(),
                text_block("c").with_key("c").into(),
            ],
            (CollectionChange::Move, true) => vec![
                text_block("c").with_key("c").into(),
                text_block("a").with_key("a").into(),
                text_block("b").with_key("b").into(),
            ],
            (CollectionChange::Remove, false) => {
                vec![text_block("a").into(), text_block("b").into()]
            }
            (CollectionChange::Remove, true) => vec![text_block("a").into()],
        };
        vstack(children).into()
    }
}

fn collection_component(
    change: CollectionChange,
    updated: bool,
    cleanups: &Rc<Cell<u32>>,
) -> Element {
    component(
        CollectionEffect {
            cleanups: Rc::clone(cleanups),
        },
        CollectionProps { change, updated },
    )
}

fn collection_changes() -> [CollectionChange; 5] {
    [
        CollectionChange::Append,
        CollectionChange::Insert,
        CollectionChange::Replace,
        CollectionChange::Move,
        CollectionChange::Remove,
    ]
}

#[test]
fn error_boundaries_discard_failed_child_collection_updates() {
    for change in collection_changes() {
        let cleanups = Rc::new(Cell::new(0));
        let old = error_boundary(collection_component(change, false, &cleanups), |_| {
            text_block("fallback").into()
        });
        let new = error_boundary(collection_component(change, true, &cleanups), |_| {
            text_block("fallback").into()
        });
        let mut reconciler = Reconciler::new(RecordingBackend::new());
        reconciler.reconcile(None, &old, None, rerender());
        reconciler.backend.fail_next(change.backend_operation());

        assert!(
            reconciler
                .reconcile(Some(&old), &new, None, rerender())
                .is_some(),
            "{change:?} did not mount the fallback"
        );

        assert_eq!(
            cleanups.get(),
            1,
            "{change:?} did not run cleanup for the discarded component"
        );
        assert_eq!(reconciler.backend.live_control_count(), 1);
        assert_eq!(reconciler.debug_logical_node_count(), 1);
        reconciler.assert_consistent();
        reconciler.backend.assert_consistent();

        reconciler.unmount_root();
        assert_eq!(cleanups.get(), 1);
        reconciler.backend.assert_consistent();
        assert_eq!(reconciler.backend.live_control_count(), 0);
    }
}

#[test]
fn failed_child_collection_updates_remain_reachable_for_teardown() {
    for change in collection_changes() {
        let cleanups = Rc::new(Cell::new(0));
        let old = collection_component(change, false, &cleanups);
        let new = collection_component(change, true, &cleanups);
        let mut reconciler = Reconciler::new(RecordingBackend::new());
        reconciler.reconcile(None, &old, None, rerender());
        reconciler.backend.fail_next(change.backend_operation());

        let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
            reconciler.reconcile(Some(&old), &new, None, rerender())
        }));

        assert!(result.is_err(), "{change:?} did not fail");
        reconciler.unmount_root();
        assert_eq!(
            cleanups.get(),
            1,
            "{change:?} did not run cleanup for the retained component"
        );
        reconciler.assert_consistent();
        reconciler.backend.assert_consistent();
        assert_eq!(reconciler.backend.live_control_count(), 0);
    }
}

#[derive(Clone, PartialEq)]
struct DestroyProps(bool);

struct DestroyEffect {
    cleanups: Rc<Cell<u32>>,
    unmounted: Rc<Cell<u32>>,
}

impl Component<DestroyProps> for DestroyEffect {
    fn render(&self, props: &DestroyProps, cx: &mut RenderCx) -> Element {
        let cleanups = Rc::clone(&self.cleanups);
        cx.use_effect_with_cleanup((), move || Some(move || cleanups.set(cleanups.get() + 1)));

        if props.0 {
            return text_block("new").into();
        }

        let first = Rc::clone(&self.unmounted);
        let second = Rc::clone(&self.unmounted);
        vstack((
            swap_chain_panel().on_unmounted(move |_| first.set(first.get() + 1)),
            swap_chain_panel().on_unmounted(move |_| second.set(second.get() + 1)),
        ))
        .into()
    }
}

fn destroy_component(
    updated: bool,
    cleanups: &Rc<Cell<u32>>,
    unmounted: &Rc<Cell<u32>>,
) -> Element {
    component(
        DestroyEffect {
            cleanups: Rc::clone(cleanups),
            unmounted: Rc::clone(unmounted),
        },
        DestroyProps(updated),
    )
}

#[test]
fn error_boundary_retries_failed_destroy_without_repeating_cleanup() {
    for occurrence in [1, 2] {
        let cleanups = Rc::new(Cell::new(0));
        let unmounted = Rc::new(Cell::new(0));
        let old = error_boundary(destroy_component(false, &cleanups, &unmounted), |_| {
            text_block("fallback").into()
        });
        let new = error_boundary(destroy_component(true, &cleanups, &unmounted), |_| {
            text_block("fallback").into()
        });
        let mut reconciler = Reconciler::new(RecordingBackend::new());
        reconciler.reconcile(None, &old, None, rerender());
        reconciler
            .backend
            .fail_on(BackendOperation::Destroy, occurrence);

        assert!(
            reconciler
                .reconcile(Some(&old), &new, None, rerender())
                .is_some(),
            "destroy occurrence {occurrence} did not mount the fallback"
        );

        assert_eq!(cleanups.get(), 1);
        assert_eq!(unmounted.get(), 2);
        assert_eq!(reconciler.backend.live_control_count(), 1);
        assert_eq!(reconciler.debug_logical_node_count(), 1);
        reconciler.assert_consistent();
        reconciler.backend.assert_consistent();

        reconciler.unmount_root();
        assert_eq!(cleanups.get(), 1);
        assert_eq!(unmounted.get(), 2);
        reconciler.backend.assert_consistent();
        assert_eq!(reconciler.backend.live_control_count(), 0);
    }
}
