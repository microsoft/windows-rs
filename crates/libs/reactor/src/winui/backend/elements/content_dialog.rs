//! ContentDialog — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    d: &Xaml::ContentDialog,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::ContentDialogTitle, PropValue::Str(s)) => Some((|| {
            let title = windows_reference::IReference::from(s.as_str());
            d.put_Title(&title)
        })()),
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
