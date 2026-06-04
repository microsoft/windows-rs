//! Typed handler for the `Expander` widget.

use crate::bindings as Xaml;
use crate::core::widgets::Expander;
use crate::core::widgets::ExpanderHeader;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::string_as_textblock;

pub fn mount(w: &Expander, handle: &Handle) -> windows_core::Result<()> {
    let e = handle.cast_inner::<Xaml::IExpander>()?;
    if let Some(ExpanderHeader::Text(h)) = &w.header {
        e.put_Header(&string_as_textblock(h)?)?;
    }
    if w.is_expanded {
        e.put_IsExpanded(true)?;
    }
    Ok(())
}

pub fn diff(old: &Expander, new: &Expander, handle: &Handle) -> windows_core::Result<()> {
    let e = handle.cast_inner::<Xaml::IExpander>()?;
    let old_text = match &old.header {
        Some(ExpanderHeader::Text(s)) => Some(s.as_str()),
        _ => None,
    };
    let new_text = match &new.header {
        Some(ExpanderHeader::Text(s)) => Some(s.as_str()),
        _ => None,
    };
    if new_text != old_text {
        match new_text {
            Some(s) => e.put_Header(&string_as_textblock(s)?)?,
            None => e.put_Header(None)?,
        }
    }
    if new.is_expanded != old.is_expanded {
        e.put_IsExpanded(new.is_expanded)?;
    }
    Ok(())
}
