//! Typed handler for the `TabView` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::TabView;
use crate::winui::backend::Handle;

pub fn mount(w: &TabView, handle: &Handle, ctx: &mut EventCtx) -> windows_core::Result<()> {
    let tv = handle.cast_inner::<Xaml::TabView>()?;
    tv.put_SelectedIndex(w.selected_index)?;
    if w.can_reorder_tabs {
        tv.put_CanReorderTabs(true)?;
    }
    if w.is_add_tab_button_visible {
        tv.put_IsAddTabButtonVisible(true)?;
    }
    ctx.mount_event(
        &w.on_selection_changed,
        Event::TabSelectionChanged,
        EventHandler::IndexChanged,
    );
    ctx.mount_event(
        &w.on_tab_close_requested,
        Event::TabCloseRequested,
        EventHandler::TabKey,
    );
    ctx.mount_event(
        &w.on_add_tab_button_click,
        Event::AddTabButtonClick,
        EventHandler::Click,
    );
    Ok(())
}

pub fn diff(
    old: &TabView,
    new: &TabView,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let tv = handle.cast_inner::<Xaml::TabView>()?;
    if new.selected_index != old.selected_index {
        tv.put_SelectedIndex(new.selected_index)?;
    }
    if new.can_reorder_tabs != old.can_reorder_tabs {
        tv.put_CanReorderTabs(new.can_reorder_tabs)?;
    }
    if new.is_add_tab_button_visible != old.is_add_tab_button_visible {
        tv.put_IsAddTabButtonVisible(new.is_add_tab_button_visible)?;
    }
    ctx.diff_event(
        &old.on_selection_changed,
        &new.on_selection_changed,
        Event::TabSelectionChanged,
        EventHandler::IndexChanged,
    );
    ctx.diff_event(
        &old.on_tab_close_requested,
        &new.on_tab_close_requested,
        Event::TabCloseRequested,
        EventHandler::TabKey,
    );
    ctx.diff_event(
        &old.on_add_tab_button_click,
        &new.on_add_tab_button_click,
        Event::AddTabButtonClick,
        EventHandler::Click,
    );
    Ok(())
}
