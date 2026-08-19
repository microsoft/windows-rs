use super::*;

pub(super) fn mount_composition_host<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: crate::composition::CompositionHostProps,
) -> Result<NodeId, EngineError> {
    assert_eq!(
        props.factory.state_type(),
        props.layout.state_type(),
        "CompositionHost factory and layout state types differ"
    );
    let id = engine.create_native(NativeKind::CompositionHost)?;
    engine.queue_control_update(
        id,
        ControlUpdate::CompositionHost(Box::new(CompositionHostUpdate::Initialize {
            factory: props.factory.clone(),
            layout: props.layout.clone(),
        })),
    )?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::CompositionHost(Box::new(props)),
    ));
    Ok(id)
}

pub(super) fn reconcile_composition_host<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: crate::composition::CompositionHostProps,
) -> Result<(), EngineError> {
    let MountedKind::CompositionHost(old) =
        &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind
    else {
        unreachable!()
    };
    assert_eq!(
        old.factory.state_type(),
        props.factory.state_type(),
        "CompositionHost state type changed without replacing the keyed element"
    );
    assert_eq!(
        props.factory.state_type(),
        props.layout.state_type(),
        "CompositionHost factory and layout state types differ"
    );
    if old.layout != props.layout {
        engine.queue_control_update(
            id,
            ControlUpdate::CompositionHost(Box::new(CompositionHostUpdate::LayoutCallback(
                props.layout.clone(),
            ))),
        )?;
    }
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::CompositionHost(Box::new(props)),
    ));
    Ok(())
}

#[cfg(feature = "webview")]
pub(super) fn mount_webview_host<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: Box<crate::webview::WebViewHostProps>,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::WebViewHost)?;
    engine.queue_control_update(
        id,
        ControlUpdate::WebViewHost(WebViewHostUpdate::Source(props.source.clone())),
    )?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::WebViewHost(props)));
    Ok(id)
}

#[cfg(feature = "webview")]
pub(super) fn reconcile_webview_host<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: Box<crate::webview::WebViewHostProps>,
) -> Result<(), EngineError> {
    let MountedKind::WebViewHost(old) =
        &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind
    else {
        unreachable!()
    };
    if old.source != props.source {
        engine.queue_control_update(
            id,
            ControlUpdate::WebViewHost(WebViewHostUpdate::Source(props.source.clone())),
        )?;
    }
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::WebViewHost(props)));
    Ok(())
}
