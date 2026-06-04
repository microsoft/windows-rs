//! Typed handler for the `SplitView` widget.

use crate::bindings as Xaml;
use crate::core::widgets::SplitViewWidget;
use crate::winui::backend::Handle;

pub fn mount(widget: &SplitViewWidget, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::SplitView(sv) = handle else {
        return Ok(false);
    };

    sv.put_DisplayMode(Xaml::SplitViewDisplayMode(widget.display_mode as i32))?;
    sv.put_IsPaneOpen(widget.is_pane_open)?;

    if let Some(len) = widget.open_pane_length {
        sv.put_OpenPaneLength(len)?;
    }
    if let Some(len) = widget.compact_pane_length {
        sv.put_CompactPaneLength(len)?;
    }

    Ok(true)
}

pub fn diff(
    old: &SplitViewWidget,
    new: &SplitViewWidget,
    handle: &Handle,
) -> windows_core::Result<bool> {
    let Handle::SplitView(sv) = handle else {
        return Ok(false);
    };

    if old.display_mode != new.display_mode {
        sv.put_DisplayMode(Xaml::SplitViewDisplayMode(new.display_mode as i32))?;
    }
    if old.is_pane_open != new.is_pane_open {
        sv.put_IsPaneOpen(new.is_pane_open)?;
    }
    if old.open_pane_length != new.open_pane_length {
        if let Some(len) = new.open_pane_length {
            sv.put_OpenPaneLength(len)?;
        }
    }
    if old.compact_pane_length != new.compact_pane_length {
        if let Some(len) = new.compact_pane_length {
            sv.put_CompactPaneLength(len)?;
        }
    }

    Ok(true)
}
