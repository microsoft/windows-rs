use super::*;

pub(super) fn info_badge_value(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<i32> {
    let Handle::InfoBadge(value) = &runtime.node(id)?.handle else {
        panic!("native node is not an InfoBadge");
    };
    value.Value()
}

pub(super) fn info_bar_properties(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(String, String, i32, bool, bool)> {
    let Handle::InfoBar { value, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not an InfoBar");
    };
    Ok((
        value.Title()?,
        value.Message()?,
        value.Severity()?.0,
        value.IsOpen()?,
        value.IsClosable()?,
    ))
}

pub(super) fn request_info_bar_close(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<bool> {
    let Handle::InfoBar { value, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not an InfoBar");
    };
    value.SetIsOpen(false)?;
    value.IsOpen()
}

pub(super) fn person_picture_properties(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(String, String)> {
    let Handle::PersonPicture(value) = &runtime.node(id)?.handle else {
        panic!("native node is not a PersonPicture");
    };
    Ok((value.DisplayName()?, value.Initials()?))
}
