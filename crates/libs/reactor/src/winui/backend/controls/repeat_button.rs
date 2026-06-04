//! Typed handler for the `RepeatButton` widget.

use crate::bindings as Xaml;
use crate::core::widgets::RepeatButton;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;
use windows_core::Interface as _;

pub fn mount(widget: &RepeatButton, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::RepeatButton(b) = handle else {
        return Ok(false);
    };

    let tb = string_as_textblock(&widget.content)?;
    b.cast::<Xaml::IContentControl>()?.put_Content(&tb)?;

    if let Some(ms) = widget.delay {
        b.put_Delay(ms)?;
    }
    if let Some(ms) = widget.interval {
        b.put_Interval(ms)?;
    }

    Ok(true)
}

pub fn diff(old: &RepeatButton, new: &RepeatButton, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::RepeatButton(b) = handle else {
        return Ok(false);
    };

    if old.content != new.content {
        let tb = string_as_textblock(&new.content)?;
        b.cast::<Xaml::IContentControl>()?.put_Content(&tb)?;
    }
    if old.delay != new.delay {
        if let Some(ms) = new.delay {
            b.put_Delay(ms)?;
        }
    }
    if old.interval != new.interval {
        if let Some(ms) = new.interval {
            b.put_Interval(ms)?;
        }
    }

    Ok(true)
}
