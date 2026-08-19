use super::mount::mount_element;
use super::reconcile::reconcile_children;
use super::*;
use crate::element::CommandBarItem;
use crate::element::props::{AppBarButtonProps, AppBarToggleButtonProps, CommandBarProps};

pub(super) fn mount_command_bar<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: CommandBarProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let CommandBarProps {
        primary,
        secondary,
        default_label_position,
        framework,
    } = props;
    let id = engine.create_native(NativeKind::CommandBar)?;
    engine.queue_control_update(id, ControlUpdate::CommandBar(default_label_position))?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::CommandBar {
            default_label_position,
            framework,
        },
    ));
    mount_command_section(engine, id, CommandSection::Primary, primary, services)?;
    mount_command_section(engine, id, CommandSection::Secondary, secondary, services)?;
    Ok(id)
}

pub(super) fn mount_app_bar_button<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: AppBarButtonProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::AppBarButton)?;
    engine.queue_control_update(
        id,
        ControlUpdate::AppBarButton(Box::new(app_bar_button_update(&props))),
    )?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::AppBarButton(props)));
    Ok(id)
}

pub(super) fn mount_app_bar_toggle_button<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: AppBarToggleButtonProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::AppBarToggleButton)?;
    engine.queue_control_update(
        id,
        ControlUpdate::AppBarToggleButton(Box::new(app_bar_toggle_button_update(&props))),
    )?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::AppBarToggleButton(props)));
    Ok(id)
}

pub(super) fn mount_app_bar_separator<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::AppBarSeparator)?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::AppBarSeparator));
    Ok(id)
}

pub(super) fn reconcile_command_bar<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: CommandBarProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let CommandBarProps {
        primary,
        secondary,
        default_label_position,
        framework,
    } = props;
    let old_position = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::CommandBar {
            default_label_position,
            ..
        } => *default_label_position,
        _ => unreachable!(),
    };
    if old_position != default_label_position {
        engine.queue_control_update(id, ControlUpdate::CommandBar(default_label_position))?;
    }
    let [primary_section, secondary_section] = *engine.arena.get(id).unwrap().children.as_slice()
    else {
        unreachable!()
    };
    reconcile_command_section(engine, primary_section, primary, services)?;
    reconcile_command_section(engine, secondary_section, secondary, services)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::CommandBar {
            default_label_position,
            framework,
        },
    });
    Ok(())
}

pub(super) fn reconcile_app_bar_button<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: AppBarButtonProps,
) -> Result<(), EngineError> {
    let changed = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::AppBarButton(old) => {
            old.label != props.label || old.enabled != props.enabled || old.icon != props.icon
        }
        _ => unreachable!(),
    };
    if changed {
        engine.queue_control_update(
            id,
            ControlUpdate::AppBarButton(Box::new(app_bar_button_update(&props))),
        )?;
    }
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::AppBarButton(props)));
    Ok(())
}

pub(super) fn reconcile_app_bar_toggle_button<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: AppBarToggleButtonProps,
) -> Result<(), EngineError> {
    let changed = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::AppBarToggleButton(old) => {
            old.label != props.label
                || old.enabled != props.enabled
                || old.checked != props.checked
                || old.icon != props.icon
        }
        _ => unreachable!(),
    };
    if changed {
        engine.queue_control_update(
            id,
            ControlUpdate::AppBarToggleButton(Box::new(app_bar_toggle_button_update(&props))),
        )?;
    }
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::AppBarToggleButton(props)));
    Ok(())
}

pub(super) fn reconcile_app_bar_separator<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
) {
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::AppBarSeparator));
}

pub(super) fn mount_command_section<R: NativeRuntime>(
    engine: &mut Engine<R>,
    parent: NodeId,
    section: CommandSection,
    items: Vec<CommandBarItem>,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let id = engine.create_command_section(section)?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(None, MountedKind::CommandSection));
    for item in items {
        let child = mount_element(engine, item.into_element(), services)?;
        engine.attach(id, child)?;
    }
    engine.attach(parent, id)
}

pub(super) fn reconcile_command_section<R: NativeRuntime>(
    engine: &mut Engine<R>,
    parent: NodeId,
    items: Vec<CommandBarItem>,
    services: &RenderServices,
) -> Result<(), EngineError> {
    reconcile_children(
        engine,
        parent,
        items
            .into_iter()
            .map(CommandBarItem::into_element)
            .collect(),
        services,
    )
}

fn app_bar_button_update(props: &AppBarButtonProps) -> AppBarButtonUpdate {
    AppBarButtonUpdate {
        label: props.label.clone(),
        enabled: props.enabled,
        icon: props.icon.clone(),
    }
}

fn app_bar_toggle_button_update(props: &AppBarToggleButtonProps) -> AppBarToggleButtonUpdate {
    AppBarToggleButtonUpdate {
        label: props.label.clone(),
        enabled: props.enabled,
        checked: props.checked,
        icon: props.icon.clone(),
    }
}
