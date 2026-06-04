//! Typed handler for the `DatePicker` widget.

use crate::core::widgets::DatePickerWidget;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;

pub fn mount(widget: &DatePickerWidget, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::DatePicker(dp) = handle else {
        return Ok(false);
    };

    if let Some(s) = &widget.header {
        dp.put_Header(&string_as_textblock(s)?)?;
    }
    if let Some(v) = widget.day_visible {
        dp.put_DayVisible(v)?;
    }
    if let Some(v) = widget.month_visible {
        dp.put_MonthVisible(v)?;
    }
    if let Some(v) = widget.year_visible {
        dp.put_YearVisible(v)?;
    }

    Ok(true)
}

pub fn diff(
    old: &DatePickerWidget,
    new: &DatePickerWidget,
    handle: &Handle,
) -> windows_core::Result<bool> {
    let Handle::DatePicker(dp) = handle else {
        return Ok(false);
    };

    if old.header != new.header {
        match &new.header {
            Some(s) => dp.put_Header(&string_as_textblock(s)?)?,
            None => dp.put_Header(None)?,
        }
    }
    if old.day_visible != new.day_visible
        && let Some(v) = new.day_visible
    {
        dp.put_DayVisible(v)?;
    }
    if old.month_visible != new.month_visible
        && let Some(v) = new.month_visible
    {
        dp.put_MonthVisible(v)?;
    }
    if old.year_visible != new.year_visible
        && let Some(v) = new.year_visible
    {
        dp.put_YearVisible(v)?;
    }

    Ok(true)
}
