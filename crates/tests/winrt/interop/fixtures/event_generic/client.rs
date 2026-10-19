use super::*;
use EventGeneric::*;
use std::sync::{Arc, Mutex};

pub fn test() -> Result<()> {
    let widget = Widget::new()?;
    let received = Arc::new(Mutex::new((None, 0)));

    let revoker = widget.Changed({
        let received = received.clone();
        move |sender, value| *received.lock().unwrap() = (sender.cloned(), value)
    })?;

    widget.Raise(1)?;
    assert_eq!(*received.lock().unwrap(), (Some(widget.clone()), 1));

    drop(revoker);
    widget.Raise(2)?;
    assert_eq!(received.lock().unwrap().1, 1);

    Ok(())
}
