use super::*;

#[test]
fn scale_requires_non_negative_finite_native_factors() {
    for value in [
        -1.0,
        -f64::MIN_POSITIVE,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::MAX,
    ] {
        assert!(std::panic::catch_unwind(|| Border::new().scale(value, 1.0)).is_err());
        assert!(std::panic::catch_unwind(|| Border::new().scale(1.0, value)).is_err());
    }
    Border::new().scale(0.0, 1.0);
    Border::new().scale(1.0, 0.0);
    Border::new().scale(0.25, 2.0);
    Border::new().scale(f64::from(f32::MAX), 1.0);
}

#[test]
fn font_icon_font_size_requires_finite_positive_values() {
    for value in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(std::panic::catch_unwind(|| FontIcon::new().font_size(value)).is_err());
    }
    FontIcon::new().font_size(16.0);
}

#[test]
fn text_block_line_height_requires_finite_non_negative_values() {
    for value in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(std::panic::catch_unwind(|| TextBlock::new().line_height(value)).is_err());
    }
    for value in [0.0, 0.5, 24.0] {
        TextBlock::new().line_height(value);
    }
}

#[test]
fn number_box_small_change_requires_finite_values() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(std::panic::catch_unwind(|| NumberBox::new().small_change(value)).is_err());
    }
    for value in [-0.5, 0.0, 0.25, 2.0] {
        NumberBox::new().small_change(value);
    }
}

#[test]
fn usize_keys_preserve_their_integer_value() {
    assert_eq!(Key::from(7usize), Key::from(7u64));
}

#[test]
fn callbacks_clone_without_clone_payloads() {
    struct Payload(u8);

    let callback = Callback::new(|payload: Payload| assert!(payload.0 < 2));
    let cloned = callback.clone();
    callback.call(Payload(0));
    cloned.call(Payload(1));

    let callback = RoutedCallback::new(|payload: Payload| payload.0 < 2);
    let cloned = callback.clone();
    assert!(callback.call(Payload(0)));
    assert!(cloned.call(Payload(1)));
}

#[test]
fn typed_relation_rejects_other_visual_kinds() {
    let contract = relation_contracts(ObjectType::Pivot)
        .iter()
        .find(|contract| contract.id == RelationId::Items)
        .unwrap();
    assert!(relation_accepts(contract, ObjectType::PivotItem));
    assert!(!relation_accepts(contract, ObjectType::Button));
}

#[test]
fn property_contract_enumeration_matches_point_lookup() {
    for kind in ALL_OBJECT_TYPES.iter().copied() {
        let contracts = property_contracts(kind);
        for id in ALL_PROPERTY_IDS.iter().copied() {
            assert_eq!(
                contracts.iter().find(|contract| contract.id == id).copied(),
                property_contract(kind, id),
                "{kind:?}.{id:?}"
            );
        }
        assert!(
            contracts
                .windows(2)
                .all(|pair| property_order(kind, pair[0].id) < property_order(kind, pair[1].id))
        );
    }
    let mut shared_shape = false;
    for (index, left) in ALL_OBJECT_TYPES.iter().copied().enumerate() {
        for right in ALL_OBJECT_TYPES.iter().copied().skip(index + 1) {
            let left = property_contracts(left);
            let right = property_contracts(right);
            if left == right {
                shared_shape = true;
                assert!(std::ptr::eq(left, right));
            }
        }
    }
    assert!(shared_shape);
}

#[test]
fn shared_value_builders_cover_the_public_family() {
    let resources = ResourceOverrides::new()
        .set("AccentFillColorDefaultBrush", Color::rgb(1, 2, 3))
        .set("ControlCornerRadius", CornerRadius::uniform(4.0))
        .set("ControlBorderThemeThickness", Thickness::uniform(1.0));
    let accelerators = KeyAccelerators::new([KeyAccelerator::new(
        AcceleratorKey::Enter,
        AcceleratorModifiers::Control,
        || {},
    )]);

    let _ = Grid::new()
        .background(ThemeBrush::SolidBackground)
        .key_accelerators(accelerators.clone())
        .keyed_children([
            keyed(
                "button",
                Button::new()
                    .background(Color::rgb(4, 5, 6))
                    .resource_overrides(resources)
                    .style(ButtonStyle::Accent)
                    .key_accelerators(accelerators),
            ),
            keyed(
                "link",
                HyperlinkButton::new()
                    .navigate_uri("https://example.com")
                    .unwrap(),
            ),
            keyed(
                "border",
                Border::new()
                    .background(ThemeBrush::CardBackground)
                    .border_brush(ThemeBrush::CardStroke)
                    .opacity_transition(Duration::from_millis(100))
                    .scale(0.95, 0.95)
                    .scale_transition(Duration::from_millis(120)),
            ),
            keyed(
                "text",
                TextBlock::new()
                    .text("Text")
                    .foreground(ThemeBrush::PrimaryText),
            ),
            keyed(
                "input",
                TextBox::new()
                    .text("Text")
                    .background(Color::rgb(7, 8, 9))
                    .border_brush(ThemeBrush::Accent)
                    .border_thickness(Thickness::uniform(2.0)),
            ),
            keyed(
                "rectangle",
                Rectangle::new()
                    .fill(ThemeBrush::Accent)
                    .stroke(Color::rgb(10, 11, 12)),
            ),
            keyed(
                "ellipse",
                Ellipse::new()
                    .fill(Color::rgb(13, 14, 15))
                    .stroke(ThemeBrush::SystemCritical),
            ),
            keyed("line", Line::new().stroke(ThemeBrush::AccentText)),
            keyed(
                "bitmap",
                BitmapIcon::new()
                    .uri_source("ms-appx:///Assets/icon.png")
                    .unwrap(),
            ),
            keyed("path", PathIcon::new().data("M 0,0 L 8,8")),
        ]);
}

#[test]
fn shared_value_validation_rejects_invalid_inputs() {
    for (x, y) in [
        (f64::NAN, 0.0),
        (0.0, f64::INFINITY),
        (f64::MAX, 0.0),
        (0.0, -f64::MAX),
    ] {
        assert!(std::panic::catch_unwind(|| Border::new().translation(x, y)).is_err());
        assert!(
            std::panic::catch_unwind(|| validate_property(
                ObjectType::Border,
                &Property {
                    id: PropertyId::Translation,
                    value: PropertyValue::Vector2([x, y]),
                }
            ))
            .is_err()
        );
    }
    assert!(HyperlinkButton::new().navigate_uri("not a uri").is_err());
    assert!(BitmapIcon::new().uri_source("").is_err());
    assert!(
        std::panic::catch_unwind(|| {
            ResourceOverrides::new().set("", Color::rgb(0, 0, 0));
        })
        .is_err()
    );
    assert!(
        std::panic::catch_unwind(|| {
            ResourceOverrides::new().set("Bad", Thickness::uniform(f64::NAN));
        })
        .is_err()
    );
}

#[test]
fn scroll_anchor_ratios_enforce_native_boundaries() {
    for ratio in [f64::NEG_INFINITY, -0.01, 1.01, f64::INFINITY] {
        assert!(
            std::panic::catch_unwind(|| ScrollView::new().horizontal_anchor_ratio(ratio)).is_err()
        );
        assert!(
            std::panic::catch_unwind(|| ScrollView::new().vertical_anchor_ratio(ratio)).is_err()
        );
    }
    for ratio in [0.0, 0.5, 1.0, f64::NAN] {
        ScrollView::new()
            .horizontal_anchor_ratio(ratio)
            .vertical_anchor_ratio(ratio);
    }
}

#[test]
fn unchanged_nan_properties_are_not_changes() {
    let declare = || {
        View::from(
            ScrollView::new()
                .horizontal_anchor_ratio(f64::NAN)
                .vertical_anchor_ratio(f64::NAN),
        )
        .0
        .object()
        .properties
        .clone()
    };
    assert_eq!(declare().as_slice(), declare().as_slice());
}

#[test]
fn grid_placement_builders_enforce_native_boundaries() {
    assert!(std::panic::catch_unwind(|| Button::new().grid_row(-1)).is_err());
    assert!(std::panic::catch_unwind(|| Button::new().grid_column(-1)).is_err());
    assert!(std::panic::catch_unwind(|| Button::new().grid_row_span(-1)).is_err());
    assert!(std::panic::catch_unwind(|| Button::new().grid_row_span(0)).is_err());
    assert!(std::panic::catch_unwind(|| Button::new().grid_column_span(-1)).is_err());
    assert!(std::panic::catch_unwind(|| Button::new().grid_column_span(0)).is_err());

    Button::new()
        .grid_row(0)
        .grid_row(1)
        .grid_column(0)
        .grid_column(1)
        .grid_row_span(1)
        .grid_column_span(1);
}

#[test]
fn grid_length_builders_enforce_native_boundaries() {
    for value in [-1.0, f64::NAN, f64::INFINITY] {
        assert!(std::panic::catch_unwind(|| GridLength::Pixel(value)).is_err());
        assert!(std::panic::catch_unwind(|| GridLength::Star(value)).is_err());
        assert!(std::panic::catch_unwind(|| GridLength::Auto.min(value)).is_err());
        assert!(std::panic::catch_unwind(|| GridLength::Auto.max(value)).is_err());
    }
    assert!(std::panic::catch_unwind(|| GridLength::Auto.max(1.0).min(2.0)).is_err());
    assert!(std::panic::catch_unwind(|| GridLength::Auto.min(2.0).max(1.0)).is_err());

    Grid::new().rows([
        GridLength::Auto,
        GridLength::STAR,
        GridLength::Pixel(0.0).min(0.0).max(1.0),
        GridLength::Star(2.0),
    ]);
}
