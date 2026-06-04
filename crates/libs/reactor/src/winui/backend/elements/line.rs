//! Line — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    l: &Xaml::Line,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::Stroke, PropValue::Brush(b)) => Some((|| {
            l.cast::<Xaml::IShape>()?
                .put_Stroke(&super::super::brush_of(b)?)
        })()),
        (Prop::StrokeThickness, PropValue::F64(v)) => {
            Some((|| l.cast::<Xaml::IShape>()?.put_StrokeThickness(*v))())
        }
        (Prop::LineEndpoints, PropValue::LineEndpoints(p)) => Some(
            l.put_X1(p.x1)
                .and_then(|_| l.put_Y1(p.y1))
                .and_then(|_| l.put_X2(p.x2))
                .and_then(|_| l.put_Y2(p.y2)),
        ),
        _ => None,
    }
}
