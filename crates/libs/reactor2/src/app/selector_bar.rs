use super::mount::mount_element;
use super::reconcile::reconcile_children;
use super::work::RenderServices;
use crate::element::props::{SelectorBarItemProps, SelectorBarProps};
use crate::engine::{Engine, EngineError};
use crate::id::NodeId;
use crate::mounted::{Mounted, MountedKind};
use crate::runtime::{ControlUpdate, NativeKind, NativeRuntime, SelectorBarItemUpdate};

pub(super) fn mount_selector_bar<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: SelectorBarProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let SelectorBarProps {
        items,
        selected_key,
        on_selection_changed,
        framework,
    } = props;
    let id = engine.create_native(NativeKind::SelectorBar)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::SelectorBar(SelectorBarProps {
            items: Vec::new(),
            selected_key,
            on_selection_changed,
            framework,
        }),
    ));
    for item in items {
        let child = mount_element(engine, item, services)?;
        engine.attach(id, child)?;
    }
    engine.queue_control_update(id, ControlUpdate::SelectorBarSelection(selected_key))?;
    Ok(id)
}

pub(super) fn reconcile_selector_bar<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: SelectorBarProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let SelectorBarProps {
        items,
        selected_key,
        on_selection_changed,
        framework,
    } = props;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::SelectorBar(SelectorBarProps {
            items: Vec::new(),
            selected_key,
            on_selection_changed,
            framework,
        }),
    ));
    reconcile_children(engine, id, items, services)?;
    engine.queue_control_update(id, ControlUpdate::SelectorBarSelection(selected_key))
}

pub(super) fn mount_selector_bar_item<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: SelectorBarItemProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::SelectorBarItem)?;
    engine.queue_control_update(
        id,
        ControlUpdate::SelectorBarItem(SelectorBarItemUpdate::Key(props.item_key)),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::SelectorBarItem(SelectorBarItemUpdate::Text(props.text.clone())),
    )?;
    engine.set_selector_bar_item_icon(id, props.icon.clone())?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::SelectorBarItem(props)));
    Ok(id)
}

pub(super) fn reconcile_selector_bar_item<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: SelectorBarItemProps,
) -> Result<(), EngineError> {
    let (item_key, text_changed, icon_changed) =
        match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
            MountedKind::SelectorBarItem(old) => {
                (old.item_key, old.text != props.text, old.icon != props.icon)
            }
            _ => unreachable!(),
        };
    assert_eq!(item_key, props.item_key, "SelectorBar item key changed");
    if text_changed {
        engine.queue_control_update(
            id,
            ControlUpdate::SelectorBarItem(SelectorBarItemUpdate::Text(props.text.clone())),
        )?;
    }
    if icon_changed {
        engine.set_selector_bar_item_icon(id, props.icon.clone())?;
    }
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::SelectorBarItem(props)));
    Ok(())
}
