//! Typed handler for the `ToggleSwitch` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::ToggleSwitch;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;

pub fn mount(w: &ToggleSwitch, handle: &Handle, ctx: &mut EventCtx) -> windows_core::Result<()> {
    let ts = handle.cast_inner::<Xaml::IToggleSwitch>()?;
    ts.put_IsOn(w.is_on)?;
    if let Some(s) = &w.on_content {
        ts.put_OnContent(&string_as_textblock(s)?)?;
    }
    if let Some(s) = &w.off_content {
        ts.put_OffContent(&string_as_textblock(s)?)?;
    }
    if let Some(s) = &w.header {
        ts.put_Header(&string_as_textblock(s)?)?;
    }
    if !w.is_enabled {
        handle
            .cast_inner::<Xaml::IControl>()?
            .put_IsEnabled(false)?;
    }
    ctx.mount_event(&w.on_changed, Event::Toggled, EventHandler::CheckedChanged);
    Ok(())
}

pub fn diff(
    old: &ToggleSwitch,
    new: &ToggleSwitch,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let ts = handle.cast_inner::<Xaml::IToggleSwitch>()?;
    super::diff_val!(old, new, is_on, ts.put_IsOn(new.is_on));
    super::diff_opt!(
        old,
        new,
        on_content,
        |s| ts.put_OnContent(&string_as_textblock(s)?),
        ts.put_OnContent(None)
    );
    super::diff_opt!(
        old,
        new,
        off_content,
        |s| ts.put_OffContent(&string_as_textblock(s)?),
        ts.put_OffContent(None)
    );
    super::diff_opt!(
        old,
        new,
        header,
        |s| ts.put_Header(&string_as_textblock(s)?),
        ts.put_Header(None)
    );
    if new.is_enabled != old.is_enabled {
        if new.is_enabled {
            handle
                .cast_inner::<Xaml::IDependencyObject>()?
                .ClearValue(&Xaml::Control::get_IsEnabledProperty()?)?;
        } else {
            handle
                .cast_inner::<Xaml::IControl>()?
                .put_IsEnabled(false)?;
        }
    }
    ctx.diff_event(
        &old.on_changed,
        &new.on_changed,
        Event::Toggled,
        EventHandler::CheckedChanged,
    );
    Ok(())
}
