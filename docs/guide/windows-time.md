# windows-time

> WinRT `TimeSpan` and `DateTime` with idiomatic Rust conversions.

- 📦 [crates.io](https://crates.io/crates/windows-time)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-time)
- 🛠 [Internals](../internals/windows-time.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/time)

## Overview

`windows-time` provides the two WinRT time primitives as plain `#[repr(C)]` Rust types:

- **`TimeSpan`** — a duration, stored as 100-nanosecond ticks.
- **`DateTime`** — an instant on a 1601-based UTC clock.

Both are `Copy`, support the usual arithmetic and comparison operators, convert
to and from `std::time` types, and `Display` as ISO-8601.

## Example

```rust
use windows_time::{DateTime, TimeSpan};
use std::time::Duration;

// Build a duration and read it back in different units.
let timeout = TimeSpan::from_seconds(30);
assert_eq!(timeout.whole_millis(), 30_000);

// Convert to and from `std::time::Duration`.
let span = TimeSpan::try_from(Duration::from_secs(5)).unwrap();
let back: Duration = span.try_into().unwrap();
assert_eq!(back, Duration::from_secs(5));

// Instants support arithmetic with durations.
let now = DateTime::now();
let later = now + TimeSpan::from_hours(1);
assert!(later > now);
assert_eq!((later - now).whole_minutes(), 60);
```

For the complete API — calendar decomposition (`year`, `month`, `day`, …),
checked and saturating arithmetic, and the full set of constructors — see the
[API reference](https://docs.rs/windows-time).