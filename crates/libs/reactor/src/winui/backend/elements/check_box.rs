//! CheckBox — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler, Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    c: &Xaml::CheckBox,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| c.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        (Prop::IsChecked, PropValue::Bool(v)) => Some((|| {
            c.cast::<Xaml::IToggleButton>()?.put_IsChecked(Some(*v))
        })()),
        (Prop::IsChecked, PropValue::Unset) => {
            Some((|| c.cast::<Xaml::IToggleButton>()?.put_IsChecked(None))())
        }
        (Prop::CheckBoxLabel, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            c.cast::<Xaml::IContentControl>()?.put_Content(&tb)
        })()),
        (Prop::CheckBoxLabel, PropValue::Unset) => {
            Some((|| c.cast::<Xaml::IContentControl>()?.put_Content(None))())
        }
        _ => None,
    }
}

pub(in crate::winui::backend) fn attach_event(
    c: &Xaml::CheckBox,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::CheckedChanged => {
            let checked_handler = handler.clone();
            revokers.push(
                c.cast::<Xaml::IToggleButton>()
                    .unwrap()
                    .add_Checked(move |sender, _args| {
                        let is_checked = super::super::sender_is_checked(sender);
                        checked_handler.invoke_bool(is_checked);
                    })
                    .unwrap(),
            );
            revokers.push(
                c.cast::<Xaml::IToggleButton>()
                    .unwrap()
                    .add_Unchecked(move |sender, _args| {
                        let is_checked = super::super::sender_is_checked(sender);
                        handler.invoke_bool(is_checked);
                    })
                    .unwrap(),
            );
        }
        _ => return None,
    }
    Some(revokers)
}
