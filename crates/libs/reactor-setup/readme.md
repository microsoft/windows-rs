## Windows Reactor Setup

Windows Reactor Setup stages the Windows App SDK runtime files needed by a
[`windows-reactor`](https://crates.io/crates/windows-reactor) application.

Add it as a build dependency:

```toml
[build-dependencies]
windows-reactor-setup = "0.100"
```

Call one setup function from `build.rs`:

```rust,no_run
windows_reactor_setup::as_framework_dependent();
```

Use `as_framework_dependent` when the Windows App Runtime is installed on the target machine.
Use `as_self_contained` to stage a private copy of the runtime next to the application.
`as_example` is the framework-dependent setup for Cargo examples.
Self-contained executables carry a deployment marker so framework-dependent binaries ignore
private runtime files left in a shared Cargo target directory.

A framework-dependent application also calls `windows_reactor::bootstrap()` before creating its
first window. A self-contained application does not call `bootstrap()`.
