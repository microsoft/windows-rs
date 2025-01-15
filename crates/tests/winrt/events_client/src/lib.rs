#![cfg(test)]

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
        move |sender: Ref<Class>, args: Ref<i32>| {
            assert_eq!(sender.as_ref().unwrap(), class);
            assert_eq!(*args, 2);
            Ok(())
        },
    ))?;

    assert_eq!(1, class.Signal(2)?);
    class.RemoveEvent(token)?;
    assert_eq!(0, class.Signal(3)?);

    class.Event(&TypedEventHandler::new(
        // TODO: ideally generics also use Ref<T> here
        move |sender: Ref<Class>, args: Ref<i32>| {
            assert_eq!(sender.as_ref().unwrap(), class);
            assert_eq!(*args, 4);
            Ok(())
        },
    ))?;

    class.Event(&TypedEventHandler::new(
        move |sender: Ref<Class>, args: Ref<i32>| {
            assert_eq!(sender.as_ref().unwrap(), class);
            assert_eq!(*args, 4);
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
        assert_eq!(*args, 2);
        Ok(())
    }))?;

    assert_eq!(1, Class::StaticSignal(2)?);
    Class::RemoveStaticEvent(token)?;
    assert_eq!(0, Class::StaticSignal(3)?);

    Class::StaticEvent(&EventHandler::new(move |_, args| {
        assert_eq!(*args, 4);
        Ok(())
    }))?;

    Class::StaticEvent(&EventHandler::new(move |_, args| {
        assert_eq!(*args, 4);
        Ok(())
    }))?;

    assert_eq!(2, Class::StaticSignal(4)?);
    Ok(())
}
