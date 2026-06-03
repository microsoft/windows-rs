//! NumberBox — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler, Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    n: &Xaml::NumberBox,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::NumericValue, PropValue::F64(v)) => Some(n.put_Value(*v)),
        (Prop::Minimum, PropValue::F64(v)) => Some(n.put_Minimum(*v)),
        (Prop::Maximum, PropValue::F64(v)) => Some(n.put_Maximum(*v)),
        (Prop::Header, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            n.put_Header(&tb)
        })()),
        (Prop::Header, PropValue::Unset) => Some(n.put_Header(None)),
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| n.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        _ => None,
    }
}

pub(in crate::winui::backend) fn attach_event(
    n: &Xaml::NumberBox,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::ValueChanged => {
            revokers.push(
                n.add_ValueChanged(move |_sender, args| {
                    if let Some(a) = args.as_ref()
                        && let Ok(v) = a.get_NewValue()
                    {
                        handler.invoke_f64(v);
                    }
                })
                .unwrap(),
            );
        }
        _ => return None,
    }
    Some(revokers)
}
