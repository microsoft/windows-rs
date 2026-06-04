//! Typed handler for the `RichEditBox` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::RichEditBoxWidget;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;

pub fn mount(
    widget: &RichEditBoxWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::RichEditBox(reb) = handle else {
        return Ok(());
    };

    if !widget.text.is_empty() {
        let doc = reb.get_Document()?;
        doc.SetText(Xaml::TextSetOptions::None, widget.text.as_str())?;
    }
    if let Some(ph) = &widget.placeholder {
        reb.put_PlaceholderText(ph.as_str())?;
    }
    if let Some(hd) = &widget.header {
        reb.put_Header(&string_as_textblock(hd)?)?;
    }
    if widget.is_read_only {
        reb.put_IsReadOnly(true)?;
    }

    ctx.mount_event(
        &widget.on_changed,
        Event::RichEditBoxTextChanged,
        EventHandler::TextChanged,
    );
    Ok(())
}

pub fn diff(
    old: &RichEditBoxWidget,
    new: &RichEditBoxWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::RichEditBox(reb) = handle else {
        return Ok(());
    };

    if old.text != new.text {
        let doc = reb.get_Document()?;
        let mut current = windows_core::HSTRING::default();
        doc.GetText(Xaml::TextGetOptions::None, &mut current).ok();
        if current != new.text.as_str() {
            doc.SetText(Xaml::TextSetOptions::None, new.text.as_str())?;
        }
    }
    if old.placeholder != new.placeholder {
        match &new.placeholder {
            Some(ph) => reb.put_PlaceholderText(ph.as_str())?,
            None => reb.put_PlaceholderText("")?,
        }
    }
    if old.header != new.header {
        match &new.header {
            Some(hd) => reb.put_Header(&string_as_textblock(hd)?)?,
            None => reb.put_Header(None)?,
        }
    }
    if old.is_read_only != new.is_read_only {
        reb.put_IsReadOnly(new.is_read_only)?;
    }

    ctx.diff_event(
        &old.on_changed,
        &new.on_changed,
        Event::RichEditBoxTextChanged,
        EventHandler::TextChanged,
    );
    Ok(())
}
