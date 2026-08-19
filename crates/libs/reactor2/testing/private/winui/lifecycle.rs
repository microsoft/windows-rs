use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use super::*;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_LIFECYCLE_FIXTURE";

#[derive(Clone, PartialEq)]
struct Values {
    text: &'static str,
    visible: bool,
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn effects_and_element_references_follow_native_lifecycle() {
    let output = test_reactor_support::run_test_process(
        "winui::tests::lifecycle::lifecycle_fixture",
        &[(FIXTURE_ENV, "1")],
        Duration::from_secs(30),
    )
    .unwrap();

    test_reactor_support::assert_success(output);
}

#[test]
fn lifecycle_fixture() {
    if std::env::var_os(FIXTURE_ENV).is_none() {
        return;
    }

    bootstrap().unwrap();
    let state = Rc::new(RefCell::new(None::<State<Values>>));
    let publish_state = Rc::clone(&state);
    let reference = Rc::new(RefCell::new(None::<ElementRef<TextBox>>));
    let publish_reference = Rc::clone(&reference);
    let mounts = Rc::new(Cell::new(0));
    let unmounts = Rc::new(Cell::new(0));
    let effect_runs = Rc::new(Cell::new(0));
    let effect_cleanups = Rc::new(Cell::new(0));
    let render_mounts = Rc::clone(&mounts);
    let render_unmounts = Rc::clone(&unmounts);
    let render_effect_runs = Rc::clone(&effect_runs);
    let render_effect_cleanups = Rc::clone(&effect_cleanups);
    let content = component(move |cx| {
        let values = cx.use_state(|| Values {
            text: "initial",
            visible: true,
        });
        publish_state.borrow_mut().replace(values.clone());
        let mounts = Rc::clone(&render_mounts);
        let unmounts = Rc::clone(&render_unmounts);
        let text_box_ref = cx.use_element_ref_with_lifecycle::<TextBox>(
            move || mounts.set(mounts.get() + 1),
            move || unmounts.set(unmounts.get() + 1),
        );
        publish_reference.borrow_mut().replace(text_box_ref.clone());
        let current = values.try_value().unwrap();
        if !current.visible {
            return text_block("remaining");
        }

        let effect_runs = Rc::clone(&render_effect_runs);
        let effect_cleanups = Rc::clone(&render_effect_cleanups);
        let effect = component(move |cx| {
            let effect_runs = Rc::clone(&effect_runs);
            let effect_cleanups = Rc::clone(&effect_cleanups);
            cx.use_effect_with_cleanup(current.text, move || {
                effect_runs.set(effect_runs.get() + 1);
                move || effect_cleanups.set(effect_cleanups.get() + 1)
            });
            text_block("effect")
        });
        StackPanel::new([
            TextBox::new(current.text, |_| {})
                .reference(&text_box_ref)
                .build(),
            effect,
        ])
        .build()
    });
    let application_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_application_state = Rc::clone(&application_state);
    let content = Rc::new(RefCell::new(Some(content)));
    let root = component(move |cx| {
        let open = cx.use_state(|| true);
        publish_application_state.borrow_mut().replace(open.clone());
        Application::new(if open.try_value().unwrap() {
            vec![
                Window::new(
                    "windows-reactor lifecycle fixture",
                    content.borrow_mut().take().unwrap(),
                    || {},
                )
                .build(),
            ]
        } else {
            Vec::new()
        })
        .build()
    });

    run_app_fixture(root, move |reactor| {
        assert_eq!(effect_runs.get(), 1);
        assert_eq!(effect_cleanups.get(), 0);
        assert!(reference.borrow().as_ref().unwrap().is_mounted());
        assert_eq!(mounts.get(), 1);
        assert_eq!(unmounts.get(), 0);

        assert!(state.borrow().as_ref().unwrap().try_set(Values {
            text: "updated",
            visible: true,
        }));
        reactor.pump();
        assert_eq!(effect_runs.get(), 2);
        assert_eq!(effect_cleanups.get(), 1);
        assert!(reference.borrow().as_ref().unwrap().is_mounted());
        assert_eq!(mounts.get(), 1);
        assert_eq!(unmounts.get(), 0);

        assert!(state.borrow().as_ref().unwrap().try_set(Values {
            text: "updated",
            visible: false,
        }));
        reactor.pump();
        assert_eq!(effect_runs.get(), 2);
        assert_eq!(effect_cleanups.get(), 2);
        assert!(!reference.borrow().as_ref().unwrap().is_mounted());
        assert_eq!(mounts.get(), 1);
        assert_eq!(unmounts.get(), 1);
        assert!(application_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        Ok(())
    })
    .unwrap();
}
