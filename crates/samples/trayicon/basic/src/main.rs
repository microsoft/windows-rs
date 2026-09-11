#![windows_subsystem = "windows"]

use windows_trayicon::{TrayIcon, TrayIconEvent};

fn main() -> windows_trayicon::Result<()> {
    let icon = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "\\..\\..\\reactor\\icon\\icon.ico"
    );
    let _icon = TrayIcon::new(icon)
        .tooltip("Windows Tray Icon - select to exit")
        .on_event(|event| match event {
            TrayIconEvent::Activate { .. } | TrayIconEvent::ContextMenu { .. } => {
                windows_window::quit();
            }
            TrayIconEvent::Unavailable => {
                eprintln!("the Windows Shell could not restore the tray icon");
                windows_window::quit();
            }
            _ => {}
        })
        .build()?;

    windows_window::run();
    Ok(())
}
