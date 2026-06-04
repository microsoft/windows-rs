//! Typed handler for the `Border` widget.

use crate::bindings as Xaml;
use crate::core::element::BrushBinding;
use crate::core::element::Thickness;
use crate::core::widgets::Border;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::brush_of;
use crate::winui::backend::convert::to_xaml_thickness;

pub fn mount(w: &Border, handle: &Handle) -> windows_core::Result<()> {
    let b = handle.cast_inner::<Xaml::IBorder>()?;
    if let Some(cr) = w.corner_radius {
        b.put_CornerRadius(Xaml::CornerRadius {
            TopLeft: cr,
            TopRight: cr,
            BottomRight: cr,
            BottomLeft: cr,
        })?;
    }
    if let Some(BrushBinding::Direct(ref br)) = w.border_brush {
        b.put_BorderBrush(&brush_of(br)?)?;
    }
    if let Some(t) = w.border_thickness {
        b.put_BorderThickness(to_xaml_thickness(t))?;
    }
    Ok(())
}

pub fn diff(old: &Border, new: &Border, handle: &Handle) -> windows_core::Result<()> {
    let b = handle.cast_inner::<Xaml::IBorder>()?;
    if new.corner_radius != old.corner_radius {
        if let Some(cr) = new.corner_radius {
            b.put_CornerRadius(Xaml::CornerRadius {
                TopLeft: cr,
                TopRight: cr,
                BottomRight: cr,
                BottomLeft: cr,
            })?;
        } else {
            b.put_CornerRadius(Xaml::CornerRadius::default())?;
        }
    }
    let old_brush = match &old.border_brush {
        Some(BrushBinding::Direct(br)) => Some(br),
        _ => None,
    };
    let new_brush = match &new.border_brush {
        Some(BrushBinding::Direct(br)) => Some(br),
        _ => None,
    };
    if new_brush != old_brush {
        if let Some(br) = new_brush {
            b.put_BorderBrush(&brush_of(br)?)?;
        } else {
            b.put_BorderBrush(None)?;
        }
    }
    if new.border_thickness != old.border_thickness {
        if let Some(t) = new.border_thickness {
            b.put_BorderThickness(to_xaml_thickness(t))?;
        } else {
            b.put_BorderThickness(to_xaml_thickness(Thickness::default()))?;
        }
    }
    Ok(())
}
