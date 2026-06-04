//! Typed handler for the `CalendarView` widget.

use super::EventCtx;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::CalendarViewWidget;
use crate::winui::backend::Handle;

pub fn mount(
    widget: &CalendarViewWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::CalendarView(cv) = handle else {
        return Ok(());
    };

    if let Some(v) = widget.is_today_highlighted {
        cv.put_IsTodayHighlighted(v)?;
    }
    if let Some(v) = widget.is_group_label_visible {
        cv.put_IsGroupLabelVisible(v)?;
    }

    ctx.mount_event(
        &widget.on_changed,
        Event::CalendarViewSelectionChanged,
        EventHandler::Click,
    );
    Ok(())
}

pub fn diff(
    old: &CalendarViewWidget,
    new: &CalendarViewWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::CalendarView(cv) = handle else {
        return Ok(());
    };

    if old.is_today_highlighted != new.is_today_highlighted
        && let Some(v) = new.is_today_highlighted
    {
        cv.put_IsTodayHighlighted(v)?;
    }
    if old.is_group_label_visible != new.is_group_label_visible
        && let Some(v) = new.is_group_label_visible
    {
        cv.put_IsGroupLabelVisible(v)?;
    }

    ctx.diff_event(
        &old.on_changed,
        &new.on_changed,
        Event::CalendarViewSelectionChanged,
        EventHandler::Click,
    );
    Ok(())
}
