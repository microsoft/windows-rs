//! Border — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use crate::core::geometry::Thickness;

pub(in crate::winui::backend) fn set_prop(
    b: &Xaml::Border,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::Padding, PropValue::Thickness(t)) => {
            Some(b.put_Padding(super::super::to_xaml_thickness(*t)))
        }
        (Prop::Padding, PropValue::Unset) => {
            Some(b.put_Padding(super::super::to_xaml_thickness(Thickness::default())))
        }
        (Prop::Background, PropValue::Brush(br)) => {
            Some((|| b.put_Background(&super::super::brush_of(br)?))())
        }
        (Prop::Background, PropValue::Unset) => Some(b.put_Background(None)),
        (Prop::CornerRadius, PropValue::F64(v)) => Some(b.put_CornerRadius(Xaml::CornerRadius {
            TopLeft: *v,
            TopRight: *v,
            BottomRight: *v,
            BottomLeft: *v,
        })),
        (Prop::CornerRadius, PropValue::Unset) => {
            Some(b.put_CornerRadius(Xaml::CornerRadius::default()))
        }
        (Prop::BorderBrush, PropValue::Brush(br)) => {
            Some((|| b.put_BorderBrush(&super::super::brush_of(br)?))())
        }
        (Prop::BorderBrush, PropValue::Unset) => Some(b.put_BorderBrush(None)),
        (Prop::BorderThickness, PropValue::Thickness(t)) => {
            Some(b.put_BorderThickness(super::super::to_xaml_thickness(*t)))
        }
        (Prop::BorderThickness, PropValue::Unset) => {
            Some(b.put_BorderThickness(super::super::to_xaml_thickness(Thickness::default())))
        }
        _ => None,
    }
}
