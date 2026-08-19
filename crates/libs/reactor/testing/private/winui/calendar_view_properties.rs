use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use super::calendar_view_access as calendar_probe;
use super::*;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_CALENDAR_VIEW_PROPERTIES_FIXTURE";

#[test]
#[ignore = "requires the Windows App Runtime"]
fn calendar_view_updates_and_dispatches_native_changes() {
    let output = test_reactor_support::run_test_process(
        "winui::tests::calendar_view_properties::calendar_view_properties_fixture",
        &[(FIXTURE_ENV, "run")],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
fn calendar_view_properties_fixture() {
    if std::env::var_os(FIXTURE_ENV).is_none() {
        return;
    }

    bootstrap().unwrap();
    let first = DateTime::from_unix_secs(1_704_067_200);
    let second = DateTime::from_unix_secs(1_704_153_600);
    let changed = DateTime::from_unix_secs(1_704_240_000);
    let observed = Rc::new(RefCell::new(Vec::<Vec<DateTime>>::new()));
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
        let (dates, mode) = match phase.value() {
            0 => (vec![second, first], CalendarSelectionMode::Multiple),
            1 => (vec![second], CalendarSelectionMode::Single),
            _ => (Vec::new(), CalendarSelectionMode::None),
        };
        let calendar = CalendarView::new(dates, move |value| {
            observed.borrow_mut().push(value);
        })
        .selection_mode(mode)
        .today_highlighted(phase.value() != 0)
        .group_label_visible(phase.value() == 0)
        .build();
        let close = open.clone();
        Application::new(if open.value() {
            vec![
                Window::new("CalendarView fixture", calendar, move || {
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
        let calendar =
            RuntimeProbe::new(reactor.engine().runtime()).nodes(NativeKind::CalendarView)[0];
        assert_eq!(
            calendar_probe::calendar_view_properties(reactor.engine().runtime(), calendar)?,
            (
                vec![first, second],
                CalendarSelectionMode::Multiple,
                false,
                true
            )
        );
        calendar_probe::set_calendar_view_dates(reactor.engine().runtime(), calendar, &[changed])?;
        reactor.pump();
        assert_eq!(observed.borrow().last(), Some(&vec![changed]));

        assert!(phase_state.borrow().as_ref().unwrap().try_set(1));
        reactor.pump();
        assert_eq!(
            RuntimeProbe::new(reactor.engine().runtime()).nodes(NativeKind::CalendarView),
            [calendar]
        );
        assert_eq!(
            calendar_probe::calendar_view_properties(reactor.engine().runtime(), calendar)?,
            (vec![second], CalendarSelectionMode::Single, true, false)
        );

        assert!(phase_state.borrow().as_ref().unwrap().try_set(2));
        reactor.pump();
        assert_eq!(
            calendar_probe::calendar_view_properties(reactor.engine().runtime(), calendar)?,
            (Vec::new(), CalendarSelectionMode::None, true, false)
        );
        assert!(open_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        Ok(())
    })
    .unwrap();
}
