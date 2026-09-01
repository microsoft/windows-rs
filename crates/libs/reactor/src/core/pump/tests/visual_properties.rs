use super::super::*;
use crate::test::RecordingRuntime;

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

fn theme_view(background: Option<ThemeBrush>, border_brush: Option<ThemeBrush>) -> View {
    Border::new()
        .background_optional(background)
        .border_brush_optional(border_brush)
        .content(TextBlock::new().text("Card"))
}

fn brush_view(background: Option<Brush>) -> View {
    Border::new()
        .background_optional(background)
        .content(TextBlock::new().text("Card"))
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
        assert!(std::panic::catch_unwind(|| Border::new().scale(invalid)).is_err());
    }

    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(
            std::panic::catch_unwind(|| Border::new().margin(Thickness::uniform(invalid))).is_err()
        );
    }

    let _ = Border::new()
        .margin(Thickness::uniform(-1.0))
        .padding(Thickness::uniform(0.0))
        .border_thickness(Thickness::uniform(0.0))
        .corner_radius(CornerRadius::uniform(0.0));
}

#[test]
fn implicit_visual_transitions_mount_update_and_clear() {
    let duration = std::time::Duration::from_secs(1);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        Border::new()
            .opacity(1.0)
            .opacity_transition(duration)
            .scale(1.0)
            .scale_transition(duration)
            .into(),
    )
    .unwrap();
    let border = pump.root().unwrap();
    assert_eq!(
        pump.runtime()
            .node(border)
            .unwrap()
            .property(PropertyId::BorderOpacityTransition),
        Some(&PropertyValue::Duration(duration))
    );
    assert_eq!(
        pump.runtime()
            .node(border)
            .unwrap()
            .property(PropertyId::BorderScaleTransition),
        Some(&PropertyValue::Duration(duration))
    );

    pump.update(
        Border::new()
            .opacity(0.2)
            .opacity_transition(duration)
            .scale(1.3)
            .scale_transition(duration)
            .into(),
    )
    .unwrap();
    assert_eq!(
        pump.runtime()
            .node(border)
            .unwrap()
            .property(PropertyId::Opacity),
        Some(&PropertyValue::F64(0.2))
    );
    assert_eq!(
        pump.runtime()
            .node(border)
            .unwrap()
            .property(PropertyId::BorderScale),
        Some(&PropertyValue::F64(1.3))
    );

    pump.update(Border::new().into()).unwrap();
    assert_eq!(
        pump.runtime()
            .node(border)
            .unwrap()
            .property(PropertyId::BorderOpacityTransition),
        None
    );
    assert_eq!(
        pump.runtime()
            .node(border)
            .unwrap()
            .property(PropertyId::BorderScaleTransition),
        None
    );
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
fn border_preserves_independent_corner_radii() {
    let radius = CornerRadius::new(1.0, 2.0, 3.0, 4.0);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(Border::new().corner_radius(radius.clone()).into())
        .unwrap();
    let border = pump.root().unwrap();

    assert_eq!(
        pump.runtime()
            .node(border)
            .unwrap()
            .property(PropertyId::BorderCornerRadius),
        Some(&PropertyValue::CornerRadius(radius))
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
            .properties
            .get(&PropertyId::BorderPadding),
        Some(&Some(PropertyValue::Thickness(Thickness::uniform(24.0))))
    );
}

#[test]
fn theme_brushes_mount_as_one_grouped_style() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(theme_view(
        Some(ThemeBrush::CardBackground),
        Some(ThemeBrush::CardStroke),
    ))
    .unwrap();
    let border = pump.root().unwrap();
    let style = ThemeStyle::new([
        Some(ThemeBrush::CardBackground),
        Some(ThemeBrush::CardStroke),
        None,
        None,
    ]);

    assert_eq!(pump.runtime().node(border).unwrap().theme_style(), style);
    assert_eq!(
        pump.runtime()
            .commands()
            .iter()
            .flatten()
            .filter(|command| matches!(command, Command::SetThemeStyle { .. }))
            .count(),
        1
    );
    assert!(
        pump.runtime()
            .commands()
            .iter()
            .flatten()
            .all(|command| !matches!(
                command,
                Command::SetProperty {
                    property: PropertyId::BorderBackground | PropertyId::BorderBorderBrush,
                    ..
                }
            ))
    );
}

#[test]
fn theme_style_update_clear_and_no_op_are_transactional() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(theme_view(
        Some(ThemeBrush::CardBackground),
        Some(ThemeBrush::CardStroke),
    ))
    .unwrap();
    let border = pump.root().unwrap();

    pump.update_view(theme_view(
        Some(ThemeBrush::SolidBackground),
        Some(ThemeBrush::CardStroke),
    ))
    .unwrap();
    assert_eq!(
        pump.runtime().node(border).unwrap().theme_style(),
        ThemeStyle::new([
            Some(ThemeBrush::SolidBackground),
            Some(ThemeBrush::CardStroke),
            None,
            None,
        ])
    );

    pump.update_view(theme_view(None, None)).unwrap();
    assert_eq!(
        pump.runtime().node(border).unwrap().theme_style(),
        ThemeStyle::default()
    );
    let batches = pump.runtime().batches();
    pump.update_view(theme_view(None, None)).unwrap();
    assert_eq!(pump.runtime().batches(), batches);
}

#[test]
fn brush_transitions_switch_between_theme_property_and_inherited_values() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(brush_view(Some(ThemeBrush::CardBackground.into())))
        .unwrap();
    let border = pump.root().unwrap();
    let color = Color::rgb(20, 40, 60);

    pump.update_view(brush_view(Some(color.into()))).unwrap();
    assert_eq!(
        pump.runtime().node(border).unwrap().theme_style(),
        ThemeStyle::default()
    );
    assert_eq!(
        pump.runtime()
            .node(border)
            .unwrap()
            .property(PropertyId::BorderBackground),
        Some(&PropertyValue::Brush(Brush::Solid(color)))
    );
    assert!(
        pump.runtime()
            .commands()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::SetThemeStyle { style, .. } if style.is_empty()
            ))
    );

    let batches = pump.runtime().batches();
    pump.update_view(brush_view(Some(color.into()))).unwrap();
    assert_eq!(pump.runtime().batches(), batches);

    pump.update_view(brush_view(Some(ThemeBrush::SolidBackground.into())))
        .unwrap();
    assert_eq!(
        pump.runtime().node(border).unwrap().theme_style(),
        ThemeStyle::new([Some(ThemeBrush::SolidBackground), None, None, None])
    );
    assert_eq!(
        pump.runtime()
            .node(border)
            .unwrap()
            .property(PropertyId::BorderBackground),
        None
    );
    assert!(
        pump.runtime()
            .commands()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::ClearProperty {
                    property: PropertyId::BorderBackground,
                    ..
                }
            ))
    );

    pump.update_view(brush_view(None)).unwrap();
    assert_eq!(
        pump.runtime().node(border).unwrap().theme_style(),
        ThemeStyle::default()
    );
    assert_eq!(
        pump.runtime()
            .node(border)
            .unwrap()
            .property(PropertyId::BorderBackground),
        None
    );
}

#[test]
fn failed_theme_style_update_does_not_publish() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(theme_view(
        Some(ThemeBrush::CardBackground),
        Some(ThemeBrush::CardStroke),
    ))
    .unwrap();
    let border = pump.root().unwrap();
    let original = pump.tree.native(border).desired.theme_style();
    pump.runtime_mut().fail_at(0);

    assert!(matches!(
        pump.update_view(theme_view(
            Some(ThemeBrush::SolidBackground),
            Some(ThemeBrush::CardStroke),
        )),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert_eq!(pump.tree.native(border).desired.theme_style(), original);
    assert_eq!(pump.runtime().node(border).unwrap().theme_style(), original);
}
