//! ToggleSwitch — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler, Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    ts: &Xaml::ToggleSwitch,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::IsOn, PropValue::Bool(v)) => Some(ts.put_IsOn(*v)),
        (Prop::OnContent, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            ts.put_OnContent(&tb)
        })()),
        (Prop::OnContent, PropValue::Unset) => Some(ts.put_OnContent(None)),
        (Prop::OffContent, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            ts.put_OffContent(&tb)
        })()),
        (Prop::OffContent, PropValue::Unset) => Some(ts.put_OffContent(None)),
        (Prop::Header, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            ts.put_Header(&tb)
        })()),
        (Prop::Header, PropValue::Unset) => Some(ts.put_Header(None)),
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| ts.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        _ => None,
    }
}

pub(in crate::winui::backend) fn attach_event(
    ts: &Xaml::ToggleSwitch,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::Toggled => {
            revokers.push(
                ts.add_Toggled(move |sender, _args| {
                    let is_on = sender
                        .as_ref()
                        .and_then(|s| s.cast::<Xaml::ToggleSwitch>().ok())
                        .and_then(|ts| ts.get_IsOn().ok())
                        .unwrap_or(false);
                    handler.invoke_bool(is_on);
                })
                .unwrap(),
            );
        }
        _ => return None,
    }
    Some(revokers)
}
