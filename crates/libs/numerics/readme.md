# windows-numerics

The `Vector2`, `Vector3`, `Vector4`, `Matrix3x2`, and `Matrix4x4` types used by Windows graphics
APIs, with arithmetic and transform helpers.

- 📦 [crates.io](https://crates.io/crates/windows-numerics)
- 📖 [docs.rs](https://docs.rs/windows-numerics)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/numerics)

## Get started

```toml
[dependencies]
windows-numerics = "0.100"
```

```rust
use windows_numerics::*;

let a = Vector2::new(3.0, 4.0);
assert_eq!(a.length(), 5.0);

let b = Vector3::new(1.0, 0.0, 0.0);
let c = Vector3::new(0.0, 1.0, 0.0);
assert_eq!(b.cross(&c), Vector3::new(0.0, 0.0, 1.0));

let transform = Matrix3x2::translation(10.0, 20.0);
assert_eq!((transform.m31, transform.m32), (10.0, 20.0));
```

## Use vectors and transforms

Vector multiplication is component-wise; scalar arithmetic uses `f32`. Use squared length or
distance when only a comparison is needed, and check for zero length before normalizing.

`Matrix3x2` provides translation, scale, rotation, and skew, including transforms around a center.
Rotation and skew angles are in degrees. Matrix multiplication order matters: check representative
points instead of assuming another math library's convention.

Equality is exact floating-point equality. Use a tolerance for computed values. The default `std`
feature supplies square-root and trigonometric operations.

See the [Canvas transform sample][transform] and [Composition animation sample][animation].

[transform]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/canvas/transform
[animation]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/composition/animation
