use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use super::*;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_PANIC_FIXTURE";

fn assert_panics(case: &str, message: &str) {
    let output = test_reactor_support::run_test_process(
        "winui::tests::faults::panic_fixture",
        &[(FIXTURE_ENV, case)],
        Duration::from_secs(30),
    )
    .unwrap();

    assert!(
        !output.status.success(),
        "panic fixture unexpectedly passed"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(message),
        "panic fixture did not report {message:?}\nstderr:\n{stderr}"
    );
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn launch_callback_panic_is_reported() {
    assert_panics("launch", "injected fixture panic");
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn dispatcher_callback_panic_is_reported() {
    assert_panics("timer", "injected timer panic");
}

#[test]
fn panic_fixture() {
    let Some(case) = std::env::var_os(FIXTURE_ENV) else {
        return;
    };

    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        default_hook(info);
        std::process::exit(101);
    }));
    bootstrap().unwrap();
    let root = Application::new([Window::new(
        "windows-reactor panic fixture",
        text_block("panic"),
        || {},
    )
    .build()])
    .build();
    match case.to_str().unwrap() {
        "launch" => {
            run_app_fixture(root, |_| panic!("injected fixture panic")).unwrap();
        }
        "timer" => {
            run_app_fixture(root, |_reactor| {
                let timer = Rc::new(RefCell::new(None::<TestTimer>));
                let timer_for_tick = Rc::clone(&timer);
                *timer.borrow_mut() =
                    Some(TestTimer::one_shot(Duration::from_millis(1), move || {
                        _ = &timer_for_tick;
                        panic!("injected timer panic");
                    })?);
                Ok(())
            })
            .unwrap();
        }
        case => panic!("unknown panic fixture: {case}"),
    }
}
