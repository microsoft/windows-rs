//! RadioButtons — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    r: &Xaml::RadioButtons,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::RadioButtonsItems, PropValue::StrList(items)) => Some((|| {
            let vec = r.get_Items()?;
            vec.Clear()?;
            for s in items {
                let insp = windows_reference::IReference::from(s.as_str());
                vec.Append(&insp)?;
            }
            Ok(())
        })()),
        (Prop::SelectedIndex, PropValue::I32(v)) => Some(r.put_SelectedIndex(*v)),
        (Prop::Header, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            r.put_Header(&tb)
        })()),
        (Prop::Header, PropValue::Unset) => Some(r.put_Header(None)),
        (Prop::RadioButtonsMaxColumns, PropValue::I32(v)) => Some(r.put_MaxColumns(*v)),
        _ => None,
    }
}
