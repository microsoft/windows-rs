use super::mount::mount_element;
use super::reconcile::{
    reconcile, reconcile_children, render_component, rerender_component_with, validate_sibling_keys,
};
use super::*;
use crate::element::tree::{ElementKind, StructuralSlot};
use crate::hooks::{ComponentMemo, RenderFn};
use crate::references::NativeElementRef;
use crate::resources::ContextProps;
use crate::runtime::FADE_TRANSITION_TIMER_SLOT;
use std::any::TypeId;
use std::time::Duration;
#[cfg(test)]
use std::time::Instant;

pub(super) fn mount_component<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    identity: TypeId,
    render: RenderFn,
    memo: Option<ComponentMemo>,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let id = engine.create_logical()?;
    let mut hooks = Vec::new();
    #[cfg(test)]
    let render_started = Instant::now();
    let child = render_component(id, &render, &mut hooks, true, services);
    #[cfg(test)]
    engine.record_tree_build(render_started.elapsed());
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::Component {
            identity,
            render,
            memo,
            contexts: services.contexts.clone(),
            hooks,
        },
    ));
    let child = mount_element(engine, child, services)?;
    engine.attach(id, child)?;
    Ok(id)
}

pub(super) fn mount_fragment<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    children: Vec<Element>,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    validate_sibling_keys(&children)?;
    let id = engine.create_logical()?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(key, MountedKind::Fragment));
    for child in children {
        let child = mount_element(engine, child, services)?;
        engine.attach(id, child)?;
    }
    Ok(id)
}

pub(super) fn mount_structural_slot<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    slot: StructuralSlot,
    child: Element,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let id = engine.create_structural_slot(slot)?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::StructuralSlot(slot)));
    let child = mount_element(engine, child, services)?;
    engine.attach(id, child)?;
    Ok(id)
}

pub(super) fn mount_context<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: ContextProps,
    child: Element,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let id = engine.create_logical()?;
    let child_services = services.with_context(props.entry.clone());
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::Context(props)));
    let child = mount_element(engine, child, &child_services)?;
    engine.attach(id, child)?;
    Ok(id)
}

pub(super) fn mount_reference<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    reference: NativeElementRef,
    child: Element,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let id = engine.create_logical()?;
    let child = mount_element(engine, child, services)?;
    let target = engine.single_projected_native_root(child).unwrap();
    reference.prepare_mount(target, Some(services.scheduler()));
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::Reference { reference, target },
    ));
    engine.add_reference();
    engine.attach(id, child)?;
    Ok(id)
}

pub(super) fn mount_fade_transition<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    child: Element,
    enter: Option<Duration>,
    exit: Option<Duration>,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let id = engine.create_logical()?;
    let child = mount_element(engine, child, services)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::FadeTransition {
            enter,
            exit,
            revision: 0,
            exiting: false,
        },
    ));
    engine.attach(id, child)?;
    if let Some(duration) = enter
        && let Some(target) = engine.single_projected_native_root(child)
    {
        engine.queue_framework_update(target, FrameworkUpdate::Opacity(Some(0.0)))?;
        engine.fade_to(target, 1.0, duration)?;
    }
    Ok(id)
}

pub(super) fn reconcile_component<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    render: RenderFn,
    memo: Option<ComponentMemo>,
    services: &RenderServices,
) -> Result<(), EngineError> {
    rerender_component_with(engine, id, Some((render, memo)), services)
}

pub(super) fn reconcile_fragment<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    children: Vec<Element>,
    services: &RenderServices,
) -> Result<(), EngineError> {
    reconcile_children(engine, id, children, services)
}

pub(super) fn reconcile_structural_slot<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    slot: StructuralSlot,
    child: Element,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let current = engine.arena.get(id).unwrap().children[0];
    reconcile(engine, current, child, services)?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::StructuralSlot(slot)));
    Ok(())
}

pub(super) fn reconcile_context<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: ContextProps,
    child: Element,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let child_services = services.with_context(props.entry.clone());
    let current = engine.arena.get(id).unwrap().children[0];
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::Context(props),
    });
    reconcile(engine, current, child, &child_services)?;
    Ok(())
}

pub(super) fn reconcile_reference<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    reference: NativeElementRef,
    child: Element,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let current = engine.arena.get(id).unwrap().children[0];
    let (previous, old_target) = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind
    {
        MountedKind::Reference { reference, target } => (reference.clone(), *target),
        _ => unreachable!(),
    };
    let replacement = reconcile(engine, current, child, services)?;
    let target = engine.single_projected_native_root(replacement).unwrap();
    if previous != reference || old_target != target {
        if let Some(cleanup) = previous.clear() {
            engine.retire_cleanup(cleanup);
        }
        reference.prepare_mount(target, Some(services.scheduler()));
    }
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::Reference { reference, target },
    });
    Ok(())
}

pub(super) fn reconcile_fade_transition<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    child: Element,
    enter: Option<Duration>,
    exit: Option<Duration>,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let current = engine.arena.get(id).unwrap().children[0];
    let (revision, exiting, old_exit) =
        match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
            MountedKind::FadeTransition {
                revision,
                exiting,
                exit,
                ..
            } => (*revision, *exiting, *exit),
            _ => unreachable!(),
        };
    let empty = matches!(&child.kind, ElementKind::Fragment { children } if children.is_empty());
    let mut next_revision = revision;
    let mut next_exiting = exiting;
    if empty && (!exiting || exit != old_exit) {
        if exiting {
            engine.stop_timer(id, FADE_TRANSITION_TIMER_SLOT, revision);
        }
        if let Some(duration) = exit
            && let Some(target) = engine.single_projected_native_root(current)
        {
            next_revision = revision.wrapping_add(1);
            next_exiting = true;
            engine.fade_to(target, 0.0, duration)?;
            engine.start_timer(TimerSpec {
                owner: id,
                slot: FADE_TRANSITION_TIMER_SLOT,
                revision: next_revision,
                interval: duration,
                repeating: false,
            })?;
        } else {
            next_exiting = false;
            reconcile(engine, current, child, services)?;
        }
    } else if !empty {
        if exiting {
            engine.stop_timer(id, FADE_TRANSITION_TIMER_SLOT, revision);
            if let Some(target) = engine.single_projected_native_root(current) {
                if let Some(duration) = enter {
                    engine.fade_to(target, 1.0, duration)?;
                } else {
                    engine.queue_framework_update(target, FrameworkUpdate::Opacity(Some(1.0)))?;
                }
            }
            next_exiting = false;
        }
        reconcile(engine, current, child, services)?;
    }
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::FadeTransition {
            enter,
            exit,
            revision: next_revision,
            exiting: next_exiting,
        },
    ));
    Ok(())
}

pub(crate) fn complete_fade_transition<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    revision: u64,
) -> Result<bool, EngineError> {
    let Some(node) = engine.arena.get(id) else {
        return Ok(false);
    };
    let Some(Mounted {
        kind:
            MountedKind::FadeTransition {
                enter,
                exit,
                revision: current_revision,
                exiting: true,
            },
        key,
    }) = &node.mounted
    else {
        return Ok(false);
    };
    if *current_revision != revision {
        return Ok(false);
    }
    let key = *key;
    let enter = *enter;
    let exit = *exit;
    let child = node.children[0];
    engine.stop_timer(id, FADE_TRANSITION_TIMER_SLOT, revision);
    engine.remove_subtree(child)?;
    let empty = engine.create_logical()?;
    engine.arena.get_mut(empty).unwrap().mounted = Some(Mounted::new(None, MountedKind::Fragment));
    engine.attach(id, empty)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::FadeTransition {
            enter,
            exit,
            revision,
            exiting: false,
        },
    ));
    Ok(true)
}
