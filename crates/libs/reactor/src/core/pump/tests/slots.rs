//! Named-slot mounting and reconciliation contracts.

use super::super::*;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

fn navigation(content: Option<View>, header: Option<View>) -> View {
    let mut slots = Vec::new();
    if let Some(content) = content {
        slots.push(SlotView::new(NavigationViewSlot::Content, content));
    }
    if let Some(header) = header {
        slots.push(SlotView::new(NavigationViewSlot::Header, header));
    }
    NavigationView::new().slots(slots)
}

fn split_view(content: Option<View>, pane: Option<View>) -> View {
    let mut slots = Vec::new();
    if let Some(pane) = pane {
        slots.push(SlotView::new(SplitViewSlot::Pane, pane));
    }
    if let Some(content) = content {
        slots.push(SlotView::new(SplitViewSlot::Content, content));
    }
    SplitView::new()
        .open_pane_length(280.0)
        .compact_pane_length(48.0)
        .display_mode(SplitViewDisplayMode::CompactInline)
        .is_pane_open(true)
        .slots(slots)
}

fn navigation_menu(items: impl IntoIterator<Item = KeyedView>) -> View {
    NavigationView::new().collection_slot(NavigationViewSlot::MenuItems, items)
}

#[test]
fn list_view_items_can_override_the_native_item_minimum_height() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(ListViewItem::new().min_height(16.0).into())
        .unwrap();

    let item = pump.runtime().node(pump.root().unwrap()).unwrap();
    assert_eq!(
        item.property(PropertyId::MinHeight),
        Some(&PropertyValue::F64(16.0))
    );
}

#[test]
fn split_view_properties_and_ui_element_slots_follow_generated_paths() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(split_view(
        Some(View::native(TextBlock::new().text("content"))),
        Some(View::native(StackPanel::new())),
    ))
    .unwrap();
    let root = pump.root().unwrap();
    let recorded = pump.runtime().node(root).unwrap();

    assert_eq!(
        recorded.property(PropertyId::SplitViewOpenPaneLength),
        Some(&PropertyValue::F64(280.0))
    );
    assert_eq!(
        recorded.property(PropertyId::SplitViewCompactPaneLength),
        Some(&PropertyValue::F64(48.0))
    );
    assert_eq!(
        recorded.property(PropertyId::SplitViewDisplayMode),
        Some(&PropertyValue::SplitViewDisplayMode(
            SplitViewDisplayMode::CompactInline
        ))
    );
    assert_eq!(
        recorded.property(PropertyId::SplitViewIsPaneOpen),
        Some(&PropertyValue::Bool(true))
    );
    assert!(recorded.slot(SlotId::SplitViewPane).is_some());
    assert!(recorded.slot(SlotId::SplitViewContent).is_some());

    let property_order = pump.runtime().commands()[0]
        .iter()
        .filter_map(|command| match command {
            Command::SetProperty { property, .. }
                if matches!(
                    property,
                    PropertyId::SplitViewOpenPaneLength
                        | PropertyId::SplitViewCompactPaneLength
                        | PropertyId::SplitViewDisplayMode
                        | PropertyId::SplitViewIsPaneOpen
                ) =>
            {
                Some(*property)
            }

            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        property_order,
        [
            PropertyId::SplitViewOpenPaneLength,
            PropertyId::SplitViewCompactPaneLength,
            PropertyId::SplitViewDisplayMode,
            PropertyId::SplitViewIsPaneOpen,
        ]
    );

    pump.update_view(split_view(None, None)).unwrap();
    let recorded = pump.runtime().node(root).unwrap();
    assert_eq!(recorded.slot(SlotId::SplitViewPane), None);
    assert_eq!(recorded.slot(SlotId::SplitViewContent), None);
}

#[test]
fn navigation_view_sets_display_mode_before_initial_pane_state() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        NavigationView::new()
            .pane_display_mode(NavigationViewPaneDisplayMode::LeftCompact)
            .is_pane_open(false)
            .into(),
    )
    .unwrap();

    let property_order = pump.runtime().commands()[0]
        .iter()
        .filter_map(|command| match command {
            Command::SetProperty { property, .. }
                if matches!(
                    property,
                    PropertyId::NavigationViewPaneDisplayMode
                        | PropertyId::NavigationViewIsPaneOpen
                ) =>
            {
                Some(*property)
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        property_order,
        [
            PropertyId::NavigationViewPaneDisplayMode,
            PropertyId::NavigationViewIsPaneOpen,
        ]
    );
}

#[test]
fn navigation_view_third_slot_uses_the_shared_slot_path() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(NavigationView::new().slots([
        SlotView::new(
            NavigationViewSlot::Content,
            TextBlock::new().text("content"),
        ),
        SlotView::new(NavigationViewSlot::Header, TextBlock::new().text("header")),
        SlotView::new(
            NavigationViewSlot::PaneCustomContent,
            StackPanel::new().children((Button::new(),)),
        ),
    ]))
    .unwrap();
    let root = pump.root().unwrap();
    let recorded = pump.runtime().node(root).unwrap();

    assert!(recorded.slot(SlotId::NavigationViewContent).is_some());
    assert!(recorded.slot(SlotId::NavigationViewHeader).is_some());
    assert!(
        recorded
            .slot(SlotId::NavigationViewPaneCustomContent)
            .is_some()
    );
}

#[test]
fn navigation_item_content_and_typed_icon_slots_update_independently() {
    let view = |icon| {
        let mut slots = vec![SlotView::new(
            NavigationViewItemSlot::Content,
            TextBlock::new().text("Home"),
        )];
        if icon {
            slots.push(SlotView::new(
                NavigationViewItemSlot::Icon,
                SymbolIcon::new().symbol(Symbol::Home),
            ));
        }
        NavigationViewItem::new().slots(slots)
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(view(true)).unwrap();
    let item = pump.root().unwrap();
    assert!(
        pump.runtime()
            .node(item)
            .unwrap()
            .slot(SlotId::NavigationViewItemContent)
            .is_some()
    );
    assert!(
        pump.runtime()
            .node(item)
            .unwrap()
            .slot(SlotId::NavigationViewItemIcon)
            .is_some()
    );

    pump.update_view(view(false)).unwrap();
    assert!(
        pump.runtime()
            .node(item)
            .unwrap()
            .slot(SlotId::NavigationViewItemContent)
            .is_some()
    );
    assert_eq!(
        pump.runtime()
            .node(item)
            .unwrap()
            .slot(SlotId::NavigationViewItemIcon),
        None
    );
}

#[test]
fn selector_bar_uses_keyed_typed_items_and_icon_slots() {
    let item = |text, icon| {
        let item = SelectorBarItem::new()
            .text(text)
            .is_selected(text == "Recent");
        KeyedView::new(
            text,
            if icon {
                item.slots([SlotView::new(
                    SelectorBarItemSlot::Icon,
                    SymbolIcon::new().symbol(Symbol::Favorite),
                )])
            } else {
                item.into()
            },
        )
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(SelectorBar::new().slots([SlotView::collection(
        SelectorBarSlot::Items,
        [item("Recent", false), item("Favorites", true)],
    )]))
    .unwrap();

    let root = pump.root().unwrap();
    let items = pump
        .runtime()
        .node(root)
        .unwrap()
        .slot_children(SlotId::SelectorBarItems);
    assert_eq!(items.len(), 2);
    assert_eq!(
        pump.runtime()
            .node(items[0])
            .unwrap()
            .property(PropertyId::SelectorBarItemText),
        Some(&PropertyValue::Str("Recent".into()))
    );
    assert!(
        pump.runtime()
            .node(items[1])
            .unwrap()
            .slot(SlotId::SelectorBarItemIcon)
            .is_some()
    );
}

#[test]
fn keyed_collection_slot_mounts_updates_reorders_and_removes_items() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(navigation_menu([
        KeyedView::new(
            "home",
            NavigationViewItem::new()
                .tag("home")
                .is_selected(true)
                .slots([SlotView::new(
                    NavigationViewItemSlot::Content,
                    TextBlock::new().text("Home"),
                )]),
        ),
        KeyedView::new(
            "text",
            NavigationViewItem::new().tag("text").slots([SlotView::new(
                NavigationViewItemSlot::Content,
                TextBlock::new().text("Text input"),
            )]),
        ),
    ]))
    .unwrap();

    let root = pump.root().unwrap();
    let items = pump
        .runtime()
        .node(root)
        .unwrap()
        .slot_children(SlotId::NavigationViewMenuItems)
        .to_vec();
    assert_eq!(items.len(), 2);
    assert_eq!(
        pump.runtime()
            .node(items[0])
            .unwrap()
            .property(PropertyId::NavigationViewItemTag),
        Some(&PropertyValue::Str("home".into()))
    );
    assert_eq!(
        pump.runtime()
            .node(items[0])
            .unwrap()
            .property(PropertyId::NavigationViewItemIsSelected),
        Some(&PropertyValue::Bool(true))
    );

    pump.update_view(navigation_menu([
        KeyedView::new(
            "text",
            NavigationViewItem::new()
                .tag("text")
                .is_selected(true)
                .slots([SlotView::new(
                    NavigationViewItemSlot::Content,
                    TextBlock::new().text("Text entry"),
                )]),
        ),
        KeyedView::new(
            "numeric",
            NavigationViewItem::new()
                .tag("numeric")
                .slots([SlotView::new(
                    NavigationViewItemSlot::Content,
                    TextBlock::new().text("Numeric input"),
                )]),
        ),
    ]))
    .unwrap();

    let updated = pump
        .runtime()
        .node(root)
        .unwrap()
        .slot_children(SlotId::NavigationViewMenuItems);
    assert_eq!(updated.len(), 2);
    assert_eq!(updated[0], items[1]);
    assert_ne!(updated[1], items[0]);
    assert!(pump.runtime().node(items[0]).is_none());
    let label = pump
        .runtime()
        .node(updated[0])
        .unwrap()
        .slot(SlotId::NavigationViewItemContent)
        .unwrap();
    assert_eq!(
        pump.runtime()
            .node(label)
            .unwrap()
            .property(PropertyId::TextBlockText),
        Some(&PropertyValue::Str("Text entry".into()))
    );
}

#[test]
fn collection_slot_pure_reorder_moves_retained_items() {
    let items = |order: &[&str]| {
        navigation_menu(order.iter().map(|tag| {
            KeyedView::new(
                *tag,
                NavigationViewItem::new().tag(*tag).slots([SlotView::new(
                    NavigationViewItemSlot::Content,
                    TextBlock::new().text(*tag),
                )]),
            )
        }))
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(items(&["a", "b", "c"])).unwrap();
    let root = pump.root().unwrap();
    let original = pump
        .runtime()
        .node(root)
        .unwrap()
        .slot_children(SlotId::NavigationViewMenuItems)
        .to_vec();

    pump.update_view(items(&["c", "a", "b"])).unwrap();

    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .slot_children(SlotId::NavigationViewMenuItems),
        [original[2], original[0], original[1]]
    );
    assert!(pump.runtime().commands()[1].iter().any(|command| matches!(
        command,
        Command::MoveChild {
            parent,
            slot: Some(SlotId::NavigationViewMenuItems),
            child,
            index: 0,
        } if *parent == root && *child == original[2]
    )));
}

#[test]
fn component_root_replacement_synchronizes_collection_slot() {
    #[derive(Clone)]
    struct Input(Rc<RefCell<Option<LocalSender<bool>>>>);

    impl PartialEq for Input {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.0, &other.0)
        }
    }

    struct Item {
        navigation: bool,
    }

    impl Component for Item {
        type Message = bool;
        type Input = Input;

        fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
            *input.0.borrow_mut() = Some(context.sender());
            Self { navigation: true }
        }

        fn update(&mut self, navigation: bool, _context: &ComponentContext<Self>) {
            self.navigation = navigation;
        }

        fn view(&self, _input: &Input, _context: &mut ViewContext<Self>) -> View {
            let child = if self.navigation {
                NavigationViewItem::new().tag("item").into()
            } else {
                Button::new().content(TextBlock::new().text("replacement"))
            };
            View::keyed_fragment([KeyedView::new("root", child)])
        }
    }

    let sender = Rc::new(RefCell::new(None));
    let view = navigation_menu([KeyedView::new(
        "item",
        View::component::<Item>(Input(Rc::clone(&sender))),
    )]);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(view).unwrap();
    let root = pump.root().unwrap();
    let original = pump
        .runtime()
        .node(root)
        .unwrap()
        .slot_children(SlotId::NavigationViewMenuItems)[0];

    assert!(sender.borrow().as_ref().unwrap().send(false));
    assert_eq!(pump.dispatch_components(1), Ok(1));

    let replacement = pump
        .runtime()
        .node(root)
        .unwrap()
        .slot_children(SlotId::NavigationViewMenuItems)[0];
    assert_ne!(replacement, original);
    assert!(pump.runtime().commands()[1].iter().any(|command| matches!(
        command,
        Command::SynchronizeChildren {
            parent,
            slot: Some(SlotId::NavigationViewMenuItems),
            children,
        } if *parent == root && children.as_slice() == [replacement]
    )));
}

#[test]
fn dense_collection_slot_reorder_preserves_item_identity() {
    let labels = (0..512).map(|index| index.to_string()).collect::<Vec<_>>();
    let mut reversed = labels.clone();
    reversed.reverse();
    let view = |labels: &[String]| {
        navigation_menu(labels.iter().map(|label| {
            KeyedView::new(
                label.clone(),
                NavigationViewItem::new().tag(label).slots([SlotView::new(
                    NavigationViewItemSlot::Content,
                    TextBlock::new().text(label),
                )]),
            )
        }))
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(view(&labels)).unwrap();
    let root = pump.root().unwrap();
    let original = pump
        .runtime()
        .node(root)
        .unwrap()
        .slot_children(SlotId::NavigationViewMenuItems)
        .iter()
        .copied()
        .collect::<HashSet<_>>();

    pump.update_view(view(&reversed)).unwrap();

    assert!(pump.runtime().commands()[1].iter().any(|command| matches!(
        command,
        Command::SynchronizeChildren {
            parent,
            slot: Some(SlotId::NavigationViewMenuItems),
            ..
        } if *parent == root
    )));
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .slot_children(SlotId::NavigationViewMenuItems)
            .iter()
            .copied()
            .collect::<HashSet<_>>(),
        original
    );
}

#[test]
fn collection_slots_reject_single_views_and_multiple_native_roots() {
    let mut pump = Pump::new(RecordingRuntime::default());
    assert_eq!(
        pump.mount_view(NavigationView::new().slots([SlotView::new(
            NavigationViewSlot::MenuItems,
            NavigationViewItem::new(),
        )])),
        Err(PumpError::StructureUnsupported)
    );
    assert!(pump.root().is_none());

    assert_eq!(
        pump.mount_view(navigation_menu([KeyedView::new(
            "multiple",
            View::fragment((NavigationViewItem::new(), NavigationViewItem::new())),
        )])),
        Err(PumpError::StructureUnsupported)
    );
    assert!(pump.root().is_none());

    pump.mount_view(navigation_menu([KeyedView::new(
        "item",
        NavigationViewItem::new(),
    )]))
    .unwrap();
    let root = pump.root().unwrap();
    let item = pump
        .runtime()
        .node(root)
        .unwrap()
        .slot_children(SlotId::NavigationViewMenuItems)[0];
    assert_eq!(
        pump.update_view(navigation_menu([KeyedView::new(
            "item",
            View::fragment((NavigationViewItem::new(), NavigationViewItem::new())),
        )])),
        Err(PumpError::StructureUnsupported)
    );
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .slot_children(SlotId::NavigationViewMenuItems),
        [item]
    );
}

#[test]
fn single_slots_reject_collection_content() {
    let mut pump = Pump::new(RecordingRuntime::default());
    assert_eq!(
        pump.mount_view(NavigationView::new().slots([SlotView::collection(
            NavigationViewSlot::Content,
            [KeyedView::new("content", TextBlock::new())],
        )])),
        Err(PumpError::StructureUnsupported)
    );
    assert!(pump.root().is_none());
}

#[test]
fn named_slots_mount_update_replace_and_clear_independently() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(navigation(
        Some(View::native(TextBlock::new().text("content-1"))),
        Some(View::native(TextBlock::new().text("header-1"))),
    ))
    .unwrap();

    let root = pump.root().unwrap();
    let content = pump
        .runtime()
        .node(root)
        .unwrap()
        .slot(SlotId::NavigationViewContent)
        .unwrap();
    let header = pump
        .runtime()
        .node(root)
        .unwrap()
        .slot(SlotId::NavigationViewHeader)
        .unwrap();

    pump.update_view(navigation(
        Some(View::native(TextBlock::new().text("content-2"))),
        Some(View::native(Slider::new())),
    ))
    .unwrap();

    let recorded = pump.runtime().node(root).unwrap();
    assert_eq!(recorded.slot(SlotId::NavigationViewContent), Some(content));
    assert_ne!(recorded.slot(SlotId::NavigationViewHeader), Some(header));
    assert_eq!(
        pump.runtime()
            .node(content)
            .unwrap()
            .property(PropertyId::TextBlockText),
        Some(&PropertyValue::Str("content-2".into()))
    );

    let header = recorded.slot(SlotId::NavigationViewHeader).unwrap();
    let header_slot = pump.tree.children(root).unwrap()[1];
    assert_eq!(
        pump.tree.kind(header_slot),
        Ok(NodeKind::NamedSlot(SlotId::NavigationViewHeader))
    );
    assert_eq!(pump.tree.children(header_slot).unwrap(), &[header]);
    assert_eq!(
        pump.tree.kind(header),
        Ok(NodeKind::Native(MountedKind::Slider))
    );
    pump.update_view(navigation(
        None,
        Some(View::native(Slider::new().is_enabled(false))),
    ))
    .unwrap();

    let recorded = pump.runtime().node(root).unwrap();
    assert_eq!(recorded.slot(SlotId::NavigationViewContent), None);
    assert_eq!(recorded.slot(SlotId::NavigationViewHeader), Some(header));
    assert_eq!(
        pump.runtime()
            .node(header)
            .unwrap()
            .property(PropertyId::SliderIsEnabled),
        Some(&PropertyValue::Bool(false))
    );
}

#[test]
fn named_slot_rejects_duplicate_assignments_and_multiple_native_roots() {
    let duplicate = NavigationView::new().slots([
        SlotView::new(NavigationViewSlot::Content, View::native(TextBlock::new())),
        SlotView::new(NavigationViewSlot::Content, View::native(Button::new())),
    ]);
    let mut pump = Pump::new(RecordingRuntime::default());
    assert_eq!(
        pump.mount_view(duplicate),
        Err(PumpError::StructureUnsupported)
    );
    assert!(pump.root().is_none());

    let multiple = View::fragment((TextBlock::new(), Button::new()));
    assert_eq!(
        pump.mount_view(navigation(Some(multiple), None)),
        Err(PumpError::StructureUnsupported)
    );
    assert!(pump.root().is_none());
}

#[test]
fn named_slots_preserve_context_and_component_effect_lifecycle() {
    #[derive(Clone)]
    struct Input {
        context: Rc<Context<String>>,
        log: Rc<RefCell<Vec<String>>>,
    }

    impl PartialEq for Input {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.context, &other.context) && Rc::ptr_eq(&self.log, &other.log)
        }
    }

    struct Consumer(Input);

    impl Component for Consumer {
        type Message = ();
        type Input = Input;

        fn create(input: &Input, _context: &ComponentContext<Self>) -> Self {
            Self(input.clone())
        }

        fn input_changed(&mut self, input: &Input, _context: &ComponentContext<Self>) {
            self.0 = input.clone();
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self>) {}

        fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
            let value = context.use_context(&self.0.context);
            let log = Rc::clone(&self.0.log);
            let effect_value = value.clone();
            context.use_effect("context", effect_value.clone(), move || {
                log.borrow_mut().push(format!("setup {effect_value}"));
                Some(Box::new(move || {
                    log.borrow_mut().push(format!("cleanup {effect_value}"));
                }))
            });
            View::native(TextBlock::new().text(value))
        }
    }

    let context = Rc::new(Context::new("default".to_string()));
    let log = Rc::new(RefCell::new(Vec::new()));
    let content = View::component::<Consumer>(Input {
        context: Rc::clone(&context),
        log: Rc::clone(&log),
    });
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::provide(
        &context,
        "provided".to_string(),
        navigation(Some(content), None),
    ))
    .unwrap();

    assert_eq!(&*log.borrow(), &["setup provided"]);
    let navigation_node = pump.tree.children(pump.root().unwrap()).unwrap()[0];
    let content = pump
        .runtime()
        .node(navigation_node)
        .unwrap()
        .slot(SlotId::NavigationViewContent)
        .unwrap();
    assert_eq!(
        pump.runtime()
            .node(content)
            .unwrap()
            .property(PropertyId::TextBlockText),
        Some(&PropertyValue::Str("provided".into()))
    );

    pump.update_view(View::provide(
        &context,
        "provided".to_string(),
        navigation(None, None),
    ))
    .unwrap();
    assert_eq!(&*log.borrow(), &["setup provided", "cleanup provided"]);
}
