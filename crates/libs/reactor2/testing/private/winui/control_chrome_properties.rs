use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use super::control_chrome_access as chrome_probe;
use super::*;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_CONTROL_CHROME_PROPERTIES_FIXTURE";

#[test]
#[ignore = "requires the Windows App Runtime"]
fn text_box_chrome_updates_and_resets_to_style() {
    let output = test_reactor_support::run_test_process(
        "winui::tests::control_chrome_properties::control_chrome_properties_fixture",
        &[(FIXTURE_ENV, "run")],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
fn control_chrome_properties_fixture() {
    if std::env::var_os(FIXTURE_ENV).is_none() {
        return;
    }

    bootstrap().unwrap();
    let phase_state = Rc::new(RefCell::new(None::<State<usize>>));
    let publish_phase_state = Rc::clone(&phase_state);
    let open_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_open_state = Rc::clone(&open_state);
    let root = component(move |cx| {
        let phase = cx.use_state(|| 0_usize);
        let open = cx.use_state(|| true);
        publish_phase_state.borrow_mut().replace(phase.clone());
        publish_open_state.borrow_mut().replace(open.clone());
        let text_box = if phase.value() == 1 {
            TextBox::display("")
                .background(Color::argb(0, 0, 0, 0))
                .border_brush(Color::rgb(60, 120, 220))
                .border_thickness(Thickness::uniform(2.0))
                .build()
        } else {
            TextBox::display("").build()
        };
        let close = open.clone();
        Application::new(if open.value() {
            vec![
                Window::new("Control chrome fixture", text_box, move || {
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
        let text_box = RuntimeProbe::new(reactor.engine().runtime()).nodes(NativeKind::TextBox)[0];
        let baseline = chrome_probe::control_chrome(reactor.engine().runtime(), text_box)?;
        assert!(phase_state.borrow().as_ref().unwrap().try_set(1));
        reactor.pump();
        assert_eq!(
            chrome_probe::control_chrome(reactor.engine().runtime(), text_box)?,
            (
                Some((0, 0, 0, 0)),
                Some((255, 60, 120, 220)),
                (2.0, 2.0, 2.0, 2.0),
            )
        );
        assert!(phase_state.borrow().as_ref().unwrap().try_set(2));
        reactor.pump();
        assert_eq!(
            chrome_probe::control_chrome(reactor.engine().runtime(), text_box)?,
            baseline
        );
        assert!(open_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        Ok(())
    })
    .unwrap();
}
