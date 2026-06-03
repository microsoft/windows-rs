//! TabViewItem — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    ti: &Xaml::TabViewItem,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::IsClosable, PropValue::Bool(v)) => Some(ti.put_IsClosable(*v)),
        (Prop::TabHeader, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            ti.put_Header(&tb)
        })()),
        (Prop::TabItemKey, PropValue::Str(s)) => Some((|| {
            let tag = windows_reference::IReference::from(s.as_str());
            ti.cast::<Xaml::IFrameworkElement>()?.put_Tag(&tag)
        })()),
        _ => None,
    }
}
