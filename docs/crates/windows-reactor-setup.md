# windows-reactor-setup

> A build-script helper that stages the Windows App SDK runtime for reactor apps.

- 📦 Not published to crates.io
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reactor-setup)

## Overview

`windows-reactor-setup` is used from the `build.rs` of a
[`windows-reactor`](windows-reactor.md) application. It downloads and stages the
Windows App SDK runtime bootstrap files next to the built executable so the app
can start WinUI 3, and writes the required application manifest. Choose the helper
that matches your deployment model — for example a framework-dependent app or a
self-contained one.

```rust,ignore
// build.rs
fn main() {
    windows_reactor_setup::as_self_contained();
}
```

See the
[samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/reactor)
for complete project layouts.

## How it's built

A small unpublished helper crate that drives the Windows App Runtime
installer/bootstrapper.

## Testing

Run `cargo test -p windows-reactor-setup`; see also the workspace test crates.
