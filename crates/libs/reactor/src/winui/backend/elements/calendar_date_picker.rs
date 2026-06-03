//! CalendarDatePicker — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    cdp: &Xaml::CalendarDatePicker,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::Header, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            cdp.put_Header(&tb)
        })()),
        (Prop::Header, PropValue::Unset) => Some(cdp.put_Header(None)),
        (Prop::Placeholder, PropValue::Str(s)) => Some(cdp.put_PlaceholderText(s.as_str())),
        (Prop::Placeholder, PropValue::Unset) => Some(cdp.put_PlaceholderText("")),
        (Prop::IsTodayHighlighted, PropValue::Bool(v)) => Some(cdp.put_IsTodayHighlighted(*v)),
        (Prop::IsCalendarOpen, PropValue::Bool(v)) => Some(cdp.put_IsCalendarOpen(*v)),
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| cdp.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        _ => None,
    }
}
