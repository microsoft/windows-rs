# Reactor public integration tests

These isolated executables exercise the public application API without Reactor's `test` feature.
Build this package separately from privileged selftests and benchmarks so Cargo does not unify
their diagnostic features into this build.

```text
cargo check -p test-reactor-integration --all-targets
cargo run -p test-reactor-integration --bin application_lifecycle -- replacement
cargo run -p test-reactor-integration --bin application_lifecycle -- multiple
cargo run -p test-reactor-integration --bin application_lifecycle -- startup-error
cargo run -p test-reactor-integration --bin optional_changes -- enable
cargo run -p test-reactor-integration --bin optional_changes -- disable
cargo run -p test-reactor-integration --bin optional_changes -- query
cargo run -p test-reactor-integration --bin application_menu
cargo run -p test-reactor-integration --bin application_menus
cargo run -p test-reactor-integration --bin application_resources -- lifecycle
cargo run -p test-reactor-integration --bin application_dispatch -- application
cargo run -p test-reactor-integration --bin application_dispatch -- notification
cargo run -p test-reactor-integration --bin tray_flyout
cargo run -p test-reactor-integration --bin menu_position -- thread-unaware
cargo run -p test-reactor-integration --bin canvas_integration
cargo run -p test-reactor-integration --bin webview_integration
cargo run -p test-reactor-integration --bin window_state
```

Each executable owns its application lifetime and watchdog. The lifecycle fixture covers
last-window replacement, multiple-window shutdown, and startup errors. The menu fixture checks
application-lifetime work after its Reactor window closes. The application-resource fixtures cover
declarative window and icon lifetimes, nested dispatch, and menu replacement and selection.
The notification dispatch mode sends interactions and recovery requests to its own callback window
during nested component and application updates, and checks that queued events cannot outlive icon
removal.
The tray-flyout fixture uses an application-owned icon and checks that its menu overlaps the icon's
monitor, extends outside that monitor's work area, and closes during application shutdown.
It measures menu-item bounds through UI Automation rather than the popup window's shadow bounds,
and requires a running Windows shell with a notification area. Companion-crate fixtures exercise
Canvas drawing and WebView initialization. The window-state fixture observes size and icon changes
through the public window callback and Win32 APIs.

The optional-changes modes test each method before runtime initialization, then query state and
check that mutations fail after XAML starts.

The menu-position fixture covers `pmv2`, `unaware`, `system`, and `thread-unaware` DPI modes. It
checks exact physical host bounds and restoration of the caller's thread context, including when
a duplicate menu request is rejected. Run on a display scaled above 100% to exercise coordinate
virtualization.

Use `--features self-contained` to run with bundled Windows App SDK dependencies. Otherwise,
the Windows App Runtime must be installed. WebView tests also need the WebView2 runtime.
