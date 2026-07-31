## Windows time types

The [windows-time](https://crates.io/crates/windows-time) crate provides Windows `TimeSpan` and
`DateTime` values with Rust constructors, arithmetic, and `std::time` conversions.

* [Getting started](https://github.com/microsoft/windows-rs/blob/master/docs/readme.md)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

`TimeSpan` is a duration and `DateTime` is a point in time. Both use 100-nanosecond ticks.

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-time]
version = "0.1"
```

Use the time types as needed:

```rust
use windows_time::*;

let span = TimeSpan::from_minutes(90);
assert_eq!(span.whole_seconds(), 5400);

let start = DateTime::from_unix_secs(1_000_000_000);
let later = start.checked_add(span).unwrap();
assert_eq!(later.unix_secs(), 1_000_005_400);
```
