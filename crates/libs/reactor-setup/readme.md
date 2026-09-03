## windows-reactor-setup

Windows Reactor Setup stages the Windows App SDK runtime files needed by a
[`windows-reactor`](https://crates.io/crates/windows-reactor) application that runs
fully self-contained.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-reactor-setup.md)

Add it as a build dependency:

```toml
[build-dependencies]
windows-reactor-setup = "0.100"
```

Call the setup function from `build.rs`:

```rust,no_run
windows_reactor_setup::as_self_contained();
```
