//! ColorPicker — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler, Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    cp: &Xaml::ColorPicker,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::ColorValue, PropValue::Color { a, r, g, b }) => Some(cp.put_Color(Xaml::Color {
            A: *a,
            R: *r,
            G: *g,
            B: *b,
        })),
        (Prop::IsAlphaEnabled, PropValue::Bool(v)) => Some(cp.put_IsAlphaEnabled(*v)),
        (Prop::IsHexInputVisible, PropValue::Bool(v)) => Some(cp.put_IsHexInputVisible(*v)),
        (Prop::IsColorSliderVisible, PropValue::Bool(v)) => Some(cp.put_IsColorSliderVisible(*v)),
        (Prop::IsColorChannelTextInputVisible, PropValue::Bool(v)) => {
            Some(cp.put_IsColorChannelTextInputVisible(*v))
        }
        _ => None,
    }
}

pub(in crate::winui::backend) fn attach_event(
    cp: &Xaml::ColorPicker,
    event: Event,
    handler: EventHandler,
) -> Option<Vec<windows_core::EventRevoker>> {
    let mut revokers = Vec::new();
    match event {
        Event::ColorChanged => {
            revokers.push(
                cp.add_ColorChanged(move |_sender, args| {
                    let color =
                        args.as_ref()
                            .and_then(|a| a.get_NewColor().ok())
                            .unwrap_or(Xaml::Color {
                                A: 255,
                                R: 0,
                                G: 0,
                                B: 0,
                            });
                    handler.invoke_color((color.A, color.R, color.G, color.B));
                })
                .unwrap(),
            );
        }
        _ => return None,
    }
    Some(revokers)
}
