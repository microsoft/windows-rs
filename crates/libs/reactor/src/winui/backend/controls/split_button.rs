//! Typed handler for the `SplitButton` widget.

use crate::bindings as Xaml;
use crate::core::widgets::SplitButtonWidget;
use crate::winui::backend::Handle;
use windows_core::Interface as _;

pub fn mount(widget: &SplitButtonWidget, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::SplitButton(sb) = handle else {
        return Ok(false);
    };

    if let Some(s) = &widget.content {
        let insp = windows_reference::IReference::from(s.as_str());
        sb.cast::<Xaml::IContentControl>()?.put_Content(&insp)?;
    }

    Ok(true)
}

pub fn diff(
    old: &SplitButtonWidget,
    new: &SplitButtonWidget,
    handle: &Handle,
) -> windows_core::Result<bool> {
    let Handle::SplitButton(sb) = handle else {
        return Ok(false);
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

    Ok(true)
}
