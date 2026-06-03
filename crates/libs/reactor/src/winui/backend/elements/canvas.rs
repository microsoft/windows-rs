//! Canvas — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    c: &Xaml::Canvas,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::Background, PropValue::Brush(br)) => Some((|| {
            c.cast::<Xaml::IPanel>()?
                .put_Background(&super::super::brush_of(br)?)
        })()),
        (Prop::Background, PropValue::Unset) => {
            Some((|| c.cast::<Xaml::IPanel>()?.put_Background(None))())
        }
        _ => None,
    }
}
