use super::mount::mount_element;
use super::reconcile::reconcile;
use super::*;
use crate::element::FlyoutPlacement;
use crate::element::props::{
    AttachedPlacement, ButtonProps, CheckBoxProps, FlyoutProps, HyperlinkButtonProps,
    RadioButtonProps, RepeatButtonProps, SplitButtonProps, ToggleButtonProps,
};
use crate::element::tree::{
    BorderElement, ButtonFlyoutElement, DropDownButtonElement, DropDownFlyoutElement,
};

struct BorderChanges {
    background: bool,
    border_brush: bool,
    border_thickness: bool,
    corner_radius: bool,
    padding: bool,
}

struct RadioButtonChanges {
    checked: bool,
    group_name: bool,
}

struct RepeatButtonChanges {
    delay: bool,
    interval: bool,
}

pub(super) fn mount_attached_child<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    placement: AttachedPlacement,
    child: Element,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let id = engine.create_logical()?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::AttachedChild(placement)));
    mount_child(engine, id, child, services)?;
    let target = engine.attached_target(id)?;
    engine.set_attached_placement(target, placement.default_for(), placement)?;
    Ok(id)
}

pub(super) fn mount_border<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    border: BorderElement,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let BorderElement { child, props } = border;
    let id = engine.create_native(NativeKind::Border)?;
    if props.background.is_some() {
        engine.queue_control_update(
            id,
            ControlUpdate::Border(Box::new(BorderUpdate::Background(props.background.clone()))),
        )?;
    }
    if props.border_brush.is_some() {
        engine.queue_control_update(
            id,
            ControlUpdate::Border(Box::new(BorderUpdate::BorderBrush(
                props.border_brush.clone(),
            ))),
        )?;
    }
    if props.border_thickness.is_some() {
        engine.queue_control_update(
            id,
            ControlUpdate::Border(Box::new(BorderUpdate::BorderThickness(
                props.border_thickness,
            ))),
        )?;
    }
    if props.corner_radius.is_some() {
        engine.queue_control_update(
            id,
            ControlUpdate::Border(Box::new(BorderUpdate::CornerRadius(props.corner_radius))),
        )?;
    }
    if props.padding.is_some() {
        engine.queue_control_update(
            id,
            ControlUpdate::Border(Box::new(BorderUpdate::Padding(props.padding))),
        )?;
    }
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(key, MountedKind::Border(props)));
    mount_child(engine, id, *child, services)?;
    Ok(id)
}

pub(super) fn mount_button<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    child: Element,
    props: ButtonProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::Button)?;
    if props.emphasis != Default::default() {
        engine.queue_control_update(id, ControlUpdate::ButtonEmphasis(props.emphasis))?;
    }
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(key, MountedKind::Button(props)));
    mount_child(engine, id, child, services)?;
    Ok(id)
}

pub(super) fn mount_button_flyout<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    button: ButtonProps,
    content: ButtonFlyoutElement,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let ButtonFlyoutElement {
        label,
        flyout,
        flyout_props,
    } = content;
    let owner = engine.create_native(NativeKind::Button)?;
    if button.emphasis != Default::default() {
        engine.queue_control_update(owner, ControlUpdate::ButtonEmphasis(button.emphasis))?;
    }
    mount_child(engine, owner, *label, services)?;
    engine.arena.get_mut(owner).unwrap().mounted = Some(Mounted::new(
        None,
        MountedKind::ButtonEvent(button.on_click.clone()),
    ));
    let accessory = engine.create_native(NativeKind::Flyout)?;
    if flyout_props.placement != FlyoutPlacement::Auto {
        engine.queue_control_update(
            accessory,
            ControlUpdate::FlyoutPlacement(flyout_props.placement),
        )?;
    }
    mount_child(engine, accessory, *flyout, services)?;
    engine.arena.get_mut(accessory).unwrap().mounted =
        Some(Mounted::new(None, MountedKind::Flyout(flyout_props)));
    let id = engine.create_owner_bound(owner, accessory, OwnerRelation::ButtonFlyout, false)?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::ButtonFlyout(button)));
    Ok(id)
}

pub(super) fn mount_drop_down_button<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    drop_down: DropDownButtonElement,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let DropDownButtonElement {
        label,
        flyout,
        props,
    } = drop_down;
    let DropDownFlyoutElement::Content(flyout) = flyout else {
        unreachable!()
    };
    let owner = engine.create_native(NativeKind::DropDownButton)?;
    mount_child(engine, owner, *label, services)?;
    let accessory = engine.create_native(NativeKind::Flyout)?;
    mount_child(engine, accessory, *flyout, services)?;
    engine.arena.get_mut(accessory).unwrap().mounted = Some(Mounted::new(
        None,
        MountedKind::Flyout(FlyoutProps {
            placement: FlyoutPlacement::Auto,
            on_opened: props.on_opened.clone(),
            on_closed: props.on_closed.clone(),
        }),
    ));
    let id = engine.create_owner_bound(owner, accessory, OwnerRelation::ButtonFlyout, false)?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::DropDownButton(props)));
    Ok(id)
}

pub(super) fn mount_hyperlink_button<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    child: Element,
    props: HyperlinkButtonProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::HyperlinkButton)?;
    if let Some(uri) = props.navigate_uri.as_deref() {
        engine.set_hyperlink_button_navigate_uri(id, Some(uri))?;
    }
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::HyperlinkButton(props)));
    mount_child(engine, id, child, services)?;
    Ok(id)
}

pub(super) fn mount_split_button<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    child: Element,
    props: SplitButtonProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::SplitButton)?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::SplitButton(props)));
    mount_child(engine, id, child, services)?;
    Ok(id)
}

pub(super) fn mount_split_button_flyout<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    button: SplitButtonProps,
    content: ButtonFlyoutElement,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let ButtonFlyoutElement {
        label,
        flyout,
        flyout_props,
    } = content;
    let owner = engine.create_native(NativeKind::SplitButton)?;
    mount_child(engine, owner, *label, services)?;
    engine.arena.get_mut(owner).unwrap().mounted = Some(Mounted::new(
        None,
        MountedKind::SplitButtonEvent(button.on_click.clone()),
    ));
    let accessory = engine.create_native(NativeKind::Flyout)?;
    if flyout_props.placement != FlyoutPlacement::Auto {
        engine.queue_control_update(
            accessory,
            ControlUpdate::FlyoutPlacement(flyout_props.placement),
        )?;
    }
    mount_child(engine, accessory, *flyout, services)?;
    engine.arena.get_mut(accessory).unwrap().mounted =
        Some(Mounted::new(None, MountedKind::Flyout(flyout_props)));
    let id = engine.create_owner_bound(owner, accessory, OwnerRelation::ButtonFlyout, false)?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::SplitButtonFlyout(button)));
    Ok(id)
}

pub(super) fn mount_repeat_button<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    child: Element,
    props: RepeatButtonProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::RepeatButton)?;
    engine.queue_control_update(
        id,
        ControlUpdate::RepeatButton(RepeatButtonUpdate::Delay(props.delay)),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::RepeatButton(RepeatButtonUpdate::Interval(props.interval)),
    )?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::RepeatButton(props)));
    mount_child(engine, id, child, services)?;
    Ok(id)
}

pub(super) fn mount_toggle_button<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    child: Element,
    props: ToggleButtonProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::ToggleButton)?;
    engine.queue_control_update(id, ControlUpdate::ToggleChecked(props.checked))?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::ToggleButton(props)));
    mount_child(engine, id, child, services)?;
    Ok(id)
}

pub(super) fn mount_check_box<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    child: Element,
    props: CheckBoxProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::CheckBox)?;
    engine.queue_control_update(id, ControlUpdate::ToggleChecked(props.checked))?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::CheckBox(props)));
    mount_child(engine, id, child, services)?;
    Ok(id)
}

pub(super) fn mount_radio_button<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    child: Element,
    props: RadioButtonProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::RadioButton)?;
    engine.queue_control_update(id, ControlUpdate::ToggleChecked(props.checked))?;
    if props.group_name.is_some() {
        engine.queue_control_update(
            id,
            ControlUpdate::RadioButtonGroupName(props.group_name.clone()),
        )?;
    }
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::RadioButton(props)));
    mount_child(engine, id, child, services)?;
    Ok(id)
}

pub(super) fn reconcile_attached_child<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    placement: AttachedPlacement,
    child: Element,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let MountedKind::AttachedChild(old_placement) =
        engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind
    else {
        unreachable!()
    };
    let old_target = engine.attached_target(id)?;
    reconcile_child(engine, id, child, services)?;
    let target = engine.attached_target(id)?;
    engine.set_attached_placement(
        target,
        if target == old_target {
            old_placement
        } else {
            placement.default_for()
        },
        placement,
    )?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::AttachedChild(placement),
    });
    Ok(())
}

pub(super) fn reconcile_border<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    border: BorderElement,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let BorderElement { child, props } = border;
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::Border(old) => BorderChanges {
            background: old.background != props.background,
            border_brush: old.border_brush != props.border_brush,
            border_thickness: old.border_thickness != props.border_thickness,
            corner_radius: old.corner_radius != props.corner_radius,
            padding: old.padding != props.padding,
        },
        _ => unreachable!(),
    };
    if changes.background {
        engine.queue_control_update(
            id,
            ControlUpdate::Border(Box::new(BorderUpdate::Background(props.background.clone()))),
        )?;
    }
    if changes.border_brush {
        engine.queue_control_update(
            id,
            ControlUpdate::Border(Box::new(BorderUpdate::BorderBrush(
                props.border_brush.clone(),
            ))),
        )?;
    }
    if changes.border_thickness {
        engine.queue_control_update(
            id,
            ControlUpdate::Border(Box::new(BorderUpdate::BorderThickness(
                props.border_thickness,
            ))),
        )?;
    }
    if changes.corner_radius {
        engine.queue_control_update(
            id,
            ControlUpdate::Border(Box::new(BorderUpdate::CornerRadius(props.corner_radius))),
        )?;
    }
    if changes.padding {
        engine.queue_control_update(
            id,
            ControlUpdate::Border(Box::new(BorderUpdate::Padding(props.padding))),
        )?;
    }
    reconcile_child(engine, id, *child, services)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::Border(props),
    });
    Ok(())
}

pub(super) fn reconcile_button<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    child: Element,
    props: ButtonProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let emphasis_changed = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::Button(old) => old.emphasis != props.emphasis,
        _ => unreachable!(),
    };
    if emphasis_changed {
        engine.queue_control_update(id, ControlUpdate::ButtonEmphasis(props.emphasis))?;
    }
    reconcile_child(engine, id, child, services)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::Button(props),
    });
    Ok(())
}

pub(super) fn reconcile_button_flyout<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    button: ButtonProps,
    content: ButtonFlyoutElement,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let ButtonFlyoutElement {
        label,
        flyout,
        flyout_props,
    } = content;
    let [owner, accessory] = engine.arena.get(id).unwrap().children.as_slice() else {
        unreachable!()
    };
    let owner = *owner;
    let accessory = *accessory;
    let (old_emphasis, old_placement) =
        match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
            MountedKind::ButtonFlyout(old) => {
                let placement = match &engine
                    .arena
                    .get(accessory)
                    .unwrap()
                    .mounted
                    .as_ref()
                    .unwrap()
                    .kind
                {
                    MountedKind::Flyout(old) => old.placement,
                    _ => unreachable!(),
                };
                (old.emphasis, placement)
            }
            _ => unreachable!(),
        };
    if old_emphasis != button.emphasis {
        engine.queue_control_update(owner, ControlUpdate::ButtonEmphasis(button.emphasis))?;
    }
    if old_placement != flyout_props.placement {
        engine.queue_control_update(
            accessory,
            ControlUpdate::FlyoutPlacement(flyout_props.placement),
        )?;
    }
    reconcile_child(engine, owner, *label, services)?;
    reconcile_child(engine, accessory, *flyout, services)?;
    engine.arena.get_mut(owner).unwrap().mounted = Some(Mounted::new(
        None,
        MountedKind::ButtonEvent(button.on_click.clone()),
    ));
    engine.arena.get_mut(accessory).unwrap().mounted =
        Some(Mounted::new(None, MountedKind::Flyout(flyout_props)));
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::ButtonFlyout(button)));
    Ok(())
}

pub(super) fn reconcile_drop_down_button<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    drop_down: DropDownButtonElement,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let DropDownButtonElement {
        label,
        flyout,
        props,
    } = drop_down;
    let DropDownFlyoutElement::Content(flyout) = flyout else {
        unreachable!()
    };
    let [owner, accessory] = engine.arena.get(id).unwrap().children.as_slice() else {
        unreachable!()
    };
    let owner = *owner;
    let accessory = *accessory;
    reconcile_child(engine, owner, *label, services)?;
    reconcile_child(engine, accessory, *flyout, services)?;
    engine.arena.get_mut(accessory).unwrap().mounted = Some(Mounted::new(
        None,
        MountedKind::Flyout(FlyoutProps {
            placement: FlyoutPlacement::Auto,
            on_opened: props.on_opened.clone(),
            on_closed: props.on_closed.clone(),
        }),
    ));
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::DropDownButton(props),
    });
    Ok(())
}

pub(super) fn reconcile_hyperlink_button<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    child: Element,
    props: HyperlinkButtonProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let changed = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::HyperlinkButton(old) => old.navigate_uri != props.navigate_uri,
        _ => unreachable!(),
    };
    if changed {
        engine.set_hyperlink_button_navigate_uri(id, props.navigate_uri.as_deref())?;
    }
    reconcile_child(engine, id, child, services)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::HyperlinkButton(props),
    });
    Ok(())
}

pub(super) fn reconcile_split_button<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    child: Element,
    props: SplitButtonProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    reconcile_child(engine, id, child, services)?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::SplitButton(props)));
    Ok(())
}

pub(super) fn reconcile_split_button_flyout<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    button: SplitButtonProps,
    content: ButtonFlyoutElement,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let ButtonFlyoutElement {
        label,
        flyout,
        flyout_props,
    } = content;
    let [owner, accessory] = engine.arena.get(id).unwrap().children.as_slice() else {
        unreachable!()
    };
    let owner = *owner;
    let accessory = *accessory;
    let old_placement = match &engine
        .arena
        .get(accessory)
        .unwrap()
        .mounted
        .as_ref()
        .unwrap()
        .kind
    {
        MountedKind::Flyout(old) => old.placement,
        _ => unreachable!(),
    };
    if old_placement != flyout_props.placement {
        engine.queue_control_update(
            accessory,
            ControlUpdate::FlyoutPlacement(flyout_props.placement),
        )?;
    }
    reconcile_child(engine, owner, *label, services)?;
    reconcile_child(engine, accessory, *flyout, services)?;
    engine.arena.get_mut(owner).unwrap().mounted = Some(Mounted::new(
        None,
        MountedKind::SplitButtonEvent(button.on_click.clone()),
    ));
    engine.arena.get_mut(accessory).unwrap().mounted =
        Some(Mounted::new(None, MountedKind::Flyout(flyout_props)));
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::SplitButtonFlyout(button)));
    Ok(())
}

pub(super) fn reconcile_repeat_button<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    child: Element,
    props: RepeatButtonProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::RepeatButton(old) => RepeatButtonChanges {
            delay: old.delay != props.delay,
            interval: old.interval != props.interval,
        },
        _ => unreachable!(),
    };
    if changes.delay {
        engine.queue_control_update(
            id,
            ControlUpdate::RepeatButton(RepeatButtonUpdate::Delay(props.delay)),
        )?;
    }
    if changes.interval {
        engine.queue_control_update(
            id,
            ControlUpdate::RepeatButton(RepeatButtonUpdate::Interval(props.interval)),
        )?;
    }
    reconcile_child(engine, id, child, services)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::RepeatButton(props),
    });
    Ok(())
}

pub(super) fn reconcile_toggle_button<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    child: Element,
    props: ToggleButtonProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    reconcile_checked(engine, id, props.checked)?;
    reconcile_child(engine, id, child, services)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::ToggleButton(props),
    });
    Ok(())
}

pub(super) fn reconcile_check_box<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    child: Element,
    props: CheckBoxProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    reconcile_checked(engine, id, props.checked)?;
    reconcile_child(engine, id, child, services)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::CheckBox(props),
    });
    Ok(())
}

pub(super) fn reconcile_radio_button<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    child: Element,
    props: RadioButtonProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::RadioButton(old) => RadioButtonChanges {
            checked: old.checked != props.checked,
            group_name: old.group_name != props.group_name,
        },
        _ => unreachable!(),
    };
    if changes.checked {
        engine.queue_control_update(id, ControlUpdate::ToggleChecked(props.checked))?;
    }
    if changes.group_name {
        engine.queue_control_update(
            id,
            ControlUpdate::RadioButtonGroupName(props.group_name.clone()),
        )?;
    }
    reconcile_child(engine, id, child, services)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::RadioButton(props),
    });
    Ok(())
}

fn reconcile_checked<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    checked: bool,
) -> Result<(), EngineError> {
    let old = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::ToggleButton(old) => old.checked,
        MountedKind::CheckBox(old) => old.checked,
        _ => unreachable!(),
    };
    if old != checked {
        engine.queue_control_update(id, ControlUpdate::ToggleChecked(checked))?;
    }
    Ok(())
}

pub(super) fn mount_child<R: NativeRuntime>(
    engine: &mut Engine<R>,
    parent: NodeId,
    child: Element,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let child = mount_element(engine, child, services)?;
    engine.attach(parent, child)
}

pub(super) fn reconcile_child<R: NativeRuntime>(
    engine: &mut Engine<R>,
    parent: NodeId,
    child: Element,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let current = engine.arena.get(parent).unwrap().children[0];
    reconcile(engine, current, child, services)?;
    Ok(())
}
