use std::cell::{Cell, RefCell};
use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::element::TextBox;
use crate::hooks::{Cleanup, SchedulerRef};
use crate::id::NodeId;

pub(crate) struct WindowRefTarget {
    pending: Cell<Option<NodeId>>,
    pub(crate) current: Cell<Option<NodeId>>,
    scheduler: RefCell<Option<SchedulerRef>>,
}

pub struct WindowRef {
    pub(crate) target: Rc<WindowRefTarget>,
}

impl WindowRef {
    pub fn new() -> Self {
        Self {
            target: Rc::new(WindowRefTarget {
                pending: Cell::new(None),
                current: Cell::new(None),
                scheduler: RefCell::new(None),
            }),
        }
    }

    pub fn is_mounted(&self) -> bool {
        self.target.current.get().is_some()
    }

    pub fn activate(&self) -> bool {
        let Some(id) = self.target.current.get() else {
            return false;
        };
        let Some(scheduler) = self.target.scheduler.borrow().clone() else {
            return false;
        };
        scheduler.activate_window(id);
        true
    }

    pub(crate) fn binding(&self) -> NativeWindowRef {
        NativeWindowRef {
            target: Rc::clone(&self.target),
        }
    }
}

impl Clone for WindowRef {
    fn clone(&self) -> Self {
        Self {
            target: Rc::clone(&self.target),
        }
    }
}

impl Default for WindowRef {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for WindowRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WindowRef")
            .field("mounted", &self.is_mounted())
            .finish()
    }
}

#[derive(Clone)]
pub(crate) struct NativeWindowRef {
    target: Rc<WindowRefTarget>,
}

impl NativeWindowRef {
    pub(crate) fn prepare_mount(&self, id: NodeId, scheduler: SchedulerRef) {
        assert!(
            self.target.current.get().is_none() && self.target.pending.get().is_none(),
            "window reference attached to multiple mounted windows"
        );
        self.target.pending.set(Some(id));
        *self.target.scheduler.borrow_mut() = Some(scheduler);
    }

    pub(crate) fn commit(&self, id: NodeId) {
        if self.target.pending.get() == Some(id) {
            self.target.pending.set(None);
            self.target.current.set(Some(id));
        }
    }

    pub(crate) fn clear(&self) {
        self.target.pending.set(None);
        self.target.current.set(None);
        self.target.scheduler.borrow_mut().take();
    }
}

impl PartialEq for NativeWindowRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.target, &other.target)
    }
}

impl Eq for NativeWindowRef {}

#[derive(Clone, Default)]
struct ElementLifecycle {
    mounted: Option<Rc<dyn Fn()>>,
    unmounted: Option<Rc<dyn Fn()>>,
}

pub(crate) struct ElementRefTarget {
    pending: Cell<Option<NodeId>>,
    pub(crate) current: Cell<Option<NodeId>>,
    scheduler: RefCell<Option<SchedulerRef>>,
    next: RefCell<ElementLifecycle>,
    committed: RefCell<ElementLifecycle>,
}

pub struct ElementRef<T> {
    pub(crate) target: Rc<ElementRefTarget>,
    marker: PhantomData<fn() -> T>,
}

impl<T> ElementRef<T> {
    pub fn new() -> Self {
        Self {
            target: Rc::new(ElementRefTarget {
                pending: Cell::new(None),
                current: Cell::new(None),
                scheduler: RefCell::new(None),
                next: RefCell::new(ElementLifecycle::default()),
                committed: RefCell::new(ElementLifecycle::default()),
            }),
            marker: PhantomData,
        }
    }

    pub fn is_mounted(&self) -> bool {
        self.target.current.get().is_some()
    }

    pub(crate) fn binding(&self) -> NativeElementRef {
        NativeElementRef {
            target: Rc::clone(&self.target),
        }
    }

    pub(crate) fn set_lifecycle(
        &self,
        mounted: Option<Rc<dyn Fn()>>,
        unmounted: Option<Rc<dyn Fn()>>,
    ) {
        *self.target.next.borrow_mut() = ElementLifecycle { mounted, unmounted };
    }

    pub(crate) fn clear_lifecycle(&self) {
        *self.target.next.borrow_mut() = ElementLifecycle::default();
    }

    pub(crate) fn schedule(&self, action: impl FnOnce(&SchedulerRef, NodeId)) -> bool {
        let Some(id) = self.target.current.get() else {
            return false;
        };
        let scheduler = self.target.scheduler.borrow();
        let Some(scheduler) = scheduler.as_ref() else {
            return false;
        };
        action(scheduler, id);
        true
    }
}

impl ElementRef<TextBox> {
    pub fn focus(&self) -> bool {
        let Some(id) = self.target.current.get() else {
            return false;
        };
        let Some(scheduler) = self.target.scheduler.borrow().clone() else {
            return false;
        };
        scheduler.focus_element(id);
        true
    }
}

impl<T> Clone for ElementRef<T> {
    fn clone(&self) -> Self {
        Self {
            target: Rc::clone(&self.target),
            marker: PhantomData,
        }
    }
}

impl<T> Default for ElementRef<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> fmt::Debug for ElementRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ElementRef")
            .field("mounted", &self.is_mounted())
            .finish()
    }
}

#[derive(Clone)]
pub(crate) struct NativeElementRef {
    target: Rc<ElementRefTarget>,
}

impl NativeElementRef {
    pub(crate) fn prepare_mount(&self, id: NodeId, scheduler: Option<SchedulerRef>) {
        assert!(
            self.target.current.get().is_none() && self.target.pending.get().is_none(),
            "element reference attached to multiple mounted elements"
        );
        self.target.pending.set(Some(id));
        *self.target.scheduler.borrow_mut() = scheduler;
    }

    pub(crate) fn commit(&self, id: NodeId) {
        self.target
            .committed
            .borrow_mut()
            .clone_from(&self.target.next.borrow());
        if self.target.pending.get() == Some(id) {
            self.target.pending.set(None);
            self.target.current.set(Some(id));
            let mounted = self.target.committed.borrow().mounted.clone();
            if let Some(mounted) = mounted {
                mounted();
            }
        }
    }

    pub(crate) fn clear(&self) -> Option<Cleanup> {
        let mounted = self.target.current.take().is_some();
        self.target.pending.set(None);
        self.target.scheduler.borrow_mut().take();
        mounted
            .then(|| self.target.committed.borrow().unmounted.clone())
            .flatten()
            .map(|unmounted| Box::new(move || unmounted()) as Cleanup)
    }
}

impl PartialEq for NativeElementRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.target, &other.target)
    }
}

impl Eq for NativeElementRef {}
