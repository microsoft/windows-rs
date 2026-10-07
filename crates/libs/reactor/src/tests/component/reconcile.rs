use super::*;

struct TooltipParent;

impl Component for TooltipParent {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        Border::new()
            .content(component::<Label>("label", Rc::from("Label")).tooltip("Help"))
            .into()
    }
}

struct ContentDialogParent;

impl Component for ContentDialogParent {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        Border::new()
            .content(
                component::<Label>("owner", Rc::from("Owner")).content_dialog(
                    ContentDialog::new().content(component::<Label>("dialog", Rc::from("Dialog"))),
                ),
            )
            .into()
    }
}

struct RootSwitch(bool);

impl Component for RootSwitch {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(false)
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
        self.0 = true;
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        if self.0 {
            Border::new().into()
        } else {
            TextBlock::new().text("Stable").into()
        }
    }
}

struct NestedRoot;

impl Component for NestedRoot {
    type Input = (f64, bool);
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        let root = Border::new().canvas_left(input.0);
        if input.1 {
            root.content(component::<RootSwitch>("child", ())).into()
        } else {
            root.into()
        }
    }
}

struct CounterParent;

impl Component for CounterParent {
    type Input = CounterInput;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        Border::new()
            .content(component::<Counter>("counter", input.clone()))
            .into()
    }
}

struct ReorderParent(bool);

impl Component for ReorderParent {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(false)
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
        self.0 = !self.0;
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        let first = component::<RootSwitch>("first", ()).keyed();
        let second = component::<RootSwitch>("second", ()).keyed();
        if self.0 {
            Grid::new().keyed_children([second, first]).into()
        } else {
            Grid::new().keyed_children([first, second]).into()
        }
    }
}

struct RelationOwnedReorder(bool);

impl Component for RelationOwnedReorder {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(false)
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
        self.0 = !self.0;
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        let first = keyed("first", View::component::<RootSwitch>(()));
        let second = keyed("second", View::component::<RootSwitch>(()));
        if self.0 {
            Grid::new().keyed_children([second, first]).into()
        } else {
            Grid::new().keyed_children([first, second]).into()
        }
    }
}

struct RelationOwnedSiblings;

impl Component for RelationOwnedSiblings {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        StackPanel::new()
            .children((
                Border::new().content(View::component::<RootSwitch>(())),
                Border::new().content(View::component::<RootSwitch>(())),
            ))
            .into()
    }
}

struct RelationOwnedTypeSwitch(bool);

impl Component for RelationOwnedTypeSwitch {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(false)
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
        self.0 = !self.0;
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        let child = if self.0 {
            View::component::<AlternateRoot>(())
        } else {
            View::component::<RootSwitch>(())
        };
        Border::new().content(child).into()
    }
}

struct AlternateRoot;

impl Component for AlternateRoot {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        Button::new().content("alternate").into()
    }
}

struct DuplicateNested;

impl Component for DuplicateNested {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        StackPanel::new()
            .children([
                component::<RootSwitch>("child", ()).into(),
                component::<RootSwitch>("child", ()).into(),
            ])
            .into()
    }
}

struct TreeContent(usize);

impl Component for TreeContent {
    type Input = Rc<str>;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(0)
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
        self.0 += 1;
    }

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        StackPanel::new()
            .children([
                TextBlock::new().text(input.clone()).into(),
                TextBlock::new().text(self.0.to_string()).into(),
            ])
            .into()
    }
}

struct TreeComponents(bool);

impl Component for TreeComponents {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self(false)
    }

    fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
        self.0 = !self.0;
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        let first = TreeNode::new("first", "First")
            .expanded(true)
            .content(component::<TreeContent>("first-content", Rc::from("First")));
        let second = TreeNode::new("second", "Second")
            .expanded(true)
            .content(component::<TreeContent>(
                "second-content",
                Rc::from("Second"),
            ));
        if self.0 {
            TreeView::new().nodes([second, first]).into()
        } else {
            TreeView::new().nodes([first, second]).into()
        }
    }
}

struct RecursiveComponent;

impl Component for RecursiveComponent {
    type Input = usize;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        if *input == 0 {
            TextBlock::new().text("leaf").into()
        } else {
            Border::new()
                .content(component::<Self>("child", *input - 1))
                .into()
        }
    }
}

struct MismatchedKey;

impl Component for MismatchedKey {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        Grid::new()
            .keyed_children([keyed("relation", component::<RootSwitch>("component", ()))])
            .into()
    }
}

#[test]
fn heterogeneous_components_update_isolated_subtrees() {
    let cleanup = Arc::new(AtomicUsize::new(0));
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [
            component::<Counter>(
                "counter",
                CounterInput {
                    cleanup: Arc::clone(&cleanup),
                    value: 0,
                },
            ),
            component::<Label>("label", Rc::from("Label")),
        ],
    )
    .unwrap();
    let counter = host.reference(&Key::from("counter")).unwrap();
    let label = host.reference(&Key::from("label")).unwrap();
    let counter_root = counter.get().unwrap();
    let label_root = label.get().unwrap();

    let mutations = host
        .update_input::<Label>(&Key::from("label"), Rc::from("Changed"))
        .unwrap();
    assert_eq!(mutations.len(), 1);
    assert_eq!(counter.get(), Some(counter_root));
    assert_eq!(label.get(), Some(label_root));

    let sender = host.sender::<Counter>(&Key::from("counter")).unwrap();
    assert!(sender.send(1));
    assert_eq!(host.drain(1).unwrap().mutations, 1);
    assert_eq!(cleanup.load(Ordering::Relaxed), 1);

    assert!(sender.send(0));
    assert_eq!(host.drain(1).unwrap().mutations, 0);
    assert_eq!(cleanup.load(Ordering::Relaxed), 1);
}

#[test]
fn tooltip_attachment_expands_across_component_boundary() {
    let host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<TooltipParent>("parent", ())],
    )
    .unwrap();
    let target = host
        .reference_at(&[Key::from("parent"), Key::from("label")])
        .unwrap()
        .get()
        .unwrap();
    let tooltip = host.runtime().graph().tooltip(target).unwrap();

    assert_eq!(
        host.runtime().adapter().tooltip(target),
        Some((tooltip, TooltipPlacement::Top))
    );
}

#[test]
fn content_dialog_attachment_expands_and_refreshes_component_roots() {
    let host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<ContentDialogParent>("parent", ())],
    )
    .unwrap();
    let owner = host
        .reference_at(&[Key::from("parent"), Key::from("owner")])
        .unwrap()
        .get()
        .unwrap();
    let content = host
        .reference_at(&[Key::from("parent"), Key::from("dialog")])
        .unwrap()
        .get()
        .unwrap();
    let dialog = host.runtime().graph().content_dialog(owner).unwrap().0;

    assert_eq!(
        host.runtime().graph().child(dialog, RelationId::Content),
        Some(content)
    );
    assert_eq!(
        host.runtime().adapter().content_dialog(owner),
        Some((dialog, false))
    );
}

#[test]
fn component_boundary_preserves_scope_across_root_type_changes() {
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [
            component::<RootSwitch>("switch", ()),
            component::<Label>("label", Rc::from("Label")),
        ],
    )
    .unwrap();
    let switch = host.sender::<RootSwitch>(&Key::from("switch")).unwrap();
    let label = host.sender::<Label>(&Key::from("label")).unwrap();
    let switch_reference = host.reference(&Key::from("switch")).unwrap();
    let label_reference = host.reference(&Key::from("label")).unwrap();
    let switch_root = switch_reference.get();
    let label_root = label_reference.get();
    host.runtime.adapter_mut().record_batches(true);

    assert!(switch.send(()));
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    assert_eq!(switch_reference.get(), switch_root);
    assert_eq!(label_reference.get(), label_root);
    assert!(
        host.runtime()
            .adapter()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|mutation| matches!(
                mutation,
                Mutation::Replace {
                    object,
                    kind: ObjectType::Border
                } if Some(*object) == switch_root
            ))
    );

    assert!(label.send(()));
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    assert_eq!(label_reference.get(), label_root);
}

#[test]
fn nested_component_updates_through_its_retained_owner() {
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<NestedRoot>("parent", (0.0, true))],
    )
    .unwrap();
    let path = [Key::from("parent"), Key::from("child")];
    let sender = host.sender_at::<RootSwitch>(&path).unwrap();
    let reference = host.reference_at(&path).unwrap();
    let root = reference.get().unwrap();
    let parent = host.runtime().graph().owner(root).unwrap();
    assert_eq!(parent.1, RelationId::Content);
    assert_eq!(
        host.runtime().graph().kind(root),
        Some(ObjectType::TextBlock)
    );

    assert!(sender.send(()));
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    assert_eq!(reference.get(), Some(root));
    assert_eq!(host.runtime().graph().kind(root), Some(ObjectType::Border));
    assert_eq!(host.runtime().graph().owner(root), Some(parent));
}

#[test]
fn parent_rerender_preserves_and_retires_nested_scope() {
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<NestedRoot>("parent", (0.0, true))],
    )
    .unwrap();
    let path = [Key::from("parent"), Key::from("child")];
    let sender = host.sender_at::<RootSwitch>(&path).unwrap();
    let reference = host.reference_at(&path).unwrap();
    let child = reference.get();

    host.update_input::<NestedRoot>(&Key::from("parent"), (12.0, true))
        .unwrap();
    assert_eq!(reference.get(), child);
    assert!(host.sender_at::<RootSwitch>(&path).is_some());

    assert!(sender.send(()));
    host.update_input::<NestedRoot>(&Key::from("parent"), (12.0, false))
        .unwrap();
    assert_eq!(reference.get(), None);
    assert!(host.sender_at::<RootSwitch>(&path).is_none());
    assert_eq!(host.drain(1).unwrap().dropped, 1);

    host.update_input::<NestedRoot>(&Key::from("parent"), (24.0, true))
        .unwrap();
    assert_eq!(host.scopes.len(), 2);
    let replacement = host.sender_at::<RootSwitch>(&path).unwrap();
    assert!(sender.send(()));
    assert_eq!(host.drain(1).unwrap().dropped, 1);
    assert!(replacement.send(()));
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
}

#[test]
fn nested_input_update_isolated_to_child_subtree() {
    let cleanup = Arc::new(AtomicUsize::new(0));
    let input = CounterInput {
        cleanup: Arc::clone(&cleanup),
        value: 1,
    };
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<CounterParent>("parent", input)],
    )
    .unwrap();
    let path = [Key::from("parent"), Key::from("counter")];
    let parent = host.reference(&Key::from("parent")).unwrap().get();
    let child = host.reference_at(&path).unwrap().get();
    host.runtime.adapter_mut().record_batches(true);

    host.update_input_at::<Counter>(
        &path,
        CounterInput {
            cleanup: Arc::clone(&cleanup),
            value: 2,
        },
    )
    .unwrap();
    assert_eq!(host.reference(&Key::from("parent")).unwrap().get(), parent);
    assert_eq!(host.reference_at(&path).unwrap().get(), child);
    assert_eq!(cleanup.load(Ordering::Relaxed), 1);
    assert!(
        host.runtime()
            .adapter()
            .batches()
            .last()
            .is_some_and(|batch| batch.iter().all(|mutation| match mutation {
                Mutation::SetProperties { object, .. } => Some(*object) == child,
                _ => false,
            }))
    );
}

#[test]
fn nested_keys_are_parent_local_and_stable_through_reorder() {
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [
            component::<ReorderParent>("left", ()),
            component::<NestedRoot>("right", (0.0, true)),
        ],
    )
    .unwrap();
    let first_path = [Key::from("left"), Key::from("first")];
    let second_path = [Key::from("left"), Key::from("second")];
    let right_path = [Key::from("right"), Key::from("child")];
    let first = host.reference_at(&first_path).unwrap().get().unwrap();
    let second = host.reference_at(&second_path).unwrap().get().unwrap();
    assert!(host.reference_at(&right_path).unwrap().get().is_some());
    let parent = host.reference(&Key::from("left")).unwrap().get().unwrap();
    assert_eq!(
        host.runtime()
            .graph()
            .children(parent, RelationId::Children)
            .unwrap(),
        [first, second]
    );

    assert!(
        host.sender::<ReorderParent>(&Key::from("left"))
            .unwrap()
            .send(())
    );
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    assert_eq!(
        host.runtime()
            .graph()
            .children(parent, RelationId::Children)
            .unwrap(),
        [second, first]
    );
    assert_eq!(host.reference_at(&first_path).unwrap().get(), Some(first));
    assert_eq!(host.reference_at(&second_path).unwrap().get(), Some(second));
}

#[test]
fn relation_owned_component_keys_preserve_objects_through_reorder() {
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<RelationOwnedReorder>("parent", ())],
    )
    .unwrap();
    let parent = host.reference(&Key::from("parent")).unwrap().get().unwrap();
    let before = host
        .runtime()
        .graph()
        .children(parent, RelationId::Children)
        .unwrap()
        .to_vec();

    assert!(
        host.sender::<RelationOwnedReorder>(&Key::from("parent"))
            .unwrap()
            .send(())
    );
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    assert_eq!(
        host.runtime()
            .graph()
            .children(parent, RelationId::Children)
            .unwrap(),
        [before[1], before[0]]
    );
}

#[test]
fn relation_owned_component_keys_include_the_complete_native_path() {
    let host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<RelationOwnedSiblings>("parent", ())],
    )
    .unwrap();
    let parent = host.reference(&Key::from("parent")).unwrap().get().unwrap();
    let borders = host
        .runtime()
        .graph()
        .children(parent, RelationId::Children)
        .unwrap();
    let first = host
        .runtime()
        .graph()
        .child(borders[0], RelationId::Content)
        .unwrap();
    let second = host
        .runtime()
        .graph()
        .child(borders[1], RelationId::Content)
        .unwrap();
    assert_ne!(first, second);
}

#[test]
fn relation_owned_component_keys_include_component_type() {
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<RelationOwnedTypeSwitch>("parent", ())],
    )
    .unwrap();
    let parent = host.reference(&Key::from("parent")).unwrap().get().unwrap();
    let before = host
        .runtime()
        .graph()
        .child(parent, RelationId::Content)
        .unwrap();

    assert!(
        host.sender::<RelationOwnedTypeSwitch>(&Key::from("parent"))
            .unwrap()
            .send(())
    );
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    let after = host
        .runtime()
        .graph()
        .child(parent, RelationId::Content)
        .unwrap();
    assert_ne!(before, after);
}

#[test]
fn duplicate_nested_keys_are_rejected_per_parent() {
    assert!(matches!(
        ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<DuplicateNested>("parent", ())]
        ),
        Err(ComponentError::DuplicateKey(key)) if key == Key::from("child")
    ));
}

#[test]
fn component_and_relation_keys_cannot_diverge() {
    assert!(matches!(
        ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<MismatchedKey>("parent", ())]
        ),
        Err(ComponentError::ComponentKey {
            component,
            relation
        }) if component == Key::from("component") && relation == Key::from("relation")
    ));
}

#[test]
fn recursive_component_expansion_enforces_depth_limit() {
    assert!(matches!(
        ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<RecursiveComponent>("root", MAX_DEPTH + 2)]
        ),
        Err(ComponentError::Runtime(UpdateError::Graph(
            GraphError::DepthExceeded
        )))
    ));
}

#[test]
fn tree_node_component_content_survives_structural_reorder() {
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<TreeComponents>("tree", ())],
    )
    .unwrap();
    let first_path = [Key::from("tree"), Key::from("first-content")];
    let second_path = [Key::from("tree"), Key::from("second-content")];
    let first = host.reference_at(&first_path).unwrap().get().unwrap();
    let second = host.reference_at(&second_path).unwrap().get().unwrap();
    assert_eq!(
        host.runtime().graph().kind(first),
        Some(ObjectType::StackPanel)
    );
    assert_eq!(
        host.runtime().graph().owner(first).unwrap().1,
        RelationId::Content
    );

    assert!(host.sender_at::<TreeContent>(&first_path).unwrap().send(()));
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    assert_eq!(host.reference_at(&first_path).unwrap().get(), Some(first));

    assert!(
        host.sender::<TreeComponents>(&Key::from("tree"))
            .unwrap()
            .send(())
    );
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    assert_eq!(host.reference_at(&first_path).unwrap().get(), Some(first));
    assert_eq!(host.reference_at(&second_path).unwrap().get(), Some(second));

    assert!(host.sender_at::<TreeContent>(&first_path).unwrap().send(()));
    assert_eq!(host.drain(1).unwrap().dispatched, 1);
    assert_eq!(host.reference_at(&first_path).unwrap().get(), Some(first));
}
