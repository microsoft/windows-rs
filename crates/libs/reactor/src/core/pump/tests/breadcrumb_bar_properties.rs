//! Breadcrumb item-source reconciliation tests.

use super::super::*;
use crate::native::*;

#[test]
fn breadcrumb_items_update_clear_and_remain_idempotent() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        BreadcrumbBar::new()
            .items_source(["Home", "Documents"])
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    assert_eq!(
        pump.runtime()
            .node(root)
            .and_then(|node| node.property(PropertyId::BreadcrumbBarItemsSource)),
        Some(&PropertyValue::StrList(Rc::new(vec![
            "Home".into(),
            "Documents".into(),
        ])))
    );

    pump.update(
        BreadcrumbBar::new()
            .items_source(["Home", "Settings"])
            .into(),
    )
    .unwrap();
    assert_eq!(
        pump.runtime()
            .node(root)
            .and_then(|node| node.property(PropertyId::BreadcrumbBarItemsSource)),
        Some(&PropertyValue::StrList(Rc::new(vec![
            "Home".into(),
            "Settings".into(),
        ])))
    );

    pump.update(
        BreadcrumbBar::new()
            .items_source_optional(None::<Vec<String>>)
            .into(),
    )
    .unwrap();
    let batches = pump.runtime().batches();
    pump.update(
        BreadcrumbBar::new()
            .items_source_optional(None::<Vec<String>>)
            .into(),
    )
    .unwrap();
    assert_eq!(pump.runtime().batches(), batches);
}
