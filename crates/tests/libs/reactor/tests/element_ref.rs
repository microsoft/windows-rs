use std::cell::RefCell;
use std::rc::Rc;
use test_reactor::*;
use windows_reactor::*;

fn noop_request_rerender() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

#[test]
fn element_ref_tracks_mount_update_and_unmount() {
    let first = ElementRef::<TextBoxHandle>::new();
    let second = ElementRef::<TextBoxHandle>::new();
    let with_first: Element = text_box("").element_ref(&first).into();
    let with_second: Element = text_box("").element_ref(&second).into();
    let without_ref: Element = text_box("").into();

    let mut reconciler = Reconciler::new(RecordingBackend::new());
    let id = reconciler
        .reconcile(None, &with_first, None, noop_request_rerender())
        .unwrap();
    assert!(first.is_mounted());
    assert!(!second.is_mounted());

    reconciler.reconcile(
        Some(&with_first),
        &with_second,
        Some(id),
        noop_request_rerender(),
    );
    assert!(!first.is_mounted());
    assert!(second.is_mounted());

    reconciler.reconcile(
        Some(&with_second),
        &without_ref,
        Some(id),
        noop_request_rerender(),
    );
    assert!(!second.is_mounted());
}

#[test]
fn element_ref_clears_before_native_destroy() {
    let reference = ElementRef::<TextBoxHandle>::new();
    let text_box: Element = text_box("").element_ref(&reference).into();
    let replacement: Element = text_block("done").into();

    let mut reconciler = Reconciler::new(RecordingBackend::new());
    let id = reconciler
        .reconcile(None, &text_box, None, noop_request_rerender())
        .unwrap();
    assert!(reference.is_mounted());

    reconciler.reconcile(
        Some(&text_box),
        &replacement,
        Some(id),
        noop_request_rerender(),
    );
    assert!(!reference.is_mounted());
}

#[test]
fn use_element_ref_is_identity_stable() {
    let mut cx = RenderCx::for_test();
    cx.begin_render();
    let first = cx.use_element_ref::<TextBoxHandle>();

    cx.begin_render();
    let second = cx.use_element_ref::<TextBoxHandle>();

    assert_eq!(first, second);
}

#[test]
fn focus_returns_false_while_unmounted() {
    let reference = ElementRef::<TextBoxHandle>::new();
    assert!(!reference.focus().unwrap());
}

struct ReferenceRoot {
    reference: ElementRef<TextBoxHandle>,
}

impl Component for ReferenceRoot {
    fn render(&self, _: &(), _: &mut RenderCx) -> Element {
        text_box("").element_ref(&self.reference).into()
    }
}

#[derive(Clone, Default)]
struct TestDispatcher {
    jobs: Rc<RefCell<Vec<Box<dyn FnOnce()>>>>,
}

impl TestDispatcher {
    fn drain(&self) {
        while let Some(job) = self.jobs.borrow_mut().pop() {
            job();
        }
    }
}

impl Dispatcher for TestDispatcher {
    fn enqueue(&self, _: DispatcherQueuePriority, job: Box<dyn FnOnce()>) -> bool {
        self.jobs.borrow_mut().push(job);
        true
    }
}

#[test]
fn render_host_drop_clears_element_ref() {
    let reference = ElementRef::<TextBoxHandle>::new();
    let dispatcher = TestDispatcher::default();
    let host = RenderHost::new(
        RecordingBackend::new(),
        Box::new(ReferenceRoot {
            reference: reference.clone(),
        }),
        dispatcher.clone(),
    );
    host.kick();
    dispatcher.drain();
    assert!(reference.is_mounted());

    drop(host);
    assert!(!reference.is_mounted());
}
