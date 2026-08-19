use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use super::*;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_INPUT_FIXTURE";

#[test]
#[ignore = "requires the Windows App Runtime"]
fn pointer_subscriptions_attach_replace_and_remove_native_revokers() {
    let output = test_reactor_support::run_test_process(
        "winui::tests::input::pointer_input_fixture",
        &[(FIXTURE_ENV, "1")],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
fn pointer_input_fixture() {
    if std::env::var_os(FIXTURE_ENV).is_none() {
        return;
    }
    bootstrap().unwrap();

    let phase_state = Rc::new(RefCell::new(None::<State<usize>>));
    let publish_phase_state = Rc::clone(&phase_state);
    let open_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_open_state = Rc::clone(&open_state);
    let root = component(move |cx| {
        let phase = cx.use_state(|| 0usize);
        let open = cx.use_state(|| true);
        publish_phase_state.borrow_mut().replace(phase.clone());
        publish_open_state.borrow_mut().replace(open.clone());
        let close = open.clone();
        Application::new(if open.value() {
            vec![
                Window::new("Pointer input fixture", content(phase.value()), move || {
                    close.set(false);
                })
                .build(),
            ]
        } else {
            Vec::new()
        })
        .build()
    });

    run_app_fixture(root, move |reactor| {
        let target = RuntimeProbe::new(reactor.engine().runtime()).nodes(NativeKind::TextBlock)[0];
        assert_all_attached(reactor.engine().runtime(), target);

        assert!(phase_state.borrow().as_ref().unwrap().try_set(1));
        reactor.pump();
        let probe = reactor
            .engine()
            .runtime()
            .pointer_attachment_probe(target)
            .unwrap();
        assert_eq!(
            probe.subscription,
            PointerSubscription {
                events: PointerEvents::PRESSED,
                capture_on_press: false,
            }
        );
        assert!(probe.pressed);
        assert!(!probe.moved);
        assert!(!probe.released);
        assert!(!probe.entered);
        assert!(!probe.exited);
        assert!(!probe.capture_lost);
        assert!(!probe.canceled);
        assert!(!probe.tapped);
        assert!(!probe.right_tapped);

        assert!(phase_state.borrow().as_ref().unwrap().try_set(2));
        reactor.pump();
        assert!(
            reactor
                .engine()
                .runtime()
                .pointer_attachment_probe(target)
                .is_none()
        );

        assert!(open_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        Ok(())
    })
    .unwrap();
}

fn content(phase: usize) -> Element {
    match phase {
        0 => TextBlock::new("target")
            .on_pointer_pressed(|_| {})
            .on_pointer_moved(|_| {})
            .on_pointer_released(|_| {})
            .on_pointer_entered(|_| {})
            .on_pointer_exited(|_| {})
            .on_pointer_capture_lost(|_| {})
            .on_pointer_canceled(|_| {})
            .on_tapped(|| {})
            .on_right_tapped(|| {})
            .capture_pointer_on_press()
            .build(),
        1 => TextBlock::new("target").on_pointer_pressed(|_| {}).build(),
        _ => TextBlock::new("target").build(),
    }
}

fn assert_all_attached(runtime: &WinUiRuntime, target: NodeId) {
    let probe = runtime.pointer_attachment_probe(target).unwrap();
    assert_eq!(
        probe.subscription,
        PointerSubscription {
            events: PointerEvents::PRESSED
                | PointerEvents::MOVED
                | PointerEvents::RELEASED
                | PointerEvents::ENTERED
                | PointerEvents::EXITED
                | PointerEvents::CAPTURE_LOST
                | PointerEvents::CANCELED
                | PointerEvents::TAPPED
                | PointerEvents::RIGHT_TAPPED,
            capture_on_press: true,
        }
    );
    assert!(probe.pressed);
    assert!(probe.moved);
    assert!(probe.released);
    assert!(probe.entered);
    assert!(probe.exited);
    assert!(probe.capture_lost);
    assert!(probe.canceled);
    assert!(probe.tapped);
    assert!(probe.right_tapped);
}
