## Windows services

The [windows-services](https://crates.io/crates/windows-services) crate provides a simple and safe way to implement Windows services in Rust.

* [Getting started](https://github.com/microsoft/windows-rs/blob/master/docs/getting-started.md)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-services]
version = "0.26"
```

Use the Windows services support as needed. Here is how you might write a simple Windows services process:

```rust,no_run
windows_services::Service::new()
    .can_pause()
    .can_stop()
    .run(|service, command| {
        // Respond to service commands...
    })
    .unwrap();
```
