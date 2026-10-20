use super::*;
use Statics::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

pub fn test() -> Result<()> {
    assert_eq!(Widget::Add(1, 2)?, 3);

    let changed = Arc::new(AtomicI32::new(0));
    let revoker = Widget::Changed({
        let changed = changed.clone();
        move |sender, value| {
            assert!(sender.is_none());
            changed.store(value, Ordering::Relaxed);
        }
    })?;

    Widget::SetValue(5)?;
    assert_eq!(Widget::Value()?, 5);
    assert_eq!(changed.load(Ordering::Relaxed), 5);

    drop(revoker);
    Widget::SetValue(6)?;
    assert_eq!(Widget::Value()?, 6);
    assert_eq!(changed.load(Ordering::Relaxed), 5);

    Ok(())
}
