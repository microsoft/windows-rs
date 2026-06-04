//! Typed handler for the `TextBox` widget.

use crate::bindings as Xaml;
use crate::core::widgets::TextBox;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;

pub fn mount(w: &TextBox, handle: &Handle) -> windows_core::Result<()> {
    let tb = handle.cast_inner::<Xaml::ITextBox>()?;
    tb.put_Text(w.value.as_str())?;
    if let Some(ph) = &w.placeholder {
        tb.put_PlaceholderText(ph.as_str())?;
    }
    if let Some(hd) = &w.header {
        tb.put_Header(&string_as_textblock(hd)?)?;
    }
    if w.accepts_return {
        tb.put_AcceptsReturn(true)?;
    }
    if w.text_wrapping_wrap {
        tb.put_TextWrapping(Xaml::TextWrapping::Wrap)?;
    }
    if !w.is_enabled {
        handle
            .cast_inner::<Xaml::IControl>()?
            .put_IsEnabled(false)?;
    }
    Ok(())
}

pub fn diff(old: &TextBox, new: &TextBox, handle: &Handle) -> windows_core::Result<()> {
    let tb = handle.cast_inner::<Xaml::ITextBox>()?;

    if new.value != old.value {
        // Avoid resetting caret position if text hasn't actually changed on the native side
        if tb.get_Text().ok().as_deref() != Some(new.value.as_str()) {
            tb.put_Text(new.value.as_str())?;
        }
    }
    if new.placeholder != old.placeholder {
        match &new.placeholder {
            Some(s) => tb.put_PlaceholderText(s.as_str())?,
            None => tb.put_PlaceholderText("")?,
        }
    }
    if new.header != old.header {
        match &new.header {
            Some(s) => tb.put_Header(&string_as_textblock(s)?)?,
            None => tb.put_Header(None)?,
        }
    }
    if new.accepts_return != old.accepts_return {
        tb.put_AcceptsReturn(new.accepts_return)?;
    }
    if new.text_wrapping_wrap != old.text_wrapping_wrap {
        let mode = if new.text_wrapping_wrap {
            Xaml::TextWrapping::Wrap
        } else {
            Xaml::TextWrapping::NoWrap
        };
        tb.put_TextWrapping(mode)?;
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
