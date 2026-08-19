use super::*;
use crate::element::props::ImageProps;
use crate::element::{Icon, IconKind};

struct ImageChanges {
    source: bool,
    stretch: bool,
    source_revision: u64,
}

#[cfg(feature = "canvas")]
pub(super) fn mount_canvas_image<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: crate::canvas::CanvasImageProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::CanvasImage)?;
    if let Some(invalidator) = &props.invalidator {
        invalidator.bind(id);
    }
    engine.update_canvas_image(
        id,
        props.draw.clone(),
        canvas_revision(props.invalidator.as_ref()),
        true,
    )?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::CanvasImage(props)));
    Ok(id)
}

#[cfg(feature = "canvas")]
pub(super) fn mount_swap_chain_canvas<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: crate::canvas::SwapChainCanvasProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::SwapChainCanvas)?;
    if let Some(invalidator) = &props.invalidator {
        invalidator.bind(id);
    }
    engine.update_swap_chain_canvas(
        id,
        props.draw.clone(),
        canvas_revision(props.invalidator.as_ref()),
        true,
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::SwapChainCanvas(Box::new(SwapChainCanvasUpdate::Continuous(
            props.continuous,
        ))),
    )?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::SwapChainCanvas(props)));
    Ok(id)
}

#[cfg(feature = "canvas")]
pub(super) fn mount_swap_chain_host<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: Box<crate::canvas::SwapChainHostProps>,
) -> Result<NodeId, EngineError> {
    assert_eq!(props.factory.state_type(), props.layout.state_type());
    assert_eq!(props.factory.state_type(), props.frame.state_type());
    let id = engine.create_native(NativeKind::SwapChainHost)?;
    engine.queue_control_update(
        id,
        ControlUpdate::SwapChainHost(Box::new(SwapChainHostUpdate::Initialize {
            factory: props.factory.clone(),
            layout: props.layout.clone(),
            frame: props.frame.clone(),
            continuous: props.continuous,
        })),
    )?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::SwapChainHost(props)));
    Ok(id)
}

pub(super) fn mount_image<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: ImageProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::Image)?;
    let source_revision = 1;
    engine.queue_control_update(
        id,
        ControlUpdate::Image(Box::new(image_update(&props, source_revision, true))),
    )?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::Image {
            props,
            source_revision,
        },
    ));
    Ok(id)
}

pub(super) fn mount_icon<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    icon: Icon,
) -> Result<NodeId, EngineError> {
    let kind = match icon.kind() {
        IconKind::Symbol(_) => NativeKind::SymbolIcon,
        IconKind::Font { .. } => NativeKind::FontIcon,
        IconKind::Bitmap { .. } => NativeKind::BitmapIcon,
        IconKind::Image(_) => NativeKind::ImageIcon,
        IconKind::Path(_) => NativeKind::PathIcon,
    };
    let id = engine.create_native(kind)?;
    engine.queue_control_update(id, ControlUpdate::Icon(Box::new(icon.clone())))?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(key, MountedKind::Icon(icon)));
    Ok(id)
}

#[cfg(feature = "canvas")]
pub(super) fn reconcile_canvas_image<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: crate::canvas::CanvasImageProps,
) -> Result<(), EngineError> {
    let old_invalidator = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::CanvasImage(old) => old.invalidator.clone(),
        _ => unreachable!(),
    };
    let same_invalidator = rebind_invalidator(id, old_invalidator, &props.invalidator);
    engine.update_canvas_image(
        id,
        props.draw.clone(),
        canvas_revision(props.invalidator.as_ref()),
        !same_invalidator,
    )?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::CanvasImage(props)));
    Ok(())
}

#[cfg(feature = "canvas")]
pub(super) fn reconcile_swap_chain_canvas<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: crate::canvas::SwapChainCanvasProps,
) -> Result<(), EngineError> {
    let old_invalidator = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::SwapChainCanvas(old) => old.invalidator.clone(),
        _ => unreachable!(),
    };
    let same_invalidator = rebind_invalidator(id, old_invalidator, &props.invalidator);
    engine.update_swap_chain_canvas(
        id,
        props.draw.clone(),
        canvas_revision(props.invalidator.as_ref()),
        !same_invalidator,
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::SwapChainCanvas(Box::new(SwapChainCanvasUpdate::Continuous(
            props.continuous,
        ))),
    )?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::SwapChainCanvas(props)));
    Ok(())
}

#[cfg(feature = "canvas")]
pub(super) fn reconcile_swap_chain_host<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: Box<crate::canvas::SwapChainHostProps>,
) -> Result<(), EngineError> {
    assert_eq!(props.factory.state_type(), props.layout.state_type());
    assert_eq!(props.factory.state_type(), props.frame.state_type());
    let MountedKind::SwapChainHost(old) =
        &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind
    else {
        unreachable!()
    };
    assert_eq!(
        old.factory.state_type(),
        props.factory.state_type(),
        "SwapChainHost state type changed without replacing the keyed element"
    );
    if old.layout != props.layout || old.frame != props.frame || old.continuous != props.continuous
    {
        engine.queue_control_update(
            id,
            ControlUpdate::SwapChainHost(Box::new(SwapChainHostUpdate::Props {
                layout: props.layout.clone(),
                frame: props.frame.clone(),
                continuous: props.continuous,
            })),
        )?;
    }
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::SwapChainHost(props)));
    Ok(())
}

pub(super) fn reconcile_image<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: ImageProps,
) -> Result<(), EngineError> {
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::Image {
            props: old,
            source_revision,
        } => ImageChanges {
            source: old.source != props.source,
            stretch: old.stretch != props.stretch,
            source_revision: *source_revision,
        },
        _ => unreachable!(),
    };
    let source_revision = if changes.source {
        changes.source_revision.wrapping_add(1)
    } else {
        changes.source_revision
    };
    if changes.source || changes.stretch {
        engine.queue_control_update(
            id,
            ControlUpdate::Image(Box::new(image_update(
                &props,
                source_revision,
                changes.source,
            ))),
        )?;
    }
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::Image {
            props,
            source_revision,
        },
    ));
    Ok(())
}

pub(super) fn reconcile_icon<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    icon: Icon,
) -> Result<(), EngineError> {
    let changed = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::Icon(old) => old != &icon,
        _ => unreachable!(),
    };
    if changed {
        engine.queue_control_update(id, ControlUpdate::Icon(Box::new(icon.clone())))?;
    }
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(key, MountedKind::Icon(icon)));
    Ok(())
}

fn image_update(props: &ImageProps, source_revision: u64, source_changed: bool) -> ImageUpdate {
    ImageUpdate {
        source: props.source.clone(),
        source_revision,
        source_changed,
        stretch: props.stretch,
    }
}

#[cfg(feature = "canvas")]
fn canvas_revision(invalidator: Option<&crate::canvas::CanvasInvalidator>) -> u64 {
    invalidator.map_or(0, crate::canvas::CanvasInvalidator::revision)
}

#[cfg(feature = "canvas")]
fn rebind_invalidator(
    id: NodeId,
    old: Option<crate::canvas::CanvasInvalidator>,
    new: &Option<crate::canvas::CanvasInvalidator>,
) -> bool {
    let same = match (&old, new) {
        (Some(old), Some(new)) => old.ptr_eq(new),
        (None, None) => true,
        _ => false,
    };
    if !same {
        if let Some(old) = old {
            old.unbind(id);
        }
        if let Some(new) = new {
            new.bind(id);
        }
    }
    same
}
