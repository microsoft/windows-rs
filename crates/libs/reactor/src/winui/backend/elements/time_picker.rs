//! TimePicker — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    tp: &Xaml::TimePicker,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::Header, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            tp.put_Header(&tb)
        })()),
        (Prop::Header, PropValue::Unset) => Some(tp.put_Header(None)),
        (Prop::ClockIdentifier, PropValue::Str(s)) => Some(tp.put_ClockIdentifier(s.as_str())),
        (Prop::MinuteIncrement, PropValue::I32(v)) => Some(tp.put_MinuteIncrement(*v)),
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| tp.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        _ => None,
    }
}
