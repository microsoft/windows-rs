//! ColorPicker — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};

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
