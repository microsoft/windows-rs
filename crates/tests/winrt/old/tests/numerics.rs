use windows_numerics::*;

macro_rules! test_with_same_type {
    ($value1:ident, $value2:ident, $op:tt, $expected:ident) => {
        let result = $value1.clone() $op $value2.clone();
        assert_eq!(result, $expected);

        let result = $value1.clone() $op &$value2;
        assert_eq!(result, $expected);

        let result = &$value1 $op $value2.clone();
        assert_eq!(result, $expected);

        let result = &$value1 $op &$value2;
        assert_eq!(result, $expected);
    }
}

macro_rules! test_with_scalar {
    ($value1:ident, $value2:ident, $op:tt, $expected:ident) => {
        let result = $value1.clone() $op $value2;
        assert_eq!(result, $expected);

        let result = &$value1 $op $value2;
        assert_eq!(result, $expected);
    }
}

#[test]
fn vector2_add() {
    let value1 = Vector2 { X: 5.0, Y: 50.0 };
    let value2 = Vector2 { X: 15.0, Y: 25.0 };
    let expected = Vector2 { X: 20.0, Y: 75.0 };

    test_with_same_type!(value1, value2, +, expected);
}

#[test]
fn vector2_sub() {
    let value1 = Vector2 { X: 5.0, Y: 50.0 };
    let value2 = Vector2 { X: 15.0, Y: 20.0 };
    let expected = Vector2 { X: -10.0, Y: 30.0 };

    test_with_same_type!(value1, value2, -, expected);
}

#[test]
fn vector2_div() {
    let value1 = Vector2 { X: 10.0, Y: 50.0 };
    let value2 = Vector2 { X: 20.0, Y: 25.0 };
    let expected = Vector2 { X: 0.5, Y: 2.0 };

    test_with_same_type!(value1, value2, /, expected);

    let value2 = 2.0;
    let expected = Vector2 { X: 5.0, Y: 25.0 };

    test_with_scalar!(value1, value2, /, expected);
}

#[test]
fn vector2_mul() {
    let value1 = Vector2 { X: 5.0, Y: 50.0 };
    let value2 = Vector2 { X: 15.0, Y: 25.0 };
    let expected = Vector2 { X: 75.0, Y: 1250.0 };

    test_with_same_type!(value1, value2, *, expected);

    let value2 = 2.5;
    let expected = Vector2 { X: 12.5, Y: 125.0 };

    test_with_scalar!(value1, value2, *, expected);
}

#[test]
fn vector3_add() {
    let value1 = Vector3 {
        X: 5.0,
        Y: 50.0,
        Z: 18.0,
    };
    let value2 = Vector3 {
        X: 15.0,
        Y: 25.0,
        Z: 3.0,
    };
    let expected = Vector3 {
        X: 20.0,
        Y: 75.0,
        Z: 21.0,
    };

    test_with_same_type!(value1, value2, +, expected);
}

#[test]
fn vector3_sub() {
    let value1 = Vector3 {
        X: 5.0,
        Y: 50.0,
        Z: 18.0,
    };
    let value2 = Vector3 {
        X: 15.0,
        Y: 20.0,
        Z: 3.0,
    };
    let expected = Vector3 {
        X: -10.0,
        Y: 30.0,
        Z: 15.0,
    };

    test_with_same_type!(value1, value2, -, expected);
}

#[test]
fn vector3_div() {
    let value1 = Vector3 {
        X: 10.0,
        Y: 50.0,
        Z: 100.0,
    };
    let value2 = Vector3 {
        X: 20.0,
        Y: 25.0,
        Z: 100.0,
    };
    let expected = Vector3 {
        X: 0.5,
        Y: 2.0,
        Z: 1.0,
    };

    test_with_same_type!(value1, value2, /, expected);

    let value2 = 2.0;
    let expected = Vector3 {
        X: 5.0,
        Y: 25.0,
        Z: 50.0,
    };

    test_with_scalar!(value1, value2, /, expected);
}

#[test]
fn vector3_mul() {
    let value1 = Vector3 {
        X: 5.0,
        Y: 50.0,
        Z: 18.0,
    };
    let value2 = Vector3 {
        X: 15.0,
        Y: 25.0,
        Z: 3.0,
    };
    let expected = Vector3 {
        X: 75.0,
        Y: 1250.0,
        Z: 54.0,
    };

    test_with_same_type!(value1, value2, *, expected);

    let value2 = 2.5;
    let expected = Vector3 {
        X: 12.5,
        Y: 125.0,
        Z: 45.0,
    };

    test_with_scalar!(value1, value2, *, expected);
}

#[test]
fn vector4_add() {
    let value1 = Vector4 {
        X: 5.0,
        Y: 50.0,
        Z: 18.0,
        W: 63.0,
    };
    let value2 = Vector4 {
        X: 15.0,
        Y: 25.0,
        Z: 3.0,
        W: 7.0,
    };
    let expected = Vector4 {
        X: 20.0,
        Y: 75.0,
        Z: 21.0,
        W: 70.0,
    };

    test_with_same_type!(value1, value2, +, expected);
}

#[test]
fn vector4_sub() {
    let value1 = Vector4 {
        X: 5.0,
        Y: 50.0,
        Z: 18.0,
        W: 63.0,
    };
    let value2 = Vector4 {
        X: 15.0,
        Y: 20.0,
        Z: 3.0,
        W: 7.0,
    };
    let expected = Vector4 {
        X: -10.0,
        Y: 30.0,
        Z: 15.0,
        W: 56.0,
    };

    test_with_same_type!(value1, value2, -, expected);
}

#[test]
fn vector4_div() {
    let value1 = Vector4 {
        X: 10.0,
        Y: 50.0,
        Z: 100.0,
        W: 1.0,
    };
    let value2 = Vector4 {
        X: 20.0,
        Y: 25.0,
        Z: 100.0,
        W: 10.0,
    };
    let expected = Vector4 {
        X: 0.5,
        Y: 2.0,
        Z: 1.0,
        W: 0.1,
    };

    test_with_same_type!(value1, value2, /, expected);

    let value2 = 2.0;
    let expected = Vector4 {
        X: 5.0,
        Y: 25.0,
        Z: 50.0,
        W: 0.5,
    };

    test_with_scalar!(value1, value2, /, expected);
}

#[test]
fn vector4_mul() {
    let value1 = Vector4 {
        X: 5.0,
        Y: 50.0,
        Z: 18.0,
        W: 2.5,
    };
    let value2 = Vector4 {
        X: 15.0,
        Y: 25.0,
        Z: 3.0,
        W: 10.0,
    };
    let expected = Vector4 {
        X: 75.0,
        Y: 1250.0,
        Z: 54.0,
        W: 25.0,
    };

    test_with_same_type!(value1, value2, *, expected);

    let value2 = 2.5;
    let expected = Vector4 {
        X: 12.5,
        Y: 125.0,
        Z: 45.0,
        W: 6.25,
    };

    test_with_scalar!(value1, value2, *, expected);
}

#[test]
fn matrix3x2_add() {
    let value1 = Matrix3x2 {
        M11: 16.0,
        M12: 15.0,
        M21: 14.0,
        M22: 13.0,
        M31: 12.0,
        M32: 11.0,
    };
    let value2 = Matrix3x2 {
        M11: 10.0,
        M12: 9.0,
        M21: 8.0,
        M22: 7.0,
        M31: 6.0,
        M32: 5.0,
    };
    let expected = Matrix3x2 {
        M11: 26.0,
        M12: 24.0,
        M21: 22.0,
        M22: 20.0,
        M31: 18.0,
        M32: 16.0,
    };

    test_with_same_type!(value1, value2, +, expected);
}

#[test]
fn matrix3x2_sub() {
    let value1 = Matrix3x2 {
        M11: 16.0,
        M12: 15.0,
        M21: 14.0,
        M22: 13.0,
        M31: 12.0,
        M32: 11.0,
    };
    let value2 = Matrix3x2 {
        M11: 32.0,
        M12: 5.0,
        M21: 14.0,
        M22: 8.0,
        M31: 6.0,
        M32: 2.0,
    };
    let expected = Matrix3x2 {
        M11: -16.0,
        M12: 10.0,
        M21: 0.0,
        M22: 5.0,
        M31: 6.0,
        M32: 9.0,
    };

    test_with_same_type!(value1, value2, -, expected);
}

#[test]
fn matrix3x2_mul() {
    let value1 = Matrix3x2 {
        M11: 16.0,
        M12: 15.0,
        M21: 14.0,
        M22: 13.0,
        M31: 12.0,
        M32: 11.0,
    };
    let value2 = Matrix3x2 {
        M11: 32.0,
        M12: 5.0,
        M21: 14.0,
        M22: 8.0,
        M31: 6.0,
        M32: 2.0,
    };
    let expected = Matrix3x2 {
        M11: 722.0,
        M12: 200.0,
        M21: 630.0,
        M22: 174.0,
        M31: 544.0,
        M32: 150.0,
    };

    test_with_same_type!(value1, value2, *, expected);

    let value2 = 2.0;
    let expected = Matrix3x2 {
        M11: 32.0,
        M12: 30.0,
        M21: 28.0,
        M22: 26.0,
        M31: 24.0,
        M32: 22.0,
    };

    test_with_scalar!(value1, value2, *, expected);
}

#[test]
fn matrix4x4_add() {
    let value1 = Matrix4x4 {
        M11: 16.0,
        M12: 15.0,
        M13: 14.0,
        M14: 13.0,
        M21: 12.0,
        M22: 11.0,
        M23: 10.0,
        M24: 9.0,
        M31: 8.0,
        M32: 7.0,
        M33: 6.0,
        M34: 5.0,
        M41: 4.0,
        M42: 3.0,
        M43: 2.0,
        M44: 1.0,
    };
    let value2 = Matrix4x4 {
        M11: 16.0,
        M12: 15.0,
        M13: 14.0,
        M14: 13.0,
        M21: 12.0,
        M22: 11.0,
        M23: 10.0,
        M24: 9.0,
        M31: 8.0,
        M32: 7.0,
        M33: 6.0,
        M34: 5.0,
        M41: 4.0,
        M42: 3.0,
        M43: 2.0,
        M44: 1.0,
    };
    let expected = Matrix4x4 {
        M11: 32.0,
        M12: 30.0,
        M13: 28.0,
        M14: 26.0,
        M21: 24.0,
        M22: 22.0,
        M23: 20.0,
        M24: 18.0,
        M31: 16.0,
        M32: 14.0,
        M33: 12.0,
        M34: 10.0,
        M41: 8.0,
        M42: 6.0,
        M43: 4.0,
        M44: 2.0,
    };

    test_with_same_type!(value1, value2, +, expected);
}

#[test]
fn matrix4x4_sub() {
    let value1 = Matrix4x4 {
        M11: 16.0,
        M12: 15.0,
        M13: 14.0,
        M14: 13.0,
        M21: 12.0,
        M22: 11.0,
        M23: 10.0,
        M24: 9.0,
        M31: 8.0,
        M32: 7.0,
        M33: 6.0,
        M34: 5.0,
        M41: 4.0,
        M42: 3.0,
        M43: 2.0,
        M44: 1.0,
    };
    let value2 = Matrix4x4 {
        M11: 32.0,
        M12: 3.0,
        M13: 25.0,
        M14: 17.0,
        M21: 6.0,
        M22: 4.0,
        M23: 12.0,
        M24: 8.0,
        M31: 22.0,
        M32: 27.0,
        M33: 3.0,
        M34: 15.0,
        M41: 2.0,
        M42: 3.0,
        M43: 1.0,
        M44: 3.0,
    };
    let expected = Matrix4x4 {
        M11: -16.0,
        M12: 12.0,
        M13: -11.0,
        M14: -4.0,
        M21: 6.0,
        M22: 7.0,
        M23: -2.0,
        M24: 1.0,
        M31: -14.0,
        M32: -20.0,
        M33: 3.0,
        M34: -10.0,
        M41: 2.0,
        M42: 0.0,
        M43: 1.0,
        M44: -2.0,
    };

    test_with_same_type!(value1, value2, -, expected);
}

#[test]
fn matrix4x4_mul() {
    let value1 = Matrix4x4 {
        M11: 16.0,
        M12: 15.0,
        M13: 14.0,
        M14: 13.0,
        M21: 12.0,
        M22: 11.0,
        M23: 10.0,
        M24: 9.0,
        M31: 8.0,
        M32: 7.0,
        M33: 6.0,
        M34: 5.0,
        M41: 4.0,
        M42: 3.0,
        M43: 2.0,
        M44: 1.0,
    };
    let value2 = Matrix4x4 {
        M11: 32.0,
        M12: 3.0,
        M13: 25.0,
        M14: 17.0,
        M21: 6.0,
        M22: 4.0,
        M23: 12.0,
        M24: 8.0,
        M31: 22.0,
        M32: 27.0,
        M33: 3.0,
        M34: 15.0,
        M41: 2.0,
        M42: 3.0,
        M43: 1.0,
        M44: 3.0,
    };
    let expected = Matrix4x4 {
        M11: 936.0,
        M12: 525.0,
        M13: 635.0,
        M14: 641.0,
        M21: 688.0,
        M22: 377.0,
        M23: 471.0,
        M24: 469.0,
        M31: 440.0,
        M32: 229.0,
        M33: 307.0,
        M34: 297.0,
        M41: 192.0,
        M42: 81.0,
        M43: 143.0,
        M44: 125.0,
    };

    test_with_same_type!(value1, value2, *, expected);

    let value2 = 2.0;
    let expected = Matrix4x4 {
        M11: 32.0,
        M12: 30.0,
        M13: 28.0,
        M14: 26.0,
        M21: 24.0,
        M22: 22.0,
        M23: 20.0,
        M24: 18.0,
        M31: 16.0,
        M32: 14.0,
        M33: 12.0,
        M34: 10.0,
        M41: 8.0,
        M42: 6.0,
        M43: 4.0,
        M44: 2.0,
    };

    test_with_scalar!(value1, value2, *, expected);
}
