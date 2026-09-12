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
use windows_trayicon::{Menu, TrayIcon, TrayIconEvent};

fn main() -> windows_trayicon::Result<()> {
    let _icon = TrayIcon::new("icon.ico")
        .tooltip("Example")
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
```

Create and drop the icon on the thread that owns an unfiltered message loop. `TrayIcon` uses
separate hidden windows for Shell reception and application callback dispatch, so a loop filtered
to `TrayIcon::hwnd()` will not dispatch callbacks. Dropping `TrayIcon` removes the notification-area
icon and destroys both hidden windows.

Establish the process DPI policy before building a `TrayIcon`. Once its hidden windows exist,
Windows may reject later attempts to change process DPI awareness.
