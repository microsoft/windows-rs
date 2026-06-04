//! Typed handler for the `DropDownButton` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::DropDownButtonWidget;
use crate::winui::backend::convert::build_menu_flyout_item_base;
use crate::winui::backend::{Handle, wire_flyout_clicks};
use windows_reference;

pub fn mount(
    w: &DropDownButtonWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    if let Some(ref s) = w.content {
        let insp = windows_reference::IReference::from(s.as_str());
        handle
            .cast_inner::<Xaml::IContentControl>()?
            .put_Content(&insp)?;
    }
    if !w.is_enabled {
        handle
            .cast_inner::<Xaml::IControl>()?
            .put_IsEnabled(false)?;
    }
    if let Some(ref items) = w.flyout_items {
        let handler = w
            .on_flyout_item_click
            .as_ref()
            .map(|cb| EventHandler::TextChanged(cb.clone()));
        mount_menu_flyout(handle, items, handler.as_ref(), ctx)?;
    }
    ctx.mount_event(&w.on_click, Event::Click, EventHandler::Click);
    Ok(())
}

pub fn diff(
    old: &DropDownButtonWidget,
    new: &DropDownButtonWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    // Content
    if new.content != old.content {
        let cc = handle.cast_inner::<Xaml::IContentControl>()?;
        if let Some(ref s) = new.content {
            let insp = windows_reference::IReference::from(s.as_str());
            cc.put_Content(&insp)?;
        } else {
            cc.put_Content(None)?;
        }
    }
    // IsEnabled
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
    // Menu flyout
    if new.flyout_items != old.flyout_items || new.on_flyout_item_click != old.on_flyout_item_click
    {
        match &new.flyout_items {
            Some(items) => {
                let handler = new
                    .on_flyout_item_click
                    .as_ref()
                    .map(|cb| EventHandler::TextChanged(cb.clone()));
                mount_menu_flyout(handle, items, handler.as_ref(), ctx)?;
            }
            None => {
                handle.cast_inner::<Xaml::IButton>()?.put_Flyout(None)?;
            }
        }
    }
    // Click event
    ctx.diff_event(
        &old.on_click,
        &new.on_click,
        Event::Click,
        EventHandler::Click,
    );
    Ok(())
}

fn mount_menu_flyout(
    handle: &Handle,
    items: &[crate::core::widgets::menu_bar::MenuItemDef],
    handler: Option<&EventHandler>,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let flyout = Xaml::MenuFlyout::new()?;
    let flyout_items = flyout.get_Items()?;
    for def in items {
        let fi = build_menu_flyout_item_base(def)?;
        flyout_items.Append(&fi)?;
    }
    handle.cast_inner::<Xaml::IButton>()?.put_Flyout(&flyout)?;
    if let Some(h) = handler {
        let revs = wire_flyout_clicks(&flyout, h);
        if !revs.is_empty() {
            ctx.store_revokers(Event::MenuFlyoutItemClicked, revs);
        }
        ctx.store_menu_handler(h.clone());
    }
    Ok(())
}
