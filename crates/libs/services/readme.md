## Windows services

The [windows-services](https://crates.io/crates/windows-services) crate implements Windows service
processes.

* [Getting started](https://github.com/microsoft/windows-rs/blob/master/docs/readme.md)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-services]
version = "0.100"
```

```rust,no_run
windows_services::Service::new()
    .can_pause()
    .can_stop()
    .run(|_service, _command| {})
    .unwrap();
```
