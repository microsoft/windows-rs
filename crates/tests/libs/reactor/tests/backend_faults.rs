use std::cell::Cell;
use std::panic::AssertUnwindSafe;
use std::rc::Rc;

use test_reactor::{BackendOperation, Op, RecordingBackend};
use windows_reactor::{
    Button, Component, ControlKind, Element, Expander, Pivot, PivotItem, Reconciler, RenderCx,
    SplitView, TabItem, TabView, component, error_boundary, text_block, vstack,
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
