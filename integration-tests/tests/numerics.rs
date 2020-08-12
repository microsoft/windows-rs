use tests::windows::foundation::numerics::*;

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
    let value1 = Vector2 { x: 5.0, y: 50.0 };
    let value2 = Vector2 { x: 15.0, y: 25.0 };
    let expected = Vector2 { x: 20.0, y: 75.0 };

    test_with_same_type!(value1, value2, +, expected);
}

#[test]
fn vector2_sub() {
    let value1 = Vector2 { x: 5.0, y: 50.0 };
    let value2 = Vector2 { x: 15.0, y: 20.0 };
    let expected = Vector2 { x: -10.0, y: 30.0 };

    test_with_same_type!(value1, value2, -, expected);
}

#[test]
fn vector2_div() {
    let value1 = Vector2 { x: 10.0, y: 50.0 };
    let value2 = Vector2 { x: 20.0, y: 25.0 };
    let expected = Vector2 { x: 0.5, y: 2.0 };

    test_with_same_type!(value1, value2, /, expected);

    let value2 = 2.0;
    let expected = Vector2 { x: 5.0, y: 25.0 };

    test_with_scalar!(value1, value2, /, expected);
}

#[test]
fn vector2_mul() {
    let value1 = Vector2 { x: 5.0, y: 50.0 };
    let value2 = Vector2 { x: 15.0, y: 25.0 };
    let expected = Vector2 { x: 75.0, y: 1250.0 };

    test_with_same_type!(value1, value2, *, expected);

    let value2 = 2.5;
    let expected = Vector2 { x: 12.5, y: 125.0 };

    test_with_scalar!(value1, value2, *, expected);
}

#[test]
fn vector3_add() {
    let value1 = Vector3 {
        x: 5.0,
        y: 50.0,
        z: 18.0,
    };
    let value2 = Vector3 {
        x: 15.0,
        y: 25.0,
        z: 3.0,
    };
    let expected = Vector3 {
        x: 20.0,
        y: 75.0,
        z: 21.0,
    };

    test_with_same_type!(value1, value2, +, expected);
}

#[test]
fn vector3_sub() {
    let value1 = Vector3 {
        x: 5.0,
        y: 50.0,
        z: 18.0,
    };
    let value2 = Vector3 {
        x: 15.0,
        y: 20.0,
        z: 3.0,
    };
    let expected = Vector3 {
        x: -10.0,
        y: 30.0,
        z: 15.0,
    };

    test_with_same_type!(value1, value2, -, expected);
}

#[test]
fn vector3_div() {
    let value1 = Vector3 {
        x: 10.0,
        y: 50.0,
        z: 100.0,
    };
    let value2 = Vector3 {
        x: 20.0,
        y: 25.0,
        z: 100.0,
    };
    let expected = Vector3 {
        x: 0.5,
        y: 2.0,
        z: 1.0,
    };

    test_with_same_type!(value1, value2, /, expected);

    let value2 = 2.0;
    let expected = Vector3 {
        x: 5.0,
        y: 25.0,
        z: 50.0,
    };

    test_with_scalar!(value1, value2, /, expected);
}

#[test]
fn vector3_mul() {
    let value1 = Vector3 {
        x: 5.0,
        y: 50.0,
        z: 18.0,
    };
    let value2 = Vector3 {
        x: 15.0,
        y: 25.0,
        z: 3.0,
    };
    let expected = Vector3 {
        x: 75.0,
        y: 1250.0,
        z: 54.0,
    };

    test_with_same_type!(value1, value2, *, expected);

    let value2 = 2.5;
    let expected = Vector3 {
        x: 12.5,
        y: 125.0,
        z: 45.0,
    };

    test_with_scalar!(value1, value2, *, expected);
}

#[test]
fn vector4_add() {
    let value1 = Vector4 {
        x: 5.0,
        y: 50.0,
        z: 18.0,
        w: 63.0,
    };
    let value2 = Vector4 {
        x: 15.0,
        y: 25.0,
        z: 3.0,
        w: 7.0,
    };
    let expected = Vector4 {
        x: 20.0,
        y: 75.0,
        z: 21.0,
        w: 70.0,
    };

    test_with_same_type!(value1, value2, +, expected);
}

#[test]
fn vector4_sub() {
    let value1 = Vector4 {
        x: 5.0,
        y: 50.0,
        z: 18.0,
        w: 63.0,
    };
    let value2 = Vector4 {
        x: 15.0,
        y: 20.0,
        z: 3.0,
        w: 7.0,
    };
    let expected = Vector4 {
        x: -10.0,
        y: 30.0,
        z: 15.0,
        w: 56.0,
    };

    test_with_same_type!(value1, value2, -, expected);
}

#[test]
fn vector4_div() {
    let value1 = Vector4 {
        x: 10.0,
        y: 50.0,
        z: 100.0,
        w: 1.0,
    };
    let value2 = Vector4 {
        x: 20.0,
        y: 25.0,
        z: 100.0,
        w: 10.0,
    };
    let expected = Vector4 {
        x: 0.5,
        y: 2.0,
        z: 1.0,
        w: 0.1,
    };

    test_with_same_type!(value1, value2, /, expected);

    let value2 = 2.0;
    let expected = Vector4 {
        x: 5.0,
        y: 25.0,
        z: 50.0,
        w: 0.5,
    };

    test_with_scalar!(value1, value2, /, expected);
}

#[test]
fn vector4_mul() {
    let value1 = Vector4 {
        x: 5.0,
        y: 50.0,
        z: 18.0,
        w: 2.5,
    };
    let value2 = Vector4 {
        x: 15.0,
        y: 25.0,
        z: 3.0,
        w: 10.0,
    };
    let expected = Vector4 {
        x: 75.0,
        y: 1250.0,
        z: 54.0,
        w: 25.0,
    };

    test_with_same_type!(value1, value2, *, expected);

    let value2 = 2.5;
    let expected = Vector4 {
        x: 12.5,
        y: 125.0,
        z: 45.0,
        w: 6.25,
    };

    test_with_scalar!(value1, value2, *, expected);
}

#[test]
fn matrix3x2_add() {
    let value1 = Matrix3x2 {
        m11: 16.0,
        m12: 15.0,
        m21: 14.0,
        m22: 13.0,
        m31: 12.0,
        m32: 11.0,
    };
    let value2 = Matrix3x2 {
        m11: 10.0,
        m12: 9.0,
        m21: 8.0,
        m22: 7.0,
        m31: 6.0,
        m32: 5.0,
    };
    let expected = Matrix3x2 {
        m11: 26.0,
        m12: 24.0,
        m21: 22.0,
        m22: 20.0,
        m31: 18.0,
        m32: 16.0,
    };

    test_with_same_type!(value1, value2, +, expected);
}

#[test]
fn matrix3x2_sub() {
    let value1 = Matrix3x2 {
        m11: 16.0,
        m12: 15.0,
        m21: 14.0,
        m22: 13.0,
        m31: 12.0,
        m32: 11.0,
    };
    let value2 = Matrix3x2 {
        m11: 32.0,
        m12: 5.0,
        m21: 14.0,
        m22: 8.0,
        m31: 6.0,
        m32: 2.0,
    };
    let expected = Matrix3x2 {
        m11: -16.0,
        m12: 10.0,
        m21: 0.0,
        m22: 5.0,
        m31: 6.0,
        m32: 9.0,
    };

    test_with_same_type!(value1, value2, -, expected);
}

#[test]
fn matrix3x2_mul() {
    let value1 = Matrix3x2 {
        m11: 16.0,
        m12: 15.0,
        m21: 14.0,
        m22: 13.0,
        m31: 12.0,
        m32: 11.0,
    };
    let value2 = Matrix3x2 {
        m11: 32.0,
        m12: 5.0,
        m21: 14.0,
        m22: 8.0,
        m31: 6.0,
        m32: 2.0,
    };
    let expected = Matrix3x2 {
        m11: 722.0,
        m12: 200.0,
        m21: 630.0,
        m22: 174.0,
        m31: 544.0,
        m32: 150.0,
    };

    test_with_same_type!(value1, value2, *, expected);

    let value2 = 2.0;
    let expected = Matrix3x2 {
        m11: 32.0,
        m12: 30.0,
        m21: 28.0,
        m22: 26.0,
        m31: 24.0,
        m32: 22.0,
    };

    test_with_scalar!(value1, value2, *, expected);
}

#[test]
fn matrix4x4_add() {
    let value1 = Matrix4x4 {
        m11: 16.0,
        m12: 15.0,
        m13: 14.0,
        m14: 13.0,
        m21: 12.0,
        m22: 11.0,
        m23: 10.0,
        m24: 9.0,
        m31: 8.0,
        m32: 7.0,
        m33: 6.0,
        m34: 5.0,
        m41: 4.0,
        m42: 3.0,
        m43: 2.0,
        m44: 1.0,
    };
    let value2 = Matrix4x4 {
        m11: 16.0,
        m12: 15.0,
        m13: 14.0,
        m14: 13.0,
        m21: 12.0,
        m22: 11.0,
        m23: 10.0,
        m24: 9.0,
        m31: 8.0,
        m32: 7.0,
        m33: 6.0,
        m34: 5.0,
        m41: 4.0,
        m42: 3.0,
        m43: 2.0,
        m44: 1.0,
    };
    let expected = Matrix4x4 {
        m11: 32.0,
        m12: 30.0,
        m13: 28.0,
        m14: 26.0,
        m21: 24.0,
        m22: 22.0,
        m23: 20.0,
        m24: 18.0,
        m31: 16.0,
        m32: 14.0,
        m33: 12.0,
        m34: 10.0,
        m41: 8.0,
        m42: 6.0,
        m43: 4.0,
        m44: 2.0,
    };

    test_with_same_type!(value1, value2, +, expected);
}

#[test]
fn matrix4x4_sub() {
    let value1 = Matrix4x4 {
        m11: 16.0,
        m12: 15.0,
        m13: 14.0,
        m14: 13.0,
        m21: 12.0,
        m22: 11.0,
        m23: 10.0,
        m24: 9.0,
        m31: 8.0,
        m32: 7.0,
        m33: 6.0,
        m34: 5.0,
        m41: 4.0,
        m42: 3.0,
        m43: 2.0,
        m44: 1.0,
    };
    let value2 = Matrix4x4 {
        m11: 32.0,
        m12: 3.0,
        m13: 25.0,
        m14: 17.0,
        m21: 6.0,
        m22: 4.0,
        m23: 12.0,
        m24: 8.0,
        m31: 22.0,
        m32: 27.0,
        m33: 3.0,
        m34: 15.0,
        m41: 2.0,
        m42: 3.0,
        m43: 1.0,
        m44: 3.0,
    };
    let expected = Matrix4x4 {
        m11: -16.0,
        m12: 12.0,
        m13: -11.0,
        m14: -4.0,
        m21: 6.0,
        m22: 7.0,
        m23: -2.0,
        m24: 1.0,
        m31: -14.0,
        m32: -20.0,
        m33: 3.0,
        m34: -10.0,
        m41: 2.0,
        m42: 0.0,
        m43: 1.0,
        m44: -2.0,
    };

    test_with_same_type!(value1, value2, -, expected);
}

#[test]
fn matrix4x4_mul() {
    let value1 = Matrix4x4 {
        m11: 16.0,
        m12: 15.0,
        m13: 14.0,
        m14: 13.0,
        m21: 12.0,
        m22: 11.0,
        m23: 10.0,
        m24: 9.0,
        m31: 8.0,
        m32: 7.0,
        m33: 6.0,
        m34: 5.0,
        m41: 4.0,
        m42: 3.0,
        m43: 2.0,
        m44: 1.0,
    };
    let value2 = Matrix4x4 {
        m11: 32.0,
        m12: 3.0,
        m13: 25.0,
        m14: 17.0,
        m21: 6.0,
        m22: 4.0,
        m23: 12.0,
        m24: 8.0,
        m31: 22.0,
        m32: 27.0,
        m33: 3.0,
        m34: 15.0,
        m41: 2.0,
        m42: 3.0,
        m43: 1.0,
        m44: 3.0,
    };
    let expected = Matrix4x4 {
        m11: 936.0,
        m12: 525.0,
        m13: 635.0,
        m14: 641.0,
        m21: 688.0,
        m22: 377.0,
        m23: 471.0,
        m24: 469.0,
        m31: 440.0,
        m32: 229.0,
        m33: 307.0,
        m34: 297.0,
        m41: 192.0,
        m42: 81.0,
        m43: 143.0,
        m44: 125.0,
    };

    test_with_same_type!(value1, value2, *, expected);

    let value2 = 2.0;
    let expected = Matrix4x4 {
        m11: 32.0,
        m12: 30.0,
        m13: 28.0,
        m14: 26.0,
        m21: 24.0,
        m22: 22.0,
        m23: 20.0,
        m24: 18.0,
        m31: 16.0,
        m32: 14.0,
        m33: 12.0,
        m34: 10.0,
        m41: 8.0,
        m42: 6.0,
        m43: 4.0,
        m44: 2.0,
    };

    test_with_scalar!(value1, value2, *, expected);
}
