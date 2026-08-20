//! Component scope, dirty tracking, and turn ordering contract tests for [`Pump`].

use super::super::*;
use super::support::*;
use crate::native::*;
use std::any::TypeId;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Clone, PartialEq)]
enum PlanningMode {
    Valid,
    InvalidArity,
    DuplicateKey,
    InvalidRole,
}

struct PlanningFailureComponent {
    mode: PlanningMode,
}

impl Component for PlanningFailureComponent {
    type Message = ();
    type Props = PlanningMode;

    fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self {
            mode: props.clone(),
        }
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        self.mode = props.clone();
    }

    fn update(&mut self, _message: Self::Message, _context: &mut ComponentContext<Self>) {}

    fn view(&self, _context: &mut ViewContext<Self>) -> View {
        match self.mode {
            PlanningMode::Valid => View::native(TextBlock::new().text("valid")),
            PlanningMode::InvalidArity => View::fragment([
                KeyedView::new("a", View::native(TextBlock::new())),
                KeyedView::new("b", View::native(TextBlock::new())),
            ]),
            PlanningMode::DuplicateKey => View::fragment([
                KeyedView::new("duplicate", View::native(TextBlock::new())),
                KeyedView::new("duplicate", View::native(TextBlock::new())),
            ]),
            PlanningMode::InvalidRole => View::Children {
                control: TextBlock::new().into(),
                children: Rc::new(Vec::new()),
            },
        }
    }
}

#[test]
fn identical_props_retry_recomposes_after_planning_failure() {
    for (mode, expected) in [
        (PlanningMode::InvalidArity, PumpError::StructureUnsupported),
        (
            PlanningMode::DuplicateKey,
            PumpError::DuplicateKey(Key::from("duplicate")),
        ),
        (PlanningMode::InvalidRole, PumpError::StructureUnsupported),
    ] {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount_view(View::component::<PlanningFailureComponent>(
            PlanningMode::Valid,
        ))
        .unwrap();
        let version = pump.version();

        assert_eq!(
            pump.update_view(View::component::<PlanningFailureComponent>(mode.clone())),
            Err(expected.clone())
        );
        assert_eq!(
            pump.update_view(View::component::<PlanningFailureComponent>(mode)),
            Err(expected)
        );
        assert_eq!(pump.version(), version);
        pump.update_view(View::native(TextBlock::new().text("replacement")))
            .unwrap();
        assert!(pump.planning_dirty.is_empty());
    }
}

#[test]
fn mounts_a_component_chain_into_the_authoritative_tree() {
    let mut pump = Pump::new(RecordingRuntime::default());

    pump.mount_view(View::component::<Root>("leaf".to_string()))
        .unwrap();

    let root = pump.root().unwrap();
    assert_eq!(pump.tree.kind(root), Ok(NodeKind::Component));
    assert_eq!(pump.tree.component_type(root), Ok(TypeId::of::<Root>()));
    let root_scope = pump.tree.component_scope(root).unwrap();
    let root_slot = pump.tree.children(root).unwrap()[0];
    assert_eq!(pump.tree.kind(root_slot), Ok(NodeKind::Slot));
    let leaf = pump.tree.children(root_slot).unwrap()[0];
    assert_eq!(pump.tree.component_type(leaf), Ok(TypeId::of::<Leaf>()));
    let leaf_scope = pump.tree.component_scope(leaf).unwrap();
    let leaf_slot = pump.tree.children(leaf).unwrap()[0];
    let native = pump.tree.children(leaf_slot).unwrap()[0];
    assert_eq!(
        pump.tree.kind(native),
        Ok(NodeKind::Native(MountedKind::TextBlock))
    );
    assert_eq!(
        pump.runtime()
            .node(native)
            .unwrap()
            .property(PropertyId::TextBlockText),
        Some(&PropertyValue::Str("leaf".to_string()))
    );
    pump.update_view(View::component::<Root>("props".to_string()))
        .unwrap();
    assert_eq!(pump.root(), Some(root));
    assert_eq!(pump.tree.component_scope(root), Ok(root_scope));
    assert_eq!(pump.tree.component_scope(leaf), Ok(leaf_scope));
    assert_eq!(
        pump.runtime()
            .node(native)
            .unwrap()
            .property(PropertyId::TextBlockText),
        Some(&PropertyValue::Str("props".to_string()))
    );

    assert_eq!(pump.dispatch_components(10), Ok(1));
    assert_eq!(
        pump.runtime()
            .node(native)
            .unwrap()
            .property(PropertyId::TextBlockText),
        Some(&PropertyValue::Str("message".to_string()))
    );
}

#[test]
fn native_mount_failure_does_not_publish_component_scopes() {
    let mut runtime = RecordingRuntime::default();
    runtime.fail_at(0);
    let mut pump = Pump::new(runtime);

    assert!(matches!(
        pump.mount_view(View::component::<Root>("leaf".to_string())),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert!(pump.poisoned());
    assert!(pump.root().is_none());
}

#[test]
fn component_can_toggle_between_empty_and_one_native_root() {
    #[derive(Clone)]
    struct Props(Rc<RefCell<Option<LocalSender<bool>>>>);

    impl PartialEq for Props {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.0, &other.0)
        }
    }

    struct OptionalLeaf {
        visible: bool,
    }

    impl Component for OptionalLeaf {
        type Message = bool;
        type Props = Props;

        fn create(props: &Props, context: &mut ComponentContext<Self>) -> Self {
            *props.0.borrow_mut() = Some(context.sender());
            Self { visible: false }
        }

        fn changed(&mut self, _props: &Props, _context: &mut ComponentContext<Self>) {}

        fn update(&mut self, visible: bool, _context: &mut ComponentContext<Self>) {
            self.visible = visible;
        }

        fn view(&self, _context: &mut ViewContext<Self>) -> View {
            if self.visible {
                View::native(TextBlock::new().text("visible"))
            } else {
                View::Empty
            }
        }
    }

    let sender = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::children(
        StackPanel::new(),
        [KeyedView::new(
            "optional",
            View::component::<OptionalLeaf>(Props(Rc::clone(&sender))),
        )],
    ))
    .unwrap();
    let root = pump.root().unwrap();
    assert!(pump.runtime().node(root).unwrap().children().is_empty());

    assert!(sender.borrow().as_ref().unwrap().send(true));
    pump.dispatch_components(1).unwrap();
    assert_eq!(recorded_text(pump.runtime(), root), ["visible"]);

    assert!(sender.borrow().as_ref().unwrap().send(false));
    pump.dispatch_components(1).unwrap();
    assert!(pump.runtime().node(root).unwrap().children().is_empty());
}

#[test]
fn local_probe_fallback_composes_once() {
    #[derive(Clone)]
    struct Props {
        sender: Rc<RefCell<Option<LocalSender<bool>>>>,
        views: Rc<Cell<u8>>,
    }

    impl PartialEq for Props {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.sender, &other.sender) && Rc::ptr_eq(&self.views, &other.views)
        }
    }

    struct ShapeChange {
        expanded: bool,
        views: Rc<Cell<u8>>,
    }

    impl Component for ShapeChange {
        type Props = Props;
        type Message = bool;

        fn create(props: &Props, context: &mut ComponentContext<Self>) -> Self {
            *props.sender.borrow_mut() = Some(context.sender());
            Self {
                expanded: false,
                views: Rc::clone(&props.views),
            }
        }

        fn changed(&mut self, _props: &Props, _context: &mut ComponentContext<Self>) {}

        fn update(&mut self, expanded: bool, _context: &mut ComponentContext<Self>) {
            self.expanded = expanded;
        }

        fn view(&self, _context: &mut ViewContext<Self>) -> View {
            self.views.set(self.views.get() + 1);
            if self.expanded {
                View::children(
                    StackPanel::new(),
                    [KeyedView::new(
                        "child",
                        View::native(TextBlock::new().text("expanded")),
                    )],
                )
            } else {
                View::native(TextBlock::new().text("collapsed"))
            }
        }
    }

    let sender = Rc::new(RefCell::new(None));
    let views = Rc::new(Cell::new(0));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ShapeChange>(Props {
        sender: Rc::clone(&sender),
        views: Rc::clone(&views),
    }))
    .unwrap();
    assert_eq!(views.get(), 1);

    assert!(sender.borrow().as_ref().unwrap().send(true));
    pump.dispatch_components(1).unwrap();
    assert_eq!(views.get(), 2);
}

#[test]
fn failed_component_candidate_is_fatal() {
    #[derive(Clone)]
    struct Props(Rc<RefCell<Option<LocalSender<bool>>>>);

    impl PartialEq for Props {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.0, &other.0)
        }
    }

    struct ShapeChange(bool);

    impl Component for ShapeChange {
        type Props = Props;
        type Message = bool;

        fn create(props: &Props, context: &mut ComponentContext<Self>) -> Self {
            *props.0.borrow_mut() = Some(context.sender());
            Self(false)
        }

        fn changed(&mut self, _props: &Props, _context: &mut ComponentContext<Self>) {}

        fn update(&mut self, changed: bool, _context: &mut ComponentContext<Self>) {
            self.0 = changed;
        }

        fn view(&self, _context: &mut ViewContext<Self>) -> View {
            if self.0 {
                View::native(Button::new())
            } else {
                View::native(TextBlock::new())
            }
        }
    }

    let sender = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ShapeChange>(Props(Rc::clone(&sender))))
        .unwrap();
    assert!(sender.borrow().as_ref().unwrap().send(true));
    pump.runtime_mut().fail_at(0);
    assert!(matches!(
        pump.dispatch_components(1),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert!(pump.poisoned());
}

#[test]
fn keyed_component_siblings_retain_scopes_across_prop_updates() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<List>(vec![
        (1, "one".to_string()),
        (2, "two".to_string()),
    ]))
    .unwrap();

    let root = pump.root().unwrap();
    let slot = pump.tree.children(root).unwrap()[0];
    let panel = pump.tree.children(slot).unwrap()[0];
    let children = pump.tree.children(panel).unwrap().to_vec();
    let scopes = children
        .iter()
        .map(|node| pump.tree.component_scope(*node).unwrap())
        .collect::<Vec<_>>();

    pump.update_view(View::component::<List>(vec![
        (1, "first".to_string()),
        (2, "second".to_string()),
    ]))
    .unwrap();

    assert_eq!(pump.tree.children(panel), Ok(children.as_slice()));
    assert_eq!(
        children
            .iter()
            .map(|node| pump.tree.component_scope(*node).unwrap())
            .collect::<Vec<_>>(),
        scopes
    );
    assert_eq!(
        recorded_text(pump.runtime(), panel),
        vec!["first".to_string(), "second".to_string()]
    );

    pump.update_view(View::component::<List>(vec![
        (2, "second".to_string()),
        (1, "first".to_string()),
    ]))
    .unwrap();

    assert_eq!(
        pump.tree.children(panel),
        Ok(&[children[1], children[0]][..])
    );
    assert_eq!(pump.tree.component_scope(children[0]), Ok(scopes[0]));
    assert_eq!(pump.tree.component_scope(children[1]), Ok(scopes[1]));
    assert_eq!(
        recorded_text(pump.runtime(), panel),
        vec!["second".to_string(), "first".to_string()]
    );

    let removed = pump
        .components()
        .token(pump.tree.component_scope(children[0]).unwrap())
        .unwrap();
    let removed_sender = pump.components().sender::<()>(removed).unwrap();
    pump.update_view(View::component::<List>(vec![
        (2, "second".to_string()),
        (3, "third".to_string()),
    ]))
    .unwrap();

    let updated = pump.tree.children(panel).unwrap();
    assert_eq!(updated.len(), 2);
    assert_eq!(pump.tree.component_scope(updated[0]), Ok(scopes[1]));
    assert_ne!(pump.tree.component_scope(updated[1]), Ok(scopes[0]));
    assert_eq!(
        recorded_text(pump.runtime(), panel),
        vec!["second".to_string(), "third".to_string()]
    );
    removed_sender.send(());
    assert_eq!(pump.components_mut().drain(1).unwrap().dropped, 0);
}

#[test]
fn same_key_different_component_type_replaces_and_retires_scope() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<MixedList>(false))
        .unwrap();
    let root = pump.root().unwrap();
    let slot = pump.tree.children(root).unwrap()[0];
    let panel = pump.tree.children(slot).unwrap()[0];
    let old = pump.tree.children(panel).unwrap()[0];
    let old_scope = pump.tree.component_scope(old).unwrap();
    let old_token = pump.components().token(old_scope).unwrap();
    let old_sender = pump.components().sender::<()>(old_token).unwrap();

    pump.update_view(View::component::<MixedList>(true))
        .unwrap();

    let replacement = pump.tree.children(panel).unwrap()[0];
    assert_eq!(
        pump.tree.component_type(replacement),
        Ok(TypeId::of::<AltLeaf>())
    );
    assert_ne!(pump.tree.component_scope(replacement), Ok(old_scope));
    assert_eq!(
        recorded_text(pump.runtime(), panel),
        vec!["alt:value".to_string()]
    );
    old_sender.send(());
    assert_eq!(pump.components_mut().drain(1).unwrap().dropped, 0);
}

#[test]
fn failed_type_replacement_is_fatal() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<MixedList>(false))
        .unwrap();
    let root = pump.root().unwrap();
    let slot = pump.tree.children(root).unwrap()[0];
    let panel = pump.tree.children(slot).unwrap()[0];
    let old = pump.tree.children(panel).unwrap()[0];
    let old_token = pump
        .components()
        .token(pump.tree.component_scope(old).unwrap())
        .unwrap();
    let old_sender = pump.components().sender::<()>(old_token).unwrap();
    pump.runtime_mut().fail_after(0, 0);

    assert!(matches!(
        pump.update_view(View::component::<MixedList>(true)),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert!(pump.poisoned());
    assert_eq!(
        pump.update_view(View::component::<MixedList>(true)),
        Err(PumpError::Poisoned)
    );
    drop(old_sender);
}

#[test]
fn repeated_component_failure_is_not_retried() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<MixedList>(false))
        .unwrap();
    pump.runtime_mut().fail_after(0, 0);

    assert!(matches!(
        pump.update_view(View::component::<MixedList>(true)),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert!(pump.poisoned());
    assert_eq!(
        pump.update_view(View::component::<MixedList>(true)),
        Err(PumpError::Poisoned)
    );
}

#[test]
fn parent_replacement_discards_dirty_work_for_the_retired_child() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<MixedList>(false))
        .unwrap();
    let root = pump.root().unwrap();
    let root_token = pump
        .components()
        .token(pump.tree.component_scope(root).unwrap())
        .unwrap();
    let root_sender = pump.components().sender::<bool>(root_token).unwrap();
    let slot = pump.tree.children(root).unwrap()[0];
    let panel = pump.tree.children(slot).unwrap()[0];
    let child = pump.tree.children(panel).unwrap()[0];
    let child_token = pump
        .components()
        .token(pump.tree.component_scope(child).unwrap())
        .unwrap();
    let child_sender = pump.components().sender::<()>(child_token).unwrap();

    root_sender.send(true);
    child_sender.send(());
    assert_eq!(pump.dispatch_components(10), Ok(1));

    assert_eq!(
        recorded_text(pump.runtime(), panel),
        vec!["alt:value".to_string()]
    );
    child_sender.send(());
    assert_eq!(pump.components_mut().drain(1).unwrap().dropped, 0);
}

#[test]
fn dirty_parent_and_child_each_compose_once_parent_first() {
    let counts = ViewCounts {
        child: Rc::new(Cell::new(0)),
        parent: Rc::new(Cell::new(0)),
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<CountingParent>(counts.clone()))
        .unwrap();
    let parent = pump.root().unwrap();
    let parent_token = pump
        .components()
        .token(pump.tree.component_scope(parent).unwrap())
        .unwrap();
    let slot = pump.tree.children(parent).unwrap()[0];
    let child = pump.tree.children(slot).unwrap()[0];
    let child_token = pump
        .components()
        .token(pump.tree.component_scope(child).unwrap())
        .unwrap();

    pump.components()
        .sender::<()>(parent_token)
        .unwrap()
        .send(());
    pump.components()
        .sender::<()>(child_token)
        .unwrap()
        .send(());
    assert_eq!(pump.dispatch_components(10), Ok(2));
    assert_eq!(counts.parent.get(), 2);
    assert_eq!(counts.child.get(), 2);
}

#[test]
fn parent_props_apply_before_a_queued_child_message() {
    #[derive(Clone)]
    struct ChildProps {
        observed: Rc<Cell<Option<bool>>>,
        sender: Rc<RefCell<Option<LocalSender<()>>>>,
        value: bool,
    }

    impl PartialEq for ChildProps {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
                && Rc::ptr_eq(&self.observed, &other.observed)
                && Rc::ptr_eq(&self.sender, &other.sender)
        }
    }

    struct Child {
        observed: Rc<Cell<Option<bool>>>,
        value: bool,
    }

    impl Component for Child {
        type Props = ChildProps;
        type Message = ();

        fn create(props: &ChildProps, context: &mut ComponentContext<Self>) -> Self {
            *props.sender.borrow_mut() = Some(context.sender());
            Self {
                observed: Rc::clone(&props.observed),
                value: props.value,
            }
        }

        fn changed(&mut self, props: &ChildProps, _context: &mut ComponentContext<Self>) {
            self.value = props.value;
        }

        fn update(&mut self, (): (), _context: &mut ComponentContext<Self>) {
            self.observed.set(Some(self.value));
        }

        fn view(&self, _context: &mut ViewContext<Self>) -> View {
            View::native(TextBlock::new().text(self.value.to_string()))
        }
    }

    #[derive(Clone)]
    struct ParentProps {
        child_sender: Rc<RefCell<Option<LocalSender<()>>>>,
        observed: Rc<Cell<Option<bool>>>,
        parent_sender: Rc<RefCell<Option<LocalSender<bool>>>>,
    }

    impl PartialEq for ParentProps {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.child_sender, &other.child_sender)
                && Rc::ptr_eq(&self.observed, &other.observed)
                && Rc::ptr_eq(&self.parent_sender, &other.parent_sender)
        }
    }

    struct Parent {
        props: ParentProps,
        value: bool,
    }

    impl Component for Parent {
        type Props = ParentProps;
        type Message = bool;

        fn create(props: &ParentProps, context: &mut ComponentContext<Self>) -> Self {
            *props.parent_sender.borrow_mut() = Some(context.sender());
            Self {
                props: props.clone(),
                value: false,
            }
        }

        fn changed(&mut self, props: &ParentProps, _context: &mut ComponentContext<Self>) {
            self.props = props.clone();
        }

        fn update(&mut self, value: bool, _context: &mut ComponentContext<Self>) {
            self.value = value;
        }

        fn view(&self, _context: &mut ViewContext<Self>) -> View {
            View::component::<Child>(ChildProps {
                observed: Rc::clone(&self.props.observed),
                sender: Rc::clone(&self.props.child_sender),
                value: self.value,
            })
        }
    }

    let child_sender = Rc::new(RefCell::new(None));
    let observed = Rc::new(Cell::new(None));
    let parent_sender = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Parent>(ParentProps {
        child_sender: Rc::clone(&child_sender),
        observed: Rc::clone(&observed),
        parent_sender: Rc::clone(&parent_sender),
    }))
    .unwrap();

    assert!(parent_sender.borrow().as_ref().unwrap().send(true));
    assert!(child_sender.borrow().as_ref().unwrap().send(()));
    assert_eq!(pump.dispatch_components(10), Ok(2));
    assert_eq!(observed.get(), Some(true));
}
