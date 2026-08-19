use super::content::reconcile_child;
use super::mount::mount_element;
use super::reconcile::reconcile_children;
use super::work::RenderServices;
use crate::element::Element;
use crate::element::props::{
    FlipViewProps, PivotItemProps, PivotProps, TabViewItemProps, TabViewProps,
};
use crate::engine::{Engine, EngineError};
use crate::id::NodeId;
use crate::mounted::{Mounted, MountedKind};
use crate::runtime::{
    ControlUpdate, NativeKind, NativeRuntime, PivotUpdate, TabViewItemUpdate, TabViewUpdate,
};

fn native_index(index: Option<usize>) -> i32 {
    index.map_or(-1, |index| i32::try_from(index).unwrap())
}

pub(super) fn mount_flip_view<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: FlipViewProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let FlipViewProps {
        items,
        selected_index,
        on_selection_changed,
        framework,
    } = props;
    let id = engine.create_native(NativeKind::FlipView)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::FlipView(FlipViewProps {
            items: Vec::new(),
            selected_index,
            on_selection_changed,
            framework,
        }),
    ));
    for item in items {
        let child = mount_element(engine, item, services)?;
        engine.attach(id, child)?;
    }
    engine.queue_control_update(
        id,
        ControlUpdate::IndexSelector(native_index(selected_index)),
    )?;
    Ok(id)
}

pub(super) fn reconcile_flip_view<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: FlipViewProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let FlipViewProps {
        items,
        selected_index,
        on_selection_changed,
        framework,
    } = props;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::FlipView(FlipViewProps {
            items: Vec::new(),
            selected_index,
            on_selection_changed,
            framework,
        }),
    ));
    reconcile_children(engine, id, items, services)?;
    engine.queue_control_update(
        id,
        ControlUpdate::IndexSelector(native_index(selected_index)),
    )
}

pub(super) fn mount_tab_view<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: TabViewProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let TabViewProps {
        items,
        selected_index,
        can_reorder_tabs,
        is_add_tab_button_visible,
        on_selection_changed,
        on_close_requested,
        on_add_tab_button_click,
        on_tabs_reordered,
        framework,
    } = props;
    let id = engine.create_native(NativeKind::TabView)?;
    engine.queue_control_update(
        id,
        ControlUpdate::TabView(TabViewUpdate::CanReorderTabs(can_reorder_tabs)),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::TabView(TabViewUpdate::IsAddTabButtonVisible(
            is_add_tab_button_visible,
        )),
    )?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::TabView(TabViewProps {
            items: Vec::new(),
            selected_index,
            can_reorder_tabs,
            is_add_tab_button_visible,
            on_selection_changed,
            on_close_requested,
            on_add_tab_button_click,
            on_tabs_reordered,
            framework,
        }),
    ));
    for item in items {
        let child = mount_element(engine, item, services)?;
        engine.attach(id, child)?;
    }
    engine.queue_control_update(
        id,
        ControlUpdate::IndexSelector(native_index(selected_index)),
    )?;
    Ok(id)
}

pub(super) fn reconcile_tab_view<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: TabViewProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let TabViewProps {
        items,
        selected_index,
        can_reorder_tabs,
        is_add_tab_button_visible,
        on_selection_changed,
        on_close_requested,
        on_add_tab_button_click,
        on_tabs_reordered,
        framework,
    } = props;
    let (reorder_changed, add_button_changed) =
        match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
            MountedKind::TabView(old) => (
                old.can_reorder_tabs != can_reorder_tabs,
                old.is_add_tab_button_visible != is_add_tab_button_visible,
            ),
            _ => unreachable!(),
        };
    if reorder_changed {
        engine.queue_control_update(
            id,
            ControlUpdate::TabView(TabViewUpdate::CanReorderTabs(can_reorder_tabs)),
        )?;
    }
    if add_button_changed {
        engine.queue_control_update(
            id,
            ControlUpdate::TabView(TabViewUpdate::IsAddTabButtonVisible(
                is_add_tab_button_visible,
            )),
        )?;
    }
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::TabView(TabViewProps {
            items: Vec::new(),
            selected_index,
            can_reorder_tabs,
            is_add_tab_button_visible,
            on_selection_changed,
            on_close_requested,
            on_add_tab_button_click,
            on_tabs_reordered,
            framework,
        }),
    ));
    reconcile_children(engine, id, items, services)?;
    engine.queue_control_update(
        id,
        ControlUpdate::IndexSelector(native_index(selected_index)),
    )
}

pub(super) fn mount_tab_view_item<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    child: Element,
    props: TabViewItemProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::TabViewItem)?;
    engine.queue_control_update(
        id,
        ControlUpdate::TabViewItem(TabViewItemUpdate::Key(props.item_key)),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::TabViewItem(TabViewItemUpdate::Header(props.header.clone())),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::TabViewItem(TabViewItemUpdate::Closable(props.closable)),
    )?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::TabViewItem(props)));
    let child = mount_element(engine, child, services)?;
    engine.attach(id, child)?;
    Ok(id)
}

pub(super) fn reconcile_tab_view_item<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    child: Element,
    props: TabViewItemProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let (item_key, header_changed, closable_changed) =
        match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
            MountedKind::TabViewItem(old) => (
                old.item_key,
                old.header != props.header,
                old.closable != props.closable,
            ),
            _ => unreachable!(),
        };
    assert_eq!(item_key, props.item_key, "TabView item key changed");
    if header_changed {
        engine.queue_control_update(
            id,
            ControlUpdate::TabViewItem(TabViewItemUpdate::Header(props.header.clone())),
        )?;
    }
    if closable_changed {
        engine.queue_control_update(
            id,
            ControlUpdate::TabViewItem(TabViewItemUpdate::Closable(props.closable)),
        )?;
    }
    reconcile_child(engine, id, child, services)?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::TabViewItem(props)));
    Ok(())
}

pub(super) fn mount_pivot<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: PivotProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let PivotProps {
        items,
        title,
        selected_index,
        on_selection_changed,
        framework,
    } = props;
    let id = engine.create_native(NativeKind::Pivot)?;
    engine.queue_control_update(id, ControlUpdate::Pivot(PivotUpdate::Title(title.clone())))?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::Pivot(PivotProps {
            items: Vec::new(),
            title,
            selected_index,
            on_selection_changed,
            framework,
        }),
    ));
    for item in items {
        let child = mount_element(engine, item, services)?;
        engine.attach(id, child)?;
    }
    engine.queue_control_update(
        id,
        ControlUpdate::IndexSelector(native_index(selected_index)),
    )?;
    Ok(id)
}

pub(super) fn reconcile_pivot<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: PivotProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let PivotProps {
        items,
        title,
        selected_index,
        on_selection_changed,
        framework,
    } = props;
    let title_changed = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::Pivot(old) => old.title != title,
        _ => unreachable!(),
    };
    if title_changed {
        engine.queue_control_update(id, ControlUpdate::Pivot(PivotUpdate::Title(title.clone())))?;
    }
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::Pivot(PivotProps {
            items: Vec::new(),
            title,
            selected_index,
            on_selection_changed,
            framework,
        }),
    ));
    reconcile_children(engine, id, items, services)?;
    engine.queue_control_update(
        id,
        ControlUpdate::IndexSelector(native_index(selected_index)),
    )
}

pub(super) fn mount_pivot_item<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    child: Element,
    props: PivotItemProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::PivotItem)?;
    engine.queue_control_update(id, ControlUpdate::PivotItemHeader(props.header.clone()))?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::PivotItem(props)));
    let child = mount_element(engine, child, services)?;
    engine.attach(id, child)?;
    Ok(id)
}

pub(super) fn reconcile_pivot_item<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    child: Element,
    props: PivotItemProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let header_changed = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::PivotItem(old) => old.header != props.header,
        _ => unreachable!(),
    };
    if header_changed {
        engine.queue_control_update(id, ControlUpdate::PivotItemHeader(props.header.clone()))?;
    }
    reconcile_child(engine, id, child, services)?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::PivotItem(props)));
    Ok(())
}
