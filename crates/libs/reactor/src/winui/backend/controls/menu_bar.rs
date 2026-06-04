//! Typed handler for the `MenuBar` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::menu_bar::MenuBarWidget;
use crate::winui::backend::convert::build_menu_flyout_item_base;
use crate::winui::backend::{Handle, wire_menu_bar_clicks};

pub fn mount(w: &MenuBarWidget, handle: &Handle, ctx: &mut EventCtx) -> windows_core::Result<()> {
    let mb: Xaml::MenuBar = handle.cast_inner()?;
    let winui_items = mb.get_Items()?;
    for bar_item_def in &w.items {
        let mbi = Xaml::MenuBarItem::new()?;
        mbi.put_Title(&bar_item_def.title)?;
        let flyout_items = mbi.get_Items()?;
        for menu_def in &bar_item_def.items {
            let fi = build_menu_flyout_item_base(menu_def)?;
            flyout_items.Append(&fi)?;
        }
        winui_items.Append(&mbi)?;
    }
    if let Some(ref on_click) = w.on_item_click {
        let handler = EventHandler::TextChanged(on_click.clone());
        let revs = wire_menu_bar_clicks(&mb, &handler);
        if !revs.is_empty() {
            ctx.store_revokers(Event::MenuBarItemClicked, revs);
        }
        ctx.store_menu_handler(handler);
    }
    Ok(())
}

pub fn diff(
    old: &MenuBarWidget,
    new: &MenuBarWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    if new.items != old.items || new.on_item_click != old.on_item_click {
        let mb: Xaml::MenuBar = handle.cast_inner()?;
        let winui_items = mb.get_Items()?;
        winui_items.Clear()?;
        for bar_item_def in &new.items {
            let mbi = Xaml::MenuBarItem::new()?;
            mbi.put_Title(&bar_item_def.title)?;
            let flyout_items = mbi.get_Items()?;
            for menu_def in &bar_item_def.items {
                let fi = build_menu_flyout_item_base(menu_def)?;
                flyout_items.Append(&fi)?;
            }
            winui_items.Append(&mbi)?;
        }
        if let Some(ref on_click) = new.on_item_click {
            let handler = EventHandler::TextChanged(on_click.clone());
            let revs = wire_menu_bar_clicks(&mb, &handler);
            if !revs.is_empty() {
                ctx.store_revokers(Event::MenuBarItemClicked, revs);
            }
            ctx.store_menu_handler(handler);
        }
    }
    Ok(())
}
