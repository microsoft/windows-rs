//! HyperlinkButton — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    h: &Xaml::HyperlinkButton,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::ButtonContent, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            h.cast::<Xaml::IContentControl>()?.put_Content(&tb)
        })()),
        (Prop::NavigateUri, PropValue::Str(s)) => Some((|| {
            let uri = Xaml::Uri::CreateUri(s.as_str())?;
            h.put_NavigateUri(&uri)
        })()),
        (Prop::NavigateUri, PropValue::Unset) => Some(h.put_NavigateUri(None)),
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| h.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        _ => None,
    }
}
