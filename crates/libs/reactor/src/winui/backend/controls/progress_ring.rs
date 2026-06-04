//! Typed handler for the `ProgressRing` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::widgets::ProgressRing;
use crate::winui::backend::Handle;
use windows_core::Interface as _;

pub fn mount(w: &ProgressRing, handle: &Handle, _ctx: &mut EventCtx) -> windows_core::Result<()> {
    let p = handle.cast_inner::<Xaml::IProgressRing>()?;
    let rb = p.cast::<Xaml::IRangeBase>()?;
    rb.put_Minimum(w.minimum)?;
    rb.put_Maximum(w.maximum)?;
    rb.put_Value(w.value)?;
    if w.is_indeterminate {
        p.put_IsIndeterminate(true)?;
    }
    if w.is_active {
        p.put_IsActive(true)?;
    }
    Ok(())
}

pub fn diff(
    old: &ProgressRing,
    new: &ProgressRing,
    handle: &Handle,
    _ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let p = handle.cast_inner::<Xaml::IProgressRing>()?;
    let rb = p.cast::<Xaml::IRangeBase>()?;
    super::diff_val!(old, new, minimum, rb.put_Minimum(new.minimum));
    super::diff_val!(old, new, maximum, rb.put_Maximum(new.maximum));
    super::diff_val!(old, new, value, rb.put_Value(new.value));
    super::diff_val!(
        old,
        new,
        is_indeterminate,
        p.put_IsIndeterminate(new.is_indeterminate)
    );
    super::diff_val!(old, new, is_active, p.put_IsActive(new.is_active));
    Ok(())
}
