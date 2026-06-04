//! Typed handler for the `ToggleButton` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::ToggleButtonWidget;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;
use windows_core::Interface as _;

pub fn mount(
    widget: &ToggleButtonWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::ToggleButton(tb) = handle else {
        return Ok(());
    };

    tb.cast::<Xaml::IToggleButton>()?
        .put_IsChecked(Some(widget.is_checked))?;

    let label_tb = string_as_textblock(&widget.label)?;
    tb.cast::<Xaml::IContentControl>()?.put_Content(&label_tb)?;

    ctx.mount_event(
        &widget.on_changed,
        Event::CheckedChanged,
        EventHandler::CheckedChanged,
    );
    Ok(())
}

pub fn diff(
    old: &ToggleButtonWidget,
    new: &ToggleButtonWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::ToggleButton(tb) = handle else {
        return Ok(());
    };

    if old.is_checked != new.is_checked {
        tb.cast::<Xaml::IToggleButton>()?
            .put_IsChecked(Some(new.is_checked))?;
    }
    if old.label != new.label {
        let label_tb = string_as_textblock(&new.label)?;
        tb.cast::<Xaml::IContentControl>()?.put_Content(&label_tb)?;
    }

    ctx.diff_event(
        &old.on_changed,
        &new.on_changed,
        Event::CheckedChanged,
        EventHandler::CheckedChanged,
    );
    Ok(())
}
