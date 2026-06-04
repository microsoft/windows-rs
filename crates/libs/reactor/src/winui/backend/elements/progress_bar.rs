//! ProgressBar — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    p: &Xaml::ProgressBar,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::NumericValue, PropValue::F64(v)) => {
            Some((|| p.cast::<Xaml::IRangeBase>()?.put_Value(*v))())
        }
        (Prop::Minimum, PropValue::F64(v)) => {
            Some((|| p.cast::<Xaml::IRangeBase>()?.put_Minimum(*v))())
        }
        (Prop::Maximum, PropValue::F64(v)) => {
            Some((|| p.cast::<Xaml::IRangeBase>()?.put_Maximum(*v))())
        }
        (Prop::IsIndeterminate, PropValue::Bool(v)) => Some(p.put_IsIndeterminate(*v)),
        _ => None,
    }
}
