//! AutoSuggestBox — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    asb: &Xaml::AutoSuggestBox,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::AutoSuggestText, PropValue::Str(s)) => Some((|| {
            if asb.get_Text().ok().as_deref() == Some(s.as_str()) {
                return Ok(());
            }
            asb.put_Text(s)
        })()),
        (Prop::AutoSuggestItems, PropValue::StrList(items)) => Some((|| {
            let vec: Vec<Option<windows_core::IInspectable>> = items
                .iter()
                .map(|s| {
                    let r = windows_reference::IReference::from(s.as_str());
                    Some(r.into())
                })
                .collect();
            let ivec: windows_collections::IVector<windows_core::IInspectable> = vec.into();
            asb.cast::<Xaml::IItemsControl>()?.put_ItemsSource(&ivec)
        })()),
        (Prop::Placeholder, PropValue::Str(s)) => Some(asb.put_PlaceholderText(s)),
        (Prop::Header, PropValue::Str(s)) => Some((|| {
            let insp = windows_reference::IReference::from(s.as_str());
            asb.put_Header(&insp)
        })()),
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| asb.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        _ => None,
    }
}
