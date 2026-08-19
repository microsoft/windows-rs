use super::*;

pub(super) fn font_size(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<f64> {
    let node = runtime.node(id)?;
    match &node.handle {
        Handle::TextBlock(text) => text.FontSize(),
        handle => handle.control()?.FontSize(),
    }
}

pub(super) fn character_spacing(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<i32> {
    let node = runtime.node(id)?;
    match &node.handle {
        Handle::TextBlock(text) => text.CharacterSpacing(),
        handle => handle.control()?.CharacterSpacing(),
    }
}

pub(super) fn font_weight(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<u16> {
    let node = runtime.node(id)?;
    match &node.handle {
        Handle::TextBlock(text) => text.FontWeight().map(|value| value.weight),
        handle => handle.control()?.FontWeight().map(|value| value.weight),
    }
}

pub(super) fn font_style(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<i32> {
    let node = runtime.node(id)?;
    match &node.handle {
        Handle::TextBlock(text) => text.FontStyle().map(|value| value.0),
        handle => handle.control()?.FontStyle().map(|value| value.0),
    }
}

pub(super) fn font_stretch(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<i32> {
    let node = runtime.node(id)?;
    match &node.handle {
        Handle::TextBlock(text) => text.FontStretch().map(|value| value.0),
        handle => handle.control()?.FontStretch().map(|value| value.0),
    }
}

pub(super) fn font_family(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<String> {
    let node = runtime.node(id)?;
    let family = match &node.handle {
        Handle::TextBlock(text) => text.FontFamily()?,
        handle => handle.control()?.FontFamily()?,
    };
    family.Source()
}

pub(super) fn foreground(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<Color> {
    let node = runtime.node(id)?;
    let brush = match &node.handle {
        Handle::TextBlock(text) => text.Foreground()?,
        handle => handle.control()?.Foreground()?,
    }
    .cast::<bindings::SolidColorBrush>()?;
    let value = brush.Color()?;
    Ok(Color {
        a: value.a,
        r: value.r,
        g: value.g,
        b: value.b,
    })
}

pub(super) fn text_properties(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(i32, i32, bool)> {
    let text = runtime.node(id)?.handle.text_block()?;
    Ok((
        text.TextWrapping()?.0,
        text.TextTrimming()?.0,
        text.IsTextSelectionEnabled()?,
    ))
}

pub(super) fn automation(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(String, String, String, i32)> {
    let object = runtime.node(id)?.handle.dependency_object()?;
    Ok((
        bindings::AutomationProperties::GetName(&object)?,
        bindings::AutomationProperties::GetAutomationId(&object)?,
        bindings::AutomationProperties::GetHelpText(&object)?,
        bindings::AutomationProperties::GetHeadingLevel(&object)?.0,
    ))
}

pub(super) fn keyboard_accelerators(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<Vec<(i32, u32)>> {
    let element: bindings::IUIElement = runtime.node(id)?.handle.ui_element()?.cast()?;
    let values = element.KeyboardAccelerators()?;
    let mut result = Vec::with_capacity(values.Size()? as usize);
    for index in 0..values.Size()? {
        let value = values.GetAt(index)?;
        result.push((value.Key()?.0, value.Modifiers()?.0));
    }
    Ok(result)
}

pub(super) fn allow_drop(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<bool> {
    runtime.node(id)?.handle.ui_element()?.AllowDrop()
}
