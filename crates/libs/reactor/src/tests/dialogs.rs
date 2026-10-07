use super::*;

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
