//! Typed handler for the `InfoBar` widget.

use crate::bindings as Xaml;
use crate::core::widgets::InfoBar;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::to_winui_info_bar_severity;

pub fn mount(w: &InfoBar, handle: &Handle) -> windows_core::Result<()> {
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
    Ok(())
}

pub fn diff(old: &InfoBar, new: &InfoBar, handle: &Handle) -> windows_core::Result<()> {
    let ib = handle.cast_inner::<Xaml::IInfoBar>()?;
    if new.title != old.title {
        match &new.title {
            Some(t) => ib.put_Title(t.as_str())?,
            None => ib.put_Title("")?,
        }
    }
    if new.message != old.message {
        match &new.message {
            Some(m) => ib.put_Message(m.as_str())?,
            None => ib.put_Message("")?,
        }
    }
    if new.severity != old.severity {
        ib.put_Severity(to_winui_info_bar_severity(new.severity))?;
    }
    if new.is_open != old.is_open {
        ib.put_IsOpen(new.is_open)?;
    }
    if new.is_closable != old.is_closable {
        ib.put_IsClosable(new.is_closable)?;
    }
    Ok(())
}
