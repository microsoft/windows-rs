use super::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct Input {
    sender: Rc<RefCell<Option<LocalSender<Message>>>>,
    updates: Rc<RefCell<Vec<&'static str>>>,
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.sender, &other.sender) && Rc::ptr_eq(&self.updates, &other.updates)
    }
}

enum Message {
    Bubble,
    Disable,
    Key,
    Pointer,
}

struct RoutedInput {
    enabled: bool,
    handled: bool,
    updates: Rc<RefCell<Vec<&'static str>>>,
}

impl Component for RoutedInput {
    type Input = Input;
    type Message = Message;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        *input.sender.borrow_mut() = Some(context.sender());
        Self {
            enabled: true,
            handled: true,
            updates: Rc::clone(&input.updates),
        }
    }

    fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Bubble => self.handled = false,
            Message::Disable => self.enabled = false,
            Message::Key => self.updates.borrow_mut().push("key"),
            Message::Pointer => self.updates.borrow_mut().push("pointer"),
        }
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let border = Border::new().on_pointer_pressed(context.callback(|_| Message::Pointer));
        if self.enabled {
            let handled = self.handled;
            border
                .on_preview_key_down(context.routed_callback(move |_| {
                    if handled {
                        RoutedMessage::handled(Message::Key)
                    } else {
                        RoutedMessage::bubble(Message::Key)
                    }
                }))
                .into()
        } else {
            border.into()
        }
    }
}

fn setup() -> (
    Pump<RecordingRuntime>,
    NodeId,
    u32,
    Rc<RefCell<Option<LocalSender<Message>>>>,
    Rc<RefCell<Vec<&'static str>>>,
) {
    let sender = Rc::new(RefCell::new(None));
    let updates = Rc::new(RefCell::new(Vec::new()));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<RoutedInput>(Input {
        sender: Rc::clone(&sender),
        updates: Rc::clone(&updates),
    }))
    .unwrap();
    let border = pump
        .runtime()
        .commands()
        .iter()
        .flatten()
        .find_map(|command| match command {
            Command::Create {
                node,
                kind: MountedKind::Border,
            } => Some(*node),
            _ => None,
        })
        .unwrap();
    let revision = pump
        .event_revision(border, EventId::BorderPreviewKeyDown)
        .unwrap();
    (pump, border, revision, sender, updates)
}

fn key() -> KeyEventInfo {
    KeyEventInfo {
        key: VirtualKey::A,
        original_key: VirtualKey::A,
        status: PhysicalKeyStatus::default(),
        modifiers: InputModifiers::CONTROL,
    }
}

#[test]
fn routed_input_decides_handled_synchronously_and_updates_later() {
    let (mut pump, border, revision, _, updates) = setup();

    assert!(
        pump.runtime_mut()
            .route_key(border, EventId::BorderPreviewKeyDown, revision, key())
    );
    assert!(updates.borrow().is_empty());
    assert_eq!(pump.dispatch_events(), Ok(1));
    assert!(updates.borrow().is_empty());
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert_eq!(&*updates.borrow(), &["key"]);
}

#[test]
fn routed_input_preserves_native_event_order() {
    let (mut pump, border, revision, _, updates) = setup();
    let pointer_revision = pump
        .event_revision(border, EventId::BorderPointerPressed)
        .unwrap();
    pump.queue_event(QueuedEvent::new(
        border,
        EventId::BorderPointerPressed,
        pointer_revision,
        EventPayload::PointerEventInfo(PointerEventInfo::default()),
    ));
    assert!(
        pump.runtime_mut()
            .route_key(border, EventId::BorderPreviewKeyDown, revision, key())
    );

    assert_eq!(pump.dispatch_events(), Ok(2));
    assert_eq!(pump.dispatch_components(2), Ok(2));
    assert_eq!(&*updates.borrow(), &["pointer", "key"]);
}

#[test]
fn stale_handled_input_is_diagnosed() {
    let (mut pump, border, revision, sender, _) = setup();
    assert!(
        pump.runtime_mut()
            .route_key(border, EventId::BorderPreviewKeyDown, revision, key())
    );
    assert!(sender.borrow().as_ref().unwrap().send(Message::Disable));
    assert_eq!(pump.dispatch_components(1), Ok(1));

    assert_eq!(pump.dispatch_events(), Ok(0));
    assert_eq!(
        pump.drain_diagnostics(),
        [PumpDiagnostic::HandledInputDropped {
            node: border,
            event: EventId::BorderPreviewKeyDown,
        }]
    );
}

#[test]
fn routed_callback_updates_without_resubscribing() {
    let (mut pump, border, revision, sender, _) = setup();
    assert!(sender.borrow().as_ref().unwrap().send(Message::Bubble));
    assert_eq!(pump.dispatch_components(1), Ok(1));
    assert_eq!(
        pump.event_revision(border, EventId::BorderPreviewKeyDown),
        Some(revision)
    );

    assert!(
        !pump
            .runtime_mut()
            .route_key(border, EventId::BorderPreviewKeyDown, revision, key())
    );
    assert_eq!(pump.dispatch_events(), Ok(1));
}

#[test]
fn full_component_queue_bubbles_input() {
    let (mut pump, border, revision, sender, _) = setup();
    let sender = sender.borrow();
    let sender = sender.as_ref().unwrap();
    for _ in 0..component::LOCAL_MESSAGE_QUEUE_CAPACITY {
        assert!(sender.send(Message::Key));
    }

    assert!(
        !pump
            .runtime_mut()
            .route_key(border, EventId::BorderPreviewKeyDown, revision, key())
    );
    assert_eq!(pump.dispatch_events(), Ok(0));
}

#[test]
fn focus_events_preserve_state_and_directness() {
    let observed = Rc::new(RefCell::new(Vec::new()));
    let got = Rc::clone(&observed);
    let lost = Rc::clone(&observed);
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(
        Border::new()
            .on_got_focus(move |info| got.borrow_mut().push(info))
            .on_lost_focus(move |info| lost.borrow_mut().push(info))
            .into(),
    )
    .unwrap();
    let border = pump.root().unwrap();
    let got_revision = pump
        .event_revision(border, EventId::BorderGotFocus)
        .unwrap();
    let lost_revision = pump
        .event_revision(border, EventId::BorderLostFocus)
        .unwrap();
    let direct = FocusEventInfo {
        state: ElementFocusState::Keyboard,
        is_direct: true,
    };
    let descendant = FocusEventInfo {
        state: ElementFocusState::Unfocused,
        is_direct: false,
    };
    pump.queue_event(QueuedEvent::new(
        border,
        EventId::BorderGotFocus,
        got_revision,
        EventPayload::FocusEventInfo(direct),
    ));
    pump.queue_event(QueuedEvent::new(
        border,
        EventId::BorderLostFocus,
        lost_revision,
        EventPayload::FocusEventInfo(descendant),
    ));

    assert_eq!(pump.dispatch_events(), Ok(2));
    assert_eq!(&*observed.borrow(), &[direct, descendant]);
}
