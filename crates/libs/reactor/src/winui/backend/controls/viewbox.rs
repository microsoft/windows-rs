//! Typed handler for the `Viewbox` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::widgets::{ImageStretch, Viewbox};
use crate::winui::backend::Handle;

pub fn mount(widget: &Viewbox, handle: &Handle, _ctx: &mut EventCtx) -> windows_core::Result<()> {
    let Handle::Viewbox(vb) = handle else {
        return Ok(());
    };

    set_stretch(vb, widget.stretch)?;
    Ok(())
}

pub fn diff(
    old: &Viewbox,
    new: &Viewbox,
    handle: &Handle,
    _ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::Viewbox(vb) = handle else {
        return Ok(());
    };

    if old.stretch != new.stretch {
        set_stretch(vb, new.stretch)?;
    }

    Ok(())
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
