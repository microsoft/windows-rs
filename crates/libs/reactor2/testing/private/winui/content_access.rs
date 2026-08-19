use super::*;

pub(super) fn viewbox_stretch(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<Stretch> {
    let Handle::Viewbox(value) = &runtime.node(id)?.handle else {
        panic!("native node is not a Viewbox");
    };
    match value.Stretch()? {
        bindings::Stretch::None => Ok(Stretch::None),
        bindings::Stretch::Fill => Ok(Stretch::Fill),
        bindings::Stretch::Uniform => Ok(Stretch::Uniform),
        bindings::Stretch::UniformToFill => Ok(Stretch::UniformToFill),
        _ => panic!("Viewbox returned an unknown stretch"),
    }
}

pub(super) fn navigate_uri(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<Option<String>> {
    let Handle::HyperlinkButton { value, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a HyperlinkButton");
    };
    match value.NavigateUri() {
        Ok(uri) => uri.AbsoluteUri().map(Some),
        Err(error) if error.code() == windows_core::HRESULT(0) => Ok(None),
        Err(error) => Err(error),
    }
}

pub(super) fn repeat_timing(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<(i32, i32)> {
    let Handle::RepeatButton { value, .. } = &runtime.node(id)?.handle else {
        panic!("native node is not a RepeatButton");
    };
    Ok((value.Delay()?, value.Interval()?))
}

pub(super) fn tooltip_attached(
    runtime: &WinUiRuntime,
    owner: NodeId,
    tooltip: NodeId,
) -> WindowsResult<bool> {
    let owner = runtime.node(owner)?.handle.ui_element()?;
    let actual = bindings::ToolTipService::GetToolTip(&owner)?;
    let expected = runtime
        .node(tooltip)?
        .handle
        .dependency_object()?
        .cast::<windows_core::IInspectable>()?;
    Ok(actual == expected)
}

pub(super) fn tooltip_empty(runtime: &WinUiRuntime, owner: NodeId) -> WindowsResult<bool> {
    let owner = runtime.node(owner)?.handle.ui_element()?;
    match bindings::ToolTipService::GetToolTip(&owner) {
        Ok(_) => Ok(false),
        Err(error) if error.code() == windows_core::HRESULT(0) => Ok(true),
        Err(error) => Err(error),
    }
}

pub(super) fn text(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<String> {
    let Handle::TextBlock(value) = &runtime.node(id)?.handle else {
        panic!("native node is not a TextBlock");
    };
    value.Text()
}
