//! Typed handler for the `CalendarView` widget.

use crate::core::widgets::CalendarViewWidget;
use crate::winui::backend::Handle;

pub fn mount(widget: &CalendarViewWidget, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::CalendarView(cv) = handle else {
        return Ok(false);
    };

    if let Some(v) = widget.is_today_highlighted {
        cv.put_IsTodayHighlighted(v)?;
    }
    if let Some(v) = widget.is_group_label_visible {
        cv.put_IsGroupLabelVisible(v)?;
    }

    Ok(true)
}

pub fn diff(
    old: &CalendarViewWidget,
    new: &CalendarViewWidget,
    handle: &Handle,
) -> windows_core::Result<bool> {
    let Handle::CalendarView(cv) = handle else {
        return Ok(false);
    };

    if old.is_today_highlighted != new.is_today_highlighted {
        if let Some(v) = new.is_today_highlighted {
            cv.put_IsTodayHighlighted(v)?;
        }
    }
    if old.is_group_label_visible != new.is_group_label_visible {
        if let Some(v) = new.is_group_label_visible {
            cv.put_IsGroupLabelVisible(v)?;
        }
    }

    Ok(true)
}
