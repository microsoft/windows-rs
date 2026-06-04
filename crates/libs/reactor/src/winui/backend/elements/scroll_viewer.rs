//! ScrollViewer — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    s: &Xaml::ScrollViewer,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::HorizontalScrollBarVisibility, PropValue::ScrollVis(v)) => {
            Some(s.put_HorizontalScrollBarVisibility(super::super::to_xaml_scroll_visibility(*v)))
        }
        (Prop::VerticalScrollBarVisibility, PropValue::ScrollVis(v)) => {
            Some(s.put_VerticalScrollBarVisibility(super::super::to_xaml_scroll_visibility(*v)))
        }
        _ => None,
    }
}
