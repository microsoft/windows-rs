//! RichEditBox — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    reb: &Xaml::RichEditBox,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::RichEditBoxText, PropValue::Str(s)) => Some((|| {
            let doc = reb.get_Document()?;
            let mut current = windows_core::HSTRING::default();
            doc.GetText(Xaml::TextGetOptions::None, &mut current).ok();
            if current == s.as_str() {
                return Ok(());
            }
            doc.SetText(Xaml::TextSetOptions::None, s.as_str())
        })()),
        (Prop::Placeholder, PropValue::Str(s)) => Some(reb.put_PlaceholderText(s.as_str())),
        (Prop::Placeholder, PropValue::Unset) => Some(reb.put_PlaceholderText("")),
        (Prop::Header, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            reb.put_Header(&tb)
        })()),
        (Prop::Header, PropValue::Unset) => Some(reb.put_Header(None)),
        (Prop::RichEditBoxIsReadOnly, PropValue::Bool(v)) => Some(reb.put_IsReadOnly(*v)),
        _ => None,
    }
}
