//! Typed handler for the `TitleBar` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::TitleBar;
use crate::winui::backend::Handle;

pub fn mount(w: &TitleBar, handle: &Handle, ctx: &mut EventCtx) -> windows_core::Result<()> {
    let tb = handle.cast_inner::<Xaml::TitleBar>()?;
    tb.put_Title(&w.title)?;
    if let Some(ref s) = w.subtitle {
        tb.put_Subtitle(s.as_str())?;
    }
    if w.is_back_button_visible {
        tb.put_IsBackButtonVisible(true)?;
    }
    if w.is_back_button_enabled {
        tb.put_IsBackButtonEnabled(true)?;
    }
    if w.is_pane_toggle_button_visible {
        tb.put_IsPaneToggleButtonVisible(true)?;
    }
    if w.is_tall {
        crate::winui::host::set_titlebar_height(true);
    }
    ctx.mount_event(
        &w.on_back_requested,
        Event::TitleBarBackRequested,
        EventHandler::Click,
    );
    ctx.mount_event(
        &w.on_pane_toggle_requested,
        Event::TitleBarPaneToggle,
        EventHandler::Click,
    );
    Ok(())
}

pub fn diff(
    old: &TitleBar,
    new: &TitleBar,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let tb = handle.cast_inner::<Xaml::TitleBar>()?;
    if new.title != old.title {
        tb.put_Title(&new.title)?;
    }
    if new.subtitle != old.subtitle {
        match &new.subtitle {
            Some(s) => tb.put_Subtitle(s.as_str())?,
            None => tb.put_Subtitle("")?,
        }
    }
    if new.is_back_button_visible != old.is_back_button_visible {
        tb.put_IsBackButtonVisible(new.is_back_button_visible)?;
    }
    if new.is_back_button_enabled != old.is_back_button_enabled {
        tb.put_IsBackButtonEnabled(new.is_back_button_enabled)?;
    }
    if new.is_pane_toggle_button_visible != old.is_pane_toggle_button_visible {
        tb.put_IsPaneToggleButtonVisible(new.is_pane_toggle_button_visible)?;
    }
    if new.is_tall != old.is_tall {
        crate::winui::host::set_titlebar_height(new.is_tall);
    }
    ctx.diff_event(
        &old.on_back_requested,
        &new.on_back_requested,
        Event::TitleBarBackRequested,
        EventHandler::Click,
    );
    ctx.diff_event(
        &old.on_pane_toggle_requested,
        &new.on_pane_toggle_requested,
        Event::TitleBarPaneToggle,
        EventHandler::Click,
    );
    Ok(())
}
