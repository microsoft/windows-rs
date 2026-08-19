use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use super::*;
use crate::winui::window::tests as window_probe;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_WINDOW_LIFECYCLE_FIXTURE";

fn run_case(case: &str) {
    let output = test_reactor_support::run_test_process(
        "winui::tests::window_lifecycle::window_lifecycle_fixture",
        &[(FIXTURE_ENV, case)],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn ownership_close_reopen_and_constraints() {
    run_case("multi-window");
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn background_content_replacement_preserves_focus() {
    run_case("focus");
}

#[test]
fn window_lifecycle_fixture() {
    let Some(case) = std::env::var_os(FIXTURE_ENV) else {
        return;
    };

    bootstrap().unwrap();
    match case.to_str().unwrap() {
        "multi-window" => multi_window_fixture(),
        "focus" => focus_fixture(),
        case => panic!("unknown window lifecycle fixture: {case}"),
    }
}

fn multi_window_fixture() {
    let first_close_requests = Rc::new(Cell::new(0));
    let close_requests = Rc::clone(&first_close_requests);
    let reopened_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_reopened_state = Rc::clone(&reopened_state);
    let reopened_constraints = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_reopened_constraints = Rc::clone(&reopened_constraints);
    let survived_stale_exit = Rc::new(Cell::new(false));
    let timer = Rc::new(RefCell::new(None::<TestTimer>));
    let window_reference = WindowRef::new();
    let render_window_reference = window_reference.clone();
    let root = component(move |cx| {
        let first_open = cx.use_state(|| true);
        let second_open = cx.use_state(|| true);
        let reopened_open = cx.use_state(|| false);
        let constrain_reopened = cx.use_state(|| true);
        publish_reopened_state
            .borrow_mut()
            .replace(reopened_open.clone());
        publish_reopened_constraints
            .borrow_mut()
            .replace(constrain_reopened.clone());
        let mut windows = Vec::new();
        let owned_windows = if second_open.try_value().unwrap() {
            let open = second_open;
            vec![
                Window::new("Second window", text_block("Second content"), move || {
                    open.set(false);
                })
                .presenter(WindowPresenter::CompactOverlay)
                .build()
                .key(2),
            ]
        } else {
            Vec::new()
        };
        if first_open.try_value().unwrap() {
            let open = first_open;
            let reopen = reopened_open.clone();
            let requests = Rc::clone(&close_requests);
            windows.push(
                Window::new("First window", text_block("First content"), move || {
                    let count = requests.get();
                    requests.set(count + 1);
                    if count != 0 {
                        open.set(false);
                        reopen.set(true);
                    }
                })
                .owned_windows(owned_windows)
                .reference(&render_window_reference)
                .build()
                .key(1),
            );
        }
        if reopened_open.try_value().unwrap() {
            let open = reopened_open;
            let window = Window::new(
                "Reopened window",
                text_block("Reopened content"),
                move || {
                    open.set(false);
                },
            )
            .client_size(640.0, 480.0);
            let window = if constrain_reopened.try_value().unwrap() {
                window.client_constraints(WindowConstraints {
                    min_width: Some(320.0),
                    min_height: None,
                    max_width: None,
                    max_height: Some(800.0),
                })
            } else {
                window
            };
            windows.push(window.build().key(3));
        }
        Application::new(windows).build()
    });

    let finish_reopened_state = Rc::clone(&reopened_state);
    let finish_reopened_constraints = Rc::clone(&reopened_constraints);
    let finish_survived = Rc::clone(&survived_stale_exit);
    let finish_timer = Rc::clone(&timer);
    let fixture_window_reference = window_reference;
    run_app_fixture(root, move |reactor| {
        let runtime = reactor.engine().runtime();
        let windows: Vec<_> = RuntimeProbe::new(runtime).windows().collect();
        assert_eq!(windows.len(), 2);
        let first = windows
            .iter()
            .copied()
            .find(|id| window_probe::title(runtime, *id) == "First window")
            .unwrap();
        let second = windows
            .iter()
            .copied()
            .find(|id| window_probe::title(runtime, *id) == "Second window")
            .unwrap();
        assert_eq!(window_probe::owner(runtime, second), Some(first));
        assert_eq!(
            window_probe::presenter(runtime, second)?,
            WindowPresenter::CompactOverlay
        );
        assert_eq!(
            window_probe::constraints(runtime, first),
            WindowConstraints::default()
        );
        assert!(fixture_window_reference.activate());
        reactor.pump();
        assert_eq!(
            window_probe::activations(reactor.engine().runtime()),
            &[first]
        );
        window_probe::queue_close_request(reactor.engine().runtime(), first);
        reactor.pump();
        assert_eq!(first_close_requests.get(), 1);
        assert_eq!(
            RuntimeProbe::new(reactor.engine().runtime())
                .windows()
                .collect::<Vec<_>>(),
            windows
        );

        window_probe::queue_close_request(reactor.engine().runtime(), first);
        reactor.pump();
        assert_eq!(first_close_requests.get(), 2);

        let reopened = finish_reopened_state.borrow().as_ref().unwrap().clone();
        let reopened_windows: Vec<_> = RuntimeProbe::new(reactor.engine().runtime())
            .windows()
            .collect();
        assert_eq!(reopened_windows.len(), 1);
        let reopened_window = reopened_windows[0];
        assert_eq!(
            window_probe::client_size(reactor.engine().runtime(), reopened_window)?,
            WindowSize {
                width: 640.0,
                height: 480.0,
            }
        );
        assert_eq!(
            window_probe::constraints(reactor.engine().runtime(), reopened_window),
            WindowConstraints {
                min_width: Some(320.0),
                min_height: None,
                max_width: None,
                max_height: Some(800.0),
            }
        );
        assert!(
            finish_reopened_constraints
                .borrow()
                .as_ref()
                .unwrap()
                .try_set(false)
        );
        reactor.pump();
        assert_eq!(
            window_probe::constraints(reactor.engine().runtime(), reopened_window),
            WindowConstraints::default()
        );
        let close_reopened = reopened;
        let mark_survived = Rc::clone(&finish_survived);
        *finish_timer.borrow_mut() =
            Some(TestTimer::one_shot(Duration::from_millis(25), move || {
                mark_survived.set(true);
                close_reopened.set(false);
            })?);
        Ok(())
    })
    .unwrap();
    assert!(
        survived_stale_exit.get(),
        "stale empty-window callback terminated a reopened application"
    );
}

fn focus_fixture() {
    let first_open_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_first_open = Rc::clone(&first_open_state);
    let second_open_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_second_open = Rc::clone(&second_open_state);
    let alternate_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_alternate = Rc::clone(&alternate_state);
    let first_reference = WindowRef::new();
    let render_first_reference = first_reference.clone();
    let root = component(move |cx| {
        let first_open = cx.use_state(|| true);
        let second_open = cx.use_state(|| true);
        let alternate = cx.use_state(|| false);
        publish_first_open.borrow_mut().replace(first_open.clone());
        publish_second_open
            .borrow_mut()
            .replace(second_open.clone());
        publish_alternate.borrow_mut().replace(alternate.clone());
        let mut windows = Vec::new();
        if first_open.try_value().unwrap() {
            windows.push(
                Window::new("Active window", text_block("Active content"), move || {
                    first_open.set(false);
                })
                .reference(&render_first_reference)
                .build()
                .key(1),
            );
        }
        if second_open.try_value().unwrap() {
            let content = if alternate.try_value().unwrap() {
                Button::new("Replacement").on_click(|| {}).build()
            } else {
                text_block("Background content")
            };
            windows.push(
                Window::new("Background window", content, move || {
                    second_open.set(false);
                })
                .build()
                .key(2),
            );
        }
        Application::new(windows).build()
    });

    let preserved = Rc::new(Cell::new(false));
    let finish_preserved = Rc::clone(&preserved);
    let timers = Rc::new(RefCell::new(Vec::<TestTimer>::new()));
    let finish_timers = Rc::clone(&timers);
    let fixture_reference = first_reference;
    run_app_fixture(root, move |reactor| {
        assert!(fixture_reference.activate());
        reactor.pump();
        let active = window_probe::active(reactor.engine().runtime()).unwrap();
        let expected_handle = window_probe::handle(reactor.engine().runtime(), active);

        let alternate = alternate_state.borrow().as_ref().unwrap().clone();
        finish_timers.borrow_mut().push(TestTimer::one_shot(
            Duration::from_millis(25),
            move || {
                alternate.set(true);
            },
        )?);

        let preserved = Rc::clone(&finish_preserved);
        let first_open = first_open_state.borrow().as_ref().unwrap().clone();
        let second_open = second_open_state.borrow().as_ref().unwrap().clone();
        finish_timers.borrow_mut().push(TestTimer::one_shot(
            Duration::from_millis(100),
            move || {
                preserved.set(window_probe::active_handle() == expected_handle);
                first_open.set(false);
                second_open.set(false);
            },
        )?);
        Ok(())
    })
    .unwrap();
    assert!(
        preserved.get(),
        "background content replacement activated its window"
    );
}
