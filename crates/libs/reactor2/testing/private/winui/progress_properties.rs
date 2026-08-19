use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use super::progress_access as progress_probe;
use super::*;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_PROGRESS_PROPERTIES_FIXTURE";

#[test]
#[ignore = "requires the Windows App Runtime"]
fn progress_properties_update_and_reset() {
    let output = test_reactor_support::run_test_process(
        "winui::tests::progress_properties::progress_properties_fixture",
        &[(FIXTURE_ENV, "run")],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
fn progress_properties_fixture() {
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
        Application::new(if open.try_value().unwrap() {
            vec![
                Window::new(
                    "Progress properties fixture",
                    content(phase.try_value().unwrap()),
                    move || {
                        close.set(false);
                    },
                )
                .build(),
            ]
        } else {
            Vec::new()
        })
        .build()
    });

    run_app_fixture(root, move |reactor| {
        assert_phase(reactor.engine().runtime(), 0);
        assert!(phase_state.borrow().as_ref().unwrap().try_set(1));
        reactor.pump();
        assert_phase(reactor.engine().runtime(), 1);
        assert!(phase_state.borrow().as_ref().unwrap().try_set(2));
        reactor.pump();
        assert_phase(reactor.engine().runtime(), 2);
        assert!(open_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        Ok(())
    })
    .unwrap();
}

fn content(phase: usize) -> Element {
    let (bar, ring) = match phase {
        0 => (
            ProgressBar::new(25.0).range(0.0, 100.0).build(),
            ProgressRing::new(25.0)
                .range(0.0, 100.0)
                .active(true)
                .build(),
        ),
        1 => (
            ProgressBar::new(225.0)
                .range(200.0, 300.0)
                .is_indeterminate(true)
                .build(),
            ProgressRing::new(225.0)
                .range(200.0, 300.0)
                .active(false)
                .is_indeterminate(true)
                .build(),
        ),
        _ => (
            ProgressBar::new(0.0).build(),
            ProgressRing::new(0.0).build(),
        ),
    };
    StackPanel::new([bar, ring]).build()
}

fn only_node(runtime: &WinUiRuntime, kind: NativeKind) -> NodeId {
    let nodes = RuntimeProbe::new(runtime).nodes(kind);
    assert_eq!(nodes.len(), 1, "expected one {kind:?} node");
    nodes[0]
}

fn assert_phase(runtime: &WinUiRuntime, phase: usize) {
    let bar = only_node(runtime, NativeKind::ProgressBar);
    let ring = only_node(runtime, NativeKind::ProgressRing);
    assert_eq!(
        progress_probe::progress_bar(runtime, bar).unwrap(),
        match phase {
            0 => (25.0, 0.0, 100.0, false),
            1 => (225.0, 200.0, 300.0, true),
            _ => (0.0, 0.0, 100.0, false),
        }
    );
    assert_eq!(
        progress_probe::progress_ring(runtime, ring).unwrap(),
        match phase {
            0 => (25.0, 0.0, 100.0, true, false),
            1 => (225.0, 200.0, 300.0, false, true),
            _ => (0.0, 0.0, 100.0, true, false),
        }
    );
}
