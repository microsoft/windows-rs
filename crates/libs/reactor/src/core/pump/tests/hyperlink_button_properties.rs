//! Hyperlink URI property reconciliation tests.

use super::super::*;

#[test]
fn hyperlink_uri_rejects_invalid_declarations() {
    assert!(HyperlinkButton::new().navigate_uri("").is_err());
    assert!(HyperlinkButton::new().navigate_uri("not a uri").is_err());
}

#[test]
fn hyperlink_uri_updates_clears_and_remains_idempotent() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        HyperlinkButton::new()
            .navigate_uri("https://example.com/first")
            .unwrap()
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    assert_eq!(
        pump.runtime()
            .node(root)
            .and_then(|node| node.property(PropertyId::HyperlinkButtonNavigateUri)),
        Some(&PropertyValue::Str("https://example.com/first".into()))
    );

    pump.update(
        HyperlinkButton::new()
            .navigate_uri("https://example.com/second")
            .unwrap()
            .into(),
    )
    .unwrap();
    assert_eq!(
        pump.runtime()
            .node(root)
            .and_then(|node| node.property(PropertyId::HyperlinkButtonNavigateUri)),
        Some(&PropertyValue::Str("https://example.com/second".into()))
    );

    pump.update(
        HyperlinkButton::new()
            .navigate_uri_optional(None::<String>)
            .unwrap()
            .into(),
    )
    .unwrap();
    assert!(
        pump.runtime()
            .commands()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::ClearProperty {
                    property: PropertyId::HyperlinkButtonNavigateUri,
                    ..
                }
            ))
    );
    let batches = pump.runtime().batches();
    pump.update(
        HyperlinkButton::new()
            .navigate_uri_optional(None::<String>)
            .unwrap()
            .into(),
    )
    .unwrap();
    assert_eq!(pump.runtime().batches(), batches);
}
