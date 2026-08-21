//! Virtualization and realization contract tests for [`Pump`].

use super::super::*;
use super::support::*;
use crate::native::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Clone)]
struct RowProps {
    changed: Rc<Cell<u32>>,
    created: Rc<Cell<u32>>,
    label: String,
    log: Rc<RefCell<Vec<String>>>,
}

impl PartialEq for RowProps {
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
    type Props = RowProps;

    fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        props.created.set(props.created.get() + 1);
        Self
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        props.changed.set(props.changed.get() + 1);
    }

    fn update(&mut self, _message: (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, props: &Self::Props, context: &mut ViewContext<Self>) -> View {
        let label = props.label.clone();
        let setup = label.clone();
        let log = Rc::clone(&props.log);
        context.use_effect("row", label, move || {
            log.borrow_mut().push(format!("setup {setup}"));
            Some(Box::new(move || {
                log.borrow_mut().push(format!("cleanup {setup}"));
            }))
        });
        TextBlock::new().text(props.label.clone()).into()
    }
}

struct ContextRow;

impl Component for ContextRow {
    type Message = ();
    type Props = Context<String>;

    fn create(_props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, context: &Self::Props, view: &mut ViewContext<Self>) -> View {
        TextBlock::new().text(view.use_context(context)).into()
    }
}

struct ReplacingRow;

impl Component for ReplacingRow {
    type Message = ();
    type Props = (bool, Rc<Cell<u32>>);

    fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        props.1.set(props.1.get() + 1);
        Self
    }

    fn update(&mut self, _message: (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, props: &Self::Props, _context: &mut ViewContext<Self>) -> View {
        if props.0 {
            Button::new().into()
        } else {
            TextBlock::new().into()
        }
    }
}

fn row_props(label: &str) -> RowProps {
    RowProps {
        changed: Rc::new(Cell::new(0)),
        created: Rc::new(Cell::new(0)),
        label: label.to_string(),
        log: Rc::new(RefCell::new(Vec::new())),
    }
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
        });
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container,
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
        });
    assert_eq!(
        pump.process_realizations().unwrap(),
        [RealizationOutcome::Rejected(RealizationRequest::Realize {
            collection,
            container,
            index: 0,
        })]
    );
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

#[test]
fn component_row_is_created_lazily_and_recycle_cleans_it_once() {
    let props = row_props("A");
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        ItemsRepeater::new()
            .item("a", View::component::<ComponentRow>(props.clone()))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    let version = pump.version();
    assert_eq!(props.created.get(), 0);
    assert!(props.log.borrow().is_empty());

    pump.runtime_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
        });
    pump.process_realizations().unwrap();

    let row = pump
        .tree
        .realized(collection, RealizedContainer(1))
        .unwrap()
        .unwrap();
    assert_eq!(pump.tree.kind(row.logical_root), Ok(NodeKind::Component));
    assert_ne!(row.logical_root, row.native_root);
    assert_eq!(props.created.get(), 1);
    assert_eq!(&*props.log.borrow(), &["setup A"]);
    assert_eq!(pump.version(), version);

    pump.runtime_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(1),
        });
    pump.process_realizations().unwrap();
    assert_eq!(&*props.log.borrow(), &["setup A", "cleanup A"]);
    assert_eq!(pump.version(), version);

    pump.shutdown();
    assert_eq!(&*props.log.borrow(), &["setup A", "cleanup A"]);
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

    let props = row_props("A");
    let mut pump = Pump::new(OrderedRuntime {
        inner: RecordingRuntime::default(),
        log: Rc::clone(&props.log),
    });
    pump.mount_view(
        ItemsRepeater::new()
            .item("a", View::component::<ComponentRow>(props.clone()))
            .into(),
    )
    .unwrap();
    props.log.borrow_mut().clear();
    let collection = pump.root().unwrap();
    pump.runtime_mut()
        .inner
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
        });

    pump.process_realizations().unwrap();
    assert_eq!(&*props.log.borrow(), &["native", "setup A"]);

    props.log.borrow_mut().clear();
    pump.runtime_mut()
        .inner
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(1),
        });
    pump.process_realizations().unwrap();
    assert_eq!(&*props.log.borrow(), &["cleanup A", "native"]);
}

#[test]
fn key_stable_source_update_reconciles_component_row_in_place() {
    let first = row_props("A");
    let second = RowProps {
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
    assert!(pump.runtime().node(before.native_root).is_none());
    assert_eq!(
        pump.runtime().node(after.native_root).unwrap().kind(),
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
        pump.runtime().node(after.native_root).unwrap().kind(),
        Some(MountedKind::Button)
    );
}

#[test]
fn source_key_removal_retires_realized_component_row() {
    let props = row_props("A");
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        ItemsRepeater::new()
            .item("a", View::component::<ComponentRow>(props.clone()))
            .into(),
    )
    .unwrap();
    let collection = pump.root().unwrap();
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
        });
    pump.process_realizations().unwrap();

    pump.update_view(ItemsRepeater::new().into()).unwrap();

    assert_eq!(&*props.log.borrow(), &["setup A", "cleanup A"]);
    assert!(pump.tree.children(collection).unwrap().is_empty());
    assert_eq!(pump.tree.virtual_model(collection).unwrap().active_len(), 0);
}

#[test]
fn invalid_row_arity_does_not_publish_or_leak_component_reservations() {
    let props = row_props("A");
    let invalid = View::fragment((
        View::component::<ComponentRow>(props.clone()),
        TextBlock::new(),
    ));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(ItemsRepeater::new().item("a", invalid).into())
        .unwrap();
    let collection = pump.root().unwrap();
    let batches = pump.runtime().batches();
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
        });

    assert_eq!(
        pump.process_realizations(),
        Err(PumpError::StructureUnsupported)
    );
    assert_eq!(pump.runtime().batches(), batches);
    assert!(pump.tree.children(collection).unwrap().is_empty());
    assert_eq!(pump.tree.virtual_model(collection).unwrap().active_len(), 0);
    assert_eq!(props.created.get(), 1);
    assert!(props.log.borrow().is_empty());

    pump.update_view(
        ItemsRepeater::new()
            .item("a", View::component::<ComponentRow>(props.clone()))
            .into(),
    )
    .unwrap();
    pump.process_realizations().unwrap();
    assert_eq!(props.created.get(), 2);
    assert_eq!(&*props.log.borrow(), &["setup A"]);
}

#[test]
fn empty_row_does_not_publish_a_lease() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(ItemsRepeater::new().item("empty", View::empty()).into())
        .unwrap();
    let collection = pump.root().unwrap();
    let batches = pump.runtime().batches();
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
        });

    assert_eq!(
        pump.process_realizations(),
        Err(PumpError::StructureUnsupported)
    );
    assert_eq!(pump.runtime().batches(), batches);
    assert!(pump.tree.children(collection).unwrap().is_empty());
    assert_eq!(pump.tree.virtual_model(collection).unwrap().active_len(), 0);
}

#[test]
fn failed_batch_retries_consumed_realizations_after_the_row_is_repaired() {
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
            });
    }

    assert_eq!(
        pump.process_realizations(),
        Err(PumpError::StructureUnsupported)
    );
    assert!(pump.tree.children(collection).unwrap().is_empty());
    assert_eq!(pump.realizations.len(), 2);

    pump.update_view(
        ItemsRepeater::new()
            .item("valid", TextBlock::new().text("valid"))
            .item("invalid", TextBlock::new().text("repaired"))
            .into(),
    )
    .unwrap();

    assert_eq!(pump.process_realizations().unwrap().len(), 2);
    assert_eq!(
        recorded_text(pump.runtime(), collection),
        ["valid", "repaired"]
    );
    assert!(pump.realizations.is_empty());
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
        }]
    );
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
    let props = row_props("A");
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        ItemsRepeater::new()
            .item("a", View::component::<ComponentRow>(props.clone()))
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
        });

    assert!(matches!(
        pump.process_realizations(),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert!(props.log.borrow().is_empty());
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
                });
        }
        assert_eq!(pump.process_realizations().unwrap().len(), 2);
        for index in 0..2 {
            pump.runtime_mut()
                .queue_realization(RealizationRequest::Recycle {
                    collection,
                    container: RealizedContainer(index),
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
        type Props = ();

        fn create(_props: &(), _context: &mut ComponentContext<Self>) -> Self {
            Self
        }

        fn changed(&mut self, _props: &(), _context: &mut ComponentContext<Self>) {}

        fn update(&mut self, _message: (), _context: &mut ComponentContext<Self>) {}

        fn view(&self, _props: &Self::Props, _context: &mut ViewContext<Self>) -> View {
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
        });
    let first = pump.process_realizations().unwrap();
    let first_child = pump.tree.children(collection).unwrap()[0];
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container,
        });
    pump.process_realizations().unwrap();
    pump.runtime_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container,
            index: 1,
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
        });
    pump.process_realizations().unwrap();
    let child = pump.tree.children(collection).unwrap()[0];
    let revision = pump.event_revision(child, EventId::ButtonClick).unwrap();
    let batches = pump.runtime().batches();
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
