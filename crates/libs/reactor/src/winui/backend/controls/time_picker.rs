//! Typed handler for the `TimePicker` widget.

use crate::core::widgets::TimePickerWidget;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;

pub fn mount(widget: &TimePickerWidget, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::TimePicker(tp) = handle else {
        return Ok(false);
    };

    if let Some(s) = &widget.header {
        tp.put_Header(&string_as_textblock(s)?)?;
    }
    if let Some(s) = &widget.clock_identifier {
        tp.put_ClockIdentifier(s.as_str())?;
    }
    if let Some(v) = widget.minute_increment {
        tp.put_MinuteIncrement(v)?;
    }

    Ok(true)
}

pub fn diff(
    old: &TimePickerWidget,
    new: &TimePickerWidget,
    handle: &Handle,
) -> windows_core::Result<bool> {
    let Handle::TimePicker(tp) = handle else {
        return Ok(false);
    };

    if old.header != new.header {
        match &new.header {
            Some(s) => tp.put_Header(&string_as_textblock(s)?)?,
            None => tp.put_Header(None)?,
        }
    }
    if old.clock_identifier != new.clock_identifier
        && let Some(s) = &new.clock_identifier
    {
        tp.put_ClockIdentifier(s.as_str())?;
    }
    if old.minute_increment != new.minute_increment
        && let Some(v) = new.minute_increment
    {
        tp.put_MinuteIncrement(v)?;
    }

    Ok(true)
}
