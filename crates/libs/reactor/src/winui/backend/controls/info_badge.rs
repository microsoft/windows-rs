//! Typed handler for the `InfoBadge` widget.

use crate::core::widgets::InfoBadge;
use crate::winui::backend::Handle;

pub fn mount(widget: &InfoBadge, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::InfoBadge(ib) = handle else {
        return Ok(false);
    };

    let v = widget.value.unwrap_or(-1);
    ib.put_Value(v)?;

    Ok(true)
}

pub fn diff(old: &InfoBadge, new: &InfoBadge, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::InfoBadge(ib) = handle else {
        return Ok(false);
    };

    if old.value != new.value {
        let v = new.value.unwrap_or(-1);
        ib.put_Value(v)?;
    }

    Ok(true)
}
