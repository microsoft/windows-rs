# Reactor native self-test

This executable runs the Reactor WinUI adapter inside the Reactor application host. Generated
coverage constructs and clears every schema property and event contract. Handwritten phases cover
controlled input, ordered native events, attachments, images, references, pointer injection,
retirement, TreeView content, ListView data, and virtualization.

```text
cargo run -p test-reactor-selftest --quiet
cargo run -p test-reactor-selftest --quiet -- --headless
```

A watchdog terminates the process with an error if WinUI hangs or the application fails to exit.
Application lifecycle, menu, Canvas, WebView, and window-state fixtures live in
[`test-reactor-integration`](../reactor_integration/readme.md). They use the public application
API without enabling Reactor's diagnostic `test` feature.

CI also runs the self-test and native library tests in debug mode with
`RUSTFLAGS="-D warnings --cfg windows_cast_diagnostics"`. `check_casts.ps1` checks their captured
output for unexpected same-pointer `QueryInterface` calls. Its allowlist covers agility checks,
canonical COM identity, activation, and known conversions from `IInspectable`; these still require
queries even when the returned pointer is unchanged. Review new diagnostics before adding an
exception. A clean result covers only the paths exercised by those tests.

Run every default Reactor, Composition, and WebView sample long enough to catch startup failures:

```text
powershell -File crates\tests\libs\reactor_selftest\run_samples.ps1
```

The script builds each package independently, captures runtime stdout and stderr, requires the
process to remain alive for three seconds, and then closes or terminates that process. Use
`-RunSeconds`, `-Family`, or `-Package` to narrow or lengthen a run, and `-KeepLogs` to retain all
captured output.
