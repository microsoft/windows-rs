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
use windows::{Foundation::*, core::*};

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

    let token = class.Event(&TypedEventHandler::new(
        move |sender: Ref<Class>, args: i32| {
            assert_eq!(sender.as_ref().unwrap(), class);
            assert_eq!(args, 2);
            Ok(())
        },
    ))?;

    assert_eq!(1, class.Signal(2)?);
    class.RemoveEvent(token)?;
    assert_eq!(0, class.Signal(3)?);

    class.Event(&TypedEventHandler::new(
        move |sender: Ref<Class>, args: i32| {
            assert_eq!(sender.as_ref().unwrap(), class);
            assert_eq!(args, 4);
            Ok(())
        },
    ))?;

    class.Event(&TypedEventHandler::new(
        move |sender: Ref<Class>, args: i32| {
            assert_eq!(sender.as_ref().unwrap(), class);
            assert_eq!(args, 4);
            Ok(())
        },
    ))?;

    assert_eq!(2, class.Signal(4)?);
    Ok(())
}

#[test]
fn test_static() -> Result<()> {
    let _lock = lock_static_event_tests();
    assert_eq!(0, Class::StaticSignal(1)?);

    let token = Class::StaticEvent(&EventHandler::new(move |_, args| {
        assert_eq!(args, 2);
        Ok(())
    }))?;

    assert_eq!(1, Class::StaticSignal(2)?);
    Class::RemoveStaticEvent(token)?;
    assert_eq!(0, Class::StaticSignal(3)?);

    let token1 = Class::StaticEvent(&EventHandler::new(move |_, args| {
        assert_eq!(args, 4);
        Ok(())
    }))?;

    let token2 = Class::StaticEvent(&EventHandler::new(move |_, args| {
        assert_eq!(args, 4);
        Ok(())
    }))?;

    assert_eq!(2, Class::StaticSignal(4)?);
    Class::RemoveStaticEvent(token1)?;
    Class::RemoveStaticEvent(token2)?;
    Ok(())
}
