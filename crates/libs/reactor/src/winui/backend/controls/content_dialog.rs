//! Typed handler for the `ContentDialog` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::callback::Callback;
use crate::core::widgets::ContentDialog;
use crate::core::widgets::content_dialog::ContentDialogResult;
use crate::winui::backend::Handle;
use windows_reference;

pub fn mount(w: &ContentDialog, handle: &Handle, ctx: &mut EventCtx) -> windows_core::Result<()> {
    let d = handle.cast_inner::<Xaml::IContentDialog>()?;
    if let Some(ref t) = w.title {
        let title = windows_reference::IReference::from(t.as_str());
        d.put_Title(&title)?;
    }
    if let Some(ref c) = w.content {
        let tb = Xaml::TextBlock::new()?;
        tb.put_Text(c.as_str())?;
        handle
            .cast_inner::<Xaml::IContentControl>()?
            .put_Content(&tb)?;
    }
    if let Some(ref s) = w.primary_button_text {
        d.put_PrimaryButtonText(s.as_str())?;
    }
    if let Some(ref s) = w.secondary_button_text {
        d.put_SecondaryButtonText(s.as_str())?;
    }
    if let Some(ref s) = w.close_button_text {
        d.put_CloseButtonText(s.as_str())?;
    }
    d.put_IsPrimaryButtonEnabled(w.is_primary_button_enabled)?;
    d.put_IsSecondaryButtonEnabled(w.is_secondary_button_enabled)?;
    if w.is_open {
        ctx.show_dialog();
    }
    // Closed event
    mount_closed_event(w, ctx);
    Ok(())
}

pub fn diff(
    old: &ContentDialog,
    new: &ContentDialog,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let d = handle.cast_inner::<Xaml::IContentDialog>()?;

    if new.title != old.title {
        if let Some(ref t) = new.title {
            let title = windows_reference::IReference::from(t.as_str());
            d.put_Title(&title)?;
        } else {
            d.put_Title(None)?;
        }
    }
    if new.content != old.content {
        let cc = handle.cast_inner::<Xaml::IContentControl>()?;
        if let Some(ref c) = new.content {
            let tb = Xaml::TextBlock::new()?;
            tb.put_Text(c.as_str())?;
            cc.put_Content(&tb)?;
        } else {
            cc.put_Content(None)?;
        }
    }
    if new.primary_button_text != old.primary_button_text {
        d.put_PrimaryButtonText(new.primary_button_text.as_deref().unwrap_or(""))?;
    }
    if new.secondary_button_text != old.secondary_button_text {
        d.put_SecondaryButtonText(new.secondary_button_text.as_deref().unwrap_or(""))?;
    }
    if new.close_button_text != old.close_button_text {
        d.put_CloseButtonText(new.close_button_text.as_deref().unwrap_or(""))?;
    }
    if new.is_primary_button_enabled != old.is_primary_button_enabled {
        d.put_IsPrimaryButtonEnabled(new.is_primary_button_enabled)?;
    }
    if new.is_secondary_button_enabled != old.is_secondary_button_enabled {
        d.put_IsSecondaryButtonEnabled(new.is_secondary_button_enabled)?;
    }
    if new.is_open != old.is_open {
        if new.is_open {
            ctx.show_dialog();
        } else {
            ctx.hide_dialog();
        }
    }
    // Closed event
    diff_closed_event(old, new, ctx);
    Ok(())
}

fn mount_closed_event(w: &ContentDialog, ctx: &mut EventCtx) {
    if let Some(ref cb) = w.on_closed {
        let handler = wrap_closed_callback(cb);
        ctx.attach(Event::ContentDialogClosed, handler);
    }
}

fn diff_closed_event(old: &ContentDialog, new: &ContentDialog, ctx: &mut EventCtx) {
    if old.on_closed != new.on_closed {
        match &new.on_closed {
            Some(cb) => {
                let handler = wrap_closed_callback(cb);
                ctx.attach(Event::ContentDialogClosed, handler);
            }
            None => {
                ctx.detach(Event::ContentDialogClosed);
            }
        }
    }
}

fn wrap_closed_callback(cb: &Callback<ContentDialogResult>) -> EventHandler {
    let cb = cb.clone();
    EventHandler::IndexChanged(Callback::new(move |i: i32| {
        cb.invoke(ContentDialogResult::from_i32(i));
    }))
}
