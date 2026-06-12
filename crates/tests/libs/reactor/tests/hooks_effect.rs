use std::cell::Cell;
use std::rc::Rc;

use windows_reactor::RenderCx;

#[test]
fn effect_runs_once_on_first_flush() {
    let fired = Rc::new(Cell::new(0));
    let mut cx = RenderCx::for_test();

    cx.begin_render();
    let fired_c = Rc::clone(&fired);
    cx.use_effect((), move || {
        fired_c.set(fired_c.get() + 1);
    });

    assert_eq!(fired.get(), 0);

    cx.flush_effects();
    assert_eq!(fired.get(), 1);
}

#[test]
fn effect_does_not_rerun_on_equal_deps() {
    let fired = Rc::new(Cell::new(0));
    let mut cx = RenderCx::for_test();

    cx.begin_render();
    let fired_c = Rc::clone(&fired);
    cx.use_effect((42_i32,), move || {
        fired_c.set(fired_c.get() + 1);
    });
    cx.flush_effects();
    assert_eq!(fired.get(), 1);

    cx.begin_render();
    let fired_c2 = Rc::clone(&fired);
    cx.use_effect((42_i32,), move || {
        fired_c2.set(fired_c2.get() + 1);
    });
    cx.flush_effects();
    assert_eq!(fired.get(), 1, "deps unchanged → no re-run");
}

#[test]
fn effect_reruns_when_deps_change() {
    let fired = Rc::new(Cell::new(0));
    let mut cx = RenderCx::for_test();

    cx.begin_render();
    let fired_c = Rc::clone(&fired);
    cx.use_effect((1_i32,), move || {
        fired_c.set(fired_c.get() + 1);
    });
    cx.flush_effects();
    assert_eq!(fired.get(), 1);

    cx.begin_render();
    let fired_c2 = Rc::clone(&fired);
    cx.use_effect((2_i32,), move || {
        fired_c2.set(fired_c2.get() + 1);
    });
    cx.flush_effects();
    assert_eq!(fired.get(), 2, "deps changed → effect runs again");
}

#[test]
fn cleanup_runs_before_next_effect() {
    let log: Rc<std::cell::RefCell<Vec<String>>> = Rc::new(std::cell::RefCell::new(Vec::new()));
    let mut cx = RenderCx::for_test();

    cx.begin_render();
    let log_c = Rc::clone(&log);
    cx.use_effect_with_cleanup((1_i32,), move || {
        log_c.borrow_mut().push("effect(1)".into());
        let log_c2 = Rc::clone(&log_c);
        Some(move || {
            log_c2.borrow_mut().push("cleanup(1)".into());
        })
    });
    cx.flush_effects();

    cx.begin_render();
    let log_c = Rc::clone(&log);
    cx.use_effect_with_cleanup((2_i32,), move || {
        log_c.borrow_mut().push("effect(2)".into());
        Some(|| {})
    });
    cx.flush_effects();

    assert_eq!(
        *log.borrow(),
        vec!["effect(1)", "cleanup(1)", "effect(2)"],
        "cleanup(1) must run before effect(2)"
    );
}

#[test]
fn cleanup_runs_on_unmount_via_run_cleanups() {
    let cleaned = Rc::new(Cell::new(false));
    let mut cx = RenderCx::for_test();

    cx.begin_render();
    let cleaned_c = Rc::clone(&cleaned);
    cx.use_effect_with_cleanup((), move || Some(move || cleaned_c.set(true)));
    cx.flush_effects();
    assert!(!cleaned.get(), "cleanup must not run yet");

    cx.run_cleanups();
    assert!(cleaned.get(), "unmount ran the cleanup");
}

#[test]
fn multiple_effects_flush_in_slot_order() {
    let log: Rc<std::cell::RefCell<Vec<i32>>> = Rc::new(std::cell::RefCell::new(Vec::new()));
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    for i in 0..4_i32 {
        let log_c = Rc::clone(&log);
        cx.use_effect((i,), move || {
            log_c.borrow_mut().push(i);
        });
    }
    cx.flush_effects();
    assert_eq!(*log.borrow(), vec![0, 1, 2, 3]);
}

#[test]
fn effect_without_cleanup_does_not_carry_state() {
    let fired = Rc::new(Cell::new(0));
    let mut cx = RenderCx::for_test();

    for i in 0..3_i32 {
        cx.begin_render();
        let fired_c = Rc::clone(&fired);
        cx.use_effect((i,), move || {
            fired_c.set(fired_c.get() + 1);
        });
        cx.flush_effects();
    }
    assert_eq!(fired.get(), 3);
}

#[test]
fn flush_effects_is_idempotent_for_stale_slots() {
    let fired = Rc::new(Cell::new(0));
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let fired_c = Rc::clone(&fired);
    cx.use_effect((), move || fired_c.set(fired_c.get() + 1));
    cx.flush_effects();
    cx.flush_effects();
    assert_eq!(fired.get(), 1);
}
