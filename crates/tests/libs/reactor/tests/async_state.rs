//! Tests for gap H5: off-UI-thread setter auto-marshal via
//! `cx.use_async_state` + `AsyncSetState`.

use std::cell::Cell;
use std::rc::Rc;
use std::thread;

use windows_reactor::RenderCx;
use windows_reactor::imp::{ChannelDispatcher, UiMarshaller, UiRerenderGuard};

#[test]
fn async_state_initial_value_returned_to_caller() {
    let dispatcher = ChannelDispatcher::new();
    let mut cx = RenderCx::for_test();
    cx.set_marshaller(Some(dispatcher.marshaller()));
    cx.begin_render();

    let (v, _set) = cx.use_async_state(7_i32);
    assert_eq!(v, 7);
}

#[test]
fn async_setter_from_off_thread_marshals_back_and_persists() {
    let dispatcher = ChannelDispatcher::new();
    let marshaller = dispatcher.marshaller();

    let rerenders = Rc::new(Cell::new(0_u32));
    let rerenders_c = Rc::clone(&rerenders);
    let _ui_guard = UiRerenderGuard::install(Rc::new(move || {
        rerenders_c.set(rerenders_c.get() + 1);
    }));

    let mut cx = RenderCx::for_test();
    cx.set_marshaller(Some(marshaller));
    cx.begin_render();
    let (initial, set) = cx.use_async_state(0_i32);
    assert_eq!(initial, 0);

    // Move the setter to a background thread and call it there.
    let handle = thread::spawn(move || {
        set.call(42);
    });
    handle.join().unwrap();

    // The write was marshalled — it lives in the channel waiting for the
    // UI thread to drain it. Nothing has applied yet.
    assert_eq!(rerenders.get(), 0);
    assert_eq!(dispatcher.pending(), 1);

    // Drain on the "UI" thread (this one).
    let fired = dispatcher.drain();
    assert_eq!(fired, 1);
    assert_eq!(rerenders.get(), 1);

    // The slot now holds the new value; the next render sees it.
    cx.begin_render();
    let (v, _) = cx.use_async_state(0_i32);
    assert_eq!(v, 42);
}

#[test]
fn async_setter_marks_owning_component_dirty_for_rerender() {
    // Regression: an off-thread `use_async_state` write must mark the owning
    // component dirty, not just request a root rerender. Without this, a
    // nested component (whose parent's element tree is unchanged) is skipped
    // by the reconciler's `can_skip_update` path and never observes the new
    // value until an unrelated `use_state` change forces it to re-render.
    let dispatcher = ChannelDispatcher::new();
    let _ui_guard = UiRerenderGuard::install(Rc::new(|| {}));

    let mut cx = RenderCx::for_test();
    cx.set_marshaller(Some(dispatcher.marshaller()));
    cx.begin_render();
    let (_, set) = cx.use_async_state(0_i32);

    // A fresh render starts clean.
    assert!(!cx.peek_state_dirty());

    thread::spawn(move || set.call(42)).join().unwrap();
    dispatcher.drain();

    // Applying the off-thread write marks the component dirty so the
    // reconciler will not skip re-rendering it.
    assert!(
        cx.peek_state_dirty(),
        "async write must mark the owning component dirty"
    );

    // The flag clears at the start of the next render.
    cx.begin_render();
    assert!(!cx.peek_state_dirty());
}

#[test]
fn async_setter_equal_value_does_not_mark_dirty() {
    let dispatcher = ChannelDispatcher::new();
    let _ui_guard = UiRerenderGuard::install(Rc::new(|| {}));

    let mut cx = RenderCx::for_test();
    cx.set_marshaller(Some(dispatcher.marshaller()));
    cx.begin_render();
    let (_, set) = cx.use_async_state(7_i32);

    thread::spawn(move || set.call(7)).join().unwrap();
    dispatcher.drain();

    assert!(
        !cx.peek_state_dirty(),
        "an unchanged async write must not mark the component dirty"
    );
}

#[test]
fn async_setter_equal_value_does_not_trigger_rerender() {
    let dispatcher = ChannelDispatcher::new();
    let marshaller = dispatcher.marshaller();

    let rerenders = Rc::new(Cell::new(0_u32));
    let rerenders_c = Rc::clone(&rerenders);
    let _ui_guard = UiRerenderGuard::install(Rc::new(move || {
        rerenders_c.set(rerenders_c.get() + 1);
    }));

    let mut cx = RenderCx::for_test();
    cx.set_marshaller(Some(marshaller));
    cx.begin_render();
    let (_, set) = cx.use_async_state("hi".to_string());

    let set2 = set;
    let handle = thread::spawn(move || set2.call("hi".into()));
    handle.join().unwrap();

    dispatcher.drain();
    assert_eq!(
        rerenders.get(),
        0,
        "equal value must not request a rerender"
    );
}

#[test]
fn use_ui_marshaller_returns_send_sync_handle() {
    let dispatcher = ChannelDispatcher::new();
    let mut cx = RenderCx::for_test();
    cx.set_marshaller(Some(dispatcher.marshaller()));
    cx.begin_render();

    let m: UiMarshaller = cx.use_ui_marshaller();

    let ran = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let ran_c = std::sync::Arc::clone(&ran);
    let m_clone = m;
    let handle = thread::spawn(move || {
        m_clone.dispatch(move || {
            ran_c.store(true, std::sync::atomic::Ordering::SeqCst);
        });
    });
    handle.join().unwrap();

    assert!(!ran.load(std::sync::atomic::Ordering::SeqCst));
    dispatcher.drain();
    assert!(ran.load(std::sync::atomic::Ordering::SeqCst));
}

#[test]
fn async_state_multiple_setters_target_same_slot() {
    let dispatcher = ChannelDispatcher::new();
    let _ui_guard = UiRerenderGuard::install(Rc::new(|| {}));

    let mut cx = RenderCx::for_test();
    cx.set_marshaller(Some(dispatcher.marshaller()));
    cx.begin_render();

    let (_, set) = cx.use_async_state(0_i32);
    let set_a = set.clone();
    let set_b = set;

    thread::spawn(move || set_a.call(1)).join().unwrap();
    thread::spawn(move || set_b.call(2)).join().unwrap();
    dispatcher.drain();

    cx.begin_render();
    let (v, _) = cx.use_async_state(0_i32);
    assert_eq!(v, 2);
}
