//! Typed handler for the `RepeatButton` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::RepeatButton;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;
use windows_core::Interface as _;

pub fn mount(
    widget: &RepeatButton,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::RepeatButton(b) = handle else {
        return Ok(());
    };

    let tb = string_as_textblock(&widget.content)?;
    b.cast::<Xaml::IContentControl>()?.put_Content(&tb)?;

    if let Some(ms) = widget.delay {
        b.put_Delay(ms)?;
    }
    if let Some(ms) = widget.interval {
        b.put_Interval(ms)?;
    }

    ctx.mount_event(&widget.on_click, Event::Click, EventHandler::Click);
    Ok(())
}

pub fn diff(
    old: &RepeatButton,
    new: &RepeatButton,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::RepeatButton(b) = handle else {
        return Ok(());
    };

    if old.content != new.content {
        let tb = string_as_textblock(&new.content)?;
        b.cast::<Xaml::IContentControl>()?.put_Content(&tb)?;
    }
    if old.delay != new.delay
        && let Some(ms) = new.delay
    {
        b.put_Delay(ms)?;
    }
    if old.interval != new.interval
        && let Some(ms) = new.interval
    {
        b.put_Interval(ms)?;
    }

    ctx.diff_event(
        &old.on_click,
        &new.on_click,
        Event::Click,
        EventHandler::Click,
    );
    Ok(())
}
