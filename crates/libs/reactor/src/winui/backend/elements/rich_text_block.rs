//! RichTextBlock — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};

pub(in crate::winui::backend) fn set_prop(
    tb: &Xaml::RichTextBlock,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::IsTextSelectionEnabled, PropValue::Bool(v)) => {
            Some(tb.put_IsTextSelectionEnabled(*v))
        }
        (Prop::IsTextSelectionEnabled, PropValue::Unset) => {
            Some(tb.put_IsTextSelectionEnabled(false))
        }
        (Prop::TextWrappingWrap, PropValue::Bool(v)) => {
            let mode = if *v {
                Xaml::TextWrapping::Wrap
            } else {
                Xaml::TextWrapping::NoWrap
            };
            Some(tb.put_TextWrapping(mode))
        }
        (Prop::Background, PropValue::Brush(_)) | (Prop::Background, PropValue::Unset) => {
            Some(Ok(()))
        }
        (Prop::Foreground, PropValue::Brush(br)) => {
            Some((|| tb.put_Foreground(&super::super::brush_of(br)?))())
        }
        (Prop::Foreground, PropValue::Unset) => Some(tb.put_Foreground(None)),
        _ => None,
    }
}
