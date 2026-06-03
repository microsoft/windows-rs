//! ToggleButton — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    tb: &Xaml::ToggleButton,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::IsChecked, PropValue::Bool(v)) => Some(tb.put_IsChecked(Some(*v))),
        (Prop::IsChecked, PropValue::Unset) => Some(tb.put_IsChecked(None)),
        (Prop::CheckBoxLabel, PropValue::Str(s)) => Some((|| {
            let txt = super::super::string_as_textblock(s)?;
            tb.cast::<Xaml::IContentControl>()?.put_Content(&txt)
        })()),
        (Prop::CheckBoxLabel, PropValue::Unset) => {
            Some((|| tb.cast::<Xaml::IContentControl>()?.put_Content(None))())
        }
        _ => None,
    }
}
