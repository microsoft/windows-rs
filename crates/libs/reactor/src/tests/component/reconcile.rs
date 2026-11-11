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

struct InputReceiver;

impl Component for InputReceiver {
    type Input = (usize, Callback<usize>);
    type Message = ();

    fn create(_: &Self::Input, _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, (): (), context: &ComponentContext<Self>) {
        let input = context.input();
        input.1.call(input.0);
    }

    fn view(&self, _: &Self::Input, _: &mut ViewContext<Self>) -> View {
        Border::new().into()
    }
}

struct InputParent;

impl Component for InputParent {
    type Input = <InputReceiver as Component>::Input;
    type Message = ();

    fn create(_: &Self::Input, _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, _: &mut ViewContext<Self>) -> View {
        Border::new()
            .content(component::<InputReceiver>("child", input.clone()))
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

struct RecursiveRoot;

impl Component for RecursiveRoot {
    type Input = ();
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        component::<Self>("child", ()).into()
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

fn expand_shared_view(view: &View) -> Declaration {
    struct Root;

    impl Component for Root {
        type Input = View;
        type Message = ();

        fn create(_: &Self::Input, _: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, input: &Self::Input, _: &mut ViewContext<Self>) -> View {
            input.clone()
        }
    }

    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<Root>("root", view.clone())],
    );
    let owner = host.keys[&Key::from("root")];
    host.expand_view(
        owner,
        view.clone(),
        &ContextEnvironment {
            contexts: &HashMap::new(),
            provided: &HashSet::new(),
        },
        0,
        &mut ExpansionState::default(),
    )
    .0
    .object()
}

#[test]
fn native_expansion_preserves_shared_relations_and_attachments() {
    fn assert_shared(before: &Declaration, after: &Declaration) {
        for (before, after) in before.relations.iter().zip(after.relations.iter()) {
            match (&before.value, &after.value) {
                (RelationValue::One(Some(before)), RelationValue::One(Some(after))) => {
                    assert!(Rc::ptr_eq(before, after));
                }
                (RelationValue::Many(before), RelationValue::Many(after)) => {
                    assert!(Rc::ptr_eq(before, after));
                }
                (RelationValue::One(None), RelationValue::One(None)) => {}
                _ => panic!("relation changed during native expansion"),
            }
        }
        if let Some(before) = &before.attachments {
            let after = after.attachments.as_ref().unwrap();
            if let Some(tooltip) = &before.tooltip {
                assert_shared(
                    tooltip.content.as_object(),
                    after.tooltip.as_ref().unwrap().content.as_object(),
                );
            }
            if let Some(flyout) = &before.flyout {
                assert_shared(
                    flyout.content.as_object(),
                    after.flyout.as_ref().unwrap().content.as_object(),
                );
            }
            if let Some(dialog) = &before.content_dialog {
                assert_shared(
                    &dialog.declaration,
                    &after.content_dialog.as_ref().unwrap().declaration,
                );
            }
        }
    }

    let children = || Grid::new().children((TextBlock::new(), Border::new()));
    let views: [View; 2] = [
        children().into(),
        Button::new()
            .content(children())
            .tooltip_with(Tooltip::rich(children()))
            .flyout_with(Flyout::rich(children()))
            .content_dialog(ContentDialog::new().content(children())),
    ];
    for view in views {
        let expanded = expand_shared_view(&view);
        assert_shared(view.0.as_object(), &expanded);
        assert!(view.0.as_object().component.is_none());
        assert!(expanded.component.is_some());
        let mut expected = view.0.as_object().clone();
        expected.component = expanded.component;
        assert_eq!(expanded, expected);
    }
}

#[test]
fn expansion_copies_changed_paths_without_mutating_shared_input() {
    let view: View = Grid::new()
        .children((
            Border::new().content(TextBlock::new().text("shared")),
            Border::new().content(component::<Label>("child", Rc::from("expanded"))),
            ItemsRepeater::new().virtual_source(VirtualSource::new(0, 1, Key::from, |_| {
                TextBlock::new().text("row")
            })),
            Border::new().content(TextBlock::new().text("shared after changes")),
        ))
        .into();
    let expanded = expand_shared_view(&view);
    let children = |declaration: &Declaration| {
        let RelationValue::Many(children) = &declaration.relations.as_slice()[0].value else {
            panic!("expected children");
        };
        Rc::clone(children)
    };
    let content = |node: &DeclaredNode| {
        let RelationValue::One(Some(content)) = &node.as_object().relations.as_slice()[0].value
        else {
            panic!("expected content");
        };
        Rc::clone(content)
    };
    let before = children(view.0.as_object());
    let after = children(&expanded);
    assert!(!Rc::ptr_eq(&before, &after));
    assert!(Rc::ptr_eq(&content(&before[0]), &content(&after[0])));
    assert!(Rc::ptr_eq(&content(&before[3]), &content(&after[3])));
    assert!(matches!(
        content(&before[1]).as_ref(),
        DeclaredNode::Component { .. }
    ));
    assert_eq!(content(&after[1]).as_object().kind, ObjectType::TextBlock);
    assert!(
        before[2]
            .as_object()
            .virtual_items
            .as_ref()
            .unwrap()
            .owner
            .is_none()
    );
    assert_eq!(
        after[2].as_object().virtual_items.as_ref().unwrap().owner,
        expanded.component
    );
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
    );
    let counter = host.reference(&Key::from("counter")).unwrap();
    let label = host.reference(&Key::from("label")).unwrap();
    let counter_root = counter.get().unwrap();
    let label_root = label.get().unwrap();

    let mutations = host.update_input::<Label>(&Key::from("label"), Rc::from("Changed"));
    assert_eq!(mutations.len(), 1);
    assert_eq!(counter.get(), Some(counter_root));
    assert_eq!(label.get(), Some(label_root));

    let sender = host.sender::<Counter>(&Key::from("counter")).unwrap();
    assert!(sender.send(1));
    assert_eq!(host.drain(1).mutations, 1);
    assert_eq!(cleanup.load(Ordering::Relaxed), 1);

    assert!(sender.send(0));
    assert_eq!(host.drain(1).mutations, 0);
    assert_eq!(cleanup.load(Ordering::Relaxed), 1);
}

#[test]
fn tooltip_attachment_expands_across_component_boundary() {
    let host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<TooltipParent>("parent", ())],
    );
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
    );
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
    );
    let switch = host.sender::<RootSwitch>(&Key::from("switch")).unwrap();
    let label = host.sender::<Label>(&Key::from("label")).unwrap();
    let switch_reference = host.reference(&Key::from("switch")).unwrap();
    let label_reference = host.reference(&Key::from("label")).unwrap();
    let switch_root = switch_reference.get();
    let label_root = label_reference.get();
    host.runtime.adapter_mut().record_batches(true);

    assert!(switch.send(()));
    assert_eq!(host.drain(1).dispatched, 1);
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
    assert_eq!(host.drain(1).dispatched, 1);
    assert_eq!(label_reference.get(), label_root);
}

#[test]
fn update_uses_input_in_message_order() {
    let seen = Rc::new(RefCell::new(Vec::new()));
    let before = {
        let seen = Rc::clone(&seen);
        Callback::new(move |value| seen.borrow_mut().push(("before", value)))
    };
    let after = {
        let seen = Rc::clone(&seen);
        Callback::new(move |value| seen.borrow_mut().push(("after", value)))
    };
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<InputReceiver>("root", (1, before))],
    );
    let sender = host.sender::<InputReceiver>(&Key::from("root")).unwrap();
    assert!(sender.send(()));
    host.update_root(component::<InputReceiver>("root", (2, after)));
    assert!(sender.send(()));
    host.drain(10);
    assert_eq!(*seen.borrow(), [("before", 1), ("after", 2)]);
}

#[test]
fn retained_child_update_uses_replaced_input_callback() {
    let before = Rc::new(Cell::new(0));
    let after = Rc::new(Cell::new(0));
    let callback = {
        let before = Rc::clone(&before);
        Callback::new(move |value| before.set(value))
    };
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<InputParent>("parent", (1, callback))],
    );
    let path = [Key::from("parent"), Key::from("child")];
    let sender = host.sender_at::<InputReceiver>(&path).unwrap();
    assert!(sender.send(()));
    host.drain(1);
    assert_eq!(before.get(), 1);
    let callback = {
        let after = Rc::clone(&after);
        Callback::new(move |value| after.set(value))
    };
    host.update_input::<InputParent>(&Key::from("parent"), (2, callback));
    assert!(sender.send(()));
    host.drain(1);
    assert_eq!(before.get(), 1);
    assert_eq!(after.get(), 2);
}

#[test]
fn nested_component_updates_through_its_retained_owner() {
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<NestedRoot>("parent", (0.0, true))],
    );
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
    assert_eq!(host.drain(1).dispatched, 1);
    assert_eq!(reference.get(), Some(root));
    assert_eq!(host.runtime().graph().kind(root), Some(ObjectType::Border));
    assert_eq!(host.runtime().graph().owner(root), Some(parent));
}

#[test]
fn cached_child_reconciles_native_changes_without_rerendering() {
    struct Editor;

    impl Component for Editor {
        type Input = Rc<Cell<usize>>;
        type Message = ();

        fn create(_: &Self::Input, _: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, renders: &Self::Input, _: &mut ViewContext<Self>) -> View {
            renders.set(renders.get() + 1);
            TextBox::new().text("declared").into()
        }
    }

    struct Parent;

    impl Component for Parent {
        type Input = Rc<Cell<usize>>;
        type Message = ();

        fn create(_: &Self::Input, _: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, renders: &Self::Input, _: &mut ViewContext<Self>) -> View {
            Border::new()
                .content(component::<Editor>("editor", Rc::clone(renders)))
                .into()
        }
    }

    let renders = Rc::new(Cell::new(0));
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<Parent>("parent", Rc::clone(&renders))],
    );
    let sender = host.sender::<Parent>(&Key::from("parent")).unwrap();
    let editor = host
        .reference_at(&[Key::from("parent"), Key::from("editor")])
        .unwrap()
        .get()
        .unwrap();
    let render = |host: &mut ComponentHost<RecordingAdapter>| {
        work::measure_work(|| {
            assert!(sender.send(()));
            host.drain(1)
        })
    };
    let (unchanged, work) = render(&mut host);
    assert_eq!(unchanged.mutations, 0);
    assert_eq!(work.views, 1);
    assert_eq!(work.reconciled, 0);

    host.test_adapter_mut().observe(Observation::SetProperty {
        object: editor,
        property: Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("native edit")),
        },
    });
    let (changed, work) = render(&mut host);
    assert_eq!(changed.mutations, 1);
    assert_eq!(work.views, 1);
    assert!(work.reconciled > 0);
    assert_eq!(renders.get(), 1);
    assert!(
        host.runtime()
            .graph()
            .properties(editor)
            .unwrap()
            .contains(&Property {
                id: PropertyId::Text,
                value: PropertyValue::String(Rc::from("declared")),
            })
    );
}

#[test]
fn parent_rerender_preserves_and_retires_nested_scope() {
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<NestedRoot>("parent", (0.0, true))],
    );
    let path = [Key::from("parent"), Key::from("child")];
    let sender = host.sender_at::<RootSwitch>(&path).unwrap();
    let reference = host.reference_at(&path).unwrap();
    let child = reference.get();

    host.update_input::<NestedRoot>(&Key::from("parent"), (12.0, true));
    assert_eq!(reference.get(), child);
    assert!(host.sender_at::<RootSwitch>(&path).is_some());

    assert!(sender.send(()));
    host.update_input::<NestedRoot>(&Key::from("parent"), (12.0, false));
    assert_eq!(reference.get(), None);
    assert!(host.sender_at::<RootSwitch>(&path).is_none());
    assert_eq!(host.drain(1).dropped, 1);

    host.update_input::<NestedRoot>(&Key::from("parent"), (24.0, true));
    assert_eq!(host.scopes.len(), 2);
    let replacement = host.sender_at::<RootSwitch>(&path).unwrap();
    assert!(sender.send(()));
    assert_eq!(host.drain(1).dropped, 1);
    assert!(replacement.send(()));
    assert_eq!(host.drain(1).dispatched, 1);
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
    );
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
    );
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
    );
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
    assert_eq!(host.drain(1).dispatched, 1);
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
    );
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
    assert_eq!(host.drain(1).dispatched, 1);
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
    );
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
    );
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
    assert_eq!(host.drain(1).dispatched, 1);
    let after = host
        .runtime()
        .graph()
        .child(parent, RelationId::Content)
        .unwrap();
    assert_ne!(before, after);
}

#[test]
#[should_panic(expected = "duplicate key")]
fn duplicate_nested_keys_panic() {
    ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<DuplicateNested>("parent", ())],
    );
}

#[test]
#[should_panic(expected = "does not match relation key")]
fn component_and_relation_keys_cannot_diverge() {
    ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<MismatchedKey>("parent", ())],
    );
}

#[test]
#[should_panic(expected = "declaration exceeds the maximum depth")]
fn recursive_component_expansion_enforces_depth_limit() {
    ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<RecursiveComponent>("root", MAX_DEPTH + 2)],
    );
}

#[test]
#[should_panic(expected = "declaration exceeds the maximum depth")]
fn recursive_root_component_enforces_depth_limit() {
    ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<RecursiveRoot>("root", ())],
    );
}

#[test]
fn tree_node_component_content_survives_structural_reorder() {
    let mut host = ComponentHost::mount(
        RecordingAdapter::default(),
        [component::<TreeComponents>("tree", ())],
    );
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
    assert_eq!(host.drain(1).dispatched, 1);
    assert_eq!(host.reference_at(&first_path).unwrap().get(), Some(first));

    assert!(
        host.sender::<TreeComponents>(&Key::from("tree"))
            .unwrap()
            .send(())
    );
    assert_eq!(host.drain(1).dispatched, 1);
    assert_eq!(host.reference_at(&first_path).unwrap().get(), Some(first));
    assert_eq!(host.reference_at(&second_path).unwrap().get(), Some(second));

    assert!(host.sender_at::<TreeContent>(&first_path).unwrap().send(()));
    assert_eq!(host.drain(1).dispatched, 1);
    assert_eq!(host.reference_at(&first_path).unwrap().get(), Some(first));
}
