//! Typed handler for the `RatingControl` widget.

use super::EventCtx;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::RatingControl;
use crate::winui::backend::Handle;

pub fn mount(
    widget: &RatingControl,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::RatingControl(r) = handle else {
        return Ok(());
    };

    r.put_Value(widget.value)?;

    if let Some(max) = widget.max_rating {
        r.put_MaxRating(max)?;
    }
    if let Some(s) = &widget.caption {
        r.put_Caption(s.as_str())?;
    }
    if let Some(v) = widget.placeholder_value {
        r.put_PlaceholderValue(v)?;
    }
    if widget.is_read_only {
        r.put_IsReadOnly(true)?;
    }

    ctx.mount_event(
        &widget.on_changed,
        Event::RatingValueChanged,
        EventHandler::ValueChanged,
    );
    Ok(())
}

pub fn diff(
    old: &RatingControl,
    new: &RatingControl,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::RatingControl(r) = handle else {
        return Ok(());
    };

    if old.value != new.value {
        r.put_Value(new.value)?;
    }
    if old.max_rating != new.max_rating
        && let Some(max) = new.max_rating
    {
        r.put_MaxRating(max)?;
    }
    if old.caption != new.caption {
        match &new.caption {
            Some(s) => r.put_Caption(s.as_str())?,
            None => r.put_Caption("")?,
        }
    }
    if old.placeholder_value != new.placeholder_value
        && let Some(v) = new.placeholder_value
    {
        r.put_PlaceholderValue(v)?;
    }
    if old.is_read_only != new.is_read_only {
        r.put_IsReadOnly(new.is_read_only)?;
    }

    ctx.diff_event(
        &old.on_changed,
        &new.on_changed,
        Event::RatingValueChanged,
        EventHandler::ValueChanged,
    );
    Ok(())
}
