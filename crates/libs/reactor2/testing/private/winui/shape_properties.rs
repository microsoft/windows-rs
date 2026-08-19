use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use super::shape_access as shape_probe;
use super::*;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_SHAPE_PROPERTIES_FIXTURE";

#[test]
#[ignore = "requires the Windows App Runtime"]
fn shape_properties_update_and_reset() {
    let output = test_reactor_support::run_test_process(
        "winui::tests::shape_properties::shape_properties_fixture",
        &[(FIXTURE_ENV, "run")],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
fn shape_properties_fixture() {
    if std::env::var_os(FIXTURE_ENV).is_none() {
        return;
    }

    bootstrap().unwrap();
    let phase_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_phase_state = Rc::clone(&phase_state);
    let open_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_open_state = Rc::clone(&open_state);
    let root = component(move |cx| {
        let phase = cx.use_state(|| false);
        let open = cx.use_state(|| true);
        publish_phase_state.borrow_mut().replace(phase.clone());
        publish_open_state.borrow_mut().replace(open.clone());
        let close = open.clone();
        Application::new(if open.value() {
            vec![
                Window::new(
                    "Shape properties fixture",
                    content(phase.value()),
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
        assert_phase(reactor.engine().runtime(), false);
        assert!(phase_state.borrow().as_ref().unwrap().try_set(true));
        reactor.pump();
        assert_phase(reactor.engine().runtime(), true);
        assert!(open_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        Ok(())
    })
    .unwrap();
}

fn content(changed: bool) -> Element {
    StackPanel::new([
        if changed {
            Shape::rectangle()
                .stroke(Color::rgb(70, 80, 90))
                .stroke_thickness(2.0)
                .build()
        } else {
            Shape::rectangle()
                .fill(Color::rgb(10, 20, 30))
                .corner_radius(8.0)
                .build()
        },
        Shape::ellipse()
            .fill(if changed {
                Color::rgb(100, 110, 120)
            } else {
                Color::rgb(40, 50, 60)
            })
            .build(),
        Shape::line(
            if changed { 1.0 } else { 0.0 },
            if changed { 2.0 } else { 0.0 },
            if changed { 30.0 } else { 20.0 },
            if changed { 4.0 } else { 0.0 },
        )
        .stroke(Color::rgb(130, 140, 150))
        .stroke_thickness(if changed { 4.0 } else { 3.0 })
        .build(),
    ])
    .build()
}

fn assert_phase(runtime: &WinUiRuntime, changed: bool) {
    let probe = RuntimeProbe::new(runtime);
    let rectangle = probe.nodes(NativeKind::Rectangle);
    let ellipse = probe.nodes(NativeKind::Ellipse);
    let line = probe.nodes(NativeKind::Line);
    assert_eq!((rectangle.len(), ellipse.len(), line.len()), (1, 1, 1));
    assert_eq!(
        shape_probe::shape_properties(runtime, rectangle[0]).unwrap(),
        if changed {
            (None, Some((255, 70, 80, 90)), 2.0)
        } else {
            (Some((255, 10, 20, 30)), None, 1.0)
        }
    );
    assert_eq!(
        shape_probe::rectangle_radius(runtime, rectangle[0]).unwrap(),
        if changed { (0.0, 0.0) } else { (8.0, 8.0) }
    );
    assert_eq!(
        shape_probe::shape_properties(runtime, ellipse[0]).unwrap(),
        (
            Some(if changed {
                (255, 100, 110, 120)
            } else {
                (255, 40, 50, 60)
            }),
            None,
            1.0,
        )
    );
    assert_eq!(
        shape_probe::shape_properties(runtime, line[0]).unwrap(),
        (
            None,
            Some((255, 130, 140, 150)),
            if changed { 4.0 } else { 3.0 }
        )
    );
    assert_eq!(
        shape_probe::line_points(runtime, line[0]).unwrap(),
        if changed {
            [1.0, 2.0, 30.0, 4.0]
        } else {
            [0.0, 0.0, 20.0, 0.0]
        }
    );
}
