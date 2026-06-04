//! Rectangle — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    r: &Xaml::Rectangle,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::Fill, PropValue::Brush(b)) => Some((|| {
            r.cast::<Xaml::IShape>()?
                .put_Fill(&super::super::brush_of(b)?)
        })()),
        (Prop::Stroke, PropValue::Brush(b)) => Some((|| {
            r.cast::<Xaml::IShape>()?
                .put_Stroke(&super::super::brush_of(b)?)
        })()),
        (Prop::StrokeThickness, PropValue::F64(v)) => {
            Some((|| r.cast::<Xaml::IShape>()?.put_StrokeThickness(*v))())
        }
        (Prop::CornerRadius, PropValue::F64(v)) => {
            Some(r.put_RadiusX(*v).and_then(|_| r.put_RadiusY(*v)))
        }
        (Prop::CornerRadius, PropValue::Unset) => {
            Some(r.put_RadiusX(0.0).and_then(|_| r.put_RadiusY(0.0)))
        }
        _ => None,
    }
}
