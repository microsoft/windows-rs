//! InfoBar — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler, Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    ib: &Xaml::InfoBar,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::InfoBarTitle, PropValue::Str(s)) => Some(ib.put_Title(s.as_str())),
        (Prop::InfoBarTitle, PropValue::Unset) => Some(ib.put_Title("")),
        (Prop::InfoBarMessage, PropValue::Str(s)) => Some(ib.put_Message(s.as_str())),
        (Prop::InfoBarMessage, PropValue::Unset) => Some(ib.put_Message("")),
        (Prop::InfoBarSeverity, PropValue::InfoBarSev(v)) => {
            Some(ib.put_Severity(super::super::to_winui_info_bar_severity(*v)))
        }
        (Prop::InfoBarIsOpen, PropValue::Bool(v)) => Some(ib.put_IsOpen(*v)),
        (Prop::IsClosable, PropValue::Bool(v)) => Some(ib.put_IsClosable(*v)),
        _ => None,
    }
}

pub(in crate::winui::backend) fn attach_event(
    ib: &Xaml::InfoBar,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::InfoBarClosed => {
            revokers.push(
                ib.add_Closed(move |_sender, _args| {
                    handler.invoke();
                })
                .unwrap(),
            );
        }
        _ => return None,
    }
    Some(revokers)
}
