//! Typed handler for the `Pivot` widget (props only; child management stays in legacy path).

use super::EventCtx;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::Pivot;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;

pub fn mount(widget: &Pivot, handle: &Handle, ctx: &mut EventCtx) -> windows_core::Result<()> {
    let Handle::Pivot(p) = handle else {
        return Ok(());
    };

    p.put_SelectedIndex(widget.selected_index)?;

    if let Some(t) = &widget.title {
        p.put_Title(&string_as_textblock(t)?)?;
    }

    ctx.mount_event(
        &widget.on_selection_changed,
        Event::PivotSelectionChanged,
        EventHandler::IndexChanged,
    );
    Ok(())
}

pub fn diff(
    old: &Pivot,
    new: &Pivot,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::Pivot(p) = handle else {
        return Ok(());
    };

    if old.selected_index != new.selected_index {
        p.put_SelectedIndex(new.selected_index)?;
    }
    if old.title != new.title {
        match &new.title {
            Some(t) => p.put_Title(&string_as_textblock(t)?)?,
            None => p.put_Title(None)?,
        }
    }

    ctx.diff_event(
        &old.on_selection_changed,
        &new.on_selection_changed,
        Event::PivotSelectionChanged,
        EventHandler::IndexChanged,
    );
    Ok(())
}
