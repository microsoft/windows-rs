use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use super::framework_access as framework_probe;
use super::*;
use crate::winui::controlled::tests as controlled_probe;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_CONTROL_PROPERTIES_FIXTURE";

#[test]
#[ignore = "requires the Windows App Runtime"]
fn enabled_toggle_and_text_properties_update_without_callbacks() {
    let output = test_reactor_support::run_test_process(
        "winui::tests::control_properties::control_properties_fixture",
        &[(FIXTURE_ENV, "run")],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
fn control_properties_fixture() {
    if std::env::var_os(FIXTURE_ENV).is_none() {
        return;
    }

    bootstrap().unwrap();
    let phase_state = Rc::new(RefCell::new(None::<State<usize>>));
    let publish_phase_state = Rc::clone(&phase_state);
    let open_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_open_state = Rc::clone(&open_state);
    let callbacks = Rc::new(Cell::new(0usize));
    let callbacks_for_render = Rc::clone(&callbacks);
    let root = component(move |cx| {
        let phase = cx.use_state(|| 0usize);
        let open = cx.use_state(|| true);
        publish_phase_state.borrow_mut().replace(phase.clone());
        publish_open_state.borrow_mut().replace(open.clone());
        let close = open.clone();
        Application::new(if open.try_value().unwrap() {
            vec![
                Window::new(
                    "Control properties fixture",
                    content(phase.try_value().unwrap(), &callbacks_for_render),
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
        assert_eq!(callbacks.get(), 0);
        assert!(phase_state.borrow().as_ref().unwrap().try_set(1));
        reactor.pump();
        assert_phase(reactor.engine().runtime(), 1);
        assert_eq!(callbacks.get(), 0);
        assert!(phase_state.borrow().as_ref().unwrap().try_set(2));
        reactor.pump();
        assert_phase(reactor.engine().runtime(), 2);
        assert_eq!(callbacks.get(), 0);
        assert!(open_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        Ok(())
    })
    .unwrap();
}

fn content(phase: usize, callbacks: &Rc<Cell<usize>>) -> Element {
    let enabled = phase != 1;
    let checked = phase == 1;
    let text = match phase {
        0 => "initial",
        1 => "updated",
        _ => "",
    };
    let callback = |callbacks: &Rc<Cell<usize>>| {
        let callbacks = Rc::clone(callbacks);
        move || callbacks.set(callbacks.get() + 1)
    };
    let bool_callback = |callbacks: &Rc<Cell<usize>>| {
        let callbacks = Rc::clone(callbacks);
        move |_| callbacks.set(callbacks.get() + 1)
    };
    let text_callback = |callbacks: &Rc<Cell<usize>>| {
        let callbacks = Rc::clone(callbacks);
        move |_| callbacks.set(callbacks.get() + 1)
    };

    StackPanel::new([
        Button::new("button")
            .on_click(callback(callbacks))
            .enabled(enabled)
            .build(),
        CheckBox::new("check box", checked, bool_callback(callbacks))
            .enabled(enabled)
            .build(),
        ToggleButton::new("toggle button", checked, bool_callback(callbacks))
            .enabled(enabled)
            .build(),
        ToggleSwitch::new(checked, bool_callback(callbacks))
            .enabled(enabled)
            .build(),
        TextBox::new(text, text_callback(callbacks))
            .enabled(enabled)
            .build(),
        PasswordBox::new(text, text_callback(callbacks))
            .enabled(enabled)
            .build(),
    ])
    .build()
}

fn only_node(runtime: &WinUiRuntime, kind: NativeKind) -> NodeId {
    let nodes = RuntimeProbe::new(runtime).nodes(kind);
    assert_eq!(nodes.len(), 1, "expected one {kind:?} node");
    nodes[0]
}

fn assert_phase(runtime: &WinUiRuntime, phase: usize) {
    let enabled = phase != 1;
    let checked = phase == 1;
    let text = match phase {
        0 => "initial",
        1 => "updated",
        _ => "",
    };
    let button = only_node(runtime, NativeKind::Button);
    let check_box = only_node(runtime, NativeKind::CheckBox);
    let toggle_button = only_node(runtime, NativeKind::ToggleButton);
    let toggle_switch = only_node(runtime, NativeKind::ToggleSwitch);
    let text_box = only_node(runtime, NativeKind::TextBox);
    let password_box = only_node(runtime, NativeKind::PasswordBox);

    for id in [
        button,
        check_box,
        toggle_button,
        toggle_switch,
        text_box,
        password_box,
    ] {
        assert_eq!(framework_probe::enabled(runtime, id).unwrap(), enabled);
    }
    assert_eq!(
        controlled_probe::checked(runtime, check_box).unwrap(),
        checked
    );
    assert_eq!(
        controlled_probe::checked(runtime, toggle_button).unwrap(),
        checked
    );
    assert_eq!(
        controlled_probe::on(runtime, toggle_switch).unwrap(),
        checked
    );
    assert_eq!(controlled_probe::text(runtime, text_box).unwrap(), text);
    assert_eq!(
        controlled_probe::password(runtime, password_box).unwrap(),
        text
    );
}
