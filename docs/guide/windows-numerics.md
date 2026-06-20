# windows-numerics

> Graphics-oriented vector and matrix math from `Windows.Foundation.Numerics`.

- 📦 [crates.io](https://crates.io/crates/windows-numerics)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-numerics)
- 🛠 [Internals](../internals/windows-numerics.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/numerics)

## Overview

`windows-numerics` provides the POD math value types used by Direct2D,
composition, and other graphics APIs: `Vector2`, `Vector3`, `Vector4`,
`Matrix3x2`, and `Matrix4x4`. Each is a plain `#[repr(C)]` struct with inherent
methods and operator overloads layered on top.

## Example

```rust
use windows_numerics::Vector2;

let a = Vector2::new(3.0, 4.0);

assert_eq!(a.length(), 5.0);
assert_eq!(a.distance(&Vector2::zero()), 5.0);
assert_eq!(a.dot(&Vector2::unit_x()), 3.0);
```