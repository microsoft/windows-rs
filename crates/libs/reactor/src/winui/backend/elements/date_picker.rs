//! DatePicker — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    dp: &Xaml::DatePicker,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::Header, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            dp.put_Header(&tb)
        })()),
        (Prop::Header, PropValue::Unset) => Some(dp.put_Header(None)),
        (Prop::DayVisible, PropValue::Bool(v)) => Some(dp.put_DayVisible(*v)),
        (Prop::MonthVisible, PropValue::Bool(v)) => Some(dp.put_MonthVisible(*v)),
        (Prop::YearVisible, PropValue::Bool(v)) => Some(dp.put_YearVisible(*v)),
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| dp.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        _ => None,
    }
}
