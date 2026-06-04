//! Typed handler for the `PasswordBox` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::PasswordBox;
use crate::core::widgets::PasswordRevealMode;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;

pub fn mount(w: &PasswordBox, handle: &Handle, ctx: &mut EventCtx) -> windows_core::Result<()> {
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
    ctx.mount_event(
        &w.on_changed,
        Event::PasswordChanged,
        EventHandler::TextChanged,
    );
    Ok(())
}

pub fn diff(
    old: &PasswordBox,
    new: &PasswordBox,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let p = handle.cast_inner::<Xaml::IPasswordBox>()?;
    if new.value != old.value && p.get_Password().ok().as_deref() != Some(new.value.as_str()) {
        p.put_Password(new.value.as_str())?;
    }
    super::diff_opt!(
        old,
        new,
        placeholder,
        |s| p.put_PlaceholderText(s.as_str()),
        p.put_PlaceholderText("")
    );
    super::diff_opt!(
        old,
        new,
        header,
        |s| p.put_Header(&string_as_textblock(s)?),
        p.put_Header(None)
    );
    super::diff_val!(
        old,
        new,
        reveal_mode,
        p.put_PasswordRevealMode(map_reveal_mode(new.reveal_mode))
    );
    super::diff_val!(
        old,
        new,
        is_password_reveal_button_enabled,
        p.put_IsPasswordRevealButtonEnabled(new.is_password_reveal_button_enabled)
    );
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
    ctx.diff_event(
        &old.on_changed,
        &new.on_changed,
        Event::PasswordChanged,
        EventHandler::TextChanged,
    );
    Ok(())
}

fn map_reveal_mode(m: PasswordRevealMode) -> Xaml::PasswordRevealMode {
    match m {
        PasswordRevealMode::Peek => Xaml::PasswordRevealMode::Peek,
        PasswordRevealMode::Hidden => Xaml::PasswordRevealMode::Hidden,
        PasswordRevealMode::Visible => Xaml::PasswordRevealMode::Visible,
    }
}
