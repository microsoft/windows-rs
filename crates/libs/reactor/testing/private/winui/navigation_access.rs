use super::*;

pub(super) fn properties(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(bool, bool, bool, f64, String, i32, usize, Option<u64>)> {
    let Handle::NavigationView(state) = &runtime.node(id)?.handle else {
        panic!("native node is not a NavigationView");
    };
    let navigation2: bindings::INavigationView2 = state.value.cast()?;
    let selected = state.value.SelectedItem()?;
    let selected_key = state.items.borrow().iter().find_map(|(key, item)| {
        item.cast::<windows_core::IInspectable>()
            .ok()
            .filter(|candidate| candidate == &selected)
            .map(|_| *key)
    });
    Ok((
        state.value.IsPaneOpen()?,
        state.value.IsSettingsVisible()?,
        state.value.IsPaneToggleButtonVisible()?,
        state.value.OpenPaneLength()?,
        navigation2.PaneTitle()?,
        navigation2.PaneDisplayMode()?.0,
        state.value.MenuItems()?.Size()? as usize,
        selected_key,
    ))
}

pub(super) fn select(runtime: &WinUiRuntime, id: NodeId, key: u64) -> WindowsResult<()> {
    let Handle::NavigationView(state) = &runtime.node(id)?.handle else {
        panic!("native node is not a NavigationView");
    };
    let item = state
        .items
        .borrow()
        .iter()
        .find(|(candidate, _)| *candidate == key)
        .unwrap()
        .1
        .cast::<windows_core::IInspectable>()?;
    state.value.SetSelectedItem(&item)
}
