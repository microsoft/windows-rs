use std::any::Any;
use std::cell::Cell;
use std::panic::AssertUnwindSafe;
use std::rc::Rc;

use test_reactor::{BackendOperation, Op, RecordingBackend};
use windows_reactor::{
    Button, Component, Context, ControlKind, Element, Expander, KeyExt, Pivot, PivotItem,
    ProvideExt, Reconciler, ReconcilerTestExt, RenderCx, SelectionMode, SplitView, TabItem,
    TabView, component, list_view, swap_chain_panel, text_block, vstack,
};

fn rerender() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

fn panic_message(payload: Box<dyn Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<&'static str>() {
        (*message).to_string()
    } else if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else {
        "<non-string panic>".to_string()
    }
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

#[derive(Clone)]
struct FailedMountEffectProps {
    operation: BackendOperation,
    setups: Rc<Cell<u32>>,
    cleanups: Rc<Cell<u32>>,
}

impl PartialEq for FailedMountEffectProps {
    fn eq(&self, other: &Self) -> bool {
        self.operation == other.operation
            && Rc::ptr_eq(&self.setups, &other.setups)
            && Rc::ptr_eq(&self.cleanups, &other.cleanups)
    }
}

struct FailedMountEffect;

impl Component<FailedMountEffectProps> for FailedMountEffect {
    fn render(&self, props: &FailedMountEffectProps, cx: &mut RenderCx) -> Element {
        let setups = Rc::clone(&props.setups);
        let cleanups = Rc::clone(&props.cleanups);
        cx.use_effect_with_cleanup((), move || {
            setups.set(setups.get() + 1);
            Some(move || cleanups.set(cleanups.get() + 1))
        });
        mount_case(props.operation)
    }
}

fn templated_mount_case() -> Element {
    list_view(vec![1_u32, 2], |item, _| text_block(item.to_string()))
        .width(100.0)
        .on_selection_changed(|_| {})
        .selection_mode(SelectionMode::Multiple)
        .can_drag_items(true)
        .can_reorder_items(true)
        .allow_drop(true)
        .on_reorder(|_| {})
        .selected_index(1)
        .build()
}

fn templated_mount_operations() -> [BackendOperation; 11] {
    [
        BackendOperation::Create,
        BackendOperation::SetProp,
        BackendOperation::AttachTemplatedSelectionChanged,
        BackendOperation::AttachTemplatedReorder,
        BackendOperation::AttachTemplatedRealization,
        BackendOperation::SetTemplatedItemCount,
        BackendOperation::SetTemplatedSelectionMode,
        BackendOperation::SetTemplatedCanDragItems,
        BackendOperation::SetTemplatedCanReorderItems,
        BackendOperation::SetTemplatedAllowDrop,
        BackendOperation::SetTemplatedSelectedIndex,
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
fn failed_component_mounts_do_not_run_effects() {
    for operation in mount_operations() {
        let setups = Rc::new(Cell::new(0));
        let cleanups = Rc::new(Cell::new(0));
        let mut backend = RecordingBackend::new();
        backend.fail_next(operation);
        let mut reconciler = Reconciler::new(backend);
        let element = component(
            FailedMountEffect,
            FailedMountEffectProps {
                operation,
                setups: Rc::clone(&setups),
                cleanups: Rc::clone(&cleanups),
            },
        );

        let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
            reconciler.reconcile(None, &element, None, rerender())
        }));

        assert!(result.is_err(), "{operation:?} did not fail");
        assert_eq!(setups.get(), 0, "{operation:?} ran an uncommitted effect");
        assert_eq!(
            cleanups.get(),
            0,
            "{operation:?} cleaned an effect that never committed"
        );
        reconciler.unmount_root();
        reconciler.assert_consistent();
        reconciler.backend.assert_consistent();
    }
}

struct PanickingCommitEffect;

impl Component for PanickingCommitEffect {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        cx.use_effect((), || panic!("post-commit effect failed"));
        text_block("committed").into()
    }
}

#[test]
fn post_commit_effect_panics_leave_the_committed_tree_usable() {
    let element = component(PanickingCommitEffect, ());
    let mut reconciler = Reconciler::new(RecordingBackend::new());

    let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
        reconciler.reconcile(None, &element, None, rerender())
    }));

    assert!(result.is_err());
    assert_eq!(reconciler.backend.live_control_count(), 1);

    let healthy: Element = text_block("healthy").into();
    assert!(
        reconciler
            .reconcile(Some(&element), &healthy, None, rerender())
            .is_some()
    );

    reconciler.unmount_root();
    reconciler.assert_consistent();
    reconciler.backend.assert_consistent();
    assert_eq!(reconciler.backend.live_control_count(), 0);
}

#[test]
fn failed_templated_mounts_roll_back_all_owned_state() {
    for operation in templated_mount_operations() {
        let mut backend = RecordingBackend::new();
        backend.fail_next(operation);
        let mut reconciler = Reconciler::new(backend);
        let element = templated_mount_case();

        let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
            reconciler.reconcile(None, &element, None, rerender());
        }));

        assert!(result.is_err(), "{operation:?} did not fail");
        reconciler.assert_consistent();
        reconciler.backend.assert_consistent();
        assert_eq!(
            reconciler.backend.live_control_count(),
            0,
            "{operation:?} leaked a templated control"
        );
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

        let retry = std::panic::catch_unwind(AssertUnwindSafe(|| {
            reconciler.reconcile(Some(&old), &new, None, rerender())
        }))
        .expect_err("reconciliation was allowed after an uncaught failure");
        assert_eq!(
            panic_message(retry),
            "cannot reconcile after an earlier reconciliation failed; tear down and replace the \
             reconciler",
            "{change:?} did not reject retry with the failure-state contract"
        );

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

#[test]
fn failed_seeded_update_can_be_torn_down_by_control_id() {
    let change = CollectionChange::Append;
    let cleanups = Rc::new(Cell::new(0));
    let old = collection_component(change, false, &cleanups);
    let new = collection_component(change, true, &cleanups);
    let mut reconciler = Reconciler::new(RecordingBackend::new());
    let root = reconciler.mount(&old).unwrap();
    reconciler.backend.fail_next(change.backend_operation());

    let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
        reconciler.reconcile(Some(&old), &new, Some(root), rerender())
    }));

    assert!(result.is_err());
    reconciler.unmount(root);
    assert_eq!(cleanups.get(), 1);
    reconciler.assert_consistent();
    reconciler.backend.assert_consistent();
    assert_eq!(reconciler.backend.live_control_count(), 0);
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

#[derive(Clone, Copy, Debug)]
enum RootUnmount {
    Root,
    Control,
}

fn teardown_root(
    reconciler: &mut Reconciler<RecordingBackend>,
    mode: RootUnmount,
    root: windows_reactor::ControlId,
) {
    match mode {
        RootUnmount::Root => reconciler.unmount_root(),
        RootUnmount::Control => reconciler.unmount(root),
    }
}

#[test]
fn failed_root_teardown_can_be_retried_without_repeating_cleanup() {
    for mode in [RootUnmount::Root, RootUnmount::Control] {
        for occurrence in [1, 2, 3] {
            let cleanups = Rc::new(Cell::new(0));
            let unmounted = Rc::new(Cell::new(0));
            let element = destroy_component(false, &cleanups, &unmounted);
            let mut reconciler = Reconciler::new(RecordingBackend::new());
            let root = reconciler
                .reconcile(None, &element, None, rerender())
                .unwrap();
            reconciler
                .backend
                .fail_on(BackendOperation::Destroy, occurrence);

            let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
                teardown_root(&mut reconciler, mode, root);
            }));

            assert!(
                result.is_err(),
                "{mode:?} destroy occurrence {occurrence} did not fail"
            );
            assert!(cleanups.get() <= 1);
            assert!(unmounted.get() <= 2);
            assert!(
                reconciler.backend.live_control_count() > 0,
                "{mode:?} destroy occurrence {occurrence} lost all live ownership"
            );

            let reconcile_result = std::panic::catch_unwind(AssertUnwindSafe(|| {
                reconciler.reconcile(Some(&element), &element, None, rerender());
            }));
            assert!(
                reconcile_result.is_err(),
                "{mode:?} destroy occurrence {occurrence} allowed reconciliation during teardown"
            );

            teardown_root(&mut reconciler, mode, root);
            assert_eq!(
                cleanups.get(),
                1,
                "{mode:?} destroy occurrence {occurrence} repeated or lost component cleanup"
            );
            assert_eq!(
                unmounted.get(),
                2,
                "{mode:?} destroy occurrence {occurrence} repeated or lost lifecycle cleanup"
            );
            reconciler.assert_consistent();
            reconciler.backend.assert_consistent();
            assert_eq!(reconciler.backend.live_control_count(), 0);
        }
    }
}
