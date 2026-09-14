## windows-notifyicon

Windows Notify Icon provides a small safe wrapper around Windows notification-area icons.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-notifyicon.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies]
windows-notifyicon = "0.100"
windows-window = "0.100"
```

```rust,no_run
use windows_notifyicon::{NotifyIcon, NotifyIconEvent};

fn main() -> windows_notifyicon::Result<()> {
    let _icon = NotifyIcon::new("icon.ico")
        .tooltip("Example")
        .on_event(|event| match event {
            NotifyIconEvent::Activate { .. } => println!("activated"),
            NotifyIconEvent::ContextMenu { .. } => windows_window::quit(),
            NotifyIconEvent::Unavailable => {
                eprintln!("the Windows Shell could not restore the notification icon");
            }
            _ => {}
        })
        .build()?;

    windows_window::run();
    Ok(())
}
```

Create and drop the icon on the thread that owns an unfiltered message loop. `NotifyIcon` uses
separate hidden windows for Shell reception and application callback dispatch, so a loop filtered
to `NotifyIcon::hwnd()` will not dispatch callbacks. Dropping `NotifyIcon` removes the
notification-area icon and destroys both hidden windows.

Like other `windows-window` windows, creating a `NotifyIcon` attempts to enable per-monitor-v2
process DPI awareness.
