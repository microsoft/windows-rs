use super::*;
use crate::{
    HorizontalAlignment as ReactorHorizontalAlignment,
    VerticalAlignment as ReactorVerticalAlignment,
};

pub(super) fn set(
    element: &UIElement,
    property: PropertyId,
    value: &PropertyValue,
) -> Result<(), RuntimeError> {
    if let (PropertyId::Opacity, PropertyValue::F64(value)) = (property, value) {
        return element.SetOpacity(*value).map_err(native_error);
    }

    let element = element.cast::<FrameworkElement>().map_err(native_error)?;
    match (property, value) {
        (PropertyId::Width, PropertyValue::F64(value)) => {
            element.SetWidth(*value).map_err(native_error)
        }
        (PropertyId::Height, PropertyValue::F64(value)) => {
            element.SetHeight(*value).map_err(native_error)
        }
        (PropertyId::MinWidth, PropertyValue::F64(value)) => {
            element.SetMinWidth(*value).map_err(native_error)
        }
        (PropertyId::MaxWidth, PropertyValue::F64(value)) => {
            element.SetMaxWidth(*value).map_err(native_error)
        }
        (PropertyId::MinHeight, PropertyValue::F64(value)) => {
            element.SetMinHeight(*value).map_err(native_error)
        }
        (PropertyId::MaxHeight, PropertyValue::F64(value)) => {
            element.SetMaxHeight(*value).map_err(native_error)
        }
        (PropertyId::HorizontalAlignment, PropertyValue::HorizontalAlignment(value)) => element
            .SetHorizontalAlignment(match value {
                ReactorHorizontalAlignment::Left => bindings::HorizontalAlignment::Left,
                ReactorHorizontalAlignment::Center => bindings::HorizontalAlignment::Center,
                ReactorHorizontalAlignment::Right => bindings::HorizontalAlignment::Right,
                ReactorHorizontalAlignment::Stretch => bindings::HorizontalAlignment::Stretch,
            })
            .map_err(native_error),
        (PropertyId::VerticalAlignment, PropertyValue::VerticalAlignment(value)) => element
            .SetVerticalAlignment(match value {
                ReactorVerticalAlignment::Top => bindings::VerticalAlignment::Top,
                ReactorVerticalAlignment::Center => bindings::VerticalAlignment::Center,
                ReactorVerticalAlignment::Bottom => bindings::VerticalAlignment::Bottom,
                ReactorVerticalAlignment::Stretch => bindings::VerticalAlignment::Stretch,
            })
            .map_err(native_error),
        (PropertyId::Margin, PropertyValue::Thickness(value)) => element
            .SetMargin(bindings::Thickness {
                left: value.left(),
                top: value.top(),
                right: value.right(),
                bottom: value.bottom(),
            })
            .map_err(native_error),
        _ => Err(RuntimeError::UnsupportedKind),
    }
}

pub(super) fn clear(element: &UIElement, property: PropertyId) -> Result<(), RuntimeError> {
    let dependency_property = match property {
        PropertyId::Width => FrameworkElement::WidthProperty(),
        PropertyId::Height => FrameworkElement::HeightProperty(),
        PropertyId::MinWidth => FrameworkElement::MinWidthProperty(),
        PropertyId::MaxWidth => FrameworkElement::MaxWidthProperty(),
        PropertyId::MinHeight => FrameworkElement::MinHeightProperty(),
        PropertyId::MaxHeight => FrameworkElement::MaxHeightProperty(),
        PropertyId::Opacity => UIElement::OpacityProperty(),
        PropertyId::HorizontalAlignment => FrameworkElement::HorizontalAlignmentProperty(),
        PropertyId::VerticalAlignment => FrameworkElement::VerticalAlignmentProperty(),
        PropertyId::Margin => FrameworkElement::MarginProperty(),
        _ => return Err(RuntimeError::UnsupportedKind),
    }
    .map_err(native_error)?;
    element
        .cast::<IDependencyObject>()
        .map_err(native_error)?
        .ClearValue(&dependency_property)
        .map_err(native_error)
}
