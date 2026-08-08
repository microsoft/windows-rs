use std::cell::{Cell, RefCell};
use std::rc::Rc;

use test_reactor::{Op, RecordingBackend};
use windows_reactor::Component;
use windows_reactor::ControlKind;
use windows_reactor::Element;
use windows_reactor::Reconciler;
use windows_reactor::RenderCx;
use windows_reactor::{KeyExt, text_block};
use windows_reactor::{Orientation, StackPanel};
use windows_reactor::{component, memo};

#[derive(Clone, PartialEq, Debug)]
struct Greeting {
    who: String,
}

struct GreetingView {
    renders: Rc<Cell<u32>>,
}

impl Component<Greeting> for GreetingView {
    fn render(&self, props: &Greeting, _cx: &mut RenderCx) -> Element {
        self.renders.set(self.renders.get() + 1);
        text_block(format!("hi {}", props.who)).into()
    }
}

fn reconcile(
    r: &mut Reconciler<RecordingBackend>,
    old: Option<&Element>,
    new: &Element,
    existing: Option<windows_reactor::ControlId>,
) -> Option<windows_reactor::ControlId> {
    r.reconcile(old, new, existing, Rc::new(|| {}))
}

#[test]
fn mounting_a_component_element_creates_its_subtree() {
    let renders = Rc::new(Cell::new(0));
    let view = GreetingView {
        renders: Rc::clone(&renders),
    };
    let el = component(view, Greeting { who: "x".into() });

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = reconcile(&mut r, None, &el, None);
    assert!(id.is_some());
    assert_eq!(renders.get(), 1);

    let creates: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter(|o| {
            matches!(
                o,
                Op::Create {
                    kind: ControlKind::TextBlock,
                    ..
                }
            )
        })
        .collect();
    assert_eq!(
        creates.len(),
        1,
        "expected one TextBlock, got ops: {:#?}",
        r.backend.ops
    );
}

#[test]
fn equal_props_skip_re_render() {
    let renders = Rc::new(Cell::new(0));
    let view_a = GreetingView {
        renders: Rc::clone(&renders),
    };
    let view_b = GreetingView {
        renders: Rc::clone(&renders),
    };
    let a = component(view_a, Greeting { who: "x".into() });
    let b = component(view_b, Greeting { who: "x".into() });

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = reconcile(&mut r, None, &a, None).unwrap();
    assert_eq!(renders.get(), 1);

    let _ = reconcile(&mut r, Some(&a), &b, Some(id));

    assert_eq!(renders.get(), 1, "equal props should not re-render");
}

#[test]
fn differing_props_trigger_re_render() {
    let renders = Rc::new(Cell::new(0));
    let view_a = GreetingView {
        renders: Rc::clone(&renders),
    };
    let view_b = GreetingView {
        renders: Rc::clone(&renders),
    };
    let a = component(view_a, Greeting { who: "x".into() });
    let b = component(view_b, Greeting { who: "y".into() });

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = reconcile(&mut r, None, &a, None).unwrap();
    assert_eq!(renders.get(), 1);

    let _ = reconcile(&mut r, Some(&a), &b, Some(id));
    assert_eq!(renders.get(), 2, "differing props must re-render");
}

struct AlwaysRender {
    renders: Rc<Cell<u32>>,
}

impl Component<Greeting> for AlwaysRender {
    fn render(&self, _props: &Greeting, _cx: &mut RenderCx) -> Element {
        self.renders.set(self.renders.get() + 1);
        text_block("always").into()
    }

    fn should_update(&self, _old: &Greeting, _new: &Greeting) -> bool {
        true
    }
}

#[test]
fn should_update_true_forces_rerender_even_with_equal_props() {
    let renders = Rc::new(Cell::new(0));
    let a = component(
        AlwaysRender {
            renders: Rc::clone(&renders),
        },
        Greeting { who: "x".into() },
    );
    let b = component(
        AlwaysRender {
            renders: Rc::clone(&renders),
        },
        Greeting { who: "x".into() },
    );

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = reconcile(&mut r, None, &a, None).unwrap();
    let _ = reconcile(&mut r, Some(&a), &b, Some(id));
    assert_eq!(renders.get(), 2, "should_update=true must always re-render");
}

struct EffectComponent {
    cleaned: Rc<Cell<u32>>,
}

#[derive(Clone, PartialEq, Debug)]
struct EffectProps;

impl Component<EffectProps> for EffectComponent {
    fn render(&self, _props: &EffectProps, cx: &mut RenderCx) -> Element {
        let cleaned = Rc::clone(&self.cleaned);
        cx.use_effect_with_cleanup((), move || {
            Some(Box::new(move || cleaned.set(cleaned.get() + 1)))
        });
        text_block("hooked").into()
    }
}

struct UsesI32State;
struct UsesStringState;

#[derive(Clone, PartialEq, Debug)]
struct EmptyProps;

impl Component<EmptyProps> for UsesI32State {
    fn render(&self, _props: &EmptyProps, cx: &mut RenderCx) -> Element {
        let (n, _set) = cx.use_state(7_i32);
        text_block(format!("i32:{n}")).into()
    }
}

impl Component<EmptyProps> for UsesStringState {
    fn render(&self, _props: &EmptyProps, cx: &mut RenderCx) -> Element {
        let (s, _set) = cx.use_state("hi".to_string());

        use windows_reactor::Button;
        Element::Button(Button {
            content: format!("str:{s}"),
            ..Button::default()
        })
    }
}

#[test]
fn component_type_swap_does_not_reuse_old_render_cx() {
    let a = component(UsesI32State, EmptyProps);
    let b = component(UsesStringState, EmptyProps);

    let mut r = Reconciler::new(RecordingBackend::new());
    let id_a = reconcile(&mut r, None, &a, None).expect("mount of A");

    let id_b = reconcile(&mut r, Some(&a), &b, Some(id_a)).expect("update A→B");

    let _ = id_b;
}

#[test]
fn parent_children_mirror_syncs_when_component_swap_changes_inner_id() {
    use test_reactor::Op;
    use windows_reactor::{ControlId, ControlKind};
    use windows_reactor::{Orientation, StackPanel};

    let stack_a = Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        children: vec![component(UsesI32State, EmptyProps)],
        ..StackPanel::default()
    });
    let stack_b = Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        children: vec![component(UsesStringState, EmptyProps)],
        ..StackPanel::default()
    });
    let empty_stack = Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        children: vec![],
        ..StackPanel::default()
    });

    let mut r = Reconciler::new(RecordingBackend::new());
    let _parent_id = reconcile(&mut r, None, &stack_a, None).expect("mount StackPanel[A]");
    let parent_id = _parent_id;

    let buttons = |r: &Reconciler<RecordingBackend>| -> Vec<ControlId> {
        r.backend
            .ops
            .iter()
            .filter_map(|op| match op {
                Op::Create {
                    id,
                    kind: ControlKind::Button,
                    ..
                } => Some(*id),
                _ => None,
            })
            .collect()
    };
    assert_eq!(buttons(&r).len(), 0, "no Buttons mounted for StackPanel[A]");

    reconcile(&mut r, Some(&stack_a), &stack_b, Some(parent_id));

    let bs = buttons(&r);
    assert_eq!(bs.len(), 1, "one Button mounted for B");
    let button_b = bs[0];

    reconcile(&mut r, Some(&stack_b), &empty_stack, Some(parent_id));

    let destroyed_ids: Vec<ControlId> = r
        .backend
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::Destroy { id } => Some(*id),
            _ => None,
        })
        .collect();

    assert!(
        destroyed_ids.contains(&button_b),
        "audit §7.1.2: StackPanel[B] → StackPanel[] must unmount B's button {button_b:?}; \
         pre-fix the parent's children_mirror would still reference A's stale \
         TextBlock id and B's Button would leak. destroyed_ids = {destroyed_ids:?}"
    );
}

#[test]
fn unmounted_component_runs_effect_cleanup() {
    use windows_reactor::{Orientation, StackPanel};

    let cleaned = Rc::new(Cell::new(0));
    let inner = component(
        EffectComponent {
            cleaned: Rc::clone(&cleaned),
        },
        EffectProps,
    );

    let with_child = Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        children: vec![inner],
        ..StackPanel::default()
    });
    let without_child = Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        children: vec![],
        ..StackPanel::default()
    });

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = reconcile(&mut r, None, &with_child, None).unwrap();

    assert_eq!(cleaned.get(), 0, "cleanup must not run before unmount");

    let _ = reconcile(&mut r, Some(&with_child), &without_child, Some(id));

    assert_eq!(
        cleaned.get(),
        1,
        "audit §7.1.1: unmount must run effect cleanups; got {}",
        cleaned.get()
    );
}

struct ToggleTextEmpty;

#[derive(Clone, PartialEq, Debug)]
struct ShowProps {
    show: bool,
}

impl Component<ShowProps> for ToggleTextEmpty {
    fn render(&self, props: &ShowProps, _cx: &mut RenderCx) -> Element {
        if props.show {
            text_block("v").into()
        } else {
            Element::Empty
        }
    }
}

#[test]
fn component_render_to_empty_does_not_double_unmount() {
    let a = component(ToggleTextEmpty, ShowProps { show: true });
    let b = component(ToggleTextEmpty, ShowProps { show: false });

    let mut r = Reconciler::new(RecordingBackend::new());
    let id_a = reconcile(&mut r, None, &a, None).expect("mount with TextBlock rendered");

    let prefix = r.backend.ops.len();
    let _ = reconcile(&mut r, Some(&a), &b, Some(id_a));
    let window = &r.backend.ops[prefix..];

    let destroys: Vec<_> = window
        .iter()
        .filter(|op| matches!(op, Op::Destroy { id } if *id == id_a))
        .collect();

    assert_eq!(
        destroys.len(),
        1,
        "audit §7.1.4: render→Empty must unmount the old root exactly once; \
         id {id_a:?} ops in swap window: {window:#?}"
    );
}

struct EmptyToggle {
    renders: Rc<Cell<u32>>,
    cleaned: Rc<Cell<u32>>,
    setter: Rc<RefCell<Option<windows_reactor::SetState<bool>>>>,
}

impl Component for EmptyToggle {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        let (show, set_show) = cx.use_state(false);
        *self.setter.borrow_mut() = Some(set_show);
        let cleaned = Rc::clone(&self.cleaned);
        cx.use_effect_with_cleanup((), move || {
            Some(Box::new(move || cleaned.set(cleaned.get() + 1)))
        });
        self.renders.set(self.renders.get() + 1);
        if show {
            text_block("visible").into()
        } else {
            Element::Empty
        }
    }
}

#[test]
fn initially_empty_component_keeps_hooks_and_rerenders_when_dirty() {
    let renders = Rc::new(Cell::new(0));
    let cleaned = Rc::new(Cell::new(0));
    let setter = Rc::new(RefCell::new(None));
    let element = component(
        EmptyToggle {
            renders: Rc::clone(&renders),
            cleaned: Rc::clone(&cleaned),
            setter: Rc::clone(&setter),
        },
        (),
    );

    let mut r = Reconciler::new(RecordingBackend::new());
    assert!(reconcile(&mut r, None, &element, None).is_none());
    assert_eq!(renders.get(), 1);
    assert_eq!(r.debug_logical_component_count(), 1);

    setter.borrow().as_ref().unwrap().call(true);
    let id = reconcile(&mut r, Some(&element), &element, None);
    assert!(
        id.is_some(),
        "dirty empty component must produce native output"
    );
    assert_eq!(renders.get(), 2);
    assert_eq!(cleaned.get(), 0, "output transitions must not run cleanup");

    setter.borrow().as_ref().unwrap().call(false);
    let id = reconcile(&mut r, Some(&element), &element, Some(id.unwrap()));
    assert!(
        id.is_none(),
        "dirty component must be able to become empty again"
    );
    assert_eq!(renders.get(), 3);
    assert_eq!(
        cleaned.get(),
        0,
        "logical component cleanup must be deferred"
    );

    setter.borrow().as_ref().unwrap().call(true);
    let id = reconcile(&mut r, Some(&element), &element, None);
    assert!(
        id.is_some(),
        "empty component must be able to become native again"
    );
    assert_eq!(renders.get(), 4);
    assert_eq!(cleaned.get(), 0, "logical component must remain mounted");

    r.unmount_root();
    assert_eq!(
        cleaned.get(),
        1,
        "cleanup runs when the logical component unmounts"
    );
}

#[test]
fn nested_empty_component_state_reaches_unchanged_native_parent() {
    let renders = Rc::new(Cell::new(0));
    let cleaned = Rc::new(Cell::new(0));
    let setter = Rc::new(RefCell::new(None));
    let tree = Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        children: vec![component(
            EmptyToggle {
                renders: Rc::clone(&renders),
                cleaned: Rc::clone(&cleaned),
                setter: Rc::clone(&setter),
            },
            (),
        )],
        ..StackPanel::default()
    });

    let mut r = Reconciler::new(RecordingBackend::new());
    let root = reconcile(&mut r, None, &tree, None).unwrap();
    assert_eq!(r.backend.children_of(root).len(), 0);

    setter.borrow().as_ref().unwrap().call(true);
    reconcile(&mut r, Some(&tree), &tree, Some(root));
    assert_eq!(r.backend.children_of(root).len(), 1);

    setter.borrow().as_ref().unwrap().call(false);
    reconcile(&mut r, Some(&tree), &tree, Some(root));
    assert_eq!(r.backend.children_of(root).len(), 0);
    assert_eq!(renders.get(), 3);
    assert_eq!(cleaned.get(), 0);
}

struct EmptyByProp {
    renders: Rc<Cell<u32>>,
}

#[derive(Clone, PartialEq, Debug)]
struct Show {
    value: bool,
}

impl Component<Show> for EmptyByProp {
    fn render(&self, props: &Show, _cx: &mut RenderCx) -> Element {
        self.renders.set(self.renders.get() + 1);
        if props.value {
            text_block("shown").into()
        } else {
            Element::Empty
        }
    }
}

fn keyed_empty_stack(show: bool, renders: &Rc<Cell<u32>>) -> Element {
    Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        children: vec![
            text_block("a").with_key("a").into(),
            component(
                EmptyByProp {
                    renders: Rc::clone(renders),
                },
                Show { value: show },
            )
            .with_key("empty"),
            text_block("b").with_key("b").into(),
        ],
        ..StackPanel::default()
    })
}

#[test]
fn empty_component_siblings_do_not_shift_native_indices() {
    let renders = Rc::new(Cell::new(0));
    let old = keyed_empty_stack(false, &renders);
    let mut r = Reconciler::new(RecordingBackend::new());
    let root = reconcile(&mut r, None, &old, None).unwrap();
    assert_eq!(r.backend.children_of(root).len(), 2);

    let shown = keyed_empty_stack(true, &renders);
    reconcile(&mut r, Some(&old), &shown, Some(root));
    let children = r.backend.children_of(root);
    assert_eq!(children.len(), 3);
    assert_eq!(renders.get(), 2);

    let hidden = keyed_empty_stack(false, &renders);
    reconcile(&mut r, Some(&shown), &hidden, Some(root));
    assert_eq!(r.backend.children_of(root).len(), 2);
    assert_eq!(renders.get(), 3);
}

fn positional_empty_stack(show: bool, renders: &Rc<Cell<u32>>) -> Element {
    Element::StackPanel(StackPanel {
        orientation: Orientation::Vertical,
        children: vec![
            text_block("a").into(),
            component(
                EmptyByProp {
                    renders: Rc::clone(renders),
                },
                Show { value: show },
            ),
            text_block("b").into(),
        ],
        ..StackPanel::default()
    })
}

#[test]
fn empty_component_siblings_do_not_shift_positional_native_indices() {
    let renders = Rc::new(Cell::new(0));
    let old = positional_empty_stack(false, &renders);
    let mut r = Reconciler::new(RecordingBackend::new());
    let root = reconcile(&mut r, None, &old, None).unwrap();
    assert_eq!(r.backend.children_of(root).len(), 2);

    let shown = positional_empty_stack(true, &renders);
    reconcile(&mut r, Some(&old), &shown, Some(root));
    assert_eq!(r.backend.children_of(root).len(), 3);

    let hidden = positional_empty_stack(false, &renders);
    reconcile(&mut r, Some(&shown), &hidden, Some(root));
    assert_eq!(r.backend.children_of(root).len(), 2);
    assert_eq!(renders.get(), 3);
}

#[test]
fn memo_skips_even_when_should_update_returns_true() {
    let renders = Rc::new(Cell::new(0));
    let a = memo(
        AlwaysRender {
            renders: Rc::clone(&renders),
        },
        Greeting { who: "x".into() },
    );
    let b = memo(
        AlwaysRender {
            renders: Rc::clone(&renders),
        },
        Greeting { who: "x".into() },
    );

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = reconcile(&mut r, None, &a, None).unwrap();
    assert_eq!(renders.get(), 1);

    let _ = reconcile(&mut r, Some(&a), &b, Some(id));
    assert_eq!(
        renders.get(),
        1,
        "memo must skip on equal props regardless of should_update"
    );
}

struct Counter {
    renders: Rc<Cell<u32>>,
}

impl Component for Counter {
    fn render(&self, _props: &(), _cx: &mut RenderCx) -> Element {
        self.renders.set(self.renders.get() + 1);
        text_block("0").into()
    }
}

#[test]
fn propless_component_still_compiles_and_renders() {
    let renders = Rc::new(Cell::new(0));
    let el = component(
        Counter {
            renders: Rc::clone(&renders),
        },
        (),
    );
    let mut r = Reconciler::new(RecordingBackend::new());
    let _ = reconcile(&mut r, None, &el, None);
    assert_eq!(renders.get(), 1);
}

#[test]
fn with_key_sets_component_element_key() {
    let renders = Rc::new(Cell::new(0));
    let el = component(
        Counter {
            renders: Rc::clone(&renders),
        },
        (),
    )
    .with_key("row-1");
    if let Element::Component(ce) = el {
        assert_eq!(ce.key.as_deref(), Some("row-1"));
    } else {
        panic!("expected Element::Component");
    }
}
