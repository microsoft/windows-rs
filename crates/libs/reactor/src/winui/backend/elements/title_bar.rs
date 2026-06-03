//! TitleBar — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    tb: &Xaml::TitleBar,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::TitleBarTitle, PropValue::Str(s)) => Some(tb.put_Title(s.as_str())),
        (Prop::TitleBarSubtitle, PropValue::Str(s)) => Some(tb.put_Subtitle(s.as_str())),
        (Prop::TitleBarSubtitle, PropValue::Unset) => Some(tb.put_Subtitle("")),
        (Prop::TitleBarTall, PropValue::Bool(v)) => {
            crate::winui::host::set_titlebar_height(*v);
            Some(Ok(()))
        }
        (Prop::IsBackButtonVisible, PropValue::Bool(v)) => Some(tb.put_IsBackButtonVisible(*v)),
        (Prop::IsBackEnabled, PropValue::Bool(v)) => Some(tb.put_IsBackButtonEnabled(*v)),
        (Prop::IsPaneToggleButtonVisible, PropValue::Bool(v)) => {
            Some(tb.put_IsPaneToggleButtonVisible(*v))
        }
        _ => None,
    }
}
