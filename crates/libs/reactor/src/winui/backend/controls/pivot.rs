//! Typed handler for the `Pivot` widget (props only; child management stays in legacy path).

use crate::core::widgets::Pivot;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;

pub fn mount(widget: &Pivot, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::Pivot(p) = handle else {
        return Ok(false);
    };

    p.put_SelectedIndex(widget.selected_index)?;

    if let Some(t) = &widget.title {
        p.put_Title(&string_as_textblock(t)?)?;
    }

    Ok(true)
}

pub fn diff(old: &Pivot, new: &Pivot, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::Pivot(p) = handle else {
        return Ok(false);
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

    Ok(true)
}
