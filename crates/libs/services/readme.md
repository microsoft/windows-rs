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
windows-services = "0.101"
```

```rust,no_run
windows_services::Service::new()
    .can_stop()
    .run(|_service, _command| {
        // Start or stop the application's workers here.
    })
    .unwrap();
```

The [simple sample][simple] logs commands to a file. Running it outside the SCM prints installation
commands. Register the executable's absolute path, start it through the SCM, then stop and remove
the test registration when finished. Registration normally needs administrative rights.

## Handle commands

Handle `Command::Start` and every enabled control. Keep callbacks responsive: start workers or
signal them instead of running the main workload inside the callback. Stop workers and release
resources during shutdown.

A service has a different account, working directory, and desktop context from an interactive
application. Use absolute paths and a service-safe log destination, not console output.

[simple]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/services/simple
