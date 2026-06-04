//! Typed handler for the `ComboBox` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::ComboBox;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;
use windows_core::Interface as _;

pub fn mount(widget: &ComboBox, handle: &Handle, ctx: &mut EventCtx) -> windows_core::Result<()> {
    let Handle::ComboBox(c) = handle else {
        return Ok(());
    };

    set_items(c, &widget.items)?;

    c.cast::<Xaml::ISelector>()?
        .put_SelectedIndex(widget.selected_index)?;

    if let Some(h) = &widget.header {
        c.put_Header(&string_as_textblock(h)?)?;
    }
    if let Some(p) = &widget.placeholder {
        c.put_PlaceholderText(p.as_str())?;
    }
    if widget.is_editable {
        c.put_IsEditable(true)?;
    }

    ctx.mount_event(
        &widget.on_selection_changed,
        Event::ComboSelectionChanged,
        EventHandler::IndexChanged,
    );
    Ok(())
}

pub fn diff(
    old: &ComboBox,
    new: &ComboBox,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::ComboBox(c) = handle else {
        return Ok(());
    };

    if old.items != new.items {
        set_items(c, &new.items)?;
    }

    super::diff_val!(
        old,
        new,
        selected_index,
        c.cast::<Xaml::ISelector>()?
            .put_SelectedIndex(new.selected_index)
    );

    super::diff_opt!(
        old,
        new,
        header,
        |h| c.put_Header(&string_as_textblock(h)?),
        c.put_Header(None)
    );

    super::diff_opt!(
        old,
        new,
        placeholder,
        |p| c.put_PlaceholderText(p.as_str()),
        c.put_PlaceholderText("")
    );

    super::diff_val!(old, new, is_editable, c.put_IsEditable(new.is_editable));

    ctx.diff_event(
        &old.on_selection_changed,
        &new.on_selection_changed,
        Event::ComboSelectionChanged,
        EventHandler::IndexChanged,
    );
    Ok(())
}

fn set_items(c: &Xaml::ComboBox, items: &[String]) -> windows_core::Result<()> {
    let coll = c
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
