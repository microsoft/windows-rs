use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use super::time_picker_access as time_probe;
use super::*;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_TIME_PICKER_PROPERTIES_FIXTURE";

#[test]
#[ignore = "requires the Windows App Runtime"]
fn time_picker_updates_and_dispatches_native_changes() {
    let output = test_reactor_support::run_test_process(
        "winui::tests::time_picker_properties::time_picker_properties_fixture",
        &[(FIXTURE_ENV, "run")],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
fn time_picker_properties_fixture() {
    if std::env::var_os(FIXTURE_ENV).is_none() {
        return;
    }

    bootstrap().unwrap();
    let observed = Rc::new(RefCell::new(Vec::<Option<TimeSpan>>::new()));
    let observed_for_render = Rc::clone(&observed);
    let phase_state = Rc::new(RefCell::new(None::<State<usize>>));
    let publish_phase_state = Rc::clone(&phase_state);
    let open_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_open_state = Rc::clone(&open_state);
    let root = component(move |cx| {
        let phase = cx.use_state(|| 0_usize);
        let open = cx.use_state(|| true);
        publish_phase_state.borrow_mut().replace(phase.clone());
        publish_open_state.borrow_mut().replace(open.clone());
        let observed = Rc::clone(&observed_for_render);
        let picker = TimePicker::new(
            (phase.value() == 0).then_some(TimeSpan::from_hours(9)),
            move |value| {
                observed.borrow_mut().push(value);
            },
        )
        .minute_increment(if phase.value() == 0 { 15 } else { 30 });
        let picker = if phase.value() == 0 {
            picker.header("Pick a time").build()
        } else {
            picker.build()
        };
        let close = open.clone();
        Application::new(if open.value() {
            vec![
                Window::new("TimePicker fixture", picker, move || {
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
        let picker = RuntimeProbe::new(reactor.engine().runtime()).nodes(NativeKind::TimePicker)[0];
        assert_eq!(
            time_probe::time_picker_properties(reactor.engine().runtime(), picker)?,
            (Some(TimeSpan::from_hours(9)), 15)
        );
        let selected = TimeSpan::from_hours(13) + TimeSpan::from_minutes(45);
        time_probe::set_time_picker_time(reactor.engine().runtime(), picker, Some(selected))?;
        reactor.pump();
        assert!(observed.borrow().last().copied().flatten().is_some());
        assert!(phase_state.borrow().as_ref().unwrap().try_set(1));
        reactor.pump();
        assert_eq!(
            RuntimeProbe::new(reactor.engine().runtime()).nodes(NativeKind::TimePicker),
            [picker]
        );
        assert_eq!(
            time_probe::time_picker_properties(reactor.engine().runtime(), picker)?,
            (None, 30)
        );
        assert!(open_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        Ok(())
    })
    .unwrap();
}
