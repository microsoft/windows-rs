//! Typed handler for the `ListBox` widget.

use crate::bindings as Xaml;
use crate::core::widgets::ListBoxWidget;
use crate::winui::backend::Handle;
use windows_core::Interface as _;

pub fn mount(widget: &ListBoxWidget, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::ListBox(lb) = handle else {
        return Ok(false);
    };

    if !widget.items.is_empty() {
        set_items(lb, &widget.items)?;
    }
    if let Some(idx) = widget.selected_index {
        lb.cast::<Xaml::ISelector>()?.put_SelectedIndex(idx)?;
    }

    Ok(true)
}

pub fn diff(
    old: &ListBoxWidget,
    new: &ListBoxWidget,
    handle: &Handle,
) -> windows_core::Result<bool> {
    let Handle::ListBox(lb) = handle else {
        return Ok(false);
    };

    if old.items != new.items {
        set_items(lb, &new.items)?;
    }
    if old.selected_index != new.selected_index {
        if let Some(idx) = new.selected_index {
            lb.cast::<Xaml::ISelector>()?.put_SelectedIndex(idx)?;
        }
    }

    Ok(true)
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
