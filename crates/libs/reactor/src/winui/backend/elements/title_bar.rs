//! TitleBar — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler, Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    tb: &Xaml::TitleBar,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::TitleBarTitle, PropValue::Str(s)) => Some(tb.put_Title(s.as_str())),
        (Prop::TitleBarSubtitle, PropValue::Str(s)) => Some(tb.put_Subtitle(s.as_str())),
        (Prop::TitleBarSubtitle, PropValue::Unset) => Some(tb.put_Subtitle("")),
        (Prop::TitleBarTall, PropValue::Bool(v)) => {
            crate::winui::host::set_titlebar_height(*v);
            Some(Ok(()))
        }
        (Prop::IsBackButtonVisible, PropValue::Bool(v)) => Some(tb.put_IsBackButtonVisible(*v)),
        (Prop::IsBackEnabled, PropValue::Bool(v)) => Some(tb.put_IsBackButtonEnabled(*v)),
        (Prop::IsPaneToggleButtonVisible, PropValue::Bool(v)) => {
            Some(tb.put_IsPaneToggleButtonVisible(*v))
        }
        _ => None,
    }
}

pub(in crate::winui::backend) fn attach_event(
    tb: &Xaml::TitleBar,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::TitleBarBackRequested => {
            revokers.push(
                tb.add_BackRequested(move |_sender, _args| {
                    handler.invoke();
                })
                .unwrap(),
            );
        }
        Event::TitleBarPaneToggle => {
            revokers.push(
                tb.add_PaneToggleRequested(move |_sender, _args| {
                    handler.invoke();
                })
                .unwrap(),
            );
        }
        _ => return None,
    }
    Some(revokers)
}
