//! Typed handler for the `CheckBox` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::CheckBox;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;

pub fn mount(w: &CheckBox, handle: &Handle, ctx: &mut EventCtx) -> windows_core::Result<()> {
    let tb = handle.cast_inner::<Xaml::IToggleButton>()?;
    tb.put_IsChecked(Some(w.is_checked))?;
    if let Some(label) = &w.label {
        let txt = string_as_textblock(label)?;
        handle
            .cast_inner::<Xaml::IContentControl>()?
            .put_Content(&txt)?;
    }
    if !w.is_enabled {
        handle
            .cast_inner::<Xaml::IControl>()?
            .put_IsEnabled(false)?;
    }
    ctx.mount_event(
        &w.on_changed,
        Event::CheckedChanged,
        EventHandler::CheckedChanged,
    );
    Ok(())
}

pub fn diff(
    old: &CheckBox,
    new: &CheckBox,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    super::diff_val!(
        old,
        new,
        is_checked,
        handle
            .cast_inner::<Xaml::IToggleButton>()?
            .put_IsChecked(Some(new.is_checked))
    );
    let cc = handle.cast_inner::<Xaml::IContentControl>()?;
    super::diff_opt!(
        old,
        new,
        label,
        |s| cc.put_Content(&string_as_textblock(s)?),
        cc.put_Content(None)
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
        Event::CheckedChanged,
        EventHandler::CheckedChanged,
    );
    Ok(())
}
