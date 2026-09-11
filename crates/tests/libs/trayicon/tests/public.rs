use windows_trayicon::{TrayIcon, TrayIconEvent};

#[test]
fn builder_accepts_the_public_configuration() {
    let _builder = TrayIcon::new("icon.ico")
        .tooltip("Example")
        .on_event(|event| match event {
            TrayIconEvent::Activate { position } | TrayIconEvent::ContextMenu { position } => {
                let _ = position;
            }
            TrayIconEvent::Unavailable => {}
            _ => {}
        });
}

#[test]
#[ignore = "requires an interactive Windows shell"]
fn live_icon_has_a_window_and_shell_rectangle() {
    let icon = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\..\\..\\..\\samples\\reactor\\icon\\icon.ico"
    );
    let icon = TrayIcon::new(icon).tooltip("Live test").build().unwrap();
    assert!(!icon.hwnd().is_null());
    let rect = icon.rect().unwrap();
    assert!(rect.right > rect.left);
    assert!(rect.bottom > rect.top);
}
