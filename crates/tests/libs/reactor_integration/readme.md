# Reactor public integration tests

These isolated executables exercise the public application API without Reactor's `test` feature.
Build this package separately from privileged selftests and benchmarks so Cargo does not unify
their diagnostic features into this build.

```text
cargo check -p test-reactor-integration --all-targets
cargo run -p test-reactor-integration --bin application_lifecycle -- replacement
cargo run -p test-reactor-integration --bin application_lifecycle -- multiple
cargo run -p test-reactor-integration --bin application_lifecycle -- startup-error
cargo run -p test-reactor-integration --bin application_menu
cargo run -p test-reactor-integration --bin canvas_integration
cargo run -p test-reactor-integration --bin webview_integration
cargo run -p test-reactor-integration --bin window_state
```

Each executable owns its application lifetime and watchdog. The lifecycle fixture covers
last-window replacement, multiple-window shutdown, and startup errors. The menu fixture checks
application-lifetime work after its Reactor window closes. Companion-crate fixtures exercise
Canvas drawing and WebView initialization. The window-state fixture observes size and icon changes
through the public window callback and Win32 APIs.

Use `--features self-contained` to run with bundled Windows App SDK dependencies. Otherwise,
the Windows App Runtime must be installed. WebView tests also need the WebView2 runtime.
