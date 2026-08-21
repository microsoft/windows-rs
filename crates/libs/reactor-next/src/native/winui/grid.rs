use super::*;
use crate::GridLength as ReactorGridLength;

pub(super) fn is_attached(property: PropertyId) -> bool {
    matches!(
        property,
        PropertyId::GridRow
            | PropertyId::GridColumn
            | PropertyId::GridRowSpan
            | PropertyId::GridColumnSpan
    )
}

pub(super) fn is_definitions(property: PropertyId) -> bool {
    matches!(property, PropertyId::GridRows | PropertyId::GridColumns)
}

pub(super) fn set_attached(
    element: &UIElement,
    property: PropertyId,
    value: &PropertyValue,
) -> Result<(), RuntimeError> {
    let PropertyValue::I32(value) = value else {
        return Err(RuntimeError::UnsupportedKind);
    };
    let element = element.cast::<FrameworkElement>().map_err(native_error)?;
    match property {
        PropertyId::GridRow => bindings::Grid::SetRow(&element, *value).map_err(native_error),
        PropertyId::GridColumn => bindings::Grid::SetColumn(&element, *value).map_err(native_error),
        PropertyId::GridRowSpan => {
            bindings::Grid::SetRowSpan(&element, *value).map_err(native_error)
        }
        PropertyId::GridColumnSpan => {
            bindings::Grid::SetColumnSpan(&element, *value).map_err(native_error)
        }
        _ => Err(RuntimeError::UnsupportedKind),
    }
}

pub(super) fn clear_attached(
    element: &UIElement,
    property: PropertyId,
) -> Result<(), RuntimeError> {
    let dependency_object = element.cast::<IDependencyObject>().map_err(native_error)?;
    let dependency_property = match property {
        PropertyId::GridRow => bindings::Grid::RowProperty(),
        PropertyId::GridColumn => bindings::Grid::ColumnProperty(),
        PropertyId::GridRowSpan => bindings::Grid::RowSpanProperty(),
        PropertyId::GridColumnSpan => bindings::Grid::ColumnSpanProperty(),
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
            for value in values.iter().copied() {
                let definition = RowDefinition::new().map_err(native_error)?;
                definition
                    .SetHeight(grid_length(value))
                    .map_err(native_error)?;
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
            for value in values.iter().copied() {
                let definition = ColumnDefinition::new().map_err(native_error)?;
                definition
                    .SetWidth(grid_length(value))
                    .map_err(native_error)?;
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
    values: &[ReactorGridLength],
) -> Result<bool, RuntimeError> {
    if usize::try_from(definitions.Size().map_err(native_error)?).unwrap() != values.len() {
        return Ok(false);
    }
    for (index, value) in values.iter().copied().enumerate() {
        let index = u32::try_from(index).map_err(|_| RuntimeError::IndexOutOfBounds)?;
        let actual = definitions
            .GetAt(index)
            .and_then(|definition| definition.Height())
            .map_err(native_error)?;
        if !grid_lengths_match(actual, grid_length(value)) {
            return Ok(false);
        }
    }
    Ok(true)
}

fn columns_match(
    definitions: &ColumnDefinitionCollection,
    values: &[ReactorGridLength],
) -> Result<bool, RuntimeError> {
    if usize::try_from(definitions.Size().map_err(native_error)?).unwrap() != values.len() {
        return Ok(false);
    }
    for (index, value) in values.iter().copied().enumerate() {
        let index = u32::try_from(index).map_err(|_| RuntimeError::IndexOutOfBounds)?;
        let actual = definitions
            .GetAt(index)
            .and_then(|definition| definition.Width())
            .map_err(native_error)?;
        if !grid_lengths_match(actual, grid_length(value)) {
            return Ok(false);
        }
    }
    Ok(true)
}

fn grid_lengths_match(left: bindings::GridLength, right: bindings::GridLength) -> bool {
    left.grid_unit_type == right.grid_unit_type && f64_eq(left.value, right.value)
}

fn grid_length(value: ReactorGridLength) -> bindings::GridLength {
    let (value, grid_unit_type) = match value {
        ReactorGridLength::Auto => (0.0, GridUnitType::Auto),
        ReactorGridLength::Pixel(value) => (value, GridUnitType::Pixel),
        ReactorGridLength::Star(value) => (value, GridUnitType::Star),
    };
    bindings::GridLength {
        value,
        grid_unit_type,
    }
}
