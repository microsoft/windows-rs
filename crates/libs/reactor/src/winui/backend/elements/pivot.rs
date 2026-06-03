//! Pivot — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    p: &Xaml::Pivot,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::SelectedIndex, PropValue::I32(v)) => Some(p.put_SelectedIndex(*v)),
        (Prop::PivotTitle, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            p.put_Title(&tb)
        })()),
        (Prop::PivotTitle, PropValue::Unset) => Some(p.put_Title(None)),
        _ => None,
    }
}
