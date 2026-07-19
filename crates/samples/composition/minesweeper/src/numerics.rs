//! Small numeric helpers shared by the sample.

use windows_composition::{Vector2, Vector3};

/// Widens a `Vector2` into a `Vector3` by appending a `z` component.
pub fn from_vector2(value: Vector2, z: f32) -> Vector3 {
    Vector3::new(value.x, value.y, z)
}
