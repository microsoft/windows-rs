//! RatingControl — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler, Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    r: &Xaml::RatingControl,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::NumericValue, PropValue::F64(v)) => Some(r.put_Value(*v)),
        (Prop::MaxRating, PropValue::I32(v)) => Some(r.put_MaxRating(*v)),
        (Prop::RatingCaption, PropValue::Str(s)) => Some(r.put_Caption(s.as_str())),
        (Prop::RatingCaption, PropValue::Unset) => Some(r.put_Caption("")),
        (Prop::PlaceholderValue, PropValue::F64(v)) => Some(r.put_PlaceholderValue(*v)),
        (Prop::IsReadOnly, PropValue::Bool(v)) => Some(r.put_IsReadOnly(*v)),
        _ => None,
    }
}

pub(in crate::winui::backend) fn attach_event(
    r: &Xaml::RatingControl,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::RatingValueChanged => {
            revokers.push(
                r.add_ValueChanged(move |sender, _args| {
                    let v = sender
                        .as_ref()
                        .and_then(|s| s.cast::<Xaml::RatingControl>().ok())
                        .and_then(|rc| rc.get_Value().ok())
                        .unwrap_or(-1.0);
                    handler.invoke_f64(v);
                })
                .unwrap(),
            );
        }
        _ => return None,
    }
    Some(revokers)
}
