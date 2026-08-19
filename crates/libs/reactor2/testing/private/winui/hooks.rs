use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use super::*;
use crate::winui::lifecycle::tests as lifecycle_probe;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_HOOK_FIXTURE";

fn run_case(case: &str) {
    let output = test_reactor_support::run_test_process(
        "winui::tests::hooks::hook_fixture",
        &[(FIXTURE_ENV, case)],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn timeout_and_interval_lifecycle() {
    run_case("timers");
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn resource_completion_wakes_reactor() {
    run_case("resource");
}

#[test]
fn hook_fixture() {
    let Some(case) = std::env::var_os(FIXTURE_ENV) else {
        return;
    };

    bootstrap().unwrap();
    match case.to_str().unwrap() {
        "timers" => timers_fixture(),
        "resource" => resource_fixture(),
        case => panic!("unknown hook fixture: {case}"),
    }
}

fn timers_fixture() {
    let interval_fires = Rc::new(Cell::new(0));
    let timeout_fires = Rc::new(Cell::new(0));
    let render_interval_fires = Rc::clone(&interval_fires);
    let render_timeout_fires = Rc::clone(&timeout_fires);
    let content = component(move |cx| {
        let visible = cx.use_state(|| true);
        let timeout_fires = Rc::clone(&render_timeout_fires);
        cx.use_timeout((), Duration::from_millis(5), move || {
            timeout_fires.set(timeout_fires.get() + 1);
        });
        if visible.try_value().unwrap() {
            let interval_fires = Rc::clone(&render_interval_fires);
            component(move |cx| {
                let visible = visible.clone();
                let interval_fires = Rc::clone(&interval_fires);
                cx.use_interval((), Duration::from_millis(10), move || {
                    let count = interval_fires.get() + 1;
                    interval_fires.set(count);
                    if count == 2 {
                        visible.set(false);
                    }
                });
                text_block("timers")
            })
        } else {
            text_block("timers complete")
        }
    });
    let root =
        Application::new([
            Window::new("windows-reactor hook timers fixture", content, || {}).build(),
        ])
        .build();
    let outcome = Rc::new(RefCell::new(None::<Result<(), String>>));
    let finish_outcome = Rc::clone(&outcome);
    let watchdog = Rc::new(RefCell::new(None::<TestTimer>));
    let finish_watchdog = Rc::clone(&watchdog);

    run_app_fixture(root, move |reactor| {
        assert_eq!(lifecycle_probe::timer_count(reactor.engine().runtime()), 2);
        let timer_ticks = lifecycle_probe::timer_ticks(reactor.engine().runtime());
        let interval_fires = Rc::clone(&interval_fires);
        let timeout_fires = Rc::clone(&timeout_fires);
        *finish_watchdog.borrow_mut() = Some(TestTimer::one_shot(
            Duration::from_millis(500),
            move || {
                let result = if timer_ticks.get() < 3 {
                    Err("native hook timers did not produce three ticks".to_string())
                } else if timeout_fires.get() != 1 {
                    Err(format!(
                        "hook timeout fired {} times instead of once",
                        timeout_fires.get()
                    ))
                } else if interval_fires.get() != 2 {
                    Err(format!(
                        "hook interval fired {} times instead of twice",
                        interval_fires.get()
                    ))
                } else {
                    Ok(())
                };
                *finish_outcome.borrow_mut() = Some(result);
                terminate_host();
            },
        )?);
        Ok(())
    })
    .unwrap();

    if let Err(error) = outcome.borrow_mut().take().unwrap() {
        panic!("{error}");
    }
}

fn resource_fixture() {
    let ready = Rc::new(Cell::new(false));
    let ready_for_render = Rc::clone(&ready);
    let content = component(move |cx| {
        match cx.use_resource((), |_cancel, ()| {
            std::thread::sleep(Duration::from_millis(25));
            Ok(42usize)
        }) {
            Resource::Loading => text_block("resource loading"),
            Resource::Ready(value) => {
                ready_for_render.set(*value == 42);
                text_block("resource ready")
            }
            Resource::Failed(error) => panic!("resource failed: {error}"),
        }
    });
    let root =
        Application::new([
            Window::new("windows-reactor hook resource fixture", content, || {}).build(),
        ])
        .build();
    let outcome = Rc::new(Cell::new(None::<bool>));
    let finish_outcome = Rc::clone(&outcome);
    let watchdog = Rc::new(RefCell::new(None::<TestTimer>));
    let finish_watchdog = Rc::clone(&watchdog);

    run_app_fixture(root, move |_reactor| {
        let ready = Rc::clone(&ready);
        *finish_watchdog.borrow_mut() = Some(TestTimer::one_shot(
            Duration::from_millis(500),
            move || {
                finish_outcome.set(Some(ready.get()));
                terminate_host();
            },
        )?);
        Ok(())
    })
    .unwrap();

    assert!(
        outcome.get().unwrap(),
        "resource completion did not wake Reactor"
    );
}
