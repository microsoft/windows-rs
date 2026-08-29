use super::super::*;
use std::cell::RefCell;
use std::rc::Rc;

fn dialog(open: bool) -> View {
    ContentDialog::new()
        .title("Delete this item?")
        .primary_button_text("Delete")
        .secondary_button_text("Archive")
        .close_button_text("Cancel")
        .is_primary_button_enabled(true)
        .is_secondary_button_enabled(true)
        .is_open(open)
        .on_closed(|_| {})
        .content(TextBlock::new().text("This action cannot be undone."))
}

fn dialog_node(runtime: &RecordingRuntime) -> NodeId {
    runtime
        .commands()
        .iter()
        .flatten()
        .find_map(|command| match command {
            Command::Create {
                node,
                kind: MountedKind::ContentDialog,
            } => Some(*node),
            _ => None,
        })
        .unwrap()
}

fn dialog_nodes(runtime: &RecordingRuntime) -> Vec<NodeId> {
    runtime
        .commands()
        .iter()
        .flatten()
        .filter_map(|command| match command {
            Command::Create {
                node,
                kind: MountedKind::ContentDialog,
            } => Some(*node),
            _ => None,
        })
        .collect()
}

fn host(open: bool) -> View {
    StackPanel::new().children((TextBlock::new().text("Page"), dialog(open)))
}

fn dialog_pair(first_open: bool, second_open: bool) -> View {
    StackPanel::new().keyed_children([
        KeyedView::new(
            "first",
            ContentDialog::new()
                .title("First")
                .is_open(first_open)
                .on_closed(|_| {}),
        ),
        KeyedView::new(
            "second",
            ContentDialog::new()
                .title("Second")
                .is_open(second_open)
                .on_closed(|_| {}),
        ),
    ])
}

#[test]
fn mounts_closed_without_showing_and_owns_rich_content() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(host(false)).unwrap();

    let dialog = dialog_node(pump.runtime());
    assert_eq!(
        pump.runtime().content_dialog(dialog),
        Some(RecordedContentDialog::default())
    );
    let content = pump.runtime().node(dialog).unwrap().children()[0];
    assert_eq!(
        pump.runtime()
            .node(content)
            .unwrap()
            .property(PropertyId::TextBlockText),
        Some(&PropertyValue::Str(
            "This action cannot be undone.".to_string()
        ))
    );
}

#[test]
fn opens_once_and_stays_stable_across_rerenders() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(host(true)).unwrap();
    let dialog = dialog_node(pump.runtime());
    assert_eq!(pump.runtime().content_dialog(dialog).unwrap().show_count, 1);
    let batches = pump.runtime().commands().len();

    pump.update_view(host(true)).unwrap();

    assert_eq!(dialog_node(pump.runtime()), dialog);
    assert_eq!(pump.runtime().content_dialog(dialog).unwrap().show_count, 1);
    assert!(
        !pump.runtime().commands()[batches..]
            .iter()
            .flatten()
            .any(|command| matches!(command, Command::SetContentDialogOpen { .. }))
    );
}

#[test]
fn declarative_close_hides_and_pending_reopen_waits_for_completion() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(
        StackPanel::new().children((
            TextBlock::new().text("Page"),
            ContentDialog::new()
                .title("Lifecycle only")
                .is_open(true)
                .content(TextBlock::new().text("No result callback")),
        )),
    )
    .unwrap();
    let dialog = dialog_node(pump.runtime());
    let revision = pump
        .event_revision(dialog, EventId::ContentDialogClosed)
        .unwrap();

    pump.update_view(
        StackPanel::new().children((
            TextBlock::new().text("Page"),
            ContentDialog::new()
                .title("Lifecycle only")
                .is_open(false)
                .content(TextBlock::new().text("No result callback")),
        )),
    )
    .unwrap();
    let closed = pump.runtime().content_dialog(dialog).unwrap();
    assert!(!closed.desired_open);
    assert!(closed.pending);
    assert_eq!(closed.hide_count, 1);

    pump.update_view(
        StackPanel::new().children((
            TextBlock::new().text("Page"),
            ContentDialog::new()
                .title("Lifecycle only")
                .is_open(true)
                .content(TextBlock::new().text("No result callback")),
        )),
    )
    .unwrap();
    let reopening = pump.runtime().content_dialog(dialog).unwrap();
    assert!(reopening.queued);
    assert_eq!(reopening.show_count, 1);

    pump.runtime_mut()
        .complete_content_dialog(dialog, revision, ContentDialogResult::None);
    assert_eq!(pump.dispatch_events().unwrap(), 0);
    let reopened = pump.runtime().content_dialog(dialog).unwrap();
    assert!(reopened.pending);
    assert!(!reopened.queued);
    assert_eq!(reopened.show_count, 2);
}

#[test]
fn declarative_hide_suppresses_closed_callback_and_idle_completion_is_ignored() {
    let calls = Rc::new(RefCell::new(0));
    let view = |open, calls: Rc<RefCell<usize>>| {
        StackPanel::new().children((
            TextBlock::new().text("Page"),
            ContentDialog::new()
                .is_open(open)
                .on_closed(move |_| *calls.borrow_mut() += 1),
        ))
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(view(true, Rc::clone(&calls))).unwrap();
    let dialog = dialog_node(pump.runtime());
    let revision = pump
        .event_revision(dialog, EventId::ContentDialogClosed)
        .unwrap();

    pump.update_view(view(false, Rc::clone(&calls))).unwrap();
    pump.runtime_mut()
        .complete_content_dialog(dialog, revision, ContentDialogResult::None);
    assert_eq!(pump.dispatch_events().unwrap(), 0);
    assert_eq!(*calls.borrow(), 0);

    pump.runtime_mut()
        .complete_content_dialog(dialog, revision, ContentDialogResult::Primary);
    assert_eq!(pump.dispatch_events().unwrap(), 0);
    assert_eq!(*calls.borrow(), 0);
}

#[test]
fn completion_routes_typed_result_through_component_messages_and_closes() {
    #[derive(Clone)]
    struct Input(Rc<RefCell<Option<ContentDialogResult>>>);

    impl PartialEq for Input {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.0, &other.0)
        }
    }

    struct DialogComponent {
        open: bool,
        result: Rc<RefCell<Option<ContentDialogResult>>>,
    }

    enum Message {
        Closed(ContentDialogResult),
    }

    impl Component for DialogComponent {
        type Input = Input;
        type Message = Message;

        fn create(input: &Input, _context: &ComponentContext<Self>) -> Self {
            Self {
                open: true,
                result: Rc::clone(&input.0),
            }
        }

        fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
            match message {
                Message::Closed(result) => {
                    *self.result.borrow_mut() = Some(result);
                    self.open = false;
                }
            }
        }

        fn view(&self, _input: &Input, context: &mut ViewContext<Self>) -> View {
            StackPanel::new().children((
                TextBlock::new().text("Page"),
                ContentDialog::new()
                    .title("Question")
                    .is_open(self.open)
                    .on_closed(context.callback(Message::Closed))
                    .content(TextBlock::new().text("Choose")),
            ))
        }
    }

    let result = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<DialogComponent>(Input(Rc::clone(
        &result,
    ))))
    .unwrap();
    let dialog = dialog_node(pump.runtime());
    let revision = pump
        .event_revision(dialog, EventId::ContentDialogClosed)
        .unwrap();

    pump.runtime_mut()
        .complete_content_dialog(dialog, revision, ContentDialogResult::Secondary);
    assert_eq!(pump.dispatch_events().unwrap(), 1);
    pump.dispatch_components(1).unwrap();

    assert_eq!(*result.borrow(), Some(ContentDialogResult::Secondary));
    let state = pump.runtime().content_dialog(dialog).unwrap();
    assert!(!state.desired_open);
    assert!(!state.pending);
}

#[test]
fn removal_hides_before_unsubscribe_and_destroy() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(host(true)).unwrap();
    let dialog = dialog_node(pump.runtime());

    pump.update_view(StackPanel::new().children((TextBlock::new().text("Page"),)))
        .unwrap();

    assert!(pump.runtime().node(dialog).is_none());
    let commands = pump.runtime().commands().last().unwrap();
    let hide = commands
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::SetContentDialogOpen {
                    node,
                    open: false,
                    ..
                } if *node == dialog
            )
        })
        .unwrap();
    let unsubscribe = commands
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::UnsubscribeEvent {
                    node,
                    event: EventId::ContentDialogClosed
                } if *node == dialog
            )
        })
        .unwrap();
    let destroy = commands
        .iter()
        .position(|command| *command == Command::Destroy { node: dialog })
        .unwrap();
    assert!(hide < unsubscribe);
    assert!(unsubscribe < destroy);
}

#[test]
fn replaces_local_content_without_replacing_the_dialog() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(StackPanel::new().children((dialog(false),)))
        .unwrap();
    let dialog = dialog_node(pump.runtime());
    let old_content = pump.runtime().node(dialog).unwrap().children()[0];

    pump.update_view(
        StackPanel::new().children((ContentDialog::new()
            .title("Delete this item?")
            .is_open(false)
            .content(Border::new().content(TextBlock::new().text("Rich replacement"))),)),
    )
    .unwrap();

    assert_eq!(dialog_node(pump.runtime()), dialog);
    let content = pump.runtime().node(dialog).unwrap().children()[0];
    assert_ne!(content, old_content);
    assert!(pump.runtime().node(old_content).is_none());
}

#[test]
fn replacing_open_native_dialog_hides_old_and_serializes_replacement_show() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(host(true)).unwrap();
    let old = dialog_node(pump.runtime());
    let revision = pump
        .event_revision(old, EventId::ContentDialogClosed)
        .unwrap();
    let first_update_batch = pump.runtime().commands().len();

    pump.update_view(StackPanel::new().children((
        TextBlock::new().text("Page"),
        ContentDialog::new().title("Replacement").is_open(true),
    )))
    .unwrap();

    let dialogs = dialog_nodes(pump.runtime());
    let replacement = *dialogs.last().unwrap();
    assert_ne!(replacement, old);
    let commands = pump.runtime().commands()[first_update_batch..]
        .iter()
        .flatten()
        .collect::<Vec<_>>();
    let hide = commands
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::SetContentDialogOpen {
                    node,
                    open: false,
                    ..
                } if *node == old
            )
        })
        .unwrap();
    let destroy = commands
        .iter()
        .position(|command| matches!(command, &&Command::Destroy { node } if node == old))
        .unwrap();
    let show = commands
        .iter()
        .position(|command| {
            matches!(
                command,
                Command::SetContentDialogOpen {
                    node,
                    open: true,
                    ..
                } if *node == replacement
            )
        })
        .unwrap();
    assert!(hide < destroy);
    assert!(destroy < show);
    assert!(pump.runtime().content_dialog(replacement).unwrap().queued);
    assert_eq!(
        pump.runtime()
            .content_dialog(replacement)
            .unwrap()
            .show_count,
        0
    );

    pump.runtime_mut()
        .complete_content_dialog(old, revision, ContentDialogResult::None);
    assert_eq!(pump.dispatch_events().unwrap(), 0);
    let replacement = pump.runtime().content_dialog(replacement).unwrap();
    assert!(replacement.pending);
    assert_eq!(replacement.show_count, 1);
}

#[test]
fn serializes_two_open_dialogs_owned_by_the_same_native_root() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(StackPanel::new().children((
        TextBlock::new().text("Page"),
        Grid::new().children((dialog(true),)),
        Grid::new().children((ContentDialog::new().title("Second").is_open(true),)),
    )))
    .unwrap();
    let dialogs = dialog_nodes(pump.runtime());
    let [first, second] = dialogs.as_slice() else {
        panic!("expected two dialogs");
    };
    let first_state = pump.runtime().content_dialog(*first).unwrap();
    let second_state = pump.runtime().content_dialog(*second).unwrap();
    assert_eq!(
        usize::from(first_state.pending) + usize::from(second_state.pending),
        1
    );
    assert_eq!(
        usize::from(first_state.queued) + usize::from(second_state.queued),
        1
    );
    let (active, waiting) = if first_state.pending {
        (*first, *second)
    } else {
        (*second, *first)
    };

    let revision = pump
        .event_revision(active, EventId::ContentDialogClosed)
        .unwrap();
    pump.runtime_mut()
        .complete_content_dialog(active, revision, ContentDialogResult::Primary);
    assert_eq!(pump.dispatch_events().unwrap(), 1);
    assert_eq!(
        pump.runtime().content_dialog(waiting).unwrap().show_count,
        1
    );
}

#[test]
fn preserves_a_reopen_queued_behind_an_older_dialog_request() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(dialog_pair(true, false)).unwrap();
    let dialogs = dialog_nodes(pump.runtime());
    let [first, second] = dialogs.as_slice() else {
        panic!("expected two dialogs");
    };

    pump.update_view(dialog_pair(true, true)).unwrap();
    pump.update_view(dialog_pair(false, true)).unwrap();
    pump.update_view(dialog_pair(true, true)).unwrap();

    let first_revision = pump
        .event_revision(*first, EventId::ContentDialogClosed)
        .unwrap();
    pump.runtime_mut()
        .complete_content_dialog(*first, first_revision, ContentDialogResult::None);
    assert_eq!(pump.dispatch_events().unwrap(), 0);
    let first_state = pump.runtime().content_dialog(*first).unwrap();
    let second_state = pump.runtime().content_dialog(*second).unwrap();
    assert!(first_state.desired_open);
    assert!(first_state.queued);
    assert!(!first_state.pending);
    assert!(second_state.pending);

    let second_revision = pump
        .event_revision(*second, EventId::ContentDialogClosed)
        .unwrap();
    pump.runtime_mut()
        .complete_content_dialog(*second, second_revision, ContentDialogResult::None);
    assert_eq!(pump.dispatch_events().unwrap(), 1);
    let first_state = pump.runtime().content_dialog(*first).unwrap();
    assert!(first_state.pending);
    assert!(!first_state.queued);
    assert_eq!(first_state.show_count, 2);
}

#[test]
fn cancels_a_queued_dialog_before_the_active_dialog_completes() {
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(dialog_pair(true, false)).unwrap();
    let dialogs = dialog_nodes(pump.runtime());
    let [first, second] = dialogs.as_slice() else {
        panic!("expected two dialogs");
    };

    pump.update_view(dialog_pair(true, true)).unwrap();
    assert!(pump.runtime().content_dialog(*second).unwrap().queued);
    pump.update_view(dialog_pair(true, false)).unwrap();
    let second_state = pump.runtime().content_dialog(*second).unwrap();
    assert!(!second_state.desired_open);
    assert!(!second_state.queued);

    let first_revision = pump
        .event_revision(*first, EventId::ContentDialogClosed)
        .unwrap();
    pump.runtime_mut()
        .complete_content_dialog(*first, first_revision, ContentDialogResult::None);
    assert_eq!(pump.dispatch_events().unwrap(), 1);
    let second_state = pump.runtime().content_dialog(*second).unwrap();
    assert!(!second_state.pending);
    assert_eq!(second_state.show_count, 0);
}

#[test]
fn keyed_parent_reconciliation_preserves_the_owned_dialog() {
    let view = |label| {
        StackPanel::new().keyed_children([
            KeyedView::new("label", TextBlock::new().text(label)),
            KeyedView::new("dialog", dialog(true)),
        ])
    };
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(view("First")).unwrap();
    let dialog = dialog_node(pump.runtime());

    pump.update_view(view("Second")).unwrap();

    assert_eq!(dialog_node(pump.runtime()), dialog);
    assert_eq!(pump.runtime().content_dialog(dialog).unwrap().show_count, 1);
}

#[test]
fn rejects_direct_native_and_nested_overlay_ownership() {
    let mut direct = Pump::new(RecordingRuntime::default());
    assert_eq!(
        direct.mount_view(View::native(Element::from(ContentDialog::new()))),
        Err(PumpError::StructureUnsupported)
    );

    let mut nested = Pump::new(RecordingRuntime::default());
    assert_eq!(
        nested.mount_view(
            ContentDialog::new()
                .content(ContentDialog::new().content(TextBlock::new().text("Nested")))
        ),
        Err(PumpError::StructureUnsupported)
    );

    let mut attached = Pump::new(RecordingRuntime::default());
    assert_eq!(
        attached.mount_view(
            Button::new()
                .content(TextBlock::new().text("Owner"))
                .flyout_with(Flyout::rich(dialog(false)))
        ),
        Err(PumpError::StructureUnsupported)
    );

    let mut ordinary_content = Pump::new(RecordingRuntime::default());
    assert_eq!(
        ordinary_content.mount_view(Border::new().content(dialog(false))),
        Err(PumpError::StructureUnsupported)
    );

    let mut named_slot = Pump::new(RecordingRuntime::default());
    assert_eq!(
        named_slot.mount_view(
            SplitView::new().slots([SlotView::new(SplitViewSlot::Content, dialog(false),)])
        ),
        Err(PumpError::StructureUnsupported)
    );
}
