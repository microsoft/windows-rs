//! Button — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    b: &Xaml::Button,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::ButtonContent, PropValue::Str(s)) => Some((|| {
            let cc = b.cast::<Xaml::IContentControl>()?;
            if let Ok(existing) = cc.get_Content()
                && let Ok(panel) = existing.cast::<Xaml::IPanel>()
            {
                let children = panel.get_Children()?;
                if children.Size()? >= 2
                    && let Ok(tb) = children.GetAt(1)?.cast::<Xaml::ITextBlock>()
                {
                    return tb.put_Text(s);
                }
            }
            let tb = super::super::string_as_textblock(s)?;
            cc.put_Content(&tb)
        })()),
        (Prop::ButtonIcon, PropValue::SymbolIcon(sym)) => Some((|| {
            let icon_elem = Xaml::SymbolIcon::CreateInstanceWithSymbol(Xaml::Symbol(sym.to_raw()))?;
            let cc = b.cast::<Xaml::IContentControl>()?;
            if let Ok(existing) = cc.get_Content()
                && let Ok(panel) = existing.cast::<Xaml::IPanel>()
            {
                let children = panel.get_Children()?;
                if children.Size()? >= 2 {
                    children.SetAt(0, &icon_elem.cast::<Xaml::UIElement>()?)?;
                    return Ok(());
                }
            }
            let use_icon_only = if let Ok(existing) = cc.get_Content() {
                existing.cast::<Xaml::ISymbolIcon>().is_ok()
                    || existing
                        .cast::<Xaml::ITextBlock>()
                        .ok()
                        .and_then(|tb| tb.get_Text().ok())
                        .is_some_and(|t| t.is_empty())
            } else {
                true
            };
            if use_icon_only {
                cc.put_Content(&icon_elem)
            } else {
                let panel = Xaml::StackPanel::new()?;
                panel.put_Orientation(Xaml::Orientation::Horizontal)?;
                panel.put_Spacing(8.0)?;
                let children = panel.cast::<Xaml::IPanel>()?.get_Children()?;
                children.Append(&icon_elem.cast::<Xaml::UIElement>()?)?;
                if let Ok(existing) = cc.get_Content()
                    && let Ok(ui) = existing.cast::<Xaml::UIElement>()
                {
                    children.Append(&ui)?;
                }
                cc.put_Content(&panel)
            }
        })()),
        (Prop::ButtonStyleVariant, PropValue::ButtonStyle(style)) => Some((|| {
            use crate::core::widgets::ButtonStyle;

            let fe = b.cast::<Xaml::IFrameworkElement>()?;
            let style_key = match style {
                ButtonStyle::Accent => Some("AccentButtonStyle"),
                ButtonStyle::Subtle => Some("SubtleButtonStyle"),
                ButtonStyle::TextLink => Some("TextBlockButtonStyle"),
                ButtonStyle::Default => None,
            };
            if let Some(key_str) = style_key {
                let resources =
                    Xaml::Application::get_Current().and_then(|app| app.get_Resources())?;
                let key = windows_reference::IReference::from(windows_core::HSTRING::from(key_str));
                let map = resources.cast::<windows_collections::IMap<
                    windows_core::IInspectable,
                    windows_core::IInspectable,
                >>()?;
                if let Ok(style_obj) = map.Lookup(&key)
                    && let Ok(s) = style_obj.cast::<Xaml::Style>()
                {
                    fe.put_Style(&s)?;
                }
            } else {
                fe.put_Style(None)?;
            }
            Ok(())
        })()),
        (Prop::IsEnabled, PropValue::Bool(v)) => {
            Some((|| b.cast::<Xaml::IControl>()?.put_IsEnabled(*v))())
        }
        (Prop::Padding, PropValue::Thickness(t)) => Some((|| {
            b.cast::<Xaml::IControl>()?
                .put_Padding(super::super::to_xaml_thickness(*t))
        })()),
        (Prop::Padding, PropValue::Unset) => Some((|| {
            b.cast::<Xaml::IControl>()?
                .put_Padding(super::super::to_xaml_thickness(
                    crate::core::geometry::Thickness::default(),
                ))
        })()),
        (Prop::Background, PropValue::Brush(br)) => Some((|| {
            b.cast::<Xaml::IControl>()?
                .put_Background(&super::super::brush_of(br)?)
        })()),
        (Prop::Background, PropValue::Unset) => {
            Some((|| b.cast::<Xaml::IControl>()?.put_Background(None))())
        }
        (Prop::Foreground, PropValue::Brush(br)) => Some((|| {
            b.cast::<Xaml::IControl>()?
                .put_Foreground(&super::super::brush_of(br)?)
        })()),
        (Prop::Foreground, PropValue::Unset) => {
            Some((|| b.cast::<Xaml::IControl>()?.put_Foreground(None))())
        }
        (Prop::FlyoutContent, PropValue::Str(s)) => Some((|| {
            let flyout = Xaml::Flyout::new()?;
            let tb = super::super::string_as_textblock(s)?;
            flyout.put_Content(&tb)?;
            b.put_Flyout(&flyout)?;
            Ok(())
        })()),
        (Prop::FlyoutPlacement, PropValue::FlyoutPlacement(p)) => Some((|| {
            if let Ok(fb) = b.get_Flyout() {
                let mode = match p {
                    crate::core::widgets::FlyoutPlacement::Top => Xaml::FlyoutPlacementMode::Top,
                    crate::core::widgets::FlyoutPlacement::Bottom => {
                        Xaml::FlyoutPlacementMode::Bottom
                    }
                    crate::core::widgets::FlyoutPlacement::Left => Xaml::FlyoutPlacementMode::Left,
                    crate::core::widgets::FlyoutPlacement::Right => {
                        Xaml::FlyoutPlacementMode::Right
                    }
                    crate::core::widgets::FlyoutPlacement::Full => Xaml::FlyoutPlacementMode::Full,
                    crate::core::widgets::FlyoutPlacement::TopEdgeAlignedLeft => {
                        Xaml::FlyoutPlacementMode::TopEdgeAlignedLeft
                    }
                    crate::core::widgets::FlyoutPlacement::TopEdgeAlignedRight => {
                        Xaml::FlyoutPlacementMode::TopEdgeAlignedRight
                    }
                    crate::core::widgets::FlyoutPlacement::BottomEdgeAlignedLeft => {
                        Xaml::FlyoutPlacementMode::BottomEdgeAlignedLeft
                    }
                    crate::core::widgets::FlyoutPlacement::BottomEdgeAlignedRight => {
                        Xaml::FlyoutPlacementMode::BottomEdgeAlignedRight
                    }
                    crate::core::widgets::FlyoutPlacement::LeftEdgeAlignedTop => {
                        Xaml::FlyoutPlacementMode::LeftEdgeAlignedTop
                    }
                    crate::core::widgets::FlyoutPlacement::LeftEdgeAlignedBottom => {
                        Xaml::FlyoutPlacementMode::LeftEdgeAlignedBottom
                    }
                    crate::core::widgets::FlyoutPlacement::RightEdgeAlignedTop => {
                        Xaml::FlyoutPlacementMode::RightEdgeAlignedTop
                    }
                    crate::core::widgets::FlyoutPlacement::RightEdgeAlignedBottom => {
                        Xaml::FlyoutPlacementMode::RightEdgeAlignedBottom
                    }
                    crate::core::widgets::FlyoutPlacement::Auto => Xaml::FlyoutPlacementMode::Auto,
                };
                let _ = fb.cast::<Xaml::IFlyoutBase>()?.put_Placement(mode);
            }
            Ok(())
        })()),
        _ => None,
    }
}
