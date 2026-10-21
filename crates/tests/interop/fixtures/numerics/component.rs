use super::*;
use Numerics::*;
use Windows::Foundation::Numerics::{Plane, Quaternion};
use windows_numerics::{Matrix3x2, Matrix4x4, Vector2, Vector3, Vector4};

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "Numerics.Widget").then(activatable::<Widget, Numerics::Widget>)
}

#[implement(Numerics::Widget)]
#[derive(Default)]
struct Widget;

impl IWidget_Impl for Widget_Impl {
    fn NegateVector2(&self, value: &Vector2) -> Result<Vector2> {
        Ok(Vector2 {
            x: -value.x,
            y: -value.y,
        })
    }

    fn NegateVector3(&self, value: &Vector3) -> Result<Vector3> {
        Ok(Vector3 {
            x: -value.x,
            y: -value.y,
            z: -value.z,
        })
    }

    fn NegateVector4(&self, value: &Vector4) -> Result<Vector4> {
        Ok(Vector4 {
            x: -value.x,
            y: -value.y,
            z: -value.z,
            w: -value.w,
        })
    }

    fn NegateMatrix3x2(&self, value: &Matrix3x2) -> Result<Matrix3x2> {
        Ok(Matrix3x2 {
            m11: -value.m11,
            m12: -value.m12,
            m21: -value.m21,
            m22: -value.m22,
            m31: -value.m31,
            m32: -value.m32,
        })
    }

    fn NegateMatrix4x4(&self, value: &Matrix4x4) -> Result<Matrix4x4> {
        Ok(Matrix4x4 {
            m11: -value.m11,
            m12: -value.m12,
            m13: -value.m13,
            m14: -value.m14,
            m21: -value.m21,
            m22: -value.m22,
            m23: -value.m23,
            m24: -value.m24,
            m31: -value.m31,
            m32: -value.m32,
            m33: -value.m33,
            m34: -value.m34,
            m41: -value.m41,
            m42: -value.m42,
            m43: -value.m43,
            m44: -value.m44,
        })
    }

    fn NegateQuaternion(&self, value: &Quaternion) -> Result<Quaternion> {
        Ok(Quaternion {
            X: -value.X,
            Y: -value.Y,
            Z: -value.Z,
            W: -value.W,
        })
    }

    fn NegatePlane(&self, value: &Plane) -> Result<Plane> {
        Ok(Plane {
            Normal: self.NegateVector3(&value.Normal)?,
            D: -value.D,
        })
    }
}
