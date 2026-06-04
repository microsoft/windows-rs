//! Typed handler for the `SplitButton` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::SplitButtonWidget;
use crate::winui::backend::Handle;
use windows_core::Interface as _;

pub fn mount(
    widget: &SplitButtonWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::SplitButton(sb) = handle else {
        return Ok(());
    };

    if let Some(s) = &widget.content {
        let insp = windows_reference::IReference::from(s.as_str());
        sb.cast::<Xaml::IContentControl>()?.put_Content(&insp)?;
    }

    ctx.mount_event(
        &widget.on_click,
        Event::SplitButtonClick,
        EventHandler::Click,
    );
    Ok(())
}

pub fn diff(
    old: &SplitButtonWidget,
    new: &SplitButtonWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::SplitButton(sb) = handle else {
        return Ok(());
    };

    if old.content != new.content {
        match &new.content {
            Some(s) => {
                let insp = windows_reference::IReference::from(s.as_str());
                sb.cast::<Xaml::IContentControl>()?.put_Content(&insp)?;
            }
            None => sb.cast::<Xaml::IContentControl>()?.put_Content(None)?,
        }
    }

    ctx.diff_event(
        &old.on_click,
        &new.on_click,
        Event::SplitButtonClick,
        EventHandler::Click,
    );
    Ok(())
}
