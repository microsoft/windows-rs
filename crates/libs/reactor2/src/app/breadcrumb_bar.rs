use crate::element::props::BreadcrumbBarProps;
use crate::engine::{Engine, EngineError};
use crate::id::NodeId;
use crate::mounted::{Mounted, MountedKind};
use crate::runtime::{ControlUpdate, NativeKind, NativeRuntime};

pub(super) fn mount_breadcrumb_bar<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: BreadcrumbBarProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::BreadcrumbBar)?;
    engine.queue_control_update(id, ControlUpdate::BreadcrumbBarItems(props.items.values()))?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::BreadcrumbBar(props)));
    Ok(id)
}

pub(super) fn reconcile_breadcrumb_bar<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: BreadcrumbBarProps,
) -> Result<(), EngineError> {
    let items_changed = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::BreadcrumbBar(old) => old.items != props.items,
        _ => unreachable!(),
    };
    if items_changed {
        engine.queue_control_update(id, ControlUpdate::BreadcrumbBarItems(props.items.values()))?;
    }
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::BreadcrumbBar(props)));
    Ok(())
}
