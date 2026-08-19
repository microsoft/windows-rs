use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::swap_chain_canvas;

use super::*;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_CANVAS_FIXTURE";

impl WinUiRuntime {
    fn force_canvas_present_loss(&mut self) {
        self.canvas_test_present_loss = true;
    }

    fn force_canvas_scale(&mut self, scale_x: f32, scale_y: f32) {
        self.canvas_test_scale = Some((scale_x, scale_y));
    }
}

fn run_case(case: &str) {
    let output = test_reactor_support::run_test_process(
        "winui::tests::canvas::canvas_fixture",
        &[(FIXTURE_ENV, case)],
        Duration::from_secs(30),
    )
    .unwrap();

    test_reactor_support::assert_success(output);
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn scale_and_present_loss_rebuild_the_surface() {
    run_case("recovery");
}

#[test]
fn canvas_fixture() {
    let Some(case) = std::env::var_os(FIXTURE_ENV) else {
        return;
    };

    match case.to_str().unwrap() {
        "recovery" => recovery_fixture(),
        case => panic!("unknown canvas fixture: {case}"),
    }
}

fn recovery_fixture() {
    bootstrap().unwrap();
    let draws = Rc::new(RefCell::new(Vec::new()));
    let record_draws = Rc::clone(&draws);
    let root = swap_chain_canvas(move |context| {
        record_draws.borrow_mut().push((
            context.scale_x,
            context.scale_y,
            context.device_changed(),
            context.surface_changed(),
        ));
        Ok(())
    })
    .width(240.0)
    .height(120.0)
    .build();
    let poll = Rc::new(RefCell::new(None::<TestTimer>));
    let ready_poll = Rc::clone(&poll);
    let ready_draws = Rc::clone(&draws);

    host::run_reactor_winui_configured_async_fixture(
        "windows-reactor canvas recovery fixture",
        root,
        |reactor| {
            reactor.engine.runtime.force_canvas_scale(1.25, 1.25);
            reactor.engine.runtime.force_canvas_present_loss();
            Ok(())
        },
        move |_reactor, finish| {
            let callback_poll = Rc::clone(&ready_poll);
            *ready_poll.borrow_mut() =
                Some(TestTimer::repeating(Duration::from_millis(5), move || {
                    if ready_draws.borrow().len() >= 2 {
                        callback_poll.borrow_mut().take();
                        finish(Ok(()));
                    }
                })?);
            Ok(())
        },
    )
    .unwrap();

    let draws = draws.borrow();
    assert!(draws.len() >= 2, "expected recovery draw, got {draws:?}");
    assert!(draws.iter().all(|draw| draw.0 == 1.25 && draw.1 == 1.25));
    assert!(draws[0].2 && draws[0].3);
    assert!(draws[1].2 && draws[1].3);
}
