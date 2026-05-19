#![cfg(all(test, windows))]

mod auto_bindings;
mod bindings;
use bindings::*;
use std::sync::{Mutex, MutexGuard};
use windows::{core::*, Foundation::*};

static STATIC_EVENT_TEST_LOCK: Mutex<()> = Mutex::new(());

fn lock_static_event_tests() -> MutexGuard<'static, ()> {
    STATIC_EVENT_TEST_LOCK.lock().unwrap()
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

mod auto_events {
    use super::auto_bindings::*;
    use windows::{core::*, Foundation::*};

    #[test]
    fn test_auto_revoker() -> Result<()> {
        let class: &'static Class = Box::leak(Box::new(Class::new()?));
        // In minimal mode, instance methods are only on the interface.
        let iclass = &windows_core::Interface::cast::<IClass>(class).unwrap();

        assert_eq!(0, iclass.Signal(1)?);

        // Auto-revoke on drop: register a handler in an inner scope, verify
        // it fires while live, then verify it's gone after the scope ends.
        {
            let _revoker = iclass.Event(&TypedEventHandler::new(move |_, _| Ok(())))?;
            assert_eq!(1, iclass.Signal(2)?);
        }
        assert_eq!(0, iclass.Signal(3)?);

        // Relying on Drop for revocation.
        let revoker = iclass.Event(&TypedEventHandler::new(move |_, _| Ok(())))?;
        assert_eq!(1, iclass.Signal(4)?);
        drop(revoker);
        assert_eq!(0, iclass.Signal(5)?);

        // into_token: recover the raw token without revoking.
        let token = iclass
            .Event(&TypedEventHandler::new(move |_, _| Ok(())))?
            .into_token();
        // Handler is still alive.
        assert_eq!(1, iclass.Signal(6)?);
        // Manually revoke via the vtable.
        unsafe {
            (windows_core::Interface::vtable(iclass).RemoveEvent)(
                windows_core::Interface::as_raw(iclass),
                token,
            )
            .ok()?;
        }
        assert_eq!(0, iclass.Signal(7)?);

        // Multiple revokers can be collected in a Vec.
        let revoker1 = iclass.Event(&TypedEventHandler::new(move |_, _| Ok(())))?;
        let revoker2 = iclass.Event(&TypedEventHandler::new(move |_, _| Ok(())))?;
        let revokers: Vec<EventRevoker<IClass>> = vec![revoker1, revoker2];
        assert_eq!(2, iclass.Signal(8)?);
        drop(revokers);
        assert_eq!(0, iclass.Signal(9)?);

        Ok(())
    }

    #[test]
    fn test_auto_revoker_static() -> Result<()> {
        let _lock = super::lock_static_event_tests();
        // Static event: auto-revoke on drop.
        {
            let _revoker = Class::StaticEvent(&EventHandler::new(move |_, _| Ok(())))?;
            assert_eq!(1, Class::StaticSignal(10)?);
        }
        assert_eq!(0, Class::StaticSignal(11)?);

        // into_token: recover the raw token without revoking.
        let revoker = Class::StaticEvent(&EventHandler::new(move |_, _| Ok(())))?;
        let token = revoker.into_token();
        assert_eq!(1, Class::StaticSignal(12)?);
        // Revoke via the IClassStatics vtable directly.
        unsafe {
            let statics = windows_core::factory::<Class, IClassStatics>()?;
            (windows_core::Interface::vtable(&statics).RemoveStaticEvent)(
                windows_core::Interface::as_raw(&statics),
                token,
            )
            .ok()?;
        }
        assert_eq!(0, Class::StaticSignal(13)?);

        Ok(())
    }
}
