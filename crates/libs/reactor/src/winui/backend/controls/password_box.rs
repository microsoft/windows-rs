//! Typed handler for the `PasswordBox` widget.

use crate::bindings as Xaml;
use crate::core::widgets::PasswordBox;
use crate::core::widgets::PasswordRevealMode;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;

pub fn mount(w: &PasswordBox, handle: &Handle) -> windows_core::Result<()> {
    let p = handle.cast_inner::<Xaml::IPasswordBox>()?;
    if !w.value.is_empty() {
        p.put_Password(w.value.as_str())?;
    }
    if let Some(ph) = &w.placeholder {
        p.put_PlaceholderText(ph.as_str())?;
    }
    if let Some(hd) = &w.header {
        p.put_Header(&string_as_textblock(hd)?)?;
    }
    if w.reveal_mode != PasswordRevealMode::Peek {
        p.put_PasswordRevealMode(map_reveal_mode(w.reveal_mode))?;
    }
    if !w.is_password_reveal_button_enabled {
        p.put_IsPasswordRevealButtonEnabled(false)?;
    }
    if !w.is_enabled {
        handle
            .cast_inner::<Xaml::IControl>()?
            .put_IsEnabled(false)?;
    }
    Ok(())
}

pub fn diff(old: &PasswordBox, new: &PasswordBox, handle: &Handle) -> windows_core::Result<()> {
    let p = handle.cast_inner::<Xaml::IPasswordBox>()?;
    if new.value != old.value {
        if p.get_Password().ok().as_deref() != Some(new.value.as_str()) {
            p.put_Password(new.value.as_str())?;
        }
    }
    if new.placeholder != old.placeholder {
        match &new.placeholder {
            Some(s) => p.put_PlaceholderText(s.as_str())?,
            None => p.put_PlaceholderText("")?,
        }
    }
    if new.header != old.header {
        match &new.header {
            Some(s) => p.put_Header(&string_as_textblock(s)?)?,
            None => p.put_Header(None)?,
        }
    }
    if new.reveal_mode != old.reveal_mode {
        p.put_PasswordRevealMode(map_reveal_mode(new.reveal_mode))?;
    }
    if new.is_password_reveal_button_enabled != old.is_password_reveal_button_enabled {
        p.put_IsPasswordRevealButtonEnabled(new.is_password_reveal_button_enabled)?;
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

fn map_reveal_mode(m: PasswordRevealMode) -> Xaml::PasswordRevealMode {
    match m {
        PasswordRevealMode::Peek => Xaml::PasswordRevealMode::Peek,
        PasswordRevealMode::Hidden => Xaml::PasswordRevealMode::Hidden,
        PasswordRevealMode::Visible => Xaml::PasswordRevealMode::Visible,
    }
}
