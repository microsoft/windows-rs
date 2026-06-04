//! Typed handler for the `ScrollView` widget.

use crate::bindings as Xaml;
use crate::core::widgets::{ScrollViewScrollBarVisibility, ScrollViewWidget};
use crate::winui::backend::Handle;

pub fn mount(widget: &ScrollViewWidget, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::ScrollView(sv) = handle else {
        return Ok(false);
    };

    set_h_vis(sv, widget.horizontal_scroll_bar_visibility)?;
    set_v_vis(sv, widget.vertical_scroll_bar_visibility)?;
    Ok(true)
}

pub fn diff(
    old: &ScrollViewWidget,
    new: &ScrollViewWidget,
    handle: &Handle,
) -> windows_core::Result<bool> {
    let Handle::ScrollView(sv) = handle else {
        return Ok(false);
    };

    if old.horizontal_scroll_bar_visibility != new.horizontal_scroll_bar_visibility {
        set_h_vis(sv, new.horizontal_scroll_bar_visibility)?;
    }
    if old.vertical_scroll_bar_visibility != new.vertical_scroll_bar_visibility {
        set_v_vis(sv, new.vertical_scroll_bar_visibility)?;
    }

    Ok(true)
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
