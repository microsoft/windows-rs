use super::*;
use crate::{GridLength, GridLengthSize};
use bindings::GridLength as NativeGridLength;

pub(super) fn set_attached(
    element: &UIElement,
    property: PropertyId,
    value: &PropertyValue,
) -> Result<(), RuntimeError> {
    let element = element.cast::<FrameworkElement>().map_err(native_error)?;
    match (property, value) {
        (PropertyId::GridRow, PropertyValue::I32(value)) => {
            bindings::Grid::SetRow(&element, *value).map_err(native_error)
        }
        (PropertyId::GridColumn, PropertyValue::I32(value)) => {
            bindings::Grid::SetColumn(&element, *value).map_err(native_error)
        }
        (PropertyId::GridRowSpan, PropertyValue::I32(value)) => {
            bindings::Grid::SetRowSpan(&element, *value).map_err(native_error)
        }
        (PropertyId::GridColumnSpan, PropertyValue::I32(value)) => {
            bindings::Grid::SetColumnSpan(&element, *value).map_err(native_error)
        }
        (PropertyId::RelativeAlignLeft, PropertyValue::Bool(value)) => {
            bindings::RelativePanel::SetAlignLeftWithPanel(&element, *value).map_err(native_error)
        }
        (PropertyId::RelativeAlignTop, PropertyValue::Bool(value)) => {
            bindings::RelativePanel::SetAlignTopWithPanel(&element, *value).map_err(native_error)
        }
        (PropertyId::RelativeAlignRight, PropertyValue::Bool(value)) => {
            bindings::RelativePanel::SetAlignRightWithPanel(&element, *value).map_err(native_error)
        }
        (PropertyId::RelativeAlignBottom, PropertyValue::Bool(value)) => {
            bindings::RelativePanel::SetAlignBottomWithPanel(&element, *value).map_err(native_error)
        }
        (PropertyId::RelativeAlignHorizontalCenter, PropertyValue::Bool(value)) => {
            bindings::RelativePanel::SetAlignHorizontalCenterWithPanel(&element, *value)
                .map_err(native_error)
        }
        (PropertyId::RelativeAlignVerticalCenter, PropertyValue::Bool(value)) => {
            bindings::RelativePanel::SetAlignVerticalCenterWithPanel(&element, *value)
                .map_err(native_error)
        }
        (PropertyId::CanvasLeft, PropertyValue::F64(value)) => {
            bindings::Canvas::SetLeft(&element, *value).map_err(native_error)
        }
        (PropertyId::CanvasTop, PropertyValue::F64(value)) => {
            bindings::Canvas::SetTop(&element, *value).map_err(native_error)
        }
        (PropertyId::AutomationName, PropertyValue::Str(value)) => {
            AutomationProperties::SetName(&element, value).map_err(native_error)
        }
        (PropertyId::AutomationId, PropertyValue::Str(value)) => {
            AutomationProperties::SetAutomationId(&element, value).map_err(native_error)
        }
        (PropertyId::AutomationHeadingLevel, PropertyValue::I32(value)) => {
            AutomationProperties::SetHeadingLevel(
                &element,
                match value {
                    1 => bindings::AutomationHeadingLevel::Level1,
                    2 => bindings::AutomationHeadingLevel::Level2,
                    3 => bindings::AutomationHeadingLevel::Level3,
                    4 => bindings::AutomationHeadingLevel::Level4,
                    5 => bindings::AutomationHeadingLevel::Level5,
                    6 => bindings::AutomationHeadingLevel::Level6,
                    7 => bindings::AutomationHeadingLevel::Level7,
                    8 => bindings::AutomationHeadingLevel::Level8,
                    9 => bindings::AutomationHeadingLevel::Level9,
                    _ => return Err(RuntimeError::UnsupportedKind),
                },
            )
            .map_err(native_error)
        }
        _ => Err(RuntimeError::UnsupportedKind),
    }
}

pub(super) fn clear_attached(
    element: &UIElement,
    property: PropertyId,
) -> Result<(), RuntimeError> {
    let element = element.cast::<FrameworkElement>().map_err(native_error)?;
    match property {
        PropertyId::AutomationName => {
            return AutomationProperties::SetName(&element, "").map_err(native_error);
        }
        PropertyId::AutomationId => {
            return AutomationProperties::SetAutomationId(&element, "").map_err(native_error);
        }
        PropertyId::AutomationHeadingLevel => {
            return AutomationProperties::SetHeadingLevel(
                &element,
                bindings::AutomationHeadingLevel::None,
            )
            .map_err(native_error);
        }
        _ => {}
    }
    let dependency_object = element.cast::<IDependencyObject>().map_err(native_error)?;
    let dependency_property = match property {
        PropertyId::GridRow => bindings::Grid::RowProperty(),
        PropertyId::GridColumn => bindings::Grid::ColumnProperty(),
        PropertyId::GridRowSpan => bindings::Grid::RowSpanProperty(),
        PropertyId::GridColumnSpan => bindings::Grid::ColumnSpanProperty(),
        PropertyId::RelativeAlignLeft => bindings::RelativePanel::AlignLeftWithPanelProperty(),
        PropertyId::RelativeAlignTop => bindings::RelativePanel::AlignTopWithPanelProperty(),
        PropertyId::RelativeAlignRight => bindings::RelativePanel::AlignRightWithPanelProperty(),
        PropertyId::RelativeAlignBottom => bindings::RelativePanel::AlignBottomWithPanelProperty(),
        PropertyId::RelativeAlignHorizontalCenter => {
            bindings::RelativePanel::AlignHorizontalCenterWithPanelProperty()
        }
        PropertyId::RelativeAlignVerticalCenter => {
            bindings::RelativePanel::AlignVerticalCenterWithPanelProperty()
        }
        PropertyId::CanvasLeft => bindings::Canvas::LeftProperty(),
        PropertyId::CanvasTop => bindings::Canvas::TopProperty(),
        _ => return Err(RuntimeError::UnsupportedKind),
    }
    .map_err(native_error)?;
    dependency_object
        .ClearValue(&dependency_property)
        .map_err(native_error)
}

pub(super) fn set_definitions(
    handle: &Handle,
    property: PropertyId,
    value: &PropertyValue,
) -> Result<(), RuntimeError> {
    let (Handle::Grid(grid), PropertyValue::GridLengths(values)) = (handle, value) else {
        return Err(RuntimeError::UnsupportedKind);
    };
    match property {
        PropertyId::GridRows => {
            let definitions = grid.RowDefinitions().map_err(native_error)?;
            if rows_match(&definitions, values)? {
                return Ok(());
            }
            definitions.Clear().map_err(native_error)?;
            for length in values.iter().copied() {
                let definition = RowDefinition::new().map_err(native_error)?;
                definition
                    .SetHeight(native_grid_length(length))
                    .map_err(native_error)?;
                if let Some(value) = length.min {
                    definition.SetMinHeight(value).map_err(native_error)?;
                }
                if let Some(value) = length.max {
                    definition.SetMaxHeight(value).map_err(native_error)?;
                }
                definitions.Append(&definition).map_err(native_error)?;
            }
            Ok(())
        }
        PropertyId::GridColumns => {
            let definitions = grid.ColumnDefinitions().map_err(native_error)?;
            if columns_match(&definitions, values)? {
                return Ok(());
            }
            definitions.Clear().map_err(native_error)?;
            for length in values.iter().copied() {
                let definition = ColumnDefinition::new().map_err(native_error)?;
                definition
                    .SetWidth(native_grid_length(length))
                    .map_err(native_error)?;
                if let Some(value) = length.min {
                    definition.SetMinWidth(value).map_err(native_error)?;
                }
                if let Some(value) = length.max {
                    definition.SetMaxWidth(value).map_err(native_error)?;
                }
                definitions.Append(&definition).map_err(native_error)?;
            }
            Ok(())
        }
        _ => Err(RuntimeError::UnsupportedKind),
    }
}

pub(super) fn clear_definitions(handle: &Handle, property: PropertyId) -> Result<(), RuntimeError> {
    let Handle::Grid(grid) = handle else {
        return Err(RuntimeError::UnsupportedKind);
    };
    match property {
        PropertyId::GridRows => grid
            .RowDefinitions()
            .map_err(native_error)?
            .Clear()
            .map_err(native_error),
        PropertyId::GridColumns => grid
            .ColumnDefinitions()
            .map_err(native_error)?
            .Clear()
            .map_err(native_error),
        _ => Err(RuntimeError::UnsupportedKind),
    }
}

fn rows_match(
    definitions: &RowDefinitionCollection,
    values: &[GridLength],
) -> Result<bool, RuntimeError> {
    if usize::try_from(definitions.Size().map_err(native_error)?).unwrap() != values.len() {
        return Ok(false);
    }
    for (index, length) in values.iter().copied().enumerate() {
        let index = u32::try_from(index).map_err(|_| RuntimeError::IndexOutOfBounds)?;
        let definition = definitions.GetAt(index).map_err(native_error)?;
        if !row_matches(&definition, length)? {
            return Ok(false);
        }
    }
    Ok(true)
}

fn columns_match(
    definitions: &ColumnDefinitionCollection,
    values: &[GridLength],
) -> Result<bool, RuntimeError> {
    if usize::try_from(definitions.Size().map_err(native_error)?).unwrap() != values.len() {
        return Ok(false);
    }
    for (index, length) in values.iter().copied().enumerate() {
        let index = u32::try_from(index).map_err(|_| RuntimeError::IndexOutOfBounds)?;
        let definition = definitions.GetAt(index).map_err(native_error)?;
        if !column_matches(&definition, length)? {
            return Ok(false);
        }
    }
    Ok(true)
}

fn grid_lengths_match(left: NativeGridLength, right: NativeGridLength) -> bool {
    left.grid_unit_type == right.grid_unit_type && f64_eq(left.value, right.value)
}

fn row_matches(definition: &RowDefinition, value: GridLength) -> Result<bool, RuntimeError> {
    let actual = definition.Height().map_err(native_error)?;
    let min = definition.MinHeight().map_err(native_error)?;
    let max = definition.MaxHeight().map_err(native_error)?;
    Ok(grid_lengths_match(actual, native_grid_length(value))
        && f64_eq(min, value.min.unwrap_or(0.0))
        && f64_eq(max, value.max.unwrap_or(f64::INFINITY)))
}

fn column_matches(definition: &ColumnDefinition, value: GridLength) -> Result<bool, RuntimeError> {
    let actual = definition.Width().map_err(native_error)?;
    let min = definition.MinWidth().map_err(native_error)?;
    let max = definition.MaxWidth().map_err(native_error)?;
    Ok(grid_lengths_match(actual, native_grid_length(value))
        && f64_eq(min, value.min.unwrap_or(0.0))
        && f64_eq(max, value.max.unwrap_or(f64::INFINITY)))
}

fn native_grid_length(length: GridLength) -> NativeGridLength {
    let (value, grid_unit_type) = match length.size {
        GridLengthSize::Auto => (0.0, GridUnitType::Auto),
        GridLengthSize::Pixel(value) => (value, GridUnitType::Pixel),
        GridLengthSize::Star(value) => (value, GridUnitType::Star),
    };
    NativeGridLength {
        value,
        grid_unit_type,
    }
}
