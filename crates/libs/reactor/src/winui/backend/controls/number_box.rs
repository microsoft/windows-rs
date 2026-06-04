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
    super::diff_val!(old, new, value, n.put_Value(new.value));
    super::diff_opt!(
        old,
        new,
        minimum,
        |v| n.put_Minimum(*v),
        n.put_Minimum(f64::MIN)
    );
    super::diff_opt!(
        old,
        new,
        maximum,
        |v| n.put_Maximum(*v),
        n.put_Maximum(f64::MAX)
    );
    super::diff_opt!(
        old,
        new,
        header,
        |s| n.put_Header(&string_as_textblock(s)?),
        n.put_Header(None)
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
        &old.on_value_changed,
        &new.on_value_changed,
        Event::ValueChanged,
        EventHandler::ValueChanged,
    );
    Ok(())
}
