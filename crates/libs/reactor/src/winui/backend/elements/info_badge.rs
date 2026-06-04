//! InfoBadge — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    ib: &Xaml::InfoBadge,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::InfoBadgeValue, PropValue::I32(v)) => Some(if *v < 0 {
            ib.put_Value(-1)
        } else {
            ib.put_Value(*v)
        }),
        _ => None,
    }
}
