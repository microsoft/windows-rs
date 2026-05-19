#![cfg(all(test, windows))]

mod auto_bindings;
mod bindings;
use bindings::*;
use windows::{core::*, Foundation::*};

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
    assert_eq!(0, Class::StaticSignal(1)?);

    let token = Class::StaticEvent(&EventHandler::new(move |_, args| {
        assert_eq!(args, 2);
        Ok(())
    }))?;

    assert_eq!(1, Class::StaticSignal(2)?);
    Class::RemoveStaticEvent(token)?;
    assert_eq!(0, Class::StaticSignal(3)?);

    Class::StaticEvent(&EventHandler::new(move |_, args| {
        assert_eq!(args, 4);
        Ok(())
    }))?;

    Class::StaticEvent(&EventHandler::new(move |_, args| {
        assert_eq!(args, 4);
        Ok(())
    }))?;

    assert_eq!(2, Class::StaticSignal(4)?);
    Ok(())
}

mod auto_events {
    use super::auto_bindings::*;
    use windows::{core::*, Foundation::*};

    #[test]
    fn test_auto_revoker() -> Result<()> {
        let class: &'static Class = Box::leak(Box::new(Class::new()?));

        assert_eq!(0, class.Signal(1)?);

        // Auto-revoke on drop: register a handler in an inner scope, verify
        // it fires while live, then verify it's gone after the scope ends.
        {
            let _revoker = class.Event(&TypedEventHandler::new(move |_, _| Ok(())))?;
            assert_eq!(1, class.Signal(2)?);
        }
        assert_eq!(0, class.Signal(3)?);

        // Explicit revoke via revoke().
        let revoker = class.Event(&TypedEventHandler::new(move |_, _| Ok(())))?;
        assert_eq!(1, class.Signal(4)?);
        revoker.revoke()?;
        assert_eq!(0, class.Signal(5)?);

        // Multiple revokers can be collected in a Vec.
        let mut revokers: Vec<EventRevoker<Class>> = Vec::new();
        revokers.push(class.Event(&TypedEventHandler::new(move |_, _| Ok(())))?);
        revokers.push(class.Event(&TypedEventHandler::new(move |_, _| Ok(())))?);
        assert_eq!(2, class.Signal(6)?);
        drop(revokers);
        assert_eq!(0, class.Signal(7)?);

        Ok(())
    }

    #[test]
    fn test_auto_revoker_static() -> Result<()> {
        // Static event: auto-revoke on drop.
        {
            let _revoker = Class::StaticEvent(&EventHandler::new(move |_, _| Ok(())))?;
            assert_eq!(1, Class::StaticSignal(10)?);
        }
        assert_eq!(0, Class::StaticSignal(11)?);

        // Explicit revoke.
        let revoker = Class::StaticEvent(&EventHandler::new(move |_, _| Ok(())))?;
        assert_eq!(1, Class::StaticSignal(12)?);
        revoker.revoke()?;
        assert_eq!(0, Class::StaticSignal(13)?);

        Ok(())
    }
}
