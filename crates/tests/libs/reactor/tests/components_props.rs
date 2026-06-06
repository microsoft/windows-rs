use std::cell::Cell;
use std::rc::Rc;

use windows_reactor::core::backend::{ControlKind, Op, RecordingBackend};
use windows_reactor::core::component::Component;
use windows_reactor::core::component_element::{component, memo};
use windows_reactor::core::element::Element;
use windows_reactor::core::reconciler::Reconciler;
use windows_reactor::core::render_context::RenderCx;
use windows_reactor::dsl::{ElementExt, text_block};

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
    existing: Option<windows_reactor::core::backend::ControlId>,
) -> Option<windows_reactor::core::backend::ControlId> {
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

        use windows_reactor::core::element::Button;
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
    use windows_reactor::core::backend::{ControlId, ControlKind, Op};
    use windows_reactor::core::element::StackPanel;

    let stack_a = Element::StackPanel(StackPanel {
        vertical: true,
        children: vec![component(UsesI32State, EmptyProps)],
        ..StackPanel::default()
    });
    let stack_b = Element::StackPanel(StackPanel {
        vertical: true,
        children: vec![component(UsesStringState, EmptyProps)],
        ..StackPanel::default()
    });
    let empty_stack = Element::StackPanel(StackPanel {
        vertical: true,
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
    use windows_reactor::core::element::StackPanel;

    let cleaned = Rc::new(Cell::new(0));
    let inner = component(
        EffectComponent {
            cleaned: Rc::clone(&cleaned),
        },
        EffectProps,
    );

    let with_child = Element::StackPanel(StackPanel {
        vertical: true,
        children: vec![inner],
        ..StackPanel::default()
    });
    let without_child = Element::StackPanel(StackPanel {
        vertical: true,
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
