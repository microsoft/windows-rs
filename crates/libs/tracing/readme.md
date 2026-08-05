## Windows tracing

The [windows-tracing](https://crates.io/crates/windows-tracing) crate writes manifest-free
TraceLogging events to Event Tracing for Windows (ETW).

* [Getting started](https://github.com/microsoft/windows-rs/blob/master/docs/readme.md)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

Define a static provider, register it for the lifetime of the component, and write events with a
compile-time schema:

```rust,no_run
use windows_tracing::{GUID, Level, Result, define_provider, write_event};

define_provider!(
    SAMPLE_PROVIDER,
    "WindowsTracingSample",
    id(GUID::from_u128(0x4bd2826e_54a1_4ba9_bf63_92b73ea1ac4a))
);

fn main() -> Result<()> {
    // SAFETY: The registration is dropped before this executable unloads.
    let _registration = unsafe { SAMPLE_PROVIDER.register()? };

    write_event!(
        SAMPLE_PROVIDER,
        "Started",
        level(Level::INFORMATIONAL),
        keyword(0x1),
        str("Message", "Hello from Rust"),
        u32("ProcessId", std::process::id()),
    )
    .ok()
}
```

Event names, field names, and field types are encoded at compile time. Disabled events do not
evaluate field expressions or allocate memory.
