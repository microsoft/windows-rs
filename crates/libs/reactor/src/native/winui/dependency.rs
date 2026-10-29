use super::*;

// Most declared properties map to a WinUI dependency property. The generated
// `GeneratedHandle::dependency_property` table names the property and how to box the value; this
// file performs the boxing and calls `SetValue` or `ClearValue`.

#[derive(Clone, Copy)]
pub(super) enum Conversion {
    Value,
    FontFamily,
    NativeColor,
    NumberBox,
    PathData,
    Rating,
    Uri,
}

/// A boxed dependency property write, applied after the adapter is released. Writes such as
/// `SelectedIndex` can lay out the control and realize virtualized rows within the call.
pub(super) struct DependencyWrite {
    element: native::IDependencyObject,
    property: native::DependencyProperty,
    // `None` clears the property; `Some(None)` sets it to null.
    value: Option<Option<IInspectable>>,
}

impl DependencyWrite {
    pub(super) fn apply(self) -> Result<(), WinUiError> {
        match self.value {
            Some(value) => self.element.SetValue(&self.property, value.as_ref())?,
            None => self.element.ClearValue(&self.property)?,
        }
        Ok(())
    }
}

pub(super) fn dependency_write(
    element: &native::UIElement,
    kind: ObjectType,
    property: PropertyId,
    value: Option<&PropertyValue>,
) -> Option<Result<DependencyWrite, WinUiError>> {
    let (dependency_property, conversion) = GeneratedHandle::dependency_property(kind, property)?;
    let value = match value {
        Some(value) => Some(box_property_value(value, conversion)?),
        None => None,
    };
    Some((|| {
        Ok(DependencyWrite {
            element: element.cast()?,
            property: dependency_property()?,
            value: value.transpose()?,
        })
    })())
}

// Returns `None` for values that are not set through a dependency property, such as theme
// brushes and image sources.
fn box_property_value(
    value: &PropertyValue,
    conversion: Conversion,
) -> Option<Result<Option<IInspectable>, WinUiError>> {
    fn boxed<T: windows_core::RuntimeType + Clone + 'static>(
        value: T,
    ) -> Result<Option<IInspectable>, WinUiError> {
        Ok(Some(windows_reference::IReference::from(value).into()))
    }
    fn object<T: Into<IInspectable>>(
        value: windows_core::Result<T>,
    ) -> Result<Option<IInspectable>, WinUiError> {
        Ok(Some(value?.into()))
    }
    Some(match (value, conversion) {
        (PropertyValue::String(value), Conversion::FontFamily) => {
            object(native::FontFamily::CreateInstanceWithName(value))
        }
        (PropertyValue::String(value), Conversion::PathData) => {
            parse_path_data(value).map(|value| Some(value.into()))
        }
        (PropertyValue::String(value), Conversion::Uri) => object(native::Uri::CreateUri(value)),
        (PropertyValue::String(value), _) => boxed(HSTRING::from(value.as_ref())),
        (PropertyValue::Bool(value), _) => boxed(*value),
        (PropertyValue::F64(value), _) => boxed(*value),
        (PropertyValue::I32(value), _) => boxed(*value),
        (PropertyValue::DateTime(value), _) => boxed(*value),
        (PropertyValue::OptionalBool(value), _) => value.map_or(Ok(None), boxed),
        (PropertyValue::OptionalDateTime(value), _) => value.map_or(Ok(None), boxed),
        (PropertyValue::OptionalF64(value), Conversion::NumberBox) => {
            boxed(native_number_box_value(*value))
        }
        (PropertyValue::OptionalF64(value), Conversion::Rating) => {
            boxed(native_rating_value(*value))
        }
        (PropertyValue::FontWeight(value), _) => boxed(native::FontWeight {
            weight: value.value(),
        }),
        (PropertyValue::CornerRadius(value), _) => boxed(native::CornerRadius {
            top_left: value.top_left,
            top_right: value.top_right,
            bottom_right: value.bottom_right,
            bottom_left: value.bottom_left,
        }),
        (PropertyValue::Thickness(value), _) => boxed(native::Thickness {
            left: value.left,
            top: value.top,
            right: value.right,
            bottom: value.bottom,
        }),
        (PropertyValue::Color(value), Conversion::NativeColor) => boxed(to_native_color(*value)),
        (PropertyValue::Color(value) | PropertyValue::Brush(Brush::Solid(value)), _) => {
            solid_color_brush(*value).map(|value| Some(value.into()))
        }
        (PropertyValue::SelectionIndex(value), _) => native_selection_index(*value).and_then(boxed),
        (PropertyValue::StringList(values), _) => {
            let values: Vec<Option<IInspectable>> = values
                .iter()
                .map(|value| Some(windows_reference::IReference::from(value.as_ref()).into()))
                .collect();
            let values: IVector<IInspectable> = values.into();
            object(values.cast::<IInspectable>())
        }
        (PropertyValue::ThemeTransitions(values), _) => theme_transitions(values),
        (PropertyValue::Enum { kind, variant }, _) => {
            Ok(Some(GeneratedHandle::box_enum(kind, variant)))
        }
        _ => return None,
    })
}

fn theme_transitions(values: &[ThemeTransition]) -> Result<Option<IInspectable>, WinUiError> {
    let collection = native::TransitionCollection::new()?;
    for value in values {
        let transition = match value {
            ThemeTransition::Reposition => {
                native::RepositionThemeTransition::new()?.cast::<native::Transition>()?
            }
        };
        collection.Append(&transition)?;
    }
    Ok(Some(collection.into()))
}
