//! Typed handler for the `RadioButtons` widget (grouped radio options).

use crate::bindings as Xaml;
use crate::core::widgets::RadioButtons;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;

pub fn mount(widget: &RadioButtons, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::RadioButtons(r) = handle else {
        return Ok(false);
    };

    if let Some(h) = &widget.header {
        r.put_Header(&string_as_textblock(h)?)?;
    }

    set_items(r, &widget.items)?;
    r.put_SelectedIndex(widget.selected_index)?;

    if let Some(n) = widget.max_columns {
        r.put_MaxColumns(n)?;
    }

    Ok(true)
}

pub fn diff(old: &RadioButtons, new: &RadioButtons, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::RadioButtons(r) = handle else {
        return Ok(false);
    };

    if old.header != new.header {
        match &new.header {
            Some(h) => r.put_Header(&string_as_textblock(h)?)?,
            None => r.put_Header(None)?,
        }
    }
    if old.items != new.items {
        set_items(r, &new.items)?;
    }
    if old.selected_index != new.selected_index {
        r.put_SelectedIndex(new.selected_index)?;
    }
    if old.max_columns != new.max_columns {
        if let Some(n) = new.max_columns {
            r.put_MaxColumns(n)?;
        }
    }

    Ok(true)
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
