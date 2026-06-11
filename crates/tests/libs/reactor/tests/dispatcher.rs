use std::cell::RefCell;
use std::rc::Rc;

use windows_reactor::core::dispatcher::{
    Dispatcher, DispatcherQueuePriority, RunOnDemandDispatcher,
};

fn job(log: Rc<RefCell<Vec<&'static str>>>, tag: &'static str) -> Box<dyn FnOnce()> {
    Box::new(move || log.borrow_mut().push(tag))
}

#[test]
fn run_on_demand_dispatcher_drains_normal_in_fifo_order() {
    let dispatcher = RunOnDemandDispatcher::new();
    let log: Rc<RefCell<Vec<&'static str>>> = Rc::new(RefCell::new(Vec::new()));
    dispatcher.enqueue(DispatcherQueuePriority::Normal, job(Rc::clone(&log), "a"));
    dispatcher.enqueue(DispatcherQueuePriority::Normal, job(Rc::clone(&log), "b"));
    dispatcher.enqueue(DispatcherQueuePriority::Normal, job(Rc::clone(&log), "c"));
    let fired = dispatcher.drain();
    assert_eq!(fired, 3);
    assert_eq!(*log.borrow(), vec!["a", "b", "c"]);
}

#[test]
fn run_on_demand_dispatcher_drains_normal_before_low() {
    let dispatcher = RunOnDemandDispatcher::new();
    let log: Rc<RefCell<Vec<&'static str>>> = Rc::new(RefCell::new(Vec::new()));

    dispatcher.enqueue(DispatcherQueuePriority::Normal, job(Rc::clone(&log), "n1"));
    dispatcher.enqueue(DispatcherQueuePriority::Low, job(Rc::clone(&log), "lo"));
    dispatcher.enqueue(DispatcherQueuePriority::Normal, job(Rc::clone(&log), "n2"));
    let fired = dispatcher.drain();
    assert_eq!(fired, 3);
    assert_eq!(*log.borrow(), vec!["n1", "n2", "lo"]);
}
