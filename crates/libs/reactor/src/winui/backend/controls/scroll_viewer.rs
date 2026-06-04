//! Typed handler for the `ScrollViewer` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::widgets::ScrollViewer;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::to_xaml_scroll_visibility;

pub fn mount(w: &ScrollViewer, handle: &Handle, _ctx: &mut EventCtx) -> windows_core::Result<()> {
    let sv = handle.cast_inner::<Xaml::IScrollViewer>()?;
    sv.put_HorizontalScrollBarVisibility(to_xaml_scroll_visibility(
        w.horizontal_scroll_bar_visibility,
    ))?;
    sv.put_VerticalScrollBarVisibility(to_xaml_scroll_visibility(
        w.vertical_scroll_bar_visibility,
    ))?;
    Ok(())
}

pub fn diff(
    old: &ScrollViewer,
    new: &ScrollViewer,
    handle: &Handle,
    _ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let sv = handle.cast_inner::<Xaml::IScrollViewer>()?;
    super::diff_val!(
        old,
        new,
        horizontal_scroll_bar_visibility,
        sv.put_HorizontalScrollBarVisibility(to_xaml_scroll_visibility(
            new.horizontal_scroll_bar_visibility,
        ))
    );
    super::diff_val!(
        old,
        new,
        vertical_scroll_bar_visibility,
        sv.put_VerticalScrollBarVisibility(to_xaml_scroll_visibility(
            new.vertical_scroll_bar_visibility,
        ))
    );
    Ok(())
}
