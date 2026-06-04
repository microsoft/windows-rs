//! Typed handler for the `RadioButtons` widget (grouped radio options).

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::RadioButtons;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;

pub fn mount(
    widget: &RadioButtons,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::RadioButtons(r) = handle else {
        return Ok(());
    };

    if let Some(h) = &widget.header {
        r.put_Header(&string_as_textblock(h)?)?;
    }

    set_items(r, &widget.items)?;
    r.put_SelectedIndex(widget.selected_index)?;

    if let Some(n) = widget.max_columns {
        r.put_MaxColumns(n)?;
    }

    ctx.mount_event(
        &widget.on_selection_changed,
        Event::RadioButtonsSelectionChanged,
        EventHandler::IndexChanged,
    );
    Ok(())
}

pub fn diff(
    old: &RadioButtons,
    new: &RadioButtons,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::RadioButtons(r) = handle else {
        return Ok(());
    };

    super::diff_opt!(
        old,
        new,
        header,
        |h| r.put_Header(&string_as_textblock(h)?),
        r.put_Header(None)
    );
    if old.items != new.items {
        set_items(r, &new.items)?;
    }
    super::diff_val!(
        old,
        new,
        selected_index,
        r.put_SelectedIndex(new.selected_index)
    );
    if old.max_columns != new.max_columns
        && let Some(n) = new.max_columns
    {
        r.put_MaxColumns(n)?;
    }

    ctx.diff_event(
        &old.on_selection_changed,
        &new.on_selection_changed,
        Event::RadioButtonsSelectionChanged,
        EventHandler::IndexChanged,
    );
    Ok(())
}

fn set_items(r: &Xaml::RadioButtons, items: &[String]) -> windows_core::Result<()> {
    let vec = r.get_Items()?;
    vec.Clear()?;
    for s in items {
        let insp = windows_reference::IReference::from(s.as_str());
        vec.Append(&insp)?;
    }
    Ok(())
}
