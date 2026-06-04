//! PasswordBox — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler, Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    p: &Xaml::PasswordBox,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::PasswordValue, PropValue::Str(s)) => Some((|| {
            if p.get_Password().ok().as_deref() == Some(s.as_str()) {
                return Ok(());
            }
            p.put_Password(s.as_str())
        })()),
        (Prop::PasswordValue, PropValue::Unset) => Some(p.put_Password("")),
        (Prop::Placeholder, PropValue::Str(s)) => Some(p.put_PlaceholderText(s.as_str())),
        (Prop::Placeholder, PropValue::Unset) => Some(p.put_PlaceholderText("")),
        (Prop::Header, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            p.put_Header(&tb)
        })()),
        (Prop::Header, PropValue::Unset) => Some(p.put_Header(None)),
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| p.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        (Prop::PasswordRevealMode, PropValue::PasswordRevealMode(m)) => Some({
            use crate::core::widgets::PasswordRevealMode as M;
            let mapped = match m {
                M::Peek => Xaml::PasswordRevealMode::Peek,
                M::Hidden => Xaml::PasswordRevealMode::Hidden,
                M::Visible => Xaml::PasswordRevealMode::Visible,
            };
            p.put_PasswordRevealMode(mapped)
        }),
        (Prop::IsPasswordRevealButtonEnabled, PropValue::Bool(v)) => {
            Some(p.put_IsPasswordRevealButtonEnabled(*v))
        }
        (Prop::IsPasswordRevealButtonEnabled, PropValue::Unset) => {
            Some(p.put_IsPasswordRevealButtonEnabled(true))
        }
        _ => None,
    }
}

pub(in crate::winui::backend) fn attach_event(
    p: &Xaml::PasswordBox,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::PasswordChanged => {
            revokers.push(
                p.add_PasswordChanged(move |sender, _args| {
                    let text = sender
                        .as_ref()
                        .and_then(|s| s.cast::<Xaml::PasswordBox>().ok())
                        .and_then(|pb| pb.get_Password().ok())
                        .unwrap_or_default();
                    handler.invoke_string(text);
                })
                .unwrap(),
            );
        }
        _ => return None,
    }
    Some(revokers)
}
