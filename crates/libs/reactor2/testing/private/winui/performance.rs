use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use super::*;
use crate::performance::{
    DispatcherTimer, HostOptions, RenderingSubscription, request_exit, run_host,
};

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_PERFORMANCE_FIXTURE";

fn run_case(case: &str) {
    let output = test_reactor_support::run_test_process(
        "winui::tests::performance::performance_fixture",
        &[(FIXTURE_ENV, case)],
        Duration::from_secs(30),
    )
    .unwrap();

    test_reactor_support::assert_success(output);
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn render_metrics_report_native_updates() {
    run_case("metrics");
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn dispatcher_and_composition_callbacks_run() {
    run_case("callbacks");
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn performance_host_reports_rendering() {
    run_case("host");
}

#[test]
fn performance_fixture() {
    let Some(case) = std::env::var_os(FIXTURE_ENV) else {
        return;
    };

    match case.to_str().unwrap() {
        "metrics" => metrics_fixture(),
        "callbacks" => callbacks_fixture(),
        "host" => host_fixture(),
        case => panic!("unknown performance fixture: {case}"),
    }
}

fn metrics_fixture() {
    bootstrap().unwrap();
    let value = Rc::new(RefCell::new(None::<State<usize>>));
    let publish_value = Rc::clone(&value);
    let root = component(move |cx| {
        let value = cx.use_state(|| 0usize);
        publish_value.borrow_mut().replace(value.clone());
        StackPanel::new([
            TextBlock::new(format!("value {}", value.try_value().unwrap())).build(),
            text_block("stable"),
        ])
        .build()
    });
    let metrics = Rc::new(RefCell::new(Vec::new()));
    let callback_metrics = Rc::clone(&metrics);
    let shutdown = Rc::new(RefCell::new(None::<TestTimer>));
    let callback_shutdown = Rc::clone(&shutdown);

    host::run_reactor_winui_async_fixture(
        "windows-reactor render metrics fixture",
        root,
        move |reactor, finish| {
            reactor.set_render_complete(move |value| {
                callback_metrics.borrow_mut().push(*value);
            });
            assert!(value.borrow().as_ref().unwrap().try_set(1));
            reactor.pump();
            assert!(value.borrow().as_ref().unwrap().try_set(2));
            reactor.pump();
            let timer_shutdown = Rc::clone(&callback_shutdown);
            *callback_shutdown.borrow_mut() =
                Some(TestTimer::one_shot(Duration::from_millis(1), move || {
                    timer_shutdown.borrow_mut().take();
                    finish(Ok(()));
                })?);
            Ok(())
        },
    )
    .unwrap();

    let metrics = metrics.borrow();
    assert_eq!(metrics.len(), 2);
    assert!(metrics.iter().all(|value| value.tree_build_ms >= 0.0));
    assert!(metrics.iter().all(|value| value.reconcile_ms >= 0.0));
    assert!(metrics.iter().all(|value| value.effects_ms >= 0.0));
    assert!(metrics.iter().all(|value| value.elements_diffed > 0));
    assert!(metrics.iter().all(|value| value.elements_created == 0));
}

fn callbacks_fixture() {
    bootstrap().unwrap();
    let ticks = Rc::new(Cell::new(0));
    let frames = Rc::new(Cell::new(0));
    let rendering = Rc::new(RefCell::new(None::<RenderingSubscription>));
    let repeating = Rc::new(RefCell::new(None::<DispatcherTimer>));
    let one_shot = Rc::new(RefCell::new(None::<DispatcherTimer>));

    host::run_reactor_winui_async_fixture(
        "windows-reactor callback fixture",
        text_block("callbacks"),
        move |_reactor, finish| {
            let callback_frames = Rc::clone(&frames);
            *rendering.borrow_mut() = Some(RenderingSubscription::new(move || {
                callback_frames.set(callback_frames.get() + 1);
            })?);
            let callback_ticks = Rc::clone(&ticks);
            let callback_repeating = Rc::clone(&repeating);
            let callback_one_shot = Rc::clone(&one_shot);
            let callback_rendering = Rc::clone(&rendering);
            let callback_frames = Rc::clone(&frames);
            let callback_finish = Rc::clone(&finish);
            *repeating.borrow_mut() = Some(DispatcherTimer::repeating(
                Duration::from_millis(1),
                move || {
                    let count = callback_ticks.get() + 1;
                    callback_ticks.set(count);
                    if count != 3 {
                        return;
                    }

                    callback_repeating
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .stop()
                        .unwrap();
                    let final_ticks = Rc::clone(&callback_ticks);
                    let final_repeating = Rc::clone(&callback_repeating);
                    let final_one_shot = Rc::clone(&callback_one_shot);
                    let final_rendering = Rc::clone(&callback_rendering);
                    let final_frames = Rc::clone(&callback_frames);
                    let final_finish = Rc::clone(&callback_finish);
                    match DispatcherTimer::one_shot(Duration::from_millis(25), move || {
                        final_repeating.borrow_mut().take();
                        final_one_shot.borrow_mut().take();
                        final_rendering.borrow_mut().take();
                        assert_eq!(final_ticks.get(), 3);
                        assert!(final_frames.get() > 0);
                        final_finish(Ok(()));
                    }) {
                        Ok(timer) => *callback_one_shot.borrow_mut() = Some(timer),
                        Err(error) => callback_finish(Err(error)),
                    }
                },
            )?);
            Ok(())
        },
    )
    .unwrap();
}

fn host_fixture() {
    bootstrap().unwrap();
    let renders = Rc::new(Cell::new(0));
    let callback_renders = Rc::clone(&renders);
    let root = component(move |cx| {
        let shutdown = cx.use_ref(|| None::<DispatcherTimer>);
        cx.use_effect((), move || {
            shutdown.set(Some(
                DispatcherTimer::one_shot(Duration::from_millis(25), || {
                    request_exit().unwrap();
                })
                .unwrap(),
            ));
        });
        text_block("performance host")
    });

    run_host(
        "windows-reactor performance host fixture",
        root,
        HostOptions { fullscreen: true },
        move |_| callback_renders.set(callback_renders.get() + 1),
    )
    .unwrap();
    assert!(renders.get() > 0);
}
