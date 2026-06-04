//! Typed handler for the `InfoBar` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::InfoBar;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::to_winui_info_bar_severity;

pub fn mount(w: &InfoBar, handle: &Handle, ctx: &mut EventCtx) -> windows_core::Result<()> {
    let ib = handle.cast_inner::<Xaml::IInfoBar>()?;
    if let Some(t) = &w.title {
        ib.put_Title(t.as_str())?;
    }
    if let Some(m) = &w.message {
        ib.put_Message(m.as_str())?;
    }
    ib.put_Severity(to_winui_info_bar_severity(w.severity))?;
    ib.put_IsOpen(w.is_open)?;
    if !w.is_closable {
        ib.put_IsClosable(false)?;
    }
    ctx.mount_event(&w.on_close, Event::InfoBarClosed, EventHandler::Click);
    Ok(())
}

pub fn diff(
    old: &InfoBar,
    new: &InfoBar,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let ib = handle.cast_inner::<Xaml::IInfoBar>()?;
    super::diff_opt!(
        old,
        new,
        title,
        |t| ib.put_Title(t.as_str()),
        ib.put_Title("")
    );
    super::diff_opt!(
        old,
        new,
        message,
        |m| ib.put_Message(m.as_str()),
        ib.put_Message("")
    );
    super::diff_val!(
        old,
        new,
        severity,
        ib.put_Severity(to_winui_info_bar_severity(new.severity))
    );
    super::diff_val!(old, new, is_open, ib.put_IsOpen(new.is_open));
    super::diff_val!(old, new, is_closable, ib.put_IsClosable(new.is_closable));
    ctx.diff_event(
        &old.on_close,
        &new.on_close,
        Event::InfoBarClosed,
        EventHandler::Click,
    );
    Ok(())
}
