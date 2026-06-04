//! Viewbox — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    vb: &Xaml::Viewbox,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::ImageStretch, PropValue::ImageStretch(s)) => {
            use crate::core::widgets::ImageStretch as E;
            use Xaml::Stretch as X;
            let mapped = match s {
                E::Uniform => X::Uniform,
                E::UniformToFill => X::UniformToFill,
                E::Fill => X::Fill,
                E::None => X::None,
            };
            Some(vb.put_Stretch(mapped))
        }
        _ => None,
    }
}
