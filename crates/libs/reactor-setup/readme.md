## Windows Reactor Setup

Windows Reactor Setup stages the Windows App SDK runtime files needed by a
[`windows-reactor`](https://crates.io/crates/windows-reactor) application.

Add it as a build dependency:

```toml
[build-dependencies]
windows-reactor-setup = "0.100"
```

Call the setup function from `build.rs`:

```rust,no_run
windows_reactor_setup::as_self_contained();
```

`as_self_contained` stages a private copy of the runtime next to the application.

A framework-dependent app does not rely on `windows-reactor-setup` at all: the bootstrap is
inlined into `windows-reactor`, which resolves the installed framework package at startup (call
`App::framework_dependent()`). It resolves a package and stages no runtime files.
