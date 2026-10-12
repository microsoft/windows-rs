#[allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::missing_transmute_annotations,
    clippy::upper_case_acronyms
)]
#[path = "test_bindings.rs"]
mod bindings;

use super::*;

#[test]
#[ignore = "requires an interactive Windows shell"]
fn callback_window_enqueues_interaction_and_recovery() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\..\\..\\tests\\libs\\reactor_integration\\icon.ico"
    );
    let events = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let queued = std::rc::Rc::clone(&events);
    let icon = NotifyIcon::new(path)
        .on_event(move |event| queued.borrow_mut().push(event))
        .build()
        .unwrap();
    unsafe {
        bindings::SendMessageW(icon.hwnd().cast(), CALLBACK_MESSAGE, 0, NIN_SELECT as isize);
        bindings::SendMessageW(
            icon.hwnd().cast(),
            register_taskbar_created().unwrap(),
            0,
            0,
        );
    }
    assert_eq!(
        *events.borrow(),
        [
            NotifyIconEvent::Activate {
                position: Point { x: 0, y: 0 },
            },
            NotifyIconEvent::Recover,
        ]
    );
    icon.recover().unwrap();
    assert!(icon.rect().is_ok());
}
