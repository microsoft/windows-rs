use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use super::content_access as content_probe;
use super::*;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_CONTENT_PROPERTIES_FIXTURE";

#[test]
#[ignore = "requires the Windows App Runtime"]
fn tooltip_hyperlink_and_repeat_button_properties_update_and_reset() {
    let output = test_reactor_support::run_test_process(
        "winui::tests::content_properties::content_properties_fixture",
        &[(FIXTURE_ENV, "run")],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
fn content_properties_fixture() {
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
                    "Content properties fixture",
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
    let repeat = match phase {
        1 => RepeatButton::new("repeat")
            .on_click(|| {})
            .delay(750)
            .interval(50)
            .build(),
        _ => RepeatButton::new("repeat").on_click(|| {}).build(),
    };
    let hyperlink = match phase {
        0 => HyperlinkButton::new("hyperlink")
            .on_click(|| {})
            .navigate_uri("https://example.com/initial")
            .build(),
        _ => HyperlinkButton::new("hyperlink").on_click(|| {}).build(),
    };
    let owner = Button::new("tooltip owner").on_click(|| {}).build();
    let owner = match phase {
        0 => owner.tooltip(text_block("tooltip initial")),
        1 => owner.tooltip(text_block("tooltip updated")),
        _ => owner,
    };
    StackPanel::new([repeat, hyperlink, owner]).build()
}

fn only_node(runtime: &WinUiRuntime, kind: NativeKind) -> NodeId {
    let nodes = RuntimeProbe::new(runtime).nodes(kind);
    assert_eq!(nodes.len(), 1, "expected one {kind:?} node");
    nodes[0]
}

fn assert_phase(runtime: &WinUiRuntime, phase: usize) {
    let probe = RuntimeProbe::new(runtime);
    let repeat = only_node(runtime, NativeKind::RepeatButton);
    let hyperlink = only_node(runtime, NativeKind::HyperlinkButton);
    let owner = only_node(runtime, NativeKind::Button);
    assert_eq!(
        content_probe::repeat_timing(runtime, repeat).unwrap(),
        if phase == 1 { (750, 50) } else { (500, 33) }
    );
    assert_eq!(
        content_probe::navigate_uri(runtime, hyperlink).unwrap(),
        if phase == 0 {
            Some("https://example.com/initial".to_string())
        } else {
            None
        }
    );

    let tooltips = probe.nodes(NativeKind::ToolTip);
    if phase == 2 {
        assert!(tooltips.is_empty());
        assert!(content_probe::tooltip_empty(runtime, owner).unwrap());
        return;
    }
    assert_eq!(tooltips.len(), 1);
    let tooltip = tooltips[0];
    assert!(content_probe::tooltip_attached(runtime, owner, tooltip).unwrap());
    let tooltip_children = probe.children(tooltip);
    assert_eq!(tooltip_children.len(), 1);
    assert_eq!(
        content_probe::text(runtime, tooltip_children[0]).unwrap(),
        if phase == 0 {
            "tooltip initial"
        } else {
            "tooltip updated"
        }
    );
}
