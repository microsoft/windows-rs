use super::*;
use crate::element::tree::StructuralSlot;
use crate::mounted::MountedKind;

#[test]
fn viewbox_reuses_single_content_ownership_and_diffs_stretch() {
    let stretch = Rc::new(RefCell::new(None::<State<Stretch>>));
    let stretch_for_render = Rc::clone(&stretch);
    let root = component(move |cx| {
        let value = cx.use_state(|| Stretch::Uniform);
        *stretch_for_render.borrow_mut() = Some(value.clone());
        Viewbox::new(text_block("content"))
            .stretch(value.get().unwrap())
            .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let runtime = reactor.engine().runtime();
    let viewbox = runtime
        .batches()
        .iter()
        .flatten()
        .find_map(|command| match command {
            Command::Create {
                id,
                kind: NativeKind::Viewbox,
            } => Some(*id),
            _ => None,
        })
        .unwrap();
    let child = runtime.children(viewbox)[0];
    assert_eq!(runtime.parent(child), Some(viewbox));
    assert!(runtime.batches().iter().flatten().any(|command| {
        matches!(
            command,
            Command::Attach {
                parent,
                child: attached,
                attachment: Attachment::Content,
            } if *parent == viewbox && *attached == child
        )
    }));

    assert!(
        stretch
            .borrow()
            .as_ref()
            .unwrap()
            .try_set(Stretch::UniformToFill)
    );
    reactor.pump();
    assert_eq!(
        reactor.engine().runtime().batches().last().unwrap(),
        &[Command::Update {
            id: viewbox,
            update: NativeUpdate::Control(ControlUpdate::ViewboxStretch(Stretch::UniformToFill)),
        }]
    );
}

#[test]
fn scroll_containers_diff_configuration_replace_handlers_and_reject_stale_events() {
    let version = Rc::new(RefCell::new(None::<State<usize>>));
    let log = Rc::new(RefCell::new(Vec::<(&'static str, ScrollEvent)>::new()));
    let version_for_render = Rc::clone(&version);
    let log_for_render = Rc::clone(&log);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *version_for_render.borrow_mut() = Some(state.clone());
        let child = match state.get().unwrap() {
            0 => ScrollViewer::new(text_block("content"))
                .on_view_changed(scroll_logger(Rc::clone(&log_for_render), "first"))
                .build(),
            1 => ScrollViewer::new(text_block("content"))
                .on_view_changed(scroll_logger(Rc::clone(&log_for_render), "latest"))
                .build(),
            2 => ScrollViewer::new(text_block("content"))
                .horizontal_scroll_bar_visibility(ScrollBarVisibility::Visible)
                .vertical_scroll_bar_visibility(ScrollBarVisibility::Hidden)
                .on_view_changed(scroll_logger(Rc::clone(&log_for_render), "configured"))
                .build(),
            3 => ScrollViewer::new(text_block("content")).build(),
            _ => ScrollView::new(text_block("content"))
                .horizontal_scroll_bar_visibility(ScrollViewBarVisibility::Visible)
                .vertical_scroll_bar_visibility(ScrollViewBarVisibility::Hidden)
                .content_orientation(ScrollOrientation::Both)
                .on_view_changed(scroll_logger(Rc::clone(&log_for_render), "modern"))
                .build(),
        };
        StackPanel::new([child.key(41)]).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let viewer = native_node(&reactor, NativeKind::ScrollViewer);
    let child = reactor.engine().runtime().children(viewer)[0];
    assert_eq!(reactor.engine().runtime().parent(child), Some(viewer));

    let batch_count = reactor.engine().runtime().batches().len();
    assert!(version.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(reactor.engine().runtime().batches().len(), batch_count);

    let event = ScrollEvent {
        horizontal_offset: 12.0,
        vertical_offset: 34.0,
        zoom_factor: 1.5,
        activity: ScrollActivity::Intermediate,
    };
    reactor.engine().runtime().queue_event(NativeEvent::Scroll {
        target: viewer,
        event,
    });
    reactor.pump();
    assert_eq!(log.borrow().last(), Some(&("latest", event)));

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
            .filter(|command| matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::ScrollViewer(
                        ScrollViewerUpdate::HorizontalScrollBarVisibility(
                            ScrollBarVisibility::Visible
                        ) | ScrollViewerUpdate::VerticalScrollBarVisibility(
                            ScrollBarVisibility::Hidden
                        )
                    )),
                } if *id == viewer
            ))
            .count(),
        2
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
            .any(|command| matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::ScrollViewer(
                        ScrollViewerUpdate::ViewChanged(false)
                    )),
                } if *id == viewer
            ))
    );
    let event_count = log.borrow().len();
    reactor.engine().runtime().queue_event(NativeEvent::Scroll {
        target: viewer,
        event,
    });
    reactor.pump();
    assert_eq!(log.borrow().len(), event_count);

    assert!(version.borrow().as_ref().unwrap().try_set(4));
    reactor.pump();
    let modern = native_node(&reactor, NativeKind::ScrollView);
    assert_ne!(modern, viewer);
    reactor.engine().runtime().queue_event(NativeEvent::Scroll {
        target: viewer,
        event,
    });
    reactor.engine().runtime().queue_event(NativeEvent::Scroll {
        target: modern,
        event,
    });
    reactor.pump();
    assert_eq!(log.borrow().last(), Some(&("modern", event)));
    assert_eq!(log.borrow().len(), event_count + 1);
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
                    update: NativeUpdate::Control(ControlUpdate::ScrollView(
                        ScrollViewUpdate::ContentOrientation(ScrollOrientation::Both)
                    )),
                } if *id == modern
            ))
    );
}

#[test]
fn scroll_notifications_rerender_without_mutating_mounted_control_state() {
    let notifications = Rc::new(RefCell::new(None::<State<usize>>));
    let notifications_for_render = Rc::clone(&notifications);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *notifications_for_render.borrow_mut() = Some(state.clone());
        let update = state.clone();
        ScrollViewer::new(text_block(format!("events {}", state.value())))
            .on_view_changed(move |_| {
                assert!(update.try_set(update.value() + 1));
            })
            .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let viewer = native_node(&reactor, NativeKind::ScrollViewer);
    let batch_count = reactor.engine().runtime().batches().len();

    reactor.engine().runtime().queue_event(NativeEvent::Scroll {
        target: viewer,
        event: ScrollEvent::default(),
    });
    reactor.pump();

    assert_eq!(notifications.borrow().as_ref().unwrap().get(), Some(1));
    assert!(
        !reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .skip(batch_count)
            .flatten()
            .any(|command| matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::ScrollViewer(_)),
                } if *id == viewer
            ))
    );
}

fn scroll_logger(
    log: Rc<RefCell<Vec<(&'static str, ScrollEvent)>>>,
    label: &'static str,
) -> impl Fn(ScrollEvent) {
    move |event| log.borrow_mut().push((label, event))
}

#[test]
fn split_view_owns_two_slots_and_dispatches_current_close_handler() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let log = Rc::new(RefCell::new(Vec::new()));
    let phase_for_render = Rc::clone(&phase);
    let log_for_render = Rc::clone(&log);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(state.clone());
        let value = state.get().unwrap();
        if value == 4 {
            return Viewbox::new(text_block("replacement")).build();
        }
        let content = text_block(if value >= 2 {
            "updated content"
        } else {
            "content"
        });
        let pane = text_block(if value >= 2 { "updated pane" } else { "pane" });
        let split = if value == 3 {
            SplitView::display(content, pane)
        } else {
            let log = Rc::clone(&log_for_render);
            SplitView::new(content, pane, move || {
                log.borrow_mut()
                    .push(if value == 0 { "first" } else { "latest" });
            })
        };
        split
            .display_mode(if value >= 2 {
                SplitViewDisplayMode::CompactOverlay
            } else {
                SplitViewDisplayMode::Inline
            })
            .is_pane_open(value < 2)
            .open_pane_length(if value >= 2 { 280.0 } else { 320.0 })
            .compact_pane_length(if value >= 2 { 40.0 } else { 48.0 })
            .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let split = native_node(&reactor, NativeKind::SplitView);
    let [content_slot, pane_slot] = *reactor
        .engine()
        .arena
        .get(split)
        .unwrap()
        .children
        .as_slice()
    else {
        unreachable!()
    };
    assert_eq!(
        reactor.engine().node_kind(content_slot),
        Some(&NodeKind::StructuralSlot(StructuralSlot::Content))
    );
    assert_eq!(
        reactor.engine().node_kind(pane_slot),
        Some(&NodeKind::StructuralSlot(StructuralSlot::Pane))
    );
    assert!(matches!(
        reactor
            .engine()
            .arena
            .get(content_slot)
            .unwrap()
            .mounted
            .as_ref()
            .map(|mounted| &mounted.kind),
        Some(MountedKind::StructuralSlot(StructuralSlot::Content))
    ));
    assert!(matches!(
        reactor
            .engine()
            .arena
            .get(pane_slot)
            .unwrap()
            .mounted
            .as_ref()
            .map(|mounted| &mounted.kind),
        Some(MountedKind::StructuralSlot(StructuralSlot::Pane))
    ));
    let children = reactor.engine().runtime().children(split);
    assert_eq!(children.len(), 2);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| matches!(
                command,
                Command::Attach {
                    parent,
                    child,
                    attachment: Attachment::Content,
                } if *parent == split && *child == children[0]
            ))
    );
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| matches!(
                command,
                Command::Attach {
                    parent,
                    child,
                    attachment: Attachment::Pane,
                } if *parent == split && *child == children[1]
            ))
    );

    let batches = reactor.engine().runtime().batches().len();
    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(reactor.engine().runtime().batches().len(), batches);
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::PaneClosed { target: split });
    reactor.pump();
    assert_eq!(*log.borrow(), ["latest"]);

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    let updates = reactor.engine().runtime().batches().last().unwrap();
    for expected in [
        SplitViewUpdate::DisplayMode(SplitViewDisplayMode::CompactOverlay),
        SplitViewUpdate::OpenPaneLength(280.0),
        SplitViewUpdate::CompactPaneLength(40.0),
    ] {
        assert!(updates.iter().any(|command| matches!(
            command,
            Command::Update {
                id,
                update: NativeUpdate::Control(ControlUpdate::SplitView(actual)),
            } if *id == split && *actual == expected
        )));
    }

    assert!(phase.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
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
                    update: NativeUpdate::Control(ControlUpdate::SplitView(
                        SplitViewUpdate::PaneClosed(false)
                    )),
                } if *id == split
            ))
    );
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::PaneClosed { target: split });
    reactor.pump();
    assert_eq!(*log.borrow(), ["latest"]);

    assert!(phase.borrow().as_ref().unwrap().try_set(4));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::PaneClosed { target: split });
    reactor.pump();
    assert_eq!(*log.borrow(), ["latest"]);
}

#[test]
fn structural_slot_components_keep_peer_roles_across_empty_and_one_root_updates() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(state.clone());
        let value = state.get().unwrap();
        let split_content = component(move |_| {
            if value == 1 {
                text_block("split content")
            } else {
                fragment([])
            }
        });
        let expander_header = component(move |_| {
            if value == 1 {
                text_block("expander header")
            } else {
                fragment([])
            }
        });
        StackPanel::new([
            SplitView::display(split_content, text_block("pane").key(10))
                .build()
                .key(1),
            Expander::display(expander_header, text_block("content").key(20))
                .build()
                .key(2),
        ])
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let split = native_node(&reactor, NativeKind::SplitView);
    let expander = native_node(&reactor, NativeKind::Expander);
    let pane = *reactor.engine().runtime().children(split).first().unwrap();
    let expander_content = *reactor
        .engine()
        .runtime()
        .children(expander)
        .first()
        .unwrap();
    assert_eq!(
        reactor.engine().runtime().attachment(pane),
        Some(Attachment::Pane)
    );
    assert_eq!(
        reactor.engine().runtime().attachment(expander_content),
        Some(Attachment::Content)
    );

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    let split_children = reactor.engine().runtime().children(split);
    assert_eq!(split_children.len(), 2);
    assert_eq!(split_children[1], pane);
    assert_eq!(
        reactor.engine().runtime().attachment(split_children[0]),
        Some(Attachment::Content)
    );
    assert_eq!(
        reactor.engine().runtime().attachment(pane),
        Some(Attachment::Pane)
    );
    let expander_children = reactor.engine().runtime().children(expander);
    assert_eq!(expander_children.len(), 2);
    assert_eq!(expander_children[1], expander_content);
    assert_eq!(
        reactor.engine().runtime().attachment(expander_children[0]),
        Some(Attachment::Header)
    );

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert_eq!(reactor.engine().runtime().children(split), [pane]);
    assert_eq!(
        reactor.engine().runtime().attachment(pane),
        Some(Attachment::Pane)
    );
    assert_eq!(
        reactor.engine().runtime().children(expander),
        [expander_content]
    );
    assert_eq!(
        reactor.engine().runtime().attachment(expander_content),
        Some(Attachment::Content)
    );
}

#[test]
#[should_panic(expected = "NativeParentRejectsChildren")]
fn structural_slot_component_rejects_multiple_projected_roots() {
    let pane = component(|_| fragment([text_block("first"), text_block("second")]));
    let root = SplitView::display(text_block("content"), pane).build();
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
}

#[test]
fn expander_owns_header_and_content_and_dispatches_current_state_handler() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let log = Rc::new(RefCell::new(Vec::new()));
    let phase_for_render = Rc::clone(&phase);
    let log_for_render = Rc::clone(&log);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(state.clone());
        let value = state.get().unwrap();
        if value == 4 {
            return Viewbox::new(text_block("replacement")).build();
        }
        let header = text_block(if value >= 2 {
            "updated header"
        } else {
            "header"
        });
        let content = text_block(if value >= 2 {
            "updated content"
        } else {
            "content"
        });
        let expander = if value == 3 {
            Expander::display(header, content)
        } else {
            let log = Rc::clone(&log_for_render);
            Expander::new(header, content, move |expanded| {
                log.borrow_mut()
                    .push((if value == 0 { "first" } else { "latest" }, expanded));
            })
        };
        expander.expanded(value >= 2).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let expander = native_node(&reactor, NativeKind::Expander);
    let [header_slot, content_slot] = *reactor
        .engine()
        .arena
        .get(expander)
        .unwrap()
        .children
        .as_slice()
    else {
        unreachable!()
    };
    assert_eq!(
        reactor.engine().node_kind(header_slot),
        Some(&NodeKind::StructuralSlot(StructuralSlot::Header))
    );
    assert_eq!(
        reactor.engine().node_kind(content_slot),
        Some(&NodeKind::StructuralSlot(StructuralSlot::Content))
    );
    let children = reactor.engine().runtime().children(expander);
    assert_eq!(children.len(), 2);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| matches!(
                command,
                Command::Attach {
                    parent,
                    child,
                    attachment: Attachment::Header,
                } if *parent == expander && *child == children[0]
            ))
    );
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| matches!(
                command,
                Command::Attach {
                    parent,
                    child,
                    attachment: Attachment::Content,
                } if *parent == expander && *child == children[1]
            ))
    );

    let batches = reactor.engine().runtime().batches().len();
    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(reactor.engine().runtime().batches().len(), batches);
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ExpandedChanged {
            target: expander,
            expanded: true,
        });
    reactor.pump();
    assert_eq!(*log.borrow(), [("latest", true)]);

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .all(|command| !matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::Expander(
                        ExpanderUpdate::Expanded(_)
                    )),
                } if *id == expander
            ))
    );

    assert!(phase.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
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
                    update: NativeUpdate::Control(ControlUpdate::Expander(
                        ExpanderUpdate::ExpandedChanged(false)
                    )),
                } if *id == expander
            ))
    );
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ExpandedChanged {
            target: expander,
            expanded: false,
        });
    reactor.pump();
    assert_eq!(*log.borrow(), [("latest", true)]);

    assert!(phase.borrow().as_ref().unwrap().try_set(4));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ExpandedChanged {
            target: expander,
            expanded: false,
        });
    reactor.pump();
    assert_eq!(*log.borrow(), [("latest", true)]);
}

#[test]
fn rejected_pane_and_expansion_feedback_is_restored_on_rerender() {
    let revision = Rc::new(RefCell::new(None::<State<usize>>));
    let revision_for_render = Rc::clone(&revision);
    let events = Rc::new(Cell::new(0usize));
    let events_for_render = Rc::clone(&events);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *revision_for_render.borrow_mut() = Some(state.clone());
        _ = state.value();
        let split_events = Rc::clone(&events_for_render);
        let expander_events = Rc::clone(&events_for_render);
        let navigation_events = Rc::clone(&events_for_render);
        let tree_events = Rc::clone(&events_for_render);
        StackPanel::new([
            SplitView::new(text_block("content"), text_block("pane"), move || {
                split_events.set(split_events.get() + 1);
            })
            .is_pane_open(true)
            .build(),
            Expander::new(text_block("header"), text_block("content"), move |_| {
                expander_events.set(expander_events.get() + 1);
            })
            .expanded(false)
            .build(),
            NavigationView::new(
                [NavigationItem::new(1, "page")],
                text_block("content"),
                |_| {},
            )
            .pane_open(true, move |_| {
                navigation_events.set(navigation_events.get() + 1);
            })
            .build(),
            TreeView::new([TreeNode::new(1, "root")], move |_, _| {
                tree_events.set(tree_events.get() + 1);
            })
            .build(),
        ])
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let split = native_node(&reactor, NativeKind::SplitView);
    let expander = native_node(&reactor, NativeKind::Expander);
    let navigation = native_node(&reactor, NativeKind::NavigationView);
    let tree = native_node(&reactor, NativeKind::TreeView);

    for event in [
        NativeEvent::PaneClosed { target: split },
        NativeEvent::ExpandedChanged {
            target: expander,
            expanded: true,
        },
        NativeEvent::NavigationPaneOpenChanged {
            target: navigation,
            open: false,
        },
        NativeEvent::TreeNodeExpandedChanged {
            target: tree,
            key: 1,
            expanded: true,
        },
    ] {
        reactor.engine().runtime().queue_event(event);
    }
    reactor.pump();
    assert_eq!(events.get(), 4);

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
    assert!(commands.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::SplitView(
                SplitViewUpdate::IsPaneOpen(true)
            )),
        } if *id == split
    )));
    assert!(commands.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::Expander(
                ExpanderUpdate::Expanded(false)
            )),
        } if *id == expander
    )));
    assert!(commands.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::NavigationView(
                NavigationUpdate::Properties(update)
            )),
        } if *id == navigation && update.pane_open
    )));
    assert!(commands.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::TreeView(update)),
        } if *id == tree
            && matches!(
                update.as_ref(),
                TreeViewUpdate::Nodes(nodes) if !nodes[0].expanded
            )
    )));
}

#[test]
fn accepted_tree_expansion_feedback_updates_mounted_state_before_the_callback() {
    let expanded = Rc::new(RefCell::new(None::<State<bool>>));
    let expanded_for_render = Rc::clone(&expanded);
    let root = component(move |cx| {
        let state = cx.use_state(|| false);
        *expanded_for_render.borrow_mut() = Some(state.clone());
        let update = state.clone();
        TreeView::new(
            [TreeNode::new(1, "root").expanded(state.value())],
            move |key, value| {
                assert_eq!(key, 1);
                assert!(update.try_set(value));
            },
        )
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let tree = native_node(&reactor, NativeKind::TreeView);
    let batch_count = reactor.engine().runtime().batches().len();

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TreeNodeExpandedChanged {
            target: tree,
            key: 1,
            expanded: true,
        });
    reactor.pump();

    assert_eq!(expanded.borrow().as_ref().unwrap().get(), Some(true));
    assert!(
        !reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .skip(batch_count)
            .flatten()
            .any(|command| matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::TreeView(update)),
                } if *id == tree && matches!(update.as_ref(), TreeViewUpdate::Nodes(_))
            ))
    );
}

#[test]
fn display_tree_expansion_feedback_is_restored_without_an_application_callback() {
    let root = TreeView::display([TreeNode::new(1, "root")]).build();
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let tree = native_node(&reactor, NativeKind::TreeView);
    let batch_count = reactor.engine().runtime().batches().len();

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TreeNodeExpandedChanged {
            target: tree,
            key: 1,
            expanded: true,
        });
    reactor.pump();

    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .skip(batch_count)
            .flatten()
            .any(|command| matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::TreeView(update)),
                } if *id == tree
                    && matches!(
                        update.as_ref(),
                        TreeViewUpdate::Nodes(nodes) if !nodes[0].expanded
                    )
            ))
    );
}

#[test]
fn tooltip_keeps_accessory_content_out_of_layout_and_tracks_owner_replacement() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(state.clone());
        let value = state.get().unwrap();
        if value == 3 {
            return Viewbox::new(text_block("replacement")).build();
        }
        let owner = component(move |_| {
            if value < 2 {
                button("owner", || {})
            } else {
                text_block("new owner")
            }
        });
        let content = if value == 0 {
            Grid::new([text_block("tooltip content")]).build()
        } else {
            text_box("updated tooltip content", |_| {})
        };
        let placement = if value == 0 {
            TooltipPlacement::Top
        } else {
            TooltipPlacement::Bottom
        };
        StackPanel::new([owner.tooltip_with(Tooltip::rich(content).placement(placement))]).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let panel = native_node(&reactor, NativeKind::StackPanel);
    let owner = native_node(&reactor, NativeKind::Button);
    let tooltip = native_node(&reactor, NativeKind::ToolTip);
    let content = native_node(&reactor, NativeKind::Grid);
    assert_eq!(reactor.engine().runtime().children(panel), &[owner]);
    assert_eq!(
        reactor.engine().runtime().relation_owner(tooltip),
        Some(owner)
    );
    assert_eq!(reactor.engine().runtime().children(tooltip), &[content]);
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| matches!(
                command,
                Command::BindOwner {
                    owner: command_owner,
                    accessory,
                    relation: OwnerRelation::ToolTip,
                } if *command_owner == owner && *accessory == tooltip
            ))
    );

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    let updated_content = native_node(&reactor, NativeKind::TextBox);
    assert!(!reactor.engine().runtime().contains(content));
    assert_eq!(reactor.engine().runtime().children(panel), &[owner]);
    assert_eq!(
        reactor.engine().runtime().relation_owner(tooltip),
        Some(owner)
    );
    assert_eq!(
        reactor.engine().runtime().children(tooltip),
        &[updated_content]
    );
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
                    update: NativeUpdate::Attached(AttachedUpdate::TooltipPlacement(Some(
                        TooltipPlacement::Bottom
                    ))),
                } if *id == owner
            ))
    );

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    let new_owner = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .rev()
        .find_map(|command| match command {
            Command::Create {
                id,
                kind: NativeKind::TextBlock,
            } if reactor.engine().runtime().contains(*id) => Some(*id),
            _ => None,
        })
        .unwrap();
    let commands = reactor.engine().runtime().batches().last().unwrap();
    let detach = commands
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::UnbindOwner {
                    owner: command_owner,
                    accessory,
                    relation: OwnerRelation::ToolTip,
                } if *command_owner == owner && *accessory == tooltip
            )
        })
        .unwrap();
    let destroy = commands
        .iter()
        .position(|command| matches!(command, Command::Destroy { id } if *id == owner))
        .unwrap();
    let reattach = commands
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::BindOwner {
                    owner: command_owner,
                    accessory,
                    relation: OwnerRelation::ToolTip,
                } if *command_owner == new_owner && *accessory == tooltip
            )
        })
        .unwrap();
    assert!(detach < destroy && destroy < reattach);
    assert_eq!(reactor.engine().runtime().children(panel), &[new_owner]);
    assert_eq!(
        reactor.engine().runtime().relation_owner(tooltip),
        Some(new_owner)
    );
    assert!(commands.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Attached(AttachedUpdate::TooltipPlacement(Some(
                TooltipPlacement::Bottom
            ))),
        } if *id == new_owner
    )));

    assert!(phase.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
    assert!(!reactor.engine().runtime().contains(new_owner));
    assert!(!reactor.engine().runtime().contains(tooltip));
    assert!(!reactor.engine().runtime().contains(updated_content));
}

#[test]
fn teaching_tip_is_owner_bound_and_tracks_owner_replacement() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(state.clone());
        let value = state.get().unwrap();
        if value == 3 {
            return text_block("replacement");
        }
        let owner = component(move |_| {
            if value < 2 {
                button("owner", || {})
            } else {
                text_block("new owner")
            }
        });
        StackPanel::new([owner.teaching_tip(
            TeachingTip::new(if value == 0 { "initial" } else { "updated" })
                .subtitle("details")
                .open(value != 0)
                .light_dismiss(true)
                .action_button("Next")
                .close_button("Close"),
        )])
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let owner = native_node(&reactor, NativeKind::Button);
    let tip = native_node(&reactor, NativeKind::TeachingTip);
    let panel = native_node(&reactor, NativeKind::StackPanel);
    assert_eq!(reactor.engine().runtime().children(panel), &[owner, tip]);
    assert_eq!(reactor.engine().runtime().parent(tip), Some(panel));
    assert_eq!(reactor.engine().runtime().relation_owner(tip), Some(owner));
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| matches!(
                command,
                Command::BindOwner {
                    owner: command_owner,
                    accessory,
                    relation: OwnerRelation::TeachingTipTarget,
                } if *command_owner == owner && *accessory == tip
            ))
    );

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::TeachingTip), tip);
    let commands = reactor.engine().runtime().batches().last().unwrap();
    assert!(commands.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::TeachingTip(
                TeachingTipUpdate::Open(true)
            )),
        } if *id == tip
    )));

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    let new_owner = reactor.engine().runtime().relation_owner(tip).unwrap();
    let commands = reactor.engine().runtime().batches().last().unwrap();
    let detach = commands
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::UnbindOwner {
                    owner: command_owner,
                    accessory,
                    relation: OwnerRelation::TeachingTipTarget,
                } if *command_owner == owner && *accessory == tip
            )
        })
        .unwrap();
    let reattach = commands
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::BindOwner {
                    owner: command_owner,
                    accessory,
                    relation: OwnerRelation::TeachingTipTarget,
                } if *command_owner == new_owner && *accessory == tip
            )
        })
        .unwrap();
    assert!(detach < reattach);
    assert_eq!(
        reactor.engine().runtime().relation_owner(tip),
        Some(new_owner)
    );

    assert!(phase.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
    assert!(!reactor.engine().runtime().contains(tip));
    assert!(!reactor.engine().runtime().contains(new_owner));
}

#[test]
fn teaching_tip_callbacks_use_current_props_and_ignore_events_after_removal() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let log = Rc::new(RefCell::new(Vec::new()));
    let log_for_render = Rc::clone(&log);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(state.clone());
        let value = state.get().unwrap();
        if value == 3 {
            return text_block("removed");
        }
        let tip = TeachingTip::new("tip").open(true);
        let tip = if value == 2 {
            tip
        } else {
            let closed_log = Rc::clone(&log_for_render);
            let action_log = Rc::clone(&log_for_render);
            tip.on_closed(move || closed_log.borrow_mut().push(("closed", value)))
                .on_action_button_click(move || action_log.borrow_mut().push(("action", value)))
        };
        StackPanel::new([button("owner", || {}).teaching_tip(tip)]).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let tip = native_node(&reactor, NativeKind::TeachingTip);

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TeachingTipAction { target: tip });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TeachingTipClosed { target: tip });
    reactor.pump();
    assert_eq!(*log.borrow(), [("action", 1), ("closed", 1)]);

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TeachingTipAction { target: tip });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TeachingTipClosed { target: tip });
    reactor.pump();
    assert_eq!(*log.borrow(), [("action", 1), ("closed", 1)]);

    assert!(phase.borrow().as_ref().unwrap().try_set(3));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TeachingTipAction { target: tip });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::TeachingTipClosed { target: tip });
    reactor.pump();
    assert_eq!(*log.borrow(), [("action", 1), ("closed", 1)]);
}

#[test]
fn drop_down_button_owns_a_nonprojected_flyout_with_reactor_content() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let root =
        component(move |cx| {
            let state = cx.use_state(|| 0usize);
            *phase_for_render.borrow_mut() = Some(state.clone());
            match state.get().unwrap() {
                0 => StackPanel::new([
                    DropDownButton::new("Open", text_block("initial content")).build()
                ])
                .build(),
                1 => StackPanel::new([DropDownButton::new(
                    "Updated",
                    Grid::new([text_block("replacement content")]).build(),
                )
                .build()])
                .build(),
                _ => text_block("removed"),
            }
        });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let owner = native_node(&reactor, NativeKind::DropDownButton);
    let flyout = native_node(&reactor, NativeKind::Flyout);
    let initial_content = reactor.engine().runtime().children(flyout)[0];
    assert_eq!(
        reactor.engine().runtime().relation_owner(flyout),
        Some(owner)
    );
    assert_eq!(reactor.engine().runtime().parent(flyout), None);
    assert_eq!(
        reactor.engine().runtime().children(flyout),
        &[initial_content]
    );
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .any(|command| matches!(
                command,
                Command::BindOwner {
                    owner: command_owner,
                    accessory,
                    relation: OwnerRelation::ButtonFlyout,
                } if *command_owner == owner && *accessory == flyout
            ))
    );

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    let replacement_content = native_node(&reactor, NativeKind::Grid);
    assert_eq!(native_node(&reactor, NativeKind::DropDownButton), owner);
    assert_eq!(native_node(&reactor, NativeKind::Flyout), flyout);
    assert!(!reactor.engine().runtime().contains(initial_content));
    assert_eq!(
        reactor.engine().runtime().children(flyout),
        &[replacement_content]
    );
    assert_eq!(
        reactor.engine().runtime().relation_owner(flyout),
        Some(owner)
    );

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert!(!reactor.engine().runtime().contains(owner));
    assert!(!reactor.engine().runtime().contains(flyout));
    assert!(!reactor.engine().runtime().contains(replacement_content));
}

#[test]
fn flyout_callbacks_use_current_props_and_ignore_events_after_removal() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let log = Rc::new(RefCell::new(Vec::new()));
    let log_for_render = Rc::clone(&log);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(state.clone());
        let value = state.get().unwrap();
        if value == 2 {
            return text_block("removed");
        }
        let opened_log = Rc::clone(&log_for_render);
        let closed_log = Rc::clone(&log_for_render);
        DropDownButton::new("Open", text_block("content"))
            .on_opened(move || opened_log.borrow_mut().push(("opened", value)))
            .on_closed(move || closed_log.borrow_mut().push(("closed", value)))
            .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let flyout = native_node(&reactor, NativeKind::Flyout);

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::FlyoutOpened { target: flyout });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::FlyoutClosed { target: flyout });
    reactor.pump();
    assert_eq!(*log.borrow(), [("opened", 1), ("closed", 1)]);

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::FlyoutOpened { target: flyout });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::FlyoutClosed { target: flyout });
    reactor.pump();
    assert_eq!(*log.borrow(), [("opened", 1), ("closed", 1)]);
}

#[test]
fn keyed_drop_down_buttons_reorder_without_recreating_flyouts() {
    let reordered = Rc::new(RefCell::new(None::<State<bool>>));
    let reordered_for_render = Rc::clone(&reordered);
    let root = component(move |cx| {
        let state = cx.use_state(|| false);
        *reordered_for_render.borrow_mut() = Some(state.clone());
        let first = DropDownButton::new("First", text_block("First flyout"))
            .build()
            .key(1);
        let second = DropDownButton::new("Second", text_block("Second flyout"))
            .build()
            .key(2);
        StackPanel::new(if state.get().unwrap() {
            [second, first]
        } else {
            [first, second]
        })
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let owners = created_nodes(&reactor, NativeKind::DropDownButton);
    let flyouts = created_nodes(&reactor, NativeKind::Flyout);
    let panel = native_node(&reactor, NativeKind::StackPanel);
    assert_eq!(reactor.engine().runtime().children(panel), owners);
    assert_eq!(
        flyouts
            .iter()
            .map(|flyout| reactor.engine().runtime().relation_owner(*flyout).unwrap())
            .collect::<Vec<_>>(),
        owners
    );

    assert!(reordered.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();
    assert_eq!(
        reactor.engine().runtime().children(panel),
        &[owners[1], owners[0]]
    );
    assert_eq!(created_nodes(&reactor, NativeKind::DropDownButton), owners);
    assert_eq!(created_nodes(&reactor, NativeKind::Flyout), flyouts);
    assert_eq!(
        flyouts
            .iter()
            .map(|flyout| reactor.engine().runtime().relation_owner(*flyout).unwrap())
            .collect::<Vec<_>>(),
        owners
    );
}

#[test]
fn image_source_revisions_reject_aba_completions_and_use_current_callbacks() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let loaded = Rc::new(RefCell::new(Vec::new()));
    let loaded_for_render = Rc::clone(&loaded);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(state.clone());
        let value = state.get().unwrap();
        let source = if value == 1 {
            ImageSource::bitmap("ms-appx:///b.png")
        } else {
            ImageSource::bitmap("ms-appx:///a.png")
        };
        let loaded = Rc::clone(&loaded_for_render);
        Image::new(source)
            .stretch(if value == 1 {
                Stretch::UniformToFill
            } else {
                Stretch::Uniform
            })
            .on_load(move |result| {
                assert!(result.is_ok());
                loaded.borrow_mut().push(value);
            })
            .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let image = native_node(&reactor, NativeKind::Image);

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();

    let revisions = reactor
        .engine()
        .runtime()
        .batches()
        .iter()
        .flatten()
        .filter_map(|command| match command {
            Command::Update {
                id,
                update: NativeUpdate::Control(ControlUpdate::Image(update)),
            } if *id == image => Some(update.source_revision),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(revisions, [1, 2, 3]);
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
                    update: NativeUpdate::Control(ControlUpdate::Image(update)),
                } if *id == image && update.stretch == Stretch::UniformToFill
            ))
    );
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
                    update: NativeUpdate::Control(ControlUpdate::Image(update)),
                } if *id == image && update.stretch == Stretch::Uniform
                    && update.source_revision == 3
            ))
    );

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ImageLoad {
            target: image,
            source_revision: 1,
            result: Ok(()),
        });
    reactor.pump();
    assert!(loaded.borrow().is_empty());

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ImageLoad {
            target: image,
            source_revision: 3,
            result: Ok(()),
        });
    reactor.pump();
    assert_eq!(&*loaded.borrow(), &[2]);
}

#[test]
fn command_bar_owns_typed_primary_and_secondary_command_sections() {
    let root = component(move |_| {
        CommandBar::new([
            CommandBarItem::button(1, "Open", || {}),
            CommandBarItem::toggle(2, "Pin", false, |_| {}),
            CommandBarItem::separator(3),
        ])
        .secondary_commands([CommandBarItem::button(4, "Settings", || {})])
        .default_label_position(CommandBarDefaultLabelPosition::Right)
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let bar = native_node(&reactor, NativeKind::CommandBar);
    let button = created_nodes(&reactor, NativeKind::AppBarButton);
    let toggle = native_node(&reactor, NativeKind::AppBarToggleButton);
    let separator = native_node(&reactor, NativeKind::AppBarSeparator);
    assert_eq!(button.len(), 2);
    assert_eq!(
        reactor.engine().runtime().children(bar),
        &[button[0], toggle, separator, button[1]]
    );
    assert_eq!(
        reactor.engine().runtime().attachment(button[0]),
        Some(Attachment::Command {
            section: CommandSection::Primary,
            index: 0,
        })
    );
    assert_eq!(
        reactor.engine().runtime().attachment(toggle),
        Some(Attachment::Command {
            section: CommandSection::Primary,
            index: 1,
        })
    );
    assert_eq!(
        reactor.engine().runtime().attachment(button[1]),
        Some(Attachment::Command {
            section: CommandSection::Secondary,
            index: 0,
        })
    );
    assert_eq!(reactor.engine().arena.get(bar).unwrap().children.len(), 2);
}

#[test]
fn command_bar_keyed_commands_reorder_and_use_current_callbacks() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let events = Rc::new(RefCell::new(Vec::new()));
    let events_for_render = Rc::clone(&events);
    let root = component(move |cx| {
        let value = cx.use_state(|| 0_usize);
        *phase_for_render.borrow_mut() = Some(value.clone());
        if value.get().unwrap() == 2 {
            return text_block("removed");
        }
        let current = value.get().unwrap();
        let click_state = value.clone();
        let toggle_state = value;
        let click_events = Rc::clone(&events_for_render);
        let toggle_events = Rc::clone(&events_for_render);
        let button = CommandBarItem::button(10, "Open", move || {
            click_events
                .borrow_mut()
                .push(("click", click_state.get().unwrap()));
        })
        .icon(Icon::symbol(if current == 0 {
            IconSymbol::EDIT
        } else {
            IconSymbol::SAVE
        }));
        let toggle = CommandBarItem::toggle(20, "Pin", current == 1, move |checked| {
            toggle_events.borrow_mut().push((
                if checked { "on" } else { "off" },
                toggle_state.get().unwrap(),
            ));
        });
        CommandBar::new(if current == 0 {
            vec![button, toggle]
        } else {
            vec![toggle, button]
        })
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let bar = native_node(&reactor, NativeKind::CommandBar);
    let button = native_node(&reactor, NativeKind::AppBarButton);
    let toggle = native_node(&reactor, NativeKind::AppBarToggleButton);

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(reactor.engine().runtime().children(bar), &[toggle, button]);
    assert_eq!(native_node(&reactor, NativeKind::AppBarButton), button);
    assert_eq!(
        native_node(&reactor, NativeKind::AppBarToggleButton),
        toggle
    );

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Click { target: button });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Toggled {
            target: toggle,
            value: false,
        });
    reactor.pump();
    assert_eq!(*events.borrow(), [("click", 1), ("off", 1)]);

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Click { target: button });
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Toggled {
            target: toggle,
            value: true,
        });
    reactor.pump();
    assert_eq!(*events.borrow(), [("click", 1), ("off", 1)]);
}

#[test]
fn rejected_command_bar_toggle_feedback_is_restored_on_rerender() {
    let revision = Rc::new(RefCell::new(None::<State<usize>>));
    let revision_for_render = Rc::clone(&revision);
    let events = Rc::new(Cell::new(0usize));
    let events_for_render = Rc::clone(&events);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *revision_for_render.borrow_mut() = Some(state.clone());
        _ = state.value();
        let events = Rc::clone(&events_for_render);
        CommandBar::new([CommandBarItem::toggle(1, "Pin", false, move |_| {
            events.set(events.get() + 1);
        })])
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let toggle = native_node(&reactor, NativeKind::AppBarToggleButton);
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::Toggled {
            target: toggle,
            value: true,
        });
    reactor.pump();
    assert_eq!(events.get(), 1);

    let batch_count = reactor.engine().runtime().batches().len();
    assert!(revision.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .skip(batch_count)
            .flatten()
            .any(|command| matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::AppBarToggleButton(update)),
                } if *id == toggle && !update.checked
            ))
    );
}

#[test]
fn content_dialog_is_owned_only_and_reconciles_typed_content() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(state.clone());
        match state.get().unwrap() {
            0 => StackPanel::new([
                button("owner", || {}),
                ContentDialog::new("Initial title", text_block("Initial content"))
                    .primary_button("Save")
                    .close_button("Cancel")
                    .build(),
            ])
            .build(),
            1 => StackPanel::new([
                button("owner", || {}),
                ContentDialog::new(
                    "Updated title",
                    Grid::new([text_block("Updated content")]).build(),
                )
                .primary_button("Apply")
                .secondary_button("Later")
                .close_button("Close")
                .secondary_button_enabled(false)
                .open(true)
                .build(),
            ])
            .build(),
            _ => button("owner", || {}),
        }
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();

    let panel = native_node(&reactor, NativeKind::StackPanel);
    let owner = native_node(&reactor, NativeKind::Button);
    let dialog = native_node(&reactor, NativeKind::ContentDialog);
    let [header_slot, content_slot] = *reactor
        .engine()
        .arena
        .get(dialog)
        .unwrap()
        .children
        .as_slice()
    else {
        unreachable!()
    };
    assert_eq!(
        reactor.engine().node_kind(header_slot),
        Some(&NodeKind::StructuralSlot(StructuralSlot::Header))
    );
    assert_eq!(
        reactor.engine().node_kind(content_slot),
        Some(&NodeKind::StructuralSlot(StructuralSlot::Content))
    );
    let initial_children = reactor.engine().runtime().children(dialog).to_vec();
    assert_eq!(
        reactor.engine().node_kind(dialog),
        Some(&NodeKind::OwnedNative)
    );
    assert_eq!(reactor.engine().runtime().children(panel), &[owner]);
    assert_eq!(reactor.engine().runtime().parent(dialog), None);
    assert_eq!(initial_children.len(), 2);

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(native_node(&reactor, NativeKind::ContentDialog), dialog);
    assert_eq!(reactor.engine().runtime().children(panel), &[owner]);
    let updated_children = reactor.engine().runtime().children(dialog);
    assert_eq!(updated_children[0], initial_children[0]);
    assert!(!reactor.engine().runtime().contains(initial_children[1]));
    assert_ne!(updated_children[1], initial_children[1]);
    let commands = reactor.engine().runtime().batches().last().unwrap();
    assert!(commands.iter().any(|command| matches!(
        command,
        Command::Update {
            id,
            update: NativeUpdate::Control(ControlUpdate::ContentDialog(
                update
            )),
        } if *id == dialog && update.open
    )));

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    assert!(!reactor.engine().runtime().contains(dialog));
}

#[test]
fn content_dialog_callbacks_use_current_props_and_ignore_stale_completion() {
    let phase = Rc::new(RefCell::new(None::<State<usize>>));
    let phase_for_render = Rc::clone(&phase);
    let log = Rc::new(RefCell::new(Vec::new()));
    let log_for_render = Rc::clone(&log);
    let root = component(move |cx| {
        let state = cx.use_state(|| 0usize);
        *phase_for_render.borrow_mut() = Some(state.clone());
        let value = state.get().unwrap();
        if value == 2 {
            return text_block("removed");
        }
        let result_log = Rc::clone(&log_for_render);
        StackPanel::new([ContentDialog::new("Title", text_block("Content"))
            .open(true)
            .on_closed(move |result| result_log.borrow_mut().push((value, result)))
            .build()])
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let dialog = native_node(&reactor, NativeKind::ContentDialog);

    assert!(phase.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ContentDialogClosed {
            target: dialog,
            result: ContentDialogResult::Primary,
        });
    reactor.pump();
    assert_eq!(*log.borrow(), [(1, ContentDialogResult::Primary)]);

    assert!(phase.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();
    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ContentDialogClosed {
            target: dialog,
            result: ContentDialogResult::Secondary,
        });
    reactor.pump();
    assert_eq!(*log.borrow(), [(1, ContentDialogResult::Primary)]);
}

#[test]
fn content_dialog_close_syncs_open_state_before_dispatching_the_result() {
    let open = Rc::new(RefCell::new(None::<State<bool>>));
    let open_for_render = Rc::clone(&open);
    let result = Rc::new(Cell::new(ContentDialogResult::None));
    let result_for_render = Rc::clone(&result);
    let root = component(move |cx| {
        let state = cx.use_state(|| true);
        *open_for_render.borrow_mut() = Some(state.clone());
        let close = state.clone();
        let result = Rc::clone(&result_for_render);
        StackPanel::new([ContentDialog::new("Title", text_block("Content"))
            .open(state.value())
            .on_closed(move |value| {
                result.set(value);
                assert!(close.try_set(false));
            })
            .build()])
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let dialog = native_node(&reactor, NativeKind::ContentDialog);
    let batch_count = reactor.engine().runtime().batches().len();

    reactor
        .engine()
        .runtime()
        .queue_event(NativeEvent::ContentDialogClosed {
            target: dialog,
            result: ContentDialogResult::Primary,
        });
    reactor.pump();

    assert_eq!(result.get(), ContentDialogResult::Primary);
    assert_eq!(open.borrow().as_ref().unwrap().get(), Some(false));
    assert!(
        !reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .skip(batch_count)
            .flatten()
            .any(|command| matches!(
                command,
                Command::Update {
                    id,
                    update: NativeUpdate::Control(ControlUpdate::ContentDialog(_)),
                } if *id == dialog
            ))
    );
}

#[test]
fn keyed_tooltip_wrappers_reorder_without_recreating_native_nodes() {
    let reordered = Rc::new(RefCell::new(None::<State<bool>>));
    let reordered_for_render = Rc::clone(&reordered);
    let root = component(move |cx| {
        let state = cx.use_state(|| false);
        *reordered_for_render.borrow_mut() = Some(state.clone());
        let button = button("button", || {})
            .key(1)
            .tooltip(text_block("button tooltip"));
        let check_box = check_box("check box", false, |_| {})
            .key(2)
            .tooltip(text_block("check box tooltip"));
        StackPanel::new(if state.get().unwrap() {
            [check_box, button]
        } else {
            [button, check_box]
        })
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let panel = native_node(&reactor, NativeKind::StackPanel);
    let button = native_node(&reactor, NativeKind::Button);
    let check_box = native_node(&reactor, NativeKind::CheckBox);
    let initial_nodes = reactor.engine().node_count();
    assert_eq!(
        reactor.engine().runtime().children(panel),
        &[button, check_box]
    );

    assert!(reordered.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();

    assert_eq!(reactor.engine().node_count(), initial_nodes);
    assert_eq!(
        reactor.engine().runtime().children(panel),
        &[check_box, button]
    );
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .all(|command| !matches!(command, Command::Create { .. } | Command::Destroy { .. }))
    );
}

#[test]
fn keyed_teaching_tip_wrapper_moves_owner_and_accessory_as_one_element() {
    let reordered = Rc::new(RefCell::new(None::<State<bool>>));
    let reordered_for_render = Rc::clone(&reordered);
    let root = component(move |cx| {
        let state = cx.use_state(|| false);
        *reordered_for_render.borrow_mut() = Some(state.clone());
        let tip = button("owner", || {})
            .key(1)
            .teaching_tip(TeachingTip::new("tip"));
        let sibling = button("sibling", || {}).key(2);
        StackPanel::new(if state.get().unwrap() {
            [sibling, tip]
        } else {
            [tip, sibling]
        })
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let panel = native_node(&reactor, NativeKind::StackPanel);
    let tip = native_node(&reactor, NativeKind::TeachingTip);
    let children = reactor.engine().runtime().children(panel);
    let owner = children[0];
    let sibling = children[2];
    assert_eq!(children, &[owner, tip, sibling]);
    let initial_nodes = reactor.engine().node_count();

    assert!(reordered.borrow().as_ref().unwrap().try_set(true));
    reactor.pump();

    assert_eq!(reactor.engine().node_count(), initial_nodes);
    assert_eq!(
        reactor.engine().runtime().children(panel),
        &[sibling, owner, tip]
    );
    assert!(
        reactor
            .engine()
            .runtime()
            .batches()
            .last()
            .unwrap()
            .iter()
            .all(|command| !matches!(command, Command::Create { .. } | Command::Destroy { .. }))
    );
}

#[test]
fn grid_append_counts_existing_tooltip_owners_as_one_native_child() {
    let count = Rc::new(RefCell::new(None::<State<usize>>));
    let count_for_render = Rc::clone(&count);
    let root = component(move |cx| {
        let state = cx.use_state(|| 1usize);
        *count_for_render.borrow_mut() = Some(state.clone());
        let mut children = vec![grid_child(
            Button::new("owner")
                .on_click(|| {})
                .build()
                .tooltip(text_block("tip"))
                .key(1),
        )];
        if state.get().unwrap() == 2 {
            children.push(grid_child(text_block("tail").key(2)));
        }
        Grid::new(children).build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let owner = native_node(&reactor, NativeKind::Button);

    assert!(count.borrow().as_ref().unwrap().try_set(2));
    reactor.pump();

    assert!(reactor.engine().contains(owner));
    assert_eq!(
        reactor
            .engine()
            .runtime()
            .batches()
            .iter()
            .flatten()
            .filter(|command| matches!(
                command,
                Command::Create {
                    kind: NativeKind::Button,
                    ..
                }
            ))
            .count(),
        1
    );
}

#[test]
fn keyed_grid_batches_pure_suffix_attachment_indexes() {
    let count = Rc::new(RefCell::new(None::<State<usize>>));
    let count_for_render = Rc::clone(&count);
    let root = component(move |cx| {
        let state = cx.use_state(|| 2usize);
        *count_for_render.borrow_mut() = Some(state.clone());
        Grid::new(
            (0..state.get().unwrap())
                .map(|index| grid_child(text_block(index.to_string()).key(index as u64))),
        )
        .build()
    });
    let mut reactor = Reactor::new(RecordingRuntime::default(), root);
    reactor.pump();
    let grid = native_node(&reactor, NativeKind::Grid);

    assert!(count.borrow().as_ref().unwrap().try_set(4));
    reactor.pump();

    let indexes = reactor
        .engine()
        .runtime()
        .batches()
        .last()
        .unwrap()
        .iter()
        .filter_map(|command| match command {
            Command::Attach {
                parent,
                attachment: Attachment::Child { index },
                ..
            } if *parent == grid => Some(*index),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(indexes, [2, 3]);
    assert_eq!(reactor.engine().runtime().children(grid).len(), 4);

    assert!(count.borrow().as_ref().unwrap().try_set(1));
    reactor.pump();
    assert_eq!(reactor.engine().runtime().children(grid).len(), 1);
}
