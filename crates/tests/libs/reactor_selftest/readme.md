# Reactor native self-test

This executable runs the Reactor WinUI adapter inside the Reactor application host. Generated
coverage constructs and clears every schema property and event contract. Handwritten phases cover
controlled input, ordered native events, attachments, images, references, pointer injection,
retirement, TreeView content, ListView data, and virtualization.

```text
cargo run -p test-reactor-selftest --quiet
cargo run -p test-reactor-selftest --quiet -- --headless
cargo run -p test-reactor-selftest --bin application_lifecycle --quiet -- replacement
cargo run -p test-reactor-selftest --bin application_lifecycle --quiet -- multiple
cargo run -p test-reactor-selftest --bin application_lifecycle --quiet -- startup-error
cargo run -p test-reactor-selftest --bin application_menu --quiet
cargo run -p test-reactor-selftest --bin canvas_integration --quiet
cargo run -p test-reactor-selftest --bin webview_integration --quiet
cargo run -p test-reactor-selftest --bin window_state --quiet
```

A watchdog terminates the process with an error if WinUI hangs or the application fails to exit.
The application lifecycle fixture covers last-window replacement, multiple-window shutdown,
and startup errors. The application menu fixture keeps application-lifetime work alive after its
Reactor window closes.

Run every default Reactor, Composition, and WebView sample long enough to catch startup failures:

```text
powershell -File crates\tests\libs\reactor_selftest\run_samples.ps1
```

The script builds each package independently, captures runtime stdout and stderr, requires the
process to remain alive for three seconds, and then closes or terminates that process. Use
`-RunSeconds`, `-Family`, or `-Package` to narrow or lengthen a run, and `-KeepLogs` to retain all
captured output.
