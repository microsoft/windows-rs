## windows-trayicon

Windows Tray Icon provides a small safe wrapper around Windows notification-area icons.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-trayicon.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies]
windows-trayicon = "0.100"
windows-window = "0.100"
```

```rust,no_run
use windows_trayicon::{TrayIcon, TrayIconEvent};

fn main() -> windows_trayicon::Result<()> {
    let _icon = TrayIcon::new("icon.ico")
        .tooltip("Example")
        .on_event(|event| match event {
            TrayIconEvent::Activate { .. } => println!("activated"),
            TrayIconEvent::ContextMenu { .. } => println!("context menu requested"),
            TrayIconEvent::Unavailable => eprintln!("the Windows Shell rejected the tray icon"),
            _ => {}
        })
        .build()?;

    windows_window::run();
    Ok(())
}
```

Create and drop the icon on the thread that owns the message loop. Dropping `TrayIcon` removes the
notification-area icon and destroys its hidden callback window.
