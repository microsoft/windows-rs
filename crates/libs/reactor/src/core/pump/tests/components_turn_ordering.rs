//! Component scope, dirty tracking, and turn ordering contract tests for [`Pump`].

use super::super::*;
use super::support::*;
use std::any::TypeId;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[test]
fn trace_component_plan_handles_a_non_empty_local_update() {
    #[derive(Clone)]
    struct TraceInput(Rc<RefCell<Option<LocalSender<()>>>>);

    impl PartialEq for TraceInput {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.0, &other.0)
        }
    }

    struct TracedComponent(u32);

    impl Component for TracedComponent {
        type Message = ();
        type Input = TraceInput;

        fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
            *input.0.borrow_mut() = Some(context.sender());
            Self(0)
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
            self.0 += 1;
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            TextBlock::new().text(self.0.to_string()).into()
        }
    }

    let sender = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<TracedComponent>(TraceInput(Rc::clone(
        &sender,
    ))))
    .unwrap();
    pump.trace_component_plans = true;

    assert!(sender.borrow().as_ref().unwrap().send(()));
    assert_eq!(pump.dispatch_components(1).unwrap(), 1);
}

#[test]
fn stable_sender_callback_input_skip_unrelated_child_recomposition() {
    #[derive(Clone)]
    struct ChildInput {
        callback: Callback<u16>,
        renders: Rc<Cell<u32>>,
    }

    impl PartialEq for ChildInput {
        fn eq(&self, other: &Self) -> bool {
            self.callback == other.callback && Rc::ptr_eq(&self.renders, &other.renders)
        }
    }

    struct Child;

    impl Component for Child {
        type Message = ();
        type Input = ChildInput;

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            input.renders.set(input.renders.get() + 1);
            View::empty()
        }
    }

    #[derive(Clone)]
    struct ParentInput {
        renders: Rc<Cell<u32>>,
        sender: Rc<RefCell<Option<LocalSender<u16>>>>,
    }

    impl PartialEq for ParentInput {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.renders, &other.renders) && Rc::ptr_eq(&self.sender, &other.sender)
        }
    }

    struct Parent;

    impl Component for Parent {
        type Message = u16;
        type Input = ParentInput;

        fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
            *input.sender.borrow_mut() = Some(context.sender());
            Self
        }

        fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
            fn forward(value: u16) -> u16 {
                value
            }

            View::component::<Child>(ChildInput {
                callback: context.callback(forward),
                renders: Rc::clone(&input.renders),
            })
        }
    }

    let renders = Rc::new(Cell::new(0));
    let sender = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Parent>(ParentInput {
        renders: Rc::clone(&renders),
        sender: Rc::clone(&sender),
    }))
    .unwrap();
    assert_eq!(renders.get(), 1);

    assert!(sender.borrow().as_ref().unwrap().send(1));
    assert_eq!(pump.dispatch_components(1).unwrap(), 1);
    assert_eq!(renders.get(), 1);
}

#[test]
fn dirty_component_inside_unchanged_widget_ancestor_recomposes() {
    #[derive(Clone)]
    struct ChildInput {
        renders: Rc<Cell<u32>>,
        sender: Rc<RefCell<Option<LocalSender<()>>>>,
    }

    impl PartialEq for ChildInput {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.renders, &other.renders) && Rc::ptr_eq(&self.sender, &other.sender)
        }
    }

    struct Child(u32);

    impl Component for Child {
        type Message = ();
        type Input = ChildInput;

        fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
            *input.sender.borrow_mut() = Some(context.sender());
            Self(0)
        }

        fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {
            self.0 += 1;
        }

        fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            input.renders.set(input.renders.get() + 1);
            TextBlock::new().text(self.0.to_string()).into()
        }
    }

    #[derive(Clone)]
    struct ParentInput {
        child: ChildInput,
        renders: Rc<Cell<u32>>,
    }

    impl PartialEq for ParentInput {
        fn eq(&self, other: &Self) -> bool {
            self.child == other.child && Rc::ptr_eq(&self.renders, &other.renders)
        }
    }

    struct Parent;

    impl Component for Parent {
        type Message = ();
        type Input = ParentInput;

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

        fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            input.renders.set(input.renders.get() + 1);
            Grid::new().children((View::component::<Child>(input.child.clone()),))
        }
    }

    let child_renders = Rc::new(Cell::new(0));
    let parent_renders = Rc::new(Cell::new(0));
    let sender = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Parent>(ParentInput {
        child: ChildInput {
            renders: Rc::clone(&child_renders),
            sender: Rc::clone(&sender),
        },
        renders: Rc::clone(&parent_renders),
    }))
    .unwrap();
    assert_eq!((parent_renders.get(), child_renders.get()), (1, 1));

    assert!(sender.borrow().as_ref().unwrap().send(()));
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert_eq!((parent_renders.get(), child_renders.get()), (1, 2));
    let grid = Pump::<RecordingRuntime>::native_root(&pump.tree, pump.root().unwrap()).unwrap();
    assert_eq!(recorded_text(pump.runtime(), grid), ["1"]);
}

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
    type Input = PlanningMode;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            mode: input.clone(),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.mode = input.clone();
    }

    fn update(&mut self, _message: Self::Message, _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        match self.mode {
            PlanningMode::Valid => View::native(TextBlock::new().text("valid")),
            PlanningMode::InvalidArity => View::fragment([TextBlock::new(), TextBlock::new()]),
            PlanningMode::DuplicateKey => View::keyed_fragment([
                KeyedView::new("duplicate", View::native(TextBlock::new())),
                KeyedView::new("duplicate", View::native(TextBlock::new())),
            ]),
            PlanningMode::InvalidRole => View::from_kind(ViewKind::Children {
                control: TextBlock::new().into(),
                children: Rc::new(Vec::new()),
            }),
        }
    }
}

#[test]
fn identical_input_recomposes_after_planning_failure() {
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
fn component_turn_recomposes_touched_child_after_planning_failure() {
    #[derive(Clone)]
    struct ParentInput(Rc<RefCell<Option<LocalSender<PlanningMode>>>>);

    impl PartialEq for ParentInput {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.0, &other.0)
        }
    }

    struct Parent {
        mode: PlanningMode,
    }

    impl Component for Parent {
        type Message = PlanningMode;
        type Input = ParentInput;

        fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
            *input.0.borrow_mut() = Some(context.sender());
            Self {
                mode: PlanningMode::Valid,
            }
        }

        fn input_changed(&mut self, _input: &Self::Input, _context: &ComponentContext<Self>) {}

        fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
            self.mode = message;
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            View::component::<PlanningFailureComponent>(self.mode.clone())
        }
    }

    let sender = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Parent>(ParentInput(Rc::clone(&sender))))
        .unwrap();
    let version = pump.version();

    assert!(
        sender
            .borrow()
            .as_ref()
            .unwrap()
            .send(PlanningMode::InvalidArity)
    );
    assert_eq!(
        pump.dispatch_components(1),
        Err(PumpError::StructureUnsupported)
    );
    assert_eq!(
        pump.dispatch_components(1),
        Err(PumpError::StructureUnsupported)
    );
    assert_eq!(pump.version(), version);
}

#[test]
fn mounts_a_component_chain_into_the_authoritative_tree() {
    let mut pump = Pump::new(RecordingRuntime::default());

    pump.mount_view(View::component::<Root>("leaf".to_string()))
        .unwrap();

    let root = pump.root().unwrap();
    assert_eq!(pump.tree.kind(root), NodeKind::Component);
    assert_eq!(pump.tree.component_type(root), TypeId::of::<Root>());
    let root_scope = pump.tree.component_scope(root);
    let root_slot = pump.tree.children(root)[0];
    assert_eq!(pump.tree.kind(root_slot), NodeKind::Slot);
    let leaf = pump.tree.children(root_slot)[0];
    assert_eq!(pump.tree.component_type(leaf), TypeId::of::<Leaf>());
    let leaf_scope = pump.tree.component_scope(leaf);
    let leaf_slot = pump.tree.children(leaf)[0];
    let native = pump.tree.children(leaf_slot)[0];
    assert_eq!(
        pump.tree.kind(native),
        NodeKind::Native(MountedKind::TextBlock)
    );
    assert_eq!(
        pump.runtime()
            .node(native)
            .unwrap()
            .property(PropertyId::TextBlockText),
        Some(&PropertyValue::Str("leaf".to_string()))
    );
    pump.update_view(View::component::<Root>("input".to_string()))
        .unwrap();
    assert_eq!(pump.root(), Some(root));
    assert_eq!(pump.tree.component_scope(root), root_scope);
    assert_eq!(pump.tree.component_scope(leaf), leaf_scope);
    assert_eq!(
        pump.runtime()
            .node(native)
            .unwrap()
            .property(PropertyId::TextBlockText),
        Some(&PropertyValue::Str("input".to_string()))
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
    struct Input(Rc<RefCell<Option<LocalSender<bool>>>>);

    impl PartialEq for Input {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.0, &other.0)
        }
    }

    struct OptionalLeaf {
        visible: bool,
    }

    impl Component for OptionalLeaf {
        type Message = bool;
        type Input = Input;

        fn create(input: &Input, context: &ComponentContext<Self>) -> Self {
            *input.0.borrow_mut() = Some(context.sender());
            Self { visible: false }
        }

        fn input_changed(&mut self, _input: &Input, _context: &ComponentContext<Self>) {}

        fn update(&mut self, visible: bool, _context: &ComponentContext<Self>) {
            self.visible = visible;
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            if self.visible {
                View::native(TextBlock::new().text("visible"))
            } else {
                View::empty()
            }
        }
    }

    let sender = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        StackPanel::new().children((View::component::<OptionalLeaf>(Input(Rc::clone(&sender))),)),
    )
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
    struct Input {
        sender: Rc<RefCell<Option<LocalSender<bool>>>>,
        views: Rc<Cell<u8>>,
    }

    impl PartialEq for Input {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.sender, &other.sender) && Rc::ptr_eq(&self.views, &other.views)
        }
    }

    struct ShapeChange {
        expanded: bool,
        views: Rc<Cell<u8>>,
    }

    impl Component for ShapeChange {
        type Input = Input;
        type Message = bool;

        fn create(input: &Input, context: &ComponentContext<Self>) -> Self {
            *input.sender.borrow_mut() = Some(context.sender());
            Self {
                expanded: false,
                views: Rc::clone(&input.views),
            }
        }

        fn input_changed(&mut self, _input: &Input, _context: &ComponentContext<Self>) {}

        fn update(&mut self, expanded: bool, _context: &ComponentContext<Self>) {
            self.expanded = expanded;
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            self.views.set(self.views.get() + 1);
            if self.expanded {
                StackPanel::new().children([TextBlock::new().text("expanded")])
            } else {
                View::native(TextBlock::new().text("collapsed"))
            }
        }
    }

    let sender = Rc::new(RefCell::new(None));
    let views = Rc::new(Cell::new(0));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ShapeChange>(Input {
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
    struct Input(Rc<RefCell<Option<LocalSender<bool>>>>);

    impl PartialEq for Input {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.0, &other.0)
        }
    }

    struct ShapeChange(bool);

    impl Component for ShapeChange {
        type Input = Input;
        type Message = bool;

        fn create(input: &Input, context: &ComponentContext<Self>) -> Self {
            *input.0.borrow_mut() = Some(context.sender());
            Self(false)
        }

        fn input_changed(&mut self, _input: &Input, _context: &ComponentContext<Self>) {}

        fn update(&mut self, changed: bool, _context: &ComponentContext<Self>) {
            self.0 = changed;
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            if self.0 {
                View::native(Button::new())
            } else {
                View::native(TextBlock::new())
            }
        }
    }

    let sender = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<ShapeChange>(Input(Rc::clone(&sender))))
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
    let slot = pump.tree.children(root)[0];
    let panel = pump.tree.children(slot)[0];
    let children = pump.tree.children(panel).to_vec();
    let scopes = children
        .iter()
        .map(|node| pump.tree.component_scope(*node))
        .collect::<Vec<_>>();

    pump.update_view(View::component::<List>(vec![
        (1, "first".to_string()),
        (2, "second".to_string()),
    ]))
    .unwrap();

    assert_eq!(pump.tree.children(panel), children);
    assert_eq!(
        children
            .iter()
            .map(|node| pump.tree.component_scope(*node))
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

    assert_eq!(pump.tree.children(panel), &[children[1], children[0]]);
    assert_eq!(pump.tree.component_scope(children[0]), scopes[0]);
    assert_eq!(pump.tree.component_scope(children[1]), scopes[1]);
    assert_eq!(
        recorded_text(pump.runtime(), panel),
        vec!["second".to_string(), "first".to_string()]
    );

    let removed = pump
        .components()
        .token(pump.tree.component_scope(children[0]));
    let removed_sender = pump.components().sender::<()>(removed);
    pump.update_view(View::component::<List>(vec![
        (2, "second".to_string()),
        (3, "third".to_string()),
    ]))
    .unwrap();

    let updated = pump.tree.children(panel);
    assert_eq!(updated.len(), 2);
    assert_eq!(pump.tree.component_scope(updated[0]), scopes[1]);
    assert_ne!(pump.tree.component_scope(updated[1]), scopes[0]);
    assert_eq!(
        recorded_text(pump.runtime(), panel),
        vec!["second".to_string(), "third".to_string()]
    );
    removed_sender.send(());
    assert_eq!(pump.components_mut().drain(1).dropped, 0);
}

#[test]
fn same_key_different_component_type_replaces_and_retires_scope() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<MixedList>(false))
        .unwrap();
    let root = pump.root().unwrap();
    let slot = pump.tree.children(root)[0];
    let panel = pump.tree.children(slot)[0];
    let old = pump.tree.children(panel)[0];
    let old_scope = pump.tree.component_scope(old);
    let old_token = pump.components().token(old_scope);
    let old_sender = pump.components().sender::<()>(old_token);

    pump.update_view(View::component::<MixedList>(true))
        .unwrap();

    let replacement = pump.tree.children(panel)[0];
    assert_eq!(
        pump.tree.component_type(replacement),
        TypeId::of::<AltLeaf>()
    );
    assert_ne!(pump.tree.component_scope(replacement), old_scope);
    assert_eq!(
        recorded_text(pump.runtime(), panel),
        vec!["alt:value".to_string()]
    );
    old_sender.send(());
    assert_eq!(pump.components_mut().drain(1).dropped, 0);
}

#[test]
fn failed_type_replacement_is_fatal() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<MixedList>(false))
        .unwrap();
    let root = pump.root().unwrap();
    let slot = pump.tree.children(root)[0];
    let panel = pump.tree.children(slot)[0];
    let old = pump.tree.children(panel)[0];
    let old_token = pump.components().token(pump.tree.component_scope(old));
    let old_sender = pump.components().sender::<()>(old_token);
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
    let root_token = pump.components().token(pump.tree.component_scope(root));
    let root_sender = pump.components().sender::<bool>(root_token);
    let slot = pump.tree.children(root)[0];
    let panel = pump.tree.children(slot)[0];
    let child = pump.tree.children(panel)[0];
    let child_token = pump.components().token(pump.tree.component_scope(child));
    let child_sender = pump.components().sender::<()>(child_token);

    root_sender.send(true);
    child_sender.send(());
    assert_eq!(pump.dispatch_components(10), Ok(1));

    assert_eq!(
        recorded_text(pump.runtime(), panel),
        vec!["alt:value".to_string()]
    );
    child_sender.send(());
    assert_eq!(pump.components_mut().drain(1).dropped, 0);
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
    let parent_token = pump.components().token(pump.tree.component_scope(parent));
    let slot = pump.tree.children(parent)[0];
    let child = pump.tree.children(slot)[0];
    let child_token = pump.components().token(pump.tree.component_scope(child));

    pump.components().sender::<()>(parent_token).send(());
    pump.components().sender::<()>(child_token).send(());
    assert_eq!(pump.dispatch_components(10), Ok(2));
    assert_eq!(counts.parent.get(), 2);
    assert_eq!(counts.child.get(), 2);
}

#[test]
fn parent_input_apply_before_a_queued_child_message() {
    #[derive(Clone)]
    struct ChildInput {
        observed: Rc<Cell<Option<bool>>>,
        sender: Rc<RefCell<Option<LocalSender<()>>>>,
        value: bool,
    }

    impl PartialEq for ChildInput {
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
        type Input = ChildInput;
        type Message = ();

        fn create(input: &ChildInput, context: &ComponentContext<Self>) -> Self {
            *input.sender.borrow_mut() = Some(context.sender());
            Self {
                observed: Rc::clone(&input.observed),
                value: input.value,
            }
        }

        fn input_changed(&mut self, input: &ChildInput, _context: &ComponentContext<Self>) {
            self.value = input.value;
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
            self.observed.set(Some(self.value));
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            View::native(TextBlock::new().text(self.value.to_string()))
        }
    }

    #[derive(Clone)]
    struct ParentInput {
        child_sender: Rc<RefCell<Option<LocalSender<()>>>>,
        observed: Rc<Cell<Option<bool>>>,
        parent_sender: Rc<RefCell<Option<LocalSender<bool>>>>,
    }

    impl PartialEq for ParentInput {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.child_sender, &other.child_sender)
                && Rc::ptr_eq(&self.observed, &other.observed)
                && Rc::ptr_eq(&self.parent_sender, &other.parent_sender)
        }
    }

    struct Parent {
        input: ParentInput,
        value: bool,
    }

    impl Component for Parent {
        type Input = ParentInput;
        type Message = bool;

        fn create(input: &ParentInput, context: &ComponentContext<Self>) -> Self {
            *input.parent_sender.borrow_mut() = Some(context.sender());
            Self {
                input: input.clone(),
                value: false,
            }
        }

        fn input_changed(&mut self, input: &ParentInput, _context: &ComponentContext<Self>) {
            self.input = input.clone();
        }

        fn update(&mut self, value: bool, _context: &ComponentContext<Self>) {
            self.value = value;
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            View::component::<Child>(ChildInput {
                observed: Rc::clone(&self.input.observed),
                sender: Rc::clone(&self.input.child_sender),
                value: self.value,
            })
        }
    }

    let child_sender = Rc::new(RefCell::new(None));
    let observed = Rc::new(Cell::new(None));
    let parent_sender = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Parent>(ParentInput {
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
