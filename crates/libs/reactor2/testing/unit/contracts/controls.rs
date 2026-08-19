use super::*;

#[test]
fn button_icon_reconciles_and_replaces_only_when_its_native_class_changes() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0_usize);
        *phase_for_render.borrow_mut() = Some(state.clone());
        match state.value() {
            0 => Button::new("Action")
                .icon(Icon::symbol(IconSymbol::FAVORITE))
                .build(),
            1 => Button::new("Updated")
                .icon(Icon::symbol(IconSymbol::SAVE))
                .build(),
            2 => Button::new("Updated")
                .icon(Icon::font("\u{E10F}", "Segoe Fluent Icons"))
                .build(),
            _ => Button::new("Updated").build(),
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let button = native_node(&reactor, NativeKind::Button);
    let stack = native_node(&reactor, NativeKind::StackPanel);
    let text = native_node(&reactor, NativeKind::TextBlock);
    let symbol = native_node(&reactor, NativeKind::SymbolIcon);

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::Button), button);
    assert_eq!(native_node(&reactor, NativeKind::StackPanel), stack);
    assert_eq!(native_node(&reactor, NativeKind::TextBlock), text);
    assert_eq!(native_node(&reactor, NativeKind::SymbolIcon), symbol);

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::Button), button);
    assert_eq!(native_node(&reactor, NativeKind::StackPanel), stack);
    assert_eq!(native_node(&reactor, NativeKind::TextBlock), text);
    assert!(!reactor.engine().runtime().contains(symbol));
    let font = native_node(&reactor, NativeKind::FontIcon);

    assert!(phase.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::Button), button);
    assert_eq!(native_node(&reactor, NativeKind::TextBlock), text);
    assert!(!reactor.engine().runtime().contains(stack));
    assert!(!reactor.engine().runtime().contains(font));
}

#[test]
fn shape_updates_retain_same_kind_and_replace_only_on_kind_change() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0_usize);
        *phase_for_render.borrow_mut() = Some(state.clone());
        match state.value() {
            0 => Shape::rectangle()
                .fill(Color::rgb(10, 20, 30))
                .corner_radius(4.0)
                .width(100.0)
                .height(40.0)
                .build(),
            1 => Shape::rectangle()
                .stroke(Color::rgb(30, 20, 10))
                .stroke_thickness(3.0)
                .width(120.0)
                .height(50.0)
                .build(),
            _ => Shape::ellipse()
                .fill(Color::rgb(40, 50, 60))
                .width(50.0)
                .height(50.0)
                .build(),
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let rectangle = native_node(&reactor, NativeKind::Rectangle);

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::Rectangle), rectangle);

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert!(!reactor.engine().runtime().contains(rectangle));
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .kind(native_node(&reactor, NativeKind::Ellipse)),
        Some(NativeKind::Ellipse)
    );
}

#[test]
fn text_box_chrome_updates_and_resets_without_replacing_the_control() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0_usize);
        *phase_for_render.borrow_mut() = Some(state.clone());
        if state.value() == 1 {
            TextBox::display("text")
                .background(Color::argb(0, 0, 0, 0))
                .border_brush(Color::rgb(60, 120, 220))
                .border_thickness(Thickness::uniform(2.0))
                .build()
        } else {
            TextBox::display("text").build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let text_box = native_node(&reactor, NativeKind::TextBox);

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::TextBox), text_box);

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::TextBox), text_box);
}

#[test]
fn virtual_list_dispatches_current_native_reorder_keys() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let hits = Rc::new(RefCell::new(Vec::new()));
    let version_for_render = Rc::clone(&version);
    let hits_for_render = Rc::clone(&hits);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        if current == 2 {
            text_block("removed")
        } else {
            let hits = Rc::clone(&hits_for_render);
            VirtualList::new(3, 240.0, |index| text_block(format!("Row {index}")))
                .item_keys(VirtualItemKeys::new([10, 20, 30]))
                .reorderable(move |keys| {
                    hits.borrow_mut().push((current, keys));
                })
                .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let list = native_node(&reactor, NativeKind::ListView);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ItemsReordered {
            target: list,
            keys: vec![30, 10, 20],
        });
    reactor.pump();
    assert_eq!(*hits.borrow(), [(1, vec![30, 10, 20])]);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ItemsReordered {
            target: list,
            keys: vec![20, 30, 10],
        });
    reactor.pump();
    assert_eq!(*hits.borrow(), [(1, vec![30, 10, 20])]);
}

#[test]
fn rejected_keyed_reorder_feedback_is_restored_on_rerender() {
    let revision = Rc::new(RefCell::new(None::<State<usize>>));
    let revision_for_render = Rc::clone(&revision);
    let events = Rc::new(Cell::new(0usize));
    let events_for_render = Rc::clone(&events);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *revision_for_render.borrow_mut() = Some(state.clone());
        _ = state.value();
        let tab_events = Rc::clone(&events_for_render);
        let list_events = Rc::clone(&events_for_render);
        StackPanel::new([
            TabView::new(
                [
                    TabViewItem::new(10, "first", text_block("first")),
                    TabViewItem::new(20, "second", text_block("second")),
                ],
                |_| {},
            )
            .reorderable(move |_| tab_events.set(tab_events.get() + 1))
            .build(),
            VirtualList::new(3, 100.0, |index| text_block(index.to_string()))
                .item_keys(VirtualItemKeys::new([10, 20, 30]))
                .reorderable(move |_| list_events.set(list_events.get() + 1))
                .build(),
        ])
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let tab = native_node(&reactor, NativeKind::TabView);
    let list = native_node(&reactor, NativeKind::ListView);
    let initial_children = reactor.engine().arena.get(tab).unwrap().children.clone();

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TabsReordered {
            target: tab,
            keys: vec![20, 10],
        });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ItemsReordered {
            target: list,
            keys: vec![30, 10, 20],
        });
    reactor.pump();
    assert_eq!(events.get(), 2);
    assert_eq!(
        reactor.engine().arena.get(tab).unwrap().children,
        [initial_children[1], initial_children[0]]
    );

    let batch_count = reactor.engine().runtime().batches().len();
    assert!(revision.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(
        reactor.engine().arena.get(tab).unwrap().children,
        initial_children
    );
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .skip(batch_count)
            .flatten()
            .any(|command| items_update(command)
                .is_some_and(|(id, keys)| id == list && keys == [10, 20, 30]))
    );
}

#[test]
fn auto_suggest_box_dispatches_current_controlled_events() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let hits = Rc::new(RefCell::new(Vec::new()));
    let version_for_render = Rc::clone(&version);
    let hits_for_render = Rc::clone(&hits);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        if current == 2 {
            text_block("removed")
        } else {
            let text_hits = Rc::clone(&hits_for_render);
            let query_hits = Rc::clone(&hits_for_render);
            let chosen_hits = Rc::clone(&hits_for_render);
            AutoSuggestBox::new(format!("query {current}"), move |value| {
                text_hits
                    .borrow_mut()
                    .push(format!("{current}:text:{value}"));
            })
            .items([(10, "Apple"), (20, "Apricot")])
            .placeholder_text("Search")
            .header("Fruit")
            .on_query_submitted(move |value| {
                query_hits
                    .borrow_mut()
                    .push(format!("{current}:query:{value}"));
            })
            .on_suggestion_chosen(move |key| {
                chosen_hits
                    .borrow_mut()
                    .push(format!("{current}:chosen:{key}"));
            })
            .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let control = native_node(&reactor, NativeKind::AutoSuggestBox);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    for event in [
        NativeEvent::TextChanged {
            target: control,
            value: "ap".into(),
        },
        NativeEvent::QuerySubmitted {
            target: control,
            value: "ap".into(),
        },
        NativeEvent::ItemInvoked {
            target: control,
            key: 20,
        },
    ] {
        reactor.engine().runtime().queue_event(event);
    }
    reactor.pump();
    assert_eq!(*hits.borrow(), ["1:text:ap", "1:query:ap", "1:chosen:20"]);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::QuerySubmitted {
            target: control,
            value: "stale".into(),
        });
    reactor.pump();
    assert_eq!(*hits.borrow(), ["1:text:ap", "1:query:ap", "1:chosen:20"]);
}

#[test]
fn breadcrumb_bar_updates_items_and_dispatches_current_key_handler() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let hits = Rc::new(RefCell::new(Vec::new()));
    let version_for_render = Rc::clone(&version);
    let hits_for_render = Rc::clone(&hits);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        if current == 2 {
            text_block("removed")
        } else {
            let items = if current == 0 {
                SelectorItems::new([(10, "Home"), (20, "Documents"), (30, "Report")])
            } else {
                SelectorItems::new([(30, "Report"), (10, "Home"), (20, "Documents")])
            };
            let hits = Rc::clone(&hits_for_render);
            BreadcrumbBar::from_items(items)
                .on_item_clicked(move |key| {
                    hits.borrow_mut().push((current, key));
                })
                .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let bar = native_node(&reactor, NativeKind::BreadcrumbBar);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::BreadcrumbBar), bar);
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ItemInvoked {
            target: bar,
            key: 20,
        });
    reactor.pump();
    assert_eq!(*hits.borrow(), [(1, 20)]);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ItemInvoked {
            target: bar,
            key: 10,
        });
    reactor.pump();
    assert_eq!(*hits.borrow(), [(1, 20)]);
}

#[test]
#[should_panic(expected = "SelectorBar item keys must be unique")]
fn selector_bar_rejects_duplicate_item_keys() {
    let _ = SelectorBar::display([
        SelectorBarItem::new(1, "First"),
        SelectorBarItem::new(1, "Second"),
    ]);
}

#[test]
#[should_panic(expected = "SelectorBar selected key is not present")]
fn selector_bar_rejects_an_unknown_selected_key() {
    let _ = SelectorBar::display([SelectorBarItem::new(1, "First")])
        .selected_key(Some(2))
        .build();
}

#[test]
fn selector_bar_reconciles_keyed_items_and_current_selection_handler() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let hits = Rc::new(RefCell::new(Vec::new()));
    let version_for_render = Rc::clone(&version);
    let hits_for_render = Rc::clone(&hits);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        if current == 2 {
            text_block("removed")
        } else {
            let items = [
                SelectorBarItem::new(10, format!("First {current}")),
                SelectorBarItem::new(20, "Second").icon(Icon::symbol(IconSymbol::PEOPLE)),
            ];
            let items = if current == 0 {
                items.into_iter().collect::<Vec<_>>()
            } else {
                items.into_iter().rev().collect()
            };
            let hits = Rc::clone(&hits_for_render);
            SelectorBar::new(items, move |key| {
                hits.borrow_mut().push((current, key));
            })
            .selected_key(Some(if current == 0 { 10 } else { 20 }))
            .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let selector = native_node(&reactor, NativeKind::SelectorBar);
    let items = created_nodes(&reactor, NativeKind::SelectorBarItem);
    assert_eq!(items.len(), 2);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::SelectorBar), selector);
    assert_eq!(created_nodes(&reactor, NativeKind::SelectorBarItem), items);
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::SelectedKeyChanged {
            target: selector,
            key: Some(10),
        });
    reactor.pump();
    assert_eq!(*hits.borrow(), [(1, Some(10))]);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::SelectedKeyChanged {
            target: selector,
            key: Some(20),
        });
    reactor.pump();
    assert_eq!(*hits.borrow(), [(1, Some(10))]);
}

#[test]
fn navigation_view_reconciles_keyed_regions_and_current_selection_handler() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let hits = Rc::new(RefCell::new(Vec::new()));
    let pane_hits = Rc::new(RefCell::new(Vec::new()));
    let mode_hits = Rc::new(RefCell::new(Vec::new()));
    let version_for_render = Rc::clone(&version);
    let hits_for_render = Rc::clone(&hits);
    let pane_hits_for_render = Rc::clone(&pane_hits);
    let mode_hits_for_render = Rc::clone(&mode_hits);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        if current == 2 {
            text_block("removed")
        } else {
            let items = [
                NavigationItem::new(10, format!("First {current}"))
                    .icon(Icon::symbol(IconSymbol::HOME)),
                NavigationItem::new(20, "Second").icon(Icon::path("M 0,0 L 1,1")),
            ];
            let items = if current == 0 {
                items.into_iter().collect::<Vec<_>>()
            } else {
                items.into_iter().rev().collect()
            };
            let hits = Rc::clone(&hits_for_render);
            let pane_hits = Rc::clone(&pane_hits_for_render);
            let mode_hits = Rc::clone(&mode_hits_for_render);
            NavigationView::new(
                items,
                text_block(format!("content {current}")),
                move |key| {
                    hits.borrow_mut().push((current, key));
                },
            )
            .pane_footer(text_block(format!("footer {current}")))
            .selected_key(Some(if current == 0 { 10 } else { 20 }))
            .pane_open(true, move |open| {
                pane_hits.borrow_mut().push((current, open));
            })
            .on_display_mode_changed(move |mode| {
                mode_hits.borrow_mut().push((current, mode));
            })
            .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let navigation = native_node(&reactor, NativeKind::NavigationView);
    let items = created_nodes(&reactor, NativeKind::NavigationViewItem);
    let text = created_nodes(&reactor, NativeKind::TextBlock);
    assert_eq!(items.len(), 2);
    assert_eq!(text.len(), 2);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(
        native_node(&reactor, NativeKind::NavigationView),
        navigation
    );
    assert_eq!(
        created_nodes(&reactor, NativeKind::NavigationViewItem),
        items
    );
    assert_eq!(created_nodes(&reactor, NativeKind::TextBlock), text);
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::SelectedKeyChanged {
            target: navigation,
            key: None,
        });
    reactor.pump();
    assert_eq!(*hits.borrow(), [(1, None)]);
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::NavigationPaneOpenChanged {
            target: navigation,
            open: false,
        });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::NavigationDisplayModeChanged {
            target: navigation,
            mode: NavigationDisplayMode::Compact,
        });
    reactor.pump();
    assert_eq!(*pane_hits.borrow(), [(1, false)]);
    assert_eq!(*mode_hits.borrow(), [(1, NavigationDisplayMode::Compact)]);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert!(!reactor.engine().contains(navigation));
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::SelectedKeyChanged {
            target: navigation,
            key: Some(20),
        });
    reactor.pump();
    assert_eq!(*hits.borrow(), [(1, None)]);
    assert_eq!(*pane_hits.borrow(), [(1, false)]);
    assert_eq!(*mode_hits.borrow(), [(1, NavigationDisplayMode::Compact)]);
}

#[test]
fn navigation_pane_feedback_attaches_replaces_and_removes_without_replacement() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let hits = Rc::new(RefCell::new(Vec::new()));
    let hits_for_render = Rc::clone(&hits);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(state.clone());
        let current = state.value();
        let navigation = NavigationView::new(
            [NavigationItem::new(1, "page")],
            text_block("content"),
            |_| {},
        );
        if current == 0 || current == 3 {
            navigation.display_pane_open(true).build()
        } else {
            let hits = Rc::clone(&hits_for_render);
            navigation
                .pane_open(true, move |open| {
                    hits.borrow_mut().push((current, open));
                })
                .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let navigation = native_node(&reactor, NativeKind::NavigationView);

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::NavigationPaneOpenChanged {
            target: navigation,
            open: false,
        });
    reactor.pump();
    assert!(hits.borrow().is_empty());

    for current in [1, 2] {
        assert!(phase.borrow().as_ref().unwrap().try_set(current));
        reactor.pump();
        assert_eq!(
            native_node(&reactor, NativeKind::NavigationView),
            navigation
        );
        reactor
            .engine()
            .runtime()
            .queue_event(NativeEvent::NavigationPaneOpenChanged {
                target: navigation,
                open: false,
            });
        reactor.pump();
    }
    assert_eq!(*hits.borrow(), [(1, false), (2, false)]);

    assert!(phase.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::NavigationPaneOpenChanged {
            target: navigation,
            open: false,
        });
    reactor.pump();
    assert_eq!(*hits.borrow(), [(1, false), (2, false)]);
}

#[test]
fn rich_edit_box_reconciles_and_dispatches_current_text_handler() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let hits = Rc::new(RefCell::new(Vec::new()));
    let version_for_render = Rc::clone(&version);
    let hits_for_render = Rc::clone(&hits);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        if current == 2 {
            text_block("removed")
        } else {
            let hits = Rc::clone(&hits_for_render);
            RichEditBox::new(format!("value {current}"), move |text| {
                hits.borrow_mut().push((current, text));
            })
            .header(format!("header {current}"))
            .placeholder_text(format!("placeholder {current}"))
            .read_only(current == 1)
            .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::RichEditBox);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::RichEditBox), target);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::RichEditBox(update)),
                } if *id == target
                    && **update == RichEditBoxUpdate {
                        text: "value 1".into(),
                        header: Some("header 1".into()),
                        placeholder: Some("placeholder 1".into()),
                        read_only: true,
                    }
            ))
    );

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TextChanged {
            target,
            value: "edited".into(),
        });
    reactor.pump();
    assert_eq!(*hits.borrow(), [(1, String::from("edited"))]);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TextChanged {
            target,
            value: "stale".into(),
        });
    reactor.pump();
    assert_eq!(*hits.borrow(), [(1, String::from("edited"))]);
}

#[test]
fn rich_text_block_reconciles_document_descriptors() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let version_for_render = Rc::clone(&version);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        RichTextBlock::new([RichTextParagraph::new([
            RichTextInline::Run(RichTextRun {
                text: format!("value {current}"),
                bold: current == 1,
                italic: false,
            }),
            RichTextInline::LineBreak,
        ])])
        .font_size(if current == 0 { 12.0 } else { 18.0 })
        .selectable(current == 1)
        .wrap(current == 1)
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::RichTextBlock);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::RichTextBlock), target);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::RichTextBlock(update)),
                } if *id == target
                    && update.font_size == Some(18.0)
                    && update.selectable
                    && update.wrap
                    && update.paragraphs[0].inlines[0]
                        == RichTextInline::Run(RichTextRun {
                            text: "value 1".into(),
                            bold: true,
                            italic: false,
                        })
            ))
    );
}

#[test]
fn tree_view_reconciles_keyed_descriptors_and_current_handler() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let hits = Rc::new(RefCell::new(Vec::new()));
    let expansions = Rc::new(RefCell::new(Vec::new()));
    let version_for_render = Rc::clone(&version);
    let hits_for_render = Rc::clone(&hits);
    let expansions_for_render = Rc::clone(&expansions);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        if current == 2 {
            text_block("removed")
        } else {
            let nodes = if current == 0 {
                vec![
                    TreeNode::new(10, "First")
                        .expanded(true)
                        .child(TreeNode::new(11, "Child")),
                    TreeNode::new(20, "Second"),
                ]
            } else {
                vec![
                    TreeNode::new(20, "Second updated"),
                    TreeNode::new(10, "First")
                        .expanded(true)
                        .child(TreeNode::new(11, "Child")),
                ]
            };
            let hits = Rc::clone(&hits_for_render);
            let expansions = Rc::clone(&expansions_for_render);
            TreeView::new(nodes, move |key, expanded| {
                expansions.borrow_mut().push((current, key, expanded));
            })
            .on_item_invoked(move |key| hits.borrow_mut().push((current, key)))
            .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TreeView);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::TreeView), target);
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ItemInvoked { target, key: 11 });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TreeNodeExpandedChanged {
            target,
            key: 20,
            expanded: true,
        });
    reactor.pump();
    assert_eq!(*hits.borrow(), [(1, 11)]);
    assert_eq!(*expansions.borrow(), [(1, 20, true)]);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ItemInvoked { target, key: 20 });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TreeNodeExpandedChanged {
            target,
            key: 20,
            expanded: false,
        });
    reactor.pump();
    assert_eq!(*hits.borrow(), [(1, 11)]);
    assert_eq!(*expansions.borrow(), [(1, 20, true)]);
}

#[test]
fn tab_view_reconciles_keyed_items_and_dispatches_current_events() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let selections = Rc::new(RefCell::new(Vec::new()));
    let closes = Rc::new(RefCell::new(Vec::new()));
    let adds = Rc::new(RefCell::new(Vec::new()));
    let reorders = Rc::new(RefCell::new(Vec::new()));
    let version_for_render = Rc::clone(&version);
    let selections_for_render = Rc::clone(&selections);
    let closes_for_render = Rc::clone(&closes);
    let adds_for_render = Rc::clone(&adds);
    let reorders_for_render = Rc::clone(&reorders);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        if current == 2 {
            text_block("removed")
        } else {
            let items = [
                TabViewItem::new(10, format!("First {current}"), text_block("first"))
                    .closable(false),
                TabViewItem::new(20, "Second", text_block("second")),
            ];
            let items = if current == 0 {
                items.into_iter().collect::<Vec<_>>()
            } else {
                items.into_iter().rev().collect()
            };
            let selections = Rc::clone(&selections_for_render);
            let closes = Rc::clone(&closes_for_render);
            let adds = Rc::clone(&adds_for_render);
            let reorders = Rc::clone(&reorders_for_render);
            let tab = TabView::new(items, move |index| {
                selections.borrow_mut().push((current, index));
            })
            .selected_index(Some(current))
            .is_add_tab_button_visible(current != 0)
            .on_close_requested(move |key| {
                closes.borrow_mut().push((current, key));
            })
            .on_add_tab_button_click(move || {
                adds.borrow_mut().push(current);
            });
            let tab = if current == 0 {
                tab
            } else {
                tab.reorderable(move |keys| {
                    reorders.borrow_mut().push((current, keys));
                })
            };
            tab.build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let tab_view = native_node(&reactor, NativeKind::TabView);
    let items = created_nodes(&reactor, NativeKind::TabViewItem);
    assert_eq!(items.len(), 2);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::TabView), tab_view);
    assert_eq!(created_nodes(&reactor, NativeKind::TabViewItem), items);
    let runtime = reactor.engine().runtime();
    runtime.queue_event(NativeEvent::IndexChanged {
        target: tab_view,
        index: Some(0),
    });
    runtime.queue_event(NativeEvent::TabCloseRequested {
        target: tab_view,
        key: 20,
    });
    runtime.queue_event(NativeEvent::AddTabButtonClick { target: tab_view });
    runtime.queue_event(NativeEvent::TabsReordered {
        target: tab_view,
        keys: vec![10, 20],
    });
    reactor.pump();
    assert_eq!(*selections.borrow(), [(1, Some(0))]);
    assert_eq!(*closes.borrow(), [(1, 20)]);
    assert_eq!(*adds.borrow(), [1]);
    assert_eq!(*reorders.borrow(), [(1, vec![10, 20])]);
    assert_eq!(reactor.engine().runtime().children(tab_view), items);
    assert_eq!(
        reactor.engine().runtime().attachment(items[0]),
        Some(Attachment::Item { index: 0 })
    );
    assert_eq!(
        reactor.engine().runtime().attachment(items[1]),
        Some(Attachment::Item { index: 1 })
    );

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert!(!reactor.engine().runtime().contains(tab_view));
    let runtime = reactor.engine().runtime();
    runtime.queue_event(NativeEvent::IndexChanged {
        target: tab_view,
        index: Some(1),
    });
    runtime.queue_event(NativeEvent::TabCloseRequested {
        target: tab_view,
        key: 10,
    });
    runtime.queue_event(NativeEvent::AddTabButtonClick { target: tab_view });
    runtime.queue_event(NativeEvent::TabsReordered {
        target: tab_view,
        keys: vec![20, 10],
    });
    reactor.pump();
    assert_eq!(*selections.borrow(), [(1, Some(0))]);
    assert_eq!(*closes.borrow(), [(1, 20)]);
    assert_eq!(*adds.borrow(), [1]);
    assert_eq!(*reorders.borrow(), [(1, vec![10, 20])]);
}

#[test]
fn flip_view_reconciles_keyed_items_selection_and_current_handler() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let hits = Rc::new(RefCell::new(Vec::new()));
    let version_for_render = Rc::clone(&version);
    let hits_for_render = Rc::clone(&hits);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        if current == 2 {
            text_block("removed")
        } else {
            let hits = Rc::clone(&hits_for_render);
            let items = [
                FlipViewItem::new(10, text_block("first")),
                FlipViewItem::new(20, text_block("second")),
            ];
            let items = if current == 0 {
                items.into_iter().collect::<Vec<_>>()
            } else {
                items.into_iter().rev().collect()
            };
            FlipView::new(items, move |index| {
                hits.borrow_mut().push((current, index));
            })
            .selected_index(Some(current))
            .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let flip_view = native_node(&reactor, NativeKind::FlipView);
    let items = created_nodes(&reactor, NativeKind::TextBlock);
    assert_eq!(items.len(), 2);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::FlipView), flip_view);
    assert_eq!(created_nodes(&reactor, NativeKind::TextBlock), items);
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::IndexChanged {
            target: flip_view,
            index: Some(0),
        });
    reactor.pump();
    assert_eq!(*hits.borrow(), [(1, Some(0))]);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert!(!reactor.engine().runtime().contains(flip_view));
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::IndexChanged {
            target: flip_view,
            index: Some(1),
        });
    reactor.pump();
    assert_eq!(*hits.borrow(), [(1, Some(0))]);
}

#[test]
fn pivot_reconciles_keyed_items_selection_and_current_handler() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let hits = Rc::new(RefCell::new(Vec::new()));
    let version_for_render = Rc::clone(&version);
    let hits_for_render = Rc::clone(&hits);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        if current == 2 {
            text_block("removed")
        } else {
            let hits = Rc::clone(&hits_for_render);
            let items = [
                PivotItem::new(10, format!("First {current}"), text_block("first")),
                PivotItem::new(20, "Second", text_block("second")),
            ];
            let items = if current == 0 {
                items.into_iter().collect::<Vec<_>>()
            } else {
                items.into_iter().rev().collect()
            };
            Pivot::new(items, move |index| {
                hits.borrow_mut().push((current, index));
            })
            .title(format!("Pivot {current}"))
            .selected_index(Some(current))
            .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let pivot = native_node(&reactor, NativeKind::Pivot);
    let items = created_nodes(&reactor, NativeKind::PivotItem);
    assert_eq!(items.len(), 2);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::Pivot), pivot);
    assert_eq!(created_nodes(&reactor, NativeKind::PivotItem), items);
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::IndexChanged {
            target: pivot,
            index: Some(0),
        });
    reactor.pump();
    assert_eq!(*hits.borrow(), [(1, Some(0))]);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert!(!reactor.engine().runtime().contains(pivot));
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::IndexChanged {
            target: pivot,
            index: Some(1),
        });
    reactor.pump();
    assert_eq!(*hits.borrow(), [(1, Some(0))]);
}

#[test]
fn menu_bar_reconciles_recursive_items_and_dispatches_current_keyed_handlers() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let hits = Rc::new(RefCell::new(Vec::new()));
    let version_for_render = Rc::clone(&version);
    let hits_for_render = Rc::clone(&hits);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        if current == 2 {
            text_block("removed")
        } else {
            let hits = Rc::clone(&hits_for_render);
            MenuBar::new(vec![MenuBarItem::new(
                1,
                if current == 0 { "File" } else { "Updated" },
                vec![
                    MenuItem::new(2, "Open", move || {
                        hits.borrow_mut().push(current);
                    }),
                    MenuItem::separator(3),
                    MenuItem::submenu(4, "Recent", vec![MenuItem::new(5, "doc.txt", || {})]),
                ],
            )])
            .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let menu = native_node(&reactor, NativeKind::MenuBar);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| menu_bar_update(command)
                .is_some_and(|(id, items)| { id == menu && items[0].title == "File" }))
    );

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::MenuBar), menu);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| menu_bar_update(command)
                .is_some_and(|(id, items)| { id == menu && items[0].title == "Updated" }))
    );
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::MenuItemClick {
            target: menu,
            key: 2,
        });
    reactor.pump();
    assert_eq!(*hits.borrow(), [1]);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::MenuItemClick {
            target: menu,
            key: 2,
        });
    reactor.pump();
    assert_eq!(*hits.borrow(), [1]);
}

#[test]
fn button_and_drop_down_menu_flyouts_share_recursive_keyed_descriptors() {
    let hits = Rc::new(RefCell::new(Vec::new()));
    let button_hits = Rc::clone(&hits);
    let drop_down_hits = Rc::clone(&hits);
    let menu_opened_hits = Rc::clone(&hits);
    let owner_opened_hits = Rc::clone(&hits);
    let root = StackPanel::new([
        Button::new("Button menu")
            .menu_flyout(MenuFlyout::new(vec![MenuItem::new(
                10,
                "Button item",
                move || {
                    button_hits.borrow_mut().push(10);
                },
            )]))
            .build(),
        DropDownButton::with_menu(
            "Drop-down menu",
            MenuFlyout::new(vec![MenuItem::submenu(
                20,
                "More",
                vec![MenuItem::new(21, "Nested item", move || {
                    drop_down_hits.borrow_mut().push(21);
                })],
            )])
            .on_opened(move || menu_opened_hits.borrow_mut().push(30)),
        )
        .on_opened(move || owner_opened_hits.borrow_mut().push(31))
        .build(),
    ])
    .build();
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let owners = [
        native_node(&reactor, NativeKind::Button),
        native_node(&reactor, NativeKind::DropDownButton),
    ];
    let flyouts = created_nodes(&reactor, NativeKind::MenuFlyout);
    assert_eq!(flyouts.len(), 2);
    assert_eq!(
        flyouts
            .iter()
            .map(|id| reactor.engine().runtime().relation_owner(*id).unwrap())
            .collect::<Vec<_>>(),
        owners
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(menu_flyout_update)
            .count(),
        2
    );

    for (target, key) in [(flyouts[0], 10), (flyouts[1], 21)] {
        reactor
            .engine()
            .runtime()
            .queue_event(NativeEvent::MenuItemClick { target, key });
    }
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::FlyoutOpened { target: flyouts[1] });
    reactor.pump();
    assert_eq!(*hits.borrow(), [10, 21, 30, 31]);
}

#[test]
fn command_bar_flyout_reuses_keyed_command_nodes_and_current_callbacks() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let hits = Rc::new(RefCell::new(Vec::new()));
    let version_for_render = Rc::clone(&version);
    let hits_for_render = Rc::clone(&hits);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        if current == 2 {
            text_block("removed")
        } else {
            let primary_hits = Rc::clone(&hits_for_render);
            let alternate_hits = Rc::clone(&hits_for_render);
            let secondary_hits = Rc::clone(&hits_for_render);
            let opened_hits = Rc::clone(&hits_for_render);
            let closed_hits = Rc::clone(&hits_for_render);
            let primary = [
                CommandBarItem::button(10, format!("Primary {current}"), move || {
                    primary_hits.borrow_mut().push(current);
                }),
                CommandBarItem::button(11, "Alternate", move || {
                    alternate_hits.borrow_mut().push(current + 1);
                }),
            ];
            let primary = if current == 0 {
                primary.into_iter().collect::<Vec<_>>()
            } else {
                primary.into_iter().rev().collect()
            };
            Button::new("Commands")
                .command_bar_flyout(CommandBarFlyout::new(primary).secondary_commands([
                    CommandBarItem::button(20, "Secondary", move || {
                        secondary_hits.borrow_mut().push(current + 10);
                    }),
                ]))
                .flyout_placement(if current == 0 {
                    FlyoutPlacement::Bottom
                } else {
                    FlyoutPlacement::Top
                })
                .on_flyout_opened(move || opened_hits.borrow_mut().push(current + 100))
                .on_flyout_closed(move || closed_hits.borrow_mut().push(current + 200))
                .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let owner = native_node(&reactor, NativeKind::Button);
    let flyout = native_node(&reactor, NativeKind::CommandBarFlyout);
    let commands = created_nodes(&reactor, NativeKind::AppBarButton);
    assert_eq!(commands.len(), 3);
    assert_eq!(
        reactor.engine().runtime().relation_owner(flyout),
        Some(owner)
    );

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::CommandBarFlyout), flyout);
    assert_eq!(created_nodes(&reactor, NativeKind::AppBarButton), commands);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| {
                flyout_placement_update(command) == Some((flyout, FlyoutPlacement::Top))
            })
    );
    for target in &commands {
        reactor
            .engine()
            .runtime()
            .queue_event(NativeEvent::Click { target: *target });
    }
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::FlyoutOpened { target: flyout });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::FlyoutClosed { target: flyout });
    reactor.pump();
    assert_eq!(*hits.borrow(), [1, 2, 11, 101, 201]);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert!(!reactor.engine().runtime().contains(flyout));
    assert!(
        commands
            .iter()
            .all(|target| !reactor.engine().runtime().contains(*target))
    );
    for target in &commands {
        reactor
            .engine()
            .runtime()
            .queue_event(NativeEvent::Click { target: *target });
    }
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::FlyoutOpened { target: flyout });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::FlyoutClosed { target: flyout });
    reactor.pump();
    assert_eq!(*hits.borrow(), [1, 2, 11, 101, 201]);
}

#[test]
fn click_handler_updates_component_state_in_the_same_pump() {
    let root = component(|cx| {
        let count = cx.use_state(|| 0usize);
        let update = count.clone();
        button(format!("Count: {}", count.get().unwrap()), move || {
            assert!(update.try_set(update.get().unwrap() + 1));
        })
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = button_node(&reactor);
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Click { target });

    reactor.pump();

    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "Count: 1"))
    );
}

#[test]
fn split_button_reconciles_content_uses_current_handler_and_ignores_stale_clicks() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let hits = Rc::new(Cell::new(0usize));
    let version_for_render = Rc::clone(&version);
    let hits_for_render = Rc::clone(&hits);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        if current == 2 {
            text_block("removed")
        } else {
            let hits = Rc::clone(&hits_for_render);
            SplitButton::new(format!("Split {current}"))
                .on_click(move || hits.set(hits.get() + current + 1))
                .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::SplitButton);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::SplitButton), target);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "Split 1"))
    );
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Click { target });
    reactor.pump();
    assert_eq!(hits.get(), 2);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert!(!reactor.engine().contains(target));
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Click { target });
    reactor.pump();
    assert_eq!(hits.get(), 2);
}

#[test]
fn split_button_flyout_reconciles_as_an_owner_bound_accessory() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let log = Rc::new(RefCell::new(Vec::new()));
    let version_for_render = Rc::clone(&version);
    let log_for_render = Rc::clone(&log);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        if current == 2 {
            text_block("removed")
        } else {
            let click_log = Rc::clone(&log_for_render);
            let opened_log = Rc::clone(&log_for_render);
            let closed_log = Rc::clone(&log_for_render);
            SplitButton::new(format!("Split {current}"))
                .on_click(move || click_log.borrow_mut().push(("click", current)))
                .flyout(if current == 0 {
                    text_block("initial split flyout")
                } else {
                    Grid::new([text_block("updated split flyout")]).build()
                })
                .flyout_placement(if current == 0 {
                    FlyoutPlacement::Right
                } else {
                    FlyoutPlacement::Left
                })
                .on_flyout_opened(move || {
                    opened_log.borrow_mut().push(("opened", current));
                })
                .on_flyout_closed(move || {
                    closed_log.borrow_mut().push(("closed", current));
                })
                .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let owner = native_node(&reactor, NativeKind::SplitButton);
    let flyout = native_node(&reactor, NativeKind::Flyout);
    let initial_content = reactor.engine().runtime().children(flyout)[0];
    assert_eq!(
        reactor.engine().runtime().relation_owner(flyout),
        Some(owner)
    );
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| {
                flyout_placement_update(command) == Some((flyout, FlyoutPlacement::Right))
            })
    );

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    let replacement_content = native_node(&reactor, NativeKind::Grid);
    assert_eq!(native_node(&reactor, NativeKind::SplitButton), owner);
    assert_eq!(native_node(&reactor, NativeKind::Flyout), flyout);
    assert!(!reactor.engine().runtime().contains(initial_content));
    assert_eq!(
        reactor.engine().runtime().children(flyout),
        &[replacement_content]
    );
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| {
                flyout_placement_update(command) == Some((flyout, FlyoutPlacement::Left))
            })
    );

    for event in [
        NativeEvent::Click { target: owner },
        NativeEvent::FlyoutOpened { target: flyout },
        NativeEvent::FlyoutClosed { target: flyout },
    ] {
        reactor.engine().runtime().queue_event(event);
    }
    reactor.pump();
    assert_eq!(*log.borrow(), [("click", 1), ("opened", 1), ("closed", 1)]);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert!(!reactor.engine().runtime().contains(owner));
    assert!(!reactor.engine().runtime().contains(flyout));
}

#[test]
fn button_flyout_reconciles_owner_content_placement_and_current_handlers() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let log = Rc::new(RefCell::new(Vec::new()));
    let version_for_render = Rc::clone(&version);
    let log_for_render = Rc::clone(&log);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        if current == 2 {
            text_block("removed")
        } else {
            let click_log = Rc::clone(&log_for_render);
            let opened_log = Rc::clone(&log_for_render);
            let closed_log = Rc::clone(&log_for_render);
            Button::new(format!("Open {current}"))
                .on_click(move || click_log.borrow_mut().push(("click", current)))
                .flyout(if current == 0 {
                    text_block("initial flyout")
                } else {
                    Grid::new([text_block("updated flyout")]).build()
                })
                .flyout_placement(if current == 0 {
                    FlyoutPlacement::Bottom
                } else {
                    FlyoutPlacement::Top
                })
                .on_flyout_opened(move || {
                    opened_log.borrow_mut().push(("opened", current));
                })
                .on_flyout_closed(move || {
                    closed_log.borrow_mut().push(("closed", current));
                })
                .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let owner = native_node(&reactor, NativeKind::Button);
    let flyout = native_node(&reactor, NativeKind::Flyout);
    let initial_content = reactor.engine().runtime().children(flyout)[0];
    assert_eq!(
        reactor.engine().runtime().relation_owner(flyout),
        Some(owner)
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .find_map(flyout_placement_update),
        Some((flyout, FlyoutPlacement::Bottom))
    );

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    let replacement_content = native_node(&reactor, NativeKind::Grid);
    assert_eq!(native_node(&reactor, NativeKind::Button), owner);
    assert_eq!(native_node(&reactor, NativeKind::Flyout), flyout);
    assert!(!reactor.engine().runtime().contains(initial_content));
    assert_eq!(
        reactor.engine().runtime().children(flyout),
        &[replacement_content]
    );
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| {
                flyout_placement_update(command) == Some((flyout, FlyoutPlacement::Top))
            })
    );

    for event in [
        NativeEvent::Click { target: owner },
        NativeEvent::FlyoutOpened { target: flyout },
        NativeEvent::FlyoutClosed { target: flyout },
    ] {
        reactor.engine().runtime().queue_event(event);
    }
    reactor.pump();
    assert_eq!(*log.borrow(), [("click", 1), ("opened", 1), ("closed", 1)]);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert!(!reactor.engine().runtime().contains(owner));
    assert!(!reactor.engine().runtime().contains(flyout));
    for event in [
        NativeEvent::Click { target: owner },
        NativeEvent::FlyoutOpened { target: flyout },
        NativeEvent::FlyoutClosed { target: flyout },
    ] {
        reactor.engine().runtime().queue_event(event);
    }
    reactor.pump();
    assert_eq!(*log.borrow(), [("click", 1), ("opened", 1), ("closed", 1)]);
}

#[test]
fn repeat_button_reconciles_properties_uses_current_handler_and_ignores_stale_clicks() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let hits = Rc::new(Cell::new(0usize));
    let version_for_render = Rc::clone(&version);
    let hits_for_render = Rc::clone(&hits);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let version = state.get().unwrap();
        if version == 2 {
            text_block("removed")
        } else {
            let hits = Rc::clone(&hits_for_render);
            RepeatButton::new(format!("Repeat {version}"))
                .on_click(move || {
                    hits.set(hits.get() + version + 1);
                })
                .delay(500 + version as i32)
                .interval(33 + version as i32)
                .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::RepeatButton);
    let commands = reactor.engine().runtime().batches().last().unwrap();
    assert!(commands.iter().any(|command| {
        repeat_button_update(command) == Some((target, RepeatButtonUpdate::Delay(500)))
    }));
    assert!(commands.iter().any(|command| {
        repeat_button_update(command) == Some((target, RepeatButtonUpdate::Interval(33)))
    }));

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::RepeatButton), target);
    let commands = reactor.engine().runtime().batches().last().unwrap();
    assert!(commands.iter().any(|command| {
        repeat_button_update(command) == Some((target, RepeatButtonUpdate::Delay(501)))
    }));
    assert!(commands.iter().any(|command| {
        repeat_button_update(command) == Some((target, RepeatButtonUpdate::Interval(34)))
    }));
    assert!(
        commands
            .iter()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "Repeat 1"))
    );

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Click { target });
    reactor.pump();
    assert_eq!(hits.get(), 2);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert!(!reactor.engine().contains(target));
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Click { target });
    reactor.pump();
    assert_eq!(hits.get(), 2);
}

#[test]
fn hyperlink_button_reconciles_uri_uses_current_handler_and_ignores_stale_clicks() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let hits = Rc::new(Cell::new(0usize));
    let version_for_render = Rc::clone(&version);
    let hits_for_render = Rc::clone(&hits);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let version = state.get().unwrap();
        if version == 4 {
            text_block("removed")
        } else {
            let hits = Rc::clone(&hits_for_render);
            let button = HyperlinkButton::new(format!("Link {version}")).on_click(move || {
                hits.set(hits.get() + version + 1);
            });
            match version {
                0 => button.navigate_uri("https://example.com/first"),
                1 => button.navigate_uri("https://example.com/second"),
                2 => button,
                3 => button.navigate_uri("https://example.com/third"),
                _ => unreachable!(),
            }
            .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::HyperlinkButton);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| hyperlink_button_update(command)
                == Some((target, Some("https://example.com/first".to_string()))))
    );

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::HyperlinkButton), target);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| hyperlink_button_update(command)
                == Some((target, Some("https://example.com/second".to_string()))))
    );

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| hyperlink_button_update(command) == Some((target, None)))
    );

    assert!(version.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| hyperlink_button_update(command)
                == Some((target, Some("https://example.com/third".to_string()))))
    );
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Click { target });
    reactor.pump();
    assert_eq!(hits.get(), 4);

    assert!(version.borrow().as_ref().unwrap().try_set(4));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Click { target });
    reactor.pump();
    assert_eq!(hits.get(), 4);
}

#[test]
fn toggle_button_reconciles_checked_and_dispatches_current_handlers() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let hits = Rc::new(Cell::new(0usize));
    let version_for_render = Rc::clone(&version);
    let hits_for_render = Rc::clone(&hits);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let version = state.get().unwrap();
        if version == 2 {
            text_block("removed")
        } else {
            let toggle_hits = Rc::clone(&hits_for_render);
            let click_hits = Rc::clone(&hits_for_render);
            ToggleButton::new(format!("Toggle {version}"), version == 1, move |_| {
                toggle_hits.set(toggle_hits.get() + version + 1);
            })
            .on_click(move || click_hits.set(click_hits.get() + version + 1))
            .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::ToggleButton);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::ToggleButton), target);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| checked_update(command) == Some((target, true)))
    );
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Toggled {
            target,
            value: false,
        });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Click { target });
    reactor.pump();
    assert_eq!(hits.get(), 4);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Toggled {
            target,
            value: true,
        });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Click { target });
    reactor.pump();
    assert_eq!(hits.get(), 4);
}

#[test]
fn toggle_switch_reconciles_on_and_dispatches_current_handler() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let hits = Rc::new(Cell::new(0usize));
    let version_for_render = Rc::clone(&version);
    let hits_for_render = Rc::clone(&hits);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let version = state.get().unwrap();
        if version == 2 {
            text_block("removed")
        } else {
            let hits = Rc::clone(&hits_for_render);
            let toggle = ToggleSwitch::new(version == 1, move |_| {
                hits.set(hits.get() + version + 1);
            });
            if version == 0 {
                toggle
                    .header("Notifications")
                    .on_content("Yes")
                    .off_content("No")
                    .build()
            } else {
                toggle.build()
            }
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::ToggleSwitch);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::ToggleSwitch), target);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| toggle_switch_update(command) == Some((target, true)))
    );
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .find_map(toggle_switch_content_update),
        Some((
            target,
            ToggleSwitchContentUpdate {
                header: None,
                on_content: None,
                off_content: None,
            },
        ))
    );
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Toggled {
            target,
            value: false,
        });
    reactor.pump();
    assert_eq!(hits.get(), 2);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Toggled {
            target,
            value: true,
        });
    reactor.pump();
    assert_eq!(hits.get(), 2);
}

#[test]
fn info_badge_reconciles_numeric_and_dot_values_without_replacement() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let version_for_render = Rc::clone(&version);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        match state.get().unwrap() {
            0 => InfoBadge::dot().build(),
            1 => InfoBadge::numeric(42).build(),
            2 => InfoBadge::dot().build(),
            _ => InfoBadge::dot().build(),
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let badge = native_node(&reactor, NativeKind::InfoBadge);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::InfoBadge), badge);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::InfoBadgeValue(Some(42))),
                } if *id == badge
            ))
    );

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::InfoBadge), badge);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::InfoBadgeValue(None)),
                } if *id == badge
            ))
    );
}

#[test]
fn info_bar_reconciles_props_uses_current_close_handler_and_ignores_stale_requests() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let hits = Rc::new(RefCell::new(Vec::new()));
    let version_for_render = Rc::clone(&version);
    let hits_for_render = Rc::clone(&hits);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        if current == 2 {
            text_block("removed")
        } else {
            let hits = Rc::clone(&hits_for_render);
            InfoBar::new(if current == 0 { "Initial" } else { "Updated" })
                .message(if current == 0 { "First" } else { "Second" })
                .severity(if current == 0 {
                    InfoBarSeverity::Informational
                } else {
                    InfoBarSeverity::Warning
                })
                .open(current == 0)
                .closable(current == 0)
                .on_close_requested(move || hits.borrow_mut().push(current))
                .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let bar = native_node(&reactor, NativeKind::InfoBar);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::InfoBar), bar);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::InfoBar(update)),
                } if *id == bar
                    && update.title == "Updated"
                    && update.message == "Second"
                    && update.severity == InfoBarSeverity::Warning
                    && !update.open
                    && !update.closable
            ))
    );

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::InfoBarCloseRequested { target: bar });
    reactor.pump();
    assert_eq!(*hits.borrow(), [1]);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::InfoBarCloseRequested { target: bar });
    reactor.pump();
    assert_eq!(*hits.borrow(), [1]);
}

#[test]
fn person_picture_reconciles_name_initials_and_resets_without_replacement() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let version_for_render = Rc::clone(&version);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        match state.get().unwrap() {
            0 => PersonPicture::new().display_name("Ada Lovelace").build(),
            1 => PersonPicture::new().initials("WR").build(),
            _ => PersonPicture::new().build(),
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let picture = native_node(&reactor, NativeKind::PersonPicture);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::PersonPicture), picture);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::PersonPicture(update)),
                } if *id == picture
                    && update.display_name.is_none()
                    && update.initials.as_deref() == Some("WR")
            ))
    );

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::PersonPicture), picture);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::PersonPicture(update)),
                } if *id == picture
                    && update.display_name.is_none()
                    && update.initials.is_none()
            ))
    );
}

#[test]
fn progress_controls_reconcile_ranges_and_flags_without_replacement() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let version_for_render = Rc::clone(&version);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        match state.get().unwrap() {
            0 => StackPanel::new([
                ProgressBar::new(25.0).build(),
                ProgressRing::new(40.0).build(),
            ])
            .build(),
            1 => StackPanel::new([
                ProgressBar::new(225.0)
                    .range(200.0, 300.0)
                    .is_indeterminate(true)
                    .build(),
                ProgressRing::new(-25.0)
                    .range(-50.0, 0.0)
                    .active(false)
                    .is_indeterminate(true)
                    .build(),
            ])
            .build(),
            _ => text_block("removed"),
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let bar = native_node(&reactor, NativeKind::ProgressBar);
    let ring = native_node(&reactor, NativeKind::ProgressRing);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::ProgressBar), bar);
    assert_eq!(native_node(&reactor, NativeKind::ProgressRing), ring);
    let updates = reactor.engine().runtime().batches().last().unwrap();
    assert!(updates.iter().any(|command| {
        progress_bar_range(command)
            == Some((
                bar,
                RangeState {
                    value: 225.0,
                    minimum: 200.0,
                    maximum: 300.0,
                },
            ))
    }));
    assert!(
        updates
            .iter()
            .any(|command| { progress_bar_indeterminate(command) == Some((bar, true)) })
    );
    assert!(updates.iter().any(|command| {
        progress_ring_range(command)
            == Some((
                ring,
                RangeState {
                    value: -25.0,
                    minimum: -50.0,
                    maximum: 0.0,
                },
            ))
    }));
    assert!(
        updates
            .iter()
            .any(|command| { progress_ring_active(command) == Some((ring, false)) })
    );
    assert!(
        updates
            .iter()
            .any(|command| { progress_ring_indeterminate(command) == Some((ring, true)) })
    );

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert!(!reactor.engine().runtime().contains(bar));
    assert!(!reactor.engine().runtime().contains(ring));
}

#[test]
fn slider_reconciles_range_orientation_and_current_handler() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let observed = Rc::new(Cell::new(0.0));
    let version_for_render = Rc::clone(&version);
    let observed_for_render = Rc::clone(&observed);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let version = state.get().unwrap();
        if version == 2 {
            text_block("removed")
        } else {
            let observed = Rc::clone(&observed_for_render);
            let (value, minimum, maximum, orientation, step, header) = if version == 0 {
                (
                    25.0,
                    0.0,
                    100.0,
                    Orientation::Horizontal,
                    1.0,
                    Some("Volume"),
                )
            } else {
                (225.0, 200.0, 300.0, Orientation::Vertical, 2.0, None)
            };
            let slider = Slider::new(value, move |value| {
                observed.set(value + version as f64 * 1000.0);
            })
            .range(minimum, maximum)
            .step(step)
            .orientation(orientation);
            if let Some(header) = header {
                slider.header(header).build()
            } else {
                slider.build()
            }
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::Slider);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::Slider), target);
    let updates = reactor.engine().runtime().batches().last().unwrap();
    assert!(updates.iter().any(|command| {
        slider_range(command)
            == Some((
                target,
                RangeState {
                    value: 225.0,
                    minimum: 200.0,
                    maximum: 300.0,
                },
            ))
    }));
    assert!(
        updates.iter().any(|command| {
            slider_orientation(command) == Some((target, Orientation::Vertical))
        })
    );
    assert!(
        updates
            .iter()
            .any(|command| slider_step(command) == Some((target, 2.0)))
    );
    assert!(
        updates
            .iter()
            .any(|command| slider_header(command) == Some((target, None)))
    );

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ValueChanged {
            target,
            value: 250.0,
        });
    reactor.pump();
    assert_eq!(observed.get(), 1250.0);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ValueChanged {
            target,
            value: 275.0,
        });
    reactor.pump();
    assert_eq!(observed.get(), 1250.0);
}

#[test]
fn number_box_reconciles_optional_values_disjoint_ranges_and_current_handler() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let observed = Rc::new(RefCell::new(Vec::new()));
    let version_for_render = Rc::clone(&version);
    let observed_for_render = Rc::clone(&observed);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let version = state.get().unwrap();
        if version == 3 {
            text_block("replacement")
        } else {
            let observed = Rc::clone(&observed_for_render);
            let (value, minimum, maximum) = match version {
                0 => (Some(25.0), 0.0, 100.0),
                1 => (None, 200.0, 300.0),
                2 => (Some(225.0), 200.0, 300.0),
                _ => unreachable!(),
            };
            let number = NumberBox::new(value, move |value| {
                observed.borrow_mut().push((version, value));
            })
            .range(minimum, maximum);
            if version == 0 {
                number.header("Quantity").build()
            } else {
                number.build()
            }
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::NumberBox);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::NumberBox), target);
    let updates = reactor.engine().runtime().batches().last().unwrap();
    assert!(
        updates
            .iter()
            .any(|command| number_box_bounds(command) == Some((target, 200.0, 300.0)))
    );
    assert!(
        updates
            .iter()
            .any(|command| number_box_header(command) == Some((target, None)))
    );
    assert!(
        updates
            .iter()
            .any(|command| number_box_value(command) == Some((target, None)))
    );

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::OptionalValueChanged {
            target,
            value: Some(250.0),
        });
    reactor.pump();
    assert_eq!(&*observed.borrow(), &[(1, Some(250.0))]);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    let updates = reactor.engine().runtime().batches().last().unwrap();
    assert!(
        updates
            .iter()
            .any(|command| number_box_value(command) == Some((target, Some(225.0))))
    );

    assert!(version.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::OptionalValueChanged {
            target,
            value: None,
        });
    reactor.pump();
    assert_eq!(&*observed.borrow(), &[(1, Some(250.0))]);
}

#[test]
fn rating_control_reconciles_optional_values_configuration_and_current_handler() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let observed = Rc::new(RefCell::new(Vec::new()));
    let version_for_render = Rc::clone(&version);
    let observed_for_render = Rc::clone(&observed);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let version = state.get().unwrap();
        if version == 3 {
            text_block("replacement")
        } else {
            let observed = Rc::clone(&observed_for_render);
            let (value, max_rating, placeholder, caption, read_only) = match version {
                0 => (Some(3.0), 5, Some(4.0), "Initial", false),
                1 => (None, 10, Some(7.5), "Updated", true),
                2 => (Some(6.0), 10, None, "Updated", false),
                _ => unreachable!(),
            };
            RatingControl::new(value, move |value| {
                observed.borrow_mut().push((version, value));
            })
            .max_rating(max_rating)
            .placeholder(placeholder)
            .caption(caption)
            .read_only(read_only)
            .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::RatingControl);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::RatingControl), target);
    let updates = reactor.engine().runtime().batches().last().unwrap();
    assert!(
        updates
            .iter()
            .any(|command| rating_max(command) == Some((target, 10)))
    );
    assert!(
        updates
            .iter()
            .any(|command| { rating_placeholder(command) == Some((target, Some(7.5))) })
    );
    assert!(
        updates
            .iter()
            .any(|command| rating_caption(command) == Some((target, "Updated")))
    );
    assert!(
        updates
            .iter()
            .any(|command| rating_read_only(command) == Some((target, true)))
    );
    assert!(
        updates
            .iter()
            .any(|command| rating_value(command) == Some((target, None)))
    );

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::OptionalValueChanged {
            target,
            value: Some(8.0),
        });
    reactor.pump();
    assert_eq!(&*observed.borrow(), &[(1, Some(8.0))]);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    let updates = reactor.engine().runtime().batches().last().unwrap();
    assert!(
        updates
            .iter()
            .any(|command| rating_placeholder(command) == Some((target, None)))
    );
    assert!(
        updates
            .iter()
            .any(|command| rating_value(command) == Some((target, Some(6.0))))
    );

    assert!(version.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::OptionalValueChanged {
            target,
            value: None,
        });
    reactor.pump();
    assert_eq!(&*observed.borrow(), &[(1, Some(8.0))]);
}

#[test]
fn color_picker_reconciles_configuration_and_current_handler() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let observed = Rc::new(RefCell::new(Vec::new()));
    let version_for_render = Rc::clone(&version);
    let observed_for_render = Rc::clone(&observed);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let version = state.get().unwrap();
        if version == 2 {
            text_block("replacement")
        } else {
            let observed = Rc::clone(&observed_for_render);
            let (color, visible) = if version == 0 {
                (Color::rgb(10, 20, 30), true)
            } else {
                (Color::argb(128, 40, 50, 60), false)
            };
            ColorPicker::new(color, move |value| {
                observed.borrow_mut().push((version, value));
            })
            .alpha_enabled(visible)
            .hex_input_visible(visible)
            .color_slider_visible(visible)
            .color_channel_text_input_visible(visible)
            .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::ColorPicker);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::ColorPicker), target);
    let updates = reactor.engine().runtime().batches().last().unwrap();
    assert!(updates.iter().any(|command| {
        color_picker_color(command) == Some((target, Color::argb(128, 40, 50, 60)))
    }));
    assert!(
        updates
            .iter()
            .any(|command| color_picker_alpha(command) == Some((target, false)))
    );
    assert!(
        updates
            .iter()
            .any(|command| color_picker_hex(command) == Some((target, false)))
    );
    assert!(
        updates
            .iter()
            .any(|command| color_picker_slider(command) == Some((target, false)))
    );
    assert!(
        updates
            .iter()
            .any(|command| { color_picker_channel(command) == Some((target, false)) })
    );

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ColorChanged {
            target,
            value: Color::rgb(70, 80, 90),
        });
    reactor.pump();
    assert_eq!(&*observed.borrow(), &[(1, Color::rgb(70, 80, 90))]);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ColorChanged {
            target,
            value: Color::rgb(90, 80, 70),
        });
    reactor.pump();
    assert_eq!(&*observed.borrow(), &[(1, Color::rgb(70, 80, 90))]);
}

#[test]
fn date_picker_reconciles_configuration_and_current_handler() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let observed = Rc::new(RefCell::new(Vec::new()));
    let version_for_render = Rc::clone(&version);
    let observed_for_render = Rc::clone(&observed);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let version = state.get().unwrap();
        if version == 2 {
            text_block("replacement")
        } else {
            let observed = Rc::clone(&observed_for_render);
            let (date, visible) = if version == 0 {
                (Some(DateTime::UNIX_EPOCH), true)
            } else {
                (None, false)
            };
            let picker = DatePicker::new(date, move |value| {
                observed.borrow_mut().push((version, value));
            })
            .day_visible(visible)
            .month_visible(visible)
            .year_visible(visible);
            if version == 0 {
                picker.header("Date").build()
            } else {
                picker.build()
            }
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::DatePicker);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::DatePicker), target);
    let updates = reactor.engine().runtime().batches().last().unwrap();
    assert!(
        updates
            .iter()
            .any(|command| date_picker_date(command) == Some((target, None)))
    );
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::DatePicker(update)),
        } if *id == target && matches!(update, DatePickerUpdate::Header(None))
    )));
    assert!(
        updates
            .iter()
            .any(|command| date_picker_day(command) == Some((target, false)))
    );
    assert!(
        updates
            .iter()
            .any(|command| date_picker_month(command) == Some((target, false)))
    );
    assert!(
        updates
            .iter()
            .any(|command| date_picker_year(command) == Some((target, false)))
    );

    let changed = DateTime::from_unix_secs(86_400);
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::DateChanged {
            target,
            value: Some(changed),
        });
    reactor.pump();
    assert_eq!(&*observed.borrow(), &[(1, Some(changed))]);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::DateChanged {
            target,
            value: None,
        });
    reactor.pump();
    assert_eq!(&*observed.borrow(), &[(1, Some(changed))]);
}

#[test]
fn calendar_date_picker_reconciles_and_dispatches_current_handler() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let observed = Rc::new(RefCell::new(Vec::new()));
    let version_for_render = Rc::clone(&version);
    let observed_for_render = Rc::clone(&observed);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        if current == 2 {
            text_block("removed")
        } else {
            let observed = Rc::clone(&observed_for_render);
            let picker = CalendarDatePicker::new(
                (current == 0).then_some(DateTime::UNIX_EPOCH),
                move |value| observed.borrow_mut().push((current, value)),
            )
            .today_highlighted(current == 0);
            if current == 0 {
                picker.header("Date").placeholder_text("Choose").build()
            } else {
                picker.build()
            }
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::CalendarDatePicker);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    let updates = reactor.engine().runtime().batches().last().unwrap();
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::CalendarDatePicker(update)),
        } if *id == target
            && matches!(update.as_ref(), CalendarDatePickerUpdate::Date(None))
    )));
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::CalendarDatePicker(update)),
        } if *id == target
            && matches!(update.as_ref(), CalendarDatePickerUpdate::Header(None))
    )));
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::CalendarDatePicker(update)),
        } if *id == target
            && matches!(update.as_ref(), CalendarDatePickerUpdate::Placeholder(None))
    )));
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::CalendarDatePicker(update)),
        } if *id == target
            && matches!(
                update.as_ref(),
                CalendarDatePickerUpdate::TodayHighlighted(false)
            )
    )));

    let changed = DateTime::from_unix_secs(86_400);
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::DateChanged {
            target,
            value: Some(changed),
        });
    reactor.pump();
    assert_eq!(&*observed.borrow(), &[(1, Some(changed))]);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::DateChanged {
            target,
            value: None,
        });
    reactor.pump();
    assert_eq!(&*observed.borrow(), &[(1, Some(changed))]);
}

#[test]
fn time_picker_reconciles_configuration_and_current_handler() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let observed = Rc::new(RefCell::new(Vec::new()));
    let version_for_render = Rc::clone(&version);
    let observed_for_render = Rc::clone(&observed);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let version = state.get().unwrap();
        if version == 2 {
            text_block("replacement")
        } else {
            let observed = Rc::clone(&observed_for_render);
            let picker = TimePicker::new(
                (version == 0).then_some(TimeSpan::from_hours(9) + TimeSpan::from_minutes(30)),
                move |value| observed.borrow_mut().push((version, value)),
            )
            .minute_increment(if version == 0 { 15 } else { 30 });
            if version == 0 {
                picker.header("Pick a time").build()
            } else {
                picker.build()
            }
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TimePicker);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::TimePicker), target);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::TimePicker(update)),
                } if *id == target
                    && **update == TimePickerUpdate {
                        time: None,
                        header: None,
                        minute_increment: 30,
                    }
            ))
    );

    let changed = TimeSpan::from_hours(13) + TimeSpan::from_minutes(45);
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TimeChanged {
            target,
            value: Some(changed),
        });
    reactor.pump();
    assert_eq!(&*observed.borrow(), &[(1, Some(changed))]);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TimeChanged {
            target,
            value: None,
        });
    reactor.pump();
    assert_eq!(&*observed.borrow(), &[(1, Some(changed))]);
}

#[test]
fn calendar_view_reconciles_selection_and_current_handler() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let observed = Rc::new(RefCell::new(Vec::new()));
    let version_for_render = Rc::clone(&version);
    let observed_for_render = Rc::clone(&observed);
    let first = DateTime::from_unix_secs(1_704_067_200);
    let second = DateTime::from_unix_secs(1_704_153_600);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let version = state.get().unwrap();
        if version == 2 {
            text_block("replacement")
        } else {
            let observed = Rc::clone(&observed_for_render);
            CalendarView::new(
                if version == 0 {
                    vec![second, first, first]
                } else {
                    vec![second]
                },
                move |value| observed.borrow_mut().push((version, value)),
            )
            .selection_mode(if version == 0 {
                CalendarSelectionMode::Multiple
            } else {
                CalendarSelectionMode::Single
            })
            .today_highlighted(version != 0)
            .group_label_visible(version == 0)
            .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::CalendarView);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::CalendarView(update)),
                } if *id == target && update.selected_dates.as_ref() == [first, second]
            ))
    );

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::CalendarView), target);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::CalendarView(update)),
                } if *id == target
                    && **update == CalendarViewUpdate {
                        selected_dates: vec![second].into(),
                        selection_mode: CalendarSelectionMode::Single,
                        today_highlighted: true,
                        group_label_visible: false,
                    }
            ))
    );

    let changed = DateTime::from_unix_secs(1_704_240_000);
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::DatesChanged {
            target,
            value: vec![changed],
        });
    reactor.pump();
    assert_eq!(&*observed.borrow(), &[(1, vec![changed])]);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::DatesChanged {
            target,
            value: Vec::new(),
        });
    reactor.pump();
    assert_eq!(&*observed.borrow(), &[(1, vec![changed])]);
}

#[test]
fn text_inputs_reconcile_headers_placeholders_and_modes() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let version_for_render = Rc::clone(&version);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        if state.get().unwrap() == 0 {
            stack_panel([
                TextBox::new("text", |_| {})
                    .header("Text header")
                    .placeholder_text("Text placeholder")
                    .multiline()
                    .build(),
                PasswordBox::new("secret", |_| {})
                    .header("Password header")
                    .placeholder_text("Password placeholder")
                    .password_reveal_mode(PasswordRevealMode::Hidden)
                    .build(),
            ])
        } else {
            stack_panel([
                TextBox::new("text", |_| {}).build(),
                PasswordBox::new("secret", |_| {}).build(),
            ])
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let text = native_node(&reactor, NativeKind::TextBox);
    let password = native_node(&reactor, NativeKind::PasswordBox);

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    let updates = reactor.engine().runtime().batches().last().unwrap();
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::TextBox(update)),
        } if *id == text && matches!(update.as_ref(), TextBoxUpdate::Header(None))
    )));
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::TextBox(update)),
        } if *id == text && matches!(update.as_ref(), TextBoxUpdate::Placeholder(None))
    )));
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::TextBox(update)),
        } if *id == text && matches!(update.as_ref(), TextBoxUpdate::AcceptsReturn(false))
    )));
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::PasswordBox(update)),
        } if *id == password && matches!(update.as_ref(), PasswordBoxUpdate::Header(None))
    )));
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::PasswordBox(update)),
        } if *id == password
            && matches!(update.as_ref(), PasswordBoxUpdate::Placeholder(None))
    )));
    assert!(updates.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::PasswordBox(update)),
        } if *id == password
            && matches!(
                update.as_ref(),
                PasswordBoxUpdate::RevealMode(PasswordRevealMode::Peek)
            )
    )));
}

#[test]
fn password_box_reconciles_and_dispatches_current_handler() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let observed_length = Rc::new(Cell::new(0usize));
    let version_for_render = Rc::clone(&version);
    let length_for_render = Rc::clone(&observed_length);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let version = state.get().unwrap();
        if version == 2 {
            text_block("removed")
        } else {
            let observed_length = Rc::clone(&length_for_render);
            PasswordBox::new(format!("secret-{version}"), move |password| {
                observed_length.set(password.len() + version);
            })
            .build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::PasswordBox);
    let initial = reactor.engine().runtime().batches().last().unwrap();
    assert!(
        initial
            .iter()
            .any(|command| password_update_matches(command, target, "secret-0"))
    );

    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::PasswordBox), target);
    let updated = reactor.engine().runtime().batches().last().unwrap();
    assert!(
        updated
            .iter()
            .any(|command| password_update_matches(command, target, "secret-1"))
    );

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::PasswordChanged {
            target,
            value: "entered-value".to_string(),
        });
    reactor.pump();
    assert_eq!(observed_length.get(), "entered-value".len() + 1);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::PasswordChanged {
            target,
            value: "stale-value".to_string(),
        });
    reactor.pump();
    assert_eq!(observed_length.get(), "entered-value".len() + 1);
}

#[test]
fn keyboard_accelerators_attach_replace_detach_and_ignore_stale_events() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let hits = Rc::new(Cell::new(0usize));
    let version_for_render = Rc::clone(&version);
    let hits_for_render = Rc::clone(&hits);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let version = state.get().unwrap();
        let (modifiers, amount) = if version == 2 {
            (
                VirtualKeyModifiers::CONTROL | VirtualKeyModifiers::SHIFT,
                100,
            )
        } else {
            (
                VirtualKeyModifiers::CONTROL,
                if version == 1 {
                    10
                } else if version == 4 {
                    1_000
                } else {
                    1
                },
            )
        };
        let hits = Rc::clone(&hits_for_render);
        let accelerator = KeyboardAccelerator::new(VirtualKey::S, modifiers, move || {
            hits.set(hits.get() + amount);
        });
        let child = if version == 3 {
            TextBlock::new("target").build()
        } else if version == 4 {
            Button::new("target")
                .on_click(|| {})
                .keyboard_accelerator(accelerator)
                .build()
        } else {
            TextBlock::new("target")
                .keyboard_accelerator(accelerator)
                .build()
        };
        StackPanel::new([child.key(23)]).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TextBlock);
    let parent = native_node(&reactor, NativeKind::StackPanel);
    let control_s = KeyboardAcceleratorSpec {
        key: VirtualKey::S,
        modifiers: VirtualKeyModifiers::CONTROL,
    };
    let first = reactor.engine().runtime().batches().first().unwrap();
    assert_eq!(
        first
            .iter()
            .filter_map(keyboard_accelerator_update)
            .collect::<Vec<_>>(),
        [(target, vec![control_s])]
    );
    let update_position = first
        .iter()
        .position(|command| keyboard_accelerator_update(command).is_some())
        .unwrap();
    let attach_position = first
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::Attach {
                    parent: command_parent,
                    child,
                    ..
                } if *command_parent == parent && *child == target
            )
        })
        .unwrap();
    assert!(update_position < attach_position);

    let update_count = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(keyboard_accelerator_update)
        .count();
    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::TextBlock), target);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(keyboard_accelerator_update)
            .count(),
        update_count
    );
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::KeyboardAcceleratorInvoked {
            target,
            accelerator: control_s,
        });
    reactor.pump();
    assert_eq!(hits.get(), 10);

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    let control_shift_s = KeyboardAcceleratorSpec {
        key: VirtualKey::S,
        modifiers: VirtualKeyModifiers::CONTROL | VirtualKeyModifiers::SHIFT,
    };
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(keyboard_accelerator_update)
            .collect::<Vec<_>>(),
        [(target, vec![control_shift_s])]
    );
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::KeyboardAcceleratorInvoked {
            target,
            accelerator: control_s,
        });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::KeyboardAcceleratorInvoked {
            target,
            accelerator: control_shift_s,
        });
    reactor.pump();
    assert_eq!(hits.get(), 110);

    assert!(version.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::TextBlock), target);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(keyboard_accelerator_update)
            .collect::<Vec<_>>(),
        [(target, Vec::new())]
    );
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::KeyboardAcceleratorInvoked {
            target,
            accelerator: control_shift_s,
        });
    reactor.pump();
    assert_eq!(hits.get(), 110);

    assert!(version.borrow().as_ref().unwrap().try_set(4));
    reactor.pump();
    let replacement = native_node(&reactor, NativeKind::Button);
    assert_ne!(replacement, target);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(keyboard_accelerator_update)
            .collect::<Vec<_>>(),
        [(replacement, vec![control_s])]
    );
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::KeyboardAcceleratorInvoked {
            target,
            accelerator: control_s,
        });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::KeyboardAcceleratorInvoked {
            target: replacement,
            accelerator: control_s,
        });
    reactor.pump();
    assert_eq!(hits.get(), 1_110);
}

#[test]
fn pointer_input_replaces_handlers_updates_subscriptions_and_ignores_stale_events() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let log = Rc::new(RefCell::new(Vec::<(&'static str, PointerEvent)>::new()));
    let gesture_log = Rc::new(RefCell::new(Vec::<&'static str>::new()));
    let version_for_render = Rc::clone(&version);
    let log_for_render = Rc::clone(&log);
    let gesture_log_for_render = Rc::clone(&gesture_log);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let version = state.get().unwrap();
        let label = if version == 1 { "latest" } else { "first" };
        let tapped_label = if version == 1 {
            "latest-tapped"
        } else {
            "first-tapped"
        };
        let right_tapped_label = if version == 1 {
            "latest-right-tapped"
        } else {
            "first-right-tapped"
        };
        let child = match version {
            0 | 1 => {
                let tapped_log = Rc::clone(&gesture_log_for_render);
                let right_tapped_log = Rc::clone(&gesture_log_for_render);
                TextBlock::new("target")
                    .on_pointer_pressed(pointer_logger(Rc::clone(&log_for_render), label))
                    .on_pointer_moved(pointer_logger(Rc::clone(&log_for_render), "moved"))
                    .on_pointer_released(pointer_logger(Rc::clone(&log_for_render), "released"))
                    .on_pointer_capture_lost(pointer_logger(
                        Rc::clone(&log_for_render),
                        "capture-lost",
                    ))
                    .on_pointer_canceled(pointer_logger(Rc::clone(&log_for_render), "canceled"))
                    .on_pointer_entered(pointer_logger(Rc::clone(&log_for_render), "entered"))
                    .on_pointer_exited(pointer_logger(Rc::clone(&log_for_render), "exited"))
                    .on_tapped(move || tapped_log.borrow_mut().push(tapped_label))
                    .on_right_tapped(move || {
                        right_tapped_log.borrow_mut().push(right_tapped_label);
                    })
                    .capture_pointer_on_press()
                    .build()
            }
            2 => TextBlock::new("target")
                .on_pointer_pressed(pointer_logger(Rc::clone(&log_for_render), "pressed-only"))
                .build(),
            3 => TextBlock::new("target").build(),
            _ => Button::new("target")
                .on_click(|| {})
                .on_pointer_pressed(pointer_logger(Rc::clone(&log_for_render), "replacement"))
                .build(),
        };
        StackPanel::new([child.key(23)]).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TextBlock);
    let parent = native_node(&reactor, NativeKind::StackPanel);
    let all_events = PointerEvents::PRESSED
        | PointerEvents::MOVED
        | PointerEvents::RELEASED
        | PointerEvents::CAPTURE_LOST
        | PointerEvents::CANCELED
        | PointerEvents::ENTERED
        | PointerEvents::EXITED
        | PointerEvents::TAPPED
        | PointerEvents::RIGHT_TAPPED;
    let first_subscription = PointerSubscription {
        events: all_events,
        capture_on_press: true,
    };
    let first = reactor.engine().runtime().batches().first().unwrap();
    assert_eq!(
        first
            .iter()
            .filter_map(pointer_subscription_update)
            .collect::<Vec<_>>(),
        [(target, first_subscription)]
    );
    let update_position = first
        .iter()
        .position(|command| pointer_subscription_update(command).is_some())
        .unwrap();
    let attach_position = first
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::Attach {
                    parent: command_parent,
                    child,
                    ..
                } if *command_parent == parent && *child == target
            )
        })
        .unwrap();
    assert!(update_position < attach_position);

    let update_count = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(pointer_subscription_update)
        .count();
    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(pointer_subscription_update)
            .count(),
        update_count
    );

    let event = PointerEvent {
        pointer_id: 7,
        x: 1.5,
        y: 2.5,
        window_x: 3.5,
        window_y: 4.5,
        capture_succeeded: true,
        is_left_button_pressed: true,
        is_right_button_pressed: false,
        is_middle_button_pressed: true,
    };
    for kind in [
        PointerEventKind::Pressed,
        PointerEventKind::Moved,
        PointerEventKind::Released,
        PointerEventKind::CaptureLost,
        PointerEventKind::Canceled,
        PointerEventKind::Entered,
        PointerEventKind::Exited,
    ] {
        reactor
            .engine()
            .runtime()
            .queue_event(NativeEvent::Pointer {
                target,
                kind,
                event,
            });
    }
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Tapped { target });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::RightTapped { target });
    reactor.pump();
    assert_eq!(
        log.borrow()
            .iter()
            .map(|(kind, event)| (*kind, *event))
            .collect::<Vec<_>>(),
        [
            ("latest", event),
            ("moved", event),
            ("released", event),
            ("capture-lost", event),
            ("canceled", event),
            ("entered", event),
            ("exited", event),
        ]
    );
    assert_eq!(
        gesture_log.borrow().as_slice(),
        ["latest-tapped", "latest-right-tapped"]
    );

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(pointer_subscription_update)
            .collect::<Vec<_>>(),
        [(
            target,
            PointerSubscription {
                events: PointerEvents::PRESSED,
                capture_on_press: false,
            }
        )]
    );
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Pointer {
            target,
            kind: PointerEventKind::Moved,
            event,
        });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Pointer {
            target,
            kind: PointerEventKind::Pressed,
            event,
        });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Tapped { target });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::RightTapped { target });
    reactor.pump();
    assert_eq!(log.borrow().last(), Some(&("pressed-only", event)));
    assert_eq!(gesture_log.borrow().len(), 2);

    assert!(version.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(pointer_subscription_update)
            .collect::<Vec<_>>(),
        [(target, PointerSubscription::default())]
    );
    let event_count = log.borrow().len();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Pointer {
            target,
            kind: PointerEventKind::Pressed,
            event,
        });
    reactor.pump();
    assert_eq!(log.borrow().len(), event_count);

    assert!(version.borrow().as_ref().unwrap().try_set(4));
    reactor.pump();
    let replacement = native_node(&reactor, NativeKind::Button);
    assert_ne!(replacement, target);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(pointer_subscription_update)
            .collect::<Vec<_>>(),
        [(
            replacement,
            PointerSubscription {
                events: PointerEvents::PRESSED,
                capture_on_press: false,
            }
        )]
    );
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Pointer {
            target,
            kind: PointerEventKind::Pressed,
            event,
        });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Pointer {
            target: replacement,
            kind: PointerEventKind::Pressed,
            event,
        });
    reactor.pump();
    assert_eq!(log.borrow().last(), Some(&("replacement", event)));
    assert_eq!(log.borrow().len(), event_count + 1);
}

#[test]
fn drop_target_replaces_handlers_updates_configuration_and_ignores_stale_events() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let log = Rc::new(RefCell::new(Vec::<(&'static str, DropEvent)>::new()));
    let version_for_render = Rc::clone(&version);
    let log_for_render = Rc::clone(&log);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let version = state.get().unwrap();
        let text_target = DropTarget::new(DropOperation::Copy, DropFormats::TEXT);
        let storage_target = DropTarget::new(DropOperation::Move, DropFormats::STORAGE_ITEMS);
        let child = match version {
            0 => TextBlock::new("target")
                .on_drop(
                    text_target,
                    drop_logger(Rc::clone(&log_for_render), "first"),
                )
                .build(),
            1 => TextBlock::new("target")
                .on_drop(
                    text_target,
                    drop_logger(Rc::clone(&log_for_render), "latest"),
                )
                .build(),
            2 => TextBlock::new("target")
                .on_drop(
                    storage_target,
                    drop_logger(Rc::clone(&log_for_render), "storage"),
                )
                .build(),
            3 => TextBlock::new("target").build(),
            _ => Button::new("target")
                .on_click(|| {})
                .on_drop(
                    text_target,
                    drop_logger(Rc::clone(&log_for_render), "replacement"),
                )
                .build(),
        };
        StackPanel::new([child.key(31)]).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TextBlock);
    let parent = native_node(&reactor, NativeKind::StackPanel);
    let text_target = DropTarget::new(DropOperation::Copy, DropFormats::TEXT);
    let first = reactor.engine().runtime().batches().first().unwrap();
    assert_eq!(
        first
            .iter()
            .filter_map(drop_target_update)
            .collect::<Vec<_>>(),
        [(target, Some(text_target))]
    );
    let update_position = first
        .iter()
        .position(|command| drop_target_update(command).is_some())
        .unwrap();
    let attach_position = first
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::Attach {
                    parent: command_parent,
                    child,
                    ..
                } if *command_parent == parent && *child == target
            )
        })
        .unwrap();
    assert!(update_position < attach_position);

    let update_count = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(drop_target_update)
        .count();
    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter_map(drop_target_update)
            .count(),
        update_count
    );

    let text_event = DropEvent {
        formats: DropFormats::TEXT,
        text: Some("dropped".to_string()),
        storage_items: Box::default(),
    };
    reactor.engine().runtime().queue_event(NativeEvent::Drop {
        target,
        result: Box::new(Ok(text_event.clone())),
    });
    reactor.pump();
    assert_eq!(log.borrow().last(), Some(&("latest", text_event.clone())));

    assert!(version.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    let storage_target = DropTarget::new(DropOperation::Move, DropFormats::STORAGE_ITEMS);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(drop_target_update)
            .collect::<Vec<_>>(),
        [(target, Some(storage_target))]
    );
    let storage_event = DropEvent {
        formats: DropFormats::STORAGE_ITEMS,
        text: None,
        storage_items: vec![DroppedItem {
            path: "C:\\drop.txt".to_string(),
            name: "drop.txt".to_string(),
            is_folder: false,
        }]
        .into_boxed_slice(),
    };
    reactor.engine().runtime().queue_event(NativeEvent::Drop {
        target,
        result: Box::new(Ok(storage_event.clone())),
    });
    reactor.pump();
    assert_eq!(log.borrow().last(), Some(&("storage", storage_event)));

    assert!(version.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(drop_target_update)
            .collect::<Vec<_>>(),
        [(target, None)]
    );
    let event_count = log.borrow().len();
    reactor.engine().runtime().queue_event(NativeEvent::Drop {
        target,
        result: Box::new(Ok(text_event.clone())),
    });
    reactor.pump();
    assert_eq!(log.borrow().len(), event_count);

    assert!(version.borrow().as_ref().unwrap().try_set(4));
    reactor.pump();
    let replacement = native_node(&reactor, NativeKind::Button);
    assert_ne!(replacement, target);
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .filter_map(drop_target_update)
            .collect::<Vec<_>>(),
        [(replacement, Some(text_target))]
    );
    reactor.engine().runtime().queue_event(NativeEvent::Drop {
        target,
        result: Box::new(Ok(text_event.clone())),
    });
    reactor.engine().runtime().queue_event(NativeEvent::Drop {
        target: replacement,
        result: Box::new(Ok(text_event.clone())),
    });
    reactor.pump();
    assert_eq!(log.borrow().last(), Some(&("replacement", text_event)));
    assert_eq!(log.borrow().len(), event_count + 1);
}

#[test]
fn button_enabled_and_handler_are_updated_during_reconciliation() {
    let enabled = Rc::new(RefCell::new(None::<State<bool>>));
    let increment = Rc::new(RefCell::new(None::<State<usize>>));
    let enabled_for_render = Rc::clone(&enabled);
    let increment_for_render = Rc::clone(&increment);
    let root = component(move |cx| {
        let enabled = cx.use_state(|| true);
        let increment = cx.use_state(|| 1usize);
        *enabled_for_render.borrow_mut() = Some(enabled.clone());
        *increment_for_render.borrow_mut() = Some(increment.clone());
        let count = cx.use_state(|| 0usize);
        let update = count.clone();
        let amount = increment.get().unwrap();
        button_enabled(
            format!("Count: {}", count.get().unwrap()),
            enabled.get().unwrap(),
            move || {
                assert!(update.try_set(update.get().unwrap() + amount));
            },
        )
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = button_node(&reactor);

    assert!(enabled.borrow().as_ref().unwrap().try_set(false));
    assert!(increment.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .any(|command| enabled_update(command) == Some((target, false)))
    );

    assert!(enabled.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Click { target });
    reactor.pump();
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "Count: 2"))
    );
}

#[test]
fn click_for_a_removed_button_is_ignored() {
    let visible = Rc::new(RefCell::new(None::<State<bool>>));
    let clicks = Rc::new(RefCell::new(0usize));
    let visible_for_render = Rc::clone(&visible);
    let clicks_for_render = Rc::clone(&clicks);
    let root = component(move |cx| {
        let visible = cx.use_state(|| true);
        *visible_for_render.borrow_mut() = Some(visible.clone());
        if visible.get().unwrap() {
            let clicks = Rc::clone(&clicks_for_render);
            button("Remove me", move || *clicks.borrow_mut() += 1)
        } else {
            text_block("Removed")
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let target = button_node(&reactor);

    assert!(visible.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Click { target });
    reactor.pump();

    assert_eq!(*clicks.borrow(), 0);
}

#[test]
#[should_panic(expected = "IncompatibleEvent")]
fn incompatible_event_for_a_live_node_panics() {
    let mut reactor = Reactor::new(RecordingRuntime::default(), text_block("not a button"));
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TextBlock);

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Click { target });
    reactor.pump();
}

#[test]
fn compatible_event_without_an_optional_handler_is_ignored() {
    for (kind, root) in [
        (NativeKind::Button, Button::new("button").build()),
        (
            NativeKind::HyperlinkButton,
            HyperlinkButton::new("hyperlink").build(),
        ),
        (
            NativeKind::RepeatButton,
            RepeatButton::new("repeat").build(),
        ),
        (
            NativeKind::ToggleButton,
            ToggleButton::new("toggle", false, |_| {}).build(),
        ),
    ] {
        let mut reactor = Reactor::new(RecordingRuntime::default(), root);
        reactor.pump();
        let target = native_node(&reactor, kind);

        reactor
            .engine()
            .runtime()
            .queue_event(NativeEvent::Click { target });
        reactor.pump();

        assert!(reactor.engine().is_valid());
    }
}

#[test]
#[should_panic(expected = "IncompatibleEvent")]
fn incompatible_coalesced_realization_event_panics() {
    let mut reactor = Reactor::new(RecordingRuntime::default(), text_block("not a collection"));
    reactor.pump();
    let target = native_node(&reactor, NativeKind::TextBlock);

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Realize {
            host: target,
            index: 0,
            lease: 1,
        });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Recycle {
            host: target,
            index: 0,
            lease: 1,
        });
    reactor.pump();
}

#[test]
fn input_and_toggle_events_update_controlled_state() {
    let root = component(|cx| {
        let text = cx.use_state(|| "before".to_string());
        let current_text = text.get().unwrap();
        let update_text = text;
        let checked = cx.use_state(|| false);
        let current_checked = checked.get().unwrap();
        let update_checked = checked;
        stack_panel([
            text_block(format!("Text: {current_text}")),
            text_box(current_text, move |value| {
                assert!(update_text.try_set(value));
            }),
            text_block(format!("Checked: {current_checked}")),
            check_box("Check", current_checked, move |value| {
                assert!(update_checked.try_set(value));
            }),
        ])
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let text_box = native_node(&reactor, NativeKind::TextBox);
    let check_box = native_node(&reactor, NativeKind::CheckBox);
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TextChanged {
            target: text_box,
            value: "after".to_string(),
        });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Toggled {
            target: check_box,
            value: true,
        });

    reactor.pump();

    let mut commands = reactor.engine().runtime().batches().iter().flatten();
    assert!(
        commands
            .clone()
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "Text: after"))
    );
    assert!(
        commands
            .clone()
            .all(|command| text_update(command) != Some((text_box, "after")))
    );
    assert!(
        commands
            .clone()
            .all(|command| checked_update(command) != Some((check_box, true)))
    );
    assert!(
        commands
            .any(|command| text_update(command).is_some_and(|(_, text)| text == "Checked: true"))
    );
}

#[test]
fn rejected_text_toggle_and_value_feedback_is_restored_on_rerender() {
    let revision = Rc::new(RefCell::new(None::<State<usize>>));
    let revision_for_render = Rc::clone(&revision);
    let events = Rc::new(Cell::new(0usize));
    let events_for_render = Rc::clone(&events);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *revision_for_render.borrow_mut() = Some(state.clone());
        _ = state.value();
        macro_rules! callback {
            () => {{
                let events = Rc::clone(&events_for_render);
                move |_| events.set(events.get() + 1)
            }};
        }
        stack_panel([
            TextBox::new("declared text", callback!()).build(),
            PasswordBox::new("declared password", callback!()).build(),
            RichEditBox::new("declared rich text", callback!()).build(),
            AutoSuggestBox::new("declared query", callback!()).build(),
            CheckBox::new("check", false, callback!()).build(),
            Slider::new(10.0, callback!()).build(),
        ])
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let text = native_node(&reactor, NativeKind::TextBox);
    let password = native_node(&reactor, NativeKind::PasswordBox);
    let rich = native_node(&reactor, NativeKind::RichEditBox);
    let suggest = native_node(&reactor, NativeKind::AutoSuggestBox);
    let toggle = native_node(&reactor, NativeKind::CheckBox);
    let slider = native_node(&reactor, NativeKind::Slider);

    for event in [
        NativeEvent::TextChanged {
            target: text,
            value: "native text".into(),
        },
        NativeEvent::PasswordChanged {
            target: password,
            value: "native password".into(),
        },
        NativeEvent::TextChanged {
            target: rich,
            value: "native rich text".into(),
        },
        NativeEvent::TextChanged {
            target: suggest,
            value: "native query".into(),
        },
        NativeEvent::Toggled {
            target: toggle,
            value: true,
        },
        NativeEvent::ValueChanged {
            target: slider,
            value: 90.0,
        },
    ] {
        reactor.engine().runtime().queue_event(event);
    }
    reactor.pump();
    assert_eq!(events.get(), 6);

    let batch_count = reactor.engine().runtime().batches().len();
    assert!(revision.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    let commands = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .skip(batch_count)
        .flatten()
        .collect::<Vec<_>>();
    assert!(commands.iter().any(|command| {
        matches!(
            command,
            Command::Update {
                id,
                update: NativeUpdate::Control(ControlUpdate::TextBox(update)),
            } if *id == text
                && matches!(update.as_ref(), TextBoxUpdate::Text(value) if value == "declared text")
        )
    }));
    assert!(commands.iter().any(|command| {
        matches!(
            command,
            Command::Update {
                id,
                update: NativeUpdate::Control(ControlUpdate::PasswordBox(update)),
            } if *id == password
                && matches!(update.as_ref(), PasswordBoxUpdate::Password(value)
                    if value == "declared password")
        )
    }));
    assert!(commands.iter().any(|command| {
        matches!(
            command,
            Command::Update {
                id,
                update: NativeUpdate::Control(ControlUpdate::RichEditBox(update)),
            } if *id == rich && update.text == "declared rich text"
        )
    }));
    assert!(commands.iter().any(|command| {
        matches!(
            command,
            Command::Update {
                id,
                update: NativeUpdate::Control(ControlUpdate::AutoSuggestBox(update)),
            } if *id == suggest
                && matches!(update.as_ref(), AutoSuggestUpdate::Text(value)
                    if value == "declared query")
        )
    }));
    assert!(commands.iter().any(|command| {
        matches!(
            command,
            Command::Update {
                id,
                update: NativeUpdate::Control(ControlUpdate::ToggleChecked(false)),
            } if *id == toggle
        )
    }));
    assert!(commands.iter().any(|command| {
        matches!(
            command,
            Command::Update {
                id,
                update: NativeUpdate::Control(ControlUpdate::Slider(update)),
            } if *id == slider
                && matches!(update.as_ref(), SliderUpdate::Range(range) if range.value == 10.0)
        )
    }));
}

#[test]
fn input_and_toggle_events_use_reconciled_handlers() {
    let generation = Rc::new(RefCell::new(None::<State<usize>>));
    let generation_for_render = Rc::clone(&generation);
    let seen = Rc::new(RefCell::new(Vec::new()));
    let seen_for_render = Rc::clone(&seen);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0);
        *generation_for_render.borrow_mut() = Some(state.clone());
        let current = state.get().unwrap();
        let text_seen = Rc::clone(&seen_for_render);
        let toggle_seen = Rc::clone(&seen_for_render);
        stack_panel([
            text_box("value", move |_| text_seen.borrow_mut().push(current)),
            check_box("Check", false, move |_| {
                toggle_seen.borrow_mut().push(current);
            }),
        ])
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let text_box = native_node(&reactor, NativeKind::TextBox);
    let check_box = native_node(&reactor, NativeKind::CheckBox);
    assert!(generation.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TextChanged {
            target: text_box,
            value: "after".to_string(),
        });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Toggled {
            target: check_box,
            value: true,
        });
    reactor.pump();

    assert_eq!(*seen.borrow(), vec![1, 1]);
}

#[test]
fn display_only_control_disables_input_and_removes_the_current_handler() {
    let display = Rc::new(RefCell::new(None::<State<bool>>));
    let display_for_render = Rc::clone(&display);
    let events = Rc::new(Cell::new(0usize));
    let events_for_render = Rc::clone(&events);
    let root = component(move |cx| {
        let state = cx.use_state(|| false);
        *display_for_render.borrow_mut() = Some(state.clone());
        if state.get().unwrap() {
            CheckBox::display("Check", false).build()
        } else {
            let events = Rc::clone(&events_for_render);
            CheckBox::new("Check", false, move |_| events.set(events.get() + 1)).build()
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let check_box = native_node(&reactor, NativeKind::CheckBox);

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Toggled {
            target: check_box,
            value: true,
        });
    reactor.pump();
    assert_eq!(events.get(), 1);

    assert!(display.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| enabled_update(command) == Some((check_box, false)))
    );

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Toggled {
            target: check_box,
            value: true,
        });
    reactor.pump();
    assert_eq!(events.get(), 1);
}

#[test]
fn input_and_toggle_events_for_removed_controls_are_ignored() {
    let visible = Rc::new(RefCell::new(None::<State<bool>>));
    let visible_for_render = Rc::clone(&visible);
    let events = Rc::new(RefCell::new(0));
    let events_for_render = Rc::clone(&events);
    let root = component(move |cx| {
        let state = cx.use_state(|| true);
        *visible_for_render.borrow_mut() = Some(state.clone());
        if state.get().unwrap() {
            let text_events = Rc::clone(&events_for_render);
            let toggle_events = Rc::clone(&events_for_render);
            stack_panel([
                text_box("value", move |_| *text_events.borrow_mut() += 1),
                check_box("Check", false, move |_| *toggle_events.borrow_mut() += 1),
            ])
        } else {
            text_block("removed")
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let text_box = native_node(&reactor, NativeKind::TextBox);
    let check_box = native_node(&reactor, NativeKind::CheckBox);
    assert!(visible.borrow().as_ref().unwrap().try_set(false));
    reactor.pump();

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TextChanged {
            target: text_box,
            value: "stale".to_string(),
        });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Toggled {
            target: check_box,
            value: true,
        });
    reactor.pump();

    assert_eq!(*events.borrow(), 0);
}
