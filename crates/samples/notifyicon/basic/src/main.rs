use windows_notifyicon::{NotifyIcon, NotifyIconEvent};

fn main() -> windows_notifyicon::Result<()> {
    let icon = concat!(env!("CARGO_MANIFEST_DIR"), "\\icon.ico");
    let _icon = NotifyIcon::new(icon)
        .tooltip("Left-click to activate; right-click to exit")
        .on_event(|event| match event {
            NotifyIconEvent::Activate { position } => {
                println!("activated at ({}, {})", position.x, position.y);
            }
            NotifyIconEvent::ContextMenu { position } => {
                println!(
                    "context menu requested at ({}, {}); exiting",
                    position.x, position.y
                );
                windows_window::quit();
            }
            NotifyIconEvent::Unavailable => {
                eprintln!("the Windows Shell could not restore the notification icon");
            }
            _ => {}
        })
        .build()?;

    windows_window::run();
    Ok(())
}
