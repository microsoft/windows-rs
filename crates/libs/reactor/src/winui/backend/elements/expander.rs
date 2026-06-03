//! Expander — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    e: &Xaml::Expander,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::Header, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            e.put_Header(&tb)
        })()),
        (Prop::Header, PropValue::Unset) => Some(e.put_Header(None)),
        (Prop::IsExpanded, PropValue::Bool(v)) => Some(e.put_IsExpanded(*v)),
        _ => None,
    }
}
