use super::*;

pub(super) fn mount_shape<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: ShapeProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(props.kind.native_kind())?;
    engine.queue_control_update(id, ControlUpdate::Shape(Box::new(shape_update(&props))))?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::Shape(Box::new(props))));
    Ok(id)
}

pub(super) fn reconcile_shape<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: ShapeProps,
) -> Result<(), EngineError> {
    let changed = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::Shape(old) => {
            old.fill != props.fill
                || old.stroke != props.stroke
                || old.stroke_thickness != props.stroke_thickness
                || old.corner_radius != props.corner_radius
                || old.line != props.line
        }
        _ => unreachable!(),
    };
    if changed {
        engine.queue_control_update(id, ControlUpdate::Shape(Box::new(shape_update(&props))))?;
    }
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::Shape(Box::new(props))));
    Ok(())
}

fn shape_update(props: &ShapeProps) -> ShapeUpdate {
    ShapeUpdate {
        kind: props.kind,
        fill: props.fill.clone(),
        stroke: props.stroke.clone(),
        stroke_thickness: props.stroke_thickness,
        corner_radius: props.corner_radius,
        line: props.line,
    }
}
