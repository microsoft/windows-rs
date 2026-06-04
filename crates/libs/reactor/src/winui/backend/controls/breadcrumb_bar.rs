//! Typed handler for the `BreadcrumbBar` widget.

use crate::core::widgets::BreadcrumbBar;
use crate::winui::backend::Handle;

pub fn mount(widget: &BreadcrumbBar, handle: &Handle) -> windows_core::Result<bool> {
    let Handle::BreadcrumbBar(bc) = handle else {
        return Ok(false);
    };

    set_items(bc, &widget.items)?;
    Ok(true)
}

pub fn diff(
    old: &BreadcrumbBar,
    new: &BreadcrumbBar,
    handle: &Handle,
) -> windows_core::Result<bool> {
    let Handle::BreadcrumbBar(bc) = handle else {
        return Ok(false);
    };

    if old.items != new.items {
        set_items(bc, &new.items)?;
    }

    Ok(true)
}

fn set_items(bc: &crate::bindings::BreadcrumbBar, items: &[String]) -> windows_core::Result<()> {
    let vec: Vec<Option<windows_core::IInspectable>> = items
        .iter()
        .map(|s| {
            let r = windows_reference::IReference::from(s.as_str());
            Some(r.into())
        })
        .collect();
    let ivec: windows_collections::IVector<windows_core::IInspectable> = vec.into();
    bc.put_ItemsSource(&ivec)
}
