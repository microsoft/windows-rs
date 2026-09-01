//! Virtualization and realization contract tests for [`Pump`].

use super::super::*;
use super::support::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Clone)]
struct RowInput {
    changed: Rc<Cell<u32>>,
    created: Rc<Cell<u32>>,
    label: String,
    log: Rc<RefCell<Vec<String>>>,
}

impl PartialEq for RowInput {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
            && Rc::ptr_eq(&self.changed, &other.changed)
            && Rc::ptr_eq(&self.created, &other.created)
            && Rc::ptr_eq(&self.log, &other.log)
    }
}

struct ComponentRow;

impl Component for ComponentRow {
    type Message = ();
    type Input = RowInput;

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        input.created.set(input.created.get() + 1);
        Self
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        input.changed.set(input.changed.get() + 1);
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let label = input.label.clone();
        let setup = label.clone();
        let log = Rc::clone(&input.log);
        context.use_effect("row", label, move || {
            log.borrow_mut().push(format!("setup {setup}"));
            Some(Box::new(move || {
                log.borrow_mut().push(format!("cleanup {setup}"));
            }))
        });
        TextBlock::new().text(input.label.clone()).into()
    }
}

struct ContextRow;

impl Component for ContextRow {
    type Message = ();
    type Input = Context<String>;

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, context: &Self::Input, view: &mut ViewContext<Self>) -> View {
        TextBlock::new().text(view.use_context(context)).into()
    }
}

struct ReplacingRow;

impl Component for ReplacingRow {
    type Message = ();
    type Input = (bool, Rc<Cell<u32>>);

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        input.1.set(input.1.get() + 1);
        Self
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        if input.0 {
            Button::new().into()
        } else {
            TextBlock::new().into()
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RowShape {
    Empty,
    One,
    Two,
}

#[derive(Clone)]
struct ShapeRowInput {
    cleanups: Rc<Cell<u32>>,
    created: Rc<Cell<u32>>,
    first: ElementRef<TextBox>,
    second: ElementRef<TextBox>,
    sender: Rc<RefCell<Option<LocalSender<RowShape>>>>,
    setups: Rc<Cell<u32>>,
    shape: RowShape,
}

impl PartialEq for ShapeRowInput {
    fn eq(&self, other: &Self) -> bool {
        self.shape == other.shape
            && Rc::ptr_eq(&self.cleanups, &other.cleanups)
            && Rc::ptr_eq(&self.created, &other.created)
            && Rc::ptr_eq(&self.sender, &other.sender)
            && Rc::ptr_eq(&self.setups, &other.setups)
    }
}

impl ShapeRowInput {
    fn new(shape: RowShape) -> Self {
        Self {
            cleanups: Rc::new(Cell::new(0)),
            created: Rc::new(Cell::new(0)),
            first: ElementRef::new(),
            second: ElementRef::new(),
            sender: Rc::new(RefCell::new(None)),
            setups: Rc::new(Cell::new(0)),
            shape,
        }
    }

    fn with_shape(&self, shape: RowShape) -> Self {
        Self {
            shape,
            ..self.clone()
        }
    }

    fn send(&self, shape: RowShape) {
        assert!(self.sender.borrow().as_ref().unwrap().send(shape));
    }
}

struct ShapeRow {
    shape: RowShape,
}

impl Component for ShapeRow {
    type Message = RowShape;
    type Input = ShapeRowInput;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        input.created.set(input.created.get() + 1);
        *input.sender.borrow_mut() = Some(context.sender());
        Self { shape: input.shape }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.shape = input.shape;
    }

    fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
        self.shape = message;
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let cleanups = Rc::clone(&input.cleanups);
        let setups = Rc::clone(&input.setups);
        context.use_effect("shape-row", (), move || {
            setups.set(setups.get() + 1);
            Some(Box::new(move || cleanups.set(cleanups.get() + 1)))
        });
        match self.shape {
            RowShape::Empty => View::empty(),
            RowShape::One => TextBox::new().element_ref(&input.first).into(),
            RowShape::Two => View::fragment((
                TextBox::new().element_ref(&input.first),
                TextBox::new().element_ref(&input.second),
            )),
        }
    }
}

fn row_input(label: &str) -> RowInput {
    RowInput {
        changed: Rc::new(Cell::new(0)),
        created: Rc::new(Cell::new(0)),
        label: label.to_string(),
        log: Rc::new(RefCell::new(Vec::new())),
    }
}

struct WindowAwareRow;

impl Component for WindowAwareRow {
    type Message = ();
    type Input = Rc<Cell<bool>>;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        input.set(context.open_window(TextBlock::new().text("additional window").into()));
        Self
    }

    fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_visuals(WindowVisuals::new().client_size(900.0, 700.0));
        TextBlock::new().text("realized").into()
    }
}

#[test]
fn realization_publishes_window_visuals_and_host_requests() {
    let opened = Rc::new(Cell::new(false));
    let visuals = WindowVisuals::new().client_size(900.0, 700.0);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        ItemsRepeater::new()
            .item("row", View::component::<WindowAwareRow>(Rc::clone(&opened)))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });

    assert!(matches!(
        pump.process_realizations().unwrap().as_slice(),
        [RealizationOutcome::Realized(_)]
    ));
    assert!(opened.get());
    assert_eq!(pump.runtime().opened_windows().len(), 1);
    assert_eq!(
        pump.runtime().window_visuals(pump.window.unwrap()),
        Some(visuals)
    );
}

#[test]
fn realization_requests_are_checked_against_arena_and_container_generations() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        ItemsRepeater::new()
            .item("a", TextBlock::new().text("A"))
            .item("b", TextBlock::new().text("B"))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    let container = RealizedContainer(7);
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container,
            index: 1,
            source_revision: 0,
        });

    let realized = pump.process_realizations().unwrap();
    let RealizationOutcome::Realized(lease) = &realized[0] else {
        panic!("expected realized lease");
    };
    assert_eq!(lease.key, Key::from("b"));
    assert_eq!(lease.container, container);

    pump.runtime_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container,
            source_revision: 0,
        });
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container,
            source_revision: 0,
        });
    assert!(matches!(
        pump.process_realizations().unwrap().as_slice(),
        [
            RealizationOutcome::Recycled(_),
            RealizationOutcome::Rejected(_)
        ]
    ));

    pump.tree.retire_subtree(collection).unwrap();
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container,
            index: 0,
            source_revision: 0,
        });
    assert_eq!(
        pump.process_realizations().unwrap(),
        [RealizationOutcome::Rejected(RealizationRequest::Realize {
            collection,
            container,
            index: 0,
            source_revision: 0,
        })]
    );
    pump.root = None;
}

#[test]
fn virtual_collection_mounts_without_eager_row_controls() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        ItemsRepeater::new()
            .item("a", TextBlock::new().text("A"))
            .item("b", TextBlock::new().text("B"))
            .into(),
    )
    .unwrap();

    let root = pump.root().unwrap();
    assert_eq!(pump.tree.kind(root), Ok(NodeKind::VirtualCollection));
    assert_eq!(pump.tree.virtual_items(root).unwrap().len(), 2);
    assert!(pump.runtime().commands()[0].iter().any(|command| {
        *command
            == Command::CreateVirtualCollection {
                node: root,
                item_count: 2,
                source_revision: 0,
            }
    }));
    assert!(!pump.runtime().commands()[0].iter().any(|command| {
        matches!(
            command,
            Command::Create {
                kind: MountedKind::TextBlock,
                ..
            }
        )
    }));
}

fn lazy_source(
    revision: u64,
    rows: Rc<Vec<(&'static str, &'static str)>>,
    key_calls: Rc<Cell<usize>>,
    view_calls: Rc<Cell<usize>>,
) -> VirtualSource {
    let key_rows = Rc::clone(&rows);
    VirtualSource::new(
        revision,
        rows.len(),
        move |index| {
            key_calls.set(key_calls.get() + 1);
            key_rows[index].0
        },
        move |index| {
            view_calls.set(view_calls.get() + 1);
            TextBlock::new().text(rows[index].1)
        },
    )
}

#[test]
fn lazy_source_builds_only_realized_views_and_skips_stable_keys() {
    let initial_keys = Rc::new(Cell::new(0));
    let initial_views = Rc::new(Cell::new(0));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        ItemsRepeater::new()
            .virtual_source(lazy_source(
                0,
                Rc::new(vec![("a", "A"), ("b", "B")]),
                Rc::clone(&initial_keys),
                Rc::clone(&initial_views),
            ))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    assert_eq!(initial_keys.get(), 2);
    assert_eq!(initial_views.get(), 0);

    pump.runtime_mut()
        .queue_realize(collection, RealizedContainer(1), 1);
    pump.process_realizations().unwrap();
    assert_eq!(initial_views.get(), 1);
    assert_eq!(
        pump.tree
            .realized(collection, RealizedContainer(1))
            .unwrap()
            .unwrap()
            .index,
        1
    );

    let update_keys = Rc::new(Cell::new(0));
    let update_views = Rc::new(Cell::new(0));
    pump.update_view(
        ItemsRepeater::new()
            .virtual_source(lazy_source(
                0,
                Rc::new(vec![("a", "A2"), ("b", "B2")]),
                Rc::clone(&update_keys),
                Rc::clone(&update_views),
            ))
            .into(),
    )
    .unwrap();
    assert_eq!(update_keys.get(), 0);
    assert_eq!(update_views.get(), 1);

    let reset_keys = Rc::new(Cell::new(0));
    let reset_views = Rc::new(Cell::new(0));
    pump.update_view(
        ItemsRepeater::new()
            .virtual_source(lazy_source(
                1,
                Rc::new(vec![("b", "B3"), ("a", "A3")]),
                Rc::clone(&reset_keys),
                Rc::clone(&reset_views),
            ))
            .into(),
    )
    .unwrap();
    assert_eq!(reset_keys.get(), 2);
    assert_eq!(reset_views.get(), 0);
    assert!(pump.tree.children(collection).unwrap().is_empty());
}

#[test]
fn component_row_is_created_lazily_and_recycle_cleans_it_once() {
    let input = row_input("A");
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        ItemsRepeater::new()
            .item("a", View::component::<ComponentRow>(input.clone()))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    let version = pump.version();
    assert_eq!(input.created.get(), 0);
    assert!(input.log.borrow().is_empty());

    pump.runtime_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    pump.process_realizations().unwrap();

    let row = pump
        .tree
        .realized(collection, RealizedContainer(1))
        .unwrap()
        .unwrap();
    assert_eq!(pump.tree.kind(row.logical_root), Ok(NodeKind::Component));
    assert_ne!(Some(row.logical_root), row.native_root);
    assert_eq!(input.created.get(), 1);
    assert_eq!(&*input.log.borrow(), &["setup A"]);
    assert_eq!(pump.version(), version);

    pump.runtime_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(1),
            source_revision: 0,
        });
    pump.process_realizations().unwrap();
    assert_eq!(&*input.log.borrow(), &["setup A", "cleanup A"]);
    assert_eq!(pump.version(), version);

    pump.shutdown();
    assert_eq!(&*input.log.borrow(), &["setup A", "cleanup A"]);
}

#[test]
fn equal_virtual_source_retries_a_dirty_realized_component() {
    let input = row_input("A");
    let source = || {
        StackPanel::new().children((
            ItemsRepeater::new().item("a", View::component::<ComponentRow>(input.clone())),
        ))
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(source()).unwrap();
    let collection = pump.tree.children(pump.root().unwrap()).unwrap()[0];
    pump.runtime_mut()
        .queue_realize(collection, RealizedContainer(1), 0);
    pump.process_realizations().unwrap();
    let row = pump
        .tree
        .realized(collection, RealizedContainer(1))
        .unwrap()
        .unwrap();
    let token = pump
        .components
        .token(pump.tree.component_scope(row.logical_root).unwrap());
    pump.planning_dirty.insert(token);

    pump.update_view(source()).unwrap();

    assert!(pump.planning_dirty.is_empty());
}

#[test]
fn row_effect_setup_and_cleanup_straddle_native_publication() {
    struct OrderedRuntime {
        inner: RecordingRuntime,
        log: Rc<RefCell<Vec<String>>>,
    }

    impl NativeRuntime for OrderedRuntime {
        fn apply(&mut self, commands: &[Command]) -> Result<(), NativeApplyError> {
            self.log.borrow_mut().push("native".to_string());
            self.inner.apply(commands)
        }

        fn reset(&mut self) {
            self.inner.reset();
        }

        fn set_identity(&mut self, identity: WindowToken) {
            self.inner.set_identity(identity);
        }

        fn drain_realizations(&mut self) -> Vec<NativeWork<RealizationRequest>> {
            self.inner.drain_realizations()
        }
    }

    let input = row_input("A");
    let mut pump = Pump::new(OrderedRuntime {
        inner: RecordingRuntime::default(),
        log: Rc::clone(&input.log),
    });
    pump.mount_view(
        ItemsRepeater::new()
            .item("a", View::component::<ComponentRow>(input.clone()))
            .into(),
    )
    .unwrap();
    input.log.borrow_mut().clear();
    let collection = pump.root().unwrap();
    pump.runtime_mut()
        .inner
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });

    pump.process_realizations().unwrap();
    assert_eq!(&*input.log.borrow(), &["native", "setup A"]);

    input.log.borrow_mut().clear();
    pump.runtime_mut()
        .inner
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(1),
            source_revision: 0,
        });
    pump.process_realizations().unwrap();
    assert_eq!(&*input.log.borrow(), &["cleanup A", "native"]);
}

#[test]
fn key_stable_source_update_reconciles_component_row_in_place() {
    let first = row_input("A");
    let second = RowInput {
        changed: Rc::clone(&first.changed),
        created: Rc::clone(&first.created),
        label: "B".to_string(),
        log: Rc::clone(&first.log),
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        ItemsRepeater::new()
            .item("row", View::component::<ComponentRow>(first.clone()))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    pump.process_realizations().unwrap();
    let before = pump
        .tree
        .realized(collection, RealizedContainer(1))
        .unwrap()
        .unwrap();

    pump.update_view(
        ItemsRepeater::new()
            .item("row", View::component::<ComponentRow>(second))
            .into(),
    )
    .unwrap();

    let after = pump
        .tree
        .realized(collection, RealizedContainer(1))
        .unwrap()
        .unwrap();
    assert_eq!(after.logical_root, before.logical_root);
    assert_eq!(after.native_root, before.native_root);
    assert_eq!(first.created.get(), 1);
    assert_eq!(first.changed.get(), 1);
    assert_eq!(&*first.log.borrow(), &["setup A", "cleanup A", "setup B"]);
    assert_eq!(recorded_text(pump.runtime(), collection), ["B"]);
}

#[test]
fn key_stable_component_row_can_replace_its_native_root() {
    let created = Rc::new(Cell::new(0));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        ItemsRepeater::new()
            .item(
                "row",
                View::component::<ReplacingRow>((false, Rc::clone(&created))),
            )
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    pump.process_realizations().unwrap();
    let before = pump
        .tree
        .realized(collection, RealizedContainer(1))
        .unwrap()
        .unwrap();

    pump.update_view(
        ItemsRepeater::new()
            .item(
                "row",
                View::component::<ReplacingRow>((true, Rc::clone(&created))),
            )
            .into(),
    )
    .unwrap();

    let after = pump
        .tree
        .realized(collection, RealizedContainer(1))
        .unwrap()
        .unwrap();
    assert_eq!(after.logical_root, before.logical_root);
    assert_ne!(after.native_root, before.native_root);
    assert_eq!(created.get(), 1);
    assert!(pump.runtime().node(before.native_root.unwrap()).is_none());
    assert_eq!(
        pump.runtime()
            .node(after.native_root.unwrap())
            .unwrap()
            .kind(),
        Some(MountedKind::Button)
    );
}

#[test]
fn key_stable_fragment_row_can_replace_its_native_root() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        ItemsRepeater::new()
            .item(
                "row",
                View::keyed_fragment([KeyedView::new("inner", TextBlock::new())]),
            )
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    pump.process_realizations().unwrap();
    let before = pump
        .tree
        .realized(collection, RealizedContainer(1))
        .unwrap()
        .unwrap();

    pump.update_view(
        ItemsRepeater::new()
            .item(
                "row",
                View::keyed_fragment([KeyedView::new("inner", Button::new())]),
            )
            .into(),
    )
    .unwrap();

    let after = pump
        .tree
        .realized(collection, RealizedContainer(1))
        .unwrap()
        .unwrap();
    assert_eq!(after.logical_root, before.logical_root);
    assert_ne!(after.native_root, before.native_root);
    assert_eq!(
        pump.runtime()
            .node(after.native_root.unwrap())
            .unwrap()
            .kind(),
        Some(MountedKind::Button)
    );
}

#[test]
fn source_key_removal_retires_realized_component_row() {
    let input = row_input("A");
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        ItemsRepeater::new()
            .item("a", View::component::<ComponentRow>(input.clone()))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    pump.process_realizations().unwrap();

    pump.update_view(ItemsRepeater::new().into()).unwrap();

    assert_eq!(&*input.log.borrow(), &["setup A", "cleanup A"]);
    assert!(pump.tree.children(collection).unwrap().is_empty());
    assert_eq!(pump.tree.virtual_model(collection).unwrap().active_len(), 0);
}

#[test]
fn initial_multi_root_row_is_realized_detached_with_a_diagnostic() {
    let input = row_input("A");
    let invalid = View::fragment((
        View::component::<ComponentRow>(input.clone()),
        TextBlock::new(),
    ));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(ItemsRepeater::new().item("a", invalid).into())
        .unwrap();
    let collection = pump.root().unwrap();
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });

    let outcomes = pump.process_realizations().unwrap();

    assert!(matches!(
        outcomes.as_slice(),
        [RealizationOutcome::Realized(_)]
    ));
    let [logical_root] = pump.tree.children(collection).unwrap() else {
        panic!("expected logical row");
    };
    assert_eq!(
        pump.tree
            .realized(collection, RealizedContainer(1))
            .unwrap()
            .unwrap(),
        RealizedRow {
            index: 0,
            logical_root: *logical_root,
            native_root: None,
        }
    );
    assert!(
        pump.runtime()
            .node(collection)
            .unwrap()
            .children()
            .is_empty()
    );
    assert_eq!(input.created.get(), 1);
    assert_eq!(&*input.log.borrow(), &["setup A"]);
    assert_eq!(
        pump.drain_diagnostics(),
        [PumpDiagnostic::VirtualRowRootCount {
            collection,
            key: Key::from("a"),
            actual: 2,
        }]
    );
    assert!(!pump.runtime().commands().last().unwrap().iter().any(
        |command| matches!(command, Command::AttachRealized { collection: node, .. } if *node == collection)
    ));
    assert!(!pump.native_work_pending());
}

#[test]
fn initial_empty_row_is_realized_logically_without_attachment() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(ItemsRepeater::new().item("empty", View::empty()).into())
        .unwrap();
    let collection = pump.root().unwrap();
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });

    assert!(matches!(
        pump.process_realizations().unwrap().as_slice(),
        [RealizationOutcome::Realized(_)]
    ));
    let [logical_root] = pump.tree.children(collection).unwrap() else {
        panic!("expected logical row");
    };
    assert_eq!(
        pump.tree
            .realized(collection, RealizedContainer(1))
            .unwrap()
            .unwrap(),
        RealizedRow {
            index: 0,
            logical_root: *logical_root,
            native_root: None,
        }
    );
    assert!(
        pump.runtime()
            .node(collection)
            .unwrap()
            .children()
            .is_empty()
    );
    assert!(pump.drain_diagnostics().is_empty());
    assert!(!pump.runtime().commands().last().unwrap().iter().any(
        |command| matches!(command, Command::AttachRealized { collection: node, .. } if *node == collection)
    ));
}

#[test]
fn component_message_one_empty_one_reuses_and_reattaches_row() {
    let input = ShapeRowInput::new(RowShape::One);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        ItemsRepeater::new()
            .item("row", View::component::<ShapeRow>(input.clone()))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    pump.runtime_mut()
        .queue_realize(collection, RealizedContainer(1), 0);
    pump.process_realizations().unwrap();
    let logical_root = pump.tree.children(collection).unwrap()[0];
    assert!(input.first.request_focus());

    input.send(RowShape::Empty);
    assert_eq!(pump.dispatch_components(1), Ok(1));
    let empty = pump
        .tree
        .realized(collection, RealizedContainer(1))
        .unwrap()
        .unwrap();
    assert_eq!(empty.logical_root, logical_root);
    assert_eq!(empty.native_root, None);
    assert!(
        pump.runtime()
            .node(collection)
            .unwrap()
            .children()
            .is_empty()
    );
    assert!(!input.first.request_focus());

    input.send(RowShape::One);
    assert_eq!(pump.dispatch_components(1), Ok(1));
    let one = pump
        .tree
        .realized(collection, RealizedContainer(1))
        .unwrap()
        .unwrap();
    assert_eq!(one.logical_root, logical_root);
    assert!(one.native_root.is_some());
    assert_eq!(pump.runtime().node(collection).unwrap().children().len(), 1);
    assert_eq!(input.created.get(), 1);
    assert_eq!(input.setups.get(), 1);
    assert_eq!(input.cleanups.get(), 0);
    assert!(pump.drain_diagnostics().is_empty());
}

#[test]
fn component_message_one_two_one_preserves_state_and_reports_shape() {
    let input = ShapeRowInput::new(RowShape::One);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        ItemsRepeater::new()
            .item("row", View::component::<ShapeRow>(input.clone()))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    pump.runtime_mut()
        .queue_realize(collection, RealizedContainer(1), 0);
    pump.process_realizations().unwrap();
    let logical_root = pump.tree.children(collection).unwrap()[0];

    input.send(RowShape::Two);
    assert_eq!(pump.dispatch_components(1), Ok(1));
    let two = pump
        .tree
        .realized(collection, RealizedContainer(1))
        .unwrap()
        .unwrap();
    assert_eq!(two.logical_root, logical_root);
    assert_eq!(two.native_root, None);
    assert!(
        pump.runtime()
            .node(collection)
            .unwrap()
            .children()
            .is_empty()
    );
    assert!(input.first.request_focus());
    assert!(input.second.request_focus());
    assert_eq!(
        pump.drain_diagnostics(),
        [PumpDiagnostic::VirtualRowRootCount {
            collection,
            key: Key::from("row"),
            actual: 2,
        }]
    );

    input.send(RowShape::One);
    assert_eq!(pump.dispatch_components(1), Ok(1));
    let one = pump
        .tree
        .realized(collection, RealizedContainer(1))
        .unwrap()
        .unwrap();
    assert_eq!(one.logical_root, logical_root);
    assert!(one.native_root.is_some());
    assert_eq!(pump.runtime().node(collection).unwrap().children().len(), 1);
    assert_eq!(input.created.get(), 1);
    assert_eq!(input.setups.get(), 1);
    assert_eq!(input.cleanups.get(), 0);
    assert!(!input.second.request_focus());
    assert!(pump.drain_diagnostics().is_empty());
}

#[test]
fn same_key_payload_shape_transitions_reuse_the_row_component() {
    let input = ShapeRowInput::new(RowShape::One);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        ItemsRepeater::new()
            .item("row", View::component::<ShapeRow>(input.clone()))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    pump.runtime_mut()
        .queue_realize(collection, RealizedContainer(1), 0);
    pump.process_realizations().unwrap();
    let logical_root = pump.tree.children(collection).unwrap()[0];

    for shape in [RowShape::Empty, RowShape::Two, RowShape::One] {
        pump.update_view(
            ItemsRepeater::new()
                .item("row", View::component::<ShapeRow>(input.with_shape(shape)))
                .into(),
        )
        .unwrap();
        assert_eq!(pump.tree.children(collection).unwrap(), &[logical_root]);
    }

    assert_eq!(input.created.get(), 1);
    assert_eq!(input.setups.get(), 1);
    assert_eq!(input.cleanups.get(), 0);
    assert!(
        pump.tree
            .realized(collection, RealizedContainer(1))
            .unwrap()
            .unwrap()
            .native_root
            .is_some()
    );
    assert_eq!(
        pump.drain_diagnostics(),
        [PumpDiagnostic::VirtualRowRootCount {
            collection,
            key: Key::from("row"),
            actual: 2,
        }]
    );
}

#[test]
fn recycle_and_reset_clean_detached_row_lifetimes_once() {
    let recycled = ShapeRowInput::new(RowShape::Two);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        ItemsRepeater::new()
            .item("row", View::component::<ShapeRow>(recycled.clone()))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    pump.runtime_mut()
        .queue_realize(collection, RealizedContainer(1), 0);
    pump.process_realizations().unwrap();
    assert!(recycled.first.request_focus());
    assert!(recycled.second.request_focus());
    pump.drain_diagnostics();

    pump.runtime_mut()
        .queue_recycle(collection, RealizedContainer(1));
    assert!(matches!(
        pump.process_realizations().unwrap().as_slice(),
        [RealizationOutcome::Recycled(_)]
    ));
    assert_eq!(recycled.cleanups.get(), 1);
    assert!(!recycled.first.request_focus());
    assert!(!recycled.second.request_focus());
    assert!(pump.tree.children(collection).unwrap().is_empty());

    let reset = ShapeRowInput::new(RowShape::Two);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        ItemsRepeater::new()
            .item("row", View::component::<ShapeRow>(reset.clone()))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    pump.runtime_mut()
        .queue_realize(collection, RealizedContainer(1), 0);
    pump.process_realizations().unwrap();
    pump.drain_diagnostics();

    pump.update_view(
        ItemsRepeater::new()
            .item("replacement", TextBlock::new())
            .into(),
    )
    .unwrap();
    assert_eq!(reset.cleanups.get(), 1);
    assert!(!reset.first.request_focus());
    assert!(!reset.second.request_focus());
    assert!(pump.tree.children(collection).unwrap().is_empty());
}

#[test]
fn unrelated_dirty_virtual_rows_publish_with_invalid_shape() {
    let first = ShapeRowInput::new(RowShape::One);
    let second = ShapeRowInput::new(RowShape::One);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        ItemsRepeater::new()
            .item("first", View::component::<ShapeRow>(first.clone()))
            .item("second", View::component::<ShapeRow>(second.clone()))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    pump.runtime_mut()
        .queue_realize(collection, RealizedContainer(1), 0);
    pump.runtime_mut()
        .queue_realize(collection, RealizedContainer(2), 1);
    pump.process_realizations().unwrap();

    first.send(RowShape::Two);
    second.send(RowShape::Empty);
    assert_eq!(pump.dispatch_components(2), Ok(2));

    let rows = pump.tree.children(collection).unwrap();
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|logical| {
        let container = pump
            .tree
            .realized_container_for_logical(collection, *logical)
            .unwrap()
            .unwrap();
        pump.tree
            .realized(collection, container)
            .unwrap()
            .unwrap()
            .native_root
            .is_none()
    }));
    assert_eq!(
        pump.drain_diagnostics(),
        [PumpDiagnostic::VirtualRowRootCount {
            collection,
            key: Key::from("first"),
            actual: 2,
        }]
    );
}

#[test]
fn failed_native_shape_update_does_not_publish_diagnostic() {
    let input = ShapeRowInput::new(RowShape::One);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        ItemsRepeater::new()
            .item("row", View::component::<ShapeRow>(input.clone()))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    pump.runtime_mut()
        .queue_realize(collection, RealizedContainer(1), 0);
    pump.process_realizations().unwrap();
    let published = pump
        .tree
        .realized(collection, RealizedContainer(1))
        .unwrap()
        .unwrap();

    pump.runtime_mut().fail_after(0, 0);
    input.send(RowShape::Two);
    assert!(matches!(
        pump.dispatch_components(1),
        Err(PumpError::NativeApplyFailed(_))
    ));

    assert_eq!(
        pump.tree
            .realized(collection, RealizedContainer(1))
            .unwrap(),
        Some(published)
    );
    assert!(pump.drain_diagnostics().is_empty());
}

#[test]
fn valid_and_empty_realizations_publish_in_one_batch() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        ItemsRepeater::new()
            .item("valid", TextBlock::new().text("valid"))
            .item("invalid", View::empty())
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    for (container, index) in [(RealizedContainer(1), 0), (RealizedContainer(2), 1)] {
        pump.runtime_mut()
            .queue_realization(RealizationRequest::Realize {
                collection,
                container,
                index,
                source_revision: 0,
            });
    }

    assert_eq!(pump.process_realizations().unwrap().len(), 2);
    assert_eq!(pump.tree.children(collection).unwrap().len(), 2);
    assert_eq!(recorded_text(pump.runtime(), collection), ["valid"]);
    assert!(pump.realizations.is_empty());

    pump.update_view(
        ItemsRepeater::new()
            .item("valid", TextBlock::new().text("valid"))
            .item("invalid", TextBlock::new().text("repaired"))
            .into(),
    )
    .unwrap();

    assert_eq!(
        recorded_text(pump.runtime(), collection),
        ["valid", "repaired"]
    );
}

#[test]
fn reference_validation_failure_retains_the_realization_request() {
    let reference = ElementRef::<TextBox>::new();
    let mut owner = Pump::new(RecordingRuntime::default());
    owner
        .mount(TextBox::new().element_ref(&reference).into())
        .unwrap();

    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        ItemsRepeater::new()
            .item("row", TextBox::new().element_ref(&reference))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });

    assert_eq!(
        pump.process_realizations(),
        Err(PumpError::DuplicateElementRef)
    );
    assert_eq!(pump.realizations.len(), 1);
    assert!(pump.tree.children(collection).unwrap().is_empty());

    owner.shutdown();
    assert_eq!(pump.process_realizations().unwrap().len(), 1);
    assert!(reference.request_focus());
}

#[test]
fn provider_and_fragment_row_use_normal_view_composition() {
    let context = Context::new("default".to_string());
    let row = View::fragment((View::provide(
        &context,
        "provided".to_string(),
        View::component::<ContextRow>(context.clone()),
    ),));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(ItemsRepeater::new().item("context", row).into())
        .unwrap();
    let collection = pump.root().unwrap();
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });

    pump.process_realizations().unwrap();

    assert_eq!(recorded_text(pump.runtime(), collection), ["provided"]);
}

#[test]
fn virtual_collection_update_resets_source_and_rejects_old_leases() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        ItemsRepeater::new()
            .item("a", TextBlock::new().text("A"))
            .item("b", TextBlock::new().text("B"))
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    let old = pump
        .tree
        .virtual_model_mut(root)
        .unwrap()
        .realize(0, RealizedContainer(1))
        .unwrap();

    pump.update(
        ItemsRepeater::new()
            .item("b", TextBlock::new().text("B2"))
            .item("c", TextBlock::new().text("C"))
            .into(),
    )
    .unwrap();

    assert!(!pump.tree.virtual_model(root).unwrap().accepts(&old));
    assert_eq!(
        pump.runtime().commands().last().unwrap(),
        &[Command::ResetVirtualCollection {
            node: root,
            item_count: 2,
            source_revision: 1,
        }]
    );
}

#[test]
fn source_reset_rejects_a_queued_old_revision_request() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        ItemsRepeater::new()
            .item("a", TextBlock::new().text("A"))
            .item("b", TextBlock::new().text("B"))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    let source_revision = pump.runtime().source_revision(collection).unwrap();
    let request = RealizationRequest::Realize {
        collection,
        container: RealizedContainer(1),
        index: 1,
        source_revision,
    };

    pump.update(
        ItemsRepeater::new()
            .item("z", TextBlock::new().text("Z"))
            .item("a", TextBlock::new().text("A"))
            .into(),
    )
    .unwrap();
    pump.runtime_mut().queue_realization(request);

    assert_eq!(
        pump.process_realizations().unwrap(),
        [RealizationOutcome::Rejected(request)]
    );
    assert!(pump.tree.children(collection).unwrap().is_empty());
    assert_eq!(pump.runtime().source_revision(collection), Some(1));
}

#[test]
fn recycle_queued_before_source_reset_is_acknowledged_after_reset() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        ItemsRepeater::new()
            .item("a", TextBlock::new().text("A"))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    let container = RealizedContainer(1);
    pump.runtime_mut().queue_realize(collection, container, 0);
    assert!(matches!(
        pump.process_realizations().unwrap().as_slice(),
        [RealizationOutcome::Realized(_)]
    ));
    pump.runtime_mut().queue_recycle(collection, container);

    pump.update(
        ItemsRepeater::new()
            .item("z", TextBlock::new().text("Z"))
            .into(),
    )
    .unwrap();

    assert!(matches!(
        pump.process_realizations().unwrap().as_slice(),
        [RealizationOutcome::Rejected(RealizationRequest::Recycle {
            container: rejected,
            ..
        })] if *rejected == container
    ));
    assert_eq!(
        pump.runtime().commands().last().unwrap(),
        &[Command::AcknowledgeRecycle {
            collection,
            container,
        }]
    );
}

#[test]
fn queued_recycle_supersedes_unprocessed_realization() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        ItemsRepeater::new()
            .item("a", TextBlock::new().text("A"))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    let dead = RealizedContainer(1);
    pump.runtime_mut().queue_realize(collection, dead, 0);
    pump.runtime_mut().queue_recycle(collection, dead);

    assert!(matches!(
        pump.process_realizations().unwrap().as_slice(),
        [
            RealizationOutcome::Rejected(RealizationRequest::Realize { container, .. }),
            RealizationOutcome::Rejected(RealizationRequest::Recycle {
                container: recycled,
                ..
            })
        ] if *container == dead && *recycled == dead
    ));
    assert!(pump.tree.children(collection).unwrap().is_empty());

    pump.runtime_mut()
        .queue_realize(collection, RealizedContainer(2), 0);
    assert!(matches!(
        pump.process_realizations().unwrap().as_slice(),
        [RealizationOutcome::Realized(lease)] if lease.key == Key::from("a")
    ));
}

#[test]
fn deep_index_realization_preserves_the_index_key() {
    let mut repeater = ItemsRepeater::new();
    for index in 0..10_000 {
        repeater = repeater.item(index.to_string(), TextBlock::new().text(index.to_string()));
    }
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(repeater.into()).unwrap();
    let collection = pump.root().unwrap();

    pump.runtime_mut()
        .queue_realize(collection, RealizedContainer(1), 9_999);
    let outcomes = pump.process_realizations().unwrap();

    let [RealizationOutcome::Realized(lease)] = outcomes.as_slice() else {
        panic!("expected deep row realization");
    };
    assert_eq!(lease.key, Key::from("9999"));
    let row = pump.tree.children(collection).unwrap()[0];
    assert_eq!(pump.tree.key(row), Ok(Some(&lease.key)));
}

#[test]
fn every_realization_command_failure_poisoned_without_publication() {
    for command in 0..3 {
        let mut pump = Pump::new(RecordingRuntime::default());
        pump.mount(
            ItemsRepeater::new()
                .item("a", TextBlock::new().text("A"))
                .into(),
        )
        .unwrap();
        let collection = pump.root().unwrap();
        pump.runtime_mut().fail_after(0, command);
        pump.runtime_mut()
            .queue_realization(RealizationRequest::Realize {
                collection,
                container: RealizedContainer(1),
                index: 0,
                source_revision: 0,
            });

        assert!(matches!(
            pump.process_realizations(),
            Err(PumpError::NativeApplyFailed(_))
        ));
        assert!(pump.tree.children(collection).unwrap().is_empty());
        assert_eq!(pump.process_realizations(), Err(PumpError::Poisoned));
    }
}

#[test]
fn component_row_native_failure_does_not_commit_effect_setup() {
    let input = row_input("A");
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        ItemsRepeater::new()
            .item("a", View::component::<ComponentRow>(input.clone()))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    pump.runtime_mut().fail_after(0, 0);
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });

    assert!(matches!(
        pump.process_realizations(),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert!(input.log.borrow().is_empty());
    assert!(pump.tree.children(collection).unwrap().is_empty());
    assert!(pump.poisoned());
}

#[test]
fn realization_work_budget_preserves_and_reports_pending_work() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(ItemsRepeater::new().into()).unwrap();
    let missing = NodeId::from_parts(u32::MAX, 0);
    for index in 0..=REALIZATION_WORK_BUDGET {
        pump.runtime_mut()
            .queue_realization(RealizationRequest::Realize {
                collection: missing,
                container: RealizedContainer(index as u64),
                index,
                source_revision: 0,
            });
    }

    assert_eq!(
        pump.process_realizations().unwrap().len(),
        REALIZATION_WORK_BUDGET
    );
    assert!(pump.native_work_pending());
    assert_eq!(pump.process_realizations().unwrap().len(), 1);
    assert!(!pump.native_work_pending());
}

#[test]
fn shutdown_discards_pending_realization_and_rejects_stale_collection() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        ItemsRepeater::new()
            .item("a", TextBlock::new().text("A"))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    let request = RealizationRequest::Realize {
        collection,
        container: RealizedContainer(1),
        index: 0,
        source_revision: 0,
    };
    let old_identity = pump.window_token();
    pump.runtime_mut().queue_realization(request);

    pump.shutdown();

    assert!(pump.process_realizations().unwrap().is_empty());
    pump.mount(
        ItemsRepeater::new()
            .item("a", TextBlock::new().text("A"))
            .into(),
    )
    .unwrap();
    assert_eq!(pump.root(), Some(collection));
    assert_ne!(pump.window_token(), old_identity);
    pump.runtime_mut()
        .queue_realization_with_identity(old_identity, request);
    assert_eq!(
        pump.process_realizations().unwrap(),
        [RealizationOutcome::Rejected(request)]
    );
    pump.runtime_mut().queue_realization(request);
    assert!(matches!(
        pump.process_realizations().unwrap().as_slice(),
        [RealizationOutcome::Realized(_)]
    ));
}

#[test]
fn repeated_virtual_mount_realize_recycle_shutdown_returns_to_zero() {
    let mut pump = Pump::new(RecordingRuntime::default());
    for cycle in 0..100 {
        pump.mount(
            ItemsRepeater::new()
                .item("a", TextBlock::new().text("A"))
                .item("b", Button::new())
                .into(),
        )
        .unwrap();
        let collection = pump.root().unwrap();
        for index in 0..2 {
            pump.runtime_mut()
                .queue_realization(RealizationRequest::Realize {
                    collection,
                    container: RealizedContainer(index),
                    index: index as usize,
                    source_revision: 0,
                });
        }
        assert_eq!(pump.process_realizations().unwrap().len(), 2);
        for index in 0..2 {
            pump.runtime_mut()
                .queue_realization(RealizationRequest::Recycle {
                    collection,
                    container: RealizedContainer(index),
                    source_revision: 0,
                });
        }
        assert_eq!(pump.process_realizations().unwrap().len(), 2);

        pump.shutdown();

        assert_eq!(pump.tree.len(), 0, "cycle {cycle}");
        assert!(pump.runtime().is_empty(), "cycle {cycle}");
    }
}

#[test]
fn component_view_reuses_a_virtual_collection_shell_immediately() {
    struct VirtualRoot;

    impl Component for VirtualRoot {
        type Message = ();
        type Input = ();

        fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn input_changed(&mut self, _input: &(), _context: &ComponentContext<Self>) {}

        fn update(&mut self, _message: (), _context: &ComponentContext<Self>) {}

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            ScrollViewer::new().content(
                ItemsRepeater::new()
                    .item("a", TextBlock::new().text("A"))
                    .item("b", TextBlock::new().text("B")),
            )
        }
    }

    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<VirtualRoot>(())).unwrap();
    let scroll = Pump::<RecordingRuntime>::native_root(&pump.tree, pump.root().unwrap()).unwrap();
    let collection = pump.tree.children(scroll).unwrap()[0];
    let container = RealizedContainer(1);
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container,
            index: 0,
            source_revision: 0,
        });
    let first = pump.process_realizations().unwrap();
    let first_child = pump.tree.children(collection).unwrap()[0];
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container,
            source_revision: 0,
        });
    pump.process_realizations().unwrap();
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container,
            index: 1,
            source_revision: 0,
        });
    let second = pump.process_realizations().unwrap();

    let [RealizationOutcome::Realized(first)] = first.as_slice() else {
        panic!("expected first realization");
    };
    let [RealizationOutcome::Realized(second)] = second.as_slice() else {
        panic!("expected second realization");
    };
    assert_eq!(first.container, container);
    assert_eq!(second.container, container);
    assert_eq!(first.key, Key::from("a"));
    assert_eq!(second.key, Key::from("b"));
    assert_eq!(pump.tree.children(collection).unwrap().len(), 1);
    assert_ne!(pump.tree.children(collection).unwrap()[0], first_child);
    assert_eq!(recorded_text(pump.runtime(), collection), ["B"]);
}

#[test]
fn virtual_source_reset_retires_realized_rows_before_clearing_leases() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        ItemsRepeater::new()
            .item("a", TextBlock::new().text("A"))
            .item("b", TextBlock::new().text("B"))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    for index in 0..2 {
        pump.runtime_mut()
            .queue_realization(RealizationRequest::Realize {
                collection,
                container: RealizedContainer(index),
                index: index as usize,
                source_revision: 0,
            });
    }
    pump.process_realizations().unwrap();
    let realized = pump.tree.children(collection).unwrap().to_vec();
    assert_eq!(realized.len(), 2);

    pump.update(
        ItemsRepeater::new()
            .item("z", TextBlock::new().text("Z"))
            .item("b", TextBlock::new().text("B"))
            .into(),
    )
    .unwrap();

    assert!(pump.tree.children(collection).unwrap().is_empty());
    assert_eq!(pump.tree.virtual_model(collection).unwrap().active_len(), 0);
    assert!(
        pump.runtime()
            .node(collection)
            .unwrap()
            .children()
            .is_empty()
    );
    for child in realized {
        assert!(pump.runtime().node(child).is_none());
    }
}

#[test]
fn virtual_payload_change_reconciles_rows_without_resetting_source() {
    let first = Rc::new(Cell::new(0));
    let second = Rc::new(Cell::new(0));
    let first_callback = Rc::clone(&first);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        ItemsRepeater::new()
            .item(
                "button",
                Button::new().on_click(move || first_callback.set(first_callback.get() + 1)),
            )
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(0),
            index: 0,
            source_revision: 0,
        });
    pump.process_realizations().unwrap();
    let child = pump.tree.children(collection).unwrap()[0];
    let revision = pump.event_revision(child, EventId::ButtonClick).unwrap();
    let batches = pump.runtime().batches();
    let source_revision = pump.runtime().source_revision(collection);
    let second_callback = Rc::clone(&second);

    pump.update(
        ItemsRepeater::new()
            .item(
                "button",
                Button::new().on_click(move || second_callback.set(second_callback.get() + 1)),
            )
            .into(),
    )
    .unwrap();

    assert_eq!(pump.runtime().batches(), batches);
    assert_eq!(pump.runtime().source_revision(collection), source_revision);
    assert_eq!(
        pump.tree
            .virtual_model(collection)
            .unwrap()
            .source_revision(),
        source_revision.unwrap()
    );
    assert_eq!(pump.tree.children(collection).unwrap(), &[child]);
    pump.queue_event(QueuedEvent::new(
        child,
        EventId::ButtonClick,
        revision,
        EventPayload::Unit,
    ));
    assert_eq!(pump.dispatch_events(), Ok(1));
    assert_eq!(first.get(), 0);
    assert_eq!(second.get(), 1);
}

#[test]
fn same_batch_container_reuse_retires_earlier_row() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        ItemsRepeater::new()
            .item("a", TextBlock::new().text("A"))
            .item("b", TextBlock::new().text("B"))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    let container = RealizedContainer(1);
    for index in 0..2 {
        pump.runtime_mut()
            .queue_realization(RealizationRequest::Realize {
                collection,
                container,
                index,
                source_revision: 0,
            });
    }

    let outcomes = pump.process_realizations().unwrap();

    let RealizationOutcome::Realized(first) = &outcomes[0] else {
        panic!("expected first lease");
    };
    let RealizationOutcome::Realized(second) = &outcomes[1] else {
        panic!("expected second lease");
    };
    assert!(!pump.tree.virtual_model(collection).unwrap().accepts(first));
    assert!(pump.tree.virtual_model(collection).unwrap().accepts(second));
    assert_eq!(pump.tree.children(collection).unwrap().len(), 1);
    assert_eq!(
        pump.tree.key(pump.tree.children(collection).unwrap()[0]),
        Ok(Some(&Key::from("b")))
    );
    assert_eq!(pump.runtime().node(collection).unwrap().children().len(), 1);
}
