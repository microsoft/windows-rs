## Windows numeric types

The [windows-numerics](https://crates.io/crates/windows-numerics) crate provides graphics-oriented math types for Windows.

* [Getting started](https://github.com/microsoft/windows-rs/blob/master/docs/readme.md)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

These are the same graphics math types used by composition, Direct2D, and Direct3D
APIs, so values flow into those APIs without conversion.

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-numerics]
version = "0.3"
```

Use the vector and matrix types as needed:

```rust
use windows_numerics::*;

let a = Vector2::new(3.0, 4.0);
assert_eq!(a.length(), 5.0);

let b = Vector3::new(1.0, 0.0, 0.0);
let c = Vector3::new(0.0, 1.0, 0.0);
assert_eq!(b.cross(&c), Vector3::new(0.0, 0.0, 1.0));

let transform = Matrix3x2::translation(10.0, 20.0);
assert_eq!(transform.m31, 10.0);
assert_eq!(transform.m32, 20.0);
```
