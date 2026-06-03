//! SplitButton — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    sb: &Xaml::SplitButton,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::ButtonContent, PropValue::Str(s)) => Some((|| {
            let insp = windows_reference::IReference::from(s.as_str());
            sb.cast::<Xaml::IContentControl>()?.put_Content(&insp)
        })()),
        (Prop::ButtonContent, PropValue::Unset) => {
            Some((|| sb.cast::<Xaml::IContentControl>()?.put_Content(None))())
        }
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| sb.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        _ => None,
    }
}
