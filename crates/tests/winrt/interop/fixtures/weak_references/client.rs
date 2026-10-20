use super::*;
use WeakReferences::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

pub fn test() -> Result<()> {
    let widget = Widget::new()?;
    let value = Arc::new(AtomicI32::new(0));

    let observer: IObserver = Observer(value.clone()).into();
    widget.Observe(&observer)?;
    assert!(widget.Notify(1)?);
    assert_eq!(value.load(Ordering::Relaxed), 1);

    drop(observer);
    assert!(!widget.Notify(2)?);
    assert_eq!(value.load(Ordering::Relaxed), 1);

    let weak = widget.downgrade()?;
    assert_eq!(weak.upgrade().as_ref(), Some(&widget));
    drop(widget);
    assert!(weak.upgrade().is_none());

    Ok(())
}

#[implement(IObserver)]
struct Observer(Arc<AtomicI32>);

impl IObserver_Impl for Observer_Impl {
    fn Notify(&self, value: i32) -> Result<()> {
        self.0.store(value, Ordering::Relaxed);
        Ok(())
    }
}
