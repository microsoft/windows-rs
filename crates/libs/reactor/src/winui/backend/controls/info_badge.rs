//! Typed handler for the `InfoBadge` widget.

use super::EventCtx;
use crate::core::widgets::InfoBadge;
use crate::winui::backend::Handle;

pub fn mount(widget: &InfoBadge, handle: &Handle, _ctx: &mut EventCtx) -> windows_core::Result<()> {
    let Handle::InfoBadge(ib) = handle else {
        return Ok(());
    };

    let v = widget.value.unwrap_or(-1);
    ib.put_Value(v)?;

    Ok(())
}

pub fn diff(
    old: &InfoBadge,
    new: &InfoBadge,
    handle: &Handle,
    _ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::InfoBadge(ib) = handle else {
        return Ok(());
    };

    if old.value != new.value {
        let v = new.value.unwrap_or(-1);
        ib.put_Value(v)?;
    }

    Ok(())
}
