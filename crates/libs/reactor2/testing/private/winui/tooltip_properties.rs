use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use super::tooltip_access as tooltip_probe;
use super::*;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_TOOLTIP_PROPERTIES_FIXTURE";

#[test]
#[ignore = "requires the Windows App Runtime"]
fn tooltip_placement_updates_and_resets() {
    let output = test_reactor_support::run_test_process(
        "winui::tests::tooltip_properties::tooltip_properties_fixture",
        &[(FIXTURE_ENV, "run")],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
fn tooltip_properties_fixture() {
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
        let owner = Button::new("Owner").build();
        let owner = match phase.value() {
            1 => owner.tooltip_with(Tooltip::text("Tip").placement(TooltipPlacement::Top)),
            2 => owner.tooltip_with(Tooltip::text("Tip").placement(TooltipPlacement::Bottom)),
            3 => owner.tooltip_with(Tooltip::text("Tip").placement(TooltipPlacement::Left)),
            4 => owner.tooltip_with(Tooltip::text("Tip").placement(TooltipPlacement::Right)),
            5 => owner.tooltip_with(Tooltip::text("Tip").placement(TooltipPlacement::Mouse)),
            _ => owner.tooltip(TextBlock::new("Tip").build()),
        };
        let close = open.clone();
        Application::new(if open.value() {
            vec![
                Window::new("Tooltip fixture", owner, move || {
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
        let owner = RuntimeProbe::new(reactor.engine().runtime()).nodes(NativeKind::Button)[0];
        let baseline = tooltip_probe::tooltip_placement(reactor.engine().runtime(), owner)?;
        for (phase, expected) in [
            (1, TooltipPlacement::Top),
            (2, TooltipPlacement::Bottom),
            (3, TooltipPlacement::Left),
            (4, TooltipPlacement::Right),
            (5, TooltipPlacement::Mouse),
        ] {
            assert!(phase_state.borrow().as_ref().unwrap().try_set(phase));
            reactor.pump();
            assert_eq!(
                RuntimeProbe::new(reactor.engine().runtime()).nodes(NativeKind::Button),
                [owner]
            );
            assert_eq!(
                tooltip_probe::tooltip_placement(reactor.engine().runtime(), owner)?,
                expected
            );
        }
        assert!(phase_state.borrow().as_ref().unwrap().try_set(6));
        reactor.pump();
        assert_eq!(
            tooltip_probe::tooltip_placement(reactor.engine().runtime(), owner)?,
            baseline
        );
        assert!(open_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        Ok(())
    })
    .unwrap();
}
