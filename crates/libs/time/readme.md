# windows-time

WinRT `TimeSpan` and `DateTime` values with arithmetic and Rust time conversions.
Use them at Windows API boundaries rather than substituting `Duration` or `SystemTime`.

- 📦 [crates.io](https://crates.io/crates/windows-time)
- 📖 [docs.rs](https://docs.rs/windows-time)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/time)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/time)

## Get started

```toml
[dependencies]
windows-time = "0.100"
```

```rust
use windows_time::*;

let span = TimeSpan::from_minutes(90);
assert_eq!(span.whole_seconds(), 5400);

let start = DateTime::from_unix_secs(1_000_000_000);
let later = start.checked_add(span).unwrap();
assert_eq!(later.unix_secs(), 1_000_005_400);
```

## Convert and calculate

`TimeSpan` is a signed duration in 100-nanosecond ticks. `DateTime` counts ticks since
1601-01-01 00:00:00 UTC. Use Unix constructors and accessors when exchanging Unix timestamps.

Conversions have 100-nanosecond precision, so finer source precision is truncated. Negative spans
cannot become `std::time::Duration`. `DateTime::now` and `SystemTime` conversions require `std`,
which is enabled by default.

Use checked arithmetic for external input. Operators panic on overflow or division by zero, while
unit and Unix constructors saturate. Validate inputs when saturation would hide a mistake.

`Display` formats UTC. `to_local()` shifts the value to local wall-clock time for reading calendar
fields; do not serialize that shifted value as UTC or convert it to `SystemTime`.
