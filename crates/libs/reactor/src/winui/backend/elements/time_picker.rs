//! TimePicker — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler, Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    tp: &Xaml::TimePicker,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::Header, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            tp.put_Header(&tb)
        })()),
        (Prop::Header, PropValue::Unset) => Some(tp.put_Header(None)),
        (Prop::ClockIdentifier, PropValue::Str(s)) => Some(tp.put_ClockIdentifier(s.as_str())),
        (Prop::MinuteIncrement, PropValue::I32(v)) => Some(tp.put_MinuteIncrement(*v)),
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| tp.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        _ => None,
    }
}

pub(in crate::winui::backend) fn attach_event(
    tp: &Xaml::TimePicker,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::TimeSelected => {
            revokers.push(
                tp.add_SelectedTimeChanged(move |_sender, args| {
                    if let Some(a) = args.as_ref()
                        && let Ok(ts) = a.get_NewTime()
                    {
                        handler.invoke_timespan(windows_time::TimeSpan::from_ticks(ts.Duration));
                    }
                })
                .unwrap(),
            );
        }
        _ => return None,
    }
    Some(revokers)
}
