use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use super::super::command::tests as command_probe;
use super::*;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_COMMAND_PROPERTIES_FIXTURE";

#[test]
#[ignore = "requires the Windows App Runtime"]
fn command_bar_items_update_and_reset() {
    let output = test_reactor_support::run_test_process(
        "winui::tests::command_properties::command_properties_fixture",
        &[(FIXTURE_ENV, "run")],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
fn command_properties_fixture() {
    if std::env::var_os(FIXTURE_ENV).is_none() {
        return;
    }

    bootstrap().unwrap();
    let phase_state = Rc::new(RefCell::new(None::<State<usize>>));
    let publish_phase_state = Rc::clone(&phase_state);
    let open_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_open_state = Rc::clone(&open_state);
    let root = component(move |cx| {
        let phase = cx.use_state(|| 0usize);
        let open = cx.use_state(|| true);
        publish_phase_state.borrow_mut().replace(phase.clone());
        publish_open_state.borrow_mut().replace(open.clone());
        let close = open.clone();
        Application::new(if open.try_value().unwrap() {
            vec![
                Window::new(
                    "Command properties fixture",
                    content(phase.try_value().unwrap()),
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
        assert_phase(reactor.engine().runtime(), 0);
        assert!(phase_state.borrow().as_ref().unwrap().try_set(1));
        reactor.pump();
        assert_phase(reactor.engine().runtime(), 1);
        assert!(phase_state.borrow().as_ref().unwrap().try_set(2));
        reactor.pump();
        assert_phase(reactor.engine().runtime(), 2);
        assert!(open_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        Ok(())
    })
    .unwrap();
}

fn content(phase: usize) -> Element {
    let (position, button_label, button_enabled, button_icon) = match phase {
        0 => (
            CommandBarDefaultLabelPosition::Bottom,
            "Open",
            true,
            Some(Icon::symbol(IconSymbol::ADD)),
        ),
        1 => (
            CommandBarDefaultLabelPosition::Right,
            "Save",
            false,
            Some(Icon::symbol(IconSymbol::SAVE)),
        ),
        _ => (
            CommandBarDefaultLabelPosition::Collapsed,
            "Reset",
            true,
            None,
        ),
    };
    let (toggle_label, toggle_enabled, toggle_checked, toggle_icon) = match phase {
        1 => (
            "Pinned",
            false,
            true,
            Some(Icon::symbol(IconSymbol::FAVORITE)),
        ),
        _ => ("Pin", true, false, None),
    };
    CommandBar::new([
        CommandBarItem::button(1, button_label, || {})
            .enabled(button_enabled)
            .icon(button_icon),
        CommandBarItem::toggle(2, toggle_label, toggle_checked, |_| {})
            .enabled(toggle_enabled)
            .icon(toggle_icon),
        CommandBarItem::separator(3),
    ])
    .secondary_commands([CommandBarItem::button(4, "Settings", || {})])
    .default_label_position(position)
    .build()
}

fn assert_phase(runtime: &WinUiRuntime, phase: usize) {
    let probe = RuntimeProbe::new(runtime);
    let bar = only_node(&probe, NativeKind::CommandBar);
    let toggle = only_node(&probe, NativeKind::AppBarToggleButton);
    assert_eq!(command_probe::command_bar(runtime, bar).unwrap(), (3, 1));

    let buttons = probe.nodes(NativeKind::AppBarButton);
    assert_eq!(buttons.len(), 2);
    let values = buttons
        .into_iter()
        .map(|id| command_probe::app_bar_button(runtime, id).unwrap())
        .collect::<Vec<_>>();
    let expected_button = match phase {
        0 => (true, true),
        1 => (false, true),
        _ => (true, false),
    };
    assert_eq!(values, [expected_button, (true, false)]);

    assert_eq!(
        command_probe::app_bar_toggle_button(runtime, toggle).unwrap(),
        match phase {
            1 => (false, true, true),
            _ => (true, false, false),
        }
    );
}

fn only_node(probe: &RuntimeProbe<'_>, kind: NativeKind) -> NodeId {
    let nodes = probe.nodes(kind);
    assert_eq!(nodes.len(), 1, "expected one {kind:?} node");
    nodes[0]
}
