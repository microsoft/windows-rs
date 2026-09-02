## Windows Reactor Setup

Windows Reactor Setup stages the Windows App SDK runtime files needed by a
[`windows-reactor`](https://crates.io/crates/windows-reactor) application that runs
fully self-contained.

Add it as a build dependency:

```toml
[build-dependencies]
windows-reactor-setup = "0.100"
```

Call the setup function from `build.rs`:

```rust,no_run
windows_reactor_setup::as_self_contained();
```

`as_self_contained` stages a private copy of the Windows App Runtime next to the
application and writes the application manifest. Self-contained executables carry a
deployment marker so framework-dependent binaries ignore private runtime files left in a
shared Cargo target directory.

A framework-dependent app does not rely on `windows-reactor-setup` at all: the bootstrap is
inlined into `windows-reactor`, which resolves the installed framework package at startup. It
stages no runtime files.
