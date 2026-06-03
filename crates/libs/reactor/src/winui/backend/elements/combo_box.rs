//! ComboBox — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    c: &Xaml::ComboBox,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::ComboBoxItems, PropValue::StrList(items)) => Some((|| {
            let coll = c
                .cast::<Xaml::IItemsControl>()?
                .get_Items()?
                .cast::<windows_collections::IVector<windows_core::IInspectable>>()?;
            coll.Clear()?;
            for s in items {
                let insp = windows_reference::IReference::from(s.as_str());
                coll.Append(&insp)?;
            }
            Ok(())
        })()),
        (Prop::SelectedIndex, PropValue::I32(v)) => {
            Some((|| c.cast::<Xaml::ISelector>()?.put_SelectedIndex(*v))())
        }
        (Prop::Header, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            c.put_Header(&tb)
        })()),
        (Prop::Header, PropValue::Unset) => Some(c.put_Header(None)),
        (Prop::Placeholder, PropValue::Str(s)) => Some(c.put_PlaceholderText(s.as_str())),
        (Prop::Placeholder, PropValue::Unset) => Some(c.put_PlaceholderText("")),
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| c.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        (Prop::IsEditable, PropValue::Bool(v)) => Some(c.put_IsEditable(*v)),
        _ => None,
    }
}
