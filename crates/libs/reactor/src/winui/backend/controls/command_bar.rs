//! Typed handler for the `CommandBar` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::backend::{Event, EventHandler};
use crate::core::widgets::command_bar::{CommandBarLabelPos, CommandBarWidget};
use crate::winui::backend::convert::build_command_bar_element;
use crate::winui::backend::{Handle, wire_command_bar_clicks};

pub fn mount(
    w: &CommandBarWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let cb = handle.cast_inner::<Xaml::ICommandBar>()?;

    // Primary commands
    let primary = cb.get_PrimaryCommands()?;
    for def in &w.primary_commands {
        let el = build_command_bar_element(def)?;
        primary.Append(&el)?;
    }
    // Secondary commands
    let secondary = cb.get_SecondaryCommands()?;
    for def in &w.secondary_commands {
        let el = build_command_bar_element(def)?;
        secondary.Append(&el)?;
    }
    // Default label position
    if w.default_label_position != CommandBarLabelPos::default() {
        cb.put_DefaultLabelPosition(map_label_pos(w.default_label_position))?;
    }
    // Wire click handlers
    if let Some(ref on_click) = w.on_click {
        let handler = EventHandler::TextChanged(on_click.clone());
        let mut revs = wire_command_bar_clicks(&primary, &handler);
        revs.extend(wire_command_bar_clicks(&secondary, &handler));
        if !revs.is_empty() {
            ctx.store_revokers(Event::CommandBarClick, revs);
        }
        ctx.store_menu_handler(handler);
    }
    Ok(())
}

pub fn diff(
    old: &CommandBarWidget,
    new: &CommandBarWidget,
    handle: &Handle,
    ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let cb = handle.cast_inner::<Xaml::ICommandBar>()?;

    let commands_changed = new.primary_commands != old.primary_commands
        || new.secondary_commands != old.secondary_commands
        || new.on_click != old.on_click;

    if commands_changed {
        // Rebuild primary
        let primary = cb.get_PrimaryCommands()?;
        primary.Clear()?;
        for def in &new.primary_commands {
            let el = build_command_bar_element(def)?;
            primary.Append(&el)?;
        }
        // Rebuild secondary
        let secondary = cb.get_SecondaryCommands()?;
        secondary.Clear()?;
        for def in &new.secondary_commands {
            let el = build_command_bar_element(def)?;
            secondary.Append(&el)?;
        }
        // Re-wire click handlers
        if let Some(ref on_click) = new.on_click {
            let handler = EventHandler::TextChanged(on_click.clone());
            let mut revs = wire_command_bar_clicks(&primary, &handler);
            revs.extend(wire_command_bar_clicks(&secondary, &handler));
            if !revs.is_empty() {
                ctx.store_revokers(Event::CommandBarClick, revs);
            }
            ctx.store_menu_handler(handler);
        }
    }
    // Label position
    if new.default_label_position != old.default_label_position {
        cb.put_DefaultLabelPosition(map_label_pos(new.default_label_position))?;
    }
    Ok(())
}

fn map_label_pos(pos: CommandBarLabelPos) -> Xaml::CommandBarDefaultLabelPosition {
    match pos {
        CommandBarLabelPos::Bottom => Xaml::CommandBarDefaultLabelPosition::Bottom,
        CommandBarLabelPos::Right => Xaml::CommandBarDefaultLabelPosition::Right,
        CommandBarLabelPos::Collapsed => Xaml::CommandBarDefaultLabelPosition::Collapsed,
    }
}
