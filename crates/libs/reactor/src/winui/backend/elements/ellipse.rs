//! Ellipse — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    e: &Xaml::Ellipse,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::Fill, PropValue::Brush(b)) => Some((|| {
            e.cast::<Xaml::IShape>()?
                .put_Fill(&super::super::brush_of(b)?)
        })()),
        (Prop::Stroke, PropValue::Brush(b)) => Some((|| {
            e.cast::<Xaml::IShape>()?
                .put_Stroke(&super::super::brush_of(b)?)
        })()),
        (Prop::StrokeThickness, PropValue::F64(v)) => {
            Some((|| e.cast::<Xaml::IShape>()?.put_StrokeThickness(*v))())
        }
        _ => None,
    }
}
