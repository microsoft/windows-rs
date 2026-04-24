use windows_numerics::{Matrix3x2, Matrix4x4, Vector2};

#[test]
fn rotation_test() {
    _ = Matrix3x2::rotation_around(0.0, Vector2::new(1.0, 2.0));
}

#[test]
fn rotation_around_zero_yields_identity() {
    // 0° rotation around any center is the identity matrix
    let m = Matrix3x2::rotation_around(0.0, Vector2::new(3.0, 5.0));
    assert_eq!(m, Matrix3x2::identity());
}

#[test]
fn skew_test() {
    _ = Matrix3x2::skew_around(2.0, 1.0, Vector2::new(1.0, 2.0));
}

#[test]
fn skew_around_zero_yields_identity() {
    // 0°/0° skew around any center is the identity matrix (tan(0) = 0)
    let m = Matrix3x2::skew_around(0.0, 0.0, Vector2::new(3.0, 5.0));
    assert_eq!(m, Matrix3x2::identity());
}

#[test]
fn rotation_y_zero_yields_identity() {
    // 0° rotation_y is the identity matrix (sin(0)=0, cos(0)=1)
    let m = Matrix4x4::rotation_y(0.0);
    assert_eq!(m.M11, 1.0);
    assert_eq!(m.M13, 0.0);
    assert_eq!(m.M22, 1.0);
    assert_eq!(m.M31, 0.0);
    assert_eq!(m.M33, 1.0);
    assert_eq!(m.M44, 1.0);
    assert_eq!(m.M12, 0.0);
    assert_eq!(m.M14, 0.0);
    assert_eq!(m.M21, 0.0);
    assert_eq!(m.M23, 0.0);
    assert_eq!(m.M24, 0.0);
    assert_eq!(m.M32, 0.0);
    assert_eq!(m.M34, 0.0);
    assert_eq!(m.M41, 0.0);
    assert_eq!(m.M42, 0.0);
    assert_eq!(m.M43, 0.0);
}
