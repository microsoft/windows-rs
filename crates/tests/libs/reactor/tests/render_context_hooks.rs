use std::cell::Cell;
use std::rc::Rc;

use windows_reactor::RenderCx;

#[test]
fn use_state_returns_initial_on_first_render() {
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let (v, _set) = cx.use_state(42_i32);
    assert_eq!(v, 42);
}

#[test]
fn setter_updates_state_across_renders() {
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let (_, set) = cx.use_state(0_i32);
    set.call(7);

    cx.begin_render();
    let (v, _) = cx.use_state(0_i32);
    assert_eq!(v, 7);
}

#[test]
fn setter_with_identical_value_does_not_request_rerender() {
    let calls = Rc::new(Cell::new(0_i32));
    let calls_c = Rc::clone(&calls);
    let mut cx = RenderCx::new(Rc::new(move || calls_c.set(calls_c.get() + 1)));
    cx.begin_render();
    let (_, set) = cx.use_state(5_i32);

    set.call(5);
    assert_eq!(calls.get(), 0, "identical set must not request rerender");

    set.call(6);
    assert_eq!(calls.get(), 1, "distinct set must request rerender");

    set.call(6);
    assert_eq!(calls.get(), 1);
}

#[test]
fn setter_fires_rerender_on_change() {
    let fired = Rc::new(Cell::new(false));
    let fired_c = Rc::clone(&fired);
    let mut cx = RenderCx::new(Rc::new(move || fired_c.set(true)));
    cx.begin_render();
    let (_, set) = cx.use_state(1_i32);
    set.call(2);
    assert!(fired.get());
}

#[test]
fn multiple_use_state_calls_map_to_distinct_slots() {
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let (a, set_a) = cx.use_state(1_i32);
    let (b, set_b) = cx.use_state("x".to_string());
    assert_eq!(a, 1);
    assert_eq!(b, "x");

    set_a.call(10);
    set_b.call("y".to_string());

    cx.begin_render();
    let (a2, _) = cx.use_state(999_i32);
    let (b2, _) = cx.use_state("zzz".to_string());
    assert_eq!(a2, 10);
    assert_eq!(b2, "y");
}

#[test]
fn set_state_handle_can_be_cloned_and_invoked_from_either_clone() {
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let (_, set) = cx.use_state(0_i32);
    let set2 = set.clone();
    set.call(3);
    set2.call(9);

    cx.begin_render();
    let (v, _) = cx.use_state(0_i32);
    assert_eq!(v, 9);
}

#[test]
#[should_panic(expected = "hook called in different order: slot 0")]
fn hook_order_mismatch_panics() {
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let _ = cx.use_state(1_i32);

    cx.begin_render();
    let _ = cx.use_state("oops".to_string());
}

#[test]
#[should_panic(expected = "slot 0 was `i32`, now called as `use_state::<alloc::string::String>`")]
fn hook_order_mismatch_names_previous_type() {
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let _ = cx.use_state(1_i32);

    cx.begin_render();
    let _ = cx.use_state("oops".to_string());
}

#[test]
fn hook_count_grows_on_first_render_and_stays_stable() {
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let _ = cx.use_state(1_i32);
    let _ = cx.use_state(2_i32);
    assert_eq!(cx.hook_count(), 2);

    cx.begin_render();
    let _ = cx.use_state(0_i32);
    let _ = cx.use_state(0_i32);
    assert_eq!(cx.hook_count(), 2);
}
