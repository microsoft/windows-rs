use windows_trayicon::{TrayIcon, TrayIconEvent};

fn main() -> windows_trayicon::Result<()> {
    let icon = concat!(env!("CARGO_MANIFEST_DIR"), "\\icon.ico");
    let _tray = TrayIcon::new(icon)
        .tooltip("Left-click to activate; right-click to exit")
        .on_event(|event| match event {
            TrayIconEvent::Activate { position } => {
                println!("activated at ({}, {})", position.x, position.y);
            }
            TrayIconEvent::ContextMenu { position } => {
                println!(
                    "context menu requested at ({}, {}); exiting",
                    position.x, position.y
                );
                windows_window::quit();
            }
            TrayIconEvent::Unavailable => {
                eprintln!("the Windows Shell could not restore the tray icon");
            }
            _ => {}
        })
        .build()?;

    windows_window::run();
    Ok(())
}
