use super::*;
use std::time::Instant;

fn assert_point(actual: [f32; 2], expected: [f32; 2]) {
    for (actual, expected) in actual.into_iter().zip(expected) {
        assert!((actual - expected).abs() < 0.01, "{actual} != {expected}");
    }
}

fn assert_progress(value: f32, from: f32, to: f32, animations: bool) {
    if animations {
        assert!(
            value > from.min(to) + 0.01 && value < from.max(to) - 0.01,
            "{from} -> {value} -> {to}"
        );
    } else {
        assert!((value - to).abs() < 0.001, "{value} != {to}");
    }
}

#[test]
#[ignore = "requires an interactive WinUI desktop"]
fn transitions_use_current_batch_policy_and_retarget_in_flight() {
    if std::env::var_os("REACTOR_TRANSITION_TEST_CHILD").is_none() {
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "native::winui::tests::transitions::transitions_use_current_batch_policy_and_retarget_in_flight",
                "--ignored",
                "--nocapture",
            ])
            .env("REACTOR_TRANSITION_TEST_CHILD", "1")
            .status()
            .unwrap();
        assert!(status.success(), "{status}");
        return;
    }
    let completed = Rc::new(Cell::new(false));
    let finished = completed.clone();
    App::run_with(move |app| {
        let view = |border: Border| {
            Border::new().content(
                border
                    .width(100.0)
                    .height(100.0)
                    .horizontal_alignment(HorizontalAlignment::Left)
                    .vertical_alignment(VerticalAlignment::Top)
                    .background(Color::rgb(70, 130, 200)),
            )
        };
        let mut runtime = Runtime::new(WinUiAdapter::default());
        let animations = runtime.adapter().animations_enabled().unwrap();
        eprintln!("Native animations enabled: {animations}");
        runtime
            .update(view(
                Border::new()
                    .translation(-40.0, -20.0)
                    .translation_transition(Duration::from_secs(3))
                    .scale(1.5)
                    .scale_transition(Duration::from_secs(3))
                    .opacity(0.6)
                    .opacity_transition(Duration::from_secs(3)),
            ))
            .unwrap();
        let root = runtime.graph().root().unwrap();
        let window = runtime
            .adapter_mut()
            .create_window_with_policy(root, &WindowPolicy::new().client_size(600.0, 400.0))
            .unwrap();
        window.activate().unwrap();
        let window = RefCell::new(Some(window));
        let runtime = RefCell::new(Some(runtime));
        let stage = Cell::new(0);
        let started = Cell::new(Instant::now());
        let origin = Cell::new([0.0, 0.0]);
        let mid = Cell::new(0.0);
        let timer = native::DispatcherQueue::GetForCurrentThread()?.CreateTimer()?;
        timer.SetInterval(TimeSpan::try_from(Duration::from_millis(20)).unwrap())?;
        let app = app.clone();
        let tick = timer.Tick(move |sender, _| {
            let wait = if stage.get() == 8 { 3500 } else { 200 };
            if started.get().elapsed() < Duration::from_millis(wait) {
                return;
            }
            let mut holder = runtime.borrow_mut();
            let Some(runtime) = holder.as_mut() else {
                sender.ok().unwrap().Stop().unwrap();
                window.borrow_mut().take().unwrap().close().unwrap();
                finished.set(true);
                app.exit().unwrap();
                return;
            };
            let object = runtime.graph().child(root, RelationId::Content).unwrap();
            let point = runtime
                .adapter()
                .transformed_point(object, 0.0, 0.0)
                .unwrap();
            let offset = [point[0] - origin.get()[0], point[1] - origin.get()[1]];
            let slow = Duration::from_secs(3);
            let next = match stage.get() {
                0 => {
                    assert_eq!(
                        runtime.adapter().translation(object).unwrap(),
                        [-40.0, -20.0]
                    );
                    let right = runtime
                        .adapter()
                        .transformed_point(object, 100.0, 0.0)
                        .unwrap();
                    assert!((right[0] - point[0] - 150.0).abs() < 0.01);
                    origin.set([point[0] + 40.0, point[1] + 20.0]);
                    Border::new()
                        .translation(160.0, 80.0)
                        .translation_transition(slow)
                }
                1 => {
                    assert_progress(offset[0], -40.0, 160.0, animations);
                    assert_progress(offset[1], -20.0, 80.0, animations);
                    mid.set(offset[0]);
                    Border::new()
                        .translation(-80.0, -40.0)
                        .translation_transition(slow)
                }
                2 => {
                    assert_progress(offset[0], mid.get(), -80.0, animations);
                    Border::new()
                        .translation(40.0, 20.0)
                        .translation_transition(Duration::ZERO)
                }
                3 => {
                    assert_point(offset, [40.0, 20.0]);
                    Border::new()
                        .translation(-160.0, -80.0)
                        .translation_transition(slow)
                }
                4 => {
                    assert_progress(offset[0], 40.0, -160.0, animations);
                    Border::new().translation(20.0, 10.0)
                }
                5 => {
                    assert_point(offset, [20.0, 10.0]);
                    // Add a transition while clearing its target: resetting to zero must animate.
                    Border::new().translation_transition(slow)
                }
                6 => {
                    assert_progress(offset[0], 20.0, 0.0, animations);
                    mid.set(offset[0]);
                    Border::new()
                }
                7 => {
                    assert_progress(offset[0], mid.get(), 0.0, animations);
                    Border::new()
                }
                8 => {
                    assert_point(offset, [0.0, 0.0]);
                    assert_eq!(runtime.adapter().translation(object).unwrap(), [0.0, 0.0]);
                    Border::new().scale(1.5).scale_transition(slow)
                }
                9 => {
                    let right = runtime
                        .adapter()
                        .transformed_point(object, 100.0, 0.0)
                        .unwrap();
                    let width = right[0] - point[0];
                    assert_progress(width, 100.0, 150.0, animations);
                    Border::new()
                        .scale(1.0)
                        .scale_transition(Duration::ZERO)
                        .opacity(0.2)
                        .opacity_transition(slow)
                }
                10 => {
                    let right = runtime
                        .adapter()
                        .transformed_point(object, 100.0, 0.0)
                        .unwrap();
                    assert!((right[0] - point[0] - 100.0).abs() < 0.01);
                    let opacity = runtime.adapter().opacity(object).unwrap();
                    // Opacity readback is the target, unlike transformed points during motion.
                    assert!((opacity - 0.2).abs() < 0.001);
                    Border::new()
                        .opacity(0.8)
                        .opacity_transition(Duration::ZERO)
                }
                11 => {
                    assert!((runtime.adapter().opacity(object).unwrap() - 0.8).abs() < 0.001);
                    Border::new()
                        .translation(100.0, 50.0)
                        .translation_transition(slow)
                }
                12 => {
                    assert_progress(offset[0], 0.0, 100.0, animations);
                    Border::new()
                }
                13 => {
                    assert_point(offset, [0.0, 0.0]);
                    runtime
                        .update(Border::new().content(TextBlock::new().text("replacement")))
                        .unwrap();
                    Border::new()
                        .translation(80.0, 40.0)
                        .translation_transition(slow)
                }
                14 => {
                    assert_point(offset, [80.0, 40.0]);
                    runtime.update(Border::new()).unwrap();
                    holder.take();
                    stage.set(15);
                    started.set(Instant::now());
                    return;
                }
                _ => unreachable!(),
            };
            runtime.update(view(next)).unwrap();
            if stage.get() == 13 {
                assert_ne!(
                    runtime.graph().child(root, RelationId::Content),
                    Some(object)
                );
            } else {
                assert_eq!(
                    runtime.graph().child(root, RelationId::Content),
                    Some(object)
                );
            }
            stage.set(stage.get() + 1);
            started.set(Instant::now());
        })?;
        timer.Start()?;
        Ok((timer, tick))
    })
    .unwrap();
    assert!(completed.get());
}
