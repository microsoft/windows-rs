//! BreadcrumbBar — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler, Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    bc: &Xaml::BreadcrumbBar,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::BreadcrumbItems, PropValue::StrList(items)) => {
            let vec: Vec<Option<windows_core::IInspectable>> = items
                .iter()
                .map(|s| {
                    let r = windows_reference::IReference::from(s.as_str());
                    Some(r.into())
                })
                .collect();
            let ivec: windows_collections::IVector<windows_core::IInspectable> = vec.into();
            Some(bc.put_ItemsSource(&ivec))
        }
        _ => None,
    }
}

pub(in crate::winui::backend) fn attach_event(
    bc: &Xaml::BreadcrumbBar,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::BreadcrumbItemClicked => {
            revokers.push(
                bc.add_ItemClicked(move |_sender, args| {
                    if let Some(idx) = args.as_ref().and_then(|a| a.get_Index().ok()) {
                        handler.invoke_i32(idx);
                    }
                })
                .unwrap(),
            );
        }
        _ => return None,
    }
    Some(revokers)
}
