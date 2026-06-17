#![cfg(all(test, windows))]

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
mod bindings;
use bindings::*;
use std::sync::{Mutex, MutexGuard};
use windows_core::*;

static STATIC_EVENT_TEST_LOCK: Mutex<()> = Mutex::new(());

fn lock_static_event_tests() -> MutexGuard<'static, ()> {
    STATIC_EVENT_TEST_LOCK
        .lock()
        .expect("static event test lock poisoned")
}

#[test]
fn test() -> Result<()> {
    // The static just simplifies testing identity in closures.
    let class: &'static Class = Box::leak(Box::new(Class::new()?));

    // Signal returns how many delegates were called.
    // The signal value is passed to each delegate.
    assert_eq!(0, class.Signal(1)?);

    let revoker = class.Event(move |sender: Ref<Class>, args: i32| {
        assert_eq!(sender.as_ref().unwrap(), class);
        assert_eq!(args, 2);
    })?;

    assert_eq!(1, class.Signal(2)?);
    drop(revoker);
    assert_eq!(0, class.Signal(3)?);

    // Handlers without a stored revoker are immediately revoked on drop.
    // Use .forget() to keep them alive indefinitely.
    class.Event(move |sender: Ref<Class>, args: i32| {
        assert_eq!(sender.as_ref().unwrap(), class);
        assert_eq!(args, 4);
    })?.forget();

    class.Event(move |sender: Ref<Class>, args: i32| {
        assert_eq!(sender.as_ref().unwrap(), class);
        assert_eq!(args, 4);
    })?.forget();

    assert_eq!(2, class.Signal(4)?);
    Ok(())
}

#[test]
fn test_static() -> Result<()> {
    let _lock = lock_static_event_tests();
    assert_eq!(0, Class::StaticSignal(1)?);

    let revoker = Class::StaticEvent(move |_, args| {
        assert_eq!(args, 2);
    })?;

    assert_eq!(1, Class::StaticSignal(2)?);
    drop(revoker);
    assert_eq!(0, Class::StaticSignal(3)?);

    Class::StaticEvent(move |_, args| {
        assert_eq!(args, 4);
    })?.forget();

    Class::StaticEvent(move |_, args| {
        assert_eq!(args, 4);
    })?.forget();

    assert_eq!(2, Class::StaticSignal(4)?);
    Ok(())
}
