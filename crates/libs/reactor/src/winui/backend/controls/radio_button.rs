//! Typed handler for the `RadioButton` widget.

use crate::bindings as Xaml;
use crate::core::widgets::RadioButton;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;

pub fn mount(w: &RadioButton, handle: &Handle) -> windows_core::Result<()> {
    let r = handle.cast_inner::<Xaml::IRadioButton>()?;
    if let Some(label) = &w.label {
        let txt = string_as_textblock(label)?;
        handle
            .cast_inner::<Xaml::IContentControl>()?
            .put_Content(&txt)?;
    }
    handle
        .cast_inner::<Xaml::IToggleButton>()?
        .put_IsChecked(Some(w.is_checked))?;
    if let Some(g) = &w.group_name {
        r.put_GroupName(g.as_str())?;
    }
    if !w.is_enabled {
        handle
            .cast_inner::<Xaml::IControl>()?
            .put_IsEnabled(false)?;
    }
    Ok(())
}

pub fn diff(old: &RadioButton, new: &RadioButton, handle: &Handle) -> windows_core::Result<()> {
    if new.label != old.label {
        let cc = handle.cast_inner::<Xaml::IContentControl>()?;
        match &new.label {
            Some(s) => cc.put_Content(&string_as_textblock(s)?)?,
            None => cc.put_Content(None)?,
        }
    }
    if new.is_checked != old.is_checked {
        handle
            .cast_inner::<Xaml::IToggleButton>()?
            .put_IsChecked(Some(new.is_checked))?;
    }
    if new.group_name != old.group_name {
        let r = handle.cast_inner::<Xaml::IRadioButton>()?;
        match &new.group_name {
            Some(g) => r.put_GroupName(g.as_str())?,
            None => r.put_GroupName("")?,
        }
    }
    if new.is_enabled != old.is_enabled {
        if new.is_enabled {
            handle
                .cast_inner::<Xaml::IDependencyObject>()?
                .ClearValue(&Xaml::Control::get_IsEnabledProperty()?)?;
        } else {
            handle
                .cast_inner::<Xaml::IControl>()?
                .put_IsEnabled(false)?;
        }
    }
    Ok(())
}
