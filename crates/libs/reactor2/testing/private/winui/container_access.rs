use super::*;

pub(in crate::winui) fn scroll_viewer(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(ScrollBarVisibility, ScrollBarVisibility)> {
    let Handle::ScrollViewer { value, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a ScrollViewer");
    };
    let convert = |value| match value {
        bindings::ScrollBarVisibility::Disabled => ScrollBarVisibility::Disabled,
        bindings::ScrollBarVisibility::Auto => ScrollBarVisibility::Auto,
        bindings::ScrollBarVisibility::Hidden => ScrollBarVisibility::Hidden,
        bindings::ScrollBarVisibility::Visible => ScrollBarVisibility::Visible,
        _ => panic!("ScrollViewer returned an unknown visibility"),
    };
    Ok((
        convert(value.HorizontalScrollBarVisibility()?),
        convert(value.VerticalScrollBarVisibility()?),
    ))
}

pub(in crate::winui) fn scroll_view(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(
    ScrollViewBarVisibility,
    ScrollViewBarVisibility,
    ScrollOrientation,
)> {
    let Handle::ScrollView { value, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a ScrollView");
    };
    let visibility = |value| match value {
        bindings::ScrollingScrollBarVisibility::Auto => ScrollViewBarVisibility::Auto,
        bindings::ScrollingScrollBarVisibility::Visible => ScrollViewBarVisibility::Visible,
        bindings::ScrollingScrollBarVisibility::Hidden => ScrollViewBarVisibility::Hidden,
        _ => panic!("ScrollView returned an unknown visibility"),
    };
    let orientation = match value.ContentOrientation()? {
        bindings::ScrollingContentOrientation::Vertical => ScrollOrientation::Vertical,
        bindings::ScrollingContentOrientation::Horizontal => ScrollOrientation::Horizontal,
        bindings::ScrollingContentOrientation::None => ScrollOrientation::None,
        bindings::ScrollingContentOrientation::Both => ScrollOrientation::Both,
        _ => panic!("ScrollView returned an unknown orientation"),
    };
    Ok((
        visibility(value.HorizontalScrollBarVisibility()?),
        visibility(value.VerticalScrollBarVisibility()?),
        orientation,
    ))
}

pub(in crate::winui) fn split_view(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(SplitViewDisplayMode, bool, f64, f64)> {
    let Handle::SplitView { value, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a SplitView");
    };
    let display_mode = match value.DisplayMode()? {
        bindings::SplitViewDisplayMode::Overlay => SplitViewDisplayMode::Overlay,
        bindings::SplitViewDisplayMode::Inline => SplitViewDisplayMode::Inline,
        bindings::SplitViewDisplayMode::CompactOverlay => SplitViewDisplayMode::CompactOverlay,
        bindings::SplitViewDisplayMode::CompactInline => SplitViewDisplayMode::CompactInline,
        _ => panic!("SplitView returned an unknown display mode"),
    };
    Ok((
        display_mode,
        value.IsPaneOpen()?,
        value.OpenPaneLength()?,
        value.CompactPaneLength()?,
    ))
}

pub(in crate::winui) fn expander(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<bool> {
    let Handle::Expander(state) = &runtime.node(id)?.handle else {
        panic!("native node is not an Expander");
    };
    state.value.IsExpanded()
}

pub(in crate::winui) fn set_split_view_open(
    runtime: &WinUiRuntime,
    id: NodeId,
    value: bool,
) -> WindowsResult<()> {
    let Handle::SplitView { value: control, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a SplitView");
    };
    control.SetIsPaneOpen(value)
}

pub(in crate::winui) fn set_expander(
    runtime: &WinUiRuntime,
    id: NodeId,
    value: bool,
) -> WindowsResult<()> {
    let Handle::Expander(state) = &runtime.node(id)?.handle else {
        panic!("native node is not an Expander");
    };
    state.value.SetIsExpanded(value)
}
