//! TextBox — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    t: &Xaml::TextBox,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| t.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        (Prop::TextBoxValue, PropValue::Str(s)) => Some((|| {
            if t.get_Text().ok().as_deref() == Some(s.as_str()) {
                return Ok(());
            }
            t.put_Text(s.as_str())
        })()),
        (Prop::Placeholder, PropValue::Str(s)) => Some(t.put_PlaceholderText(s.as_str())),
        (Prop::Placeholder, PropValue::Unset) => Some(t.put_PlaceholderText("")),
        (Prop::Header, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            t.put_Header(&tb)
        })()),
        (Prop::Header, PropValue::Unset) => Some(t.put_Header(None)),
        (Prop::AcceptsReturn, PropValue::Bool(v)) => Some(t.put_AcceptsReturn(*v)),
        (Prop::AcceptsReturn, PropValue::Unset) => Some(t.put_AcceptsReturn(false)),
        (Prop::TextWrappingWrap, PropValue::Bool(v)) => {
            let mode = if *v {
                Xaml::TextWrapping::Wrap
            } else {
                Xaml::TextWrapping::NoWrap
            };
            Some(t.put_TextWrapping(mode))
        }
        (Prop::TextWrappingWrap, PropValue::Unset) => {
            Some(t.put_TextWrapping(Xaml::TextWrapping::NoWrap))
        }
        _ => None,
    }
}
