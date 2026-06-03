//! DropDownButton — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler, Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    ddb: &Xaml::DropDownButton,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::ButtonContent, PropValue::Str(s)) => Some((|| {
            let insp = windows_reference::IReference::from(s.as_str());
            ddb.cast::<Xaml::IContentControl>()?.put_Content(&insp)
        })()),
        (Prop::ButtonContent, PropValue::Unset) => {
            Some((|| ddb.cast::<Xaml::IContentControl>()?.put_Content(None))())
        }
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| ddb.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        _ => None,
    }
}

pub(in crate::winui::backend) fn attach_event(
    ddb: &Xaml::DropDownButton,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::Click => {
            revokers.push(
                ddb.cast::<Xaml::IButtonBase>()
                    .unwrap()
                    .add_Click(move |_sender, _args| {
                        handler.invoke();
                    })
                    .unwrap(),
            );
        }
        _ => return None,
    }
    Some(revokers)
}
