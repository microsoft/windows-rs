use crate::element::props::AutoSuggestBoxProps;
use crate::engine::{Engine, EngineError};
use crate::id::NodeId;
use crate::mounted::{Mounted, MountedKind};
use crate::runtime::{AutoSuggestUpdate, ControlUpdate, NativeKind, NativeRuntime};

pub(super) fn mount_auto_suggest_box<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: AutoSuggestBoxProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::AutoSuggestBox)?;
    engine.queue_control_update(
        id,
        ControlUpdate::AutoSuggestBox(Box::new(AutoSuggestUpdate::Items(props.items.values()))),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::AutoSuggestBox(Box::new(AutoSuggestUpdate::Header(props.header.clone()))),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::AutoSuggestBox(Box::new(AutoSuggestUpdate::Placeholder(
            props.placeholder.clone(),
        ))),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::AutoSuggestBox(Box::new(AutoSuggestUpdate::Text(props.text.clone()))),
    )?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::AutoSuggestBox(props)));
    Ok(id)
}

pub(super) fn reconcile_auto_suggest_box<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: AutoSuggestBoxProps,
) -> Result<(), EngineError> {
    let (items_changed, header_changed, placeholder_changed) =
        match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
            MountedKind::AutoSuggestBox(old) => (
                old.items != props.items,
                old.header != props.header,
                old.placeholder != props.placeholder,
            ),
            _ => unreachable!(),
        };
    if items_changed {
        engine.queue_control_update(
            id,
            ControlUpdate::AutoSuggestBox(Box::new(AutoSuggestUpdate::Items(props.items.values()))),
        )?;
    }
    if header_changed {
        engine.queue_control_update(
            id,
            ControlUpdate::AutoSuggestBox(Box::new(AutoSuggestUpdate::Header(
                props.header.clone(),
            ))),
        )?;
    }
    if placeholder_changed {
        engine.queue_control_update(
            id,
            ControlUpdate::AutoSuggestBox(Box::new(AutoSuggestUpdate::Placeholder(
                props.placeholder.clone(),
            ))),
        )?;
    }
    engine.queue_control_update(
        id,
        ControlUpdate::AutoSuggestBox(Box::new(AutoSuggestUpdate::Text(props.text.clone()))),
    )?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::AutoSuggestBox(props)));
    Ok(())
}
