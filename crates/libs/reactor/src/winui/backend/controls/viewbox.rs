//! Typed handler for the `Viewbox` widget.

use crate::bindings as Xaml;
use crate::core::widgets::{ImageStretch, Viewbox};
use crate::winui::backend::Handle;

pub fn mount(widget: &Viewbox, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::Viewbox(vb) = handle else {
        return Ok(false);
    };

    set_stretch(vb, widget.stretch)?;
    Ok(true)
}

pub fn diff(old: &Viewbox, new: &Viewbox, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::Viewbox(vb) = handle else {
        return Ok(false);
    };

    if old.stretch != new.stretch {
        set_stretch(vb, new.stretch)?;
    }

    Ok(true)
}

fn set_stretch(vb: &Xaml::Viewbox, stretch: ImageStretch) -> windows_core::Result<()> {
    let mapped = match stretch {
        ImageStretch::Uniform => Xaml::Stretch::Uniform,
        ImageStretch::UniformToFill => Xaml::Stretch::UniformToFill,
        ImageStretch::Fill => Xaml::Stretch::Fill,
        ImageStretch::None => Xaml::Stretch::None,
    };
    vb.put_Stretch(mapped)
}
