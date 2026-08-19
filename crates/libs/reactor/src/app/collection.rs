use super::mount::mount_element;
use super::reconcile::reconcile;
use super::*;
use crate::element::props::{
    ComboBoxProps, ListBoxProps, RadioButtonsProps, VirtualCollectionProps,
};

struct ListBoxChanges {
    items: bool,
    selection_mode: bool,
    selection: bool,
}

struct ComboBoxChanges {
    items: bool,
    header: bool,
    placeholder: bool,
    editable: bool,
    selection: bool,
}

struct RadioButtonsChanges {
    items: bool,
    header: bool,
    selection: bool,
    max_columns: bool,
}

struct VirtualCollectionChanges {
    items: bool,
    height: bool,
    automation_name: bool,
    help_text: bool,
    selection_mode: bool,
    selection: bool,
    display_selection: bool,
    invocation: bool,
    reorder: bool,
}

pub(super) fn mount_list_box<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: ListBoxProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::ListBox)?;
    engine.queue_control_update(
        id,
        ControlUpdate::ListBox(ListBoxUpdate::Items(props.items.values())),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::ListBox(ListBoxUpdate::SelectionMode(props.selection_mode)),
    )?;
    if !props.selection.is_empty() {
        engine.queue_control_update(
            id,
            ControlUpdate::ListBox(ListBoxUpdate::Selection(props.selection.clone())),
        )?;
    }
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::ListBox(props)));
    Ok(id)
}

pub(super) fn mount_combo_box<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: ComboBoxProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::ComboBox)?;
    engine.queue_control_update(
        id,
        ControlUpdate::ComboBox(Box::new(ComboBoxUpdate::Items(props.items.values()))),
    )?;
    if props.header.is_some() {
        engine.queue_control_update(
            id,
            ControlUpdate::ComboBox(Box::new(ComboBoxUpdate::Header(props.header.clone()))),
        )?;
    }
    if props.placeholder.is_some() {
        engine.queue_control_update(
            id,
            ControlUpdate::ComboBox(Box::new(ComboBoxUpdate::Placeholder(
                props.placeholder.clone(),
            ))),
        )?;
    }
    if props.editable {
        engine.queue_control_update(
            id,
            ControlUpdate::ComboBox(Box::new(ComboBoxUpdate::Editable(true))),
        )?;
    }
    if props.selected_key.is_some() {
        engine.queue_control_update(
            id,
            ControlUpdate::ComboBox(Box::new(ComboBoxUpdate::Selection(props.selected_key))),
        )?;
    }
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::ComboBox(props)));
    Ok(id)
}

pub(super) fn mount_radio_buttons<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: RadioButtonsProps,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::RadioButtons)?;
    engine.queue_control_update(
        id,
        ControlUpdate::RadioButtons(RadioButtonsUpdate::Items(props.items.values())),
    )?;
    if props.header.is_some() {
        engine.queue_control_update(
            id,
            ControlUpdate::RadioButtons(RadioButtonsUpdate::Header(props.header.clone())),
        )?;
    }
    engine.queue_control_update(
        id,
        ControlUpdate::RadioButtons(RadioButtonsUpdate::MaxColumns(props.max_columns)),
    )?;
    if props.selected_key.is_some() {
        engine.queue_control_update(
            id,
            ControlUpdate::RadioButtons(RadioButtonsUpdate::Selection(props.selected_key)),
        )?;
    }
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::RadioButtons(props)));
    Ok(id)
}

pub(super) fn mount_virtual_collection<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    mut props: VirtualCollectionProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let empty = props.empty.take();
    let id = engine.create_virtual_host(match props.kind {
        VirtualCollectionKind::ListView => NativeKind::ListView,
        VirtualCollectionKind::GridView => NativeKind::GridView,
    })?;
    engine.set_height(id, Some(props.height))?;
    set_virtual_collection_items(engine, id, &props.items)?;
    if props.automation_name.is_some() {
        engine.set_automation_name(id, props.automation_name.clone())?;
    }
    if props.help_text.is_some() {
        engine.set_help_text(id, props.help_text.clone())?;
    }
    engine.queue_control_update(
        id,
        ControlUpdate::Collection(CollectionUpdate::SelectionMode(props.selection_mode)),
    )?;
    if props.selection_display_only {
        engine.queue_control_update(
            id,
            ControlUpdate::Collection(CollectionUpdate::SelectionDisplayOnly(true)),
        )?;
    }
    if !props.selection.is_empty() {
        engine.queue_control_update(
            id,
            ControlUpdate::Collection(CollectionUpdate::Selection(props.selection.clone())),
        )?;
    }
    if props.on_item_invoked.is_some() {
        engine.queue_control_update(
            id,
            ControlUpdate::Collection(CollectionUpdate::ItemClickEnabled(true)),
        )?;
    }
    if props.can_reorder_items {
        engine.queue_control_update(
            id,
            ControlUpdate::Collection(CollectionUpdate::CanReorderItems(true)),
        )?;
    }
    let items_empty = props.items.is_empty();
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::VirtualCollection(Box::new(props)),
    ));
    if items_empty && let Some(empty) = empty {
        let empty = mount_element(engine, *empty, services)?;
        engine.attach_virtual_empty(id, empty)?;
    }
    Ok(id)
}

pub(super) fn reconcile_list_box<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: ListBoxProps,
) -> Result<(), EngineError> {
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::ListBox(old) => ListBoxChanges {
            items: old.items != props.items,
            selection_mode: old.selection_mode != props.selection_mode,
            selection: old.selection != props.selection,
        },
        _ => unreachable!(),
    };
    if changes.items {
        engine.queue_control_update(
            id,
            ControlUpdate::ListBox(ListBoxUpdate::Items(props.items.values())),
        )?;
    }
    if changes.selection_mode {
        engine.queue_control_update(
            id,
            ControlUpdate::ListBox(ListBoxUpdate::SelectionMode(props.selection_mode)),
        )?;
    }
    if changes.items || changes.selection_mode || changes.selection {
        engine.queue_control_update(
            id,
            ControlUpdate::ListBox(ListBoxUpdate::Selection(props.selection.clone())),
        )?;
    }
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::ListBox(props),
    });
    Ok(())
}

pub(super) fn reconcile_combo_box<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: ComboBoxProps,
) -> Result<(), EngineError> {
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::ComboBox(old) => ComboBoxChanges {
            items: old.items != props.items,
            header: old.header != props.header,
            placeholder: old.placeholder != props.placeholder,
            editable: old.editable != props.editable,
            selection: old.selected_key != props.selected_key,
        },
        _ => unreachable!(),
    };
    if changes.items {
        engine.queue_control_update(
            id,
            ControlUpdate::ComboBox(Box::new(ComboBoxUpdate::Items(props.items.values()))),
        )?;
    }
    if changes.header {
        engine.queue_control_update(
            id,
            ControlUpdate::ComboBox(Box::new(ComboBoxUpdate::Header(props.header.clone()))),
        )?;
    }
    if changes.placeholder {
        engine.queue_control_update(
            id,
            ControlUpdate::ComboBox(Box::new(ComboBoxUpdate::Placeholder(
                props.placeholder.clone(),
            ))),
        )?;
    }
    if changes.editable {
        engine.queue_control_update(
            id,
            ControlUpdate::ComboBox(Box::new(ComboBoxUpdate::Editable(props.editable))),
        )?;
    }
    if changes.items || changes.selection {
        engine.queue_control_update(
            id,
            ControlUpdate::ComboBox(Box::new(ComboBoxUpdate::Selection(props.selected_key))),
        )?;
    }
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::ComboBox(props),
    });
    Ok(())
}

pub(super) fn reconcile_radio_buttons<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: RadioButtonsProps,
) -> Result<(), EngineError> {
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::RadioButtons(old) => RadioButtonsChanges {
            items: old.items != props.items,
            header: old.header != props.header,
            selection: old.selected_key != props.selected_key,
            max_columns: old.max_columns != props.max_columns,
        },
        _ => unreachable!(),
    };
    if changes.items {
        engine.queue_control_update(
            id,
            ControlUpdate::RadioButtons(RadioButtonsUpdate::Items(props.items.values())),
        )?;
    }
    if changes.max_columns {
        engine.queue_control_update(
            id,
            ControlUpdate::RadioButtons(RadioButtonsUpdate::MaxColumns(props.max_columns)),
        )?;
    }
    if changes.header {
        engine.queue_control_update(
            id,
            ControlUpdate::RadioButtons(RadioButtonsUpdate::Header(props.header.clone())),
        )?;
    }
    if changes.items || changes.selection {
        engine.queue_control_update(
            id,
            ControlUpdate::RadioButtons(RadioButtonsUpdate::Selection(props.selected_key)),
        )?;
    }
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::RadioButtons(props),
    });
    Ok(())
}

pub(super) fn reconcile_virtual_collection<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    mut props: VirtualCollectionProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let empty = props.empty.take();
    let row_services = services.with_contexts(contexts_for_node(engine, id));
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::VirtualCollection(old) => VirtualCollectionChanges {
            items: old.items != props.items,
            height: old.height != props.height,
            automation_name: old.automation_name != props.automation_name,
            help_text: old.help_text != props.help_text,
            selection_mode: old.selection_mode != props.selection_mode,
            selection: old.selection != props.selection,
            display_selection: old.selection_display_only != props.selection_display_only,
            invocation: old.on_item_invoked.is_some() != props.on_item_invoked.is_some(),
            reorder: old.can_reorder_items != props.can_reorder_items,
        },
        _ => unreachable!(),
    };
    if changes.items {
        let rows = engine.take_virtual_row_roots(id)?;
        let (rows, removed) = match_virtual_rows(&props.items, rows);
        let mut parked = Vec::new();
        for (index, item_key, root) in rows {
            reconcile(engine, root, (props.row)(index), &row_services)?;
            parked.push((item_key, root));
        }
        for root in removed {
            engine.remove_subtree(root)?;
        }
        set_virtual_collection_items(engine, id, &props.items)?;
        engine.park_virtual_rows(id, parked)?;
    }
    if changes.height {
        engine.set_height(id, Some(props.height))?;
    }
    if changes.automation_name {
        engine.set_automation_name(id, props.automation_name.clone())?;
    }
    if changes.help_text {
        engine.set_help_text(id, props.help_text.clone())?;
    }
    if changes.selection_mode {
        engine.queue_control_update(
            id,
            ControlUpdate::Collection(CollectionUpdate::SelectionMode(props.selection_mode)),
        )?;
    }
    if changes.items || changes.selection_mode || changes.selection {
        engine.queue_control_update(
            id,
            ControlUpdate::Collection(CollectionUpdate::Selection(props.selection.clone())),
        )?;
    }
    if changes.display_selection {
        engine.queue_control_update(
            id,
            ControlUpdate::Collection(CollectionUpdate::SelectionDisplayOnly(
                props.selection_display_only,
            )),
        )?;
    }
    if changes.invocation {
        engine.queue_control_update(
            id,
            ControlUpdate::Collection(CollectionUpdate::ItemClickEnabled(
                props.on_item_invoked.is_some(),
            )),
        )?;
    }
    if changes.reorder {
        engine.queue_control_update(
            id,
            ControlUpdate::Collection(CollectionUpdate::CanReorderItems(props.can_reorder_items)),
        )?;
    }
    let current_empty = engine.virtual_empty(id);
    match (props.items.is_empty(), empty, current_empty) {
        (true, Some(empty), Some(current)) => {
            reconcile(engine, current, *empty, &row_services)?;
        }
        (true, Some(empty), None) => {
            let empty = mount_element(engine, *empty, &row_services)?;
            engine.attach_virtual_empty(id, empty)?;
        }
        (_, _, Some(current)) => engine.remove_subtree(current)?,
        _ => {}
    }
    if !changes.items {
        let realized = match &engine.arena.get(id).unwrap().kind {
            NodeKind::VirtualHost { realized } => realized
                .iter()
                .map(|(index, row)| (*index, row.root))
                .collect::<Vec<_>>(),
            _ => unreachable!(),
        };
        for (index, root) in realized {
            reconcile(engine, root, (props.row)(index), &row_services)?;
        }
        let (parked, removed) = match_virtual_rows(&props.items, engine.parked_virtual_rows(id));
        debug_assert!(removed.is_empty());
        for (index, _, root) in parked {
            reconcile(engine, root, (props.row)(index), &row_services)?;
        }
    }
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::VirtualCollection(Box::new(props)),
    });
    Ok(())
}
