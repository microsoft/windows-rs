//! Application reconciliation, hooks, context, lifecycle, and failure tests.

use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::*;

mod support;

use support::*;

struct NotClone(usize);

struct OrderedRuntime {
    inner: RecordingRuntime,
    log: Rc<RefCell<Vec<&'static str>>>,
}

struct ReferenceRuntime {
    inner: RecordingRuntime,
    reference: Rc<RefCell<Option<ElementRef<TextBox>>>>,
    log: Rc<RefCell<Vec<&'static str>>>,
}

impl NativeRuntime for ReferenceRuntime {
    fn apply(&mut self, commands: &[Command]) {
        if commands
            .iter()
            .any(|command| matches!(command, Command::Destroy { .. }))
        {
            assert!(!self.reference.borrow().as_ref().unwrap().is_mounted());
            self.log.borrow_mut().push("destroy");
        } else {
            self.log.borrow_mut().push("commit");
        }
        self.inner.apply(commands);
    }

    fn drain_events(&mut self) -> Vec<NativeEvent> {
        self.inner.drain_events()
    }
}

impl NativeRuntime for OrderedRuntime {
    fn apply(&mut self, commands: &[Command]) {
        if commands
            .iter()
            .any(|command| matches!(command, Command::Destroy { .. }))
        {
            self.log.borrow_mut().push("destroy");
        } else {
            self.log.borrow_mut().push("commit");
        }
        self.inner.apply(commands);
    }

    fn drain_events(&mut self) -> Vec<NativeEvent> {
        self.inner.drain_events()
    }

    fn set_event_waker(&mut self, waker: Option<Rc<dyn Fn()>>) {
        self.inner.set_event_waker(waker);
    }
}

mod canvas;
mod components;
mod containers;
mod controls;
mod hooks;
mod properties;
mod structure;
