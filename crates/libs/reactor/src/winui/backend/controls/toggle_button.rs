//! Typed handler for the `ToggleButton` widget.

use crate::bindings as Xaml;
use crate::core::widgets::ToggleButtonWidget;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;
use windows_core::Interface as _;

pub fn mount(widget: &ToggleButtonWidget, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::ToggleButton(tb) = handle else {
        return Ok(false);
    };

    tb.cast::<Xaml::IToggleButton>()?
        .put_IsChecked(Some(widget.is_checked))?;

    let label_tb = string_as_textblock(&widget.label)?;
    tb.cast::<Xaml::IContentControl>()?.put_Content(&label_tb)?;

    Ok(true)
}

pub fn diff(
    old: &ToggleButtonWidget,
    new: &ToggleButtonWidget,
    handle: &Handle,
) -> windows_core::Result<bool> {
    let Handle::ToggleButton(tb) = handle else {
        return Ok(false);
    };

    if old.is_checked != new.is_checked {
        tb.cast::<Xaml::IToggleButton>()?
            .put_IsChecked(Some(new.is_checked))?;
    }
    if old.label != new.label {
        let label_tb = string_as_textblock(&new.label)?;
        tb.cast::<Xaml::IContentControl>()?.put_Content(&label_tb)?;
    }

    Ok(true)
}
