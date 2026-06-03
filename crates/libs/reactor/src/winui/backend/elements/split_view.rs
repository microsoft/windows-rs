//! SplitView — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    sv: &Xaml::SplitView,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::SplitViewDisplayMode, PropValue::I32(m)) => {
            Some(sv.put_DisplayMode(Xaml::SplitViewDisplayMode(*m)))
        }
        (Prop::SplitViewIsPaneOpen, PropValue::Bool(v)) => Some(sv.put_IsPaneOpen(*v)),
        (Prop::SplitViewOpenPaneLength, PropValue::F64(v)) => Some(sv.put_OpenPaneLength(*v)),
        (Prop::SplitViewCompactPaneLength, PropValue::F64(v)) => Some(sv.put_CompactPaneLength(*v)),
        _ => None,
    }
}
