use super::command::{mount_command_section, reconcile_command_section};
use super::content::{mount_child, reconcile_child};
use super::work::RenderServices;
use crate::element::Element;
use crate::element::props::{
    ButtonProps, CommandBarFlyoutProps, DropDownButtonProps, FlyoutProps, MenuBarProps,
    MenuFlyoutProps,
};
use crate::engine::{Engine, EngineError};
use crate::id::NodeId;
use crate::mounted::{Mounted, MountedKind};
use crate::runtime::{CommandSection, ControlUpdate, NativeKind, NativeRuntime, OwnerRelation};

pub(super) fn mount_menu_bar<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: MenuBarProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::MenuBar)?;
    engine.queue_control_update(id, ControlUpdate::MenuBar(props.items.clone()))?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::MenuBar(props)));
    Ok(id)
}

pub(super) fn reconcile_menu_bar<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: MenuBarProps,
) -> Result<(), EngineError> {
    let changed = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::MenuBar(old) => old.items != props.items,
        _ => unreachable!(),
    };
    if changed {
        engine.queue_control_update(id, ControlUpdate::MenuBar(props.items.clone()))?;
    }
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::MenuBar(props)));
    Ok(())
}

pub(super) fn mount_button_menu_flyout<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    button: ButtonProps,
    label: Element,
    flyout: MenuFlyoutProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let owner = engine.create_native(NativeKind::Button)?;
    if button.emphasis != Default::default() {
        engine.queue_control_update(owner, ControlUpdate::ButtonEmphasis(button.emphasis))?;
    }
    mount_child(engine, owner, label, services)?;
    engine.arena.get_mut(owner).unwrap().mounted = Some(Mounted::new(
        None,
        MountedKind::ButtonEvent(button.on_click.clone()),
    ));
    let accessory = mount_menu_flyout(engine, flyout)?;
    let id = engine.create_owner_bound(owner, accessory, OwnerRelation::ButtonFlyout, false)?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::ButtonMenuFlyout(button)));
    Ok(id)
}

pub(super) fn reconcile_button_menu_flyout<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    button: ButtonProps,
    label: Element,
    flyout: MenuFlyoutProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let [owner, accessory] = *engine.arena.get(id).unwrap().children.as_slice() else {
        unreachable!()
    };
    let emphasis_changed = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::ButtonMenuFlyout(old) => old.emphasis != button.emphasis,
        _ => unreachable!(),
    };
    if emphasis_changed {
        engine.queue_control_update(owner, ControlUpdate::ButtonEmphasis(button.emphasis))?;
    }

    reconcile_child(engine, owner, label, services)?;
    reconcile_menu_flyout(engine, accessory, flyout)?;
    engine.arena.get_mut(owner).unwrap().mounted = Some(Mounted::new(
        None,
        MountedKind::ButtonEvent(button.on_click.clone()),
    ));
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::ButtonMenuFlyout(button)));
    Ok(())
}

pub(super) fn mount_drop_down_menu_flyout<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    button: DropDownButtonProps,
    label: Element,
    flyout: MenuFlyoutProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let owner = engine.create_native(NativeKind::DropDownButton)?;
    mount_child(engine, owner, label, services)?;
    let accessory = mount_menu_flyout(engine, flyout)?;
    let id = engine.create_owner_bound(owner, accessory, OwnerRelation::ButtonFlyout, false)?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::DropDownMenuFlyout(button)));
    Ok(id)
}

pub(super) fn reconcile_drop_down_menu_flyout<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    button: DropDownButtonProps,
    label: Element,
    flyout: MenuFlyoutProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let [owner, accessory] = *engine.arena.get(id).unwrap().children.as_slice() else {
        unreachable!()
    };
    reconcile_child(engine, owner, label, services)?;
    reconcile_menu_flyout(engine, accessory, flyout)?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::DropDownMenuFlyout(button)));
    Ok(())
}

fn mount_menu_flyout<R: NativeRuntime>(
    engine: &mut Engine<R>,
    props: MenuFlyoutProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::MenuFlyout)?;
    engine.queue_control_update(id, ControlUpdate::MenuFlyout(props.items.clone()))?;
    if props.placement != Default::default() {
        engine.queue_control_update(id, ControlUpdate::FlyoutPlacement(props.placement))?;
    }
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(None, MountedKind::MenuFlyout(props)));
    Ok(id)
}

fn reconcile_menu_flyout<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    props: MenuFlyoutProps,
) -> Result<(), EngineError> {
    let (items_changed, placement_changed) =
        match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
            MountedKind::MenuFlyout(old) => {
                (old.items != props.items, old.placement != props.placement)
            }

            _ => unreachable!(),
        };
    if items_changed {
        engine.queue_control_update(id, ControlUpdate::MenuFlyout(props.items.clone()))?;
    }
    if placement_changed {
        engine.queue_control_update(id, ControlUpdate::FlyoutPlacement(props.placement))?;
    }
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(None, MountedKind::MenuFlyout(props)));
    Ok(())
}

pub(super) fn mount_button_command_bar_flyout<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    button: ButtonProps,
    label: Element,
    flyout: CommandBarFlyoutProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let owner = engine.create_native(NativeKind::Button)?;
    if button.emphasis != Default::default() {
        engine.queue_control_update(owner, ControlUpdate::ButtonEmphasis(button.emphasis))?;
    }
    mount_child(engine, owner, label, services)?;
    engine.arena.get_mut(owner).unwrap().mounted = Some(Mounted::new(
        None,
        MountedKind::ButtonEvent(button.on_click.clone()),
    ));
    let accessory = mount_command_bar_flyout(engine, flyout, services)?;
    let id = engine.create_owner_bound(owner, accessory, OwnerRelation::ButtonFlyout, false)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::ButtonCommandBarFlyout(button),
    ));
    Ok(id)
}

pub(super) fn reconcile_button_command_bar_flyout<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    button: ButtonProps,
    label: Element,
    flyout: CommandBarFlyoutProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let [owner, accessory] = *engine.arena.get(id).unwrap().children.as_slice() else {
        unreachable!()
    };
    let emphasis_changed = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::ButtonCommandBarFlyout(old) => old.emphasis != button.emphasis,
        _ => unreachable!(),
    };
    if emphasis_changed {
        engine.queue_control_update(owner, ControlUpdate::ButtonEmphasis(button.emphasis))?;
    }
    reconcile_child(engine, owner, label, services)?;
    reconcile_command_bar_flyout(engine, accessory, flyout, services)?;
    engine.arena.get_mut(owner).unwrap().mounted = Some(Mounted::new(
        None,
        MountedKind::ButtonEvent(button.on_click.clone()),
    ));
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::ButtonCommandBarFlyout(button),
    ));
    Ok(())
}

fn mount_command_bar_flyout<R: NativeRuntime>(
    engine: &mut Engine<R>,
    props: CommandBarFlyoutProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let CommandBarFlyoutProps {
        primary,
        secondary,
        placement,
        on_opened,
        on_closed,
    } = props;
    let id = engine.create_native(NativeKind::CommandBarFlyout)?;
    if placement != Default::default() {
        engine.queue_control_update(id, ControlUpdate::FlyoutPlacement(placement))?;
    }
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        None,
        MountedKind::CommandBarFlyout(FlyoutProps {
            placement,
            on_opened,
            on_closed,
        }),
    ));
    mount_command_section(engine, id, CommandSection::Primary, primary, services)?;
    mount_command_section(engine, id, CommandSection::Secondary, secondary, services)?;
    Ok(id)
}

fn reconcile_command_bar_flyout<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    props: CommandBarFlyoutProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let CommandBarFlyoutProps {
        primary,
        secondary,
        placement,
        on_opened,
        on_closed,
    } = props;
    let old_placement = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::CommandBarFlyout(old) => old.placement,
        _ => unreachable!(),
    };
    if old_placement != placement {
        engine.queue_control_update(id, ControlUpdate::FlyoutPlacement(placement))?;
    }
    let [primary_section, secondary_section] = *engine.arena.get(id).unwrap().children.as_slice()
    else {
        unreachable!()
    };
    reconcile_command_section(engine, primary_section, primary, services)?;
    reconcile_command_section(engine, secondary_section, secondary, services)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        None,
        MountedKind::CommandBarFlyout(FlyoutProps {
            placement,
            on_opened,
            on_closed,
        }),
    ));
    Ok(())
}
