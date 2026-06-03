//! InfoBar — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    ib: &Xaml::InfoBar,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::InfoBarTitle, PropValue::Str(s)) => Some(ib.put_Title(s.as_str())),
        (Prop::InfoBarTitle, PropValue::Unset) => Some(ib.put_Title("")),
        (Prop::InfoBarMessage, PropValue::Str(s)) => Some(ib.put_Message(s.as_str())),
        (Prop::InfoBarMessage, PropValue::Unset) => Some(ib.put_Message("")),
        (Prop::InfoBarSeverity, PropValue::InfoBarSev(v)) => {
            Some(ib.put_Severity(super::super::to_winui_info_bar_severity(*v)))
        }
        (Prop::InfoBarIsOpen, PropValue::Bool(v)) => Some(ib.put_IsOpen(*v)),
        (Prop::IsClosable, PropValue::Bool(v)) => Some(ib.put_IsClosable(*v)),
        _ => None,
    }
}
