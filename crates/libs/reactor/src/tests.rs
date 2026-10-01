use super::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[test]
fn usize_keys_preserve_their_integer_value() {
    assert_eq!(Key::from(7usize), Key::from(7u64));
}

fn text(value: &str) -> View {
    TextBlock::new().text(value).into()
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
fn initial_mount_reserves_the_validated_object_count() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children((0usize..512).map(|index| keyed(index, Border::new()))))
        .unwrap();

    let memory = runtime.graph().retained_memory();
    assert_eq!(memory.slot_len, 513);
    assert_eq!(memory.slot_capacity, 513);
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
                    .opacity_transition(std::time::Duration::from_millis(100))
                    .scale(0.95)
                    .scale_transition(std::time::Duration::from_millis(120)),
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
fn rich_edit_text_is_canonical_and_uses_deferred_exact_feedback() {
    let values = Rc::new(RefCell::new(Vec::new()));
    let callback = {
        let values = Rc::clone(&values);
        Callback::new(move |value: Rc<str>| values.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            RichEditBox::new()
                .text("one\r\ntwo\rthree")
                .on_text_changed(callback.clone()),
        )
        .unwrap();
    let object = runtime.graph().root().unwrap();
    assert!(
        runtime
            .graph()
            .properties(object)
            .unwrap()
            .contains(&Property {
                id: PropertyId::Document,
                value: PropertyValue::String(Rc::from("one\ntwo\nthree")),
            })
    );
    assert!(
        runtime
            .update(
                RichEditBox::new()
                    .text("one\ntwo\nthree")
                    .on_text_changed(callback.clone()),
            )
            .unwrap()
            .is_empty()
    );

    runtime.adapter_mut().queue_native_event(
        Some(Observation::SetProperty {
            object,
            property: Property {
                id: PropertyId::Document,
                value: PropertyValue::String(Rc::from("native\ntext")),
            },
        }),
        Some(EventDispatch::new(
            object,
            EventId::TextChanged,
            EventValue::String(callback),
            EventPayload::String(Rc::from("native\ntext")),
        )),
    );
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(&*values.borrow(), &[Rc::from("native\ntext")]);
}

#[test]
fn rich_edit_document_removal_clears_once_and_stays_omitted() {
    let callback = Callback::new(|_: Rc<str>| {});
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            RichEditBox::new()
                .text("Before")
                .on_text_changed(callback.clone()),
        )
        .unwrap();
    let object = runtime.graph().root().unwrap();

    let mutations = runtime
        .update(RichEditBox::new().on_text_changed(callback.clone()))
        .unwrap();
    assert!(mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::SetProperties { object: changed, clear, .. }
            if *changed == object && clear.as_ref() == [PropertyId::Document]
    )));
    assert!(
        runtime
            .graph()
            .properties(object)
            .unwrap()
            .iter()
            .all(|property| property.id != PropertyId::Document)
    );
    assert!(
        runtime
            .update(RichEditBox::new().on_text_changed(callback))
            .unwrap()
            .is_empty()
    );
}

#[test]
fn deferred_exact_feedback_suppresses_matching_observations_until_a_difference() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(RichEditBox::new().text("Before")).unwrap();
    let object = runtime.graph().root().unwrap();
    let property = Property {
        id: PropertyId::Document,
        value: PropertyValue::String(Rc::from("After")),
    };
    let mut feedback = FeedbackState::default();
    feedback.begin(
        object,
        EventId::TextChanged,
        FeedbackExpectation::DeferredExact(property.clone()),
    );
    assert_eq!(feedback.finish(object, EventId::TextChanged), None);
    for _ in 0..3 {
        assert!(!feedback.observe(
            object,
            EventId::TextChanged,
            Observation::SetProperty {
                object,
                property: property.clone(),
            }
        ));
    }
    let different = Observation::SetProperty {
        object,
        property: Property {
            id: PropertyId::Document,
            value: PropertyValue::String(Rc::from("Different")),
        },
    };
    assert!(feedback.observe(object, EventId::TextChanged, different.clone()));
    assert!(feedback.observe(object, EventId::TextChanged, different));
}

#[test]
fn newer_deferred_exact_write_replaces_the_previous_expectation() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(RichEditBox::new()).unwrap();
    let object = runtime.graph().root().unwrap();
    let mut feedback = FeedbackState::default();
    let property = |value| Property {
        id: PropertyId::Document,
        value: PropertyValue::String(Rc::from(value)),
    };
    feedback.begin(
        object,
        EventId::TextChanged,
        FeedbackExpectation::DeferredExact(property("Old")),
    );
    assert_eq!(feedback.finish(object, EventId::TextChanged), None);
    feedback.begin(
        object,
        EventId::TextChanged,
        FeedbackExpectation::DeferredExact(property("New")),
    );
    assert_eq!(feedback.finish(object, EventId::TextChanged), None);

    assert!(!feedback.observe(
        object,
        EventId::TextChanged,
        Observation::SetProperty {
            object,
            property: property("New"),
        }
    ));
    assert!(feedback.observe(
        object,
        EventId::TextChanged,
        Observation::SetProperty {
            object,
            property: property("Old"),
        }
    ));
}

#[test]
fn deferred_exact_clear_suppresses_repeated_empty_observations() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(RichEditBox::new().text("Before")).unwrap();
    let object = runtime.graph().root().unwrap();
    let empty = Property {
        id: PropertyId::Document,
        value: PropertyValue::String(Rc::from("")),
    };
    let mut feedback = FeedbackState::default();
    feedback.begin(
        object,
        EventId::TextChanged,
        FeedbackExpectation::DeferredExact(empty.clone()),
    );
    assert_eq!(feedback.finish(object, EventId::TextChanged), None);
    for _ in 0..2 {
        assert!(!feedback.observe(
            object,
            EventId::TextChanged,
            Observation::SetProperty {
                object,
                property: empty.clone(),
            }
        ));
    }
    feedback.remove_object(object);
    assert!(feedback.observe(
        object,
        EventId::TextChanged,
        Observation::SetProperty {
            object,
            property: empty,
        }
    ));
}

#[test]
fn disparate_controls_mount_into_the_same_retained_arena() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Grid::new().keyed_children([
                keyed("border", Border::new().content(text("content"))),
                keyed(
                    "tree",
                    TreeView::new().nodes([TreeNode::new("root", "Root")
                        .content(text("label"))
                        .children([TreeNode::new("child", "Child")])]),
                ),
                keyed(
                    "list",
                    ListView::new().items([
                        DataItem::new("first", "First"),
                        DataItem::new("second", "Second"),
                    ]),
                ),
            ]),
        )
        .unwrap();
    let root = runtime.graph().root().unwrap();

    assert_eq!(runtime.graph().object_count(), 10);
    assert_eq!(runtime.graph().kind(root), Some(ObjectType::Grid));
    assert_eq!(
        runtime
            .graph()
            .children(root, RelationId::Children)
            .map(<[ObjectId]>::len),
        Some(3)
    );
}

#[test]
fn stack_panel_keyed_reorder_preserves_objects_and_emits_one_generic_reorder() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(StackPanel::new().keyed_children([
            keyed("first", TextBlock::new().text("First")),
            keyed("second", TextBlock::new().text("Second")),
        ]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let before = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()
        .to_vec();

    let mutations = runtime
        .update(StackPanel::new().keyed_children([
            keyed("second", TextBlock::new().text("Second")),
            keyed("first", TextBlock::new().text("Changed")),
        ]))
        .unwrap();
    let after = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap();

    assert_eq!(after, [before[1], before[0]]);
    assert_eq!(
        mutations
            .iter()
            .filter(|mutation| matches!(mutation, Mutation::Reorder { .. }))
            .count(),
        1
    );
    assert!(
        !mutations
            .iter()
            .any(|mutation| matches!(mutation, Mutation::Create { .. } | Mutation::Destroy { .. }))
    );
    assert!(mutations
        .iter()
        .any(|mutation| matches!(mutation, Mutation::SetProperties { object, .. } if *object == before[0])));
}

#[test]
fn tab_view_keyed_reorder_preserves_tab_objects() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TabView::new().keyed_tab_items([
            keyed("first", TabViewItem::new().header("First")),
            keyed("second", TabViewItem::new().header("Second")),
        ]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let before = runtime
        .graph()
        .children(root, RelationId::TabItems)
        .unwrap()
        .to_vec();

    let mutations = runtime
        .update(TabView::new().keyed_tab_items([
            keyed("second", TabViewItem::new().header("Second")),
            keyed("first", TabViewItem::new().header("Changed")),
        ]))
        .unwrap();
    let after = runtime
        .graph()
        .children(root, RelationId::TabItems)
        .unwrap();

    assert_eq!(after, [before[1], before[0]]);
    assert_eq!(
        mutations
            .iter()
            .filter(|mutation| matches!(mutation, Mutation::Reorder { .. }))
            .count(),
        1
    );
    assert!(
        !mutations
            .iter()
            .any(|mutation| matches!(mutation, Mutation::Create { .. } | Mutation::Destroy { .. }))
    );
}

#[test]
fn keyed_insert_and_remove_produce_the_declared_order() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([
            keyed("first", TextBlock::new().text("First")),
            keyed("second", TextBlock::new().text("Second")),
        ]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let second = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[1];

    runtime
        .update(Grid::new().keyed_children([
            keyed("second", TextBlock::new().text("Second")),
            keyed("third", TextBlock::new().text("Third")),
        ]))
        .unwrap();

    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap();
    assert_eq!(children[0], second);
    assert_eq!(
        runtime.graph().kind(children[1]),
        Some(ObjectType::TextBlock)
    );
    assert_eq!(
        runtime.adapter().children(root, RelationId::Children),
        Some(children)
    );
}

#[test]
fn hierarchical_objects_use_the_same_keyed_relation_reconciler() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            TreeView::new().nodes([TreeNode::new("root", "Root")
                .content(TextBlock::new().text("Content"))
                .children([
                    TreeNode::new("first", "First"),
                    TreeNode::new("second", "Second"),
                ])]),
        )
        .unwrap();
    let tree = runtime.graph().root().unwrap();
    let root = runtime.graph().children(tree, RelationId::Roots).unwrap()[0];
    let content = runtime.graph().child(root, RelationId::Content).unwrap();
    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()
        .to_vec();

    let mutations = runtime
        .update(
            TreeView::new().nodes([TreeNode::new("root", "Renamed")
                .content(TextBlock::new().text("Updated"))
                .children([
                    TreeNode::new("second", "Second"),
                    TreeNode::new("first", "First"),
                ])]),
        )
        .unwrap();

    assert_eq!(
        runtime.graph().children(root, RelationId::Children),
        Some([children[1], children[0]].as_slice())
    );
    assert_eq!(
        runtime.graph().child(root, RelationId::Content),
        Some(content)
    );
    assert_eq!(
        mutations
            .iter()
            .filter(|mutation| matches!(mutation, Mutation::Reorder { .. }))
            .count(),
        1
    );
    assert!(
        !mutations
            .iter()
            .any(|mutation| matches!(mutation, Mutation::Create { .. } | Mutation::Destroy { .. }))
    );
}

#[test]
fn container_generated_data_uses_the_same_keyed_relation_reconciler() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(ListView::new().items([
            DataItem::new("first", "First"),
            DataItem::new("second", "Second"),
        ]))
        .unwrap();
    let list = runtime.graph().root().unwrap();
    let before = runtime
        .graph()
        .children(list, RelationId::Items)
        .unwrap()
        .to_vec();

    let mutations = runtime
        .update(ListView::new().items([
            DataItem::new("second", "Second"),
            DataItem::new("first", "Changed"),
        ]))
        .unwrap();

    assert_eq!(
        runtime.graph().children(list, RelationId::Items),
        Some([before[1], before[0]].as_slice())
    );
    assert_eq!(
        mutations
            .iter()
            .filter(|mutation| matches!(mutation, Mutation::Reorder { .. }))
            .count(),
        1
    );
}

#[test]
fn owned_content_replacement_uses_generic_relation_mutations() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Border::new().content(TextBlock::new().text("Text")))
        .unwrap();
    let border = runtime.graph().root().unwrap();
    let previous = runtime.graph().child(border, RelationId::Content).unwrap();

    let mutations = runtime.update(Border::new().content(Grid::new())).unwrap();
    let next = runtime.graph().child(border, RelationId::Content).unwrap();

    assert_ne!(next, previous);
    assert_eq!(runtime.graph().kind(next), Some(ObjectType::Grid));
    assert!(matches!(
        mutations.as_slice(),
        [
            Mutation::Detach { .. },
            Mutation::Destroy { .. },
            Mutation::Create { .. },
            Mutation::Attach { .. }
        ]
    ));
}

#[test]
fn positional_children_reconcile_by_index_without_keys() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            StackPanel::new()
                .children([TextBlock::new().text("First").into(), Border::new().into()]),
        )
        .unwrap();
    let panel = runtime.graph().root().unwrap();
    let before = runtime
        .graph()
        .children(panel, RelationId::Children)
        .unwrap()
        .to_vec();

    runtime
        .update(StackPanel::new().children([
            TextBlock::new().text("Changed").into(),
            Grid::new().into(),
            TextBlock::new().text("Third").into(),
        ]))
        .unwrap();

    let after = runtime
        .graph()
        .children(panel, RelationId::Children)
        .unwrap();
    assert_eq!(after[0], before[0]);
    assert_ne!(after[1], before[1]);
    assert_eq!(runtime.graph().kind(after[1]), Some(ObjectType::Grid));
    assert_eq!(
        runtime.adapter().children(panel, RelationId::Children),
        Some(after)
    );
}

#[test]
fn property_updates_only_emit_changed_values() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TreeView::new().nodes([TreeNode::new("root", "Before").expanded(true)]))
        .unwrap();

    let mutations = runtime
        .update(TreeView::new().nodes([TreeNode::new("root", "After").expanded(true)]))
        .unwrap();

    let (set, clear) = mutations
        .iter()
        .find_map(|mutation| match mutation {
            Mutation::SetProperties { set, clear, .. } => Some((set, clear)),
            _ => None,
        })
        .unwrap();
    assert_eq!(set.len(), 1);
    assert_eq!(set[0].id, PropertyId::Text);
    assert!(clear.is_empty());
}

#[test]
fn visual_transitions_are_retained_stable_and_removed_when_omitted() {
    let transitions = || [ThemeTransition::Reposition];
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Border::new()
                .margin(Thickness::new(10.0, 20.0, 0.0, 0.0))
                .transitions(transitions()),
        )
        .unwrap();
    let border = runtime.graph().root().unwrap();
    assert!(matches!(
        runtime
            .graph()
            .properties(border)
            .unwrap()
            .iter()
            .find(|property| property.id == PropertyId::Transitions),
        Some(Property {
            value: PropertyValue::ThemeTransitions(value),
            ..
        }) if value.as_ref() == transitions().as_slice()
    ));

    assert!(
        runtime
            .update(
                Border::new()
                    .margin(Thickness::new(10.0, 20.0, 0.0, 0.0))
                    .transitions(transitions()),
            )
            .unwrap()
            .is_empty()
    );

    let mutations = runtime
        .update(Border::new().margin(Thickness::new(30.0, 40.0, 0.0, 0.0)))
        .unwrap();
    assert_eq!(
        mutations,
        [Mutation::SetProperties {
            object: border,
            set: Rc::from([Property {
                id: PropertyId::Margin,
                value: PropertyValue::Thickness(Thickness::new(30.0, 40.0, 0.0, 0.0)),
            }]),
            clear: Rc::from([PropertyId::Transitions]),
        }]
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
fn reference_only_changes_reconcile_without_native_mutations() {
    let reference: ElementRef = ElementRef::default();
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(Button::new()).unwrap();
    let button = runtime.graph().root().unwrap();

    assert!(
        runtime
            .update(Button::new().element_ref(&reference))
            .unwrap()
            .is_empty()
    );
    assert_eq!(reference.get(), Some(button));

    assert!(runtime.update(Button::new()).unwrap().is_empty());
    assert_eq!(reference.get(), None);
}

struct HoldingImperativeAdapter {
    inner: RecordingAdapter,
    commands: Rc<RefCell<Vec<ImperativeRequest>>>,
}

impl Adapter for HoldingImperativeAdapter {
    type Error = AdapterError;

    fn preview_native_events(&self, events: &mut Vec<NativeEvent>) {
        self.inner.preview_native_events(events);
    }

    fn pop_native_event(&mut self) -> Option<NativeEvent> {
        self.inner.pop_native_event()
    }

    fn validate(&self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        self.inner.validate(mutations)
    }

    fn apply(&mut self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        self.inner.apply(mutations)
    }

    fn focus(&mut self, object: ObjectId) -> Result<bool, Self::Error> {
        self.inner.focus(object)
    }

    fn imperative(&mut self, request: ImperativeRequest) -> Result<(), Self::Error> {
        self.commands.borrow_mut().push(request);
        Ok(())
    }
}

#[test]
fn typed_references_cover_all_imperative_controls() {
    let text_box = ElementRef::<TextBox>::new();
    let grid = ElementRef::<Grid>::new();
    let image = ElementRef::<Image>::new();
    let webview = ElementRef::<WebView2>::new();
    let swap_chain = ElementRef::<SwapChainPanel>::new();

    let _: View = TextBox::new("").element_ref(&text_box).into();
    let _: View = Grid::new().element_ref(&grid).into();
    let _: View = Image::new().element_ref(&image).into();
    let _: View = WebView2::new().element_ref(&webview).into();
    let _: View = SwapChainPanel::new().element_ref(&swap_chain).into();
}

#[test]
fn typed_element_reference_queues_focus() {
    let reference = ElementRef::<TextBox>::new();
    let result = Rc::new(Cell::new(None));
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TextBox::new("").element_ref(&reference))
        .unwrap();
    let completed = Rc::clone(&result);
    assert!(reference.request_focus_result(move |value| {
        completed.set(Some(value));
    }));
    runtime.dispatch_native_events().unwrap();
    assert_eq!(result.get(), Some(Ok(true)));
}

#[test]
fn title_bar_declaration_owns_window_attachment() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    let mutations = runtime
        .update(Grid::new().keyed_children([keyed(
            "title",
            TitleBar::new().preferred_height(WindowTitleBarHeight::Tall),
        )]))
        .unwrap();
    let (title_bar, height) = runtime.graph().window_title_bar().unwrap().unwrap();
    let insert = mutations
        .iter()
        .position(
            |mutation| matches!(mutation, Mutation::Insert { child, .. } if *child == title_bar),
        )
        .unwrap();
    let set = mutations
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::SetWindowTitleBar { object, .. } if *object == title_bar
            )
        })
        .unwrap();

    assert_eq!(height, WindowTitleBarHeight::Tall);
    assert!(insert < set);
    assert_eq!(
        runtime.adapter().window_title_bar(),
        Some((title_bar, WindowTitleBarHeight::Tall))
    );

    let mutations = runtime
        .update(Grid::new().keyed_children([keyed(
            "title",
            TitleBar::new().preferred_height(WindowTitleBarHeight::Standard),
        )]))
        .unwrap();
    assert!(mutations.iter().any(|mutation| {
        matches!(
            mutation,
            Mutation::SetWindowTitleBar {
                object,
                height: WindowTitleBarHeight::Standard
            } if *object == title_bar
        )
    }));
    assert_eq!(
        runtime.adapter().window_title_bar(),
        Some((title_bar, WindowTitleBarHeight::Standard))
    );
}

#[test]
fn title_bar_clear_precedes_destruction() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([keyed("title", TitleBar::new())]))
        .unwrap();
    let title_bar = runtime.graph().window_title_bar().unwrap().unwrap().0;

    let mutations = runtime.update(Grid::new()).unwrap();
    let clear = mutations
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::ClearWindowTitleBar { object } if *object == title_bar
            )
        })
        .unwrap();
    let destroy = mutations
        .iter()
        .position(
            |mutation| matches!(mutation, Mutation::Destroy { object } if *object == title_bar),
        )
        .unwrap();

    assert!(clear < destroy);
    assert_eq!(runtime.graph().window_title_bar().unwrap(), None);
    assert_eq!(runtime.adapter().window_title_bar(), None);
}

#[test]
fn duplicate_title_bars_fail_before_adapter_apply() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.record_batches(true);

    assert_eq!(
        runtime.update(Grid::new().keyed_children([
            keyed("first", TitleBar::new()),
            keyed("second", TitleBar::new()),
        ])),
        Err(UpdateError::Graph(GraphError::DuplicateWindowTitleBar))
    );
    assert!(runtime.adapter().batches().is_empty());
    assert!(runtime.graph().root().is_none());
}

#[test]
fn tooltip_attachment_reconciles_content_and_placement() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    let mutations = runtime
        .update(
            TextBlock::new()
                .text("Target")
                .tooltip_with(Tooltip::text("First").placement(TooltipPlacement::Bottom)),
        )
        .unwrap();
    let target = runtime.graph().root().unwrap();
    let tooltip = runtime.graph().tooltip(target).unwrap();
    let attach = mutations
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::SetTooltip {
                    target: current_target,
                    tooltip: Some(current_tooltip),
                    placement: TooltipPlacement::Bottom,
                } if *current_target == target && *current_tooltip == tooltip
            )
        })
        .unwrap();
    let create = mutations
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::Create {
                    object,
                    kind: ObjectType::ToolTip,
                } if *object == tooltip
            )
        })
        .unwrap();
    assert!(create < attach);
    assert_eq!(
        runtime.adapter().tooltip(target),
        Some((tooltip, TooltipPlacement::Bottom))
    );

    let mutations = runtime
        .update(
            TextBlock::new()
                .text("Target")
                .tooltip_with(Tooltip::text("Second").placement(TooltipPlacement::Bottom)),
        )
        .unwrap();
    assert_eq!(runtime.graph().tooltip(target), Some(tooltip));
    assert!(
        !mutations
            .iter()
            .any(|mutation| matches!(mutation, Mutation::SetTooltip { .. }))
    );

    let mutations = runtime
        .update(
            TextBlock::new()
                .text("Target")
                .tooltip_with(Tooltip::text("Third").placement(TooltipPlacement::Left)),
        )
        .unwrap();
    assert_eq!(runtime.graph().tooltip(target), Some(tooltip));
    assert!(mutations.iter().any(|mutation| {
        matches!(
            mutation,
            Mutation::SetTooltip {
                target: current_target,
                tooltip: Some(current_tooltip),
                placement: TooltipPlacement::Left,
            } if *current_target == target && *current_tooltip == tooltip
        )
    }));
}

#[test]
fn flyout_attachment_reconciles_content_placement_and_removal() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    let mutations = runtime
        .update(
            Button::new()
                .content("Target")
                .flyout_with(Flyout::text("First").placement(FlyoutPlacement::Bottom)),
        )
        .unwrap();
    let target = runtime.graph().root().unwrap();
    let (content, placement) = runtime.graph().flyout(target).unwrap();
    assert_eq!(placement, FlyoutPlacement::Bottom);
    assert!(mutations.iter().any(|mutation| {
        matches!(
            mutation,
            Mutation::SetFlyout {
                target: current_target,
                content: Some(current_content),
                placement: FlyoutPlacement::Bottom,
            } if *current_target == target && *current_content == content
        )
    }));
    assert_eq!(
        runtime.adapter().flyout(target),
        Some((content, FlyoutPlacement::Bottom))
    );

    let mutations = runtime
        .update(
            Button::new()
                .content("Target")
                .flyout_with(Flyout::text("Second").placement(FlyoutPlacement::Bottom)),
        )
        .unwrap();
    assert_eq!(
        runtime.graph().flyout(target),
        Some((content, FlyoutPlacement::Bottom))
    );
    assert!(
        !mutations
            .iter()
            .any(|mutation| matches!(mutation, Mutation::SetFlyout { .. }))
    );

    let mutations = runtime.update(Button::new().content("Target")).unwrap();
    let clear = mutations
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::SetFlyout {
                    target: current,
                    content: None,
                    ..
                } if *current == target
            )
        })
        .unwrap();
    let destroy = mutations
        .iter()
        .position(|mutation| matches!(mutation, Mutation::Destroy { object } if *object == content))
        .unwrap();
    assert!(clear < destroy);
    assert_eq!(runtime.adapter().flyout(target), None);
}

#[test]
fn flyout_rejects_unsupported_targets() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    assert_eq!(
        runtime.update(TextBlock::new().text("Target").flyout("Content")),
        Err(UpdateError::Graph(GraphError::InvalidAttachment(
            ObjectType::TextBlock
        )))
    );
}

#[test]
fn flyout_target_replacement_detaches_before_replace_and_reattaches() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Border::new().content(Button::new().content("Button").flyout("Content")))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let target = runtime.graph().child(root, RelationId::Content).unwrap();
    let content = runtime.graph().flyout(target).unwrap().0;

    let mutations = runtime
        .update_subtree(
            target,
            SplitButton::new().content("Split").flyout("Content"),
        )
        .unwrap();
    let clear = mutations
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::SetFlyout {
                    target: current,
                    content: None,
                    ..
                } if *current == target
            )
        })
        .unwrap();
    let replace = mutations
        .iter()
        .position(
            |mutation| matches!(mutation, Mutation::Replace { object, .. } if *object == target),
        )
        .unwrap();
    let attach = mutations
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::SetFlyout {
                    target: current_target,
                    content: Some(current_content),
                    ..
                } if *current_target == target && *current_content == content
            )
        })
        .unwrap();
    assert!(clear < replace);
    assert!(replace < attach);
}

#[test]
fn menu_attachment_routes_nested_keys_and_suppresses_stale_clicks() {
    let clicked = Rc::new(RefCell::new(Vec::new()));
    let capture = Rc::clone(&clicked);
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Button::new().content("Open").menu(Menu::new(
            [
                MenuItem::item("new", "New"),
                MenuItem::separator("separator"),
                MenuItem::submenu("share", "Share", [MenuItem::item("email", "Email")]),
            ],
            move |key| capture.borrow_mut().push(key),
        )))
        .unwrap();
    let target = runtime.graph().root().unwrap();
    assert_eq!(runtime.adapter().menu(target).unwrap().items.len(), 3);

    runtime.adapter_mut().queue_menu_click(target, "email");
    assert_eq!(runtime.dispatch_native_events(), Ok(1));
    assert_eq!(&*clicked.borrow(), &[Key::from("email")]);

    let mutations = runtime.update(Button::new().content("Open")).unwrap();
    assert!(mutations.iter().any(|mutation| {
        matches!(
            mutation,
            Mutation::SetMenu {
                target: current,
                menu: None,
                ..
            } if *current == target
        )
    }));
    assert_eq!(&*clicked.borrow(), &[Key::from("email")]);
}

#[test]
fn menu_attachment_supports_drop_down_and_menu_bar_targets() {
    let mut drop_down = Runtime::new(RecordingAdapter::default());
    drop_down
        .update(
            DropDownButton::new()
                .content("Open")
                .menu(Menu::new([MenuItem::item("open", "Open")], |_| {})),
        )
        .unwrap();
    assert!(
        drop_down
            .adapter()
            .menu(drop_down.graph().root().unwrap())
            .is_some()
    );

    let mut menu_bar = Runtime::new(RecordingAdapter::default());
    menu_bar
        .update(
            MenuBarItem::new()
                .title("File")
                .menu(Menu::new([MenuItem::item("exit", "Exit")], |_| {})),
        )
        .unwrap();
    assert!(
        menu_bar
            .adapter()
            .menu(menu_bar.graph().root().unwrap())
            .is_some()
    );
}

#[test]
fn menu_attachment_rejects_unsupported_targets_and_flyout_conflicts() {
    let mut unsupported = Runtime::new(RecordingAdapter::default());
    assert_eq!(
        unsupported.update(
            TextBlock::new()
                .text("Target")
                .menu(Menu::new([MenuItem::item("open", "Open")], |_| {}))
        ),
        Err(UpdateError::Graph(GraphError::InvalidAttachment(
            ObjectType::TextBlock
        )))
    );

    let mut conflict = Runtime::new(RecordingAdapter::default());
    assert_eq!(
        conflict.update(
            Button::new()
                .menu(Menu::new([MenuItem::item("open", "Open")], |_| {}))
                .flyout("Flyout")
        ),
        Err(UpdateError::Graph(GraphError::InvalidAttachment(
            ObjectType::Button
        )))
    );

    let mut duplicate = Runtime::new(RecordingAdapter::default());
    assert_eq!(
        duplicate.update(Button::new().menu(Menu::new(
            [
                MenuItem::item("duplicate", "First"),
                MenuItem::submenu(
                    "submenu",
                    "Submenu",
                    [MenuItem::item("duplicate", "Second")],
                ),
            ],
            |_| {},
        ))),
        Err(UpdateError::Graph(GraphError::DuplicateKey(Key::from(
            "duplicate"
        ))))
    );
}

#[test]
fn command_bar_owned_commands_route_keys_through_generated_children() {
    let clicked = Rc::new(RefCell::new(Vec::new()));
    let capture = Rc::clone(&clicked);
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(CommandBar::new().owned_commands(
            [
                CommandBarCommand::button_with_icon("save", "Save", Symbol::Save),
                CommandBarCommand::separator("separator"),
            ],
            [CommandBarCommand::disabled("print", "Print")],
            move |key| capture.borrow_mut().push(key),
        ))
        .unwrap();
    let command_bar = runtime.graph().root().unwrap();
    let save = runtime
        .graph()
        .children(command_bar, RelationId::PrimaryCommands)
        .unwrap()[0];
    assert_eq!(runtime.graph().kind(save), Some(ObjectType::AppBarButton));
    let EventValue::Unit(callback) = runtime.graph().events(save).unwrap()[0].value.clone() else {
        panic!();
    };
    runtime.adapter_mut().queue_event(EventDispatch::new(
        save,
        EventId::Click,
        EventValue::Unit(callback),
        EventPayload::Unit,
    ));
    assert_eq!(runtime.dispatch_native_events(), Ok(1));
    assert_eq!(&*clicked.borrow(), &[Key::from("save")]);
}

#[test]
fn command_bar_flyout_routes_keys_and_clears_on_removal() {
    let clicked = Rc::new(RefCell::new(Vec::new()));
    let capture = Rc::clone(&clicked);
    let first = CommandBarFlyout::new(
        [CommandBarCommand::button_with_icon(
            "bold",
            "Bold",
            Symbol::Bold,
        )],
        [
            CommandBarCommand::separator("separator"),
            CommandBarCommand::button("copy", "Copy"),
        ],
        move |key| capture.borrow_mut().push(key),
    );
    let first_callback = first.on_click.clone();
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Button::new().content("Format").command_bar_flyout(first))
        .unwrap();
    let target = runtime.graph().root().unwrap();
    assert_eq!(
        runtime
            .adapter()
            .command_bar_flyout(target)
            .unwrap()
            .secondary
            .len(),
        2
    );
    runtime
        .adapter_mut()
        .queue_command_bar_click(target, "copy");
    assert_eq!(runtime.dispatch_native_events(), Ok(1));
    assert_eq!(&*clicked.borrow(), &[Key::from("copy")]);

    let replacement_clicked = Rc::new(RefCell::new(Vec::new()));
    let replacement_capture = Rc::clone(&replacement_clicked);
    runtime
        .update(
            Button::new()
                .content("Format")
                .command_bar_flyout(CommandBarFlyout::new(
                    [CommandBarCommand::button("paste", "Paste")],
                    [],
                    move |key| replacement_capture.borrow_mut().push(key),
                )),
        )
        .unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        target,
        EventId::CommandInvoked,
        EventValue::Key(first_callback),
        EventPayload::Key(Key::from("copy")),
    ));
    runtime
        .adapter_mut()
        .queue_command_bar_click(target, "paste");
    assert_eq!(runtime.dispatch_native_events(), Ok(1));
    assert_eq!(&*replacement_clicked.borrow(), &[Key::from("paste")]);

    let mutations = runtime.update(Button::new().content("Format")).unwrap();
    assert!(mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::SetCommandBarFlyout {
            target: current,
            flyout: None,
            ..
        } if *current == target
    )));
}

#[test]
fn command_bar_flyout_rejects_invalid_targets_conflicts_and_duplicate_keys() {
    let flyout = || CommandBarFlyout::new([CommandBarCommand::button("copy", "Copy")], [], |_| {});
    let mut unsupported = Runtime::new(RecordingAdapter::default());
    assert_eq!(
        unsupported.update(DropDownButton::new().command_bar_flyout(flyout())),
        Err(UpdateError::Graph(GraphError::InvalidAttachment(
            ObjectType::DropDownButton
        )))
    );

    let mut conflict = Runtime::new(RecordingAdapter::default());
    assert_eq!(
        conflict.update(Button::new().flyout("Flyout").command_bar_flyout(flyout())),
        Err(UpdateError::Graph(GraphError::InvalidAttachment(
            ObjectType::Button
        )))
    );

    let mut duplicate = Runtime::new(RecordingAdapter::default());
    assert_eq!(
        duplicate.update(Button::new().command_bar_flyout(CommandBarFlyout::new(
            [CommandBarCommand::button("duplicate", "First")],
            [CommandBarCommand::button("duplicate", "Second")],
            |_| {},
        ))),
        Err(UpdateError::Graph(GraphError::DuplicateKey(Key::from(
            "duplicate"
        ))))
    );
}

#[test]
fn tooltip_target_replacement_clears_before_replace_and_reattaches() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Border::new().content(TextBlock::new().text("Target").tooltip("Tip")))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let target = runtime.graph().child(root, RelationId::Content).unwrap();
    let tooltip = runtime.graph().tooltip(target).unwrap();

    let mutations = runtime
        .update_subtree(target, Button::new().content("Target").tooltip("Tip"))
        .unwrap();
    let clear = mutations
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::SetTooltip {
                    target: current,
                    tooltip: None,
                    ..
                } if *current == target
            )
        })
        .unwrap();
    let replace = mutations
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::Replace { object, .. } if *object == target
            )
        })
        .unwrap();
    let attach = mutations
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::SetTooltip {
                    target: current_target,
                    tooltip: Some(current_tooltip),
                    ..
                } if *current_target == target && *current_tooltip == tooltip
            )
        })
        .unwrap();

    assert!(clear < replace);
    assert!(replace < attach);
    assert_eq!(
        runtime.adapter().tooltip(target),
        Some((tooltip, TooltipPlacement::Top))
    );
}

#[test]
fn recording_adapter_enforces_tooltip_ownership_transactionally() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([
            keyed("first", Button::new().tooltip("Tip")),
            keyed("second", Border::new()),
        ]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap();
    let first = children[0];
    let second = children[1];
    let tooltip = runtime.graph().tooltip(first).unwrap();

    assert_eq!(
        runtime.adapter_mut().apply(&[Mutation::SetTooltip {
            target: second,
            tooltip: Some(tooltip),
            placement: TooltipPlacement::Top,
        }]),
        Err(AdapterError::AlreadyOwned(tooltip))
    );
    assert_eq!(
        runtime.adapter().tooltip(first),
        Some((tooltip, TooltipPlacement::Top))
    );
    assert_eq!(runtime.adapter().tooltip(second), None);

    assert_eq!(
        runtime.adapter_mut().apply(&[Mutation::Attach {
            parent: second,
            relation: RelationId::Content,
            child: tooltip,
        }]),
        Err(AdapterError::AlreadyOwned(tooltip))
    );
    assert_eq!(
        runtime
            .adapter_mut()
            .apply(&[Mutation::Destroy { object: first }]),
        Err(AdapterError::StillOwned(first))
    );
    assert_eq!(
        runtime
            .adapter_mut()
            .apply(&[Mutation::Destroy { object: tooltip }]),
        Err(AdapterError::StillOwned(tooltip))
    );
    assert_eq!(
        runtime.adapter_mut().apply(&[Mutation::Replace {
            object: first,
            kind: ObjectType::Border,
        }]),
        Err(AdapterError::StillOwned(first))
    );

    runtime
        .adapter_mut()
        .apply(&[
            Mutation::SetTooltip {
                target: first,
                tooltip: None,
                placement: TooltipPlacement::Top,
            },
            Mutation::Remove {
                parent: root,
                relation: RelationId::Children,
                child: first,
                index: 0,
            },
            Mutation::Destroy { object: first },
        ])
        .unwrap();
}

#[test]
fn tooltip_clear_precedes_destruction() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TextBlock::new().text("Target").tooltip("Tip"))
        .unwrap();
    let target = runtime.graph().root().unwrap();
    let tooltip = runtime.graph().tooltip(target).unwrap();

    let mutations = runtime.update(TextBlock::new().text("Target")).unwrap();
    let clear = mutations
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::SetTooltip {
                    target: current_target,
                    tooltip: None,
                    ..
                } if *current_target == target
            )
        })
        .unwrap();
    let destroy = mutations
        .iter()
        .position(|mutation| matches!(mutation, Mutation::Destroy { object } if *object == tooltip))
        .unwrap();

    assert!(clear < destroy);
    assert_eq!(runtime.graph().tooltip(target), None);
    assert_eq!(runtime.adapter().tooltip(target), None);
}

fn content_dialog(open: bool) -> ContentDialog {
    ContentDialog::new()
        .title("Question")
        .primary_button_text("Yes")
        .secondary_button_text("No")
        .content(TextBlock::new().text("Choose"))
        .is_open(open)
}

#[test]
fn content_dialog_mounts_closed_as_an_attachment() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    let mutations = runtime
        .update(
            Button::new()
                .content("Owner")
                .content_dialog(content_dialog(false)),
        )
        .unwrap();
    let owner = runtime.graph().root().unwrap();
    let (dialog, open) = runtime.graph().content_dialog(owner).unwrap();

    assert!(!open);
    assert_eq!(
        runtime.adapter().content_dialog(owner),
        Some((dialog, false))
    );
    assert_eq!(
        runtime.graph().kind(dialog),
        Some(ObjectType::ContentDialog)
    );
    assert!(!mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::Attach {
            parent,
            child,
            ..
        } if *parent == owner && *child == dialog
    )));
    assert!(!mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::SetContentDialogOpen { dialog: current, .. } if *current == dialog
    )));
}

#[test]
fn content_dialog_opens_once_and_stays_stable() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Button::new().content_dialog(content_dialog(true)))
        .unwrap();
    let owner = runtime.graph().root().unwrap();
    let dialog = runtime.graph().content_dialog(owner).unwrap().0;
    assert_eq!(
        runtime.adapter().content_dialog_state(dialog),
        Some(RecordedContentDialog {
            desired_open: true,
            pending: true,
            show_count: 1,
            ..Default::default()
        })
    );

    let mutations = runtime
        .update(Button::new().content_dialog(content_dialog(true)))
        .unwrap();
    assert!(mutations.iter().all(|mutation| !matches!(
        mutation,
        Mutation::SetContentDialog { .. } | Mutation::SetContentDialogOpen { .. }
    )));
    assert_eq!(
        runtime
            .adapter()
            .content_dialog_state(dialog)
            .unwrap()
            .show_count,
        1
    );
}

#[test]
fn content_dialog_declarative_hide_suppresses_close_and_reopen_waits() {
    let calls = Rc::new(Cell::new(0));
    let view = |open| {
        let callback_calls = Rc::clone(&calls);
        Button::new().content_dialog(content_dialog(open).on_closed(move |_| {
            callback_calls.set(callback_calls.get() + 1);
        }))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(view(true)).unwrap();
    let owner = runtime.graph().root().unwrap();
    let dialog = runtime.graph().content_dialog(owner).unwrap().0;

    let mutations = runtime.update(view(false)).unwrap();
    assert!(mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::SetContentDialogOpen {
            dialog: current,
            open: false
        } if *current == dialog
    )));
    let state = runtime.adapter().content_dialog_state(dialog).unwrap();
    assert!(state.pending);
    assert_eq!(state.hide_count, 1);
    assert!(runtime.complete_content_dialog(dialog, ContentDialogResult::None));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(calls.get(), 0);

    runtime.update(view(true)).unwrap();
    runtime.update(view(false)).unwrap();
    runtime.update(view(true)).unwrap();
    assert!(
        runtime
            .adapter()
            .content_dialog_state(dialog)
            .unwrap()
            .queued
    );
    assert!(runtime.complete_content_dialog(dialog, ContentDialogResult::None));
    let state = runtime.adapter().content_dialog_state(dialog).unwrap();
    assert!(state.pending);
    assert!(!state.queued);
    assert_eq!(state.show_count, 3);
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
}

#[test]
fn content_dialog_closed_dispatches_typed_result() {
    let result = Rc::new(Cell::new(ContentDialogResult::None));
    let callback_result = Rc::clone(&result);
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Button::new().content_dialog(
            content_dialog(true).on_closed(move |value| callback_result.set(value)),
        ))
        .unwrap();
    let owner = runtime.graph().root().unwrap();
    let dialog = runtime.graph().content_dialog(owner).unwrap().0;

    assert!(runtime.complete_content_dialog(dialog, ContentDialogResult::Secondary));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(result.get(), ContentDialogResult::Secondary);
}

#[test]
fn content_dialog_updates_preserve_attachment_identity() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Button::new().content_dialog(
                ContentDialog::new()
                    .title("First")
                    .content(TextBlock::new().text("First"))
                    .is_open(true),
            ),
        )
        .unwrap();
    let owner = runtime.graph().root().unwrap();
    let dialog = runtime.graph().content_dialog(owner).unwrap().0;
    let content = runtime.graph().child(dialog, RelationId::Content).unwrap();

    let mutations = runtime
        .update(
            Button::new().content_dialog(
                ContentDialog::new()
                    .title("Second")
                    .content(TextBlock::new().text("Second"))
                    .is_open(true),
            ),
        )
        .unwrap();

    assert_eq!(runtime.graph().content_dialog(owner), Some((dialog, true)));
    assert_eq!(
        runtime.graph().child(dialog, RelationId::Content),
        Some(content)
    );
    assert!(mutations.iter().all(|mutation| !matches!(
        mutation,
        Mutation::SetContentDialog { .. } | Mutation::SetContentDialogOpen { .. }
    )));
}

#[test]
fn content_dialog_removal_hides_clears_and_destroys_before_owner() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([keyed(
            "owner",
            Button::new().content_dialog(content_dialog(true)),
        )]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let owner = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    let dialog = runtime.graph().content_dialog(owner).unwrap().0;

    let mutations = runtime.update(Grid::new()).unwrap();
    let hide = mutations
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::SetContentDialogOpen {
                    dialog: current,
                    open: false
                } if *current == dialog
            )
        })
        .unwrap();
    let clear = mutations
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::SetContentDialog {
                    owner: current,
                    dialog: None
                } if *current == owner
            )
        })
        .unwrap();
    let destroy_dialog = mutations
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::Destroy { object } if *object == dialog
            )
        })
        .unwrap();
    let destroy_owner = mutations
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::Destroy { object } if *object == owner
            )
        })
        .unwrap();
    assert!(hide < clear && clear < destroy_dialog && destroy_dialog < destroy_owner);
}

#[test]
fn content_dialog_owner_replacement_reuses_dialog_in_order() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Border::new().content(
                TextBlock::new()
                    .text("Owner")
                    .content_dialog(content_dialog(true)),
            ),
        )
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let owner = runtime.graph().child(root, RelationId::Content).unwrap();
    let dialog = runtime.graph().content_dialog(owner).unwrap().0;

    let mutations = runtime
        .update_subtree(owner, Button::new().content_dialog(content_dialog(true)))
        .unwrap();
    let position = |predicate: &dyn Fn(&Mutation) -> bool| {
        mutations
            .iter()
            .position(predicate)
            .unwrap_or_else(|| panic!("missing mutation in {mutations:#?}"))
    };
    let hide = position(&|mutation| {
        matches!(
            mutation,
            Mutation::SetContentDialogOpen {
                dialog: current,
                open: false
            } if *current == dialog
        )
    });
    let clear = position(&|mutation| {
        matches!(
            mutation,
            Mutation::SetContentDialog {
                owner: current,
                dialog: None
            } if *current == owner
        )
    });
    let replace = position(&|mutation| {
        matches!(
            mutation,
            Mutation::Replace { object, .. } if *object == owner
        )
    });
    let attach = position(&|mutation| {
        matches!(
            mutation,
            Mutation::SetContentDialog {
                owner: current_owner,
                dialog: Some(current_dialog)
            } if *current_owner == owner && *current_dialog == dialog
        )
    });
    let open = position(&|mutation| {
        matches!(
            mutation,
            Mutation::SetContentDialogOpen {
                dialog: current,
                open: true
            } if *current == dialog
        )
    });
    assert!(hide < clear && clear < replace && replace < attach && attach < open);
    assert_eq!(runtime.graph().content_dialog(owner), Some((dialog, true)));
}

#[test]
fn content_dialog_protocol_and_nesting_are_rejected() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([
            keyed("first", Button::new().content_dialog(content_dialog(true))),
            keyed("second", Button::new()),
        ]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap();
    let first = children[0];
    let second = children[1];
    let dialog = runtime.graph().content_dialog(first).unwrap().0;
    assert_eq!(
        runtime.adapter_mut().apply(&[Mutation::SetContentDialog {
            owner: second,
            dialog: Some(dialog),
        }]),
        Err(AdapterError::AlreadyOwned(dialog))
    );
    assert_eq!(
        runtime.adapter_mut().apply(&[Mutation::SetContentDialog {
            owner: first,
            dialog: None,
        }]),
        Err(AdapterError::InvalidContentDialog(first))
    );

    let nested = Button::new().content_dialog(
        ContentDialog::new().content(Border::new().content_dialog(ContentDialog::new())),
    );
    assert_eq!(
        Runtime::new(RecordingAdapter::default()).update(nested),
        Err(UpdateError::Graph(GraphError::InvalidAttachment(
            ObjectType::ContentDialog
        )))
    );
    let tooltip_nested = Button::new().tooltip_with(Tooltip::rich(
        Border::new().content_dialog(ContentDialog::new()),
    ));
    assert_eq!(
        Runtime::new(RecordingAdapter::default()).update(tooltip_nested),
        Err(UpdateError::Graph(GraphError::InvalidAttachment(
            ObjectType::ContentDialog
        )))
    );
    Runtime::new(RecordingAdapter::default())
        .update(
            Button::new()
                .content_dialog(ContentDialog::new().content(Border::new().tooltip("Allowed"))),
        )
        .unwrap();
    let direct = View(DeclaredNode::Object(Declaration::new(
        ObjectType::ContentDialog,
    )));
    assert_eq!(
        Runtime::new(RecordingAdapter::default()).update(direct),
        Err(UpdateError::Graph(GraphError::InvalidAttachment(
            ObjectType::ContentDialog
        )))
    );
}

#[test]
fn stale_imperative_completion_reports_unavailable_after_rebinding() {
    let commands = Rc::new(RefCell::new(Vec::new()));
    let reference = ElementRef::<Image>::new();
    let replacement = ElementRef::<Image>::new();
    let results = Rc::new(RefCell::new(Vec::new()));
    let mut runtime = Runtime::new(HoldingImperativeAdapter {
        inner: RecordingAdapter::default(),
        commands: Rc::clone(&commands),
    });
    runtime
        .update(Image::new().element_ref(&reference))
        .unwrap();
    let callback_results = Rc::clone(&results);
    assert!(reference.request_set_native_source(None, move |result| {
        callback_results.borrow_mut().push(result);
    }));
    runtime.dispatch_native_events().unwrap();
    runtime
        .update(Image::new().element_ref(&replacement))
        .unwrap();

    let ImperativeRequest::SetNativeImageSource { completion, .. } =
        commands.borrow_mut().remove(0)
    else {
        panic!("expected image-source request");
    };
    completion.call(Ok(()));

    assert_eq!(
        results.borrow().as_slice(),
        [Err(IntegrationError::Unavailable)]
    );
    assert_eq!(reference.get(), None);
    assert!(replacement.get().is_some());
}

#[test]
fn observations_follow_reference_rebinding_and_drop() {
    let commands = Rc::new(RefCell::new(Vec::new()));
    let reference = ElementRef::<Grid>::new();
    let observation = reference.observe_composition_host(|_| {});
    let mut runtime = Runtime::new(HoldingImperativeAdapter {
        inner: RecordingAdapter::default(),
        commands: Rc::clone(&commands),
    });

    runtime.update(Grid::new().element_ref(&reference)).unwrap();
    runtime.dispatch_native_events().unwrap();
    assert!(matches!(
        commands.borrow().last(),
        Some(ImperativeRequest::ObserveCompositionHost { .. })
    ));

    runtime.update(Grid::new()).unwrap();
    runtime.dispatch_native_events().unwrap();
    assert!(matches!(
        commands.borrow().last(),
        Some(ImperativeRequest::RevokeObservation { .. })
    ));

    runtime.update(Grid::new().element_ref(&reference)).unwrap();
    runtime.dispatch_native_events().unwrap();
    assert!(matches!(
        commands.borrow().last(),
        Some(ImperativeRequest::ObserveCompositionHost { .. })
    ));

    drop(observation);
    runtime.dispatch_native_events().unwrap();
    assert!(matches!(
        commands.borrow().last(),
        Some(ImperativeRequest::RevokeObservation { .. })
    ));
}

#[test]
fn observation_lifecycle_survives_imperative_queue_saturation() {
    let commands = Rc::new(RefCell::new(Vec::new()));
    let reference = ElementRef::<Grid>::new();
    let mut runtime = Runtime::new(HoldingImperativeAdapter {
        inner: RecordingAdapter::default(),
        commands: Rc::clone(&commands),
    });
    runtime.update(Grid::new().element_ref(&reference)).unwrap();

    for _ in 0..IMPERATIVE_QUEUE_CAPACITY {
        assert!(reference.request_focus());
    }
    assert!(!reference.request_focus());
    let observation = reference.observe_composition_host(|_| {});
    for _ in 0..=IMPERATIVE_QUEUE_CAPACITY / 64 {
        runtime.dispatch_native_events().unwrap();
    }
    assert!(matches!(
        commands.borrow().last(),
        Some(ImperativeRequest::ObserveCompositionHost { .. })
    ));
    commands.borrow_mut().clear();

    for _ in 0..IMPERATIVE_QUEUE_CAPACITY {
        assert!(reference.request_focus());
    }
    runtime.update(Grid::new()).unwrap();
    for _ in 0..=IMPERATIVE_QUEUE_CAPACITY / 64 {
        runtime.dispatch_native_events().unwrap();
    }
    assert!(matches!(
        commands.borrow().last(),
        Some(ImperativeRequest::RevokeObservation { .. })
    ));
    commands.borrow_mut().clear();

    runtime.update(Grid::new().element_ref(&reference)).unwrap();
    runtime.dispatch_native_events().unwrap();
    commands.borrow_mut().clear();
    for _ in 0..IMPERATIVE_QUEUE_CAPACITY {
        assert!(reference.request_focus());
    }
    drop(observation);
    for _ in 0..=IMPERATIVE_QUEUE_CAPACITY / 64 {
        runtime.dispatch_native_events().unwrap();
    }
    assert!(matches!(
        commands.borrow().last(),
        Some(ImperativeRequest::RevokeObservation { .. })
    ));
}

#[test]
fn destroying_observed_object_discards_queued_revocation() {
    let reference = ElementRef::<Grid>::new();
    let _observation = reference.observe_composition_host(|_| {});
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Grid::new().keyed_children([keyed("observed", Grid::new().element_ref(&reference))]),
        )
        .unwrap();
    runtime.dispatch_native_events().unwrap();

    runtime.update(Grid::new()).unwrap();

    assert_eq!(reference.get(), None);
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
}

#[test]
fn dropping_runtime_clears_typed_reference_and_pending_completion() {
    let reference = ElementRef::<SwapChainPanel>::new();
    let results = Rc::new(RefCell::new(Vec::new()));
    {
        let mut runtime = Runtime::new(RecordingAdapter::default());
        runtime
            .update(SwapChainPanel::new().element_ref(&reference))
            .unwrap();
        let callback_results = Rc::clone(&results);
        assert!(reference.request_clear_swap_chain(move |result| {
            callback_results.borrow_mut().push(result);
        }));
    }
    assert_eq!(reference.get(), None);
    assert_eq!(
        results.borrow().as_slice(),
        [Err(IntegrationError::Unavailable)]
    );
}

#[test]
fn imperative_budget_rearms_pending_work() {
    let reference = ElementRef::<Image>::new();
    let wakes = Rc::new(Cell::new(0));
    let wake_count = Rc::clone(&wakes);
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.set_imperative_waker(move || wake_count.set(wake_count.get() + 1));
    runtime
        .update(Image::new().element_ref(&reference))
        .unwrap();
    for _ in 0..65 {
        assert!(reference.request_set_native_source(None, |_| {}));
    }
    wakes.set(0);

    runtime.dispatch_native_events().unwrap();

    assert_eq!(wakes.get(), 1);
    runtime.dispatch_native_events().unwrap();
    assert_eq!(runtime.adapter().imperatives().len(), 65);
}

#[test]
fn reference_transfers_are_order_independent() {
    let reference: ElementRef = ElementRef::default();
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(StackPanel::new().children([
            Button::new().into(),
            Button::new().element_ref(&reference).into(),
        ]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()
        .to_vec();
    assert_eq!(reference.get(), Some(children[1]));

    assert!(
        runtime
            .update(StackPanel::new().children([
                Button::new().element_ref(&reference).into(),
                Button::new().into(),
            ]),)
            .unwrap()
            .is_empty()
    );
    assert_eq!(reference.get(), Some(children[0]));

    assert!(
        runtime
            .update(StackPanel::new().children([
                Button::new().into(),
                Button::new().element_ref(&reference).into(),
            ]),)
            .unwrap()
            .is_empty()
    );
    assert_eq!(reference.get(), Some(children[1]));
}

#[test]
fn references_survive_keyed_replacement_and_clear_on_destruction() {
    let reference: ElementRef = ElementRef::default();
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Grid::new().keyed_children([keyed("focused", Button::new().element_ref(&reference))]),
        )
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let child = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    assert_eq!(reference.get(), Some(child));

    runtime
        .update(Grid::new().keyed_children([keyed(
            "focused",
            TextBox::new("replacement").element_ref(&reference),
        )]))
        .unwrap();
    let replacement = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    assert_ne!(replacement, child);
    assert_eq!(reference.get(), Some(replacement));
    assert_eq!(runtime.graph().kind(replacement), Some(ObjectType::TextBox));

    runtime.update(Grid::new()).unwrap();
    assert_eq!(reference.get(), None);
}

#[test]
fn duplicate_references_are_rejected_before_reconciliation() {
    let reference: ElementRef = ElementRef::default();
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(StackPanel::new().children([Button::new().into()]))
        .unwrap();
    let root = runtime.graph().root().unwrap();

    assert_eq!(
        runtime.update(StackPanel::new().children([
            Button::new().element_ref(&reference).into(),
            Button::new().element_ref(&reference).into(),
        ]),),
        Err(UpdateError::Graph(GraphError::DuplicateReference))
    );
    assert_eq!(reference.get(), None);
    assert_eq!(runtime.graph().root(), Some(root));
    assert_eq!(
        runtime
            .graph()
            .children(root, RelationId::Children)
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn exit_retirement_leaves_only_native_state_until_completion() {
    let reference: ElementRef = ElementRef::default();
    let calls = Rc::new(Cell::new(0));
    let callback = {
        let calls = Rc::clone(&calls);
        Callback::new(move |()| calls.set(calls.get() + 1))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Grid::new().keyed_children([
                keyed(
                    "item",
                    Button::new()
                        .element_ref(&reference)
                        .exit_fade(std::time::Duration::from_millis(200))
                        .on_click(callback.clone())
                        .content(TextBlock::new().text("old")),
                ),
                keyed("tail", TextBlock::new().text("tail")),
            ]),
        )
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let old = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    let old_content = runtime.graph().child(old, RelationId::Content).unwrap();
    for _ in 0..2 {
        runtime.adapter_mut().queue_event(EventDispatch::new(
            old,
            EventId::Click,
            EventValue::Unit(callback.clone()),
            EventPayload::Unit,
        ));
    }
    let mut active = runtime.next_native_event().unwrap().unwrap();
    active.invoke();
    runtime.adapter_mut().record_batches(true);

    let mutations = runtime
        .update(Grid::new().keyed_children([keyed("tail", TextBlock::new().text("tail"))]))
        .unwrap();
    drop(active);
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(calls.get(), 1);
    runtime
        .update(Grid::new().keyed_children([
            keyed("item", Button::new().content(TextBlock::new().text("new"))),
            keyed("tail", TextBlock::new().text("tail")),
        ]))
        .unwrap();
    let replacement = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    assert_ne!(replacement, old);
    assert_eq!(runtime.graph().kind(old), None);
    assert_eq!(runtime.graph().kind(old_content), None);
    assert_eq!(runtime.graph().retired_count(), 1);
    assert_eq!(reference.get(), None);
    assert!(matches!(
        runtime.focus(&reference),
        Err(UpdateError::Graph(GraphError::ReferenceUnavailable))
    ));
    assert!(
        mutations
            .iter()
            .any(|mutation| matches!(mutation, Mutation::Retire { root, .. } if *root == old))
    );
    assert!(
        !mutations
            .iter()
            .any(|mutation| matches!(mutation, Mutation::Remove { child, .. } if *child == old))
    );
    assert!(
        !mutations
            .iter()
            .any(|mutation| matches!(mutation, Mutation::Destroy { object } if *object == old))
    );
    assert_eq!(mutations.len(), 2);
    assert_eq!(
        runtime
            .adapter()
            .children(root, RelationId::Children)
            .unwrap(),
        &[
            old,
            replacement,
            runtime
                .graph()
                .children(root, RelationId::Children)
                .unwrap()[1]
        ]
    );
    let tail = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[1];
    runtime
        .update(Grid::new().keyed_children([
            keyed("tail", TextBlock::new().text("tail")),
            keyed("item", Button::new().content(TextBlock::new().text("new"))),
        ]))
        .unwrap();
    assert_eq!(
        runtime
            .adapter()
            .children(root, RelationId::Children)
            .unwrap(),
        &[old, tail, replacement]
    );

    assert!(runtime.adapter_mut().complete_retirement(old));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert!(matches!(
        runtime.adapter().batches().last().unwrap().as_slice(),
        [Mutation::CompleteRetirement { root, .. }] if *root == old
    ));
    assert_eq!(runtime.graph().retired_count(), 0);
    assert_eq!(runtime.adapter().retirement_count(), 0);
    assert_eq!(
        runtime.adapter().object_count(),
        runtime.graph().object_count()
    );
    assert!(!runtime.adapter_mut().complete_retirement(old));
}

#[test]
fn owned_reorder_preserves_interleaved_retirement_slot_in_recording_adapter() {
    let children = |include_retiring: bool, reversed: bool| {
        let mut values = if reversed {
            vec![
                keyed("d", TextBlock::new().text("d")),
                keyed("c", TextBlock::new().text("c")),
                keyed("b", TextBlock::new().text("b")),
                keyed("a", TextBlock::new().text("a")),
            ]
        } else {
            vec![
                keyed("a", TextBlock::new().text("a")),
                keyed("b", TextBlock::new().text("b")),
                keyed("c", TextBlock::new().text("c")),
                keyed("d", TextBlock::new().text("d")),
            ]
        };
        if include_retiring {
            values.insert(
                2,
                keyed(
                    "retiring",
                    TextBlock::new()
                        .text("retiring")
                        .exit_fade(std::time::Duration::from_secs(1)),
                ),
            );
        }
        Grid::new().keyed_children(values)
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(children(true, false)).unwrap();
    let root = runtime.graph().root().unwrap();
    let initial = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()
        .to_vec();
    let retiring = initial[2];
    let active = [initial[4], initial[3], initial[1], initial[0]];

    runtime.update(children(false, true)).unwrap();

    assert_eq!(
        runtime
            .adapter()
            .children(root, RelationId::Children)
            .unwrap(),
        &[active[0], active[1], retiring, active[2], active[3]]
    );
    assert!(runtime.adapter_mut().complete_retirement(retiring));
    runtime.dispatch_native_events().unwrap();
    assert_eq!(
        runtime
            .adapter()
            .children(root, RelationId::Children)
            .unwrap(),
        active
    );
}

#[test]
fn exit_retirement_handles_concurrency_zero_duration_and_parent_removal() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([
            keyed(
                "first",
                Button::new().exit_fade(std::time::Duration::from_millis(100)),
            ),
            keyed(
                "second",
                Button::new().exit_fade(std::time::Duration::from_millis(200)),
            ),
            keyed("tail", TextBlock::new().text("tail")),
        ]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let previous = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()
        .to_vec();

    runtime
        .update(Grid::new().keyed_children([keyed("tail", TextBlock::new().text("tail"))]))
        .unwrap();
    assert_eq!(runtime.graph().retired_count(), 2);
    assert!(runtime.adapter_mut().complete_retirement(previous[1]));
    runtime.dispatch_native_events().unwrap();
    assert_eq!(runtime.graph().retired_count(), 1);
    assert!(runtime.adapter_mut().complete_retirement(previous[0]));
    runtime.dispatch_native_events().unwrap();
    assert_eq!(runtime.graph().retired_count(), 0);

    runtime
        .update(Grid::new().keyed_children([keyed(
            "zero",
            Button::new().exit_fade(std::time::Duration::ZERO),
        )]))
        .unwrap();
    let zero = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    runtime.update(Grid::new()).unwrap();
    assert_eq!(runtime.graph().retired_count(), 0);
    assert_eq!(runtime.adapter().retirement_count(), 0);
    assert_eq!(runtime.graph().kind(zero), None);

    runtime
        .update(Grid::new().keyed_children([keyed(
            "parent",
            Grid::new().keyed_children([keyed(
                "child",
                Button::new().exit_fade(std::time::Duration::from_millis(200)),
            )]),
        )]))
        .unwrap();
    let parent = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    let child = runtime
        .graph()
        .children(parent, RelationId::Children)
        .unwrap()[0];
    runtime
        .update(Grid::new().keyed_children([keyed("parent", Grid::new())]))
        .unwrap();
    assert_eq!(runtime.graph().retired_count(), 1);
    runtime.update(Grid::new()).unwrap();
    assert_eq!(runtime.graph().retired_count(), 0);
    assert_eq!(runtime.adapter().retirement_count(), 0);
    assert_eq!(runtime.graph().kind(child), None);
    assert!(!runtime.adapter_mut().complete_retirement(child));
}

#[test]
fn exit_transition_rejects_single_child_attachment() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Button::new().content(
                TextBlock::new()
                    .text("old")
                    .exit_fade(std::time::Duration::from_millis(200)),
            ),
        )
        .unwrap();

    assert_eq!(
        runtime.update(Button::new()),
        Err(UpdateError::Graph(GraphError::ExitTransitionUnsupported))
    );
}

#[test]
fn unsupported_retirement_rolls_back_the_entire_update() {
    let retiring_reference: ElementRef = ElementRef::default();
    let nested_reference: ElementRef = ElementRef::default();
    let callback = Callback::new(|()| {});
    let initial = || {
        Grid::new().keyed_children([
            keyed(
                "retiring",
                Button::new()
                    .element_ref(&retiring_reference)
                    .exit_fade(std::time::Duration::from_millis(100))
                    .on_click(callback.clone()),
            ),
            keyed(
                "holder",
                Button::new().content(
                    Button::new()
                        .element_ref(&nested_reference)
                        .exit_fade(std::time::Duration::from_millis(100))
                        .on_click(callback.clone()),
                ),
            ),
        ])
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.adapter_mut().record_batches(true);
    runtime.update(initial()).unwrap();
    let graph = runtime.graph().clone();
    let adapter = runtime.adapter().clone();
    let retiring = retiring_reference.get();
    let nested = nested_reference.get();

    assert_eq!(
        runtime.update(Grid::new().keyed_children([keyed("holder", Button::new())])),
        Err(UpdateError::Graph(GraphError::ExitTransitionUnsupported))
    );
    assert_eq!(runtime.graph(), &graph);
    assert_eq!(runtime.adapter(), &adapter);
    assert_eq!(retiring_reference.get(), retiring);
    assert_eq!(nested_reference.get(), nested);
    assert_eq!(runtime.graph().retired_count(), 0);

    assert!(runtime.update(initial()).unwrap().is_empty());
    runtime
        .update(
            Grid::new().keyed_children([keyed(
                "holder",
                Button::new().content(
                    Button::new()
                        .element_ref(&nested_reference)
                        .exit_fade(std::time::Duration::from_millis(100))
                        .on_click(callback),
                ),
            )]),
        )
        .unwrap();
    assert_eq!(runtime.graph().retired_count(), 1);
}

#[test]
fn observations_survive_a_later_planning_failure() {
    let initial = || {
        Grid::new().keyed_children([
            keyed(
                "retiring",
                Button::new().exit_fade(std::time::Duration::from_millis(100)),
            ),
            keyed(
                "holder",
                Button::new().content(
                    TextBlock::new()
                        .text("nested")
                        .exit_fade(std::time::Duration::from_millis(100)),
                ),
            ),
        ])
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    let mut expected = Runtime::new(RecordingAdapter::default());
    runtime.update(initial()).unwrap();
    expected.update(initial()).unwrap();
    let root = runtime.graph().root().unwrap();
    let holder = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[1];
    let expected_root = expected.graph().root().unwrap();
    let expected_holder = expected
        .graph()
        .children(expected_root, RelationId::Children)
        .unwrap()[1];
    let observed = Property {
        id: PropertyId::IsEnabled,
        value: PropertyValue::Bool(false),
    };
    runtime.adapter_mut().observe(Observation::SetProperty {
        object: holder,
        property: observed.clone(),
    });
    expected.adapter_mut().observe(Observation::SetProperty {
        object: expected_holder,
        property: observed.clone(),
    });
    expected.dispatch_native_events().unwrap();

    assert_eq!(
        runtime
            .update(Grid::new().keyed_children([keyed("holder", Button::new().is_enabled(false))])),
        Err(UpdateError::Graph(GraphError::ExitTransitionUnsupported))
    );

    assert_eq!(runtime.graph(), expected.graph());
    assert_eq!(runtime.adapter(), expected.adapter());
    assert_eq!(runtime.graph().properties(holder).unwrap(), [observed]);
    assert!(runtime.update(initial()).unwrap().iter().any(
        |mutation| matches!(mutation, Mutation::SetProperties { object, .. } if *object == holder)
    ));
}

#[test]
fn allocation_from_the_free_list_rolls_back_generation_and_order() {
    let nested = || {
        TextBlock::new()
            .text("nested")
            .exit_fade(std::time::Duration::from_millis(100))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([
            keyed("spare", Button::new()),
            keyed("kept", Button::new()),
            keyed("holder", Button::new().content(nested())),
        ]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let spare = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    runtime
        .update(Grid::new().keyed_children([
            keyed("kept", Button::new()),
            keyed("holder", Button::new().content(nested())),
        ]))
        .unwrap();
    let before = runtime.graph().clone();

    assert_eq!(
        runtime.update(Grid::new().keyed_children([
            keyed("kept", Button::new()),
            keyed("new", Button::new()),
            keyed("holder", Button::new()),
        ])),
        Err(UpdateError::Graph(GraphError::ExitTransitionUnsupported))
    );
    assert_eq!(runtime.graph(), &before);

    runtime
        .update(Grid::new().keyed_children([
            keyed("kept", Button::new()),
            keyed("new", Button::new()),
            keyed("holder", Button::new().content(nested())),
        ]))
        .unwrap();
    let allocated = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[1];
    assert_eq!(allocated.index(), spare.index());
    assert_eq!(allocated.generation(), spare.generation().wrapping_add(1));
}

#[test]
fn removal_and_reallocation_roll_back_multiple_slot_generations() {
    let nested = || {
        TextBlock::new()
            .text("nested")
            .exit_fade(std::time::Duration::from_millis(100))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([
            keyed("first", Button::new()),
            keyed("second", Button::new()),
            keyed("holder", Button::new().content(nested())),
        ]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let previous = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()
        .to_vec();
    let before = runtime.graph().clone();

    assert_eq!(
        runtime.update(Grid::new().keyed_children([
            keyed("new-first", Button::new()),
            keyed("new-second", Button::new()),
            keyed("holder", Button::new()),
        ])),
        Err(UpdateError::Graph(GraphError::ExitTransitionUnsupported))
    );
    assert_eq!(runtime.graph(), &before);

    runtime
        .update(Grid::new().keyed_children([
            keyed("new-first", Button::new()),
            keyed("new-second", Button::new()),
            keyed("holder", Button::new().content(nested())),
        ]))
        .unwrap();
    let allocated = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap();
    for (allocated, previous) in allocated[..2].iter().zip(&previous[..2]) {
        assert_eq!(allocated.index(), previous.index());
        assert_eq!(
            allocated.generation(),
            previous.generation().wrapping_add(1)
        );
    }
}

#[test]
fn forced_retirement_completion_rolls_back_with_parent_removal() {
    let nested = || {
        TextBlock::new()
            .text("nested")
            .exit_fade(std::time::Duration::from_millis(100))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([
            keyed(
                "parent",
                Grid::new().keyed_children([keyed(
                    "retiring",
                    Button::new().exit_fade(std::time::Duration::from_millis(100)),
                )]),
            ),
            keyed("holder", Button::new().content(nested())),
        ]))
        .unwrap();
    runtime
        .update(Grid::new().keyed_children([
            keyed("parent", Grid::new()),
            keyed("holder", Button::new().content(nested())),
        ]))
        .unwrap();
    let before_graph = runtime.graph().clone();
    let before_adapter = runtime.adapter().clone();

    assert_eq!(
        runtime.update(Grid::new().keyed_children([keyed("holder", Button::new())])),
        Err(UpdateError::Graph(GraphError::ExitTransitionUnsupported))
    );
    assert_eq!(runtime.graph(), &before_graph);
    assert_eq!(runtime.adapter(), &before_adapter);
    assert_eq!(runtime.graph().retired_count(), 1);
}

#[test]
fn duplicate_retirement_completion_is_idempotent() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.adapter_mut().record_batches(true);
    runtime
        .update(Grid::new().keyed_children([keyed(
            "retiring",
            Button::new().exit_fade(std::time::Duration::from_millis(100)),
        )]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let retired = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    runtime.update(Grid::new()).unwrap();
    runtime.adapter_mut().queue_retirement_completion(retired);
    runtime.adapter_mut().queue_retirement_completion(retired);

    runtime.dispatch_native_events().unwrap();

    assert_eq!(runtime.graph().retired_count(), 0);
    assert_eq!(runtime.adapter().retirement_count(), 0);
    assert_eq!(
        runtime.adapter().batches().last().unwrap(),
        &[Mutation::CompleteRetirement {
            root: retired,
            nodes: vec![retired],
        }]
    );
}

#[test]
fn mixed_duplicate_and_stale_retirement_completions_are_idempotent() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.adapter_mut().record_batches(true);
    runtime
        .update(Grid::new().keyed_children([
            keyed(
                "first",
                Button::new().exit_fade(std::time::Duration::from_millis(100)),
            ),
            keyed(
                "second",
                Button::new().exit_fade(std::time::Duration::from_millis(100)),
            ),
        ]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let retired = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()
        .to_vec();
    runtime.update(Grid::new()).unwrap();
    let completion_batches = runtime.adapter().batches().len();

    for object in [retired[0], retired[0], retired[1], retired[1]] {
        runtime.adapter_mut().queue_retirement_completion(object);
    }
    runtime.dispatch_native_events().unwrap();
    assert_eq!(runtime.graph().retired_count(), 0);
    assert_eq!(runtime.adapter().retirement_count(), 0);
    let completed = runtime.adapter().batches()[completion_batches..]
        .iter()
        .flatten()
        .filter_map(|mutation| match mutation {
            Mutation::CompleteRetirement { root, .. } => Some(*root),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(completed, retired);

    runtime
        .update(Grid::new().keyed_children([keyed("first", Button::new())]))
        .unwrap();
    let replacement = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    assert_ne!(replacement, retired[0]);
    let batches = runtime.adapter().batches().len();
    runtime
        .adapter_mut()
        .queue_retirement_completion(retired[0]);
    runtime
        .adapter_mut()
        .queue_retirement_completion(retired[0]);
    runtime.dispatch_native_events().unwrap();
    assert_eq!(runtime.adapter().batches().len(), batches);
    assert_eq!(runtime.graph().kind(replacement), Some(ObjectType::Button));
}

#[test]
fn native_event_before_retirement_completion_keeps_chronological_order() {
    let calls = Rc::new(Cell::new(0));
    let callback = {
        let calls = Rc::clone(&calls);
        Callback::new(move |()| calls.set(calls.get() + 1))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([
            keyed(
                "retiring",
                Button::new().exit_fade(std::time::Duration::from_millis(100)),
            ),
            keyed("active", Button::new().on_click(callback.clone())),
        ]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()
        .to_vec();
    runtime
        .update(
            Grid::new().keyed_children([keyed("active", Button::new().on_click(callback.clone()))]),
        )
        .unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        children[1],
        EventId::Click,
        EventValue::Unit(callback),
        EventPayload::Unit,
    ));
    runtime
        .adapter_mut()
        .queue_retirement_completion(children[0]);

    let mut event = runtime.next_native_event().unwrap().unwrap();
    assert_eq!(runtime.graph().retired_count(), 1);
    event.invoke();
    drop(event);
    assert_eq!(calls.get(), 1);
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(runtime.graph().retired_count(), 0);
}

#[test]
fn retirement_completion_before_later_event_keeps_chronological_order() {
    let calls = Rc::new(Cell::new(0));
    let callback = {
        let calls = Rc::clone(&calls);
        Callback::new(move |()| calls.set(calls.get() + 1))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([
            keyed(
                "retiring",
                Button::new().exit_fade(std::time::Duration::from_millis(100)),
            ),
            keyed("active", Button::new().on_click(callback.clone())),
        ]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()
        .to_vec();
    runtime
        .update(
            Grid::new().keyed_children([keyed("active", Button::new().on_click(callback.clone()))]),
        )
        .unwrap();
    runtime
        .adapter_mut()
        .queue_retirement_completion(children[0]);
    runtime.adapter_mut().queue_event(EventDispatch::new(
        children[1],
        EventId::Click,
        EventValue::Unit(callback),
        EventPayload::Unit,
    ));

    let mut event = runtime.next_native_event().unwrap().unwrap();
    assert_eq!(runtime.graph().retired_count(), 0);
    event.invoke();
    drop(event);
    assert_eq!(calls.get(), 1);
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
}

#[test]
fn events_queued_after_logical_retirement_are_stale_before_completion() {
    let calls = Rc::new(Cell::new(0));
    let callback = {
        let calls = Rc::clone(&calls);
        Callback::new(move |()| calls.set(calls.get() + 1))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Grid::new().keyed_children([keyed(
                "retiring",
                Button::new()
                    .exit_fade(std::time::Duration::from_millis(100))
                    .on_click(callback.clone()),
            )]),
        )
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let retired = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    runtime.update(Grid::new()).unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        retired,
        EventId::Click,
        EventValue::Unit(callback),
        EventPayload::Unit,
    ));
    runtime.adapter_mut().queue_retirement_completion(retired);

    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(calls.get(), 0);
    assert_eq!(runtime.graph().retired_count(), 0);
}

#[test]
fn keyed_visual_moves_preserve_identity_and_only_update_changed_state() {
    let first_callback = Callback::new(|_: PointerEventInfo| {});
    let second_callback = Callback::new(|_: PointerEventInfo| {});
    let card = |key: &str, left: f64, background: Color, callback: Callback<PointerEventInfo>| {
        keyed(
            key,
            Border::new()
                .margin(Thickness::new(left, 0.0, 0.0, 0.0))
                .background(background)
                .transitions([ThemeTransition::Reposition])
                .on_pointer_released(callback),
        )
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([
            card("first", 0.0, Color::rgb(255, 255, 255), first_callback),
            card(
                "second",
                100.0,
                Color::rgb(255, 255, 255),
                second_callback.clone(),
            ),
        ]))
        .unwrap();
    let grid = runtime.graph().root().unwrap();
    let before = runtime
        .graph()
        .children(grid, RelationId::Children)
        .unwrap()
        .to_vec();
    let replacement_callback = Callback::new(|_: PointerEventInfo| {});

    let mutations = runtime
        .update(Grid::new().keyed_children([
            card("second", 100.0, Color::rgb(255, 255, 255), second_callback),
            card(
                "first",
                200.0,
                Color::rgb(255, 220, 220),
                replacement_callback,
            ),
        ]))
        .unwrap();
    let after = runtime
        .graph()
        .children(grid, RelationId::Children)
        .unwrap();

    assert_eq!(after, [before[1], before[0]]);
    assert_eq!(
        mutations
            .iter()
            .filter(|mutation| matches!(mutation, Mutation::Reorder { .. }))
            .count(),
        1
    );
    assert!(
        !mutations
            .iter()
            .any(|mutation| matches!(mutation, Mutation::Create { .. } | Mutation::Destroy { .. }))
    );
    assert!(mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::SetProperties { object, set, clear }
            if *object == before[0]
                && clear.is_empty()
                && set.len() == 2
                && set.iter().any(|property| property.id == PropertyId::Margin)
                && set.iter().any(|property| property.id == PropertyId::Background)
    )));
    assert!(mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::SetEvents { object, set, clear }
            if *object == before[0]
                && clear.is_empty()
                && set.len() == 1
                && set[0].id == EventId::PointerReleased
    )));
    assert!(!mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::SetProperties { set, .. }
            if set.iter().any(|property| property.id == PropertyId::Transitions)
    )));
}

#[test]
fn event_callbacks_update_without_recreating_the_object() {
    let first = Callback::new(|_: Rc<str>| {});
    let second = Callback::new(|_: Rc<str>| {});
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TextBox::new("Before").on_text_changed(first))
        .unwrap();
    let root = runtime.graph().root().unwrap();

    let mutations = runtime
        .update(TextBox::new("Before").on_text_changed(second))
        .unwrap();

    assert!(matches!(
        mutations.as_slice(),
        [Mutation::SetEvents { object, .. }] if *object == root
    ));
}

#[test]
fn event_callbacks_can_be_removed_without_recreating_the_object() {
    let callback = Callback::new(|_: Rc<str>| {});
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TextBox::new("Text").on_text_changed(callback))
        .unwrap();
    let root = runtime.graph().root().unwrap();

    let mutations = runtime.update(TextBox::new("Text")).unwrap();

    assert!(matches!(
        mutations.as_slice(),
        [Mutation::SetEvents { object, set, clear }]
            if *object == root && set.is_empty() && clear.as_ref() == [EventId::TextChanged]
    ));
}

#[test]
fn native_property_observation_updates_retained_state_before_reconciliation() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(TextBox::new("Before")).unwrap();
    let root = runtime.graph().root().unwrap();
    runtime.adapter_mut().observe(Observation::SetProperty {
        object: root,
        property: Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("After")),
        },
    });

    let mutations = runtime.update(TextBox::new("After")).unwrap();

    assert!(mutations.is_empty());
    assert_eq!(
        runtime.graph().properties(root).unwrap(),
        [Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("After")),
        }]
    );
}

#[test]
fn subtree_update_consumes_pending_controlled_observation() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([keyed("text", TextBox::new("Before"))]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let text = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    runtime.adapter_mut().observe(Observation::SetProperty {
        object: text,
        property: Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("After")),
        },
    });

    let mutations = runtime.update_subtree(text, TextBox::new("After")).unwrap();

    assert!(mutations.is_empty());
    assert_eq!(
        runtime.graph().properties(text).unwrap(),
        [Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("After")),
        }]
    );
}

#[test]
fn remove_child_consumes_pending_observation_before_planning() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([keyed("text", TextBox::new("Before"))]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let text = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    runtime.adapter_mut().observe(Observation::SetProperty {
        object: text,
        property: Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("After")),
        },
    });

    runtime
        .remove_child(root, RelationId::Children, text)
        .unwrap();

    assert_eq!(
        runtime
            .graph()
            .children(root, RelationId::Children)
            .unwrap(),
        []
    );
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
}

#[test]
fn invalid_observation_batch_rolls_back_and_remains_queued() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(TextBox::new("Before")).unwrap();
    let root = runtime.graph().root().unwrap();
    runtime.adapter_mut().observe(Observation::SetProperty {
        object: root,
        property: Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("After")),
        },
    });
    runtime.adapter_mut().observe(Observation::SetProperty {
        object: root,
        property: Property {
            id: PropertyId::Expanded,
            value: PropertyValue::Bool(true),
        },
    });
    let graph = runtime.graph().clone();
    let adapter = runtime.adapter().clone();

    assert_eq!(
        runtime.update(TextBox::new("After")),
        Err(UpdateError::InvalidNativeEvent(
            GraphError::InvalidProperty(ObjectType::TextBox, PropertyId::Expanded)
        ))
    );
    assert_eq!(runtime.graph(), &graph);
    assert_eq!(runtime.adapter(), &adapter);
    assert_eq!(
        runtime.update(TextBox::new("Before")),
        Err(UpdateError::Poisoned)
    );
}

#[test]
fn ordered_native_events_hide_future_observations_from_earlier_callbacks() {
    let values = Rc::new(RefCell::new(Vec::new()));
    let callback = {
        let values = Rc::clone(&values);
        Callback::new(move |value: Rc<str>| values.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TextBox::new("Before").on_text_changed(callback.clone()))
        .unwrap();
    let object = runtime.graph().root().unwrap();
    for value in ["A", "B"] {
        runtime.adapter_mut().queue_native_event(
            Some(Observation::SetProperty {
                object,
                property: Property {
                    id: PropertyId::Text,
                    value: PropertyValue::String(Rc::from(value)),
                },
            }),
            Some(EventDispatch::new(
                object,
                EventId::TextChanged,
                EventValue::String(callback.clone()),
                EventPayload::String(Rc::from(value)),
            )),
        );
    }

    let mut first = runtime.next_native_event().unwrap().unwrap();
    assert_eq!(
        runtime.graph().properties(object).unwrap()[0].value,
        PropertyValue::String(Rc::from("A"))
    );
    first.invoke();
    assert_eq!(&*values.borrow(), &[Rc::from("A")]);
    drop(first);

    let mut second = runtime.next_native_event().unwrap().unwrap();
    assert_eq!(
        runtime.graph().properties(object).unwrap()[0].value,
        PropertyValue::String(Rc::from("B"))
    );
    second.invoke();
    drop(second);
    assert_eq!(&*values.borrow(), &[Rc::from("A"), Rc::from("B")]);
}

#[test]
fn callback_reconciliation_completes_before_the_next_native_occurrence() {
    let rendered = Rc::new(Cell::new(false));
    let callback = {
        let rendered = Rc::clone(&rendered);
        Callback::new(move |_: Rc<str>| rendered.set(true))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TextBox::new("Before").on_text_changed(callback.clone()))
        .unwrap();
    let object = runtime.graph().root().unwrap();
    for value in ["A", "B"] {
        runtime.adapter_mut().queue_native_event(
            Some(Observation::SetProperty {
                object,
                property: Property {
                    id: PropertyId::Text,
                    value: PropertyValue::String(Rc::from(value)),
                },
            }),
            Some(EventDispatch::new(
                object,
                EventId::TextChanged,
                EventValue::String(callback.clone()),
                EventPayload::String(Rc::from(value)),
            )),
        );
    }

    let mut first = runtime.next_native_event().unwrap().unwrap();
    first.invoke();
    assert!(rendered.get());
    runtime
        .update(TextBox::new("Rendered").on_text_changed(callback))
        .unwrap();
    assert_eq!(
        runtime.graph().properties(object).unwrap()[0].value,
        PropertyValue::String(Rc::from("Rendered"))
    );
    drop(first);

    let mut second = runtime.next_native_event().unwrap().unwrap();
    assert_eq!(
        runtime.graph().properties(object).unwrap()[0].value,
        PropertyValue::String(Rc::from("B"))
    );
    second.invoke();
    drop(second);
}

#[test]
fn callback_panic_releases_the_native_boundary_and_remaining_events_continue() {
    let panic_callback = Callback::new(|_: Rc<str>| panic!("test callback panic"));
    let calls = Rc::new(Cell::new(0));
    let next_callback = {
        let calls = Rc::clone(&calls);
        Callback::new(move |_: Rc<str>| calls.set(calls.get() + 1))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TextBox::new("Before").on_text_changed(panic_callback.clone()))
        .unwrap();
    let object = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::TextChanged,
        EventValue::String(panic_callback),
        EventPayload::String(Rc::from("A")),
    ));
    runtime.adapter_mut().observe(Observation::SetProperty {
        object,
        property: Property {
            id: PropertyId::Text,
            value: PropertyValue::String(Rc::from("B")),
        },
    });

    let mut event = runtime.next_native_event().unwrap().unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| event.invoke()));

    assert!(result.is_err());
    runtime
        .update(TextBox::new("B").on_text_changed(next_callback.clone()))
        .unwrap();
    drop(event);
    assert_eq!(
        runtime.graph().properties(object).unwrap()[0].value,
        PropertyValue::String(Rc::from("B"))
    );
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::TextChanged,
        EventValue::String(next_callback),
        EventPayload::String(Rc::from("C")),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(calls.get(), 1);
}

#[test]
fn exact_feedback_suppresses_matching_observation_until_write_finishes() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(TextBox::new("Before")).unwrap();
    let object = runtime.graph().root().unwrap();
    let property = Property {
        id: PropertyId::Text,
        value: PropertyValue::String(Rc::from("After")),
    };
    let mut feedback = FeedbackState::default();
    feedback.begin(
        object,
        EventId::TextChanged,
        FeedbackExpectation::Exact(property.clone()),
    );

    assert!(!feedback.observe(
        object,
        EventId::TextChanged,
        Observation::SetProperty {
            object,
            property: property.clone(),
        }
    ));
    assert_eq!(feedback.finish(object, EventId::TextChanged), None);

    feedback.begin(
        object,
        EventId::TextChanged,
        FeedbackExpectation::Exact(property),
    );
    assert!(feedback.observe(
        object,
        EventId::TextChanged,
        Observation::SetProperty {
            object,
            property: Property {
                id: PropertyId::Text,
                value: PropertyValue::String(Rc::from("Normalized")),
            },
        }
    ));
}

#[test]
fn normalized_feedback_defers_latest_observation_until_write_finishes() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(Slider::new()).unwrap();
    let object = runtime.graph().root().unwrap();
    let mut feedback = FeedbackState::default();
    feedback.begin(
        object,
        EventId::ValueChanged,
        FeedbackExpectation::Normalized { observation: None },
    );
    let observation = Observation::SetProperty {
        object,
        property: Property {
            id: PropertyId::Value,
            value: PropertyValue::F64(0.75),
        },
    };

    assert!(!feedback.observe(object, EventId::ValueChanged, observation.clone()));
    assert_eq!(
        feedback.finish(object, EventId::ValueChanged),
        Some(observation)
    );
}

#[test]
fn suppressed_feedback_drops_selection_observations_during_native_writes() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(ListBox::new().keyed_items([
            keyed("first", ListBoxItem::new().tag("first").is_selected(true)),
            keyed(
                "second",
                ListBoxItem::new().tag("second").is_selected(false),
            ),
        ]))
        .unwrap();
    let owner = runtime.graph().root().unwrap();
    let selected = runtime.graph().children(owner, RelationId::Items).unwrap()[1];
    let observation = Observation::SetSelection {
        object: owner,
        selected: Some(selected),
    };
    let mut feedback = FeedbackState::default();
    feedback.begin(
        owner,
        EventId::SelectionChanged,
        FeedbackExpectation::Suppressed,
    );

    assert!(!feedback.observe(owner, EventId::SelectionChanged, observation));
    assert_eq!(feedback.finish(owner, EventId::SelectionChanged), None);
}

#[test]
fn retained_selection_tracks_keyed_items_across_reorder_and_removal() {
    let selected_values = Rc::new(RefCell::new(Vec::new()));
    let selected_values_for_callback = Rc::clone(&selected_values);
    let callback =
        Callback::new(move |value| selected_values_for_callback.borrow_mut().push(value));
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            ListBox::new()
                .keyed_items([
                    keyed("first", ListBoxItem::new().tag("first").is_selected(true)),
                    keyed(
                        "second",
                        ListBoxItem::new().tag("second").is_selected(false),
                    ),
                ])
                .on_selected_tag_changed(callback.clone()),
        )
        .unwrap();
    let owner = runtime.graph().root().unwrap();
    let before = runtime
        .graph()
        .children(owner, RelationId::Items)
        .unwrap()
        .to_vec();
    runtime.adapter_mut().queue_native_event(
        Some(Observation::SetSelection {
            object: owner,
            selected: Some(before[1]),
        }),
        Some(EventDispatch::new(
            owner,
            EventId::SelectionChanged,
            EventValue::Selection(callback.clone()),
            EventPayload::Selection(SelectionChange {
                item: Some(before[1]),
                value: Some(Rc::from("second")),
            }),
        )),
    );

    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(
        runtime.graph().properties(before[0]).unwrap(),
        [
            Property {
                id: PropertyId::IsSelected,
                value: PropertyValue::Bool(false),
            },
            Property {
                id: PropertyId::Tag,
                value: PropertyValue::String(Rc::from("first")),
            },
        ]
    );
    assert_eq!(
        runtime.graph().properties(before[1]).unwrap(),
        [
            Property {
                id: PropertyId::IsSelected,
                value: PropertyValue::Bool(true),
            },
            Property {
                id: PropertyId::Tag,
                value: PropertyValue::String(Rc::from("second")),
            },
        ]
    );
    assert_eq!(&*selected_values.borrow(), &[Some(Rc::from("second"))]);

    let mutations = runtime
        .update(
            ListBox::new()
                .keyed_items([
                    keyed("second", ListBoxItem::new().tag("second").is_selected(true)),
                    keyed("first", ListBoxItem::new().tag("first").is_selected(false)),
                ])
                .on_selected_tag_changed(callback.clone()),
        )
        .unwrap();
    assert_eq!(
        runtime.graph().children(owner, RelationId::Items).unwrap(),
        [before[1], before[0]]
    );
    assert_eq!(
        mutations
            .iter()
            .filter(|mutation| matches!(mutation, Mutation::Reorder { .. }))
            .count(),
        1
    );
    assert!(!mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::SetProperties { object, .. } if before.contains(object)
    )));

    runtime
        .update(
            ListBox::new()
                .keyed_items([keyed(
                    "first",
                    ListBoxItem::new().tag("first").is_selected(true),
                )])
                .on_selected_tag_changed(callback.clone()),
        )
        .unwrap();
    runtime.adapter_mut().queue_native_event(
        Some(Observation::SetSelection {
            object: owner,
            selected: Some(before[1]),
        }),
        Some(EventDispatch::new(
            owner,
            EventId::SelectionChanged,
            EventValue::Selection(callback),
            EventPayload::Selection(SelectionChange {
                item: Some(before[1]),
                value: Some(Rc::from("stale")),
            }),
        )),
    );
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(&*selected_values.borrow(), &[Some(Rc::from("second"))]);
    assert_eq!(
        runtime.graph().properties(before[0]).unwrap(),
        [
            Property {
                id: PropertyId::IsSelected,
                value: PropertyValue::Bool(true),
            },
            Property {
                id: PropertyId::Tag,
                value: PropertyValue::String(Rc::from("first")),
            },
        ]
    );
}

#[test]
fn selection_contracts_cover_navigation_list_box_and_selector_bar() {
    for (owner, item, relations, payload) in [
        (
            ObjectType::NavigationView,
            ObjectType::NavigationViewItem,
            &[RelationId::MenuItems, RelationId::FooterMenuItems][..],
            PropertyId::Tag,
        ),
        (
            ObjectType::ListBox,
            ObjectType::ListBoxItem,
            &[RelationId::Items][..],
            PropertyId::Tag,
        ),
        (
            ObjectType::SelectorBar,
            ObjectType::SelectorBarItem,
            &[RelationId::Items][..],
            PropertyId::Text,
        ),
    ] {
        let contract = selection_contract(owner).unwrap();
        assert_eq!(contract.item, item);
        assert_eq!(contract.relations, relations);
        assert_eq!(contract.selected_property, PropertyId::IsSelected);
        assert_eq!(contract.event, EventId::SelectionChanged);
        assert_eq!(contract.payload_property, payload);
    }
}

#[test]
fn straightforward_event_payloads_round_trip_through_recording_protocol() {
    let boolean = Rc::new(Cell::new(false));
    let boolean_callback = {
        let boolean = Rc::clone(&boolean);
        Callback::new(move |value| boolean.set(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(ToggleSwitch::new().on_toggled(boolean_callback.clone()))
        .unwrap();
    let object = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::Toggled,
        EventValue::Bool(boolean_callback),
        EventPayload::Bool(true),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert!(boolean.get());

    let text = Rc::new(RefCell::new(None));
    let text_callback = {
        let text = Rc::clone(&text);
        Callback::new(move |value| *text.borrow_mut() = Some(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(PasswordBox::new().on_password_changed(text_callback.clone()))
        .unwrap();
    let object = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::PasswordChanged,
        EventValue::String(text_callback),
        EventPayload::String(Rc::from("secret")),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(*text.borrow(), Some(Rc::from("secret")));

    let number = Rc::new(Cell::new(None));
    let number_callback = {
        let number = Rc::clone(&number);
        Callback::new(move |value| number.set(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(RatingControl::new().on_value_changed(number_callback.clone()))
        .unwrap();
    let object = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::ValueChanged,
        EventValue::OptionalF64(number_callback),
        EventPayload::OptionalF64(Some(4.0)),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(number.get(), Some(4.0));

    let date = DateTime::from_unix_secs(1_700_000_000);
    let dates = Rc::new(RefCell::new(Vec::new()));
    let date_callback = {
        let dates = Rc::clone(&dates);
        Callback::new(move |value| dates.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(CalendarDatePicker::new().on_date_changed(date_callback.clone()))
        .unwrap();
    let object = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::DateChanged,
        EventValue::OptionalDateTime(date_callback.clone()),
        EventPayload::OptionalDateTime(Some(date)),
    ));
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::DateChanged,
        EventValue::OptionalDateTime(date_callback),
        EventPayload::OptionalDateTime(None),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 2);
    assert_eq!(&*dates.borrow(), &[Some(date), None]);

    let index = Rc::new(Cell::new(None));
    let index_callback = {
        let index = Rc::clone(&index);
        Callback::new(move |value| index.set(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(ComboBox::new().on_selection_changed(index_callback.clone()))
        .unwrap();
    let object = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::SelectionChanged,
        EventValue::SelectionIndex(index_callback),
        EventPayload::SelectionIndex(Some(2)),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(index.get(), Some(2));
}

#[test]
fn invalid_native_property_observation_is_rejected() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(TextBox::new("Text")).unwrap();
    let root = runtime.graph().root().unwrap();
    runtime.adapter_mut().observe(Observation::SetProperty {
        object: root,
        property: Property {
            id: PropertyId::Expanded,
            value: PropertyValue::Bool(true),
        },
    });

    assert!(matches!(
        runtime.update(TextBox::new("Text")),
        Err(UpdateError::InvalidNativeEvent(
            GraphError::InvalidProperty(ObjectType::TextBox, PropertyId::Expanded)
        ))
    ));
    assert_eq!(
        runtime.update(TextBox::new("Text")),
        Err(UpdateError::Poisoned)
    );
}

#[test]
fn invalid_generated_enum_observation_is_rejected() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(StackPanel::new().orientation(Orientation::Vertical))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    runtime.adapter_mut().observe(Observation::SetProperty {
        object: root,
        property: Property {
            id: PropertyId::Orientation,
            value: PropertyValue::Enum {
                kind: "Orientation",
                variant: "Diagonal",
            },
        },
    });

    assert!(matches!(
        runtime.update(StackPanel::new()),
        Err(UpdateError::InvalidNativeEvent(
            GraphError::InvalidPropertyValue(PropertyId::Orientation)
        ))
    ));
    assert_eq!(
        runtime.update(StackPanel::new()),
        Err(UpdateError::Poisoned)
    );
}

#[test]
fn stale_queued_event_does_not_reach_replacement_callback() {
    let first_count = Rc::new(Cell::new(0));
    let first_count_for_callback = Rc::clone(&first_count);
    let first = Callback::new(move |_: Rc<str>| {
        first_count_for_callback.set(first_count_for_callback.get() + 1);
    });
    let second_count = Rc::new(Cell::new(0));
    let second_count_for_callback = Rc::clone(&second_count);
    let second = Callback::new(move |_: Rc<str>| {
        second_count_for_callback.set(second_count_for_callback.get() + 1);
    });
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TextBox::new("Text").on_text_changed(first.clone()))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    for value in ["First", "Stale"] {
        runtime.adapter_mut().queue_native_event(
            Some(Observation::SetProperty {
                object: root,
                property: Property {
                    id: PropertyId::Text,
                    value: PropertyValue::String(Rc::from(value)),
                },
            }),
            Some(EventDispatch::new(
                root,
                EventId::TextChanged,
                EventValue::String(first.clone()),
                EventPayload::String(Rc::from(value)),
            )),
        );
    }
    let mut active = runtime.next_native_event().unwrap().unwrap();
    active.invoke();
    runtime
        .update(TextBox::new("First").on_text_changed(second.clone()))
        .unwrap();
    drop(active);

    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(
        runtime.graph().properties(root).unwrap()[0].value,
        PropertyValue::String(Rc::from("Stale"))
    );
    assert!(
        runtime
            .update(TextBox::new("Stale").on_text_changed(second))
            .unwrap()
            .is_empty()
    );
    assert_eq!(first_count.get(), 1);
    assert_eq!(second_count.get(), 0);
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
fn pointer_event_payload_round_trips_through_recording_protocol() {
    let received = Rc::new(RefCell::new(None));
    let received_for_callback = Rc::clone(&received);
    let callback = Callback::new(move |value| {
        *received_for_callback.borrow_mut() = Some(value);
    });
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Border::new().on_pointer_released(callback.clone()))
        .unwrap();
    let border = runtime.graph().root().unwrap();
    let payload = PointerEventInfo {
        x: 12.5,
        y: 24.5,
        window_x: 112.5,
        window_y: 224.5,
        pointer_id: 42,
        capture_succeeded: None,
        is_captured: true,
        is_left_button_pressed: false,
        is_right_button_pressed: true,
        is_middle_button_pressed: false,
    };
    runtime.adapter_mut().queue_event(EventDispatch::new(
        border,
        EventId::PointerReleased,
        EventValue::PointerEventInfo(callback),
        EventPayload::PointerEventInfo(payload),
    ));

    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);

    assert_eq!(*received.borrow(), Some(payload));
    assert!(
        event_contracts(ObjectType::Border).contains(&EventContract {
            id: EventId::PointerReleased,
            value: ValueType::PointerEventInfo,
        })
    );
}

#[test]
fn pointer_policy_records_set_and_clear() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Border::new()
                .capture_pointer_on_press(true)
                .focus_on_pointer_release(true),
        )
        .unwrap();
    let border = runtime.graph().root().unwrap();
    assert!(
        runtime
            .graph()
            .properties(border)
            .unwrap()
            .contains(&Property {
                id: PropertyId::CapturePointerOnPress,
                value: PropertyValue::Bool(true),
            })
    );
    assert!(
        runtime
            .graph()
            .properties(border)
            .unwrap()
            .contains(&Property {
                id: PropertyId::FocusOnPointerRelease,
                value: PropertyValue::Bool(true),
            })
    );

    let mutations = runtime.update(Border::new()).unwrap();
    assert!(mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::SetProperties { object, set, clear }
            if *object == border
                && set.is_empty()
                && clear.as_ref()
                    == [
                        PropertyId::CapturePointerOnPress,
                        PropertyId::FocusOnPointerRelease
                    ]
    )));
}

#[test]
fn routed_keyboard_payloads_preserve_the_handled_result() {
    let key = KeyEventInfo {
        key: VirtualKey::ENTER,
        original_key: VirtualKey::ENTER,
        status: PhysicalKeyStatus::default(),
        modifiers: InputModifiers::CONTROL,
    };
    let character = CharacterEventInfo {
        character: b'A' as u16,
        status: PhysicalKeyStatus::default(),
        modifiers: InputModifiers::SHIFT,
    };
    let key_callback = RoutedCallback::new(move |value| value == key);
    let character_callback = RoutedCallback::new(move |value| value == character);
    assert!(key_callback.call(key));
    assert!(character_callback.call(character));

    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Border::new()
                .on_preview_key_down(key_callback.clone())
                .on_character_received(character_callback.clone()),
        )
        .unwrap();
    let border = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        border,
        EventId::PreviewKeyDown,
        EventValue::KeyEventInfo(key_callback),
        EventPayload::KeyEventInfo(key),
    ));
    runtime.adapter_mut().queue_event(EventDispatch::new(
        border,
        EventId::CharacterReceived,
        EventValue::CharacterEventInfo(character_callback),
        EventPayload::CharacterEventInfo(character),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 2);
}

#[test]
fn focus_payloads_round_trip_through_recording_protocol() {
    let values = Rc::new(RefCell::new(Vec::new()));
    let callback = {
        let values = Rc::clone(&values);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let got = FocusEventInfo {
        state: ElementFocusState::Keyboard,
        is_direct: true,
    };
    let lost = FocusEventInfo {
        state: ElementFocusState::Unfocused,
        is_direct: false,
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Border::new()
                .on_got_focus(callback.clone())
                .on_lost_focus(callback.clone()),
        )
        .unwrap();
    let border = runtime.graph().root().unwrap();
    for (event, value) in [(EventId::GotFocus, got), (EventId::LostFocus, lost)] {
        runtime.adapter_mut().queue_event(EventDispatch::new(
            border,
            event,
            EventValue::FocusEventInfo(callback.clone()),
            EventPayload::FocusEventInfo(value),
        ));
    }
    assert_eq!(runtime.dispatch_native_events().unwrap(), 2);
    assert_eq!(&*values.borrow(), &[got, lost]);
}

#[test]
fn drag_drop_policy_and_payloads_round_trip_through_recording_protocol() {
    let policy = DragDropPolicy::new()
        .storage_items(DragDropAction::new(DragDropOperation::Move))
        .text(DragDropAction::new(DragDropOperation::Copy).caption("Copy text"));
    let drag_values = Rc::new(RefCell::new(Vec::new()));
    let dropped_values = Rc::new(RefCell::new(Vec::new()));
    let drag_callback = {
        let values = Rc::clone(&drag_values);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let drop_callback = {
        let values = Rc::clone(&dropped_values);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Border::new()
                .drop_policy(policy.clone())
                .on_drag_enter(drag_callback.clone())
                .on_drag_over(drag_callback.clone())
                .on_drop(drop_callback.clone()),
        )
        .unwrap();
    let border = runtime.graph().root().unwrap();
    assert!(
        runtime
            .graph()
            .properties(border)
            .unwrap()
            .contains(&Property {
                id: PropertyId::DropPolicy,
                value: PropertyValue::DragDropPolicy(Rc::new(policy)),
            })
    );
    for (event, value) in [
        (EventId::DragEnter, DragKind::Text),
        (EventId::DragOver, DragKind::StorageItems),
    ] {
        runtime.adapter_mut().queue_event(EventDispatch::new(
            border,
            event,
            EventValue::DragKind(drag_callback.clone()),
            EventPayload::DragKind(value),
        ));
    }
    let dropped = DroppedData::Text("value".to_string());
    runtime.adapter_mut().queue_event(EventDispatch::new(
        border,
        EventId::Drop,
        EventValue::DroppedData(drop_callback),
        EventPayload::DroppedData(dropped.clone()),
    ));

    assert_eq!(runtime.dispatch_native_events().unwrap(), 3);
    assert_eq!(
        &*drag_values.borrow(),
        &[DragKind::Text, DragKind::StorageItems]
    );
    assert_eq!(&*dropped_values.borrow(), &[dropped]);

    let mutations = runtime.update(Border::new()).unwrap();
    assert!(mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::SetProperties { object, set, clear }
            if *object == border
                && set.is_empty()
                && clear.as_ref() == [PropertyId::DropPolicy]
    )));
}

#[test]
fn grid_view_reorder_payload_preserves_item_tag_order() {
    let values = Rc::new(RefCell::new(Vec::new()));
    let callback = {
        let values = Rc::clone(&values);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            GridView::new()
                .items([
                    GridViewItem::new().tag("first").into(),
                    GridViewItem::new().tag("second").into(),
                ])
                .on_reordered(callback.clone()),
        )
        .unwrap();
    let grid = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        grid,
        EventId::DragItemsCompleted,
        EventValue::StringList(callback),
        EventPayload::StringList(vec!["second".to_string(), "first".to_string()]),
    ));

    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(
        &*values.borrow(),
        &[vec!["second".to_string(), "first".to_string()]]
    );
}

#[test]
fn list_view_properties_update_and_clear() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    let mutations = runtime
        .update(
            ListView::new()
                .selected_index(Some(1))
                .selection_mode(ListViewSelectionMode::Extended)
                .can_drag_items(true)
                .can_reorder_items(true)
                .allow_drop(true),
        )
        .unwrap();
    let list = runtime.graph().root().unwrap();
    let set = mutations
        .iter()
        .find_map(|mutation| match mutation {
            Mutation::SetProperties { object, set, .. } if *object == list => Some(set),
            _ => None,
        })
        .unwrap();
    for property in [
        PropertyId::SelectedIndex,
        PropertyId::SelectionMode,
        PropertyId::CanDragItems,
        PropertyId::CanReorderItems,
        PropertyId::AllowDrop,
    ] {
        assert!(set.iter().any(|value| value.id == property));
    }

    let mutations = runtime.update(ListView::new()).unwrap();
    let clear = mutations
        .iter()
        .find_map(|mutation| match mutation {
            Mutation::SetProperties { object, clear, .. } if *object == list => Some(clear),
            _ => None,
        })
        .unwrap();
    assert_eq!(clear.len(), 5);
    for property in [
        PropertyId::SelectedIndex,
        PropertyId::SelectionMode,
        PropertyId::CanDragItems,
        PropertyId::CanReorderItems,
        PropertyId::AllowDrop,
    ] {
        assert!(clear.contains(&property));
    }
}

#[test]
fn list_view_selection_and_reorder_callbacks_replace_remove_and_reject_stale_events() {
    let first_selections = Rc::new(RefCell::new(Vec::new()));
    let first_selection = {
        let values = Rc::clone(&first_selections);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let first_reorders = Rc::new(RefCell::new(Vec::new()));
    let first_reorder = {
        let values = Rc::clone(&first_reorders);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let second_selections = Rc::new(RefCell::new(Vec::new()));
    let second_selection = {
        let values = Rc::clone(&second_selections);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let second_reorders = Rc::new(RefCell::new(Vec::new()));
    let second_reorder = {
        let values = Rc::clone(&second_reorders);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            ListView::new()
                .on_selection_changed(first_selection.clone())
                .on_reordered(first_reorder.clone()),
        )
        .unwrap();
    let list = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_native_event(
        Some(Observation::SetProperty {
            object: list,
            property: Property {
                id: PropertyId::SelectedIndex,
                value: PropertyValue::SelectionIndex(Some(1)),
            },
        }),
        Some(EventDispatch::new(
            list,
            EventId::SelectionChanged,
            EventValue::SelectionIndex(first_selection.clone()),
            EventPayload::SelectionIndex(Some(1)),
        )),
    );
    runtime.adapter_mut().queue_event(EventDispatch::new(
        list,
        EventId::DragItemsCompleted,
        EventValue::StringList(first_reorder.clone()),
        EventPayload::StringList(vec!["second".to_string(), "first".to_string()]),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 2);
    assert_eq!(&*first_selections.borrow(), &[Some(1)]);
    assert_eq!(
        &*first_reorders.borrow(),
        &[vec!["second".to_string(), "first".to_string()]]
    );

    runtime
        .update(
            ListView::new()
                .selected_index(Some(1))
                .on_selection_changed(second_selection.clone())
                .on_reordered(second_reorder.clone()),
        )
        .unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        list,
        EventId::SelectionChanged,
        EventValue::SelectionIndex(first_selection),
        EventPayload::SelectionIndex(None),
    ));
    runtime.adapter_mut().queue_event(EventDispatch::new(
        list,
        EventId::DragItemsCompleted,
        EventValue::StringList(first_reorder),
        EventPayload::StringList(vec!["stale".to_string()]),
    ));
    runtime.adapter_mut().queue_event(EventDispatch::new(
        list,
        EventId::SelectionChanged,
        EventValue::SelectionIndex(second_selection.clone()),
        EventPayload::SelectionIndex(None),
    ));
    runtime.adapter_mut().queue_event(EventDispatch::new(
        list,
        EventId::DragItemsCompleted,
        EventValue::StringList(second_reorder.clone()),
        EventPayload::StringList(vec!["current".to_string()]),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 2);
    assert_eq!(&*second_selections.borrow(), &[None]);
    assert_eq!(&*second_reorders.borrow(), &[vec!["current".to_string()]]);

    runtime.update(ListView::new()).unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        list,
        EventId::SelectionChanged,
        EventValue::SelectionIndex(second_selection),
        EventPayload::SelectionIndex(Some(2)),
    ));
    runtime.adapter_mut().queue_event(EventDispatch::new(
        list,
        EventId::DragItemsCompleted,
        EventValue::StringList(second_reorder),
        EventPayload::StringList(vec!["removed".to_string()]),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
}

#[test]
fn tree_view_selection_mode_and_item_invocation_replace_remove_and_reject_stale_events() {
    let first = Rc::new(RefCell::new(Vec::new()));
    let first_callback = {
        let values = Rc::clone(&first);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let second = Rc::new(RefCell::new(Vec::new()));
    let second_callback = {
        let values = Rc::clone(&second);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    let mutations = runtime
        .update(
            TreeView::new()
                .selection_mode(TreeViewSelectionMode::Multiple)
                .nodes([TreeNode::new("root", "Root")])
                .on_item_invoked(first_callback.clone()),
        )
        .unwrap();
    let tree = runtime.graph().root().unwrap();
    assert!(mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::SetProperties { object, set, .. }
            if *object == tree
                && set.iter().any(|property| property.id == PropertyId::TreeSelectionMode)
    )));
    runtime.adapter_mut().queue_event(EventDispatch::new(
        tree,
        EventId::ItemInvoked,
        EventValue::String(first_callback.clone()),
        EventPayload::String(Rc::from("Root")),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(&*first.borrow(), &[Rc::from("Root")]);

    runtime
        .update(
            TreeView::new()
                .selection_mode(TreeViewSelectionMode::Single)
                .nodes([TreeNode::new("root", "Root")])
                .on_item_invoked(second_callback.clone()),
        )
        .unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        tree,
        EventId::ItemInvoked,
        EventValue::String(first_callback),
        EventPayload::String(Rc::from("Stale")),
    ));
    runtime.adapter_mut().queue_event(EventDispatch::new(
        tree,
        EventId::ItemInvoked,
        EventValue::String(second_callback.clone()),
        EventPayload::String(Rc::from("Current")),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
    assert_eq!(&*second.borrow(), &[Rc::from("Current")]);

    let mutations = runtime
        .update(TreeView::new().nodes([TreeNode::new("root", "Root")]))
        .unwrap();
    assert!(mutations.iter().any(|mutation| matches!(
        mutation,
        Mutation::SetProperties { object, clear, .. }
            if *object == tree && clear.contains(&PropertyId::TreeSelectionMode)
    )));
    runtime.adapter_mut().queue_event(EventDispatch::new(
        tree,
        EventId::ItemInvoked,
        EventValue::String(second_callback),
        EventPayload::String(Rc::from("Removed")),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
}

#[test]
fn inspectable_string_item_events_deliver_typed_values() {
    let breadcrumb_values = Rc::new(RefCell::new(Vec::new()));
    let breadcrumb_callback = {
        let values = Rc::clone(&breadcrumb_values);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let mut breadcrumb = Runtime::new(RecordingAdapter::default());
    breadcrumb
        .update(
            BreadcrumbBar::new()
                .items_source(["Root", "Current"])
                .on_item_clicked(breadcrumb_callback.clone()),
        )
        .unwrap();
    let object = breadcrumb.graph().root().unwrap();
    breadcrumb.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::ItemClicked,
        EventValue::String(breadcrumb_callback),
        EventPayload::String(Rc::from("Current")),
    ));
    assert_eq!(breadcrumb.dispatch_native_events().unwrap(), 1);
    assert_eq!(&*breadcrumb_values.borrow(), &[Rc::from("Current")]);

    let suggestion_values = Rc::new(RefCell::new(Vec::new()));
    let suggestion_callback = {
        let values = Rc::clone(&suggestion_values);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let mut suggestions = Runtime::new(RecordingAdapter::default());
    suggestions
        .update(
            AutoSuggestBox::new()
                .items_source(["First", "Second"])
                .on_suggestion_chosen(suggestion_callback.clone()),
        )
        .unwrap();
    let object = suggestions.graph().root().unwrap();
    suggestions.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::SuggestionChosen,
        EventValue::String(suggestion_callback),
        EventPayload::String(Rc::from("Second")),
    ));
    assert_eq!(suggestions.dispatch_native_events().unwrap(), 1);
    assert_eq!(&*suggestion_values.borrow(), &[Rc::from("Second")]);
}

#[test]
fn tab_view_events_deliver_item_tags() {
    let closed_values = Rc::new(RefCell::new(Vec::new()));
    let closed_callback = {
        let values = Rc::clone(&closed_values);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let reordered_values = Rc::new(RefCell::new(Vec::new()));
    let reordered_callback = {
        let values = Rc::clone(&reordered_values);
        Callback::new(move |value| values.borrow_mut().push(value))
    };
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            TabView::new()
                .tab_items([
                    TabViewItem::new().tag("first").into(),
                    TabViewItem::new().tag("second").into(),
                ])
                .on_close_requested(closed_callback.clone())
                .on_reordered(reordered_callback.clone()),
        )
        .unwrap();
    let object = runtime.graph().root().unwrap();
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::TabCloseRequested,
        EventValue::String(closed_callback),
        EventPayload::String(Rc::from("second")),
    ));
    runtime.adapter_mut().queue_event(EventDispatch::new(
        object,
        EventId::TabItemsChanged,
        EventValue::StringList(reordered_callback),
        EventPayload::StringList(vec!["second".into(), "first".into()]),
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 2);
    assert_eq!(&*closed_values.borrow(), &[Rc::from("second")]);
    assert_eq!(
        &*reordered_values.borrow(),
        &[vec!["second".to_string(), "first".to_string()]]
    );
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
        .update(DatePicker::new().on_selected_date_changed(date_callback.clone()))
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
        .update(TimePicker::new().on_selected_time_changed(time_callback.clone()))
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
fn pointer_motion_events_use_the_shared_typed_payload() {
    let received = Rc::new(RefCell::new(Vec::new()));
    let callback = |event| {
        let received = Rc::clone(&received);
        Callback::new(move |_: PointerEventInfo| received.borrow_mut().push(event))
    };
    let moved = callback(EventId::PointerMoved);
    let entered = callback(EventId::PointerEntered);
    let exited = callback(EventId::PointerExited);
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(
            Border::new()
                .on_pointer_moved(moved.clone())
                .on_pointer_entered(entered.clone())
                .on_pointer_exited(exited.clone()),
        )
        .unwrap();
    let border = runtime.graph().root().unwrap();
    for (event, callback) in [
        (EventId::PointerMoved, moved),
        (EventId::PointerEntered, entered),
        (EventId::PointerExited, exited),
    ] {
        runtime.adapter_mut().queue_event(EventDispatch::new(
            border,
            event,
            EventValue::PointerEventInfo(callback),
            EventPayload::PointerEventInfo(PointerEventInfo::default()),
        ));
    }

    assert_eq!(runtime.dispatch_native_events().unwrap(), 3);
    assert_eq!(
        &*received.borrow(),
        &[
            EventId::PointerMoved,
            EventId::PointerEntered,
            EventId::PointerExited
        ]
    );
}

#[test]
fn stale_pointer_event_does_not_reach_replacement_callback() {
    let first_count = Rc::new(Cell::new(0));
    let first_count_for_callback = Rc::clone(&first_count);
    let first = Callback::new(move |_: PointerEventInfo| {
        first_count_for_callback.set(first_count_for_callback.get() + 1);
    });
    let second_count = Rc::new(Cell::new(0));
    let second_count_for_callback = Rc::clone(&second_count);
    let second = Callback::new(move |_: PointerEventInfo| {
        second_count_for_callback.set(second_count_for_callback.get() + 1);
    });
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Border::new().on_pointer_released(first.clone()))
        .unwrap();
    let border = runtime.graph().root().unwrap();
    for _ in 0..2 {
        runtime.adapter_mut().queue_event(EventDispatch::new(
            border,
            EventId::PointerReleased,
            EventValue::PointerEventInfo(first.clone()),
            EventPayload::PointerEventInfo(PointerEventInfo::default()),
        ));
    }
    let mut active = runtime.next_native_event().unwrap().unwrap();
    active.invoke();
    runtime
        .update(Border::new().on_pointer_released(second))
        .unwrap();
    drop(active);

    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(first_count.get(), 1);
    assert_eq!(second_count.get(), 0);
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

#[test]
fn subtree_update_reconciles_only_the_target_object() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([
            keyed("first", TextBlock::new().text("First")),
            keyed("second", TextBlock::new().text("Second")),
        ]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap();
    let first = children[0];
    let second = children[1];

    let mutations = runtime
        .update_subtree(first, TextBlock::new().text("Changed"))
        .unwrap();

    assert_eq!(
        mutations,
        vec![Mutation::SetProperties {
            object: first,
            set: Rc::from([Property {
                id: PropertyId::Text,
                value: PropertyValue::String(Rc::from("Changed")),
            }]),
            clear: Rc::from([]),
        }]
    );
    assert_eq!(
        runtime
            .graph()
            .children(root, RelationId::Children)
            .unwrap(),
        [first, second]
    );
}

#[test]
fn subtree_root_type_replacement_preserves_identity() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([keyed("child", TextBlock::new().text("Text"))]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let child = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];

    let mutations = runtime.update_subtree(child, Border::new()).unwrap();

    assert_eq!(
        mutations,
        vec![Mutation::Replace {
            object: child,
            kind: ObjectType::Border,
        }]
    );
    assert_eq!(runtime.graph().kind(child), Some(ObjectType::Border));
    assert_eq!(
        runtime
            .graph()
            .children(root, RelationId::Children)
            .unwrap(),
        [child]
    );
}

#[test]
fn subtree_root_type_replacement_rejects_incompatible_relations() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TreeView::new().nodes([TreeNode::new("node", "Node")]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let node = runtime.graph().children(root, RelationId::Roots).unwrap()[0];

    assert_eq!(
        runtime.update_subtree(node, TextBlock::new().text("Invalid")),
        Err(UpdateError::Graph(GraphError::InvalidChildCategory(
            RelationId::Roots
        )))
    );
    assert_eq!(runtime.graph().kind(node), Some(ObjectType::TreeNode));
}

#[test]
fn recording_adapter_rejects_invalid_replacement_batches() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(TextBlock::new().text("Root")).unwrap();
    let root = runtime.graph().root().unwrap();
    assert_eq!(
        runtime.adapter_mut().apply(&[Mutation::Replace {
            object: root,
            kind: ObjectType::Border,
        }]),
        Err(AdapterError::InvalidReplacement(root))
    );

    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(TreeView::new().nodes([TreeNode::new("node", "Node")]))
        .unwrap();
    let tree = runtime.graph().root().unwrap();
    let node = runtime.graph().children(tree, RelationId::Roots).unwrap()[0];
    assert_eq!(
        runtime.adapter_mut().apply(&[Mutation::Replace {
            object: node,
            kind: ObjectType::TextBlock,
        }]),
        Err(AdapterError::InvalidReplacement(node))
    );
}

#[test]
fn replacement_discards_observations_and_events_from_the_old_native_object() {
    let calls = Rc::new(Cell::new(0));
    let callback_calls = Rc::clone(&calls);
    let callback = Callback::new(move |_| callback_calls.set(callback_calls.get() + 1));
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([keyed(
            "child",
            TextBox::new("Old").on_text_changed(callback.clone()),
        )]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let child = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    for value in ["First", "Stale"] {
        runtime.adapter_mut().queue_native_event(
            Some(Observation::SetProperty {
                object: child,
                property: Property {
                    id: PropertyId::Text,
                    value: PropertyValue::String(Rc::from(value)),
                },
            }),
            Some(EventDispatch::new(
                child,
                EventId::TextChanged,
                EventValue::String(callback.clone()),
                EventPayload::String(Rc::from(value)),
            )),
        );
    }
    let mut active = runtime.next_native_event().unwrap().unwrap();
    active.invoke();
    runtime.update_subtree(child, Border::new()).unwrap();
    drop(active);

    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(calls.get(), 1);
    assert_eq!(runtime.graph().kind(child), Some(ObjectType::Border));
    assert!(runtime.graph().properties(child).unwrap().is_empty());
}

#[test]
fn targeted_child_removal_preserves_sibling_identity() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Grid::new().keyed_children([
            keyed("first", TextBlock::new().text("First")),
            keyed("second", TextBlock::new().text("Second")),
        ]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let children = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap();
    let first = children[0];
    let second = children[1];

    let mutations = runtime
        .remove_child(root, RelationId::Children, first)
        .unwrap();

    assert_eq!(
        mutations,
        vec![
            Mutation::Remove {
                parent: root,
                relation: RelationId::Children,
                child: first,
                index: 0,
            },
            Mutation::Destroy { object: first },
        ]
    );
    assert_eq!(
        runtime
            .graph()
            .children(root, RelationId::Children)
            .unwrap(),
        [second]
    );
    assert_eq!(runtime.graph().kind(first), None);
}

#[test]
fn no_change_produces_no_mutations() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime
        .update(Border::new().content(TextBlock::new().text("Text")))
        .unwrap();

    assert!(
        runtime
            .update(Border::new().content(TextBlock::new().text("Text")))
            .unwrap()
            .is_empty()
    );
}

#[derive(Default)]
struct NeverCalledAdapter;

impl Adapter for NeverCalledAdapter {
    type Error = ();

    fn validate(&self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        panic!("invalid declarations must not reach the adapter")
    }

    fn apply(&mut self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        panic!("invalid declarations must not reach the adapter")
    }

    fn focus(&mut self, _object: ObjectId) -> Result<bool, Self::Error> {
        panic!("invalid declarations must not reach the adapter")
    }
}

#[test]
fn excessive_depth_is_rejected_before_reconciliation() {
    let mut child: View = TextBlock::new().text("leaf").into();
    for _ in 0..10_000 {
        child = Border::new().content(child).into();
    }
    let mut runtime = Runtime::new(NeverCalledAdapter);

    assert_eq!(
        runtime.update(child),
        Err(UpdateError::Graph(GraphError::DepthExceeded))
    );
}

#[test]
fn shared_declarations_cannot_expand_beyond_the_graph_limit() {
    let mut child: View = TextBlock::new().text("leaf").into();
    for _ in 0..17 {
        child = StackPanel::new().children([child.clone(), child]).into();
    }
    let mut runtime = Runtime::new(NeverCalledAdapter);

    assert_eq!(
        runtime.update(child),
        Err(UpdateError::Graph(GraphError::SizeExceeded))
    );
}

#[derive(Default)]
struct ApplyOnceAdapter {
    applied: bool,
}

impl Adapter for ApplyOnceAdapter {
    type Error = ();

    fn validate(&self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn apply(&mut self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        if self.applied {
            Err(())
        } else {
            self.applied = true;
            Ok(())
        }
    }

    fn focus(&mut self, _object: ObjectId) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

#[test]
fn root_type_changes_are_rejected_before_native_mutation() {
    let mut runtime = Runtime::new(ApplyOnceAdapter::default());
    runtime.update(Border::new()).unwrap();

    assert_eq!(
        runtime.update(Grid::new()),
        Err(UpdateError::Graph(GraphError::RootTypeChanged {
            previous: ObjectType::Border,
            next: ObjectType::Grid,
        }))
    );
}

#[derive(Default)]
struct RejectingAdapter;

impl Adapter for RejectingAdapter {
    type Error = ();

    fn validate(&self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        Err(())
    }

    fn apply(&mut self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        panic!("an invalid plan must not be applied")
    }

    fn focus(&mut self, _object: ObjectId) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

#[test]
fn adapter_validation_failure_poisons_the_runtime() {
    let mut runtime = Runtime::new(RejectingAdapter);

    assert_eq!(
        runtime.update(Border::new().content(TextBlock::new().text("Text"))),
        Err(UpdateError::Adapter(()))
    );
    assert_eq!(runtime.update(Border::new()), Err(UpdateError::Poisoned));
}

#[derive(Default)]
struct FailingApplyAdapter;

impl Adapter for FailingApplyAdapter {
    type Error = ();

    fn validate(&self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn apply(&mut self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        Err(())
    }

    fn focus(&mut self, _object: ObjectId) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

#[test]
fn adapter_apply_failure_poisons_the_runtime() {
    let mut runtime = Runtime::new(FailingApplyAdapter);

    assert_eq!(
        runtime.update(Border::new().content(TextBlock::new().text("Text"))),
        Err(UpdateError::Adapter(()))
    );
    assert_eq!(runtime.update(Border::new()), Err(UpdateError::Poisoned));
    assert_eq!(runtime.graph().root(), None);
}

#[test]
fn graph_transaction_drop_rolls_back_during_unwind() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(TextBox::new("Before")).unwrap();
    let root = runtime.graph().root().unwrap();
    let mut graph = runtime.graph().clone();
    let before = graph.clone();

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        panic_during_transaction(&mut graph, root);
    }));

    assert!(result.is_err());
    assert_eq!(graph, before);
}

#[test]
fn before_apply_unwind_rolls_back_and_keeps_runtime_usable() {
    let mut runtime = Runtime::new(RecordingAdapter::default());
    runtime.update(TextBox::new("Before")).unwrap();
    let root = runtime.graph().root().unwrap();
    let before = runtime.graph().clone();

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        runtime
            .update_subtree_before_apply(root, TextBox::new("Changed"), || {
                panic!("test before_apply unwind");
            })
            .unwrap();
    }));

    assert!(result.is_err());
    assert_eq!(runtime.graph(), &before);
    runtime.update_subtree(root, TextBox::new("After")).unwrap();
}

#[derive(Default)]
struct PanicAdapter {
    inner: RecordingAdapter,
    panic_validate: bool,
    panic_apply: bool,
}

impl Adapter for PanicAdapter {
    type Error = AdapterError;

    fn preview_native_events(&self, events: &mut Vec<NativeEvent>) {
        self.inner.preview_native_events(events);
    }

    fn pop_native_event(&mut self) -> Option<NativeEvent> {
        self.inner.pop_native_event()
    }

    fn validate(&self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        assert!(!self.panic_validate, "test adapter validation unwind");
        self.inner.validate(mutations)
    }

    fn apply(&mut self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        if self.panic_apply {
            if let Some(mutation) = mutations.first() {
                self.inner.apply(std::slice::from_ref(mutation))?;
            }
            panic!("test adapter application unwind");
        }
        self.inner.apply(mutations)
    }

    fn focus(&mut self, object: ObjectId) -> Result<bool, Self::Error> {
        self.inner.focus(object)
    }
}

#[test]
fn adapter_validation_unwind_rolls_back_and_poisons_runtime() {
    let mut runtime = Runtime::new(PanicAdapter {
        panic_validate: true,
        ..Default::default()
    });

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        runtime.update(TextBox::new("Text")).unwrap();
    }));

    assert!(result.is_err());
    assert_eq!(runtime.graph().root(), None);
    assert_eq!(
        runtime.update(TextBox::new("Text")),
        Err(UpdateError::Poisoned)
    );
}

#[test]
fn adapter_application_unwind_rolls_back_and_poisons_runtime() {
    let mut runtime = Runtime::new(PanicAdapter {
        panic_apply: true,
        ..Default::default()
    });

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        runtime.update(TextBox::new("Text")).unwrap();
    }));

    assert!(result.is_err());
    assert_eq!(runtime.graph().root(), None);
    assert_eq!(
        runtime.update(TextBox::new("Text")),
        Err(UpdateError::Poisoned)
    );
}

fn referenced_virtual_view(nested: &ElementRef, row: &ElementRef) -> Grid {
    Grid::new().keyed_children([
        keyed("button", Button::new().element_ref(nested)),
        keyed(
            "repeater",
            ItemsRepeater::new().item("row", Button::new().element_ref(row)),
        ),
    ])
}

#[derive(Default)]
struct ControlledFailureAdapter {
    inner: RecordingAdapter,
    fail_validate: bool,
    fail_apply: bool,
}

impl Adapter for ControlledFailureAdapter {
    type Error = ();

    fn preview_native_events(&self, events: &mut Vec<NativeEvent>) {
        self.inner.preview_native_events(events);
    }

    fn pop_native_event(&mut self) -> Option<NativeEvent> {
        self.inner.pop_native_event()
    }

    fn validate(&self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        if self.fail_validate {
            Err(())
        } else {
            self.inner.validate(mutations).map_err(|_| ())
        }
    }

    fn apply(&mut self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        if self.fail_apply {
            Err(())
        } else {
            self.inner.apply(mutations).map_err(|_| ())
        }
    }

    fn focus(&mut self, object: ObjectId) -> Result<bool, Self::Error> {
        self.inner.focus(object).map_err(|_| ())
    }
}

fn realize_controlled_virtual_row(
    runtime: &mut Runtime<ControlledFailureAdapter>,
    collection: ObjectId,
) -> Result<(), UpdateError<()>> {
    runtime
        .adapter_mut()
        .inner
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    runtime.dispatch_native_events().map(|_| ())
}

#[test]
fn adapter_failures_clear_all_retained_references_before_discard() {
    for fail_validate in [true, false] {
        let nested: ElementRef = ElementRef::default();
        let row: ElementRef = ElementRef::default();
        let mut runtime = Runtime::new(ControlledFailureAdapter::default());
        runtime
            .update(referenced_virtual_view(&nested, &row))
            .unwrap();
        let root = runtime.graph().root().unwrap();
        let collection = runtime
            .graph()
            .children(root, RelationId::Children)
            .unwrap()[1];
        realize_controlled_virtual_row(&mut runtime, collection).unwrap();
        assert!(nested.get().is_some());
        assert!(row.get().is_some());

        runtime.adapter_mut().fail_validate = fail_validate;
        runtime.adapter_mut().fail_apply = !fail_validate;
        assert!(matches!(
            runtime.update(referenced_virtual_view(&nested, &row).width(1.0)),
            Err(UpdateError::Adapter(_))
        ));
        assert_eq!(nested.get(), None);
        assert_eq!(row.get(), None);
        assert_eq!(
            runtime.update(referenced_virtual_view(&nested, &row)),
            Err(UpdateError::Poisoned)
        );
    }
}

fn referenced_failure_subtree(
    primary: Option<(&'static str, &ElementRef)>,
    row: &ElementRef,
) -> Grid {
    let mut children = Vec::new();
    if let Some((key, reference)) = primary {
        children.push(keyed(key, Button::new().element_ref(reference)));
    }
    children.push(keyed(
        "repeater",
        ItemsRepeater::new().item("row", Button::new().element_ref(row)),
    ));
    Grid::new().keyed_children(children)
}

#[test]
fn subtree_adapter_failures_clear_detached_virtual_and_unrelated_references() {
    for fail_validate in [true, false] {
        let nested: ElementRef = ElementRef::default();
        let reused: ElementRef = ElementRef::default();
        let row: ElementRef = ElementRef::default();
        let outside: ElementRef = ElementRef::default();
        let replacement: ElementRef = ElementRef::default();
        let mut runtime = Runtime::new(ControlledFailureAdapter::default());
        runtime
            .update(Grid::new().keyed_children([
                keyed(
                    "target",
                    referenced_failure_subtree(Some(("nested", &nested)), &row),
                ),
                keyed("outside", Button::new().element_ref(&outside)),
            ]))
            .unwrap();
        let root = runtime.graph().root().unwrap();
        let children = runtime
            .graph()
            .children(root, RelationId::Children)
            .unwrap();
        let target = children[0];
        let collection = runtime
            .graph()
            .children(target, RelationId::Children)
            .unwrap()[1];
        realize_controlled_virtual_row(&mut runtime, collection).unwrap();
        let nested_id = nested.get().unwrap();

        runtime
            .update_subtree(target, referenced_failure_subtree(None, &row))
            .unwrap();
        assert_eq!(nested.get(), None);
        runtime
            .update_subtree(
                target,
                referenced_failure_subtree(Some(("reused", &reused)), &row),
            )
            .unwrap();
        let reused_id = reused.get().unwrap();
        assert_eq!(reused_id.index(), nested_id.index());
        assert_ne!(reused_id.generation(), nested_id.generation());
        assert!(row.get().is_some());
        assert!(outside.get().is_some());

        runtime.adapter_mut().fail_validate = fail_validate;
        runtime.adapter_mut().fail_apply = !fail_validate;
        assert!(matches!(
            runtime.update_subtree(
                target,
                Grid::new().keyed_children([keyed(
                    "replacement",
                    Button::new().element_ref(&replacement),
                )])
            ),
            Err(UpdateError::Adapter(_))
        ));
        assert_eq!(nested.get(), None);
        assert_eq!(reused.get(), None);
        assert_eq!(row.get(), None);
        assert_eq!(outside.get(), None);
        assert_eq!(replacement.get(), None);
        assert_eq!(runtime.graph().root(), None);
        assert_eq!(runtime.update(Grid::new()), Err(UpdateError::Poisoned));
    }
}

#[test]
fn adapter_unwinds_clear_all_retained_references() {
    for panic_validate in [true, false] {
        let nested: ElementRef = ElementRef::default();
        let row: ElementRef = ElementRef::default();
        let mut runtime = Runtime::new(PanicAdapter::default());
        runtime
            .update(referenced_virtual_view(&nested, &row))
            .unwrap();
        let root = runtime.graph().root().unwrap();
        let collection = runtime
            .graph()
            .children(root, RelationId::Children)
            .unwrap()[1];
        runtime
            .adapter_mut()
            .inner
            .queue_realization(RealizationRequest::Realize {
                collection,
                container: RealizedContainer(1),
                index: 0,
                source_revision: 0,
            });
        runtime.dispatch_native_events().unwrap();

        runtime.adapter_mut().panic_validate = panic_validate;
        runtime.adapter_mut().panic_apply = !panic_validate;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            runtime
                .update(referenced_virtual_view(&nested, &row).width(1.0))
                .unwrap();
        }));

        assert!(result.is_err());
        assert_eq!(nested.get(), None);
        assert_eq!(row.get(), None);
        assert_eq!(
            runtime.update(referenced_virtual_view(&nested, &row)),
            Err(UpdateError::Poisoned)
        );
    }
}

#[derive(Default)]
struct CompletionFailAdapter {
    inner: RecordingAdapter,
    fail_completion: bool,
}

impl Adapter for CompletionFailAdapter {
    type Error = ();

    fn preview_native_events(&self, events: &mut Vec<NativeEvent>) {
        self.inner.preview_native_events(events);
    }

    fn pop_native_event(&mut self) -> Option<NativeEvent> {
        self.inner.pop_native_event()
    }

    fn validate(&self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        self.inner.validate(mutations).map_err(|_| ())
    }

    fn apply(&mut self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        if self.fail_completion
            && mutations
                .iter()
                .any(|mutation| matches!(mutation, Mutation::CompleteRetirement { .. }))
        {
            return Err(());
        }
        self.inner.apply(mutations).map_err(|_| ())
    }

    fn focus(&mut self, object: ObjectId) -> Result<bool, Self::Error> {
        self.inner.focus(object).map_err(|_| ())
    }
}

#[test]
fn retirement_completion_errors_poison_the_runtime() {
    let mut runtime = Runtime::new(CompletionFailAdapter::default());
    runtime
        .update(Grid::new().keyed_children([keyed(
            "retiring",
            Button::new().exit_fade(std::time::Duration::from_millis(100)),
        )]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let retiring = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    runtime.update(Grid::new()).unwrap();
    runtime.adapter_mut().fail_completion = true;
    assert!(runtime.adapter_mut().inner.complete_retirement(retiring));

    assert_eq!(
        runtime.dispatch_native_events(),
        Err(UpdateError::Adapter(()))
    );
    assert_eq!(runtime.update(Grid::new()), Err(UpdateError::Poisoned));
}

#[test]
fn virtual_items_realize_only_requested_rows() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    let source = VirtualSource::new(1, 10_000, Key::from, |index| -> View {
        TextBlock::new().text(index.to_string()).into()
    });
    runtime
        .update(ItemsRepeater::new().virtual_source(source))
        .unwrap();
    let collection = runtime.graph().root().unwrap();
    assert_eq!(runtime.adapter().object_count(), 1);

    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(7),
            index: 9_999,
            source_revision: 0,
        });
    let Some(NativeWork::Virtual(VirtualWork::Realize {
        lease, index, view, ..
    })) = runtime.next_native_work().unwrap()
    else {
        panic!("expected realization");
    };
    runtime.realize_virtual(&lease, index, *view).unwrap();

    assert_eq!(runtime.adapter().object_count(), 2);
    assert_eq!(runtime.adapter().realized_count(collection), 1);
    assert_eq!(
        runtime
            .adapter()
            .children(collection, RelationId::Items)
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn virtual_items_recycle_and_reuse_containers_without_stale_ownership() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime
        .update(
            ItemsRepeater::new()
                .item(1_u64, TextBlock::new().text("one"))
                .item(2_u64, TextBlock::new().text("two")),
        )
        .unwrap();
    let collection = runtime.graph().root().unwrap();
    let realize = |runtime: &mut Runtime<RecordingAdapter>, container, index| -> RealizationLease {
        runtime
            .adapter_mut()
            .queue_realization(RealizationRequest::Realize {
                collection,
                container,
                index,
                source_revision: 0,
            });
        let Some(NativeWork::Virtual(VirtualWork::Realize {
            lease, index, view, ..
        })) = runtime.next_native_work().unwrap()
        else {
            panic!("expected realization");
        };
        runtime.realize_virtual(&lease, index, *view).unwrap();
        lease
    };
    let first = realize(&mut runtime, RealizedContainer(1), 0);
    let second = realize(&mut runtime, RealizedContainer(1), 1);
    assert_ne!(first.key, second.key);
    assert_eq!(runtime.adapter().object_count(), 2);
    assert_eq!(runtime.adapter().realized_count(collection), 1);

    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(1),
            source_revision: 0,
        });
    let Some(NativeWork::Virtual(VirtualWork::Recycle { lease })) =
        runtime.next_native_work().unwrap()
    else {
        panic!("expected recycle");
    };
    runtime.recycle_virtual(&lease).unwrap();
    assert_eq!(runtime.adapter().object_count(), 1);
    assert_eq!(runtime.adapter().realized_count(collection), 0);
}

#[test]
fn virtual_items_preserve_keyed_child_across_reorder() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime
        .update(
            ItemsRepeater::new()
                .item("a", TextBlock::new().text("a"))
                .item("b", TextBlock::new().text("b")),
        )
        .unwrap();
    let collection = runtime.graph().root().unwrap();
    for index in 0..2 {
        runtime
            .adapter_mut()
            .queue_realization(RealizationRequest::Realize {
                collection,
                container: RealizedContainer(index as u64),
                index,
                source_revision: 0,
            });
        let Some(NativeWork::Virtual(VirtualWork::Realize {
            lease, index, view, ..
        })) = runtime.next_native_work().unwrap()
        else {
            panic!("expected realization");
        };
        runtime.realize_virtual(&lease, index, *view).unwrap();
    }
    let before = runtime
        .adapter()
        .children(collection, RelationId::Items)
        .unwrap()
        .to_vec();

    runtime
        .update(
            ItemsRepeater::new()
                .item("b", TextBlock::new().text("b"))
                .item("a", TextBlock::new().text("updated")),
        )
        .unwrap();
    for _ in 0..2 {
        let Some(NativeWork::Virtual(VirtualWork::Realize {
            lease, index, view, ..
        })) = runtime.next_native_work().unwrap()
        else {
            panic!("expected refresh");
        };
        runtime.realize_virtual(&lease, index, *view).unwrap();
    }
    assert_eq!(
        runtime
            .adapter()
            .children(collection, RelationId::Items)
            .unwrap(),
        &[before[1], before[0]]
    );
}

#[test]
fn virtual_items_reject_duplicate_keys_without_mutation() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime.update(ItemsRepeater::new()).unwrap();
    let before = runtime.graph().clone();

    assert_eq!(
        runtime.update(
            ItemsRepeater::new()
                .item("duplicate", TextBlock::new().text("first"))
                .item("duplicate", TextBlock::new().text("second"))
        ),
        Err(UpdateError::Graph(GraphError::DuplicateKey(Key::from(
            "duplicate"
        ))))
    );
    assert_eq!(runtime.graph(), &before);
    assert_eq!(runtime.adapter().object_count(), 1);
}

#[test]
fn virtual_items_ignore_stale_source_requests() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime
        .update(ItemsRepeater::new().item("first", TextBlock::new().text("first")))
        .unwrap();
    let collection = runtime.graph().root().unwrap();
    runtime
        .update(ItemsRepeater::new().item("second", TextBlock::new().text("second")))
        .unwrap();
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });

    assert!(matches!(
        runtime.next_native_work().unwrap(),
        Some(NativeWork::Maintenance)
    ));
    assert!(runtime.next_native_work().unwrap().is_none());
    assert_eq!(runtime.adapter().object_count(), 1);
}

#[test]
fn virtual_items_remove_active_rows_when_source_becomes_empty() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime
        .update(ItemsRepeater::new().item("first", TextBlock::new().text("first")))
        .unwrap();
    let collection = runtime.graph().root().unwrap();
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    let Some(NativeWork::Virtual(VirtualWork::Realize {
        lease, index, view, ..
    })) = runtime.next_native_work().unwrap()
    else {
        panic!("expected realization");
    };
    runtime.realize_virtual(&lease, index, *view).unwrap();

    runtime.update(ItemsRepeater::new()).unwrap();
    assert_eq!(runtime.adapter().object_count(), 1);
    assert_eq!(runtime.adapter().realized_count(collection), 0);
    assert!(
        runtime
            .adapter()
            .children(collection, RelationId::Items)
            .unwrap()
            .is_empty()
    );
}

#[test]
fn virtual_realization_batches_validate_before_consumption() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime
        .update(ItemsRepeater::new().item("only", TextBlock::new().text("only")))
        .unwrap();
    let collection = runtime.graph().root().unwrap();
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(2),
            index: 1,
            source_revision: 0,
        });

    assert!(matches!(
        runtime.next_native_work(),
        Err(UpdateError::InvalidNativeEvent(GraphError::InvalidRealization(
            object,
            1
        ))) if object == collection
    ));
    assert_eq!(runtime.adapter().object_count(), 1);
    assert_eq!(
        runtime.update(ItemsRepeater::new()),
        Err(UpdateError::Poisoned)
    );
}

#[test]
fn duplicate_virtual_recycle_is_idempotent() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime
        .update(ItemsRepeater::new().item("only", TextBlock::new().text("only")))
        .unwrap();
    let collection = runtime.graph().root().unwrap();
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    runtime.dispatch_native_events().unwrap();
    for _ in 0..2 {
        runtime
            .adapter_mut()
            .queue_realization(RealizationRequest::Recycle {
                collection,
                container: RealizedContainer(1),
                source_revision: 0,
            });
    }
    runtime.dispatch_native_events().unwrap();
    assert_eq!(runtime.adapter().object_count(), 1);
    assert_eq!(runtime.adapter().realized_count(collection), 0);
}

#[test]
fn realized_virtual_children_recycle_before_repeater_destruction() {
    let mut adapter = RecordingAdapter::new();
    adapter.record_batches(true);
    let mut runtime = Runtime::new(adapter);
    runtime
        .update(Grid::new().keyed_children([keyed(
            "repeater",
            ItemsRepeater::new().item("row", TextBlock::new().text("row")),
        )]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let repeater = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection: repeater,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    runtime.dispatch_native_events().unwrap();

    runtime.update(Grid::new()).unwrap();
    let batch = runtime.adapter().batches().last().unwrap();
    let recycle = batch
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::Recycle { parent, .. } if *parent == repeater
            )
        })
        .unwrap();
    let destroy = batch
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::Destroy { object } if *object == repeater
            )
        })
        .unwrap();
    assert!(recycle < destroy);
    assert!(!batch.iter().any(|mutation| {
        matches!(
            mutation,
            Mutation::Remove {
                parent,
                relation: RelationId::Items,
                ..
            } if *parent == repeater
        )
    }));
}

#[test]
fn realized_virtual_children_recycle_before_repeater_replacement() {
    let mut adapter = RecordingAdapter::new();
    adapter.record_batches(true);
    let mut runtime = Runtime::new(adapter);
    runtime
        .update(Grid::new().keyed_children([keyed(
            "slot",
            ItemsRepeater::new().item("row", TextBlock::new().text("row")),
        )]))
        .unwrap();
    let root = runtime.graph().root().unwrap();
    let repeater = runtime
        .graph()
        .children(root, RelationId::Children)
        .unwrap()[0];
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection: repeater,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    runtime.dispatch_native_events().unwrap();

    runtime
        .update(Grid::new().keyed_children([keyed("slot", Border::new())]))
        .unwrap();
    let batch = runtime.adapter().batches().last().unwrap();
    let recycle = batch
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::Recycle { parent, .. } if *parent == repeater
            )
        })
        .unwrap();
    let replace = batch
        .iter()
        .position(|mutation| {
            matches!(
                mutation,
                Mutation::Destroy { object } if *object == repeater
            )
        })
        .unwrap();
    assert!(recycle < replace);
    assert!(!batch.iter().any(|mutation| {
        matches!(
            mutation,
            Mutation::Remove {
                parent,
                relation: RelationId::Items,
                ..
            } if *parent == repeater
        )
    }));
}

#[test]
fn update_peeks_pending_virtual_work_without_consuming_it() {
    let view = || ItemsRepeater::new().item("row", TextBlock::new().text("row"));
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime.update(view()).unwrap();
    let collection = runtime.graph().root().unwrap();
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });

    assert!(matches!(
        runtime.update(view()),
        Err(UpdateError::PendingNativeEvent)
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(runtime.adapter().realized_count(collection), 1);
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);

    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(1),
            source_revision: 0,
        });
    assert!(matches!(
        runtime.update(view()),
        Err(UpdateError::PendingNativeEvent)
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(runtime.adapter().realized_count(collection), 0);
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
}

#[test]
fn public_event_api_preserves_virtual_work_until_ordered_dispatch() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime
        .update(
            ItemsRepeater::new()
                .item("old", TextBlock::new().text("old"))
                .item("new", TextBlock::new().text("new")),
        )
        .unwrap();
    let collection = runtime.graph().root().unwrap();

    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    assert!(matches!(
        runtime.next_native_event(),
        Err(UpdateError::PendingNativeEvent)
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(runtime.adapter().realized_count(collection), 1);
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);

    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(1),
            source_revision: 0,
        });
    assert!(matches!(
        runtime.next_native_event(),
        Err(UpdateError::PendingNativeEvent)
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(runtime.adapter().realized_count(collection), 0);
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);

    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(2),
            index: 1,
            source_revision: 0,
        });
    assert!(matches!(
        runtime.next_native_event(),
        Err(UpdateError::PendingNativeEvent)
    ));
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(2),
            source_revision: 0,
        });
    assert!(matches!(
        runtime.next_native_event(),
        Err(UpdateError::PendingNativeEvent)
    ));
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(runtime.adapter().realized_count(collection), 0);
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
}

#[test]
fn recycle_before_dispatch_cancels_realization_and_allows_token_reuse() {
    let mut runtime = Runtime::new(RecordingAdapter::new());
    runtime
        .update(
            ItemsRepeater::new()
                .item("old", TextBlock::new().text("old"))
                .item("new", TextBlock::new().text("new")),
        )
        .unwrap();
    let collection = runtime.graph().root().unwrap();
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(1),
            index: 0,
            source_revision: 0,
        });
    assert!(matches!(
        runtime.update(
            ItemsRepeater::new()
                .item("old", TextBlock::new().text("old"))
                .item("new", TextBlock::new().text("new"))
        ),
        Err(UpdateError::PendingNativeEvent)
    ));
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Recycle {
            collection,
            container: RealizedContainer(1),
            source_revision: 0,
        });
    runtime
        .adapter_mut()
        .queue_realization(RealizationRequest::Realize {
            collection,
            container: RealizedContainer(2),
            index: 1,
            source_revision: 0,
        });

    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert_eq!(runtime.adapter().realized_count(collection), 1);
    assert_eq!(runtime.graph().object_count(), 2);
    assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
    assert!(runtime.update(ItemsRepeater::new()).is_ok());
}
