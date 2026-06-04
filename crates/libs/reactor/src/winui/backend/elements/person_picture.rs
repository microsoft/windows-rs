//! PersonPicture — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    p: &Xaml::PersonPicture,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::PersonDisplayName, PropValue::Str(s)) => Some(p.put_DisplayName(s.as_str())),
        (Prop::PersonDisplayName, PropValue::Unset) => Some(p.put_DisplayName("")),
        (Prop::PersonInitials, PropValue::Str(s)) => Some(p.put_Initials(s.as_str())),
        (Prop::PersonInitials, PropValue::Unset) => Some(p.put_Initials("")),
        _ => None,
    }
}
