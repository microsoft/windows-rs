# windows-trayicon

> A small safe wrapper around Windows notification-area icons.

- 📦 [crates.io](https://crates.io/crates/windows-trayicon)
- 📖 [docs.rs](https://docs.rs/windows-trayicon)
- 🚀 [Getting started](../../crates/libs/trayicon/readme.md)
- 🧩 [Standalone sample](../../crates/samples/trayicon/basic)
- 🧩 [Reactor sample](../../crates/samples/reactor/trayicon)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/trayicon)
- [Window guide](windows-window.md)

## When to use it

Use `windows-trayicon` when a desktop application needs an icon in the taskbar notification area.
The crate manages Shell registration and a hidden callback window without depending on the
`windows` or `windows-sys` umbrella crates.

The initial API covers icon files, standard tooltips, activation, context-menu requests, a minimal
native popup menu, icon geometry, and runtime icon or tooltip replacement. It does not provide
balloon notifications, promotion policy, raw input, submenus, or owner-drawn menus.

## Create an icon

Create the icon on the thread that owns the application message loop:

```rust,no_run
use windows_trayicon::{Menu, TrayIcon, TrayIconEvent};

fn main() -> windows_trayicon::Result<()> {
    let _icon = TrayIcon::new("icon.ico")
        .tooltip("Example")
        .menu(Menu::new().item(1, "Exit"))
        .on_event(|event| match event {
            TrayIconEvent::Activate { position } => {
                println!("selected at {}, {}", position.x, position.y);
            }
            TrayIconEvent::MenuItem { id: 1 } => windows_window::quit(),
            TrayIconEvent::Unavailable => {
                eprintln!("the Windows Shell could not restore the icon");
            }
            _ => {}
        })
        .build()?;

    windows_window::run();
    Ok(())
}
```

`TrayIcon` owns an unshown top-level `windows-window` window. Any message loop running on that
thread can dispatch its callbacks; the application does not need to use `windows_window::run`.
Dropping the value removes the Shell icon before destroying the callback window and loaded icon.
Shell callbacks are reposted before the application handler runs, avoiding reentrant application
work inside a Shell call and waking message loops blocked while waiting for posted messages.

## Respond to events

`Activate` represents selection by mouse or keyboard. `ContextMenu` provides the screen position
reported by the Shell. Applications can use the event position or call `rect` to anchor a popup
without assuming which monitor or taskbar edge contains the icon.

Without a configured `Menu`, `ContextMenu` reports the requested position so the application can
show a custom popup. With a configured menu, the crate applies the foreground-window and
light-dismiss rules, anchors the popup to the icon on its monitor, and reports a selected item as
`MenuItem { id }`. The Shell-reported position is used if icon geometry is unavailable.

The initial menu API supports labeled items and separators:

```rust,ignore
let menu = Menu::new()
    .item(1, "Open")
    .separator()
    .item(2, "Exit");
```

Item identifiers must be nonzero and unique.

The crate requests `NOTIFYICON_VERSION_4`, which provides consistent selection events, keyboard
activation, signed screen coordinates, and better accessibility. Lower-level mouse messages are
not exposed as duplicate activation events.

## Update the icon

`set_icon` loads a replacement `.ico` file and `set_tooltip` changes or clears the standard
tooltip:

```rust,ignore
icon.set_icon("connected.ico")?;
icon.set_tooltip(Some("Connected"))?;
icon.set_tooltip(None)?;
```

Failed changes retain the previous owned resource and return a `windows_core::Error`.

## Shell restarts

The hidden callback window listens for the registered `TaskbarCreated` message. When Explorer
restarts, the crate adds the icon again and reapplies `NOTIFYICON_VERSION_4`. Transient failures
receive three attempts one second apart. `Unavailable` reports that all recovery attempts failed;
the application can drop the value and build another `TrayIcon` to try again later.

The crate uses window-plus-numeric identity internally. It does not manipulate notification-area
promotion settings; visibility in the main notification area or overflow remains under user and
Shell control.

## Live test and sample

Unit tests cover callback decoding and tooltip validation without modifying the notification area.
An ignored live test exercises Shell registration and `Shell_NotifyIconGetRect`:

```text
cargo test -p test_trayicon -- --ignored --nocapture --test-threads=1
```

The ignored UI Automation test injects version-4 callbacks into the hidden test window, opens the
real native popup menu, finds its item through UI Automation, and invokes it. Windows 11 does not
consistently expose notification icons themselves in the desktop UIA tree, so real icon mouse and
keyboard input remains part of the manual gate.

The Reactor sample starts with no Reactor windows, opens them from tray events, and demonstrates
that either the window or tray icon can be removed and created again while the other remains
active. It exits when the last window closes after the tray icon has been removed.

Run the standalone sample from a terminal. It reports activation and provides a native Exit menu:

```text
cargo run -p trayicon-basic
```

---

## Internal documentation

`src/bindings.rs` is generated by `tool-bindings` from
`crates/tools/bindings/src/trayicon.txt`. It contains only the Shell notification-icon, icon-file,
registered-message, and geometry APIs used by the wrapper.

`crates/tests/libs/trayicon/tests/support/bindings.rs` is generated from
`crates/tools/bindings/src/trayicon_test.txt`. These private test bindings provide UI Automation
and raw message-loop APIs without adding them to the published crate.

`TrayIconBuilder::build` loads the icon, registers `TaskbarCreated`, and creates an unshown
per-monitor-v2 `windows-window` top-level window without changing the host's process DPI policy. It
then calls `NIM_ADD` followed by `NIM_SETVERSION` with `NOTIFYICON_VERSION_4`. Failure to set the
version deletes the partially added icon.

`NIM_MODIFY` failures receive one full add-and-version attempt in case Explorer restarted between
operations. The callback window decodes only activation and context-menu events. Keep additional
Shell messages private until a public use case requires them.

The hidden callback window is created under a temporary per-monitor-v2 thread context. Windows
restores that window's creation-time context while dispatching its window procedure, so Shell
geometry, callback coordinates, and `TrackPopupMenu` all use physical screen coordinates. The
temporary context is restored immediately after window creation and does not change the host's
process DPI policy.

After changing the binding filter, run:

```text
cargo run -p tool-bindings --quiet
cargo check -p windows-trayicon --quiet
```
