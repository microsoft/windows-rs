winrt::import!(
    dependencies
        nuget: Microsoft.Windows.SDK.Contracts
    types
        windows::foundation::numerics::*
);

use windows::foundation::numerics::*;

#[test]
fn vector2_add() {
    let value1 = Vector2 { x: 5.0, y: 50.0 };
    let value2 = Vector2 { x: 15.0, y: 25.0 };
    let expected = Vector2 { x: 20.0, y: 75.0 };

    let result = value1.clone() + value2.clone();
    assert_eq!(result, expected);

    let result = value1.clone() + &value2;
    assert_eq!(result, expected);

    let result = &value1 + value2.clone();
    assert_eq!(result, expected);

    let result = &value1 + &value2;
    assert_eq!(result, expected);
}

#[test]
fn vector2_sub() {
    let value1 = Vector2 { x: 5.0, y: 50.0 };
    let value2 = Vector2 { x: 15.0, y: 20.0 };
    let expected = Vector2 { x: -10.0, y: 30.0 };

    let result = value1.clone() - value2.clone();
    assert_eq!(result, expected);

    let result = value1.clone() - &value2;
    assert_eq!(result, expected);

    let result = &value1 - value2.clone();
    assert_eq!(result, expected);

    let result = &value1 - &value2;
    assert_eq!(result, expected);
}

#[test]
fn vector2_div() {
    let value1 = Vector2 { x: 10.0, y: 50.0 };
    let value2 = Vector2 { x: 20.0, y: 25.0 };
    let expected = Vector2 { x: 0.5, y: 2.0 };

    let result = value1.clone() / value2.clone();
    assert_eq!(result, expected);

    let result = value1.clone() / &value2;
    assert_eq!(result, expected);

    let result = &value1 / value2.clone();
    assert_eq!(result, expected);

    let result = &value1 / &value2;
    assert_eq!(result, expected);

    let value2 = 2.0;
    let expected = Vector2 { x: 5.0, y: 25.0 };

    let result = value1.clone() / value2;
    assert_eq!(result, expected);

    let result = &value1 / value2;
    assert_eq!(result, expected);
}

#[test]
fn vector2_mul() {
    let value1 = Vector2 { x: 5.0, y: 50.0 };
    let value2 = Vector2 { x: 15.0, y: 25.0 };
    let expected = Vector2 { x: 75.0, y: 1250.0 };

    let result = value1.clone() * value2.clone();
    assert_eq!(result, expected);

    let result = value1.clone() * &value2;
    assert_eq!(result, expected);

    let result = &value1 * value2.clone();
    assert_eq!(result, expected);

    let result = &value1 * &value2;
    assert_eq!(result, expected);

    let value2 = 2.5;
    let expected = Vector2 { x: 12.5, y: 125.0 };

    let result = value1.clone() * value2;
    assert_eq!(result, expected);

    let result = &value1 * value2;
    assert_eq!(result, expected);
}

#[test]
fn vector3_add() {
    let value1 = Vector3 { x: 5.0, y: 50.0, z: 18.0 };
    let value2 = Vector3 { x: 15.0, y: 25.0, z: 3.0 };
    let expected = Vector3 { x: 20.0, y: 75.0, z: 21.0 };

    let result = value1.clone() + value2.clone();
    assert_eq!(result, expected);

    let result = value1.clone() + &value2;
    assert_eq!(result, expected);

    let result = &value1 + value2.clone();
    assert_eq!(result, expected);

    let result = &value1 + &value2;
    assert_eq!(result, expected);
}

#[test]
fn vector3_sub() {
    let value1 = Vector3 { x: 5.0, y: 50.0, z: 18.0 };
    let value2 = Vector3 { x: 15.0, y: 20.0, z: 3.0 };
    let expected = Vector3 { x: -10.0, y: 30.0, z: 15.0 };

    let result = value1.clone() - value2.clone();
    assert_eq!(result, expected);

    let result = value1.clone() - &value2;
    assert_eq!(result, expected);

    let result = &value1 - value2.clone();
    assert_eq!(result, expected);

    let result = &value1 - &value2;
    assert_eq!(result, expected);
}

#[test]
fn vector3_div() {
    let value1 = Vector3 { x: 10.0, y: 50.0, z: 100.0 };
    let value2 = Vector3 { x: 20.0, y: 25.0, z: 100.0 };
    let expected = Vector3 { x: 0.5, y: 2.0, z: 1.0 };

    let result = value1.clone() / value2.clone();
    assert_eq!(result, expected);

    let result = value1.clone() / &value2;
    assert_eq!(result, expected);

    let result = &value1 / value2.clone();
    assert_eq!(result, expected);

    let result = &value1 / &value2;
    assert_eq!(result, expected);

    let value2 = 2.0;
    let expected = Vector3 { x: 5.0, y: 25.0, z: 50.0 };

    let result = value1.clone() / value2;
    assert_eq!(result, expected);

    let result = &value1 / value2;
    assert_eq!(result, expected);
}

#[test]
fn vector3_mul() {
    let value1 = Vector3 { x: 5.0, y: 50.0, z: 18.0 };
    let value2 = Vector3 { x: 15.0, y: 25.0, z: 3.0 };
    let expected = Vector3 { x: 75.0, y: 1250.0, z: 54.0 };

    let result = value1.clone() * value2.clone();
    assert_eq!(result, expected);

    let result = value1.clone() * &value2;
    assert_eq!(result, expected);

    let result = &value1 * value2.clone();
    assert_eq!(result, expected);

    let result = &value1 * &value2;
    assert_eq!(result, expected);

    let value2 = 2.5;
    let expected = Vector3 { x: 12.5, y: 125.0, z: 45.0 };

    let result = value1.clone() * value2;
    assert_eq!(result, expected);

    let result = &value1 * value2;
    assert_eq!(result, expected);
}

#[test]
fn vector4_add() {
    let value1 = Vector4 { x: 5.0, y: 50.0, z: 18.0, w: 63.0 };
    let value2 = Vector4 { x: 15.0, y: 25.0, z: 3.0, w: 7.0 };
    let expected = Vector4 { x: 20.0, y: 75.0, z: 21.0, w: 70.0 };

    let result = value1.clone() + value2.clone();
    assert_eq!(result, expected);

    let result = value1.clone() + &value2;
    assert_eq!(result, expected);

    let result = &value1 + value2.clone();
    assert_eq!(result, expected);

    let result = &value1 + &value2;
    assert_eq!(result, expected);
}

#[test]
fn vector4_sub() {
    let value1 = Vector4 { x: 5.0, y: 50.0, z: 18.0, w: 63.0 };
    let value2 = Vector4 { x: 15.0, y: 20.0, z: 3.0, w: 7.0 };
    let expected = Vector4 { x: -10.0, y: 30.0, z: 15.0, w: 56.0 };

    let result = value1.clone() - value2.clone();
    assert_eq!(result, expected);

    let result = value1.clone() - &value2;
    assert_eq!(result, expected);

    let result = &value1 - value2.clone();
    assert_eq!(result, expected);

    let result = &value1 - &value2;
    assert_eq!(result, expected);
}

#[test]
fn vector4_div() {
    let value1 = Vector4 { x: 10.0, y: 50.0, z: 100.0, w: 1.0 };
    let value2 = Vector4 { x: 20.0, y: 25.0, z: 100.0, w: 10.0 };
    let expected = Vector4 { x: 0.5, y: 2.0, z: 1.0, w: 0.1 };

    let result = value1.clone() / value2.clone();
    assert_eq!(result, expected);

    let result = value1.clone() / &value2;
    assert_eq!(result, expected);

    let result = &value1 / value2.clone();
    assert_eq!(result, expected);

    let result = &value1 / &value2;
    assert_eq!(result, expected);

    let value2 = 2.0;
    let expected = Vector4 { x: 5.0, y: 25.0, z: 50.0, w: 0.5 };

    let result = value1.clone() / value2;
    assert_eq!(result, expected);

    let result = &value1 / value2;
    assert_eq!(result, expected);
}

#[test]
fn vector4_mul() {
    let value1 = Vector4 { x: 5.0, y: 50.0, z: 18.0, w: 2.5 };
    let value2 = Vector4 { x: 15.0, y: 25.0, z: 3.0, w: 10.0 };
    let expected = Vector4 { x: 75.0, y: 1250.0, z: 54.0, w: 25.0 };

    let result = value1.clone() * value2.clone();
    assert_eq!(result, expected);

    let result = value1.clone() * &value2;
    assert_eq!(result, expected);

    let result = &value1 * value2.clone();
    assert_eq!(result, expected);

    let result = &value1 * &value2;
    assert_eq!(result, expected);

    let value2 = 2.5;
    let expected = Vector4 { x: 12.5, y: 125.0, z: 45.0, w: 6.25 };

    let result = value1.clone() * value2;
    assert_eq!(result, expected);

    let result = &value1 * value2;
    assert_eq!(result, expected);
}