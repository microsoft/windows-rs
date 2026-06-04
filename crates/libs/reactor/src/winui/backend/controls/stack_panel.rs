//! Typed handler for the `StackPanel` widget.

use crate::bindings as Xaml;
use crate::core::widgets::StackPanel;
use crate::winui::backend::Handle;

pub fn mount(w: &StackPanel, handle: &Handle) -> windows_core::Result<()> {
    let sp = handle.cast_inner::<Xaml::IStackPanel>()?;
    sp.put_Orientation(if w.vertical {
        Xaml::Orientation::Vertical
    } else {
        Xaml::Orientation::Horizontal
    })?;
    if let Some(spacing) = w.spacing {
        sp.put_Spacing(spacing)?;
    }
    Ok(())
}

pub fn diff(old: &StackPanel, new: &StackPanel, handle: &Handle) -> windows_core::Result<()> {
    let sp = handle.cast_inner::<Xaml::IStackPanel>()?;
    if new.vertical != old.vertical {
        sp.put_Orientation(if new.vertical {
            Xaml::Orientation::Vertical
        } else {
            Xaml::Orientation::Horizontal
        })?;
    }
    if new.spacing != old.spacing {
        if let Some(v) = new.spacing {
            sp.put_Spacing(v)?;
        } else {
            sp.put_Spacing(0.0)?;
        }
    }
    Ok(())
}
