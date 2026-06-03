//! ListBox — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler, Prop, PropValue};
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

pub(in crate::winui::backend) fn attach_event(
    lb: &Xaml::ListBox,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::ListBoxSelectionChanged => {
            revokers.push(
                lb.cast::<Xaml::ISelector>()
                    .unwrap()
                    .add_SelectionChanged(move |sender, _args| {
                        if let Some(sel) = sender.as_ref()
                            && let Ok(idx) = sel
                                .cast::<Xaml::ISelector>()
                                .and_then(|s| s.get_SelectedIndex())
                        {
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
