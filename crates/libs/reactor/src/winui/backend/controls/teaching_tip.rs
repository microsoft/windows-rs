//! Typed handler for the `TeachingTip` widget.

use crate::bindings as Xaml;
use crate::core::widgets::{TeachingTipPlacement, TeachingTipWidget};
use crate::winui::backend::Handle;
use windows_core::Interface as _;

pub fn mount(widget: &TeachingTipWidget, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::TeachingTip(tt) = handle else {
        return Ok(false);
    };

    tt.put_Title(&widget.title)?;

    if let Some(sub) = &widget.subtitle {
        tt.put_Subtitle(sub.as_str())?;
    }

    tt.put_IsOpen(widget.is_open)?;

    if widget.is_light_dismiss_enabled {
        tt.put_IsLightDismissEnabled(true)?;
    }

    set_placement(tt, widget.preferred_placement)?;

    if let Some(text) = &widget.action_button_text {
        let boxed: windows_core::IInspectable =
            windows_reference::IReference::<windows_core::HSTRING>::from(
                windows_core::HSTRING::from(text.as_str()),
            )
            .cast()?;
        tt.put_ActionButtonContent(&boxed)?;
    }
    if let Some(text) = &widget.close_button_text {
        let boxed: windows_core::IInspectable =
            windows_reference::IReference::<windows_core::HSTRING>::from(
                windows_core::HSTRING::from(text.as_str()),
            )
            .cast()?;
        tt.put_CloseButtonContent(&boxed)?;
    }

    Ok(true)
}

pub fn diff(
    old: &TeachingTipWidget,
    new: &TeachingTipWidget,
    handle: &Handle,
) -> windows_core::Result<bool> {
    let Handle::TeachingTip(tt) = handle else {
        return Ok(false);
    };

    if old.title != new.title {
        tt.put_Title(&new.title)?;
    }
    if old.subtitle != new.subtitle {
        match &new.subtitle {
            Some(sub) => tt.put_Subtitle(sub.as_str())?,
            None => tt.put_Subtitle("")?,
        }
    }
    if old.is_open != new.is_open {
        tt.put_IsOpen(new.is_open)?;
    }
    if old.is_light_dismiss_enabled != new.is_light_dismiss_enabled {
        tt.put_IsLightDismissEnabled(new.is_light_dismiss_enabled)?;
    }
    if old.preferred_placement != new.preferred_placement {
        set_placement(tt, new.preferred_placement)?;
    }
    if old.action_button_text != new.action_button_text {
        match &new.action_button_text {
            Some(text) => {
                let boxed: windows_core::IInspectable =
                    windows_reference::IReference::<windows_core::HSTRING>::from(
                        windows_core::HSTRING::from(text.as_str()),
                    )
                    .cast()?;
                tt.put_ActionButtonContent(&boxed)?;
            }
            None => tt.put_ActionButtonContent(None)?,
        }
    }
    if old.close_button_text != new.close_button_text {
        match &new.close_button_text {
            Some(text) => {
                let boxed: windows_core::IInspectable =
                    windows_reference::IReference::<windows_core::HSTRING>::from(
                        windows_core::HSTRING::from(text.as_str()),
                    )
                    .cast()?;
                tt.put_CloseButtonContent(&boxed)?;
            }
            None => tt.put_CloseButtonContent(None)?,
        }
    }

    Ok(true)
}

fn set_placement(
    tt: &Xaml::TeachingTip,
    placement: TeachingTipPlacement,
) -> windows_core::Result<()> {
    use TeachingTipPlacement as E;
    use Xaml::TeachingTipPlacementMode as W;
    let mapped = match placement {
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
    tt.put_PreferredPlacement(mapped)
}
