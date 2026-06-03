//! Expander — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler, Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    e: &Xaml::Expander,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::Header, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            e.put_Header(&tb)
        })()),
        (Prop::Header, PropValue::Unset) => Some(e.put_Header(None)),
        (Prop::IsExpanded, PropValue::Bool(v)) => Some(e.put_IsExpanded(*v)),
        _ => None,
    }
}

pub(in crate::winui::backend) fn attach_event(
    e: &Xaml::Expander,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::ExpandedChanged => {
            let expanding_handler = handler.clone();
            revokers.push(
                e.add_Expanding(move |_sender, _args| {
                    expanding_handler.invoke_bool(true);
                })
                .unwrap(),
            );
            revokers.push(
                e.add_Collapsed(move |_sender, _args| {
                    handler.invoke_bool(false);
                })
                .unwrap(),
            );
        }
        _ => return None,
    }
    Some(revokers)
}
