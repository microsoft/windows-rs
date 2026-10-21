use super::*;
use Numerics::*;
use Windows::Foundation::Numerics::{Plane, Quaternion};
use windows_numerics::{Matrix3x2, Matrix4x4, Vector2, Vector3, Vector4};

pub fn test() -> Result<()> {
    let widget = Widget::new()?;
    assert_eq!(
        widget.NegateVector2(Vector2 { x: 1.25, y: -2.5 })?,
        Vector2 { x: -1.25, y: 2.5 }
    );
    assert_eq!(
        widget.NegateVector3(Vector3 {
            x: 1.25,
            y: -2.5,
            z: 3.75,
        })?,
        Vector3 {
            x: -1.25,
            y: 2.5,
            z: -3.75,
        }
    );
    assert_eq!(
        widget.NegateVector4(Vector4 {
            x: 1.25,
            y: -2.5,
            z: 3.75,
            w: -4.5,
        })?,
        Vector4 {
            x: -1.25,
            y: 2.5,
            z: -3.75,
            w: 4.5,
        }
    );
    assert_eq!(
        widget.NegateMatrix3x2(Matrix3x2 {
            m11: 1.0,
            m12: -2.0,
            m21: 3.0,
            m22: -4.0,
            m31: 5.0,
            m32: -6.0,
        })?,
        Matrix3x2 {
            m11: -1.0,
            m12: 2.0,
            m21: -3.0,
            m22: 4.0,
            m31: -5.0,
            m32: 6.0,
        }
    );
    assert_eq!(
        widget.NegateMatrix4x4(Matrix4x4 {
            m11: 1.0,
            m12: -2.0,
            m13: 3.0,
            m14: -4.0,
            m21: 5.0,
            m22: -6.0,
            m23: 7.0,
            m24: -8.0,
            m31: 9.0,
            m32: -10.0,
            m33: 11.0,
            m34: -12.0,
            m41: 13.0,
            m42: -14.0,
            m43: 15.0,
            m44: -16.0,
        })?,
        Matrix4x4 {
            m11: -1.0,
            m12: 2.0,
            m13: -3.0,
            m14: 4.0,
            m21: -5.0,
            m22: 6.0,
            m23: -7.0,
            m24: 8.0,
            m31: -9.0,
            m32: 10.0,
            m33: -11.0,
            m34: 12.0,
            m41: -13.0,
            m42: 14.0,
            m43: -15.0,
            m44: 16.0,
        }
    );
    assert_eq!(
        widget.NegateQuaternion(Quaternion {
            X: 1.25,
            Y: -2.5,
            Z: 3.75,
            W: -4.5,
        })?,
        Quaternion {
            X: -1.25,
            Y: 2.5,
            Z: -3.75,
            W: 4.5,
        }
    );
    assert_eq!(
        widget.NegatePlane(Plane {
            Normal: Vector3 {
                x: 1.25,
                y: -2.5,
                z: 3.75,
            },
            D: -4.5,
        })?,
        Plane {
            Normal: Vector3 {
                x: -1.25,
                y: 2.5,
                z: -3.75,
            },
            D: 4.5,
        }
    );
    Ok(())
}
