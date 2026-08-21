use super::super::*;
use crate::native::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

fn focus_count(runtime: &RecordingRuntime) -> usize {
    runtime
        .commands()
        .iter()
        .flatten()
        .filter(|command| matches!(command, Command::Focus { .. }))
        .count()
}

#[test]
fn unmounted_reference_rejects_focus() {
    let reference = ElementRef::<TextBox>::new();
    assert!(!reference.request_focus());
}

#[test]
fn imperative_queue_rejects_excess_work_and_drains_with_a_budget() {
    const WORK_BUDGET: usize = 64;
    let reference = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBox::new().element_ref(&reference).into())
        .unwrap();

    for _ in 0..4_096 {
        assert!(reference.request_focus());
    }
    assert!(!reference.request_focus());
    assert_eq!(pump.process_imperatives(), Ok(WORK_BUDGET));
    assert!(pump.native_work_pending());
    assert_eq!(focus_count(pump.runtime()), WORK_BUDGET);
}

#[test]
fn mount_binds_only_after_successful_native_apply() {
    let reference = ElementRef::<TextBox>::new();
    let mut failed = RecordingRuntime::default();
    failed.fail_at(0);
    let mut pump = Pump::new(failed);
    assert!(matches!(
        pump.mount(TextBox::new().element_ref(&reference).into()),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert!(!reference.request_focus());

    let reference = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBox::new().element_ref(&reference).into())
        .unwrap();
    assert!(reference.request_focus());
    assert_eq!(pump.process_imperatives(), Ok(1));
    assert_eq!(focus_count(pump.runtime()), 1);
}

#[test]
fn reference_swap_and_removal_unbind_the_published_owner() {
    let first = ElementRef::<TextBox>::new();
    let second = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBox::new().element_ref(&first).into())
        .unwrap();

    pump.update(TextBox::new().element_ref(&second).into())
        .unwrap();
    assert!(!first.request_focus());
    assert!(second.request_focus());

    pump.update(TextBlock::new().into()).unwrap();
    assert!(!second.request_focus());
    assert_eq!(pump.process_imperatives(), Ok(0));
}

#[test]
fn stale_request_is_discarded_after_replacement_and_window_epoch_change() {
    let reference = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBox::new().element_ref(&reference).into())
        .unwrap();
    assert!(reference.request_focus());
    pump.update(TextBlock::new().into()).unwrap();
    assert_eq!(pump.process_imperatives(), Ok(0));

    pump.shutdown();
    assert!(!reference.request_focus());
    assert_eq!(pump.process_imperatives(), Ok(0));
    assert_eq!(focus_count(pump.runtime()), 0);
}

#[test]
fn failed_update_does_not_publish_candidate_reference() {
    let current = ElementRef::<TextBox>::new();
    let candidate = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBox::new().element_ref(&current).into())
        .unwrap();
    pump.runtime_mut().fail_at(0);

    assert!(matches!(
        pump.update(
            TextBox::new()
                .text("native mutation")
                .element_ref(&candidate)
                .into()
        ),
        Err(PumpError::NativeApplyFailed(_))
    ));
    assert!(current.request_focus());
    assert!(!candidate.request_focus());
}

#[test]
fn failed_planning_does_not_publish_candidate_reference() {
    let candidate = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(TextBlock::new().into()).unwrap();

    assert_eq!(
        pump.update_view(View::fragment((
            TextBox::new().element_ref(&candidate),
            TextBlock::new(),
        ))),
        Err(PumpError::StructureUnsupported)
    );
    assert!(!candidate.request_focus());
}

#[test]
fn shutdown_and_window_close_clear_references() {
    let shutdown = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBox::new().element_ref(&shutdown).into())
        .unwrap();
    pump.shutdown();
    assert!(!shutdown.request_focus());

    let closed = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(TextBox::new().element_ref(&closed).into())
        .unwrap();
    pump.native_window_closed();
    assert!(!closed.request_focus());
}

#[test]
fn windows_have_isolated_imperative_queues() {
    let first = ElementRef::<TextBox>::new();
    let second = ElementRef::<TextBox>::new();
    let mut left = Pump::new(RecordingRuntime::default());
    let mut right = Pump::new(RecordingRuntime::default());
    left.mount(TextBox::new().element_ref(&first).into())
        .unwrap();
    right
        .mount(TextBox::new().element_ref(&second).into())
        .unwrap();

    assert!(first.request_focus());
    assert_eq!(right.process_imperatives(), Ok(0));
    assert_eq!(left.process_imperatives(), Ok(1));
    assert_eq!(focus_count(right.runtime()), 0);
    assert_eq!(focus_count(left.runtime()), 1);
}

#[test]
fn one_reference_cannot_own_two_published_elements() {
    let reference = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    assert_eq!(
        pump.mount_view(StackPanel::new().children((
            TextBox::new().element_ref(&reference),
            TextBox::new().element_ref(&reference),
        ))),
        Err(PumpError::DuplicateElementRef)
    );
    assert!(!reference.request_focus());

    let mut first = Pump::new(RecordingRuntime::default());
    first
        .mount(TextBox::new().element_ref(&reference).into())
        .unwrap();
    let mut second = Pump::new(RecordingRuntime::default());
    assert_eq!(
        second.mount(TextBox::new().element_ref(&reference).into()),
        Err(PumpError::DuplicateElementRef)
    );
    assert!(reference.request_focus());
}

#[derive(Clone)]
struct DuplicateOwnerProps {
    dropped: Rc<Cell<usize>>,
    reference: ElementRef<TextBox>,
}

impl PartialEq for DuplicateOwnerProps {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.dropped, &other.dropped) && self.reference == other.reference
    }
}

struct DuplicateOwner(DuplicateOwnerProps);

impl Drop for DuplicateOwner {
    fn drop(&mut self) {
        self.0.dropped.set(self.0.dropped.get() + 1);
    }
}

impl Component for DuplicateOwner {
    type Message = ();
    type Props = DuplicateOwnerProps;

    fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self(props.clone())
    }

    fn update(&mut self, (): (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, _props: &Self::Props, _context: &mut ViewContext<Self>) -> View {
        StackPanel::new().children((
            TextBox::new().element_ref(&self.0.reference),
            TextBox::new().element_ref(&self.0.reference),
        ))
    }
}

#[test]
fn duplicate_reference_validation_removes_component_reservations() {
    let dropped = Rc::new(Cell::new(0));
    let reference = ElementRef::new();
    let mut pump = Pump::new(RecordingRuntime::default());

    assert_eq!(
        pump.mount_view(View::component::<DuplicateOwner>(DuplicateOwnerProps {
            dropped: Rc::clone(&dropped),
            reference,
        })),
        Err(PumpError::DuplicateElementRef)
    );
    assert_eq!(dropped.get(), 1);
}

struct PropReference;

impl Component for PropReference {
    type Message = ();
    type Props = ElementRef<TextBox>;

    fn create(_props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, (): (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, reference: &Self::Props, _context: &mut ViewContext<Self>) -> View {
        TextBox::new().element_ref(reference).into()
    }
}

#[test]
fn duplicate_reference_retry_recomposes_staged_component_props() {
    let shared = ElementRef::<TextBox>::new();
    let mut owner = Pump::new(RecordingRuntime::default());
    owner
        .mount(TextBox::new().element_ref(&shared).into())
        .unwrap();

    let original = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<PropReference>(original.clone()))
        .unwrap();

    assert_eq!(
        pump.update_view(View::component::<PropReference>(shared.clone())),
        Err(PumpError::DuplicateElementRef)
    );
    assert!(original.request_focus());
    assert!(shared.request_focus());
    assert_eq!(owner.process_imperatives(), Ok(1));
    assert_eq!(pump.process_imperatives(), Ok(1));

    owner.shutdown();
    pump.update_view(View::component::<PropReference>(shared.clone()))
        .unwrap();
    assert!(!original.request_focus());
    assert!(shared.request_focus());
}

#[derive(Clone)]
struct EffectProps {
    accepted: Rc<Cell<bool>>,
    reference: ElementRef<TextBox>,
}

impl PartialEq for EffectProps {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.accepted, &other.accepted) && self.reference == other.reference
    }
}

struct EffectFocus;

impl Component for EffectFocus {
    type Message = ();
    type Props = EffectProps;

    fn create(_props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, (): (), _context: &mut ComponentContext<Self>) {}

    fn view(&self, props: &Self::Props, context: &mut ViewContext<Self>) -> View {
        let reference = props.reference.clone();
        let accepted = Rc::clone(&props.accepted);
        context.use_effect("focus", (), move || {
            accepted.set(reference.request_focus());
            None
        });
        TextBox::new().element_ref(&props.reference).into()
    }
}

#[test]
fn effect_setup_can_enqueue_focus_after_publication() {
    let accepted = Rc::new(Cell::new(false));
    let reference = ElementRef::<TextBox>::new();
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<EffectFocus>(EffectProps {
        accepted: Rc::clone(&accepted),
        reference,
    }))
    .unwrap();

    assert!(accepted.get());
    assert_eq!(pump.process_imperatives(), Ok(1));
    assert_eq!(focus_count(pump.runtime()), 1);
}

#[derive(Clone)]
struct LocalProps {
    exposed: Rc<RefCell<Option<(ElementRef<TextBox>, ElementRef<TextBox>)>>>,
    sender: Rc<RefCell<Option<LocalSender<bool>>>>,
}

impl PartialEq for LocalProps {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.exposed, &other.exposed) && Rc::ptr_eq(&self.sender, &other.sender)
    }
}

struct LocalReference {
    first: ElementRef<TextBox>,
    second: ElementRef<TextBox>,
    use_second: bool,
}

impl Component for LocalReference {
    type Message = bool;
    type Props = LocalProps;

    fn create(props: &Self::Props, context: &mut ComponentContext<Self>) -> Self {
        let first = ElementRef::new();
        let second = ElementRef::new();
        *props.exposed.borrow_mut() = Some((first.clone(), second.clone()));
        *props.sender.borrow_mut() = Some(context.sender());
        Self {
            first,
            second,
            use_second: false,
        }
    }

    fn update(&mut self, message: bool, _context: &mut ComponentContext<Self>) {
        self.use_second = message;
    }

    fn view(&self, _props: &Self::Props, _context: &mut ViewContext<Self>) -> View {
        let reference = if self.use_second {
            &self.second
        } else {
            &self.first
        };
        TextBox::new().element_ref(reference).into()
    }
}

#[test]
fn local_component_fast_path_commits_reference_changes() {
    let exposed = Rc::new(RefCell::new(None));
    let sender = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<LocalReference>(LocalProps {
        exposed: Rc::clone(&exposed),
        sender: Rc::clone(&sender),
    }))
    .unwrap();
    let (first, second) = exposed.borrow().clone().unwrap();
    assert!(first.request_focus());
    assert!(!second.request_focus());
    pump.process_imperatives().unwrap();

    assert!(sender.borrow().as_ref().unwrap().send(true));
    pump.dispatch_components(1).unwrap();
    assert!(!first.request_focus());
    assert!(second.request_focus());
    assert_eq!(pump.process_imperatives(), Ok(1));
}

struct Removal {
    reference: ElementRef<TextBox>,
    removed: bool,
}

#[derive(Clone)]
struct RemovalProps(Rc<RefCell<Option<(ElementRef<TextBox>, LocalSender<()>)>>>);

impl PartialEq for RemovalProps {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Component for Removal {
    type Message = ();
    type Props = RemovalProps;

    fn create(props: &Self::Props, context: &mut ComponentContext<Self>) -> Self {
        let reference = ElementRef::new();
        *props.0.borrow_mut() = Some((reference.clone(), context.sender()));
        Self {
            reference,
            removed: false,
        }
    }

    fn update(&mut self, (): (), _context: &mut ComponentContext<Self>) {
        self.removed = true;
    }

    fn view(&self, _props: &Self::Props, _context: &mut ViewContext<Self>) -> View {
        if self.removed {
            TextBlock::new().into()
        } else {
            TextBox::new().element_ref(&self.reference).into()
        }
    }
}

#[test]
fn component_publication_precedes_queued_imperative_work() {
    let exposed = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<Removal>(RemovalProps(Rc::clone(
        &exposed,
    ))))
    .unwrap();
    let (reference, sender) = exposed.borrow().clone().unwrap();

    assert!(reference.request_focus());
    assert!(sender.send(()));
    pump.dispatch_components(1).unwrap();
    assert!(!reference.request_focus());
    assert_eq!(pump.process_imperatives(), Ok(0));
    assert_eq!(focus_count(pump.runtime()), 0);
}

struct BacklogRemoval {
    reference: ElementRef<TextBox>,
    removed: bool,
}

#[derive(Clone)]
struct BacklogProps(Rc<RefCell<Option<(ElementRef<TextBox>, LocalSender<bool>)>>>);

impl PartialEq for BacklogProps {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Component for BacklogRemoval {
    type Message = bool;
    type Props = BacklogProps;

    fn create(props: &Self::Props, context: &mut ComponentContext<Self>) -> Self {
        let reference = ElementRef::new();
        *props.0.borrow_mut() = Some((reference.clone(), context.sender()));
        Self {
            reference,
            removed: false,
        }
    }

    fn update(&mut self, remove: bool, _context: &mut ComponentContext<Self>) {
        self.removed |= remove;
    }

    fn view(&self, _props: &Self::Props, _context: &mut ViewContext<Self>) -> View {
        if self.removed {
            TextBlock::new().into()
        } else {
            TextBox::new().element_ref(&self.reference).into()
        }
    }
}

#[test]
fn imperative_work_waits_for_the_component_message_backlog() {
    let exposed = Rc::new(RefCell::new(None));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount_view(View::component::<BacklogRemoval>(BacklogProps(Rc::clone(
        &exposed,
    ))))
    .unwrap();
    let (reference, sender) = exposed.borrow().clone().unwrap();
    assert!(reference.request_focus());
    for _ in 0..64 {
        assert!(sender.send(false));
    }
    assert!(sender.send(true));

    assert_eq!(pump.dispatch_components(64), Ok(64));
    assert_eq!(pump.process_imperatives(), Ok(0));
    assert!(reference.request_focus());

    assert_eq!(pump.dispatch_components(64), Ok(1));
    assert!(!reference.request_focus());
    assert_eq!(pump.process_imperatives(), Ok(0));
    assert_eq!(focus_count(pump.runtime()), 0);
}
