use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use super::typography_access as typography_probe;
use super::*;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_TYPOGRAPHY_FIXTURE";

#[test]
#[ignore = "requires the Windows App Runtime"]
fn typography_accessibility_and_input_properties_update_and_reset() {
    let output = test_reactor_support::run_test_process(
        "winui::tests::typography_properties::typography_fixture",
        &[(FIXTURE_ENV, "run")],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
fn typography_fixture() {
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
                    "Typography fixture",
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
        let mut text = None;
        assert_phase(reactor.engine().runtime(), 0, &mut text);
        assert!(phase_state.borrow().as_ref().unwrap().try_set(1));
        reactor.pump();
        assert_phase(reactor.engine().runtime(), 1, &mut text);
        assert!(phase_state.borrow().as_ref().unwrap().try_set(2));
        reactor.pump();
        assert_phase(reactor.engine().runtime(), 2, &mut text);
        assert!(open_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        Ok(())
    })
    .unwrap();
}

fn content(phase: usize) -> Element {
    let text = match phase {
        0 => TextBlock::new("typography")
            .font_size(16.0)
            .character_spacing(100)
            .font_weight(FontWeight::BOLD)
            .font_style(FontStyle::Italic)
            .font_stretch(FontStretch::Condensed)
            .font_family(Some("Arial".to_string()))
            .foreground(Color::rgb(10, 20, 30))
            .text_wrapping(TextWrapping::Wrap)
            .text_trimming(TextTrimming::WordEllipsis)
            .text_selection_enabled(true)
            .keyboard_accelerator(KeyboardAccelerator::new(
                VirtualKey::S,
                VirtualKeyModifiers::CONTROL,
                || {},
            ))
            .automation_name("initial name")
            .automation_id("initial-id")
            .help_text("initial help")
            .heading_level(AutomationHeadingLevel::Level1)
            .on_drop(
                DropTarget::new(DropOperation::Copy, DropFormats::TEXT),
                |_| {},
            )
            .build(),
        1 => TextBlock::new("typography")
            .font_size(24.0)
            .character_spacing(-50)
            .font_weight(FontWeight::LIGHT)
            .font_style(FontStyle::Oblique)
            .font_stretch(FontStretch::Expanded)
            .font_family(Some("Consolas".to_string()))
            .foreground(Color::argb(128, 40, 50, 60))
            .text_wrapping(TextWrapping::WrapWholeWords)
            .text_trimming(TextTrimming::Clip)
            .text_selection_enabled(false)
            .keyboard_accelerator(KeyboardAccelerator::new(
                VirtualKey::S,
                VirtualKeyModifiers::CONTROL | VirtualKeyModifiers::SHIFT,
                || {},
            ))
            .automation_name("updated name")
            .automation_id("updated-id")
            .help_text("updated help")
            .heading_level(AutomationHeadingLevel::Level2)
            .on_drop(
                DropTarget::new(DropOperation::Move, DropFormats::STORAGE_ITEMS),
                |_| {},
            )
            .build(),
        _ => TextBlock::new("typography").build(),
    };
    let button = match phase {
        0 => Button::new("button")
            .on_click(|| {})
            .font_size(16.0)
            .character_spacing(100)
            .font_weight(FontWeight::BOLD)
            .font_style(FontStyle::Italic)
            .font_stretch(FontStretch::Condensed)
            .font_family(Some("Arial".to_string()))
            .foreground(Color::rgb(10, 20, 30))
            .automation_name("initial name")
            .automation_id("initial-id")
            .help_text("initial help")
            .heading_level(AutomationHeadingLevel::Level1)
            .build(),
        1 => Button::new("button")
            .on_click(|| {})
            .font_size(24.0)
            .character_spacing(-50)
            .font_weight(FontWeight::LIGHT)
            .font_style(FontStyle::Oblique)
            .font_stretch(FontStretch::Expanded)
            .font_family(Some("Consolas".to_string()))
            .foreground(Color::argb(128, 40, 50, 60))
            .automation_name("updated name")
            .automation_id("updated-id")
            .help_text("updated help")
            .heading_level(AutomationHeadingLevel::Level2)
            .build(),
        _ => Button::new("button").on_click(|| {}).build(),
    };
    StackPanel::new([text, button]).build()
}

fn only_node(runtime: &WinUiRuntime, kind: NativeKind) -> NodeId {
    let nodes = RuntimeProbe::new(runtime).nodes(kind);
    assert_eq!(nodes.len(), 1, "expected one {kind:?} node");
    nodes[0]
}

fn assert_phase(runtime: &WinUiRuntime, phase: usize, text: &mut Option<NodeId>) {
    let text = *text.get_or_insert_with(|| {
        RuntimeProbe::new(runtime)
            .nodes(NativeKind::TextBlock)
            .into_iter()
            .find(|id| typography_probe::automation(runtime, *id).unwrap().0 == "initial name")
            .unwrap()
    });
    let button = only_node(runtime, NativeKind::Button);
    let expected = match phase {
        0 => (
            16.0,
            100,
            FontWeight::BOLD.weight(),
            FontStyle::Italic as i32,
            FontStretch::Condensed as i32,
            "Arial",
            Color::rgb(10, 20, 30),
            ("initial name", "initial-id", "initial help", 1),
        ),
        1 => (
            24.0,
            -50,
            FontWeight::LIGHT.weight(),
            FontStyle::Oblique as i32,
            FontStretch::Expanded as i32,
            "Consolas",
            Color::argb(128, 40, 50, 60),
            ("updated name", "updated-id", "updated help", 2),
        ),
        _ => (
            14.0,
            0,
            FontWeight::NORMAL.weight(),
            FontStyle::Normal as i32,
            FontStretch::Normal as i32,
            "Segoe UI Variable",
            Color::rgb(255, 255, 255),
            ("", "", "", 0),
        ),
    };

    for id in [text, button] {
        assert_eq!(
            typography_probe::font_size(runtime, id).unwrap(),
            expected.0
        );
        assert_eq!(
            typography_probe::character_spacing(runtime, id).unwrap(),
            expected.1
        );
        assert_eq!(
            typography_probe::font_weight(runtime, id).unwrap(),
            expected.2
        );
        assert_eq!(
            typography_probe::font_style(runtime, id).unwrap(),
            expected.3
        );
        assert_eq!(
            typography_probe::font_stretch(runtime, id).unwrap(),
            expected.4
        );
        assert_eq!(
            typography_probe::font_family(runtime, id).unwrap(),
            expected.5
        );
        assert_eq!(
            typography_probe::foreground(runtime, id).unwrap(),
            expected.6
        );
        assert_eq!(
            typography_probe::automation(runtime, id).unwrap(),
            (
                expected.7.0.to_string(),
                expected.7.1.to_string(),
                expected.7.2.to_string(),
                expected.7.3,
            )
        );
    }

    assert_eq!(
        typography_probe::text_properties(runtime, text).unwrap(),
        match phase {
            0 => (
                TextWrapping::Wrap as i32,
                TextTrimming::WordEllipsis as i32,
                true,
            ),
            1 => (
                TextWrapping::WrapWholeWords as i32,
                TextTrimming::Clip as i32,
                false,
            ),
            _ => (
                TextWrapping::NoWrap as i32,
                TextTrimming::None as i32,
                false,
            ),
        }
    );
    assert_eq!(
        typography_probe::keyboard_accelerators(runtime, text).unwrap(),
        match phase {
            0 => vec![(VirtualKey::S.code(), VirtualKeyModifiers::CONTROL.bits())],
            1 => vec![(
                VirtualKey::S.code(),
                (VirtualKeyModifiers::CONTROL | VirtualKeyModifiers::SHIFT).bits(),
            )],
            _ => Vec::new(),
        }
    );
    assert_eq!(
        typography_probe::allow_drop(runtime, text).unwrap(),
        phase != 2
    );
}
