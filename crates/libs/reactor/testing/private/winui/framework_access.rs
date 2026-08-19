use super::*;

pub(super) fn width(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<f64> {
    runtime
        .node(id)?
        .handle
        .framework_element()
        .and_then(|value| value.Width())
}

pub(super) fn height(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<f64> {
    runtime
        .node(id)?
        .handle
        .framework_element()
        .and_then(|value| value.Height())
}

pub(super) fn min_width(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<f64> {
    runtime
        .node(id)?
        .handle
        .framework_element()
        .and_then(|value| value.MinWidth())
}

pub(super) fn max_width(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<f64> {
    runtime
        .node(id)?
        .handle
        .framework_element()
        .and_then(|value| value.MaxWidth())
}

pub(super) fn min_height(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<f64> {
    runtime
        .node(id)?
        .handle
        .framework_element()
        .and_then(|value| value.MinHeight())
}

pub(super) fn max_height(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<f64> {
    runtime
        .node(id)?
        .handle
        .framework_element()
        .and_then(|value| value.MaxHeight())
}

pub(super) fn margin(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<Thickness> {
    runtime
        .node(id)?
        .handle
        .framework_element()
        .and_then(|value| value.Margin())
        .map(|value| Thickness {
            left: value.left,
            top: value.top,
            right: value.right,
            bottom: value.bottom,
        })
}

pub(super) fn alignment(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(HorizontalAlignment, VerticalAlignment)> {
    let element = runtime.node(id)?.handle.framework_element()?;
    Ok((
        public_horizontal_alignment(element.HorizontalAlignment()?),
        public_vertical_alignment(element.VerticalAlignment()?),
    ))
}

pub(super) fn visibility(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<Visibility> {
    runtime
        .node(id)?
        .handle
        .ui_element()
        .and_then(|value| value.Visibility())
        .map(public_visibility)
}

pub(super) fn opacity(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<f64> {
    runtime
        .node(id)?
        .handle
        .ui_element()
        .and_then(|value| value.Opacity())
}

pub(super) fn enabled(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<bool> {
    runtime.node(id)?.handle.control()?.IsEnabled()
}

pub(super) fn padding(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<Thickness> {
    let value = match &runtime.node(id)?.handle {
        Handle::StackPanel(control) => control.Padding(),
        Handle::TextBlock(control) => control.Padding(),
        _ => panic!("node does not support padding"),
    }?;
    Ok(Thickness {
        left: value.left,
        top: value.top,
        right: value.right,
        bottom: value.bottom,
    })
}

pub(super) fn stack_layout(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(Orientation, f64)> {
    let Handle::StackPanel(control) = &runtime.node(id)?.handle else {
        panic!("native node is not a StackPanel");
    };
    let orientation = match control.Orientation()? {
        bindings::Orientation::Horizontal => Orientation::Horizontal,
        bindings::Orientation::Vertical => Orientation::Vertical,
        _ => panic!("invalid StackPanel orientation"),
    };
    Ok((orientation, control.Spacing()?))
}

pub(super) fn grid_placement(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(i32, i32, i32, i32)> {
    let element = runtime.node(id)?.handle.framework_element()?;
    Ok((
        bindings::Grid::GetRow(&element)?,
        bindings::Grid::GetColumn(&element)?,
        bindings::Grid::GetRowSpan(&element)?,
        bindings::Grid::GetColumnSpan(&element)?,
    ))
}

pub(super) fn grid_definitions(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(Vec<GridLength>, Vec<GridLength>)> {
    let Handle::Grid(control) = &runtime.node(id)?.handle else {
        panic!("Grid definition target is not a Grid");
    };
    let columns = control.ColumnDefinitions()?;
    let rows = control.RowDefinitions()?;
    let mut column_values = Vec::with_capacity(columns.Size()? as usize);
    for index in 0..columns.Size()? {
        let definition = columns.GetAt(index)?;
        column_values.push(reactor_grid_length(definition_grid_length(&definition)?));
    }
    let mut row_values = Vec::with_capacity(rows.Size()? as usize);
    for index in 0..rows.Size()? {
        let definition = rows.GetAt(index)?;
        row_values.push(reactor_grid_length(definition_grid_length(&definition)?));
    }
    Ok((column_values, row_values))
}

pub(super) fn canvas_placement(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(f64, f64, i32)> {
    let element = runtime.node(id)?.handle.framework_element()?;
    Ok((
        bindings::Canvas::GetLeft(&element)?,
        bindings::Canvas::GetTop(&element)?,
        bindings::Canvas::GetZIndex(&element)?,
    ))
}

pub(super) fn relative_placement(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(bool, bool, bool, bool, bool, bool)> {
    let element = runtime.node(id)?.handle.framework_element()?;
    Ok((
        bindings::RelativePanel::GetAlignLeftWithPanel(&element)?,
        bindings::RelativePanel::GetAlignRightWithPanel(&element)?,
        bindings::RelativePanel::GetAlignTopWithPanel(&element)?,
        bindings::RelativePanel::GetAlignBottomWithPanel(&element)?,
        bindings::RelativePanel::GetAlignHorizontalCenterWithPanel(&element)?,
        bindings::RelativePanel::GetAlignVerticalCenterWithPanel(&element)?,
    ))
}
