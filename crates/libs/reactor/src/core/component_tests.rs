use super::*;

#[test]
#[cfg(target_pointer_width = "64")]
fn component_runtime_layouts_remain_compact() {
    assert_eq!(size_of::<ComponentEffects>(), 8);
    assert_eq!(size_of::<ComponentEffectState>(), 48);
    assert_eq!(size_of::<TypedScope<(), (), ()>>(), 104);
}

#[test]
fn context_dependencies_compare_as_sets_across_inline_and_many_storage() {
    let first = ContextDependency {
        id: ContextId(1),
        provider: Some(NodeId::from_parts(2, 3)),
    };
    let second = ContextDependency {
        id: ContextId(4),
        provider: None,
    };
    let mut left = ContextDependencies::default();
    left.insert(first);
    left.insert(second);
    let mut right = ContextDependencies::default();
    right.insert(second);
    right.insert(first);

    assert!(left == right);
    assert!(left.contains(&first));
    assert!(left.contains(&second));
}

#[test]
fn context_snapshot_keeps_nearest_provider_and_multiple_contexts() {
    let first = Context::new(0u32);
    let second = Context::new("default");
    let nearest = ContextProvision::new(&first, 1);
    let shadowed = ContextProvision::new(&first, 2);
    let other = ContextProvision::new(&second, "resolved");
    let nearest_node = NodeId::from_parts(1, 0);
    let mut snapshot = ContextSnapshot::default();

    snapshot.insert(nearest_node, &nearest);
    snapshot.insert(NodeId::from_parts(2, 0), &shadowed);
    snapshot.insert(NodeId::from_parts(3, 0), &other);

    assert_eq!(snapshot.get(&first), Some((nearest_node, 1)));
    assert_eq!(
        snapshot.get(&second).map(|(_, value)| value),
        Some("resolved")
    );
}

#[test]
fn duplicate_effect_marker_survives_later_registrations() {
    let mut effects = ComponentEffects::default();
    effects.use_effect("duplicate".into(), 1, || None);
    effects.use_effect("duplicate".into(), 2, || None);
    effects.use_effect("later".into(), 3, || None);

    assert_eq!(
        effects.finish_view(),
        Err(ComponentStoreError::DuplicateEffectKey(EffectKey::from(
            "duplicate"
        )))
    );
}

#[test]
fn panicking_effect_cleanup_preserves_remaining_cleanups() {
    let cleanup_count = Rc::new(Cell::new(0));
    let mut effects = ComponentEffects::default();
    let count = Rc::clone(&cleanup_count);
    effects.use_effect("remaining".into(), (), move || {
        Some(Box::new(move || count.set(count.get() + 1)))
    });
    effects.use_effect("panicking".into(), (), || {
        Some(Box::new(|| panic!("injected cleanup panic")))
    });
    effects.commit();

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        effects.cleanup();
    }));

    assert!(result.is_err());
    effects.cleanup();
    assert_eq!(cleanup_count.get(), 1);
}

use crate::TextBlock;
use crate::core::{WindowId, WindowToken};
use std::cell::Cell;
use std::sync::Barrier;
use std::sync::atomic::AtomicUsize;
use std::time::{Duration, Instant};

struct Passive;

impl Component for Passive {
    type Input = ();
    type Message = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &(), _context: &mut ViewContext<Self>) -> View {
        View::empty()
    }
}

#[test]
fn component_update_defaults_to_no_op() {
    let _ = View::component::<Passive>(());
}

#[derive(Clone)]
struct PanickingEffectInput {
    cleanup_count: Rc<Cell<usize>>,
    panic: Rc<Cell<bool>>,
}

impl PartialEq for PanickingEffectInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.cleanup_count, &other.cleanup_count)
            && Rc::ptr_eq(&self.panic, &other.panic)
    }
}

struct PanickingEffect;

impl Component for PanickingEffect {
    type Input = PanickingEffectInput;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let cleanup_count = Rc::clone(&input.cleanup_count);
        context.use_effect("effect", (), move || {
            Some(Box::new(move || {
                cleanup_count.set(cleanup_count.get() + 1);
            }))
        });
        assert!(!input.panic.get(), "injected view panic");
        View::empty()
    }
}

#[test]
fn panicking_view_preserves_effect_cleanup_state() {
    let cleanup_count = Rc::new(Cell::new(0));
    let panic = Rc::new(Cell::new(false));
    let mut store = store();
    let token = store
        .reserve_component::<PanickingEffect>(PanickingEffectInput {
            cleanup_count: Rc::clone(&cleanup_count),
            panic: Rc::clone(&panic),
        })
        .unwrap();
    store.view(token, ContextSnapshot::default()).unwrap();
    store.commit_effects(token).unwrap();

    panic.set(true);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        store.view(token, ContextSnapshot::default()).unwrap();
    }));

    assert!(result.is_err());
    store.cleanup_effects(token).unwrap();
    assert_eq!(cleanup_count.get(), 1);
}

struct State {
    changed: usize,
    sender: Option<LocalSender<u32>>,
    value: u32,
}

impl Component for State {
    type Input = String;
    type Message = u32;

    fn create(_input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        Self {
            changed: 0,
            sender: Some(context.sender()),
            value: 0,
        }
    }

    fn input_changed(&mut self, _input: &Self::Input, _context: &ComponentContext<Self>) {
        self.changed += 1;
    }

    fn update(&mut self, message: u32, _context: &ComponentContext<Self>) {
        self.value += message;
        if message == 1 {
            self.sender.as_ref().unwrap().send(2);
        }
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        View::empty()
    }
}

fn store() -> ComponentStore {
    ComponentStore::new(WindowToken::new(WindowId::allocate()))
}

fn reserve_state(store: &mut ComponentStore, input: &str) -> ComponentToken {
    store.reserve_component::<State>(input.to_string()).unwrap()
}

#[test]
fn zero_sized_sender_mappers_have_stable_component_identity() {
    fn map(value: u16) -> u32 {
        u32::from(value)
    }

    let mut store = store();
    let first = reserve_state(&mut store, "first");
    let second = reserve_state(&mut store, "second");
    let first_sender = store.sender::<u32>(first).unwrap();
    let second_sender = store.sender::<u32>(second).unwrap();

    assert_eq!(first_sender.callback(map), first_sender.callback(map));
    assert_ne!(first_sender.callback(map), second_sender.callback(map));

    let offset = 1;
    assert_ne!(
        first_sender.callback(move |value: u16| u32::from(value) + offset),
        first_sender.callback(move |value: u16| u32::from(value) + offset)
    );
}

struct DirectProps;

impl Component for DirectProps {
    type Input = String;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _message: Self::Message, _context: &ComponentContext<Self>) {}

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        View::native(TextBlock::new().text(input.clone()))
    }
}

#[test]
fn view_receives_current_store_owned_input() {
    let mut store = store();
    let token = store
        .reserve_component::<DirectProps>("first".to_string())
        .unwrap();
    store.publish(token).unwrap();

    assert_eq!(
        store.view(token, ContextSnapshot::default()).unwrap().view,
        View::native(TextBlock::new().text("first"))
    );
    assert!(store.apply_input(token, "second".to_string()).unwrap());
    assert_eq!(
        store.view(token, ContextSnapshot::default()).unwrap().view,
        View::native(TextBlock::new().text("second"))
    );
}

fn wait_for_status(task: &ComponentTask, status: ComponentTaskStatus) {
    let deadline = Instant::now() + Duration::from_secs(2);
    while task.status() != status {
        assert!(Instant::now() < deadline, "background task timed out");
        std::thread::sleep(Duration::from_millis(1));
    }
}

#[test]
fn reserved_messages_wait_for_publication_and_reentrant_messages_queue() {
    let mut store = store();
    let token = reserve_state(&mut store, "first");
    let sender = store.sender::<u32>(token).unwrap();
    sender.send(1);

    assert_eq!(
        store.drain(10),
        Ok(DrainReport {
            blocked: true,
            dispatched: 0,
            dropped: 0,
            dirty: Vec::new(),
        })
    );
    store.publish(token).unwrap();
    assert_eq!(store.drain(10).unwrap().dispatched, 2);
    assert_eq!(store.component::<State>(token).unwrap().value, 3);
}

#[test]
fn payload_callback_maps_into_the_component_queue() {
    let mut store = store();
    let token = reserve_state(&mut store, "");
    store.publish(token).unwrap();
    let callback = store
        .sender::<u32>(token)
        .unwrap()
        .callback(|value: String| value.len() as u32);

    assert!(callback.call("mapped".to_string()));
    assert_eq!(store.pending(), 1);
    assert_eq!(store.drain(1).unwrap().dispatched, 1);
    assert_eq!(store.component::<State>(token).unwrap().value, 6);
}

struct RepeatedMessage {
    clones: Rc<Cell<usize>>,
    value: u32,
}

impl Clone for RepeatedMessage {
    fn clone(&self) -> Self {
        self.clones.set(self.clones.get() + 1);
        Self {
            clones: Rc::clone(&self.clones),
            value: self.value,
        }
    }
}

struct RepeatedState {
    value: u32,
}

impl Component for RepeatedState {
    type Input = ();
    type Message = RepeatedMessage;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, message: RepeatedMessage, _context: &ComponentContext<Self>) {
        self.value += message.value;
    }

    fn view(&self, _input: &(), _context: &mut ViewContext<Self>) -> View {
        View::empty()
    }
}

#[test]
fn unit_message_callback_clones_for_every_delivery() {
    let mut store = store();
    let token = store.reserve_component::<RepeatedState>(()).unwrap();
    store.publish(token).unwrap();
    let clones = Rc::new(Cell::new(0));
    let callback = store
        .sender::<RepeatedMessage>(token)
        .unwrap()
        .message(RepeatedMessage {
            clones: Rc::clone(&clones),
            value: 7,
        });

    assert!(callback.call(()));
    assert!(callback.call(()));
    assert_eq!(clones.get(), 2);
    assert_eq!(store.drain(2).unwrap().dispatched, 2);
    assert_eq!(store.component::<RepeatedState>(token).unwrap().value, 14);
}

#[test]
fn inputs_are_typed_coalesced_and_applied_before_messages() {
    let mut store = store();
    let token = reserve_state(&mut store, "first");
    store.publish(token).unwrap();

    assert_eq!(store.apply_input(token, "first".to_string()), Ok(false));
    assert_eq!(store.apply_input(token, "second".to_string()), Ok(true));
    assert_eq!(store.component::<State>(token).unwrap().changed, 1);
    assert!(matches!(
        store.apply_input(token, 1u32),
        Err(ComponentStoreError::InputTypeMismatch { .. })
    ));
}

#[test]
fn stale_tokens_cannot_reach_a_reused_slot() {
    let mut store = store();
    let first = reserve_state(&mut store, "");
    store.publish(first).unwrap();
    let old_sender = store.sender::<u32>(first).unwrap();
    store.remove(first).unwrap();
    old_sender.send(1);
    assert_eq!(store.drain(10).unwrap().dropped, 0);

    let second = reserve_state(&mut store, "");
    assert_ne!(first, second);
    store.publish(second).unwrap();
    old_sender.send(4);

    assert_eq!(store.drain(10).unwrap().dropped, 0);
    assert_eq!(store.component::<State>(second).unwrap().value, 0);
}

#[test]
fn sender_creation_checks_the_message_type() {
    let mut store = store();
    let token = reserve_state(&mut store, "");

    assert!(matches!(
        store.sender::<String>(token),
        Err(ComponentStoreError::MessageTypeMismatch { .. })
    ));
}

#[test]
fn background_queue_capacity_rejects_excess_completion() {
    let mut store = store();
    let token = reserve_state(&mut store, "");
    store.publish(token).unwrap();
    {
        let mut queue = store.background.lock().unwrap();
        for _ in 0..BACKGROUND_MESSAGE_QUEUE_CAPACITY {
            queue.envelopes.push_back(BackgroundEnvelope {
                control: Arc::new(TaskControl::new()),
                delivery: BackgroundDelivery::Completion,
                payload: Box::new(0u32),
                rejection: None,
                token,
            });
        }
    }

    let task = TaskSpawner {
        limiter: Arc::clone(&store.task_limiter),
        queue: Arc::clone(&store.background),
        token,
    }
    .spawn_with_rejection::<u32, _>(|_| 1, 7);
    wait_for_status(&task, ComponentTaskStatus::Rejected);
    assert_eq!(
        store.background.lock().unwrap().envelopes.len(),
        BACKGROUND_MESSAGE_QUEUE_CAPACITY + 1
    );
    assert_eq!(
        store
            .drain(BACKGROUND_MESSAGE_QUEUE_CAPACITY + 1)
            .unwrap()
            .dispatched,
        1
    );
    assert_eq!(store.component::<State>(token).unwrap().value, 7);
}

#[test]
fn panicking_background_work_is_rejected() {
    let mut store = store();
    let token = reserve_state(&mut store, "");
    store.publish(token).unwrap();

    let task = TaskSpawner {
        limiter: Arc::clone(&store.task_limiter),
        queue: Arc::clone(&store.background),
        token,
    }
    .spawn_with_rejection::<u32, _>(|_| panic!("injected task panic"), 9);
    wait_for_status(&task, ComponentTaskStatus::Rejected);
    assert_eq!(store.drain(1).unwrap().dispatched, 1);
    assert_eq!(store.component::<State>(token).unwrap().value, 9);
}

#[test]
fn dropping_task_handle_does_not_cancel_delivery() {
    let mut store = store();
    let token = reserve_state(&mut store, "");
    store.publish(token).unwrap();

    drop(
        TaskSpawner {
            limiter: Arc::clone(&store.task_limiter),
            queue: Arc::clone(&store.background),
            token,
        }
        .spawn::<u32, _>(|_| 7),
    );
    let deadline = Instant::now() + Duration::from_secs(2);
    while store.pending() == 0 {
        assert!(Instant::now() < deadline, "background task timed out");
        std::thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(store.drain(1).unwrap().dispatched, 1);
    assert_eq!(store.component::<State>(token).unwrap().value, 7);
}

#[test]
fn failed_wake_rejects_every_completion_waiting_behind_it() {
    let mut store = store();
    let token = reserve_state(&mut store, "");
    store.publish(token).unwrap();
    let wake_barrier = Arc::new(Barrier::new(2));
    let wakes = Arc::new(AtomicUsize::new(0));
    {
        let wake_barrier = Arc::clone(&wake_barrier);
        let wakes = Arc::clone(&wakes);
        store.set_background_waker(Arc::new(move || {
            wakes.fetch_add(1, Ordering::AcqRel);
            wake_barrier.wait();
            false
        }));
    }
    let work_barrier = Arc::new(Barrier::new(3));
    let spawn = TaskSpawner {
        limiter: Arc::clone(&store.task_limiter),
        queue: Arc::clone(&store.background),
        token,
    };
    let first_barrier = Arc::clone(&work_barrier);
    let first = spawn.spawn::<u32, _>(move |_| {
        first_barrier.wait();
        1
    });
    let second_barrier = Arc::clone(&work_barrier);
    let second = spawn.spawn::<u32, _>(move |_| {
        second_barrier.wait();
        2
    });
    work_barrier.wait();
    let deadline = Instant::now() + Duration::from_secs(2);
    while store.background.lock().unwrap().envelopes.len() != 2 {
        assert!(Instant::now() < deadline, "background tasks timed out");
        std::thread::sleep(Duration::from_millis(1));
    }
    wake_barrier.wait();

    wait_for_status(&first, ComponentTaskStatus::Rejected);
    wait_for_status(&second, ComponentTaskStatus::Rejected);
    assert_eq!(wakes.load(Ordering::Acquire), 1);
    assert_eq!(store.pending(), 0);
}

#[test]
fn cancelled_reserved_completion_cannot_deliver_after_publication() {
    let mut store = store();
    let token = reserve_state(&mut store, "");
    let task = TaskSpawner {
        limiter: Arc::clone(&store.task_limiter),
        queue: Arc::clone(&store.background),
        token,
    }
    .spawn::<u32, _>(|_| 4);
    wait_for_status(&task, ComponentTaskStatus::Queued);
    assert!(store.drain(1).unwrap().blocked);

    task.cancel();
    store.publish(token).unwrap();
    assert_eq!(store.drain(1).unwrap().dispatched, 0);
    assert_eq!(store.component::<State>(token).unwrap().value, 0);
}

#[test]
fn local_and_background_messages_alternate() {
    let mut store = store();
    let token = reserve_state(&mut store, "");
    store.publish(token).unwrap();
    let sender = store.sender::<u32>(token).unwrap();
    assert!(sender.send(3));
    let control = Arc::new(TaskControl::new());
    control.queue();
    store
        .background
        .lock()
        .unwrap()
        .envelopes
        .push_back(BackgroundEnvelope {
            control,
            delivery: BackgroundDelivery::Completion,
            payload: Box::new(10u32),
            rejection: None,
            token,
        });

    assert_eq!(store.drain(1).unwrap().dispatched, 1);
    assert_eq!(store.component::<State>(token).unwrap().value, 3);
    assert!(sender.send(100));
    assert_eq!(store.drain(1).unwrap().dispatched, 1);
    assert_eq!(store.component::<State>(token).unwrap().value, 13);
    assert_eq!(store.pending(), 1);
}

#[test]
fn stale_background_completion_cannot_reach_a_reused_scope() {
    let mut store = store();
    let first = reserve_state(&mut store, "");
    store.publish(first).unwrap();
    store.remove(first).unwrap();
    let second = reserve_state(&mut store, "");
    store.publish(second).unwrap();
    let control = Arc::new(TaskControl::new());
    control.queue();
    store
        .background
        .lock()
        .unwrap()
        .envelopes
        .push_back(BackgroundEnvelope {
            control: Arc::clone(&control),
            delivery: BackgroundDelivery::Completion,
            payload: Box::new(8u32),
            rejection: None,
            token: first,
        });

    assert_eq!(store.drain(1).unwrap().dropped, 1);
    assert_eq!(control.status(), ComponentTaskStatus::Cancelled);
    assert_eq!(store.component::<State>(second).unwrap().value, 0);
}

#[test]
fn closed_store_and_live_task_limit_reject_without_spawning() {
    let mut store = store();
    let token = reserve_state(&mut store, "");
    let spawn = TaskSpawner {
        limiter: Arc::clone(&store.task_limiter),
        queue: Arc::clone(&store.background),
        token,
    };
    store
        .task_limiter
        .active
        .store(BACKGROUND_TASK_CAPACITY, Ordering::Release);
    let limited = spawn.spawn::<u32, _>(|_| 1);
    assert_eq!(limited.status(), ComponentTaskStatus::Rejected);
    let restarted = store.restarted(WindowToken::new(WindowId::allocate()));
    assert!(restarted.task_limiter.acquire().is_none());

    store.task_limiter.active.store(0, Ordering::Release);
    store.close();
    let closed = spawn.spawn_with_rejection::<u32, _>(|_| 1, 7);
    assert_eq!(closed.status(), ComponentTaskStatus::Rejected);
    assert_eq!(store.pending(), 0);
}

#[test]
fn valid_sender_wakes_once_and_retirement_blocks_new_traffic() {
    let wakes = Rc::new(Cell::new(0));
    let wake_capture = Rc::clone(&wakes);
    let mut store = store();
    store.set_waker(Rc::new(move || {
        wake_capture.set(wake_capture.get() + 1);
    }));
    let token = reserve_state(&mut store, "");
    store.publish(token).unwrap();
    let sender = store.sender::<u32>(token).unwrap();

    sender.send(1);
    sender.send(2);
    assert_eq!(wakes.get(), 1);
    assert_eq!(store.pending(), 2);
    store.remove(token).unwrap();
    assert_eq!(store.pending(), 0);
    sender.send(3);
    assert_eq!(store.pending(), 0);
    assert_eq!(wakes.get(), 1);
}

#[test]
fn local_message_queue_reports_backpressure_at_its_fixed_capacity() {
    let mut store = store();
    let token = reserve_state(&mut store, "");
    store.publish(token).unwrap();
    let sender = store.sender::<u32>(token).unwrap();

    for value in 0..LOCAL_MESSAGE_QUEUE_CAPACITY {
        assert!(sender.send(value as u32));
    }
    assert!(!sender.send(u32::MAX));
    assert_eq!(store.pending(), LOCAL_MESSAGE_QUEUE_CAPACITY);
}

struct Counter {
    value: u32,
}

impl Component for Counter {
    type Input = u32;
    type Message = u32;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
        context.sender().send(1);
        Self { value: *input }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.value = *input;
    }

    fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
        self.value += message;
    }

    fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        View::native(TextBlock::new().text(self.value.to_string()))
    }
}

#[test]
fn component_factory_creates_with_a_reserved_sender_and_composes() {
    let first = ComponentView::new::<Counter>(2);
    let same = ComponentView::new::<Counter>(2);
    let changed = ComponentView::new::<Counter>(3);
    let different = ComponentView::new::<Passive>(());
    assert_eq!(first, same);
    assert_ne!(first, changed);
    assert_ne!(first, different);

    let mut store = store();
    let token = first.reserve(&mut store).unwrap();
    assert_eq!(store.pending(), 1);
    assert!(store.drain(10).unwrap().blocked);
    store.publish(token).unwrap();
    assert_eq!(store.drain(10).unwrap().dispatched, 1);
    assert_eq!(store.component::<Counter>(token).unwrap().value, 3);
    assert_eq!(
        store.view(token, ContextSnapshot::default()).unwrap().view,
        View::native(TextBlock::new().text("3"))
    );
}
