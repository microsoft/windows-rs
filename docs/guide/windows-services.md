# windows-services

> A simple, safe way to implement Windows services in Rust.

- 📦 [crates.io](https://crates.io/crates/windows-services)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-services)
- 🛠 [Internals](../internals/windows-services.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/services)

## Overview

`windows-services` lets you write a Windows service process with a builder. Use
`Service::new` to declare which control commands the service accepts
(`can_stop`, `can_pause`, …), then `run` to hand control to the service control
manager. Your closure receives the service handle and each incoming command.

## Example

```rust,no_run
fn main() {
    windows_services::Service::new()
        .can_pause()
        .can_stop()
        .run(|_service, _command| {
            // Respond to service commands...
        })
        .unwrap();
}
```