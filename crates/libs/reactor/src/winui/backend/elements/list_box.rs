//! ListBox — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    lb: &Xaml::ListBox,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::ListBoxItems, PropValue::StrList(items)) => Some((|| {
            let coll = lb
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
            Some((|| lb.cast::<Xaml::ISelector>()?.put_SelectedIndex(*v))())
        }
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| lb.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        _ => None,
    }
}
