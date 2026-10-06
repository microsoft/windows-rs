use super::*;

#[derive(Default)]
struct NeverCalledAdapter;

impl Adapter for NeverCalledAdapter {
    type Error = ();

    fn validate(&self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        panic!("invalid declarations must not reach the adapter")
    }

    fn apply(&mut self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        panic!("invalid declarations must not reach the adapter")
    }

    fn focus(&mut self, _object: ObjectId) -> Result<bool, Self::Error> {
        panic!("invalid declarations must not reach the adapter")
    }
}

#[test]
fn excessive_depth_is_rejected_before_reconciliation() {
    let mut child: View = TextBlock::new().text("leaf").into();
    for _ in 0..10_000 {
        child = Border::new().content(child).into();
    }
    let mut runtime = Runtime::new(NeverCalledAdapter);

    assert_eq!(
        runtime.update(child),
        Err(UpdateError::Graph(GraphError::DepthExceeded))
    );
}

#[test]
fn shared_declarations_cannot_expand_beyond_the_graph_limit() {
    let mut child: View = TextBlock::new().text("leaf").into();
    for _ in 0..17 {
        child = StackPanel::new().children([child.clone(), child]).into();
    }
    let mut runtime = Runtime::new(NeverCalledAdapter);

    assert_eq!(
        runtime.update(child),
        Err(UpdateError::Graph(GraphError::SizeExceeded))
    );
}

#[derive(Default)]
struct ApplyOnceAdapter {
    applied: bool,
}

impl Adapter for ApplyOnceAdapter {
    type Error = ();

    fn validate(&self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn apply(&mut self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        if self.applied {
            Err(())
        } else {
            self.applied = true;
            Ok(())
        }
    }

    fn focus(&mut self, _object: ObjectId) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

#[test]
fn root_type_changes_are_rejected_before_native_mutation() {
    let mut runtime = Runtime::new(ApplyOnceAdapter::default());
    runtime.update(Border::new()).unwrap();

    assert_eq!(
        runtime.update(Grid::new()),
        Err(UpdateError::Graph(GraphError::RootTypeChanged {
            previous: ObjectType::Border,
            next: ObjectType::Grid,
        }))
    );
}

#[derive(Default)]
struct RejectingAdapter;

impl Adapter for RejectingAdapter {
    type Error = ();

    fn validate(&self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        Err(())
    }

    fn apply(&mut self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        panic!("an invalid plan must not be applied")
    }

    fn focus(&mut self, _object: ObjectId) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

#[test]
fn adapter_validation_failure_poisons_the_runtime() {
    let mut runtime = Runtime::new(RejectingAdapter);

    assert_eq!(
        runtime.update(Border::new().content(TextBlock::new().text("Text"))),
        Err(UpdateError::Adapter(()))
    );
    assert_eq!(runtime.update(Border::new()), Err(UpdateError::Poisoned));
}

#[derive(Default)]
struct FailingApplyAdapter;

impl Adapter for FailingApplyAdapter {
    type Error = ();

    fn validate(&self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn apply(&mut self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        Err(())
    }

    fn focus(&mut self, _object: ObjectId) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

#[test]
fn adapter_apply_failure_poisons_the_runtime() {
    let mut runtime = Runtime::new(FailingApplyAdapter);

    assert_eq!(
        runtime.update(Border::new().content(TextBlock::new().text("Text"))),
        Err(UpdateError::Adapter(()))
    );
    assert_eq!(runtime.update(Border::new()), Err(UpdateError::Poisoned));
    assert_eq!(runtime.graph().root(), None);
}

#[test]
fn graph_transaction_drop_rolls_back_during_unwind() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(TextBox::new("Before")).unwrap();
    let root = runtime.graph().root().unwrap();
    let mut graph = runtime.graph().clone();
    let before = graph.clone();

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        panic_during_transaction(&mut graph, root);
    }));

    assert!(result.is_err());
    assert_eq!(graph, before);
}

#[test]
fn before_apply_unwind_rolls_back_and_keeps_runtime_usable() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(TextBox::new("Before")).unwrap();
    let root = runtime.graph().root().unwrap();
    let before = runtime.graph().clone();

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        runtime
            .update_subtree_before_apply(root, TextBox::new("Changed"), || {
                panic!("test before_apply unwind");
            })
            .unwrap();
    }));

    assert!(result.is_err());
    assert_eq!(runtime.graph(), &before);
    runtime.update_subtree(root, TextBox::new("After")).unwrap();
}

#[derive(Default)]
struct PanicAdapter {
    inner: RecordingAdapter,
    panic_validate: bool,
    panic_apply: bool,
}

impl Adapter for PanicAdapter {
    type Error = AdapterError;

    fn preview_native_events(&self, events: &mut Vec<NativeEvent>) {
        self.inner.preview_native_events(events);
    }

    fn pop_native_event(&mut self) -> Option<NativeEvent> {
        self.inner.pop_native_event()
    }

    fn validate(&self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        assert!(!self.panic_validate, "test adapter validation unwind");
        self.inner.validate(mutations)
    }

    fn apply(&mut self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        if self.panic_apply {
            if let Some(mutation) = mutations.first() {
                self.inner.apply(std::slice::from_ref(mutation))?;
            }
            panic!("test adapter application unwind");
        }
        self.inner.apply(mutations)
    }

    fn focus(&mut self, object: ObjectId) -> Result<bool, Self::Error> {
        self.inner.focus(object)
    }
}

#[test]
fn adapter_validation_unwind_rolls_back_and_poisons_runtime() {
    let mut runtime = Runtime::new(PanicAdapter {
        panic_validate: true,
        ..Default::default()
    });

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        runtime.update(TextBox::new("Text")).unwrap();
    }));

    assert!(result.is_err());
    assert_eq!(runtime.graph().root(), None);
    assert_eq!(
        runtime.update(TextBox::new("Text")),
        Err(UpdateError::Poisoned)
    );
}

#[test]
fn adapter_application_unwind_rolls_back_and_poisons_runtime() {
    let mut runtime = Runtime::new(PanicAdapter {
        panic_apply: true,
        ..Default::default()
    });

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        runtime.update(TextBox::new("Text")).unwrap();
    }));

    assert!(result.is_err());
    assert_eq!(runtime.graph().root(), None);
    assert_eq!(
        runtime.update(TextBox::new("Text")),
        Err(UpdateError::Poisoned)
    );
}

fn referenced_virtual_view(nested: &ElementRef, row: &ElementRef) -> Grid {
    Grid::new().keyed_children([
        keyed("button", Button::new().element_ref(nested)),
        keyed(
            "repeater",
            ItemsRepeater::new().item("row", Button::new().element_ref(row)),
        ),
    ])
}

#[derive(Default)]
struct ControlledFailureAdapter {
    inner: RecordingAdapter,
    fail_validate: bool,
    fail_apply: bool,
}

impl Adapter for ControlledFailureAdapter {
    type Error = ();

    fn preview_native_events(&self, events: &mut Vec<NativeEvent>) {
        self.inner.preview_native_events(events);
    }

    fn pop_native_event(&mut self) -> Option<NativeEvent> {
        self.inner.pop_native_event()
    }

    fn validate(&self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        if self.fail_validate {
            Err(())
        } else {
            self.inner.validate(mutations).map_err(|_| ())
        }
    }

    fn apply(&mut self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        if self.fail_apply {
            Err(())
        } else {
            self.inner.apply(mutations).map_err(|_| ())
        }
    }

    fn focus(&mut self, object: ObjectId) -> Result<bool, Self::Error> {
        self.inner.focus(object).map_err(|_| ())
    }
}

fn realize_controlled_virtual_row(
    runtime: &mut Runtime<ControlledFailureAdapter>,
    collection: ObjectId,
) -> Result<(), UpdateError<()>> {
    runtime
        .adapter_mut()
        .inner
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    runtime.dispatch_native_events().map(|_| ())
}

#[test]
fn adapter_failures_clear_all_retained_references_before_discard() {
    for fail_validate in [true, false] {
        let nested: ElementRef = ElementRef::default();
        let row: ElementRef = ElementRef::default();
        let mut runtime = Runtime::new(ControlledFailureAdapter::default());
        runtime
            .update(referenced_virtual_view(&nested, &row))
            .unwrap();
        let root = runtime.graph().root().unwrap();
        let collection = runtime
            .graph()
            .children(root, RelationId::Children)
            .unwrap()[1];
        realize_controlled_virtual_row(&mut runtime, collection).unwrap();
        assert!(nested.get().is_some());
        assert!(row.get().is_some());

        runtime.adapter_mut().fail_validate = fail_validate;
        runtime.adapter_mut().fail_apply = !fail_validate;
        assert!(matches!(
            runtime.update(referenced_virtual_view(&nested, &row).width(1.0)),
            Err(UpdateError::Adapter(_))
        ));
        assert_eq!(nested.get(), None);
        assert_eq!(row.get(), None);
        assert_eq!(
            runtime.update(referenced_virtual_view(&nested, &row)),
            Err(UpdateError::Poisoned)
        );
    }
}

fn referenced_failure_subtree(
    primary: Option<(&'static str, &ElementRef)>,
    row: &ElementRef,
) -> Grid {
    let mut children = Vec::new();
    if let Some((key, reference)) = primary {
        children.push(keyed(key, Button::new().element_ref(reference)));
    }
    children.push(keyed(
        "repeater",
        ItemsRepeater::new().item("row", Button::new().element_ref(row)),
    ));
    Grid::new().keyed_children(children)
}

#[test]
fn subtree_adapter_failures_clear_detached_virtual_and_unrelated_references() {
    for fail_validate in [true, false] {
        let nested: ElementRef = ElementRef::default();
        let reused: ElementRef = ElementRef::default();
        let row: ElementRef = ElementRef::default();
        let outside: ElementRef = ElementRef::default();
        let replacement: ElementRef = ElementRef::default();
        let mut runtime = Runtime::new(ControlledFailureAdapter::default());
        runtime
            .update(Grid::new().keyed_children([
                keyed(
                    "target",
                    referenced_failure_subtree(Some(("nested", &nested)), &row),
                ),
                keyed("outside", Button::new().element_ref(&outside)),
            ]))
            .unwrap();
        let root = runtime.graph().root().unwrap();
        let children = runtime
            .graph()
            .children(root, RelationId::Children)
            .unwrap();
        let target = children[0];
        let collection = runtime
            .graph()
            .children(target, RelationId::Children)
            .unwrap()[1];
        realize_controlled_virtual_row(&mut runtime, collection).unwrap();
        let nested_id = nested.get().unwrap();

        runtime
            .update_subtree(target, referenced_failure_subtree(None, &row))
            .unwrap();
        assert_eq!(nested.get(), None);
        runtime
            .update_subtree(
                target,
                referenced_failure_subtree(Some(("reused", &reused)), &row),
            )
            .unwrap();
        let reused_id = reused.get().unwrap();
        assert_eq!(reused_id.index(), nested_id.index());
        assert_ne!(reused_id.generation(), nested_id.generation());
        assert!(row.get().is_some());
        assert!(outside.get().is_some());

        runtime.adapter_mut().fail_validate = fail_validate;
        runtime.adapter_mut().fail_apply = !fail_validate;
        assert!(matches!(
            runtime.update_subtree(
                target,
                Grid::new().keyed_children([keyed(
                    "replacement",
                    Button::new().element_ref(&replacement),
                )])
            ),
            Err(UpdateError::Adapter(_))
        ));
        assert_eq!(nested.get(), None);
        assert_eq!(reused.get(), None);
        assert_eq!(row.get(), None);
        assert_eq!(outside.get(), None);
        assert_eq!(replacement.get(), None);
        assert_eq!(runtime.graph().root(), None);
        assert_eq!(runtime.update(Grid::new()), Err(UpdateError::Poisoned));
    }
}

#[test]
fn adapter_unwinds_clear_all_retained_references() {
    for panic_validate in [true, false] {
        let nested: ElementRef = ElementRef::default();
        let row: ElementRef = ElementRef::default();
        let mut runtime = Runtime::new(PanicAdapter::default());
        runtime
            .update(referenced_virtual_view(&nested, &row))
            .unwrap();
        let root = runtime.graph().root().unwrap();
        let collection = runtime
            .graph()
            .children(root, RelationId::Children)
            .unwrap()[1];
        runtime
            .adapter_mut()
            .inner
            .queue_realization(RealizationRequest::Realize {
                collection,
                container: RealizedContainer(1),
                index: 0,
                source_revision: 0,
            });
        runtime.dispatch_native_events().unwrap();

        runtime.adapter_mut().panic_validate = panic_validate;
        runtime.adapter_mut().panic_apply = !panic_validate;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            runtime
                .update(referenced_virtual_view(&nested, &row).width(1.0))
                .unwrap();
        }));

        assert!(result.is_err());
        assert_eq!(nested.get(), None);
        assert_eq!(row.get(), None);
        assert_eq!(
            runtime.update(referenced_virtual_view(&nested, &row)),
            Err(UpdateError::Poisoned)
        );
    }
}

#[derive(Default)]
struct CompletionFailAdapter {
    inner: RecordingAdapter,
    fail_completion: bool,
}

impl Adapter for CompletionFailAdapter {
    type Error = ();

    fn preview_native_events(&self, events: &mut Vec<NativeEvent>) {
        self.inner.preview_native_events(events);
    }

    fn pop_native_event(&mut self) -> Option<NativeEvent> {
        self.inner.pop_native_event()
    }

    fn validate(&self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        self.inner.validate(mutations).map_err(|_| ())
    }

    fn apply(&mut self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        if self.fail_completion
            && mutations
                .iter()
                .any(|mutation| matches!(mutation, Mutation::CompleteRetirement { .. }))
        {
            return Err(());
        }
        self.inner.apply(mutations).map_err(|_| ())
    }

    fn focus(&mut self, object: ObjectId) -> Result<bool, Self::Error> {
        self.inner.focus(object).map_err(|_| ())
    }
}

#[test]
fn retirement_completion_errors_poison_the_runtime() {
    let mut runtime = Runtime::new(CompletionFailAdapter::default());
    runtime
        .update(Grid::new().keyed_children([keyed(
            "retiring",
            Button::new().exit_fade(Duration::from_millis(100)),
        )]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let retiring = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    runtime.update(Grid::new()).unwrap();
    runtime.adapter_mut().fail_completion = true;
    assert!(runtime.adapter_mut().inner.complete_retirement(retiring));

    assert_eq!(
        runtime.dispatch_native_events(),
        Err(UpdateError::Adapter(()))
    );
    assert_eq!(runtime.update(Grid::new()), Err(UpdateError::Poisoned));
}
