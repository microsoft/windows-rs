//! Regression: a nested component dirtied only by its own `use_state` must
//! re-render even when its parents are structurally-unchanged non-component
//! containers (e.g. `scroll_viewer` -> `grid`). Before the fix, the reconcile
//! pass pruned at the unchanged parent before ever reaching the dirty
//! component, so its re-render was silently dropped (field-reported friction).

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use test_reactor::{Op, RecordingBackend};
use windows_reactor::{
    Component, Dispatcher, DispatcherQueuePriority, Element, Expander, Prop, PropValue, RenderCx,
    RenderHost, SetState, TextBlock, component, grid, list_view, memo, scroll_viewer, text_block,
};

type Job = Box<dyn FnOnce()>;

#[derive(Clone, Default)]
struct TestDispatcher {
    queue: Rc<RefCell<Vec<Job>>>,
}

impl TestDispatcher {
    fn drain(&self) {
        loop {
            let item = {
                let mut q = self.queue.borrow_mut();
                if q.is_empty() {
                    None
                } else {
                    Some(q.remove(0))
                }
            };
            match item {
                Some(f) => f(),
                None => break,
            }
        }
    }
}

impl Dispatcher for TestDispatcher {
    fn enqueue(&self, _p: DispatcherQueuePriority, f: Box<dyn FnOnce()>) -> bool {
        self.queue.borrow_mut().push(f);
        true
    }
}

/// The leaf component: owns its own `use_state`, publishes the setter, and
/// renders a `TextBlock` whose text is the current state so we can observe
/// re-renders via `Op::SetProp { Text }`.
struct Inner {
    renders: Rc<Cell<u32>>,
    setter_out: Rc<RefCell<Option<SetState<u64>>>>,
}

impl Component for Inner {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let (n, set) = cx.use_state(0_u64);
        *self.setter_out.borrow_mut() = Some(set);
        self.renders.set(self.renders.get() + 1);
        Element::TextBlock(TextBlock::new(format!("count-{n}")))
    }
}

/// The root: wraps the leaf under *unchanged non-component* parents,
/// `scroll_viewer` -> `grid`, exactly like the field report.
struct Root {
    renders: Rc<Cell<u32>>,
    setter_out: Rc<RefCell<Option<SetState<u64>>>>,
}

impl Component for Root {
    fn render(&self, _props: &(), _cx: &mut RenderCx) -> Element {
        let inner = component(
            Inner {
                renders: Rc::clone(&self.renders),
                setter_out: Rc::clone(&self.setter_out),
            },
            (),
        );
        Element::ScrollViewer(scroll_viewer(Element::Grid(grid(vec![inner]))))
    }
}

fn last_text(ops: &[Op]) -> Option<String> {
    ops.iter().rev().find_map(|op| match op {
        Op::SetProp {
            prop: Prop::Text,
            value: PropValue::Str(s),
            ..
        } => Some(s.clone()),
        _ => None,
    })
}

#[test]
fn nested_component_rerenders_from_own_use_state() {
    let dispatcher = TestDispatcher::default();
    let setter_out = Rc::new(RefCell::new(None));
    let inner_renders = Rc::new(Cell::new(0));
    let root: Box<dyn Component> = Box::new(Root {
        renders: Rc::clone(&inner_renders),
        setter_out: Rc::clone(&setter_out),
    });
    let host = RenderHost::new(RecordingBackend::new(), root, dispatcher.clone());
    host.kick();
    dispatcher.drain();

    assert_eq!(inner_renders.get(), 1, "inner mounts once");
    assert_eq!(
        host.with_reconciler(|r| last_text(&r.backend.ops)),
        Some("count-0".to_string())
    );

    // The leaf mutates ONLY its own state. Nothing at the parents changes.
    setter_out.borrow().as_ref().unwrap().call(1);
    dispatcher.drain();

    assert_eq!(
        inner_renders.get(),
        2,
        "BUG: nested component dirtied by its own use_state never re-rendered \
         (pruned at the unchanged scroll_viewer/grid parent)"
    );
    assert_eq!(
        host.with_reconciler(|r| last_text(&r.backend.ops)),
        Some("count-1".to_string()),
        "BUG: leaf's text was never updated to the new state value"
    );
}

/// Control: with the leaf as a *direct* child of the root (no unchanged
/// non-component parent in between), the reconciler visits the leaf node itself,
/// `is_component_state_dirty` catches it, and it re-renders correctly. This
/// isolates the bug above to the pruning at the intermediate non-component
/// parents, not to `use_state` re-rendering in general.
struct DirectRoot {
    renders: Rc<Cell<u32>>,
    setter_out: Rc<RefCell<Option<SetState<u64>>>>,
}

impl Component for DirectRoot {
    fn render(&self, _props: &(), _cx: &mut RenderCx) -> Element {
        component(
            Inner {
                renders: Rc::clone(&self.renders),
                setter_out: Rc::clone(&self.setter_out),
            },
            (),
        )
    }
}

#[test]
fn direct_child_component_rerenders_from_own_use_state() {
    let dispatcher = TestDispatcher::default();
    let setter_out = Rc::new(RefCell::new(None));
    let inner_renders = Rc::new(Cell::new(0));
    let root: Box<dyn Component> = Box::new(DirectRoot {
        renders: Rc::clone(&inner_renders),
        setter_out: Rc::clone(&setter_out),
    });
    let host = RenderHost::new(RecordingBackend::new(), root, dispatcher.clone());
    host.kick();
    dispatcher.drain();

    assert_eq!(inner_renders.get(), 1);

    setter_out.borrow().as_ref().unwrap().call(1);
    dispatcher.drain();

    assert_eq!(
        inner_renders.get(),
        2,
        "direct-child leaf re-renders as expected"
    );
    assert_eq!(
        host.with_reconciler(|r| last_text(&r.backend.ops)),
        Some("count-1".to_string())
    );
}

struct PassThrough;

impl Component for PassThrough {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        cx.use_effect_with_cleanup((), || {
            PASS_THROUGH_WRAPPER_EFFECTS.set(PASS_THROUGH_WRAPPER_EFFECTS.get() + 1);
            Some(|| PASS_THROUGH_WRAPPER_CLEANUPS.set(PASS_THROUGH_WRAPPER_CLEANUPS.get() + 1))
        });
        component(StatefulLeaf, ())
    }
}

struct StatefulLeaf;

thread_local! {
    static PASS_THROUGH_SETTER: RefCell<Option<SetState<u64>>> = const { RefCell::new(None) };
    static PASS_THROUGH_EFFECTS: Cell<u32> = const { Cell::new(0) };
    static PASS_THROUGH_CLEANUPS: Cell<u32> = const { Cell::new(0) };
    static PASS_THROUGH_WRAPPER_EFFECTS: Cell<u32> = const { Cell::new(0) };
    static PASS_THROUGH_WRAPPER_CLEANUPS: Cell<u32> = const { Cell::new(0) };
}

impl Component for StatefulLeaf {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let (value, setter) = cx.use_state(0_u64);
        PASS_THROUGH_SETTER.set(Some(setter));
        cx.use_effect_with_cleanup((), || {
            PASS_THROUGH_EFFECTS.set(PASS_THROUGH_EFFECTS.get() + 1);
            Some(|| PASS_THROUGH_CLEANUPS.set(PASS_THROUGH_CLEANUPS.get() + 1))
        });
        TextBlock::new(format!("pass-through-{value}")).into()
    }
}

struct MemoRoot;

impl Component for MemoRoot {
    fn render(&self, _props: &(), _cx: &mut RenderCx) -> Element {
        memo(PassThrough, ())
    }
}

#[test]
fn pass_through_component_keeps_independent_state_and_effects() {
    PASS_THROUGH_SETTER.set(None);
    PASS_THROUGH_EFFECTS.set(0);
    PASS_THROUGH_CLEANUPS.set(0);
    PASS_THROUGH_WRAPPER_EFFECTS.set(0);
    PASS_THROUGH_WRAPPER_CLEANUPS.set(0);

    let dispatcher = TestDispatcher::default();
    let host = RenderHost::new(
        RecordingBackend::new(),
        Box::new(MemoRoot),
        dispatcher.clone(),
    );
    host.kick();
    dispatcher.drain();

    assert_eq!(PASS_THROUGH_EFFECTS.get(), 1);
    assert_eq!(PASS_THROUGH_WRAPPER_EFFECTS.get(), 1);
    assert_eq!(
        host.with_reconciler(|r| last_text(&r.backend.ops)),
        Some("pass-through-0".to_string())
    );

    PASS_THROUGH_SETTER.with_borrow(|setter| setter.as_ref().unwrap().call(1));
    dispatcher.drain();

    assert_eq!(PASS_THROUGH_EFFECTS.get(), 1);
    assert_eq!(PASS_THROUGH_WRAPPER_EFFECTS.get(), 1);
    assert_eq!(
        host.with_reconciler(|r| last_text(&r.backend.ops)),
        Some("pass-through-1".to_string())
    );

    let root_id = host.root_id().unwrap();
    host.with_reconciler_mut(|r| r.unmount(root_id));
    assert_eq!(PASS_THROUGH_CLEANUPS.get(), 1);
    assert_eq!(PASS_THROUGH_WRAPPER_CLEANUPS.get(), 1);
}

struct WidgetRootWrapper;

impl Component for WidgetRootWrapper {
    fn render(&self, _props: &(), _cx: &mut RenderCx) -> Element {
        grid(vec![component(WidgetRootLeaf, ())]).into()
    }
}

struct WidgetRootLeaf;

thread_local! {
    static WIDGET_ROOT_SETTER: RefCell<Option<SetState<u64>>> = const { RefCell::new(None) };
    static WIDGET_ROOT_RENDERS: Cell<u32> = const { Cell::new(0) };
}

impl Component for WidgetRootLeaf {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let (value, setter) = cx.use_state(0_u64);
        WIDGET_ROOT_SETTER.set(Some(setter));
        WIDGET_ROOT_RENDERS.set(WIDGET_ROOT_RENDERS.get() + 1);
        text_block(format!("widget-root-{value}")).into()
    }
}

struct WidgetRootMemo;

impl Component for WidgetRootMemo {
    fn render(&self, _props: &(), _cx: &mut RenderCx) -> Element {
        memo(WidgetRootWrapper, ())
    }
}

#[test]
fn dirty_descendant_rerenders_through_memoized_widget_root() {
    WIDGET_ROOT_SETTER.set(None);
    WIDGET_ROOT_RENDERS.set(0);

    let dispatcher = TestDispatcher::default();
    let host = RenderHost::new(
        RecordingBackend::new(),
        Box::new(WidgetRootMemo),
        dispatcher.clone(),
    );
    host.kick();
    dispatcher.drain();

    assert_eq!(WIDGET_ROOT_RENDERS.get(), 1);
    assert_eq!(
        host.with_reconciler(|r| r.debug_logical_component_count()),
        2
    );
    assert_eq!(
        host.with_reconciler(|r| last_text(&r.backend.ops)),
        Some("widget-root-0".to_string())
    );

    WIDGET_ROOT_SETTER.with_borrow(|setter| setter.as_ref().unwrap().call(1));
    dispatcher.drain();

    assert_eq!(
        WIDGET_ROOT_RENDERS.get(),
        2,
        "a dirty descendant must pierce a memoized component with its own widget root"
    );
    assert_eq!(
        host.with_reconciler(|r| last_text(&r.backend.ops)),
        Some("widget-root-1".to_string())
    );

    let root_id = host.root_id().unwrap();
    host.with_reconciler_mut(|r| r.unmount(root_id));
    assert_eq!(
        host.with_reconciler(|r| r.debug_logical_component_count()),
        0
    );
}

struct DeepPassThrough {
    cleanups: Rc<RefCell<Vec<u8>>>,
    setter_out: Rc<RefCell<Option<SetState<u64>>>>,
    leaf_renders: Rc<Cell<u32>>,
}

impl Component<u8> for DeepPassThrough {
    fn render(&self, depth: &u8, cx: &mut RenderCx) -> Element {
        let cleanup_depth = *depth;
        let cleanups = Rc::clone(&self.cleanups);
        cx.use_effect_with_cleanup((), move || {
            Some(move || cleanups.borrow_mut().push(cleanup_depth))
        });

        if *depth == 0 {
            let (value, setter) = cx.use_state(0_u64);
            *self.setter_out.borrow_mut() = Some(setter);
            self.leaf_renders.set(self.leaf_renders.get() + 1);
            text_block(format!("deep-pass-through-{value}")).into()
        } else {
            component(
                Self {
                    cleanups: Rc::clone(&self.cleanups),
                    setter_out: Rc::clone(&self.setter_out),
                    leaf_renders: Rc::clone(&self.leaf_renders),
                },
                depth - 1,
            )
        }
    }
}

struct DeepPassThroughRoot {
    cleanups: Rc<RefCell<Vec<u8>>>,
    setter_out: Rc<RefCell<Option<SetState<u64>>>>,
    leaf_renders: Rc<Cell<u32>>,
}

impl Component for DeepPassThroughRoot {
    fn render(&self, _props: &(), _cx: &mut RenderCx) -> Element {
        memo(
            DeepPassThrough {
                cleanups: Rc::clone(&self.cleanups),
                setter_out: Rc::clone(&self.setter_out),
                leaf_renders: Rc::clone(&self.leaf_renders),
            },
            3,
        )
    }
}

#[test]
fn deep_pass_through_components_keep_identity_and_cleanup_order() {
    let dispatcher = TestDispatcher::default();
    let cleanups = Rc::new(RefCell::new(Vec::new()));
    let setter_out = Rc::new(RefCell::new(None));
    let leaf_renders = Rc::new(Cell::new(0));
    let host = RenderHost::new(
        RecordingBackend::new(),
        Box::new(DeepPassThroughRoot {
            cleanups: Rc::clone(&cleanups),
            setter_out: Rc::clone(&setter_out),
            leaf_renders: Rc::clone(&leaf_renders),
        }),
        dispatcher.clone(),
    );
    host.kick();
    dispatcher.drain();

    assert_eq!(leaf_renders.get(), 1);
    assert_eq!(
        host.with_reconciler(|r| r.debug_logical_component_count()),
        4
    );

    setter_out.borrow().as_ref().unwrap().call(1);
    dispatcher.drain();

    assert_eq!(leaf_renders.get(), 2);
    assert_eq!(
        host.with_reconciler(|r| last_text(&r.backend.ops)),
        Some("deep-pass-through-1".to_string())
    );

    let root_id = host.root_id().unwrap();
    host.with_reconciler_mut(|r| r.unmount(root_id));
    assert_eq!(&*cleanups.borrow(), &[0, 1, 2, 3]);
    assert_eq!(
        host.with_reconciler(|r| r.debug_logical_component_count()),
        0
    );
}

struct HeaderLeaf {
    setter_out: Rc<RefCell<Option<SetState<u64>>>>,
    renders: Rc<Cell<u32>>,
}

impl Component for HeaderLeaf {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let (value, setter) = cx.use_state(0_u64);
        *self.setter_out.borrow_mut() = Some(setter);
        self.renders.set(self.renders.get() + 1);
        text_block(format!("header-state-{value}")).into()
    }
}

struct HeaderRoot {
    setter_out: Rc<RefCell<Option<SetState<u64>>>>,
    renders: Rc<Cell<u32>>,
}

impl Component for HeaderRoot {
    fn render(&self, _props: &(), _cx: &mut RenderCx) -> Element {
        Expander::new(text_block("body"))
            .header_content(component(
                HeaderLeaf {
                    setter_out: Rc::clone(&self.setter_out),
                    renders: Rc::clone(&self.renders),
                },
                (),
            ))
            .expanded(true)
            .into()
    }
}

#[test]
fn header_component_state_reaches_secondary_owned_subtree() {
    let dispatcher = TestDispatcher::default();
    let setter_out = Rc::new(RefCell::new(None));
    let renders = Rc::new(Cell::new(0));
    let host = RenderHost::new(
        RecordingBackend::new(),
        Box::new(HeaderRoot {
            setter_out: Rc::clone(&setter_out),
            renders: Rc::clone(&renders),
        }),
        dispatcher.clone(),
    );
    host.kick();
    dispatcher.drain();

    assert_eq!(renders.get(), 1);
    setter_out.borrow().as_ref().unwrap().call(1);
    dispatcher.drain();

    assert_eq!(renders.get(), 2);
    assert_eq!(
        host.with_reconciler(|r| last_text(&r.backend.ops)),
        Some("header-state-1".to_string())
    );
}

struct TemplatedRowRoot {
    setter_out: Rc<RefCell<Option<SetState<u64>>>>,
    renders: Rc<Cell<u32>>,
}

impl Component for TemplatedRowRoot {
    fn render(&self, _props: &(), _cx: &mut RenderCx) -> Element {
        let setter_out = Rc::clone(&self.setter_out);
        let renders = Rc::clone(&self.renders);
        list_view(vec![0_u8], move |_, _| {
            component(
                HeaderLeaf {
                    setter_out: Rc::clone(&setter_out),
                    renders: Rc::clone(&renders),
                },
                (),
            )
        })
        .build()
    }
}

#[test]
fn realized_row_component_state_reaches_templated_owner() {
    let dispatcher = TestDispatcher::default();
    let setter_out = Rc::new(RefCell::new(None));
    let renders = Rc::new(Cell::new(0));
    let host = RenderHost::new(
        RecordingBackend::new(),
        Box::new(TemplatedRowRoot {
            setter_out: Rc::clone(&setter_out),
            renders: Rc::clone(&renders),
        }),
        dispatcher.clone(),
    );
    host.kick();
    dispatcher.drain();

    let list_id = host.root_id().unwrap();
    host.with_reconciler_mut(|r| r.backend.simulate_prepare_row(list_id, 0));
    dispatcher.drain();

    assert_eq!(renders.get(), 1);
    setter_out.borrow().as_ref().unwrap().call(1);
    dispatcher.drain();

    assert_eq!(renders.get(), 2);
    assert_eq!(
        host.with_reconciler(|r| last_text(&r.backend.ops)),
        Some("header-state-1".to_string())
    );
}

struct CleanupHeader {
    order: Rc<RefCell<Vec<&'static str>>>,
}

impl Component for CleanupHeader {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let order = Rc::clone(&self.order);
        cx.use_effect_with_cleanup((), move || Some(move || order.borrow_mut().push("header")));
        text_block("header").into()
    }
}

struct CleanupOwner {
    order: Rc<RefCell<Vec<&'static str>>>,
}

impl Component for CleanupOwner {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let order = Rc::clone(&self.order);
        cx.use_effect_with_cleanup((), move || Some(move || order.borrow_mut().push("owner")));
        Expander::new(text_block("body"))
            .header_content(component(
                CleanupHeader {
                    order: Rc::clone(&self.order),
                },
                (),
            ))
            .into()
    }
}

struct CleanupApp {
    order: Rc<RefCell<Vec<&'static str>>>,
}

impl Component for CleanupApp {
    fn render(&self, _props: &(), _cx: &mut RenderCx) -> Element {
        component(
            CleanupOwner {
                order: Rc::clone(&self.order),
            },
            (),
        )
    }
}

#[test]
fn secondary_owned_subtree_cleans_up_before_owner() {
    let order = Rc::new(RefCell::new(Vec::new()));
    let dispatcher = TestDispatcher::default();
    let host = RenderHost::new(
        RecordingBackend::new(),
        Box::new(CleanupApp {
            order: Rc::clone(&order),
        }),
        dispatcher.clone(),
    );
    host.kick();
    dispatcher.drain();

    host.with_reconciler_mut(|r| r.unmount(host.root_id().unwrap()));

    assert_eq!(&*order.borrow(), &["header", "owner"]);
}
