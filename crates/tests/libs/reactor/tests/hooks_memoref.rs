use std::cell::Cell;
use std::rc::Rc;

use windows_reactor::core::callback::Callback;
use windows_reactor::core::render_context::RenderCx;

#[test]
fn memo_first_render_runs_factory() {
    let fired = Rc::new(Cell::new(0));
    let fired_c = Rc::clone(&fired);
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let v = cx.use_memo((1_i32,), || {
        fired_c.set(fired_c.get() + 1);
        42_i32
    });
    assert_eq!(v, 42);
    assert_eq!(fired.get(), 1);
}

#[test]
fn memo_skips_factory_when_deps_unchanged() {
    let fired = Rc::new(Cell::new(0));
    let fired_c = Rc::clone(&fired);
    let mut cx = RenderCx::for_test();

    cx.begin_render();
    let v1 = cx.use_memo((7_i32,), || {
        fired_c.set(fired_c.get() + 1);
        String::from("cached")
    });
    assert_eq!(v1, "cached");
    assert_eq!(fired.get(), 1);

    cx.begin_render();
    let fired_c2 = Rc::clone(&fired);
    let v2 = cx.use_memo((7_i32,), || {
        fired_c2.set(fired_c2.get() + 1);
        String::from("freshly computed — should not be used")
    });
    assert_eq!(v2, "cached");
    assert_eq!(fired.get(), 1, "factory must not run when deps unchanged");
}

#[test]
fn memo_recomputes_on_deps_change() {
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let v1 = cx.use_memo((1_i32,), || "a".to_string());
    assert_eq!(v1, "a");

    cx.begin_render();
    let v2 = cx.use_memo((2_i32,), || "b".to_string());
    assert_eq!(v2, "b");

    cx.begin_render();
    let v3 = cx.use_memo((2_i32,), || "c-would-be".to_string());
    assert_eq!(v3, "b", "deps 2 == 2, stored value kept");
}

#[test]
fn memo_with_empty_tuple_deps_computes_only_once() {
    let fired = Rc::new(Cell::new(0));
    let mut cx = RenderCx::for_test();

    for _ in 0..5 {
        cx.begin_render();
        let fired_c = Rc::clone(&fired);
        let _ = cx.use_memo((), || {
            fired_c.set(fired_c.get() + 1);
            123_i32
        });
    }
    assert_eq!(fired.get(), 1, "empty-deps memo computes exactly once");
}

#[test]
fn callback_identity_is_stable_across_equal_deps() {
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let cb1: Callback<()> = cx.use_callback((0_i32,), |()| {});

    cx.begin_render();
    let cb2: Callback<()> = cx.use_callback((0_i32,), |()| {});

    assert_eq!(cb1, cb2, "callback identity must be stable for equal deps");
}

#[test]
fn callback_identity_changes_when_deps_change() {
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let cb1: Callback<()> = cx.use_callback((0_i32,), |()| {});

    cx.begin_render();
    let cb2: Callback<()> = cx.use_callback((1_i32,), |()| {});

    assert_ne!(cb1, cb2, "deps changed → new callback");
}

#[test]
fn callback_dispatches_captured_closure() {
    let counter = Rc::new(Cell::new(0));
    let counter_c = Rc::clone(&counter);
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let cb: Callback<i32> = cx.use_callback((), move |n| {
        counter_c.set(counter_c.get() + n);
    });
    cb.invoke(3);
    cb.invoke(4);
    assert_eq!(counter.get(), 7);
}

#[test]
fn ref_returns_same_cell_across_renders() {
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let r1 = cx.use_ref(0_i32);
    r1.set(7);

    cx.begin_render();
    let r2 = cx.use_ref::<i32>(999);
    assert_eq!(*r2.borrow(), 7);
}

#[test]
fn ref_mutation_does_not_trigger_rerender() {
    let fired = Rc::new(Cell::new(0));
    let fired_c = Rc::clone(&fired);
    let mut cx = RenderCx::new(Rc::new(move || fired_c.set(fired_c.get() + 1)));
    cx.begin_render();
    let r = cx.use_ref(0_i32);

    r.set(100);
    *r.borrow_mut() += 1;
    r.replace(42);

    assert_eq!(fired.get(), 0, "ref writes must NOT request rerender");
    assert_eq!(*r.borrow(), 42);
}

#[test]
fn ref_is_independent_per_slot() {
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let a = cx.use_ref("a".to_string());
    let b = cx.use_ref("b".to_string());

    a.set("A".into());
    b.set("B".into());

    cx.begin_render();
    let a2 = cx.use_ref(String::new());
    let b2 = cx.use_ref(String::new());
    assert_eq!(*a2.borrow(), "A");
    assert_eq!(*b2.borrow(), "B");
}

#[test]
fn memo_across_many_renders_is_returned_by_value() {
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let v1 = cx.use_memo((0_i32,), || Rc::new("hi".to_string()));

    cx.begin_render();
    let v2 = cx.use_memo((0_i32,), || Rc::new("ignored".to_string()));

    assert!(Rc::ptr_eq(&v1, &v2), "stable Rc across equal-deps renders");
}
