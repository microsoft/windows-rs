# windows-services

Implement a Windows service process with startup, status, and control callbacks.
Installation and configuration belong to an installer or the Service Control Manager (SCM).

- 📦 [crates.io](https://crates.io/crates/windows-services)
- 📖 [docs.rs](https://docs.rs/windows-services)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/services)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/services)

## Get started

```toml
[dependencies]
windows-services = "0.100"
```

```rust,no_run
windows_services::Service::new()
    .can_pause()
    .can_stop()
    .run(|_service, _command| {
        // Start, stop, pause, or resume the application's workers here.
    })
    .unwrap();
```

The [simple sample][simple] logs commands to a file. Running it outside the SCM prints installation
commands. Register the executable's absolute path, start it through the SCM, then stop and remove
the test registration when finished. Registration normally needs administrative rights.

## Handle commands

| Builder method | Enables |
| --- | --- |
| `can_stop()` | Stop and shutdown controls |
| `can_pause()` | Pause and resume |
| `can_accept(mask)` | Other controls delivered as `Command::Extended` |
| `can_fallback(closure)` | An intentional console-mode execution path |

Handle `Command::Start` and every enabled control. Keep callbacks responsive: start workers or
signal them instead of running the main workload inside the callback. Stop workers and release
resources during shutdown.

The crate reports normal pending and stable states around callbacks. Use `state()` to inspect
them and `set_state()` when an explicit transition is needed.

A service has a different account, working directory, and desktop context from an interactive
application. Use absolute paths and a service-safe log destination, not console output.

[simple]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/services/simple
