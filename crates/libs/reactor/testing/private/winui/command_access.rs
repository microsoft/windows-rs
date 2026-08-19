use super::*;

pub(crate) fn command_bar(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<(u32, u32)> {
    let Handle::CommandBar { state, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a CommandBar");
    };
    Ok((state.primary.Size()?, state.secondary.Size()?))
}

pub(crate) fn app_bar_button(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<(bool, bool)> {
    let Handle::AppBarButton(state) = &runtime.node(id)?.handle else {
        panic!("native node is not an AppBarButton");
    };
    Ok((state.control.IsEnabled()?, state.icon.is_some()))
}

pub(crate) fn app_bar_toggle_button(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(bool, bool, bool)> {
    let Handle::AppBarToggleButton(state) = &runtime.node(id)?.handle else {
        panic!("native node is not an AppBarToggleButton");
    };
    Ok((
        state.control.IsEnabled()?,
        state.toggle.IsChecked()?,
        state.icon.is_some(),
    ))
}
