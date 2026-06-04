//! ScrollView — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    sv: &Xaml::ScrollView,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::HorizontalScrollBarVisibility, PropValue::ScrollViewScrollBarVis(v)) => {
            use crate::core::widgets::ScrollViewScrollBarVisibility as E;
            use Xaml::ScrollingScrollBarVisibility as W;
            let mapped = match v {
                E::Auto => W::Auto,
                E::Visible => W::Visible,
                E::Hidden => W::Hidden,
            };
            Some(sv.put_HorizontalScrollBarVisibility(mapped))
        }
        (Prop::VerticalScrollBarVisibility, PropValue::ScrollViewScrollBarVis(v)) => {
            use crate::core::widgets::ScrollViewScrollBarVisibility as E;
            use Xaml::ScrollingScrollBarVisibility as W;
            let mapped = match v {
                E::Auto => W::Auto,
                E::Visible => W::Visible,
                E::Hidden => W::Hidden,
            };
            Some(sv.put_VerticalScrollBarVisibility(mapped))
        }
        _ => None,
    }
}
