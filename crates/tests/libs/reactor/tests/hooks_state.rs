use std::cell::Cell;
use std::rc::Rc;

use windows_reactor::core::render_context::RenderCx;

#[test]
fn first_render_returns_initial() {
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let (v, _) = cx.use_state(0_i32);
    assert_eq!(v, 0);
}

#[test]
fn setter_value_persists_across_renders() {
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let (_, set) = cx.use_state(0_i32);
    set.call(42);

    cx.begin_render();
    let (v, _) = cx.use_state(0_i32);
    assert_eq!(v, 42);
}

#[test]
fn identical_setter_value_does_not_trigger_rerender() {
    let fired = Rc::new(Cell::new(0));
    let fired_c = Rc::clone(&fired);
    let mut cx = RenderCx::new(Rc::new(move || fired_c.set(fired_c.get() + 1)));

    cx.begin_render();
    let (_, set) = cx.use_state("hi".to_string());
    set.call("hi".into());
    assert_eq!(fired.get(), 0);
}

#[test]
fn distinct_setter_value_fires_rerender() {
    let fired = Rc::new(Cell::new(0));
    let fired_c = Rc::clone(&fired);
    let mut cx = RenderCx::new(Rc::new(move || fired_c.set(fired_c.get() + 1)));

    cx.begin_render();
    let (_, set) = cx.use_state("hi".to_string());
    set.call("bye".into());
    assert_eq!(fired.get(), 1);
}

#[test]
#[should_panic(expected = "hook called in different order")]
fn hook_order_change_panics() {
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let _ = cx.use_state(1_i32);

    cx.begin_render();

    let _ = cx.use_state("bad".to_string());
}

#[test]
fn cloned_setter_invokes_same_slot() {
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let (_, set) = cx.use_state(0_i32);
    let set2 = set.clone();
    set.call(5);
    set2.call(12);

    cx.begin_render();
    let (v, _) = cx.use_state(0_i32);
    assert_eq!(v, 12);
}

#[test]
fn independent_state_slots_are_independent() {
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let (a, set_a) = cx.use_state(1_i32);
    let (b, set_b) = cx.use_state(1_i32);
    assert_eq!(a, 1);
    assert_eq!(b, 1);

    set_a.call(10);
    cx.begin_render();
    let (a2, _) = cx.use_state(0);
    let (b2, _) = cx.use_state(0);
    assert_eq!(a2, 10);
    assert_eq!(b2, 1);

    set_b.call(20);
    cx.begin_render();
    let (a3, _) = cx.use_state(0);
    let (b3, _) = cx.use_state(0);
    assert_eq!(a3, 10);
    assert_eq!(b3, 20);
}
