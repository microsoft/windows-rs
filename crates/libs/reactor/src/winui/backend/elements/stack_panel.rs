//! StackPanel — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    s: &Xaml::StackPanel,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::Orientation, PropValue::Vertical(vert)) => Some(s.put_Orientation(if *vert {
            Xaml::Orientation::Vertical
        } else {
            Xaml::Orientation::Horizontal
        })),
        (Prop::Spacing, PropValue::F64(v)) => Some(s.put_Spacing(*v)),
        (Prop::Spacing, PropValue::Unset) => Some(s.put_Spacing(0.0)),
        (Prop::Background, PropValue::Brush(br)) => Some((|| {
            s.cast::<Xaml::IPanel>()?
                .put_Background(&super::super::brush_of(br)?)
        })()),
        (Prop::Background, PropValue::Unset) => {
            Some((|| s.cast::<Xaml::IPanel>()?.put_Background(None))())
        }
        _ => None,
    }
}
