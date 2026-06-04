//! RepeatButton — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler, Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    b: &Xaml::RepeatButton,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::ButtonContent, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            b.cast::<Xaml::IContentControl>()?.put_Content(&tb)
        })()),
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| b.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        (Prop::RepeatDelay, PropValue::I32(v)) => Some(b.put_Delay(*v)),
        (Prop::RepeatInterval, PropValue::I32(v)) => Some(b.put_Interval(*v)),
        _ => None,
    }
}

pub(in crate::winui::backend) fn attach_event(
    b: &Xaml::RepeatButton,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::Click => {
            revokers.push(
                b.cast::<Xaml::IButtonBase>()
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
