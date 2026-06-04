//! Typed handler for the `TeachingTip` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::{TeachingTipPlacement, TeachingTipWidget};
use crate::winui::backend::Handle;
use windows_core::Interface as _;

pub fn mount(
    widget: &TeachingTipWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::TeachingTip(tt) = handle else {
        return Ok(());
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

    ctx.mount_event(
        &widget.on_closed,
        Event::TeachingTipClosed,
        EventHandler::Click,
    );
    ctx.mount_event(
        &widget.on_action_click,
        Event::TeachingTipActionClick,
        EventHandler::Click,
    );
    Ok(())
}

pub fn diff(
    old: &TeachingTipWidget,
    new: &TeachingTipWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::TeachingTip(tt) = handle else {
        return Ok(());
    };

    super::diff_val!(old, new, title, tt.put_Title(&new.title));
    super::diff_opt!(
        old,
        new,
        subtitle,
        |sub| tt.put_Subtitle(sub.as_str()),
        tt.put_Subtitle("")
    );
    super::diff_val!(old, new, is_open, tt.put_IsOpen(new.is_open));
    super::diff_val!(
        old,
        new,
        is_light_dismiss_enabled,
        tt.put_IsLightDismissEnabled(new.is_light_dismiss_enabled)
    );
    super::diff_val!(
        old,
        new,
        preferred_placement,
        set_placement(tt, new.preferred_placement)
    );
    super::diff_opt!(
        old,
        new,
        action_button_text,
        |text| {
            let boxed: windows_core::IInspectable =
                windows_reference::IReference::<windows_core::HSTRING>::from(
                    windows_core::HSTRING::from(text.as_str()),
                )
                .cast()?;
            tt.put_ActionButtonContent(&boxed)
        },
        tt.put_ActionButtonContent(None)
    );
    super::diff_opt!(
        old,
        new,
        close_button_text,
        |text| {
            let boxed: windows_core::IInspectable =
                windows_reference::IReference::<windows_core::HSTRING>::from(
                    windows_core::HSTRING::from(text.as_str()),
                )
                .cast()?;
            tt.put_CloseButtonContent(&boxed)
        },
        tt.put_CloseButtonContent(None)
    );

    ctx.diff_event(
        &old.on_closed,
        &new.on_closed,
        Event::TeachingTipClosed,
        EventHandler::Click,
    );
    ctx.diff_event(
        &old.on_action_click,
        &new.on_action_click,
        Event::TeachingTipActionClick,
        EventHandler::Click,
    );
    Ok(())
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
