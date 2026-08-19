use super::mount::mount_element;
use super::reconcile::reconcile;
use super::*;
use crate::element::TooltipPlacement;
use crate::element::tree::{ContentDialogElement, TeachingTipElement, ToolTipElement};

struct TeachingTipChanges {
    title: Option<String>,
    subtitle: Option<String>,
    open: Option<bool>,
    light_dismiss: Option<bool>,
    action_button: Option<Option<String>>,
    close_button: Option<Option<String>>,
    action_button_click: Option<bool>,
}

pub(super) fn mount_content_dialog<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    dialog: ContentDialogElement,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let ContentDialogElement {
        title,
        content,
        props,
    } = dialog;
    let id = engine.create_owned_native(NativeKind::ContentDialog)?;
    apply_content_dialog_props(engine, id, &props)?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::ContentDialog(props)));
    let title = mount_element(engine, *title, services)?;
    engine.attach(id, title)?;
    let content = mount_element(engine, *content, services)?;
    engine.attach(id, content)?;
    Ok(id)
}

pub(super) fn mount_teaching_tip<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    tip: TeachingTipElement,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let TeachingTipElement { owner, props } = tip;
    let owner = mount_element(engine, *owner, services)?;
    let native = engine.create_native(NativeKind::TeachingTip)?;
    apply_teaching_tip_props(engine, native, &props, false)?;
    let id = engine.create_owner_bound(owner, native, OwnerRelation::TeachingTipTarget, true)?;
    engine.queue_control_update(
        native,
        ControlUpdate::TeachingTip(TeachingTipUpdate::Open(props.open)),
    )?;
    engine.arena.get_mut(native).unwrap().mounted =
        Some(Mounted::new(None, MountedKind::TeachingTip(props)));
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::TeachingTipOwner));
    Ok(id)
}

pub(super) fn mount_tooltip<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    tooltip: ToolTipElement,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let ToolTipElement {
        owner,
        content,
        placement,
    } = tooltip;
    let owner = mount_element(engine, *owner, services)?;
    let tooltip = engine.create_native(NativeKind::ToolTip)?;
    let content = mount_element(engine, *content, services)?;
    engine.attach(tooltip, content)?;
    let id = engine.create_owner_bound(owner, tooltip, OwnerRelation::ToolTip, false)?;
    apply_tooltip_placement(engine, id, placement)?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::ToolTip(placement)));
    Ok(id)
}

pub(super) fn reconcile_content_dialog<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    dialog: ContentDialogElement,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let ContentDialogElement {
        title,
        content,
        props,
    } = dialog;
    reconcile_content_dialog_props(engine, id, &props)?;
    let [current_title, current_content] = *engine.arena.get(id).unwrap().children.as_slice()
    else {
        unreachable!()
    };
    reconcile(engine, current_title, *title, services)?;
    reconcile(engine, current_content, *content, services)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::ContentDialog(props),
    });
    Ok(())
}

pub(super) fn reconcile_teaching_tip<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    tip: TeachingTipElement,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let TeachingTipElement { owner, props } = tip;
    let current_owner = engine.arena.get(id).unwrap().children[0];
    let native = engine.arena.get(id).unwrap().children[1];
    reattach_owner_if_changed(engine, id, current_owner, *owner, services)?;
    reconcile_teaching_tip_props(engine, native, &props)?;
    engine.arena.get_mut(native).unwrap().mounted =
        Some(Mounted::new(None, MountedKind::TeachingTip(props)));
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::TeachingTipOwner,
    });
    Ok(())
}

pub(super) fn reconcile_tooltip<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    tooltip: ToolTipElement,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let ToolTipElement {
        owner,
        content,
        placement,
    } = tooltip;
    let MountedKind::ToolTip(previous) =
        engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind
    else {
        unreachable!()
    };
    let current_owner = engine.arena.get(id).unwrap().children[0];
    let tooltip = engine.arena.get(id).unwrap().children[1];
    let owner_changed = reattach_owner_if_changed(engine, id, current_owner, *owner, services)?;
    let current_content = engine.arena.get(tooltip).unwrap().children[0];
    reconcile(engine, current_content, *content, services)?;
    if owner_changed || previous != placement {
        apply_tooltip_placement(engine, id, placement)?;
    }
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::ToolTip(placement),
    });
    Ok(())
}

fn apply_tooltip_placement<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    placement: Option<TooltipPlacement>,
) -> Result<(), EngineError> {
    let owner = engine.arena.get(id).unwrap().children[0];
    let native = engine
        .single_projected_native_root(owner)
        .ok_or(EngineError::NativeParentRejectsChildren(id))?;
    engine.queue_attached_update(native, AttachedUpdate::TooltipPlacement(placement))
}

fn reattach_owner_if_changed<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    current_owner: NodeId,
    owner: Element,
    services: &RenderServices,
) -> Result<bool, EngineError> {
    let old_owner_root = engine
        .single_projected_native_root(current_owner)
        .ok_or(EngineError::NativeParentRejectsChildren(id))?;
    reconcile(engine, current_owner, owner, services)?;
    let new_owner_root = engine
        .single_projected_native_root(engine.arena.get(id).unwrap().children[0])
        .ok_or(EngineError::NativeParentRejectsChildren(id))?;
    if old_owner_root != new_owner_root {
        engine.reattach_owner_bound(id)?;
    }
    Ok(old_owner_root != new_owner_root)
}

pub(super) fn apply_teaching_tip_props<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    props: &TeachingTipProps,
    include_open: bool,
) -> Result<(), EngineError> {
    engine.queue_control_update(
        id,
        ControlUpdate::TeachingTip(TeachingTipUpdate::Title(props.title.clone())),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::TeachingTip(TeachingTipUpdate::Subtitle(props.subtitle.clone())),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::TeachingTip(TeachingTipUpdate::LightDismiss(props.light_dismiss)),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::TeachingTip(TeachingTipUpdate::ActionButton(props.action_button.clone())),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::TeachingTip(TeachingTipUpdate::CloseButton(props.close_button.clone())),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::TeachingTip(TeachingTipUpdate::ActionButtonClick(
            props.on_action_button_click.is_some(),
        )),
    )?;
    if include_open {
        engine.queue_control_update(
            id,
            ControlUpdate::TeachingTip(TeachingTipUpdate::Open(props.open)),
        )?;
    }
    Ok(())
}

pub(super) fn reconcile_teaching_tip_props<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    props: &TeachingTipProps,
) -> Result<(), EngineError> {
    let changes = {
        let node = engine.arena.get(id).ok_or(EngineError::InvalidNode(id))?;
        let Some(Mounted {
            kind: MountedKind::TeachingTip(old),
            ..
        }) = &node.mounted
        else {
            return Err(EngineError::InvalidNode(id));
        };
        TeachingTipChanges {
            title: (old.title != props.title).then(|| props.title.clone()),
            subtitle: (old.subtitle != props.subtitle).then(|| props.subtitle.clone()),
            open: (old.open != props.open).then_some(props.open),
            light_dismiss: (old.light_dismiss != props.light_dismiss)
                .then_some(props.light_dismiss),
            action_button: (old.action_button != props.action_button)
                .then(|| props.action_button.clone()),
            close_button: (old.close_button != props.close_button)
                .then(|| props.close_button.clone()),
            action_button_click: (old.on_action_button_click.is_some()
                != props.on_action_button_click.is_some())
            .then_some(props.on_action_button_click.is_some()),
        }
    };
    if let Some(value) = changes.title {
        engine.queue_control_update(
            id,
            ControlUpdate::TeachingTip(TeachingTipUpdate::Title(value)),
        )?;
    }
    if let Some(value) = changes.subtitle {
        engine.queue_control_update(
            id,
            ControlUpdate::TeachingTip(TeachingTipUpdate::Subtitle(value)),
        )?;
    }
    if let Some(value) = changes.open {
        engine.queue_control_update(
            id,
            ControlUpdate::TeachingTip(TeachingTipUpdate::Open(value)),
        )?;
    }
    if let Some(value) = changes.light_dismiss {
        engine.queue_control_update(
            id,
            ControlUpdate::TeachingTip(TeachingTipUpdate::LightDismiss(value)),
        )?;
    }
    if let Some(value) = changes.action_button {
        engine.queue_control_update(
            id,
            ControlUpdate::TeachingTip(TeachingTipUpdate::ActionButton(value)),
        )?;
    }
    if let Some(value) = changes.close_button {
        engine.queue_control_update(
            id,
            ControlUpdate::TeachingTip(TeachingTipUpdate::CloseButton(value)),
        )?;
    }
    if let Some(value) = changes.action_button_click {
        engine.queue_control_update(
            id,
            ControlUpdate::TeachingTip(TeachingTipUpdate::ActionButtonClick(value)),
        )?;
    }
    Ok(())
}

pub(super) fn apply_content_dialog_props<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    props: &ContentDialogProps,
) -> Result<(), EngineError> {
    engine.queue_control_update(
        id,
        ControlUpdate::ContentDialog(Box::new(content_dialog_update(props))),
    )
}

pub(super) fn reconcile_content_dialog_props<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    props: &ContentDialogProps,
) -> Result<(), EngineError> {
    let changed = {
        let node = engine.arena.get(id).ok_or(EngineError::InvalidNode(id))?;
        let Some(Mounted {
            kind: MountedKind::ContentDialog(old),
            ..
        }) = &node.mounted
        else {
            return Err(EngineError::InvalidNode(id));
        };
        old.primary_button_text != props.primary_button_text
            || old.secondary_button_text != props.secondary_button_text
            || old.close_button_text != props.close_button_text
            || old.primary_button_enabled != props.primary_button_enabled
            || old.secondary_button_enabled != props.secondary_button_enabled
            || old.open != props.open
    };
    if changed {
        engine.queue_control_update(
            id,
            ControlUpdate::ContentDialog(Box::new(content_dialog_update(props))),
        )?;
    }
    Ok(())
}

fn content_dialog_update(props: &ContentDialogProps) -> ContentDialogUpdate {
    ContentDialogUpdate {
        primary_button_text: props.primary_button_text.clone(),
        secondary_button_text: props.secondary_button_text.clone(),
        close_button_text: props.close_button_text.clone(),
        primary_button_enabled: props.primary_button_enabled,
        secondary_button_enabled: props.secondary_button_enabled,
        open: props.open,
    }
}
