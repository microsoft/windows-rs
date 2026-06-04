//! Typed handler for the `PersonPicture` widget.

use crate::core::widgets::PersonPicture;
use crate::winui::backend::Handle;

pub fn mount(widget: &PersonPicture, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::PersonPicture(pp) = handle else {
        return Ok(false);
    };

    if let Some(s) = &widget.display_name {
        pp.put_DisplayName(s.as_str())?;
    }
    if let Some(s) = &widget.initials {
        pp.put_Initials(s.as_str())?;
    }

    Ok(true)
}

pub fn diff(
    old: &PersonPicture,
    new: &PersonPicture,
    handle: &Handle,
) -> windows_core::Result<bool> {
    let Handle::PersonPicture(pp) = handle else {
        return Ok(false);
    };

    if old.display_name != new.display_name {
        let s = new.display_name.as_deref().unwrap_or("");
        pp.put_DisplayName(s)?;
    }
    if old.initials != new.initials {
        let s = new.initials.as_deref().unwrap_or("");
        pp.put_Initials(s)?;
    }

    Ok(true)
}
