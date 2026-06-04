//! SplitButton — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler, Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    sb: &Xaml::SplitButton,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::ButtonContent, PropValue::Str(s)) => Some((|| {
            let insp = windows_reference::IReference::from(s.as_str());
            sb.cast::<Xaml::IContentControl>()?.put_Content(&insp)
        })()),
        (Prop::ButtonContent, PropValue::Unset) => {
            Some((|| sb.cast::<Xaml::IContentControl>()?.put_Content(None))())
        }
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| sb.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        _ => None,
    }
}

pub(in crate::winui::backend) fn attach_event(
    sb: &Xaml::SplitButton,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::SplitButtonClick => {
            revokers.push(
                sb.add_Click(move |_sender, _args| {
                    handler.invoke();
                })
                .unwrap(),
            );
        }
        _ => return None,
    }
    Some(revokers)
}
