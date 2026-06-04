//! Typed handler for the `PersonPicture` widget.

use super::EventCtx;
use crate::core::widgets::PersonPicture;
use crate::winui::backend::Handle;

pub fn mount(
    widget: &PersonPicture,
    handle: &Handle,
    _ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::PersonPicture(pp) = handle else {
        return Ok(());
    };

    if let Some(s) = &widget.display_name {
        pp.put_DisplayName(s.as_str())?;
    }
    if let Some(s) = &widget.initials {
        pp.put_Initials(s.as_str())?;
    }

    Ok(())
}

pub fn diff(
    old: &PersonPicture,
    new: &PersonPicture,
    handle: &Handle,
    _ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::PersonPicture(pp) = handle else {
        return Ok(());
    };

    super::diff_opt!(
        old,
        new,
        display_name,
        |s| pp.put_DisplayName(s.as_str()),
        pp.put_DisplayName("")
    );
    super::diff_opt!(
        old,
        new,
        initials,
        |s| pp.put_Initials(s.as_str()),
        pp.put_Initials("")
    );

    Ok(())
}
