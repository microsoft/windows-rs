use super::*;

pub(super) fn tooltip_placement(
    runtime: &WinUiRuntime,
    owner: NodeId,
) -> WindowsResult<TooltipPlacement> {
    let owner = runtime.node(owner)?.handle.ui_element()?;
    match bindings::ToolTipService::GetPlacement(&owner)? {
        bindings::PlacementMode::Top => Ok(TooltipPlacement::Top),
        bindings::PlacementMode::Bottom => Ok(TooltipPlacement::Bottom),
        bindings::PlacementMode::Left => Ok(TooltipPlacement::Left),
        bindings::PlacementMode::Right => Ok(TooltipPlacement::Right),
        bindings::PlacementMode::Mouse => Ok(TooltipPlacement::Mouse),
        _ => panic!("ToolTipService returned an unknown placement"),
    }
}
