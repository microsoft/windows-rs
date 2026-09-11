use windows_trayicon::{Menu, TrayIcon, TrayIconEvent};

#[test]
fn builder_accepts_the_public_configuration() {
    let _builder = TrayIcon::new("icon.ico")
        .tooltip("Example")
        .menu(Menu::new().item(1, "Exit").separator().item(2, "Settings"))
        .on_event(|event| match event {
            TrayIconEvent::Activate { position } | TrayIconEvent::ContextMenu { position } => {
                let _ = position;
            }
            TrayIconEvent::Unavailable => {}
            TrayIconEvent::MenuItem { id } => {
                let _ = id;
            }
            _ => {}
        });
}

#[test]
#[ignore = "requires an interactive Windows shell"]
fn live_icon_has_a_window_and_shell_rectangle() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\..\\..\\..\\samples\\reactor\\icon\\icon.ico"
    );
    let mut icon = TrayIcon::new(path).tooltip("Live test").build().unwrap();
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
fn window_and_tray_icon_lifetimes_are_independent() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\..\\..\\..\\samples\\reactor\\icon\\icon.ico"
    );

    let window = windows_window::Window::new("Tray icon lifetime test")
        .quit_on_close(false)
        .create()
        .unwrap();
    let icon = TrayIcon::new(path).build().unwrap();
    drop(window);
    assert!(icon.rect().is_ok());

    let window = windows_window::Window::new("Window lifetime test")
        .quit_on_close(false)
        .create()
        .unwrap();
    drop(icon);
    assert!(!window.hwnd().is_null());
}
