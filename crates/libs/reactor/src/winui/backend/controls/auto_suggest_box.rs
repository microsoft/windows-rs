//! Typed handler for the `AutoSuggestBox` widget.

use crate::bindings as Xaml;
use crate::core::widgets::AutoSuggestBoxWidget;
use crate::winui::backend::Handle;
use windows_core::Interface as _;

pub fn mount(widget: &AutoSuggestBoxWidget, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::AutoSuggestBox(asb) = handle else {
        return Ok(false);
    };

    if !widget.text.is_empty() {
        asb.put_Text(&widget.text)?;
    }

    set_items(asb, &widget.items)?;

    if let Some(ph) = &widget.placeholder {
        asb.put_PlaceholderText(ph)?;
    }
    if let Some(hd) = &widget.header {
        let insp = windows_reference::IReference::from(hd.as_str());
        asb.put_Header(&insp)?;
    }

    Ok(true)
}

pub fn diff(
    old: &AutoSuggestBoxWidget,
    new: &AutoSuggestBoxWidget,
    handle: &Handle,
) -> windows_core::Result<bool> {
    let Handle::AutoSuggestBox(asb) = handle else {
        return Ok(false);
    };

    if old.text != new.text {
        // Skip SetText when the control already has this value —
        // calling SetText during a user-initiated TextChanged
        // cycle steals focus from the input field.
        if asb.get_Text().ok().as_deref() != Some(new.text.as_str()) {
            asb.put_Text(&new.text)?;
        }
    }

    if old.items != new.items {
        set_items(asb, &new.items)?;
    }

    if old.placeholder != new.placeholder {
        match &new.placeholder {
            Some(ph) => asb.put_PlaceholderText(ph)?,
            None => asb.put_PlaceholderText("")?,
        }
    }

    if old.header != new.header {
        match &new.header {
            Some(hd) => {
                let insp = windows_reference::IReference::from(hd.as_str());
                asb.put_Header(&insp)?;
            }
            None => asb.put_Header(None)?,
        }
    }

    Ok(true)
}

fn set_items(asb: &Xaml::AutoSuggestBox, items: &[String]) -> windows_core::Result<()> {
    let vec: Vec<Option<windows_core::IInspectable>> = items
        .iter()
        .map(|s| {
            let r = windows_reference::IReference::from(s.as_str());
            Some(r.into())
        })
        .collect();
    let ivec: windows_collections::IVector<windows_core::IInspectable> = vec.into();
    asb.cast::<Xaml::IItemsControl>()?.put_ItemsSource(&ivec)
}
