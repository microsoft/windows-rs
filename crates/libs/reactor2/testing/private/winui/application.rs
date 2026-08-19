use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use super::*;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_EMPTY_APPLICATION_FIXTURE";
const RESOURCES_FIXTURE_ENV: &str = "WINDOWS_REACTOR_APPLICATION_RESOURCES_FIXTURE";

#[test]
#[ignore = "requires the Windows App Runtime"]
fn empty_application_creates_no_native_window() {
    let output = test_reactor_support::run_test_process(
        "winui::tests::application::empty_application_fixture",
        &[(FIXTURE_ENV, "1")],
        Duration::from_secs(30),
    )
    .unwrap();

    test_reactor_support::assert_success(output);
}

#[test]
fn empty_application_fixture() {
    if std::env::var_os(FIXTURE_ENV).is_none() {
        return;
    }

    bootstrap().unwrap();
    run_app_fixture(
        Application::new(std::iter::empty::<Element>()).build(),
        |reactor| {
            assert!(
                RuntimeProbe::new(reactor.engine().runtime())
                    .windows()
                    .next()
                    .is_none(),
                "empty application created a native window"
            );
            Ok(())
        },
    )
    .unwrap();
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn application_resources_update_and_clear() {
    let output = test_reactor_support::run_test_process(
        "winui::tests::application::application_resources_fixture",
        &[(RESOURCES_FIXTURE_ENV, "1")],
        Duration::from_secs(30),
    )
    .unwrap();

    test_reactor_support::assert_success(output);
}

#[test]
fn application_resources_fixture() {
    if std::env::var_os(RESOURCES_FIXTURE_ENV).is_none() {
        return;
    }

    bootstrap().unwrap();
    let first = ApplicationResources::new([
        (
            "ReactorFixtureBrush",
            ApplicationResource::from(Color::rgb(20, 40, 60)),
        ),
        (
            "ReactorFixturePadding",
            ApplicationResource::from(Thickness::uniform(8.0)),
        ),
        ("ReactorFixtureLabel", ApplicationResource::from("First")),
    ]);
    let second = ApplicationResources::new([
        (
            "ReactorFixtureBrush",
            ApplicationResource::from(Color::rgb(60, 40, 20)),
        ),
        ("ReactorFixtureLabel", ApplicationResource::from("Second")),
    ]);
    let open_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_open_state = Rc::clone(&open_state);
    let phase_state = Rc::new(RefCell::new(None::<State<usize>>));
    let publish_phase_state = Rc::clone(&phase_state);
    let first_for_render = first.clone();
    let second_for_render = second.clone();
    let root = component(move |cx| {
        let open = cx.use_state(|| true);
        let phase = cx.use_state(|| 0usize);
        publish_open_state.borrow_mut().replace(open.clone());
        publish_phase_state.borrow_mut().replace(phase.clone());
        let resources = match phase.try_value().unwrap() {
            0 => first_for_render.clone(),
            1 => second_for_render.clone(),
            _ => ApplicationResources::default(),
        };
        let close = open.clone();
        Application::new(if open.try_value().unwrap() {
            vec![
                Window::new(
                    "Resource window",
                    text_block("Resource content"),
                    move || {
                        close.set(false);
                    },
                )
                .build()
                .key(1),
            ]
        } else {
            Vec::new()
        })
        .resources(resources)
        .build()
    });

    run_app_fixture(root, move |reactor| {
        assert_eq!(
            RuntimeProbe::new(reactor.engine().runtime()).application_resources(),
            &first
        );
        assert!(phase_state.borrow().as_ref().unwrap().try_set(1));
        reactor.pump();
        assert_eq!(
            RuntimeProbe::new(reactor.engine().runtime()).application_resources(),
            &second
        );
        assert!(phase_state.borrow().as_ref().unwrap().try_set(2));
        reactor.pump();
        assert!(
            RuntimeProbe::new(reactor.engine().runtime())
                .application_resources()
                .is_empty()
        );
        assert!(open_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        Ok(())
    })
    .unwrap();
}
