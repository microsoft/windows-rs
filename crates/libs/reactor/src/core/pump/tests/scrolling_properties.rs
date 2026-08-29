//! Scrolling control property reconciliation tests.

use super::super::*;

#[test]
fn scroll_viewer_properties_update_and_clear() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        ScrollViewer::new()
            .horizontal_scroll_bar_visibility(ScrollBarVisibility::Visible)
            .vertical_scroll_bar_visibility(ScrollBarVisibility::Hidden)
            .max_width(280.0)
            .max_height(80.0)
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    assert_eq!(
        pump.runtime()
            .node(root)
            .and_then(|node| node.property(PropertyId::MaxWidth)),
        Some(&PropertyValue::F64(280.0))
    );

    pump.update(ScrollViewer::new().into()).unwrap();
    assert!(
        pump.runtime()
            .commands()
            .last()
            .unwrap()
            .iter()
            .filter(|command| matches!(command, Command::ClearProperty { .. }))
            .count()
            >= 4
    );
}

#[test]
fn scroll_view_properties_update_and_clear() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        ScrollView::new()
            .horizontal_scroll_bar_visibility(ScrollingScrollBarVisibility::Visible)
            .vertical_scroll_bar_visibility(ScrollingScrollBarVisibility::Hidden)
            .max_width(280.0)
            .max_height(80.0)
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    assert_eq!(
        pump.runtime()
            .node(root)
            .and_then(|node| node.property(PropertyId::MaxHeight)),
        Some(&PropertyValue::F64(80.0))
    );

    pump.update(ScrollView::new().into()).unwrap();
    assert!(
        pump.runtime()
            .commands()
            .last()
            .unwrap()
            .iter()
            .filter(|command| matches!(command, Command::ClearProperty { .. }))
            .count()
            >= 4
    );
}

#[test]
fn scrolling_dimensions_reject_invalid_values() {
    for value in [-1.0, f64::INFINITY, f64::NAN] {
        assert!(std::panic::catch_unwind(|| ScrollViewer::new().max_width(value)).is_err());
        assert!(std::panic::catch_unwind(|| ScrollViewer::new().max_height(value)).is_err());
        assert!(std::panic::catch_unwind(|| ScrollView::new().max_width(value)).is_err());
        assert!(std::panic::catch_unwind(|| ScrollView::new().max_height(value)).is_err());
    }
}
