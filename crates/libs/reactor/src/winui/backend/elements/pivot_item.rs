//! PivotItem — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    pi: &Xaml::PivotItem,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::PivotItemHeader, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            pi.put_Header(&tb)
        })()),
        _ => None,
    }
}
