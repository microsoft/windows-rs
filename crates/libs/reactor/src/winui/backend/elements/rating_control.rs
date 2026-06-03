//! RatingControl — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    r: &Xaml::RatingControl,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::NumericValue, PropValue::F64(v)) => Some(r.put_Value(*v)),
        (Prop::MaxRating, PropValue::I32(v)) => Some(r.put_MaxRating(*v)),
        (Prop::RatingCaption, PropValue::Str(s)) => Some(r.put_Caption(s.as_str())),
        (Prop::RatingCaption, PropValue::Unset) => Some(r.put_Caption("")),
        (Prop::PlaceholderValue, PropValue::F64(v)) => Some(r.put_PlaceholderValue(*v)),
        (Prop::IsReadOnly, PropValue::Bool(v)) => Some(r.put_IsReadOnly(*v)),
        _ => None,
    }
}
