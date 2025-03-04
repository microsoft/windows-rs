use windows_numerics::{Matrix3x2, Vector2};

#[test]
fn rotation_test() {
    _ = Matrix3x2::rotation_around(0.0, Vector2::new(1.0, 2.0));
}

#[test]
fn skew_test() {
    _ = Matrix3x2::skew_around(2.0, 1.0, Vector2::new(1.0, 2.0));
}
