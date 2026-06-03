//! CalendarView — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    cv: &Xaml::CalendarView,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::IsTodayHighlighted, PropValue::Bool(v)) => Some(cv.put_IsTodayHighlighted(*v)),
        (Prop::IsGroupLabelVisible, PropValue::Bool(v)) => Some(cv.put_IsGroupLabelVisible(*v)),
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| cv.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        _ => None,
    }
}
