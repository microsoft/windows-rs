//! CommandBar — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    cb: &Xaml::CommandBar,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::CommandBarDefaultLabelPosition, PropValue::CommandBarLabelPosition(pos)) => {
            use crate::core::widgets::CommandBarLabelPos as E;
            use Xaml::CommandBarDefaultLabelPosition as W;
            let mapped = match pos {
                E::Bottom => W::Bottom,
                E::Right => W::Right,
                E::Collapsed => W::Collapsed,
            };
            Some(cb.put_DefaultLabelPosition(mapped))
        }
        _ => None,
    }
}
