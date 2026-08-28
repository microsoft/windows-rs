//! Image URI property reconciliation tests.

use super::super::*;
use crate::native::*;

#[test]
fn image_uri_source_rejects_invalid_declarations() {
    assert!(Image::new().source("").is_err());
    assert!(Image::new().source("assets/logo.png").is_err());
    assert!(
        Image::new()
            .source_file(std::path::Path::new(r"assets\logo.png"))
            .is_err()
    );
}

#[test]
fn image_uri_source_updates_clears_and_remains_idempotent() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        Image::new()
            .source("file:///first.png")
            .unwrap()
            .width(0.0)
            .height(0.0)
            .stretch(Stretch::Uniform)
            .into(),
    )
    .unwrap();
    let root = pump.root().unwrap();
    assert_eq!(
        pump.runtime()
            .node(root)
            .and_then(|node| node.property(PropertyId::ImageSource)),
        Some(&PropertyValue::Str("file:///first.png".into()))
    );

    pump.update(Image::new().source("file:///second.png").unwrap().into())
        .unwrap();
    assert_eq!(
        pump.runtime()
            .node(root)
            .and_then(|node| node.property(PropertyId::ImageSource)),
        Some(&PropertyValue::Str("file:///second.png".into()))
    );

    pump.update(Image::new().source_optional(None::<String>).unwrap().into())
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
                    property: PropertyId::ImageSource,
                    ..
                }
            ))
    );
    let batches = pump.runtime().batches();
    pump.update(Image::new().source_optional(None::<String>).unwrap().into())
        .unwrap();
    assert_eq!(pump.runtime().batches(), batches);
}

#[test]
fn bitmap_dimensions_reject_invalid_values() {
    for value in [-1.0, f64::INFINITY, f64::NAN] {
        assert!(std::panic::catch_unwind(|| Image::new().width(value)).is_err());
        assert!(std::panic::catch_unwind(|| Image::new().height(value)).is_err());
    }
}
