use std::sync::atomic::*;
use std::sync::*;
use windows::{core::*, Foundation::*};

#[test]
fn add_remove() -> Result<()> {
    let event = Event::<EventHandler<i32>>::new();

    // Remove a bogus event handler from an empty event source.
    event.remove(-123);

    let check = Arc::new(AtomicI32::new(0));
    let check_sender = check.clone();

    // Add event handler.
    event.add(&EventHandler::<i32>::new(move |_, args| {
        check_sender.store(*args, Ordering::Relaxed);
        Ok(())
    }))?;

    // Remove a bogus event handler from a non-empty event source.
    event.remove(-123);

    // Raise and observe event.
    assert_eq!(check.load(Ordering::Relaxed), 0);
    event.call(|delegate| delegate.Invoke(None, 123));
    assert_eq!(check.load(Ordering::Relaxed), 123);

    // Remove event handler.
    event.clear();

    // Raise event without effect.
    check.store(0, Ordering::Relaxed);
    event.call(|delegate| delegate.Invoke(None, 123));
    assert_eq!(check.load(Ordering::Relaxed), 0);

    Ok(())
}

#[test]
fn multiple() -> Result<()> {
    let event = Event::<EventHandler<i32>>::new();

    let a_check = Arc::new(AtomicI32::new(0));
    let a_check_sender = a_check.clone();
    let b_check = Arc::new(AtomicI32::new(0));
    let b_check_sender = b_check.clone();
    let c_check = Arc::new(AtomicI32::new(0));
    let c_check_sender = c_check.clone();

    assert_eq!(a_check.load(Ordering::Relaxed), 0);
    assert_eq!(b_check.load(Ordering::Relaxed), 0);
    assert_eq!(c_check.load(Ordering::Relaxed), 0);
    event.call(|delegate| delegate.Invoke(None, 10));
    assert_eq!(a_check.load(Ordering::Relaxed), 0);
    assert_eq!(b_check.load(Ordering::Relaxed), 0);
    assert_eq!(c_check.load(Ordering::Relaxed), 0);

    let a_token = event.add(&EventHandler::<i32>::new(move |_, args| {
        a_check_sender.store(*args, Ordering::Relaxed);
        Ok(())
    }))?;

    assert_eq!(a_check.load(Ordering::Relaxed), 0);
    assert_eq!(b_check.load(Ordering::Relaxed), 0);
    assert_eq!(c_check.load(Ordering::Relaxed), 0);
    event.call(|delegate| delegate.Invoke(None, 10));
    assert_eq!(a_check.load(Ordering::Relaxed), 10);
    assert_eq!(b_check.load(Ordering::Relaxed), 0);
    assert_eq!(c_check.load(Ordering::Relaxed), 0);

    let b_token = event.add(&EventHandler::<i32>::new(move |_, args| {
        b_check_sender.store(*args, Ordering::Relaxed);
        Ok(())
    }))?;

    assert_eq!(a_check.load(Ordering::Relaxed), 10);
    assert_eq!(b_check.load(Ordering::Relaxed), 0);
    assert_eq!(c_check.load(Ordering::Relaxed), 0);
    event.call(|delegate| delegate.Invoke(None, 20));
    assert_eq!(a_check.load(Ordering::Relaxed), 20);
    assert_eq!(b_check.load(Ordering::Relaxed), 20);
    assert_eq!(c_check.load(Ordering::Relaxed), 0);

    let c_token = event.add(&EventHandler::<i32>::new(move |_, args| {
        c_check_sender.store(*args, Ordering::Relaxed);
        Ok(())
    }))?;

    assert_eq!(a_check.load(Ordering::Relaxed), 20);
    assert_eq!(b_check.load(Ordering::Relaxed), 20);
    assert_eq!(c_check.load(Ordering::Relaxed), 0);
    event.call(|delegate| delegate.Invoke(None, 30));
    assert_eq!(a_check.load(Ordering::Relaxed), 30);
    assert_eq!(b_check.load(Ordering::Relaxed), 30);
    assert_eq!(c_check.load(Ordering::Relaxed), 30);

    event.remove(a_token);

    assert_eq!(a_check.load(Ordering::Relaxed), 30);
    assert_eq!(b_check.load(Ordering::Relaxed), 30);
    assert_eq!(c_check.load(Ordering::Relaxed), 30);
    event.call(|delegate| delegate.Invoke(None, 40));
    assert_eq!(a_check.load(Ordering::Relaxed), 30);
    assert_eq!(b_check.load(Ordering::Relaxed), 40);
    assert_eq!(c_check.load(Ordering::Relaxed), 40);

    event.remove(b_token);

    assert_eq!(a_check.load(Ordering::Relaxed), 30);
    assert_eq!(b_check.load(Ordering::Relaxed), 40);
    assert_eq!(c_check.load(Ordering::Relaxed), 40);
    event.call(|delegate| delegate.Invoke(None, 50));
    assert_eq!(a_check.load(Ordering::Relaxed), 30);
    assert_eq!(b_check.load(Ordering::Relaxed), 40);
    assert_eq!(c_check.load(Ordering::Relaxed), 50);

    event.remove(c_token);

    assert_eq!(a_check.load(Ordering::Relaxed), 30);
    assert_eq!(b_check.load(Ordering::Relaxed), 40);
    assert_eq!(c_check.load(Ordering::Relaxed), 50);
    event.call(|delegate| delegate.Invoke(None, 60));
    assert_eq!(a_check.load(Ordering::Relaxed), 30);
    assert_eq!(b_check.load(Ordering::Relaxed), 40);
    assert_eq!(c_check.load(Ordering::Relaxed), 50);

    Ok(())
}
