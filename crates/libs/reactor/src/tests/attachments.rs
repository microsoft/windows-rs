use super::*;

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
