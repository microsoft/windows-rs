use std::cell::RefCell;
use std::rc::Rc;

use windows_reactor::{
    Backend, Callback, ControlId, ControlKind, Event, EventHandler, Modifiers, Op, Prop,
    PropBindings, PropValue, Reconciler, RecordingBackend, Widget,
};

type Log = Rc<RefCell<Vec<(&'static str, bool)>>>;

struct ProbeWidget {
    modifiers: Modifiers,
    mounted: Option<Callback<Option<windows_core::IInspectable>>>,
    unmounted: Option<Callback<Option<windows_core::IInspectable>>>,
}

impl ProbeWidget {
    fn new(log: &Log, with_mount: bool, with_unmount: bool) -> Self {
        let mk = |tag: &'static str| {
            let log = Rc::clone(log);
            Callback::new(move |native: Option<windows_core::IInspectable>| {
                log.borrow_mut().push((tag, native.is_some()));
            })
        };

        Self {
            modifiers: Modifiers::default(),
            mounted: with_mount.then(|| mk("mount")),
            unmounted: with_unmount.then(|| mk("unmount")),
        }
    }
}

impl Widget for ProbeWidget {
    fn kind(&self) -> ControlKind {
        ControlKind::Border
    }

    fn key(&self) -> Option<&str> {
        None
    }

    fn modifiers(&self) -> &Modifiers {
        &self.modifiers
    }

    fn bindings(&self) -> PropBindings {
        Vec::new()
    }

    fn on_mounted_callback(&self) -> Option<&Callback<Option<windows_core::IInspectable>>> {
        self.mounted.as_ref()
    }

    fn on_unmounted_callback(&self) -> Option<&Callback<Option<windows_core::IInspectable>>> {
        self.unmounted.as_ref()
    }
}

#[derive(Default)]
struct NoNativeBackend {
    next_id: u32,
    destroyed: Vec<ControlId>,
}

impl Backend for NoNativeBackend {
    fn create(&mut self, _kind: ControlKind) -> ControlId {
        self.next_id += 1;
        ControlId::new(self.next_id)
    }

    fn set_prop(&mut self, _id: ControlId, _prop: Prop, _value: &PropValue) {}

    fn append_child(&mut self, _parent: ControlId, _child: ControlId) {}

    fn remove_child(&mut self, _parent: ControlId, _index: usize) {}

    fn replace_child(&mut self, _parent: ControlId, _index: usize, _new: ControlId) {}

    fn move_child(&mut self, _parent: ControlId, _from: usize, _to: usize) {}

    fn insert_child(&mut self, _parent: ControlId, _index: usize, _child: ControlId) {}

    fn destroy(&mut self, id: ControlId) {
        self.destroyed.push(id);
    }

    fn attach_event(&mut self, _id: ControlId, _event: Event, _handler: EventHandler) {}
}

fn reconciler() -> Reconciler<RecordingBackend> {
    Reconciler::new(RecordingBackend::new())
}

fn destroyed(reconciler: &Reconciler<RecordingBackend>, id: ControlId) -> bool {
    reconciler
        .backend
        .ops
        .iter()
        .any(|op| matches!(op, Op::Destroy { id: destroyed } if *destroyed == id))
}

#[test]
fn mount_callback_fires_once_with_native_when_available() {
    let log: Log = Rc::default();
    let mut reconciler = reconciler();
    let probe = ProbeWidget::new(&log, true, false);
    reconciler.mount_widget(&probe);
    assert_eq!(*log.borrow(), vec![("mount", true)]);
}

#[test]
fn mount_callback_fires_once_with_none_when_no_native() {
    let log: Log = Rc::default();
    let mut reconciler = Reconciler::new(NoNativeBackend::default());
    let probe = ProbeWidget::new(&log, true, false);
    reconciler.mount_widget(&probe);
    assert_eq!(*log.borrow(), vec![("mount", false)]);
}

#[test]
fn unmount_callback_fires_before_destroy_with_native() {
    let log: Log = Rc::default();
    let mut reconciler = reconciler();
    let probe = ProbeWidget::new(&log, false, true);
    let id = reconciler.mount_widget(&probe);
    assert!(log.borrow().is_empty(), "must not fire before unmount");

    reconciler.unmount(id);
    assert_eq!(*log.borrow(), vec![("unmount", true)]);
    assert!(destroyed(&reconciler, id), "control was destroyed");
}

#[test]
fn unmount_callback_fires_with_none_when_no_native() {
    let log: Log = Rc::default();
    let mut reconciler = Reconciler::new(NoNativeBackend::default());
    let probe = ProbeWidget::new(&log, false, true);
    let id = reconciler.mount_widget(&probe);
    reconciler.unmount(id);
    assert_eq!(*log.borrow(), vec![("unmount", false)]);
    assert!(
        reconciler.backend.destroyed.contains(&id),
        "control was destroyed"
    );
}

#[test]
fn mount_and_unmount_fire_across_lifecycle() {
    let log: Log = Rc::default();
    let mut reconciler = reconciler();
    let probe = ProbeWidget::new(&log, true, true);
    let id = reconciler.mount_widget(&probe);
    assert_eq!(*log.borrow(), vec![("mount", true)]);
    reconciler.unmount(id);
    assert_eq!(*log.borrow(), vec![("mount", true), ("unmount", true)]);
}

#[test]
fn update_replaces_then_removes_unmount_callback() {
    let log: Log = Rc::default();
    let mut reconciler = reconciler();

    let first = ProbeWidget::new(&log, false, true);
    let id = reconciler.mount_widget(&first);

    let second_log: Log = Rc::default();
    let second = ProbeWidget::new(&second_log, false, true);
    reconciler.update_widget(&first, &second, id);

    let third = ProbeWidget::new(&log, false, false);
    reconciler.update_widget(&second, &third, id);

    reconciler.unmount(id);
    assert!(
        log.borrow().is_empty(),
        "neither the original nor a cleared callback fires"
    );
    assert!(
        second_log.borrow().is_empty(),
        "the replaced callback was cleared before teardown"
    );
}
