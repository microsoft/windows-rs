//! Typed handler for the `BreadcrumbBar` widget.

use super::EventCtx;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::BreadcrumbBar;
use crate::winui::backend::Handle;

pub fn mount(
    widget: &BreadcrumbBar,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::BreadcrumbBar(bc) = handle else {
        return Ok(());
    };

    set_items(bc, &widget.items)?;
    ctx.mount_event(
        &widget.on_item_clicked,
        Event::BreadcrumbItemClicked,
        EventHandler::IndexChanged,
    );
    Ok(())
}

pub fn diff(
    old: &BreadcrumbBar,
    new: &BreadcrumbBar,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let Handle::BreadcrumbBar(bc) = handle else {
        return Ok(());
    };

    if old.items != new.items {
        set_items(bc, &new.items)?;
    }

    ctx.diff_event(
        &old.on_item_clicked,
        &new.on_item_clicked,
        Event::BreadcrumbItemClicked,
        EventHandler::IndexChanged,
    );
    Ok(())
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
