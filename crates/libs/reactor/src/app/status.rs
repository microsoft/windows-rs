use super::*;

fn set_mounted<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    kind: MountedKind,
) {
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(key, kind));
}

pub(super) fn mount_info_badge<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: InfoBadgeProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::InfoBadge)?;
    engine.queue_control_update(id, ControlUpdate::InfoBadgeValue(props.value))?;
    set_mounted(engine, id, key, MountedKind::InfoBadge(props));
    Ok(id)
}

pub(super) fn reconcile_info_badge<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: InfoBadgeProps,
) -> Result<(), EngineError> {
    let changed = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::InfoBadge(old) => old.value != props.value,
        _ => unreachable!(),
    };
    if changed {
        engine.queue_control_update(id, ControlUpdate::InfoBadgeValue(props.value))?;
    }
    set_mounted(engine, id, key, MountedKind::InfoBadge(props));
    Ok(())
}

fn info_bar_update(props: &InfoBarProps) -> InfoBarUpdate {
    InfoBarUpdate {
        title: props.title.clone(),
        message: props.message.clone(),
        severity: props.severity,
        open: props.open,
        closable: props.closable,
    }
}

pub(super) fn mount_info_bar<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: InfoBarProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::InfoBar)?;
    engine.queue_control_update(
        id,
        ControlUpdate::InfoBar(Box::new(info_bar_update(&props))),
    )?;
    set_mounted(engine, id, key, MountedKind::InfoBar(Box::new(props)));
    Ok(id)
}

pub(super) fn reconcile_info_bar<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: InfoBarProps,
) -> Result<(), EngineError> {
    let changed = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::InfoBar(old) => {
            old.title != props.title
                || old.message != props.message
                || old.severity != props.severity
                || old.open != props.open
                || old.closable != props.closable
        }
        _ => unreachable!(),
    };
    if changed {
        engine.queue_control_update(
            id,
            ControlUpdate::InfoBar(Box::new(info_bar_update(&props))),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::InfoBar(Box::new(props)));
    Ok(())
}

fn person_picture_update(props: &PersonPictureProps) -> PersonPictureUpdate {
    PersonPictureUpdate {
        display_name: props.display_name.clone(),
        initials: props.initials.clone(),
    }
}

pub(super) fn mount_person_picture<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: PersonPictureProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::PersonPicture)?;
    engine.queue_control_update(
        id,
        ControlUpdate::PersonPicture(Box::new(person_picture_update(&props))),
    )?;
    set_mounted(engine, id, key, MountedKind::PersonPicture(Box::new(props)));
    Ok(id)
}

pub(super) fn reconcile_person_picture<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: PersonPictureProps,
) -> Result<(), EngineError> {
    let changed = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::PersonPicture(old) => {
            old.display_name != props.display_name || old.initials != props.initials
        }
        _ => unreachable!(),
    };
    if changed {
        engine.queue_control_update(
            id,
            ControlUpdate::PersonPicture(Box::new(person_picture_update(&props))),
        )?;
    }
    set_mounted(engine, id, key, MountedKind::PersonPicture(Box::new(props)));
    Ok(())
}
