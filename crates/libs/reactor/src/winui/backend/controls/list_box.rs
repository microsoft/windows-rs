//! Typed handler for the `ListBox` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::ListBoxWidget;
use crate::winui::backend::Handle;
use windows_core::Interface as _;

pub fn mount(
    widget: &ListBoxWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::ListBox(lb) = handle else {
        return Ok(());
    };

    if !widget.items.is_empty() {
        set_items(lb, &widget.items)?;
    }
    if let Some(idx) = widget.selected_index {
        lb.cast::<Xaml::ISelector>()?.put_SelectedIndex(idx)?;
    }

    ctx.mount_event(
        &widget.on_selection_changed,
        Event::ListBoxSelectionChanged,
        EventHandler::IndexChanged,
    );
    Ok(())
}

pub fn diff(
    old: &ListBoxWidget,
    new: &ListBoxWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::ListBox(lb) = handle else {
        return Ok(());
    };

    if old.items != new.items {
        set_items(lb, &new.items)?;
    }
    if old.selected_index != new.selected_index
        && let Some(idx) = new.selected_index
    {
        lb.cast::<Xaml::ISelector>()?.put_SelectedIndex(idx)?;
    }

    ctx.diff_event(
        &old.on_selection_changed,
        &new.on_selection_changed,
        Event::ListBoxSelectionChanged,
        EventHandler::IndexChanged,
    );
    Ok(())
}

fn set_items(lb: &Xaml::ListBox, items: &[String]) -> windows_core::Result<()> {
    let coll = lb
        .cast::<Xaml::IItemsControl>()?
        .get_Items()?
        .cast::<windows_collections::IVector<windows_core::IInspectable>>()?;
    coll.Clear()?;
    for s in items {
        let insp = windows_reference::IReference::from(s.as_str());
        coll.Append(&insp)?;
    }
    Ok(())
}
