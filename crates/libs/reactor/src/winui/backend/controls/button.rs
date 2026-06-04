//! Typed handler for the `Button` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::Button;
use crate::core::widgets::button::ButtonStyle;
use crate::core::widgets::flyout::FlyoutPlacement;
use crate::winui::backend::convert::{build_command_bar_element, build_menu_flyout_item_base};
use crate::winui::backend::{Handle, wire_command_bar_clicks, wire_flyout_clicks};
use windows_collections;
use windows_core::Interface;
use windows_reference;

pub fn mount(w: &Button, handle: &Handle, ctx: &mut EventCtx) -> windows_core::Result<()> {
    let cc = handle.cast_inner::<Xaml::IContentControl>()?;

    // Set content: icon+text, icon-only, or text-only
    set_button_content(&cc, &w.label, w.icon)?;

    if !w.is_enabled {
        handle
            .cast_inner::<Xaml::IControl>()?
            .put_IsEnabled(false)?;
    }
    if w.style != ButtonStyle::Default {
        apply_button_style(handle, &w.style)?;
    }
    // Simple flyout
    if let Some(ref fly) = w.flyout {
        mount_simple_flyout(handle, &fly.text, fly.placement)?;
    }
    // Menu flyout
    if let Some(ref items) = w.menu_flyout_items {
        let handler = w
            .on_menu_item_clicked
            .as_ref()
            .map(|cb| EventHandler::TextChanged(cb.clone()));
        mount_menu_flyout(handle, items, handler.as_ref(), ctx)?;
    }
    // Command bar flyout
    if let Some(ref primary) = w.command_bar_flyout_primary {
        let secondary = w.command_bar_flyout_secondary.as_deref().unwrap_or(&[]);
        let handler = w
            .on_command_bar_flyout_click
            .as_ref()
            .map(|cb| EventHandler::TextChanged(cb.clone()));
        mount_command_bar_flyout(handle, primary, secondary, handler.as_ref(), ctx)?;
    }
    // Click event
    ctx.mount_event(&w.on_click, Event::Click, EventHandler::new);
    Ok(())
}

pub fn diff(
    old: &Button,
    new: &Button,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let cc = handle.cast_inner::<Xaml::IContentControl>()?;

    // Label or icon changed — rebuild content
    if new.label != old.label || new.icon != old.icon {
        set_button_content(&cc, &new.label, new.icon)?;
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
    // Style
    if new.style != old.style {
        apply_button_style(handle, &new.style)?;
    }
    // Simple flyout
    if new.flyout != old.flyout {
        match &new.flyout {
            Some(fly) => mount_simple_flyout(handle, &fly.text, fly.placement)?,
            None => handle.cast_inner::<Xaml::IButton>()?.put_Flyout(None)?,
        }
    }
    // Menu flyout
    if new.menu_flyout_items != old.menu_flyout_items
        || new.on_menu_item_clicked != old.on_menu_item_clicked
    {
        match &new.menu_flyout_items {
            Some(items) => {
                let handler = new
                    .on_menu_item_clicked
                    .as_ref()
                    .map(|cb| EventHandler::TextChanged(cb.clone()));
                mount_menu_flyout(handle, items, handler.as_ref(), ctx)?;
            }
            None => {
                handle.cast_inner::<Xaml::IButton>()?.put_Flyout(None)?;
            }
        }
    }
    // Command bar flyout
    if new.command_bar_flyout_primary != old.command_bar_flyout_primary
        || new.command_bar_flyout_secondary != old.command_bar_flyout_secondary
        || new.on_command_bar_flyout_click != old.on_command_bar_flyout_click
    {
        match &new.command_bar_flyout_primary {
            Some(primary) => {
                let secondary = new.command_bar_flyout_secondary.as_deref().unwrap_or(&[]);
                let handler = new
                    .on_command_bar_flyout_click
                    .as_ref()
                    .map(|cb| EventHandler::TextChanged(cb.clone()));
                mount_command_bar_flyout(handle, primary, secondary, handler.as_ref(), ctx)?;
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
        EventHandler::new,
    );
    Ok(())
}

// ── Helpers ──────────────────────────────────────────────────────────────

fn set_button_content(
    cc: &Xaml::IContentControl,
    label: &str,
    icon: Option<crate::core::widgets::SymbolGlyph>,
) -> windows_core::Result<()> {
    if let Some(sym) = icon {
        let icon_elem = Xaml::SymbolIcon::CreateInstanceWithSymbol(Xaml::Symbol(sym.to_raw()))?;
        if label.is_empty() {
            cc.put_Content(&icon_elem)?;
        } else {
            let panel = Xaml::StackPanel::new()?;
            panel.put_Orientation(Xaml::Orientation::Horizontal)?;
            panel.put_Spacing(8.0)?;
            let children = panel.cast::<Xaml::IPanel>()?.get_Children()?;
            children.Append(&icon_elem.cast::<Xaml::UIElement>()?)?;
            let tb = Xaml::TextBlock::new()?;
            tb.put_Text(label)?;
            children.Append(&tb.cast::<Xaml::UIElement>()?)?;
            cc.put_Content(&panel)?;
        }
    } else {
        let tb = Xaml::TextBlock::new()?;
        tb.put_Text(label)?;
        cc.put_Content(&tb)?;
    }
    Ok(())
}

fn apply_button_style(handle: &Handle, style: &ButtonStyle) -> windows_core::Result<()> {
    let fe = handle.cast_inner::<Xaml::IFrameworkElement>()?;
    let style_key = match style {
        ButtonStyle::Accent => Some("AccentButtonStyle"),
        ButtonStyle::Subtle => Some("SubtleButtonStyle"),
        ButtonStyle::TextLink => Some("TextBlockButtonStyle"),
        ButtonStyle::Default => None,
    };
    if let Some(key_str) = style_key {
        let resources = Xaml::Application::get_Current().and_then(|app| app.get_Resources())?;
        let key = windows_reference::IReference::from(windows_core::HSTRING::from(key_str));
        let map = resources.cast::<windows_collections::IMap<
            windows_core::IInspectable,
            windows_core::IInspectable,
        >>()?;
        if let Ok(style_obj) = map.Lookup(&key)
            && let Ok(s) = style_obj.cast::<Xaml::Style>()
        {
            fe.put_Style(&s)?;
        }
    } else {
        fe.put_Style(None)?;
    }
    Ok(())
}

fn mount_simple_flyout(
    handle: &Handle,
    text: &str,
    placement: FlyoutPlacement,
) -> windows_core::Result<()> {
    let flyout = Xaml::Flyout::new()?;
    let tb = Xaml::TextBlock::new()?;
    tb.put_Text(text)?;
    flyout.put_Content(&tb)?;
    if placement != FlyoutPlacement::default() {
        let mode = map_flyout_placement(placement);
        flyout.cast::<Xaml::IFlyoutBase>()?.put_Placement(mode)?;
    }
    handle.cast_inner::<Xaml::IButton>()?.put_Flyout(&flyout)?;
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

fn mount_command_bar_flyout(
    handle: &Handle,
    primary: &[crate::core::widgets::command_bar::CommandBarCommandDef],
    secondary: &[crate::core::widgets::command_bar::CommandBarCommandDef],
    handler: Option<&EventHandler>,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let flyout = Xaml::CommandBarFlyout::new()?;
    let primary_cmds = flyout.get_PrimaryCommands()?;
    let secondary_cmds = flyout.get_SecondaryCommands()?;
    for def in primary {
        let el = build_command_bar_element(def)?;
        primary_cmds.Append(&el)?;
    }
    for def in secondary {
        let el = build_command_bar_element(def)?;
        secondary_cmds.Append(&el)?;
    }
    handle.cast_inner::<Xaml::IButton>()?.put_Flyout(&flyout)?;
    if let Some(h) = handler {
        let mut revs = wire_command_bar_clicks(&primary_cmds, h);
        revs.extend(wire_command_bar_clicks(&secondary_cmds, h));
        if !revs.is_empty() {
            ctx.store_revokers(Event::CommandBarFlyoutClick, revs);
        }
        ctx.store_cbf_handler(h.clone());
    }
    Ok(())
}

fn map_flyout_placement(p: FlyoutPlacement) -> Xaml::FlyoutPlacementMode {
    match p {
        FlyoutPlacement::Top => Xaml::FlyoutPlacementMode::Top,
        FlyoutPlacement::Bottom => Xaml::FlyoutPlacementMode::Bottom,
        FlyoutPlacement::Left => Xaml::FlyoutPlacementMode::Left,
        FlyoutPlacement::Right => Xaml::FlyoutPlacementMode::Right,
        FlyoutPlacement::Full => Xaml::FlyoutPlacementMode::Full,
        FlyoutPlacement::TopEdgeAlignedLeft => Xaml::FlyoutPlacementMode::TopEdgeAlignedLeft,
        FlyoutPlacement::TopEdgeAlignedRight => Xaml::FlyoutPlacementMode::TopEdgeAlignedRight,
        FlyoutPlacement::BottomEdgeAlignedLeft => Xaml::FlyoutPlacementMode::BottomEdgeAlignedLeft,
        FlyoutPlacement::BottomEdgeAlignedRight => {
            Xaml::FlyoutPlacementMode::BottomEdgeAlignedRight
        }
        FlyoutPlacement::LeftEdgeAlignedTop => Xaml::FlyoutPlacementMode::LeftEdgeAlignedTop,
        FlyoutPlacement::LeftEdgeAlignedBottom => Xaml::FlyoutPlacementMode::LeftEdgeAlignedBottom,
        FlyoutPlacement::RightEdgeAlignedTop => Xaml::FlyoutPlacementMode::RightEdgeAlignedTop,
        FlyoutPlacement::RightEdgeAlignedBottom => {
            Xaml::FlyoutPlacementMode::RightEdgeAlignedBottom
        }
        FlyoutPlacement::Auto => Xaml::FlyoutPlacementMode::Auto,
    }
}
