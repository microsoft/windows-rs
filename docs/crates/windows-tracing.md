# windows-tracing

> Compile-time TraceLogging events for Event Tracing for Windows.

- 📦 [crates.io](https://crates.io/crates/windows-tracing)
- 📖 [docs.rs](https://docs.rs/windows-tracing)
- 🚀 [Getting started](../../crates/libs/tracing/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/tracing)

`windows-tracing` defines static, manifest-free ETW providers. Its event macro encodes provider,
event, and field metadata at compile time. The disabled path checks the provider's atomic level and
keyword state before evaluating field expressions. The enabled path constructs only a fixed-size
descriptor array on the stack before calling `EventWriteTransfer`.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is for contributors and
is **not needed to use `windows-tracing`**.

### How it's built

`windows-tracing` contains the ETW registration, filtering, descriptor, and write runtime. It is
`no_std`. `crates/tools/bindings/src/tracing.txt` selects the required ETW functions, constants,
and structures. `tool_bindings` generates `src/bindings.rs`; do not edit that file by hand.

`windows-tracing-macros` is the proc-macro crate required by Rust. It emits provider metadata,
event metadata, typed field bindings, and stack-based `EVENT_DATA_DESCRIPTOR` arrays. Applications
use only the macros re-exported by `windows-tracing`.

### Registration safety

`Provider::register` is unsafe because ETW retains an enable callback into the registering module.
A DLL must drop its `Registration` before unloading. The registration unregisters on drop and also
provides `Registration::unregister` when the caller needs to handle an unregister error.

The callback publishes level and keyword filters through atomics. It marks the provider disabled
while replacing a filter configuration so an event cannot observe a newly enabled configuration
before all filter values are available.

### Initial schema support

The event macro supports primitive integers, floating-point values, 32-bit booleans, `GUID`,
`HRESULT`, `WIN32_ERROR`, counted UTF-8 and UTF-16 strings, and counted binary data. Event IDs,
versions, levels, and keywords are supported. Runtime-defined schemas, provider groups, activities,
nested structures, arrays, and custom formatting are reserved for later additions.

### Testing

`test_tracing` covers provider identity, disabled expression suppression, exclusive registration,
filtering, every supported field encoding, oversized counted fields, and registration reuse. Its
integration test captures an ETL file, decodes it with TDH, and verifies the event descriptor,
provider name, property metadata, and property values.

Run the runtime doctests, macro unit tests, and ETL integration tests:

```powershell
cargo test -p windows-tracing
cargo test -p windows-tracing-macros
cargo test -p test_tracing
```

Regenerate the native bindings after changing `tracing.txt`:

```powershell
cargo run -p tool_bindings --quiet
```
