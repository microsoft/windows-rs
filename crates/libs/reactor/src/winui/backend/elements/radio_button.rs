//! RadioButton — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler, Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    r: &Xaml::RadioButton,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::RadioLabel, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            r.cast::<Xaml::IContentControl>()?.put_Content(&tb)
        })()),
        (Prop::RadioLabel, PropValue::Unset) => {
            Some((|| r.cast::<Xaml::IContentControl>()?.put_Content(None))())
        }
        (Prop::IsChecked, PropValue::Bool(v)) => Some((|| {
            r.cast::<Xaml::IToggleButton>()?.put_IsChecked(Some(*v))
        })()),
        (Prop::GroupName, PropValue::Str(s)) => Some(r.put_GroupName(s.as_str())),
        (Prop::GroupName, PropValue::Unset) => Some(r.put_GroupName("")),
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| r.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        _ => None,
    }
}

pub(in crate::winui::backend) fn attach_event(
    r: &Xaml::RadioButton,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::RadioChecked => {
            revokers.push(
                r.cast::<Xaml::IToggleButton>()
                    .unwrap()
                    .add_Checked(move |_sender, _args| {
                        handler.invoke();
                    })
                    .unwrap(),
            );
        }
        _ => return None,
    }
    Some(revokers)
}
