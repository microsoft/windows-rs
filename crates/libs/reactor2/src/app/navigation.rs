use super::mount::mount_element;
use super::reconcile::reconcile_children;
use super::*;
use crate::element::props::{NavigationViewItemProps, NavigationViewProps};
use crate::element::tree::NavigationViewElement;

pub(super) fn mount_navigation_view<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    value: NavigationViewElement,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let NavigationViewElement {
        items,
        content,
        footer,
        props,
    } = value;
    let id = engine.create_native(NativeKind::NavigationView)?;
    engine.queue_control_update(
        id,
        ControlUpdate::NavigationView(NavigationUpdate::Properties(Box::new(
            navigation_view_update(&props),
        ))),
    )?;
    mount_navigation_section(engine, id, NavigationSection::Menu, items, services)?;
    mount_navigation_section(
        engine,
        id,
        NavigationSection::Content,
        vec![*content],
        services,
    )?;
    mount_navigation_section(
        engine,
        id,
        NavigationSection::Footer,
        footer.into_iter().map(|value| *value).collect(),
        services,
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::NavigationView(NavigationUpdate::Selection(props.selected_key)),
    )?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::NavigationView(Box::new(props)),
    ));
    Ok(id)
}

pub(super) fn reconcile_navigation_view<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    value: NavigationViewElement,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let NavigationViewElement {
        items,
        content,
        footer,
        props,
    } = value;
    let (configuration_changed, selected_changed) =
        match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
            MountedKind::NavigationView(old) => (
                navigation_view_update(old) != navigation_view_update(&props),
                old.selected_key != props.selected_key,
            ),
            _ => unreachable!(),
        };
    if configuration_changed {
        engine.queue_control_update(
            id,
            ControlUpdate::NavigationView(NavigationUpdate::Properties(Box::new(
                navigation_view_update(&props),
            ))),
        )?;
    }
    let [menu, content_section, footer_section] =
        *engine.arena.get(id).unwrap().children.as_slice()
    else {
        unreachable!()
    };
    reconcile_children(engine, menu, items, services)?;
    reconcile_children(engine, content_section, vec![*content], services)?;
    reconcile_children(
        engine,
        footer_section,
        footer.into_iter().map(|value| *value).collect(),
        services,
    )?;
    if selected_changed {
        engine.queue_control_update(
            id,
            ControlUpdate::NavigationView(NavigationUpdate::Selection(props.selected_key)),
        )?;
    }
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::NavigationView(Box::new(props)),
    ));
    Ok(())
}

pub(super) fn mount_navigation_view_item<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: NavigationViewItemProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::NavigationViewItem)?;
    engine.queue_control_update(
        id,
        ControlUpdate::NavigationViewItem(Box::new(navigation_view_item_update(&props))),
    )?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::NavigationViewItem(props)));
    Ok(id)
}

pub(super) fn reconcile_navigation_view_item<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: NavigationViewItemProps,
) -> Result<(), EngineError> {
    let changed = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::NavigationViewItem(old) => old != &props,
        _ => unreachable!(),
    };
    if changed {
        engine.queue_control_update(
            id,
            ControlUpdate::NavigationViewItem(Box::new(navigation_view_item_update(&props))),
        )?;
    }
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::NavigationViewItem(props)));
    Ok(())
}

fn mount_navigation_section<R: NativeRuntime>(
    engine: &mut Engine<R>,
    parent: NodeId,
    section: NavigationSection,
    children: Vec<Element>,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let id = engine.create_navigation_section(section)?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(None, MountedKind::NavigationSection));
    for child in children {
        let child = mount_element(engine, child, services)?;
        engine.attach(id, child)?;
    }
    engine.attach(parent, id)
}

fn navigation_view_update(props: &NavigationViewProps) -> NavigationViewUpdate {
    NavigationViewUpdate {
        header: props.header.clone(),
        pane_title: props.pane_title.clone(),
        settings_visible: props.settings_visible,
        pane_toggle_visible: props.pane_toggle_visible,
        pane_open: props.pane_open,
        open_pane_length: props.open_pane_length,
        pane_display_mode: props.pane_display_mode,
        selection_feedback: props.on_selection_changed.is_some(),
        pane_feedback: props.on_pane_open_changed.is_some(),
    }
}

fn navigation_view_item_update(props: &NavigationViewItemProps) -> NavigationViewItemUpdate {
    NavigationViewItemUpdate {
        item_key: props.item_key,
        label: props.label.clone(),
        icon: props.icon.clone(),
    }
}
