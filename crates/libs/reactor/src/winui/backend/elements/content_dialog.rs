//! ContentDialog — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler, Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    d: &Xaml::ContentDialog,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::ContentDialogTitle, PropValue::Str(s)) => Some({
            let title = windows_reference::IReference::from(s.as_str());
            d.put_Title(&title)
        }),
        (Prop::ContentDialogTitle, PropValue::Unset) => Some(d.put_Title(None)),
        (Prop::ContentDialogBody, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            d.cast::<Xaml::IContentControl>()?.put_Content(&tb)
        })()),
        (Prop::ContentDialogBody, PropValue::Unset) => {
            Some((|| d.cast::<Xaml::IContentControl>()?.put_Content(None))())
        }
        (Prop::ContentDialogPrimaryText, PropValue::Str(s)) => {
            Some(d.put_PrimaryButtonText(s.as_str()))
        }
        (Prop::ContentDialogPrimaryText, PropValue::Unset) => Some(d.put_PrimaryButtonText("")),
        (Prop::ContentDialogSecondaryText, PropValue::Str(s)) => {
            Some(d.put_SecondaryButtonText(s.as_str()))
        }
        (Prop::ContentDialogSecondaryText, PropValue::Unset) => Some(d.put_SecondaryButtonText("")),
        (Prop::ContentDialogCloseText, PropValue::Str(s)) => {
            Some(d.put_CloseButtonText(s.as_str()))
        }
        (Prop::ContentDialogCloseText, PropValue::Unset) => Some(d.put_CloseButtonText("")),
        (Prop::ContentDialogPrimaryEnabled, PropValue::Bool(v)) => {
            Some(d.put_IsPrimaryButtonEnabled(*v))
        }
        (Prop::ContentDialogSecondaryEnabled, PropValue::Bool(v)) => {
            Some(d.put_IsSecondaryButtonEnabled(*v))
        }
        _ => None,
    }
}

pub(in crate::winui::backend) fn attach_event(
    d: &Xaml::ContentDialog,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::ContentDialogClosed => {
            revokers.push(
                d.add_Closed(move |_sender, args| {
                    let result = args
                        .as_ref()
                        .and_then(|a| a.get_Result().ok())
                        .unwrap_or(Xaml::ContentDialogResult(0));
                    handler.invoke_i32(result.0);
                })
                .unwrap(),
            );
        }
        _ => return None,
    }
    Some(revokers)
}
