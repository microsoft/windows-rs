//! TabView — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler, Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    tv: &Xaml::TabView,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::SelectedIndex, PropValue::I32(v)) => Some(tv.put_SelectedIndex(*v)),
        (Prop::CanReorderTabs, PropValue::Bool(v)) => Some(tv.put_CanReorderTabs(*v)),
        (Prop::IsAddTabButtonVisible, PropValue::Bool(v)) => Some(tv.put_IsAddTabButtonVisible(*v)),
        _ => None,
    }
}

pub(in crate::winui::backend) fn attach_event(
    tv: &Xaml::TabView,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::TabSelectionChanged => {
            revokers.push(
                tv.add_SelectionChanged(move |sender, _args| {
                    let idx = sender
                        .as_ref()
                        .and_then(|s| s.cast::<Xaml::TabView>().ok())
                        .and_then(|t| t.get_SelectedIndex().ok())
                        .unwrap_or(-1);
                    if idx >= 0 {
                        handler.invoke_i32(idx);
                    }
                })
                .unwrap(),
            );
        }
        Event::TabCloseRequested => {
            revokers.push(
                tv.add_TabCloseRequested(move |_sender, args| {
                    let key = args
                        .as_ref()
                        .and_then(|a| a.get_Tab().ok())
                        .and_then(|tab| {
                            tab.cast::<Xaml::IFrameworkElement>()
                                .unwrap()
                                .get_Tag()
                                .ok()
                        })
                        .and_then(|tag_obj| {
                            tag_obj
                                .cast::<windows_reference::IReference<windows_core::HSTRING>>()
                                .ok()
                                .and_then(|pv| pv.Value().ok())
                        })
                        .map(|h| h.to_string_lossy())
                        .unwrap_or_default();
                    handler.invoke_string(key);
                })
                .unwrap(),
            );
        }
        Event::AddTabButtonClick => {
            revokers.push(
                tv.add_AddTabButtonClick(move |_sender, _args| {
                    handler.invoke();
                })
                .unwrap(),
            );
        }
        _ => return None,
    }
    Some(revokers)
}
