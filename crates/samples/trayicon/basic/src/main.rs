use windows_trayicon::{Menu, TrayIcon, TrayIconEvent};

fn main() -> windows_trayicon::Result<()> {
    let icon = concat!(env!("CARGO_MANIFEST_DIR"), "\\icon.ico");
    let _tray = TrayIcon::new(icon)
        .tooltip("Windows tray icon sample")
        .menu(Menu::new().item(1, "Exit"))
        .on_event(|event| match event {
            TrayIconEvent::Activate { .. } => println!("activated"),
            TrayIconEvent::MenuItem { id: 1 } => windows_window::quit(),
            TrayIconEvent::Unavailable => {
                eprintln!("the Windows Shell could not restore the tray icon");
            }
            _ => {}
        })
        .build()?;

    windows_window::run();
    Ok(())
}
