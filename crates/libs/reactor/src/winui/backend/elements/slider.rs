//! Slider — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    s: &Xaml::Slider,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::NumericValue, PropValue::F64(v)) => {
            Some((|| s.cast::<Xaml::IRangeBase>()?.put_Value(*v))())
        }
        (Prop::Minimum, PropValue::F64(v)) => {
            Some((|| s.cast::<Xaml::IRangeBase>()?.put_Minimum(*v))())
        }
        (Prop::Maximum, PropValue::F64(v)) => {
            Some((|| s.cast::<Xaml::IRangeBase>()?.put_Maximum(*v))())
        }
        (Prop::Step, PropValue::F64(v)) => Some((|| {
            s.put_StepFrequency(*v)?;
            s.cast::<Xaml::IRangeBase>()?.put_SmallChange(*v)
        })()),
        (Prop::Step, PropValue::Unset) => Some((|| {
            s.put_StepFrequency(1.0)?;
            s.cast::<Xaml::IRangeBase>()?.put_SmallChange(1.0)
        })()),
        (Prop::Header, PropValue::Str(sv)) => Some((|| {
            let tb = super::super::string_as_textblock(sv)?;
            s.put_Header(&tb)
        })()),
        (Prop::Header, PropValue::Unset) => Some(s.put_Header(None)),
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| s.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        (Prop::Orientation, PropValue::Vertical(vert)) => Some(s.put_Orientation(if *vert {
            Xaml::Orientation::Vertical
        } else {
            Xaml::Orientation::Horizontal
        })),
        _ => None,
    }
}
