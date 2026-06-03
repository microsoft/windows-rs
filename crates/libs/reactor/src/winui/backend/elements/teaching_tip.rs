//! TeachingTip — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    tt: &Xaml::TeachingTip,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::TeachingTipTitle, PropValue::Str(s)) => Some(tt.put_Title(s.as_str())),
        (Prop::TeachingTipSubtitle, PropValue::Str(s)) => Some(tt.put_Subtitle(s.as_str())),
        (Prop::TeachingTipIsOpen, PropValue::Bool(v)) => Some(tt.put_IsOpen(*v)),
        (Prop::TeachingTipIsLightDismiss, PropValue::Bool(v)) => {
            Some(tt.put_IsLightDismissEnabled(*v))
        }
        (Prop::TeachingTipPlacement, PropValue::TeachingTipPlacement(p)) => {
            use crate::core::widgets::TeachingTipPlacement as E;
            use Xaml::TeachingTipPlacementMode as W;
            let mapped = match p {
                E::Auto => W::Auto,
                E::Top => W::Top,
                E::Bottom => W::Bottom,
                E::Left => W::Left,
                E::Right => W::Right,
                E::TopRight => W::TopRight,
                E::TopLeft => W::TopLeft,
                E::BottomRight => W::BottomRight,
                E::BottomLeft => W::BottomLeft,
                E::LeftTop => W::LeftTop,
                E::LeftBottom => W::LeftBottom,
                E::RightTop => W::RightTop,
                E::RightBottom => W::RightBottom,
                E::Center => W::Center,
            };
            Some(tt.put_PreferredPlacement(mapped))
        }
        (Prop::TeachingTipActionButton, PropValue::Str(s)) => Some((|| {
            let boxed: windows_core::IInspectable =
                windows_reference::IReference::<windows_core::HSTRING>::from(
                    windows_core::HSTRING::from(s.as_str()),
                )
                .cast()?;
            tt.put_ActionButtonContent(&boxed)
        })()),
        (Prop::TeachingTipCloseButton, PropValue::Str(s)) => Some((|| {
            let boxed: windows_core::IInspectable =
                windows_reference::IReference::<windows_core::HSTRING>::from(
                    windows_core::HSTRING::from(s.as_str()),
                )
                .cast()?;
            tt.put_CloseButtonContent(&boxed)
        })()),
        _ => None,
    }
}
