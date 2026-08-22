use super::super::*;
use crate::native::RecordingRuntime;

fn grid_view(first_row: i32, include_definitions: bool) -> View {
    let grid = Grid::new().row_spacing(8.0).column_spacing(12.0);
    let grid = if include_definitions {
        grid.rows([GridLength::Auto, GridLength::STAR])
            .columns([GridLength::Pixel(120.0), GridLength::STAR])
    } else {
        grid
    };
    grid.keyed_children([
        KeyedView::new(
            "label",
            TextBlock::new()
                .text("Name")
                .grid_row(first_row)
                .grid_column(0),
        ),
        KeyedView::new(
            "value",
            TextBox::new()
                .grid_row(first_row)
                .grid_column(1)
                .grid_column_span(2),
        ),
    ])
}

fn virtual_grid_child(row: Option<i32>) -> View {
    let repeater = ItemsRepeater::new().item("row", TextBlock::new().text("row"));
    let repeater = if let Some(row) = row {
        repeater.grid_row(row)
    } else {
        repeater
    };
    Grid::new().children((repeater,))
}

#[test]
fn grid_definitions_reject_invalid_lengths_before_mount() {
    for invalid in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(
            std::panic::catch_unwind(|| Grid::new().rows([GridLength::Pixel(invalid)])).is_err()
        );
        assert!(
            std::panic::catch_unwind(|| Grid::new().columns([GridLength::Star(invalid)])).is_err()
        );
    }

    let _ = Grid::new()
        .rows([GridLength::Pixel(0.0)])
        .columns([GridLength::Star(0.0)]);
}

#[test]
fn grid_mount_records_definitions_spacing_and_child_placement() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(grid_view(0, true)).unwrap();
    let root = pump.root().unwrap();
    let children = pump.runtime().node(root).unwrap().children();

    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::GridRows),
        Some(&PropertyValue::GridLengths(
            vec![GridLength::Auto, GridLength::STAR].into()
        ))
    );
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::GridColumns),
        Some(&PropertyValue::GridLengths(
            vec![GridLength::Pixel(120.0), GridLength::STAR].into()
        ))
    );
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::GridRowSpacing),
        Some(&PropertyValue::F64(8.0))
    );
    assert_eq!(
        pump.runtime()
            .node(children[0])
            .unwrap()
            .property(PropertyId::GridColumn),
        Some(&PropertyValue::I32(0))
    );
    assert_eq!(
        pump.runtime()
            .node(children[1])
            .unwrap()
            .property(PropertyId::GridColumnSpan),
        Some(&PropertyValue::I32(2))
    );
}

#[test]
fn grid_update_and_clear_publish_only_after_native_success() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(grid_view(0, true)).unwrap();
    let root = pump.root().unwrap();
    let child = pump.runtime().node(root).unwrap().children()[0];

    pump.update_view(grid_view(2, false)).unwrap();
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::GridRows),
        None
    );
    assert_eq!(
        pump.runtime()
            .node(child)
            .unwrap()
            .property(PropertyId::GridRow),
        Some(&PropertyValue::I32(2))
    );

    pump.runtime_mut().fail_at(0);
    assert!(matches!(
        pump.update_view(grid_view(3, false)),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert_eq!(
        pump.tree
            .native(child)
            .unwrap()
            .properties
            .get(&PropertyId::GridRow),
        Some(&Some(PropertyValue::I32(2)))
    );
}

#[test]
fn identical_grid_update_is_a_native_no_op() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(grid_view(0, true)).unwrap();
    let batches = pump.runtime().batches();

    pump.update_view(grid_view(0, true)).unwrap();

    assert_eq!(pump.runtime().batches(), batches);
}

#[test]
fn keyed_grid_reorder_keeps_placement_with_node_identity() {
    let children = |reverse| {
        let label = KeyedView::new("label", TextBlock::new().text("Name").grid_column(0));
        let value = KeyedView::new("value", TextBox::new().grid_column(1));
        if reverse {
            [value, label]
        } else {
            [label, value]
        }
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(Grid::new().keyed_children(children(false)))
        .unwrap();
    let root = pump.root().unwrap();
    let original = pump.runtime().node(root).unwrap().children().to_vec();

    pump.update_view(Grid::new().keyed_children(children(true)))
        .unwrap();

    let reordered = pump.runtime().node(root).unwrap().children();
    assert_eq!(reordered, &[original[1], original[0]]);
    assert_eq!(
        pump.runtime()
            .node(original[0])
            .unwrap()
            .property(PropertyId::GridColumn),
        Some(&PropertyValue::I32(0))
    );
    assert_eq!(
        pump.runtime()
            .node(original[1])
            .unwrap()
            .property(PropertyId::GridColumn),
        Some(&PropertyValue::I32(1))
    );
}

#[test]
fn keyed_grid_child_replacement_applies_placement_to_the_new_node() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(Grid::new().keyed_children([KeyedView::new(
        "child",
        TextBlock::new().grid_row(1).grid_column(2),
    )]))
    .unwrap();
    let root = pump.root().unwrap();
    let old = pump.runtime().node(root).unwrap().children()[0];

    pump.update_view(Grid::new().keyed_children([KeyedView::new(
        "child",
        TextBox::new().grid_row(3).grid_column(4),
    )]))
    .unwrap();

    let child = pump.runtime().node(root).unwrap().children()[0];
    assert_ne!(child, old);
    assert_eq!(
        pump.runtime()
            .node(child)
            .unwrap()
            .property(PropertyId::GridRow),
        Some(&PropertyValue::I32(3))
    );
    assert_eq!(
        pump.runtime()
            .node(child)
            .unwrap()
            .property(PropertyId::GridColumn),
        Some(&PropertyValue::I32(4))
    );
}

#[test]
fn virtual_grid_child_mounts_updates_and_clears_placement() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(virtual_grid_child(Some(1))).unwrap();
    let root = pump.root().unwrap();
    let collection = pump.runtime().node(root).unwrap().children()[0];
    let commands = &pump.runtime().commands()[0];
    let create = commands
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::CreateVirtualCollection { node, .. } if *node == collection
            )
        })
        .unwrap();
    let placement = commands
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::SetProperty {
                    node,
                    property: PropertyId::GridRow,
                    ..
                } if *node == collection
            )
        })
        .unwrap();
    assert!(create < placement);
    assert_eq!(
        pump.runtime()
            .node(collection)
            .unwrap()
            .property(PropertyId::GridRow),
        Some(&PropertyValue::I32(1))
    );

    pump.update_view(virtual_grid_child(Some(2))).unwrap();
    assert_eq!(
        pump.runtime()
            .node(collection)
            .unwrap()
            .property(PropertyId::GridRow),
        Some(&PropertyValue::I32(2))
    );

    pump.update_view(virtual_grid_child(None)).unwrap();
    assert_eq!(
        pump.runtime()
            .node(collection)
            .unwrap()
            .property(PropertyId::GridRow),
        None
    );
}

#[test]
fn failed_virtual_grid_placement_does_not_publish_candidate_state() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(virtual_grid_child(Some(1))).unwrap();
    let collection = pump
        .runtime()
        .node(pump.root().unwrap())
        .unwrap()
        .children()[0];
    pump.runtime_mut().fail_at(0);

    assert!(matches!(
        pump.update_view(virtual_grid_child(Some(2))),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert_eq!(
        pump.tree
            .native(collection)
            .unwrap()
            .properties
            .get(&PropertyId::GridRow),
        Some(&Some(PropertyValue::I32(1)))
    );
    assert!(pump.poisoned());
}
