use super::super::*;
use crate::native::RecordingRuntime;

fn visual_view(styled: bool) -> View {
    let border = Border::new();
    let text = TextBlock::new().text("Card");
    if styled {
        border
            .padding(Thickness::uniform(24.0))
            .border_thickness(Thickness::uniform(1.0))
            .corner_radius(CornerRadius::uniform(8.0))
            .content(text.font_size(28.0))
    } else {
        border.content(text)
    }
}

#[test]
fn visual_values_reject_invalid_components_before_mount() {
    for invalid in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(
            std::panic::catch_unwind(|| Border::new().padding(Thickness::uniform(invalid)))
                .is_err()
        );
        assert!(
            std::panic::catch_unwind(|| {
                Border::new().corner_radius(CornerRadius::uniform(invalid))
            })
            .is_err()
        );
        assert!(std::panic::catch_unwind(|| TextBlock::new().font_size(invalid)).is_err());
    }

    let _ = Border::new()
        .padding(Thickness::uniform(0.0))
        .border_thickness(Thickness::uniform(0.0))
        .corner_radius(CornerRadius::uniform(0.0));
}

#[test]
fn border_mount_records_struct_values_content_and_typography() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(visual_view(true)).unwrap();
    let border = pump.root().unwrap();
    let text = pump.runtime().node(border).unwrap().children()[0];

    assert_eq!(
        pump.runtime()
            .node(border)
            .unwrap()
            .property(PropertyId::BorderPadding),
        Some(&PropertyValue::Thickness(Thickness::uniform(24.0)))
    );
    assert_eq!(
        pump.runtime()
            .node(border)
            .unwrap()
            .property(PropertyId::BorderCornerRadius),
        Some(&PropertyValue::CornerRadius(CornerRadius::uniform(8.0)))
    );
    assert_eq!(
        pump.runtime()
            .node(text)
            .unwrap()
            .property(PropertyId::TextBlockFontSize),
        Some(&PropertyValue::F64(28.0))
    );
}

#[test]
fn visual_property_clear_and_no_op_use_the_shared_property_path() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(visual_view(true)).unwrap();
    let border = pump.root().unwrap();
    let text = pump.runtime().node(border).unwrap().children()[0];

    pump.update_view(visual_view(false)).unwrap();
    assert_eq!(
        pump.runtime()
            .node(border)
            .unwrap()
            .property(PropertyId::BorderPadding),
        None
    );
    assert_eq!(
        pump.runtime()
            .node(text)
            .unwrap()
            .property(PropertyId::TextBlockFontSize),
        None
    );
    let batches = pump.runtime().batches();

    pump.update_view(visual_view(false)).unwrap();
    assert_eq!(pump.runtime().batches(), batches);
}

#[test]
fn failed_visual_update_does_not_publish_struct_values() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(visual_view(true)).unwrap();
    let border = pump.root().unwrap();
    pump.runtime_mut().fail_at(0);

    assert!(matches!(
        pump.update_view(
            Border::new()
                .padding(Thickness::uniform(32.0))
                .content(TextBlock::new().text("Card"))
        ),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert_eq!(
        pump.tree
            .native(border)
            .unwrap()
            .properties
            .get(&PropertyId::BorderPadding),
        Some(&Some(PropertyValue::Thickness(Thickness::uniform(24.0))))
    );
}
