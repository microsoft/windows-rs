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

#[cfg(windows)]
mod d2d1_comparison {
    use super::*;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-6
    }

    fn assert_matrix3x2_approx_eq(rust: &Matrix3x2, d2d1: &Matrix3x2) {
        assert!(
            approx_eq(rust.M11, d2d1.M11),
            "M11: {} != {}",
            rust.M11,
            d2d1.M11
        );
        assert!(
            approx_eq(rust.M12, d2d1.M12),
            "M12: {} != {}",
            rust.M12,
            d2d1.M12
        );
        assert!(
            approx_eq(rust.M21, d2d1.M21),
            "M21: {} != {}",
            rust.M21,
            d2d1.M21
        );
        assert!(
            approx_eq(rust.M22, d2d1.M22),
            "M22: {} != {}",
            rust.M22,
            d2d1.M22
        );
        assert!(
            approx_eq(rust.M31, d2d1.M31),
            "M31: {} != {}",
            rust.M31,
            d2d1.M31
        );
        assert!(
            approx_eq(rust.M32, d2d1.M32),
            "M32: {} != {}",
            rust.M32,
            d2d1.M32
        );
    }

    #[test]
    fn rotation_around_matches_d2d1() {
        windows_link::link!("d2d1.dll" "system" fn D2D1MakeRotateMatrix(angle: f32, center: Vector2, matrix: *mut Matrix3x2));
        for (angle, cx, cy) in [
            (0.0f32, 0.0f32, 0.0f32),
            (45.0, 1.0, 2.0),
            (90.0, -3.0, 5.0),
            (180.0, 0.5, -0.5),
        ] {
            let rust = Matrix3x2::rotation_around(angle, Vector2::new(cx, cy));
            let mut d2d1 = Matrix3x2::default();
            unsafe {
                D2D1MakeRotateMatrix(angle, Vector2::new(cx, cy), &mut d2d1);
            }
            assert_matrix3x2_approx_eq(&rust, &d2d1);
        }
    }

    #[test]
    fn skew_around_matches_d2d1() {
        windows_link::link!("d2d1.dll" "system" fn D2D1MakeSkewMatrix(angle_x: f32, angle_y: f32, center: Vector2, matrix: *mut Matrix3x2));
        for (ax, ay, cx, cy) in [
            (0.0f32, 0.0f32, 0.0f32, 0.0f32),
            (15.0, 30.0, 1.0, 2.0),
            (45.0, 0.0, -1.0, 3.0),
        ] {
            let rust = Matrix3x2::skew_around(ax, ay, Vector2::new(cx, cy));
            let mut d2d1 = Matrix3x2::default();
            unsafe {
                D2D1MakeSkewMatrix(ax, ay, Vector2::new(cx, cy), &mut d2d1);
            }
            assert_matrix3x2_approx_eq(&rust, &d2d1);
        }
    }

    #[test]
    fn rotation_y_matches_d2d1_sincos() {
        windows_link::link!("d2d1.dll" "system" fn D2D1SinCos(angle: f32, sin: *mut f32, cos: *mut f32));
        for degree in [0.0f32, 45.0, 90.0, 180.0, 270.0] {
            let rust = Matrix4x4::rotation_y(degree);
            let mut d2d1_sin = 0.0f32;
            let mut d2d1_cos = 0.0f32;
            unsafe {
                D2D1SinCos(degree.to_radians(), &mut d2d1_sin, &mut d2d1_cos);
            }
            assert!(
                approx_eq(rust.M11, d2d1_cos),
                "rotation_y({degree}) M11: {} != {}",
                rust.M11,
                d2d1_cos
            );
            assert!(
                approx_eq(rust.M13, -d2d1_sin),
                "rotation_y({degree}) M13: {} != {}",
                rust.M13,
                -d2d1_sin
            );
            assert!(
                approx_eq(rust.M31, d2d1_sin),
                "rotation_y({degree}) M31: {} != {}",
                rust.M31,
                d2d1_sin
            );
            assert!(
                approx_eq(rust.M33, d2d1_cos),
                "rotation_y({degree}) M33: {} != {}",
                rust.M33,
                d2d1_cos
            );
        }
    }
}
