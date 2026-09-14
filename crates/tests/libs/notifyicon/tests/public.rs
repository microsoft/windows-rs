use windows_notifyicon::{NotifyIcon, NotifyIconEvent};

#[test]
fn builder_accepts_the_public_configuration() {
    let _builder = NotifyIcon::new("icon.ico")
        .tooltip("Example")
        .on_event(|event| match event {
            NotifyIconEvent::Activate { position } | NotifyIconEvent::ContextMenu { position } => {
                let _ = position;
            }
            NotifyIconEvent::Unavailable => {}
            _ => {}
        });
}

#[test]
#[ignore = "requires an interactive Windows shell"]
fn live_icon_has_a_window_and_shell_rectangle() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "\\assets\\icon.ico");
    let mut icon = NotifyIcon::new(path).tooltip("Live test").build().unwrap();
    assert!(!icon.hwnd().is_null());
    icon.set_tooltip(Some("Updated live test")).unwrap();
    icon.set_icon(path).unwrap();
    icon.set_tooltip(None).unwrap();
    let rect = icon.rect().unwrap();
    assert!(rect.right > rect.left);
    assert!(rect.bottom > rect.top);
}

#[test]
#[ignore = "requires an interactive Windows shell"]
fn window_and_notification_icon_lifetimes_are_independent() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "\\assets\\icon.ico");

    let window = windows_window::Window::new("Notification icon lifetime test")
        .quit_on_close(false)
        .create()
        .unwrap();
    let icon = NotifyIcon::new(path).build().unwrap();
    drop(window);
    assert!(icon.rect().is_ok());

    let window = windows_window::Window::new("Window lifetime test")
        .quit_on_close(false)
        .create()
        .unwrap();
    drop(icon);
    assert!(!window.hwnd().is_null());
}
