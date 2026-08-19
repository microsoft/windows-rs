use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use super::status_access as status_probe;
use super::*;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_STATUS_PROPERTIES_FIXTURE";

#[test]
#[ignore = "requires the Windows App Runtime"]
fn status_controls_update_and_preserve_controlled_state() {
    let output = test_reactor_support::run_test_process(
        "winui::tests::status_properties::status_properties_fixture",
        &[(FIXTURE_ENV, "run")],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
fn status_properties_fixture() {
    if std::env::var_os(FIXTURE_ENV).is_none() {
        return;
    }

    bootstrap().unwrap();
    let phase_state = Rc::new(RefCell::new(None::<State<usize>>));
    let publish_phase_state = Rc::clone(&phase_state);
    let open_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_open_state = Rc::clone(&open_state);
    let close_requests = Rc::new(Cell::new(0usize));
    let render_close_requests = Rc::clone(&close_requests);
    let root = component(move |cx| {
        let phase = cx.use_state(|| 0usize);
        let open = cx.use_state(|| true);
        publish_phase_state.borrow_mut().replace(phase.clone());
        publish_open_state.borrow_mut().replace(open.clone());
        let badge = match phase.value() {
            0 => InfoBadge::dot().build(),
            1 => InfoBadge::numeric(42).build(),
            _ => InfoBadge::dot().build(),
        };
        let close_requests = Rc::clone(&render_close_requests);
        let bar = match phase.value() {
            0 => InfoBar::new("Initial")
                .message("First")
                .on_close_requested(move || close_requests.set(close_requests.get() + 1))
                .build(),
            1 => InfoBar::new("Updated")
                .message("Second")
                .warning()
                .closable(false)
                .on_close_requested(move || close_requests.set(close_requests.get() + 1))
                .build(),
            _ => InfoBar::new("Updated")
                .message("Second")
                .warning()
                .closable(false)
                .open(false)
                .on_close_requested(move || close_requests.set(close_requests.get() + 1))
                .build(),
        };
        let picture = match phase.value() {
            0 => PersonPicture::new().display_name("Ada Lovelace").build(),
            1 => PersonPicture::new().initials("WR").build(),
            _ => PersonPicture::new().build(),
        };
        let content = StackPanel::new([badge, bar, picture]).build();
        let close = open.clone();
        Application::new(if open.value() {
            vec![
                Window::new("Status properties fixture", content, move || {
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
        assert_value(reactor.engine().runtime(), -1);
        assert_info_bar(
            reactor.engine().runtime(),
            ("Initial", "First", 0, true, true),
        );
        assert_person_picture(reactor.engine().runtime(), ("Ada Lovelace", ""));
        let bar = RuntimeProbe::new(reactor.engine().runtime()).nodes(NativeKind::InfoBar)[0];
        assert!(status_probe::request_info_bar_close(reactor.engine().runtime(), bar).unwrap());
        assert_eq!(close_requests.get(), 0);
        reactor.pump();
        assert_eq!(close_requests.get(), 1);
        assert!(phase_state.borrow().as_ref().unwrap().try_set(1));
        reactor.pump();
        assert_value(reactor.engine().runtime(), 42);
        assert_info_bar(
            reactor.engine().runtime(),
            ("Updated", "Second", 2, true, false),
        );
        assert_person_picture(reactor.engine().runtime(), ("", "WR"));
        assert!(phase_state.borrow().as_ref().unwrap().try_set(2));
        reactor.pump();
        assert_value(reactor.engine().runtime(), -1);
        assert_info_bar(
            reactor.engine().runtime(),
            ("Updated", "Second", 2, false, false),
        );
        assert_person_picture(reactor.engine().runtime(), ("", ""));
        assert_eq!(close_requests.get(), 1);
        assert!(open_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        Ok(())
    })
    .unwrap();
}

fn assert_person_picture(runtime: &WinUiRuntime, expected: (&str, &str)) {
    let pictures = RuntimeProbe::new(runtime).nodes(NativeKind::PersonPicture);
    assert_eq!(pictures.len(), 1);
    let actual = status_probe::person_picture_properties(runtime, pictures[0]).unwrap();
    assert_eq!((actual.0.as_str(), actual.1.as_str()), expected);
}

fn assert_info_bar(runtime: &WinUiRuntime, expected: (&str, &str, i32, bool, bool)) {
    let bars = RuntimeProbe::new(runtime).nodes(NativeKind::InfoBar);
    assert_eq!(bars.len(), 1);
    let actual = status_probe::info_bar_properties(runtime, bars[0]).unwrap();
    assert_eq!(
        (
            actual.0.as_str(),
            actual.1.as_str(),
            actual.2,
            actual.3,
            actual.4
        ),
        expected
    );
}

fn assert_value(runtime: &WinUiRuntime, expected: i32) {
    let badges = RuntimeProbe::new(runtime).nodes(NativeKind::InfoBadge);
    assert_eq!(badges.len(), 1);
    assert_eq!(
        status_probe::info_badge_value(runtime, badges[0]).unwrap(),
        expected
    );
}
