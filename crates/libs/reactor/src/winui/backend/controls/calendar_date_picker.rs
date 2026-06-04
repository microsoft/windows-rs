//! Typed handler for the `CalendarDatePicker` widget.

use super::EventCtx;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::CalendarDatePickerWidget;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;

pub fn mount(
    widget: &CalendarDatePickerWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::CalendarDatePicker(cdp) = handle else {
        return Ok(());
    };

    if let Some(s) = &widget.header {
        cdp.put_Header(&string_as_textblock(s)?)?;
    }
    if let Some(s) = &widget.placeholder_text {
        cdp.put_PlaceholderText(s.as_str())?;
    }
    if let Some(v) = widget.is_today_highlighted {
        cdp.put_IsTodayHighlighted(v)?;
    }
    if let Some(v) = widget.is_calendar_open {
        cdp.put_IsCalendarOpen(v)?;
    }

    ctx.mount_event(&widget.on_changed, Event::CalendarDateSelected, |cb| {
        EventHandler::DateTimeChanged(crate::core::Callback::new(move |dt| cb.invoke(Some(dt))))
    });
    Ok(())
}

pub fn diff(
    old: &CalendarDatePickerWidget,
    new: &CalendarDatePickerWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::CalendarDatePicker(cdp) = handle else {
        return Ok(());
    };

    if old.header != new.header {
        match &new.header {
            Some(s) => cdp.put_Header(&string_as_textblock(s)?)?,
            None => cdp.put_Header(None)?,
        }
    }
    if old.placeholder_text != new.placeholder_text {
        match &new.placeholder_text {
            Some(s) => cdp.put_PlaceholderText(s.as_str())?,
            None => cdp.put_PlaceholderText("")?,
        }
    }
    if old.is_today_highlighted != new.is_today_highlighted
        && let Some(v) = new.is_today_highlighted
    {
        cdp.put_IsTodayHighlighted(v)?;
    }
    if old.is_calendar_open != new.is_calendar_open
        && let Some(v) = new.is_calendar_open
    {
        cdp.put_IsCalendarOpen(v)?;
    }

    ctx.diff_event(
        &old.on_changed,
        &new.on_changed,
        Event::CalendarDateSelected,
        |cb| {
            EventHandler::DateTimeChanged(crate::core::Callback::new(move |dt| cb.invoke(Some(dt))))
        },
    );
    Ok(())
}
