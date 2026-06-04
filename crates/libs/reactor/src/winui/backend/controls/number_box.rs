//! Typed handler for the `NumberBox` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::NumberBox;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;

pub fn mount(w: &NumberBox, handle: &Handle, ctx: &mut EventCtx) -> windows_core::Result<()> {
    let n = handle.cast_inner::<Xaml::INumberBox>()?;
    n.put_Value(w.value)?;
    if let Some(min) = w.minimum {
        n.put_Minimum(min)?;
    }
    if let Some(max) = w.maximum {
        n.put_Maximum(max)?;
    }
    if let Some(s) = &w.header {
        n.put_Header(&string_as_textblock(s)?)?;
    }
    if !w.is_enabled {
        handle
            .cast_inner::<Xaml::IControl>()?
            .put_IsEnabled(false)?;
    }
    ctx.mount_event(
        &w.on_value_changed,
        Event::ValueChanged,
        EventHandler::ValueChanged,
    );
    Ok(())
}

pub fn diff(
    old: &NumberBox,
    new: &NumberBox,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let n = handle.cast_inner::<Xaml::INumberBox>()?;
    if new.value != old.value {
        n.put_Value(new.value)?;
    }
    if new.minimum != old.minimum {
        match new.minimum {
            Some(v) => n.put_Minimum(v)?,
            None => n.put_Minimum(f64::MIN)?,
        }
    }
    if new.maximum != old.maximum {
        match new.maximum {
            Some(v) => n.put_Maximum(v)?,
            None => n.put_Maximum(f64::MAX)?,
        }
    }
    if new.header != old.header {
        match &new.header {
            Some(s) => n.put_Header(&string_as_textblock(s)?)?,
            None => n.put_Header(None)?,
        }
    }
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
        &old.on_value_changed,
        &new.on_value_changed,
        Event::ValueChanged,
        EventHandler::ValueChanged,
    );
    Ok(())
}
