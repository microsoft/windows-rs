//! CheckBox — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    c: &Xaml::CheckBox,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| c.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        (Prop::IsChecked, PropValue::Bool(v)) => Some((|| {
            c.cast::<Xaml::IToggleButton>()?.put_IsChecked(Some(*v))
        })()),
        (Prop::IsChecked, PropValue::Unset) => {
            Some((|| c.cast::<Xaml::IToggleButton>()?.put_IsChecked(None))())
        }
        (Prop::CheckBoxLabel, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            c.cast::<Xaml::IContentControl>()?.put_Content(&tb)
        })()),
        (Prop::CheckBoxLabel, PropValue::Unset) => {
            Some((|| c.cast::<Xaml::IContentControl>()?.put_Content(None))())
        }
        _ => None,
    }
}
