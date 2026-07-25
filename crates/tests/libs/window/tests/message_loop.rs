//! The non-blocking message loop helpers observe quit messages.

use std::cell::Cell;
use std::rc::Rc;
use windows_window::{pump, quit, run_with};

#[test]
fn pump_returns_false_after_quit() {
    assert!(pump());
    quit();
    assert!(!pump());
}

#[test]
fn run_with_stops_when_render_quits() {
    let count = Rc::new(Cell::new(0));
    let calls = count.clone();
    let result = run_with(move || {
        calls.set(calls.get() + 1);
        quit();
        Ok(true)
    });
    assert!(result.is_ok());
    assert_eq!(count.get(), 1);
}
