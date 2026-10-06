use super::*;

#[test]
fn navigation_view_flags_set_update_and_clear_independently() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    let view = NavigationView::new()
        .is_back_enabled(true)
        .is_pane_visible(false);
    runtime.update(view.clone()).unwrap();
    let root = runtime.graph().root().unwrap();
    assert_eq!(
        runtime.graph().properties(root).unwrap(),
        [
            Property {
                id: PropertyId::IsBackEnabled,
                value: PropertyValue::Bool(true),
            },
            Property {
                id: PropertyId::IsPaneVisible,
                value: PropertyValue::Bool(false),
            },
        ]
    );
    assert!(runtime.update(view).unwrap().is_empty());

    let mutations = runtime
        .update(
            NavigationView::new()
                .is_back_enabled(false)
                .is_pane_visible(true),
        )
        .unwrap();
    assert_eq!(mutations.len(), 1);
    let Mutation::SetProperties { object, set, clear } = &mutations[0] else {
        panic!("expected property mutation");
    };
    assert_eq!(*object, root);
    assert_eq!(
        set.as_ref(),
        [
            Property {
                id: PropertyId::IsBackEnabled,
                value: PropertyValue::Bool(false),
            },
            Property {
                id: PropertyId::IsPaneVisible,
                value: PropertyValue::Bool(true),
            },
        ]
    );
    assert!(clear.is_empty());

    let mutations = runtime
        .update(NavigationView::new().is_pane_visible(true))
        .unwrap();
    assert_eq!(mutations.len(), 1);
    assert!(matches!(
        &mutations[0],
        Mutation::SetProperties { object, set, clear }
            if *object == root && set.is_empty() && clear.as_ref() == [PropertyId::IsBackEnabled]
    ));
    let mutations = runtime.update(NavigationView::new()).unwrap();
    assert_eq!(mutations.len(), 1);
    assert!(matches!(
        &mutations[0],
        Mutation::SetProperties { object, set, clear }
            if *object == root && set.is_empty() && clear.as_ref() == [PropertyId::IsPaneVisible]
    ));
    assert_eq!(runtime.graph().root(), Some(root));
    assert!(runtime.graph().properties(root).unwrap().is_empty());
}

#[test]
fn navigation_view_pane_header_updates_replaces_and_detaches() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    let view = || NavigationView::new().pane_footer("Footer").content("Body");
    runtime.update(view().pane_header("Header")).unwrap();
    let root = runtime.graph().root().unwrap();
    let header = runtime.graph().child(root, RelationId::PaneHeader).unwrap();
    let footer = runtime.graph().child(root, RelationId::PaneFooter).unwrap();
    let content = runtime.graph().child(root, RelationId::Content).unwrap();
    assert_eq!(runtime.graph().kind(header), Some(ObjectType::TextBlock));
    assert!(
        runtime
            .update(view().pane_header("Header"))
            .unwrap()
            .is_empty()
    );

    runtime.update(view().pane_header("Updated")).unwrap();
    assert_eq!(
        runtime.graph().child(root, RelationId::PaneHeader),
        Some(header)
    );
    assert_eq!(
        runtime.graph().properties(header).unwrap(),
        [Property {
            id: PropertyId::Text,
            value: PropertyValue::String("Updated".into()),
        }]
    );
    runtime
        .update(view().pane_header(Button::new().content("Replacement")))
        .unwrap();
    let replacement = runtime.graph().child(root, RelationId::PaneHeader).unwrap();
    assert_ne!(replacement, header);
    assert_eq!(runtime.graph().kind(header), None);
    assert_eq!(runtime.graph().kind(replacement), Some(ObjectType::Button));

    runtime.update(view()).unwrap();
    assert_eq!(runtime.graph().child(root, RelationId::PaneHeader), None);
    assert_eq!(runtime.graph().kind(replacement), None);
    assert_eq!(
        runtime.graph().child(root, RelationId::PaneFooter),
        Some(footer)
    );
    assert_eq!(
        runtime.graph().child(root, RelationId::Content),
        Some(content)
    );
    assert_eq!(runtime.graph().root(), Some(root));
    assert_eq!(runtime.graph().object_count(), 3);
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
fn strings_convert_to_text_visuals() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update("Text").unwrap();

    let root = runtime.graph().root().unwrap();
    assert_eq!(runtime.graph().kind(root), Some(ObjectType::TextBlock));
    assert_eq!(
        runtime.graph().properties(root).unwrap(),
        [Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("Text")),
        }]
    );
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
                    .scale(0.95)
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
                TextBox::new("Text")
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
fn shared_values_record_set_update_clear_and_unchanged_updates() {
    let resources = ResourceOverrides::new().set("ControlCornerRadius", CornerRadius::uniform(6.0));
    let accelerators = KeyAccelerators::new([KeyAccelerator::new(
        AcceleratorKey::R,
        AcceleratorModifiers::Control,
        || {},
    )]);
    let initial = Button::new()
        .background(ThemeBrush::Accent)
        .resource_overrides(resources.clone())
        .style(ButtonStyle::Subtle)
        .key_accelerators(accelerators.clone());
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(initial.clone()).unwrap();
    let button = runtime.graph().root().unwrap();

    assert!(runtime.update(initial).unwrap().is_empty());
    assert_eq!(
        runtime.graph().properties(button).unwrap(),
        [
            Property {
                id: PropertyId::Background,
                value: PropertyValue::Brush(Brush::Theme(ThemeBrush::Accent)),
            },
            Property {
                id: PropertyId::Resources,
                value: PropertyValue::ResourceOverrides(resources),
            },
            Property {
                id: PropertyId::Style,
                value: PropertyValue::ButtonStyle(ButtonStyle::Subtle),
            },
            Property {
                id: PropertyId::KeyboardAccelerators,
                value: PropertyValue::KeyAccelerators(accelerators),
            },
        ]
    );

    let mutations = runtime
        .update(Button::new().background(Color::rgb(20, 30, 40)))
        .unwrap();
    let Mutation::SetProperties { set, clear, .. } = &mutations[0] else {
        panic!("expected property mutation");
    };
    assert_eq!(
        set.as_ref(),
        [Property {
            id: PropertyId::Background,
            value: PropertyValue::Brush(Brush::Solid(Color::rgb(20, 30, 40))),
        }]
    );
    assert_eq!(
        clear.as_ref(),
        [
            PropertyId::Resources,
            PropertyId::Style,
            PropertyId::KeyboardAccelerators,
        ]
    );
    assert!(
        runtime
            .update(Button::new())
            .unwrap()
            .iter()
            .any(|mutation| {
                matches!(
                    mutation,
                    Mutation::SetProperties { clear, .. }
                        if clear.as_ref() == [PropertyId::Background]
                )
            })
    );
}

#[test]
fn icon_slots_record_set_update_clear_and_unchanged_updates() {
    let initial_icon = Icon::font("\u{E721}");
    let initial = NavigationViewItem::new()
        .content("Search")
        .icon(initial_icon.clone());
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(initial.clone()).unwrap();
    let item = runtime.graph().root().unwrap();

    assert!(runtime.update(initial).unwrap().is_empty());
    assert_eq!(
        runtime.graph().properties(item).unwrap(),
        [Property {
            id: PropertyId::Icon,
            value: PropertyValue::Icon(initial_icon),
        }]
    );

    let path = Icon::path("M 0,0 L 8,8");
    let mutations = runtime
        .update(
            NavigationViewItem::new()
                .content("Search")
                .icon(path.clone()),
        )
        .unwrap();
    let Mutation::SetProperties { set, clear, .. } = &mutations[0] else {
        panic!("expected property mutation");
    };
    assert_eq!(
        set.as_ref(),
        [Property {
            id: PropertyId::Icon,
            value: PropertyValue::Icon(path),
        }]
    );
    assert!(clear.is_empty());

    let mutations = runtime
        .update(NavigationViewItem::new().content("Search"))
        .unwrap();
    assert!(mutations.iter().any(|mutation| {
        matches!(
            mutation,
            Mutation::SetProperties { clear, .. } if clear.as_ref() == [PropertyId::Icon]
        )
    }));
}

#[test]
fn text_box_appearance_records_set_and_clear() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            TextBox::new("Text")
                .placeholder_text("Enter text")
                .accepts_return(true)
                .text_wrapping(TextWrapping::Wrap),
        )
        .unwrap();
    let text_box = runtime.graph().root().unwrap();

    assert_eq!(
        runtime.graph().properties(text_box).unwrap(),
        [
            Property {
                id: PropertyId::Text,
                value: PropertyValue::String(Rc::from("Text")),
            },
            Property {
                id: PropertyId::PlaceholderText,
                value: PropertyValue::String(Rc::from("Enter text")),
            },
            Property {
                id: PropertyId::AcceptsReturn,
                value: PropertyValue::Bool(true),
            },
            Property {
                id: PropertyId::TextWrapping,
                value: PropertyValue::Enum {
                    kind: "TextWrapping",
                    variant: "Wrap",
                },
            },
        ]
    );

    let mutations = runtime.update(TextBox::new("Text")).unwrap();
    let Mutation::SetProperties { set, clear, .. } = &mutations[0] else {
        panic!("expected property mutation");
    };
    assert!(set.is_empty());
    assert_eq!(
        clear.as_ref(),
        [
            PropertyId::PlaceholderText,
            PropertyId::AcceptsReturn,
            PropertyId::TextWrapping,
        ]
    );
}

#[test]
fn shared_value_validation_rejects_invalid_inputs() {
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

#[test]
fn grid_definitions_update_and_clear_as_value_properties() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    let initial = Grid::new()
        .rows([GridLength::Auto, GridLength::Star(2.0)])
        .columns([GridLength::Pixel(20.0).min(10.0)]);
    let mutations = runtime.update(initial.clone()).unwrap();
    assert_eq!(
        mutations
            .iter()
            .filter_map(|mutation| match mutation {
                Mutation::SetProperties { set, .. } => Some(set.len()),
                _ => None,
            })
            .sum::<usize>(),
        2
    );
    assert!(runtime.update(initial).unwrap().is_empty());

    let mutations = runtime
        .update(Grid::new().rows([GridLength::Pixel(10.0)]))
        .unwrap();
    assert_eq!(
        mutations
            .iter()
            .filter_map(|mutation| match mutation {
                Mutation::SetProperties { set, clear, .. } => Some(
                    set.iter()
                        .filter(|property| {
                            matches!(property.id, PropertyId::GridRows | PropertyId::GridColumns)
                        })
                        .count()
                        + clear
                            .iter()
                            .filter(|property| {
                                matches!(property, PropertyId::GridRows | PropertyId::GridColumns)
                            })
                            .count(),
                ),
                _ => None,
            })
            .sum::<usize>(),
        2
    );
}

#[test]
fn shared_visual_properties_apply_to_handwritten_objects() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            TextBox::new("Text")
                .width(240.0)
                .margin(Thickness::uniform(12.0)),
        )
        .unwrap();
    let text_box = runtime.graph().root().unwrap();
    assert!(
        runtime
            .graph()
            .properties(text_box)
            .unwrap()
            .contains(&Property {
                id: PropertyId::Width,
                value: PropertyValue::F64(240.0),
            })
    );

    assert_eq!(
        runtime.update(TextBox::new("Text")).unwrap(),
        [Mutation::SetProperties {
            object: text_box,
            set: Rc::from([]),
            clear: Rc::from([PropertyId::Width, PropertyId::Margin]),
        }]
    );
}

#[test]
fn shared_capabilities_record_exact_property_and_focus_changes() {
    let reference: ElementRef = ElementRef::default();
    let mut adapter = RecordingAdapter::default();
    adapter.record_batches(true);
    let mut runtime = Runtime::new(adapter);
    runtime
        .update(
            StackPanel::new().children([
                Button::new()
                    .element_ref(&reference)
                    .is_enabled(false)
                    .grid_row(2)
                    .relative_align_left()
                    .automation_name("action")
                    .into(),
                TextBlock::new()
                    .text("Text")
                    .min_width(20.0)
                    .max_height(80.0)
                    .font_weight(FontWeight::SEMI_BOLD)
                    .into(),
            ]),
        )
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()
        .to_vec();
    assert_eq!(reference.get(), Some(children[0]));
    assert_eq!(runtime.adapter().batches().last().unwrap().len(), 7);
    assert_eq!(
        runtime
            .adapter()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(|mutation| match mutation {
                Mutation::SetProperties { set, .. } => Some(set.len()),
                _ => None,
            })
            .sum::<usize>(),
        8
    );
    assert!(runtime.focus(&reference).unwrap());
    assert_eq!(runtime.adapter().focuses(), [children[0]]);

    let mutations = runtime
        .update(StackPanel::new().children([
            Button::new().element_ref(&reference).into(),
            TextBlock::new().text("Text").into(),
        ]))
        .unwrap();
    assert_eq!(mutations.len(), 2);
    assert_eq!(
        mutations
            .iter()
            .filter_map(|mutation| match mutation {
                Mutation::SetProperties { clear, .. } => Some(clear.len()),
                _ => None,
            })
            .sum::<usize>(),
        7
    );
    assert_eq!(reference.get(), Some(children[0]));
}

#[test]
fn button_content_and_click_use_generated_contracts() {
    let calls = Rc::new(Cell::new(0));
    let callback_calls = Rc::clone(&calls);
    let callback = Callback::new(move |()| callback_calls.set(callback_calls.get() + 1));
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Button::new()
                .content(TextBlock::new().text("Deal"))
                .on_click(callback.clone()),
        )
        .unwrap();
    let button = runtime.graph().root().unwrap();
    let content = runtime.graph().child(button, RelationId::Content).unwrap();
    assert_eq!(runtime.graph().kind(button), Some(ObjectType::Button));
    assert_eq!(runtime.graph().kind(content), Some(ObjectType::TextBlock));

    runtime.adapter_mut().queue_event(EventDispatch::new(
        button,
        EventId::Click,
        EventValue::Unit(callback),
        EventPayload::Unit,
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(calls.get(), 1);
}

#[test]
fn final_value_parity_contracts_are_typed() {
    let mut image = Runtime::new(RecordingAdapter::default());
    image
        .update(
            Image::new()
                .source("https://example.com/image.png")
                .unwrap(),
        )
        .unwrap();
    assert!(matches!(
        &image.graph().properties(image.graph().root().unwrap()).unwrap()[0].value,
        PropertyValue::ImageSource(value)
            if matches!(
                value.value(),
                ImageSourceValue::Uri(value)
                    if value.as_ref() == "https://example.com/image.png"
            )
    ));
    image
        .update(Image::new().source_file(r"C:\work dir\a#b%20.png").unwrap())
        .unwrap();
    assert!(matches!(
        &image.graph().properties(image.graph().root().unwrap()).unwrap()[0].value,
        PropertyValue::ImageSource(value)
            if matches!(
                value.value(),
                ImageSourceValue::Uri(value)
                    if value.as_ref() == "file:///C:/work%20dir/a%23b%2520.png"
            )
    ));
    let encoded = EncodedImage::new(Vec::from(b"encoded image"));
    image
        .update(Image::new().source_data(encoded.clone()))
        .unwrap();
    assert!(matches!(
        &image.graph().properties(image.graph().root().unwrap()).unwrap()[0].value,
        PropertyValue::ImageSource(value)
            if matches!(value.value(), ImageSourceValue::Encoded(value) if value == &encoded)
    ));
    assert!(Image::new().source_file(r"images\asset.png").is_err());
    assert_eq!(
        EncodedImage::from_static(b"encoded image"),
        EncodedImage::new(Vec::from(b"encoded image"))
    );

    let rich_text = RichText::new([
        RichTextParagraph::new([
            RichTextInline::Run(RichTextRun::plain("First")),
            RichTextInline::LineBreak,
        ]),
        RichTextParagraph::new([RichTextInline::Hyperlink(RichTextHyperlink {
            text: "Second".into(),
            uri: "https://example.com".into(),
        })]),
    ]);
    let mut rich = Runtime::new(RecordingAdapter::default());
    rich.update(RichTextBlock::new().paragraphs(rich_text.clone()))
        .unwrap();
    assert_eq!(
        rich.graph()
            .properties(rich.graph().root().unwrap())
            .unwrap()[0]
            .value,
        PropertyValue::RichText(rich_text)
    );

    let color = Color::rgb(10, 20, 30);
    let color_values = Rc::new(RefCell::new(Vec::new()));
    let color_callback = {
        let values = Rc::clone(&color_values);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let mut picker = Runtime::new(RecordingAdapter::default());
    picker
        .update(
            ColorPicker::new()
                .color(color)
                .on_color_changed(color_callback.clone()),
        )
        .unwrap();
    let object = picker.graph().root().unwrap();
    picker.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::ColorChanged,
        EventValue::Color(color_callback),
        EventPayload::Color(color),
    ));
    assert_eq!(picker.dispatch_native_events().unwrap(), 1);
    assert_eq!(&*color_values.borrow(), &[color]);

    let date = DateTime::from_unix_secs(1_700_000_000);
    let date_values = Rc::new(RefCell::new(Vec::new()));
    let date_callback = {
        let values = Rc::clone(&date_values);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let mut date_picker = Runtime::new(RecordingAdapter::default());
    date_picker
        .update(DatePicker::new().on_date_changed(date_callback.clone()))
        .unwrap();
    let object = date_picker.graph().root().unwrap();
    date_picker.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::SelectedDateChanged,
        EventValue::OptionalDateTime(date_callback),
        EventPayload::OptionalDateTime(Some(date)),
    ));
    assert_eq!(date_picker.dispatch_native_events().unwrap(), 1);
    assert_eq!(&*date_values.borrow(), &[Some(date)]);

    let time = TimeSpan::from_millis(90_000);
    let time_values = Rc::new(RefCell::new(Vec::new()));
    let time_callback = {
        let values = Rc::clone(&time_values);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let mut time_picker = Runtime::new(RecordingAdapter::default());
    time_picker
        .update(TimePicker::new().on_time_changed(time_callback.clone()))
        .unwrap();
    let object = time_picker.graph().root().unwrap();
    time_picker.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::SelectedTimeChanged,
        EventValue::OptionalTimeSpan(time_callback),
        EventPayload::OptionalTimeSpan(Some(time)),
    ));
    assert_eq!(time_picker.dispatch_native_events().unwrap(), 1);
    assert_eq!(&*time_values.borrow(), &[Some(time)]);

    let modes = Rc::new(RefCell::new(Vec::new()));
    let mode_callback = {
        let values = Rc::clone(&modes);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let mut navigation = Runtime::new(RecordingAdapter::default());
    navigation
        .update(NavigationView::new().on_display_mode_changed(mode_callback.clone()))
        .unwrap();
    let object = navigation.graph().root().unwrap();
    navigation.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::DisplayModeChanged,
        EventValue::NavigationViewDisplayMode(mode_callback),
        EventPayload::NavigationViewDisplayMode(NavigationViewDisplayMode::Compact),
    ));
    assert_eq!(navigation.dispatch_native_events().unwrap(), 1);
    assert_eq!(&*modes.borrow(), &[NavigationViewDisplayMode::Compact]);
}

#[test]
fn generated_controls_cover_distinct_native_value_and_relation_shapes() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            StackPanel::new()
                .spacing(8.0)
                .orientation(Orientation::Horizontal)
                .children([
                    Slider::new().minimum(-10.0).maximum(10.0).value(2.5).into(),
                    CheckBox::new()
                        .is_checked(Some(true))
                        .content(TextBlock::new().text("Enabled"))
                        .into(),
                    ScrollViewer::new()
                        .content(
                            Canvas::new().children([TextBlock::new()
                                .text("Scrollable canvas")
                                .canvas_left(12.0)
                                .canvas_top(24.0)
                                .into()]),
                        )
                        .into(),
                ]),
        )
        .unwrap();

    let root = runtime.graph().root().unwrap();
    assert_eq!(
        runtime.graph().properties(root).unwrap(),
        [
            Property {
                id: PropertyId::Spacing,
                value: PropertyValue::F64(8.0),
            },
            Property {
                id: PropertyId::Orientation,
                value: PropertyValue::Enum {
                    kind: "Orientation",
                    variant: "Horizontal",
                },
            },
        ]
    );
    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap();
    assert_eq!(runtime.graph().kind(children[0]), Some(ObjectType::Slider));
    assert_eq!(
        runtime.graph().properties(children[0]).unwrap(),
        [
            Property {
                id: PropertyId::Minimum,
                value: PropertyValue::F64(-10.0),
            },
            Property {
                id: PropertyId::Maximum,
                value: PropertyValue::F64(10.0),
            },
            Property {
                id: PropertyId::Value,
                value: PropertyValue::F64(2.5),
            },
        ]
    );
    assert_eq!(
        runtime.graph().kind(children[1]),
        Some(ObjectType::CheckBox)
    );
    assert_eq!(
        runtime.graph().properties(children[1]).unwrap(),
        [Property {
            id: PropertyId::IsChecked,
            value: PropertyValue::OptionalBool(Some(true)),
        }]
    );
    let check_box_content = runtime
        .graph()
        .child(children[1], RelationId::Content)
        .unwrap();
    assert_eq!(
        runtime.graph().kind(check_box_content),
        Some(ObjectType::TextBlock)
    );
    assert_eq!(
        runtime.graph().kind(children[2]),
        Some(ObjectType::ScrollViewer)
    );
    let canvas = runtime
        .graph()
        .child(children[2], RelationId::Content)
        .unwrap();
    assert_eq!(runtime.graph().kind(canvas), Some(ObjectType::Canvas));
    assert_eq!(
        runtime
            .graph()
            .children(canvas, RelationId::Children)
            .unwrap()
            .len(),
        1
    );
    let canvas_child = runtime
        .graph()
        .children(canvas, RelationId::Children)
        .unwrap()[0];
    assert_eq!(
        runtime.graph().properties(canvas_child).unwrap(),
        [
            Property {
                id: PropertyId::CanvasLeft,
                value: PropertyValue::F64(12.0),
            },
            Property {
                id: PropertyId::CanvasTop,
                value: PropertyValue::F64(24.0),
            },
            Property {
                id: PropertyId::Text,
                value: PropertyValue::String(Rc::from("Scrollable canvas")),
            },
        ]
    );

    let mutations = runtime
        .update(
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(8.0)
                .children([
                    Slider::new().value(2.5).maximum(10.0).minimum(-10.0).into(),
                    CheckBox::new()
                        .content(TextBlock::new().text("Enabled"))
                        .is_checked(Some(true))
                        .into(),
                    ScrollViewer::new()
                        .content(
                            Canvas::new().children([TextBlock::new()
                                .text("Scrollable canvas")
                                .canvas_top(24.0)
                                .canvas_left(12.0)
                                .into()]),
                        )
                        .into(),
                ]),
        )
        .unwrap();
    assert!(mutations.is_empty());

    let mutations = runtime
        .update(
            StackPanel::new().children([
                Slider::new().into(),
                CheckBox::new()
                    .content(TextBlock::new().text("Enabled"))
                    .into(),
                ScrollViewer::new()
                    .content(
                        Canvas::new().children([TextBlock::new().text("Scrollable canvas").into()]),
                    )
                    .into(),
            ]),
        )
        .unwrap();
    assert!(mutations.iter().any(|mutation| {
        matches!(
            mutation,
            Mutation::SetProperties { object, clear, .. }
                if *object == root
                    && clear.len() == 2
                    && clear.contains(&PropertyId::Spacing)
                    && clear.contains(&PropertyId::Orientation)
        )
    }));
    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap();
    assert!(runtime.graph().properties(root).unwrap().is_empty());
    assert!(runtime.graph().properties(children[0]).unwrap().is_empty());
    assert!(runtime.graph().properties(children[1]).unwrap().is_empty());
    let canvas = runtime
        .graph()
        .child(children[2], RelationId::Content)
        .unwrap();
    let canvas_child = runtime
        .graph()
        .children(canvas, RelationId::Children)
        .unwrap()[0];
    assert_eq!(
        runtime.graph().properties(canvas_child).unwrap(),
        [Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("Scrollable canvas")),
        }]
    );
}
