//! TabView — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    tv: &Xaml::TabView,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::SelectedIndex, PropValue::I32(v)) => Some(tv.put_SelectedIndex(*v)),
        (Prop::CanReorderTabs, PropValue::Bool(v)) => Some(tv.put_CanReorderTabs(*v)),
        (Prop::IsAddTabButtonVisible, PropValue::Bool(v)) => Some(tv.put_IsAddTabButtonVisible(*v)),
        _ => None,
    }
}
