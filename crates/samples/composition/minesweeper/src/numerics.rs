use windows_composition::{Vector2, Vector3};

pub fn from_vector2(value: Vector2, z: f32) -> Vector3 {
    Vector3::new(value.x, value.y, z)
}
