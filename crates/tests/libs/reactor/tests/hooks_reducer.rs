use std::cell::Cell;
use std::rc::Rc;

use windows_reactor::core::render_context::RenderCx;

#[test]
fn updater_first_render_returns_initial() {
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let (v, _update) = cx.use_reducer(10_i32);
    assert_eq!(v, 10);
}

#[test]
fn updater_next_render_sees_reduced_value() {
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let (_, update) = cx.use_reducer(1_i32);
    update.call(|v| v + 2);
    update.call(|v| v + 10);

    cx.begin_render();
    let (v, _) = cx.use_reducer(0_i32);
    assert_eq!(v, 13);
}

#[test]
fn updater_vec_push_triggers_rerender() {
    let fired = Rc::new(Cell::new(0));
    let fired_c = Rc::clone(&fired);
    let mut cx = RenderCx::new(Rc::new(move || fired_c.set(fired_c.get() + 1)));

    cx.begin_render();
    let (_, update) = cx.use_reducer::<Vec<i32>>(Vec::new());

    update.call(|mut v| {
        v.push(1);
        v
    });
    assert_eq!(fired.get(), 1, "push via reducer must re-render");

    update.call(|mut v| {
        v.push(2);
        v
    });
    assert_eq!(fired.get(), 2);

    cx.begin_render();
    let (v, _) = cx.use_reducer::<Vec<i32>>(Vec::new());
    assert_eq!(v, vec![1, 2]);
}

#[test]
fn updater_identical_returns_dont_rerender() {
    let fired = Rc::new(Cell::new(0));
    let fired_c = Rc::clone(&fired);
    let mut cx = RenderCx::new(Rc::new(move || fired_c.set(fired_c.get() + 1)));

    cx.begin_render();
    let (_, update) = cx.use_reducer(5_i32);

    update.call(|v| v);
    assert_eq!(fired.get(), 0);

    update.call(|_| 5);
    assert_eq!(fired.get(), 0);

    update.call(|v| v + 1);
    assert_eq!(fired.get(), 1);
}

#[test]
fn reducer_fn_dispatch_applies_reducer() {
    #[derive(Clone, PartialEq, Debug, Default)]
    struct Counter {
        value: i32,
    }
    enum Action {
        Inc,
        Dec,
        Set(i32),
    }
    let reducer = |s: Counter, a: Action| -> Counter {
        match a {
            Action::Inc => Counter { value: s.value + 1 },
            Action::Dec => Counter { value: s.value - 1 },
            Action::Set(v) => Counter { value: v },
        }
    };

    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let (s, dispatch) = cx.use_reducer_fn(reducer, Counter::default());
    assert_eq!(s.value, 0);

    dispatch.call(Action::Inc);
    dispatch.call(Action::Inc);
    dispatch.call(Action::Set(42));
    dispatch.call(Action::Dec);

    cx.begin_render();
    let (s2, _) = cx.use_reducer_fn(reducer, Counter::default());
    assert_eq!(s2.value, 41);
}

#[test]
fn reducer_fn_identical_output_does_not_rerender() {
    #[derive(Clone, PartialEq)]
    struct S(i32);
    let reducer = |s: S, a: i32| S(s.0.max(a));

    let fired = Rc::new(Cell::new(0));
    let fired_c = Rc::clone(&fired);
    let mut cx = RenderCx::new(Rc::new(move || fired_c.set(fired_c.get() + 1)));
    cx.begin_render();
    let (_, dispatch) = cx.use_reducer_fn(reducer, S(10));

    dispatch.call(5);
    assert_eq!(fired.get(), 0);

    dispatch.call(11);
    assert_eq!(fired.get(), 1);
}

#[test]
fn updater_handle_can_be_cloned() {
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let (_, update) = cx.use_reducer(0_i32);
    let update2 = update.clone();

    update.call(|v| v + 1);
    update2.call(|v| v + 10);

    cx.begin_render();
    let (v, _) = cx.use_reducer(0_i32);
    assert_eq!(v, 11);
}
