# Reactor2 native self-test

This executable runs the Reactor2 WinUI adapter inside the existing Reactor application host. It
opens a real TreeView with custom visual content and performs 100 alternating keyed reorders while
updating text.

```text
cargo run -p test-reactor2-selftest --quiet
```

A watchdog terminates the process with an error if WinUI hangs or the application fails to exit.
