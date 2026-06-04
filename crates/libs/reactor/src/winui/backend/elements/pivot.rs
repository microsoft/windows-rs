//! Pivot — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler, Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    p: &Xaml::Pivot,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::SelectedIndex, PropValue::I32(v)) => Some(p.put_SelectedIndex(*v)),
        (Prop::PivotTitle, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            p.put_Title(&tb)
        })()),
        (Prop::PivotTitle, PropValue::Unset) => Some(p.put_Title(None)),
        _ => None,
    }
}

pub(in crate::winui::backend) fn attach_event(
    p: &Xaml::Pivot,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::PivotSelectionChanged => {
            revokers.push(
                p.add_SelectionChanged(move |sender, _args| {
                    let idx = sender
                        .as_ref()
                        .and_then(|s| s.cast::<Xaml::Selector>().ok())
                        .and_then(|sel| {
                            sel.cast::<Xaml::ISelector>()
                                .unwrap()
                                .get_SelectedIndex()
                                .ok()
                        })
                        .unwrap_or(-1);
                    if idx >= 0 {
                        handler.invoke_i32(idx);
                    }
                })
                .unwrap(),
            );
        }
        _ => return None,
    }
    Some(revokers)
}
