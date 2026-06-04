//! Typed handler for the `ScrollView` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::widgets::{ScrollViewScrollBarVisibility, ScrollViewWidget};
use crate::winui::backend::Handle;

pub fn mount(
    widget: &ScrollViewWidget,
    handle: &Handle,
    _ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::ScrollView(sv) = handle else {
        return Ok(());
    };

    set_h_vis(sv, widget.horizontal_scroll_bar_visibility)?;
    set_v_vis(sv, widget.vertical_scroll_bar_visibility)?;
    Ok(())
}

pub fn diff(
    old: &ScrollViewWidget,
    new: &ScrollViewWidget,
    handle: &Handle,
    _ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::ScrollView(sv) = handle else {
        return Ok(());
    };

    if old.horizontal_scroll_bar_visibility != new.horizontal_scroll_bar_visibility {
        set_h_vis(sv, new.horizontal_scroll_bar_visibility)?;
    }
    if old.vertical_scroll_bar_visibility != new.vertical_scroll_bar_visibility {
        set_v_vis(sv, new.vertical_scroll_bar_visibility)?;
    }

    Ok(())
}

fn map_vis(v: ScrollViewScrollBarVisibility) -> Xaml::ScrollingScrollBarVisibility {
    match v {
        ScrollViewScrollBarVisibility::Auto => Xaml::ScrollingScrollBarVisibility::Auto,
        ScrollViewScrollBarVisibility::Visible => Xaml::ScrollingScrollBarVisibility::Visible,
        ScrollViewScrollBarVisibility::Hidden => Xaml::ScrollingScrollBarVisibility::Hidden,
    }
}

fn set_h_vis(
    sv: &Xaml::ScrollView,
    vis: ScrollViewScrollBarVisibility,
) -> windows_core::Result<()> {
    sv.put_HorizontalScrollBarVisibility(map_vis(vis))
}

fn set_v_vis(
    sv: &Xaml::ScrollView,
    vis: ScrollViewScrollBarVisibility,
) -> windows_core::Result<()> {
    sv.put_VerticalScrollBarVisibility(map_vis(vis))
}
