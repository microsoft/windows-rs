use super::mount::mount_element;
use super::reconcile::{
    reconcile, reconcile_children, reconcile_children_validated, validate_sibling_keys,
};
use super::*;
use crate::element::props::{
    GridProps, GridState, PanelProps, ScrollViewProps, ScrollViewerProps, StackPanelProps,
    StackPanelState, ViewboxProps,
};
use crate::element::tree::{ExpanderElement, SplitViewElement};

struct ScrollViewerChanges {
    horizontal: bool,
    vertical: bool,
    subscription: bool,
}

struct ScrollViewChanges {
    horizontal: bool,
    vertical: bool,
    orientation: bool,
    subscription: bool,
}

struct SplitViewChanges {
    display_mode: bool,
    pane_open: bool,
    open_pane_length: bool,
    compact_pane_length: bool,
    subscription: bool,
}

struct ExpanderChanges {
    expanded: bool,
    subscription: bool,
}

struct StackPanelChanges {
    orientation: bool,
    spacing: bool,
    padding: bool,
}

struct GridChanges {
    columns: bool,
    rows: bool,
    column_spacing: bool,
    row_spacing: bool,
}

pub(super) fn mount_stack_panel<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: StackPanelProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    validate_sibling_keys(&props.children)?;
    let id = engine.create_native(NativeKind::StackPanel)?;
    if props.orientation != crate::element::Orientation::Vertical {
        engine.queue_control_update(
            id,
            ControlUpdate::StackPanel(StackPanelUpdate::Orientation(props.orientation)),
        )?;
    }
    if props.spacing != 0.0 {
        engine.queue_control_update(
            id,
            ControlUpdate::StackPanel(StackPanelUpdate::Spacing(props.spacing)),
        )?;
    }
    if props.padding.is_some() {
        engine.set_padding(id, props.padding)?;
    }
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::StackPanel(StackPanelState {
            orientation: props.orientation,
            spacing: props.spacing,
            padding: props.padding,
            framework: props.framework,
        }),
    ));
    mount_children(engine, id, props.children, services)?;
    Ok(id)
}

pub(super) fn mount_grid<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    props: GridProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    validate_sibling_keys(&props.children)?;
    let id = engine.create_native(NativeKind::Grid)?;
    if !props.columns.is_empty() {
        engine.set_grid_columns(id, props.columns.clone())?;
    }
    if !props.rows.is_empty() {
        engine.set_grid_rows(id, props.rows.clone())?;
    }
    if props.column_spacing != 0.0 {
        engine.queue_control_update(
            id,
            ControlUpdate::Grid(GridUpdate::ColumnSpacing(props.column_spacing)),
        )?;
    }
    if props.row_spacing != 0.0 {
        engine.queue_control_update(
            id,
            ControlUpdate::Grid(GridUpdate::RowSpacing(props.row_spacing)),
        )?;
    }
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(
        key,
        MountedKind::Grid(GridState {
            columns: props.columns,
            rows: props.rows,
            column_spacing: props.column_spacing,
            row_spacing: props.row_spacing,
            framework: props.framework,
        }),
    ));
    mount_children(engine, id, props.children, services)?;
    Ok(id)
}

pub(super) fn mount_panel<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    kind: NativeKind,
    props: PanelProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    validate_sibling_keys(&props.children)?;
    let id = engine.create_native(kind)?;
    let mounted = match kind {
        NativeKind::Canvas => MountedKind::Canvas(props.framework),
        NativeKind::RelativePanel => MountedKind::RelativePanel(props.framework),
        _ => unreachable!(),
    };
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted::new(key, mounted));
    mount_children(engine, id, props.children, services)?;
    Ok(id)
}

pub(super) fn mount_viewbox<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    child: Element,
    props: ViewboxProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::Viewbox)?;
    engine.queue_control_update(id, ControlUpdate::ViewboxStretch(props.stretch))?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::Viewbox(props)));
    mount_child(engine, id, child, services)?;
    Ok(id)
}

pub(super) fn mount_scroll_viewer<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    child: Element,
    props: ScrollViewerProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::ScrollViewer)?;
    engine.queue_control_update(
        id,
        ControlUpdate::ScrollViewer(ScrollViewerUpdate::HorizontalScrollBarVisibility(
            props.horizontal_scroll_bar_visibility,
        )),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::ScrollViewer(ScrollViewerUpdate::VerticalScrollBarVisibility(
            props.vertical_scroll_bar_visibility,
        )),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::ScrollViewer(ScrollViewerUpdate::ViewChanged(
            props.on_view_changed.is_some(),
        )),
    )?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::ScrollViewer(props)));
    mount_child(engine, id, child, services)?;
    Ok(id)
}

pub(super) fn mount_scroll_view<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    child: Element,
    props: ScrollViewProps,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let id = engine.create_native(NativeKind::ScrollView)?;
    engine.queue_control_update(
        id,
        ControlUpdate::ScrollView(ScrollViewUpdate::HorizontalScrollBarVisibility(
            props.horizontal_scroll_bar_visibility,
        )),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::ScrollView(ScrollViewUpdate::VerticalScrollBarVisibility(
            props.vertical_scroll_bar_visibility,
        )),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::ScrollView(ScrollViewUpdate::ContentOrientation(
            props.content_orientation,
        )),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::ScrollView(ScrollViewUpdate::ViewChanged(
            props.on_view_changed.is_some(),
        )),
    )?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::ScrollView(props)));
    mount_child(engine, id, child, services)?;
    Ok(id)
}

pub(super) fn mount_split_view<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    split: SplitViewElement,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let SplitViewElement {
        content,
        pane,
        props,
    } = split;
    let id = engine.create_native(NativeKind::SplitView)?;
    engine.queue_control_update(
        id,
        ControlUpdate::SplitView(SplitViewUpdate::DisplayMode(props.display_mode)),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::SplitView(SplitViewUpdate::IsPaneOpen(props.is_pane_open)),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::SplitView(SplitViewUpdate::OpenPaneLength(props.open_pane_length)),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::SplitView(SplitViewUpdate::CompactPaneLength(
            props.compact_pane_length,
        )),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::SplitView(SplitViewUpdate::PaneClosed(props.on_pane_closed.is_some())),
    )?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::SplitView(Box::new(props))));
    mount_child(engine, id, *content, services)?;
    mount_child(engine, id, *pane, services)?;
    Ok(id)
}

pub(super) fn mount_expander<R: NativeRuntime>(
    engine: &mut Engine<R>,
    key: Option<u64>,
    expander: ExpanderElement,
    services: &RenderServices,
) -> Result<NodeId, EngineError> {
    let ExpanderElement {
        header,
        content,
        props,
    } = expander;
    let id = engine.create_native(NativeKind::Expander)?;
    engine.queue_control_update(
        id,
        ControlUpdate::Expander(ExpanderUpdate::Expanded(props.expanded)),
    )?;
    engine.queue_control_update(
        id,
        ControlUpdate::Expander(ExpanderUpdate::ExpandedChanged(
            props.on_expanded_changed.is_some(),
        )),
    )?;
    engine.arena.get_mut(id).unwrap().mounted =
        Some(Mounted::new(key, MountedKind::Expander(Box::new(props))));
    mount_child(engine, id, *header, services)?;
    mount_child(engine, id, *content, services)?;
    Ok(id)
}

pub(super) fn reconcile_stack_panel<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: StackPanelProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    validate_sibling_keys(&props.children)?;
    let changes = {
        let node = engine.arena.get(id).unwrap();
        let Some(Mounted {
            kind: MountedKind::StackPanel(old),
            ..
        }) = &node.mounted
        else {
            unreachable!()
        };
        StackPanelChanges {
            orientation: old.orientation != props.orientation,
            spacing: old.spacing != props.spacing,
            padding: old.padding != props.padding,
        }
    };
    if changes.orientation {
        engine.queue_control_update(
            id,
            ControlUpdate::StackPanel(StackPanelUpdate::Orientation(props.orientation)),
        )?;
    }
    if changes.spacing {
        engine.queue_control_update(
            id,
            ControlUpdate::StackPanel(StackPanelUpdate::Spacing(props.spacing)),
        )?;
    }
    if changes.padding {
        engine.set_padding(id, props.padding)?;
    }
    reconcile_children_validated(engine, id, props.children, services)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::StackPanel(StackPanelState {
            orientation: props.orientation,
            spacing: props.spacing,
            padding: props.padding,
            framework: props.framework,
        }),
    });
    Ok(())
}

pub(super) fn reconcile_grid<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    props: GridProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    validate_sibling_keys(&props.children)?;
    let changes = {
        let node = engine.arena.get(id).unwrap();
        let Some(Mounted {
            kind: MountedKind::Grid(old),
            ..
        }) = &node.mounted
        else {
            unreachable!()
        };
        GridChanges {
            columns: old.columns != props.columns,
            rows: old.rows != props.rows,
            column_spacing: old.column_spacing != props.column_spacing,
            row_spacing: old.row_spacing != props.row_spacing,
        }
    };
    if changes.columns {
        engine.set_grid_columns(id, props.columns.clone())?;
    }
    if changes.rows {
        engine.set_grid_rows(id, props.rows.clone())?;
    }
    if changes.column_spacing {
        engine.queue_control_update(
            id,
            ControlUpdate::Grid(GridUpdate::ColumnSpacing(props.column_spacing)),
        )?;
    }
    if changes.row_spacing {
        engine.queue_control_update(
            id,
            ControlUpdate::Grid(GridUpdate::RowSpacing(props.row_spacing)),
        )?;
    }
    reconcile_children_validated(engine, id, props.children, services)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::Grid(GridState {
            columns: props.columns,
            rows: props.rows,
            column_spacing: props.column_spacing,
            row_spacing: props.row_spacing,
            framework: props.framework,
        }),
    });
    Ok(())
}

pub(super) fn reconcile_panel<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    kind: NativeKind,
    props: PanelProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    reconcile_children(engine, id, props.children, services)?;
    let mounted = match kind {
        NativeKind::Canvas => MountedKind::Canvas(props.framework),
        NativeKind::RelativePanel => MountedKind::RelativePanel(props.framework),
        _ => unreachable!(),
    };
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted { key, kind: mounted });
    Ok(())
}

pub(super) fn reconcile_viewbox<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    child: Element,
    props: ViewboxProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let old_stretch = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::Viewbox(old) => old.stretch,
        _ => unreachable!(),
    };
    if old_stretch != props.stretch {
        engine.queue_control_update(id, ControlUpdate::ViewboxStretch(props.stretch))?;
    }
    reconcile_child(engine, id, 0, child, services)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::Viewbox(props),
    });
    Ok(())
}

pub(super) fn reconcile_scroll_viewer<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    child: Element,
    props: ScrollViewerProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::ScrollViewer(old) => ScrollViewerChanges {
            horizontal: old.horizontal_scroll_bar_visibility
                != props.horizontal_scroll_bar_visibility,
            vertical: old.vertical_scroll_bar_visibility != props.vertical_scroll_bar_visibility,
            subscription: old.on_view_changed.is_some() != props.on_view_changed.is_some(),
        },
        _ => unreachable!(),
    };
    if changes.horizontal {
        engine.queue_control_update(
            id,
            ControlUpdate::ScrollViewer(ScrollViewerUpdate::HorizontalScrollBarVisibility(
                props.horizontal_scroll_bar_visibility,
            )),
        )?;
    }
    if changes.vertical {
        engine.queue_control_update(
            id,
            ControlUpdate::ScrollViewer(ScrollViewerUpdate::VerticalScrollBarVisibility(
                props.vertical_scroll_bar_visibility,
            )),
        )?;
    }
    if changes.subscription {
        engine.queue_control_update(
            id,
            ControlUpdate::ScrollViewer(ScrollViewerUpdate::ViewChanged(
                props.on_view_changed.is_some(),
            )),
        )?;
    }
    reconcile_child(engine, id, 0, child, services)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::ScrollViewer(props),
    });
    Ok(())
}

pub(super) fn reconcile_scroll_view<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    child: Element,
    props: ScrollViewProps,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::ScrollView(old) => ScrollViewChanges {
            horizontal: old.horizontal_scroll_bar_visibility
                != props.horizontal_scroll_bar_visibility,
            vertical: old.vertical_scroll_bar_visibility != props.vertical_scroll_bar_visibility,
            orientation: old.content_orientation != props.content_orientation,
            subscription: old.on_view_changed.is_some() != props.on_view_changed.is_some(),
        },
        _ => unreachable!(),
    };
    if changes.horizontal {
        engine.queue_control_update(
            id,
            ControlUpdate::ScrollView(ScrollViewUpdate::HorizontalScrollBarVisibility(
                props.horizontal_scroll_bar_visibility,
            )),
        )?;
    }
    if changes.vertical {
        engine.queue_control_update(
            id,
            ControlUpdate::ScrollView(ScrollViewUpdate::VerticalScrollBarVisibility(
                props.vertical_scroll_bar_visibility,
            )),
        )?;
    }
    if changes.orientation {
        engine.queue_control_update(
            id,
            ControlUpdate::ScrollView(ScrollViewUpdate::ContentOrientation(
                props.content_orientation,
            )),
        )?;
    }
    if changes.subscription {
        engine.queue_control_update(
            id,
            ControlUpdate::ScrollView(ScrollViewUpdate::ViewChanged(
                props.on_view_changed.is_some(),
            )),
        )?;
    }
    reconcile_child(engine, id, 0, child, services)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::ScrollView(props),
    });
    Ok(())
}

pub(super) fn reconcile_split_view<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    split: SplitViewElement,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let SplitViewElement {
        content,
        pane,
        props,
    } = split;
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::SplitView(old) => SplitViewChanges {
            display_mode: old.display_mode != props.display_mode,
            pane_open: old.is_pane_open != props.is_pane_open,
            open_pane_length: old.open_pane_length.to_bits() != props.open_pane_length.to_bits(),
            compact_pane_length: old.compact_pane_length.to_bits()
                != props.compact_pane_length.to_bits(),
            subscription: old.on_pane_closed.is_some() != props.on_pane_closed.is_some(),
        },
        _ => unreachable!(),
    };
    if changes.display_mode {
        engine.queue_control_update(
            id,
            ControlUpdate::SplitView(SplitViewUpdate::DisplayMode(props.display_mode)),
        )?;
    }
    if changes.pane_open {
        engine.queue_control_update(
            id,
            ControlUpdate::SplitView(SplitViewUpdate::IsPaneOpen(props.is_pane_open)),
        )?;
    }
    if changes.open_pane_length {
        engine.queue_control_update(
            id,
            ControlUpdate::SplitView(SplitViewUpdate::OpenPaneLength(props.open_pane_length)),
        )?;
    }
    if changes.compact_pane_length {
        engine.queue_control_update(
            id,
            ControlUpdate::SplitView(SplitViewUpdate::CompactPaneLength(
                props.compact_pane_length,
            )),
        )?;
    }
    if changes.subscription {
        engine.queue_control_update(
            id,
            ControlUpdate::SplitView(SplitViewUpdate::PaneClosed(props.on_pane_closed.is_some())),
        )?;
    }
    reconcile_child(engine, id, 0, *content, services)?;
    reconcile_child(engine, id, 1, *pane, services)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::SplitView(Box::new(props)),
    });
    Ok(())
}

pub(super) fn reconcile_expander<R: NativeRuntime>(
    engine: &mut Engine<R>,
    id: NodeId,
    key: Option<u64>,
    expander: ExpanderElement,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let ExpanderElement {
        header,
        content,
        props,
    } = expander;
    let changes = match &engine.arena.get(id).unwrap().mounted.as_ref().unwrap().kind {
        MountedKind::Expander(old) => ExpanderChanges {
            expanded: old.expanded != props.expanded,
            subscription: old.on_expanded_changed.is_some() != props.on_expanded_changed.is_some(),
        },
        _ => unreachable!(),
    };
    if changes.expanded {
        engine.queue_control_update(
            id,
            ControlUpdate::Expander(ExpanderUpdate::Expanded(props.expanded)),
        )?;
    }
    if changes.subscription {
        engine.queue_control_update(
            id,
            ControlUpdate::Expander(ExpanderUpdate::ExpandedChanged(
                props.on_expanded_changed.is_some(),
            )),
        )?;
    }
    reconcile_child(engine, id, 0, *header, services)?;
    reconcile_child(engine, id, 1, *content, services)?;
    engine.arena.get_mut(id).unwrap().mounted = Some(Mounted {
        key,
        kind: MountedKind::Expander(Box::new(props)),
    });
    Ok(())
}

fn mount_child<R: NativeRuntime>(
    engine: &mut Engine<R>,
    parent: NodeId,
    child: Element,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let child = mount_element(engine, child, services)?;
    engine.attach(parent, child)
}

fn reconcile_child<R: NativeRuntime>(
    engine: &mut Engine<R>,
    parent: NodeId,
    index: usize,
    child: Element,
    services: &RenderServices,
) -> Result<(), EngineError> {
    let current = engine.arena.get(parent).unwrap().children[index];
    reconcile(engine, current, child, services)?;
    Ok(())
}

fn mount_children<R: NativeRuntime>(
    engine: &mut Engine<R>,
    parent: NodeId,
    children: Vec<Element>,
    services: &RenderServices,
) -> Result<(), EngineError> {
    for child in children {
        let child = mount_element(engine, child, services)?;
        engine.attach(parent, child)?;
    }
    Ok(())
}
