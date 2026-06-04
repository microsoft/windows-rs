//! RichEditBox — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler, Prop, PropValue};
use windows_core::Interface;

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

pub(in crate::winui::backend) fn attach_event(
    reb: &Xaml::RichEditBox,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::RichEditBoxTextChanged => {
            revokers.push(
                reb.add_TextChanged(move |sender, _args| {
                    let text = sender
                        .as_ref()
                        .and_then(|s| s.cast::<Xaml::RichEditBox>().ok())
                        .and_then(|reb| {
                            let doc = reb.get_Document().ok()?;
                            let mut buf = windows_core::HSTRING::default();
                            doc.GetText(Xaml::TextGetOptions::None, &mut buf).ok()?;
                            Some(buf.to_string_lossy())
                        })
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
