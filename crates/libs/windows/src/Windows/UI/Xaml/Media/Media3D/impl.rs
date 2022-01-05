#[cfg(feature = "implement_exclusive")]
pub trait ICompositeTransform3DImpl: Sized {
    fn CenterX(&self) -> ::windows::core::Result<f64>;
    fn SetCenterX(&self, value: f64) -> ::windows::core::Result<()>;
    fn CenterY(&self) -> ::windows::core::Result<f64>;
    fn SetCenterY(&self, value: f64) -> ::windows::core::Result<()>;
    fn CenterZ(&self) -> ::windows::core::Result<f64>;
    fn SetCenterZ(&self, value: f64) -> ::windows::core::Result<()>;
    fn RotationX(&self) -> ::windows::core::Result<f64>;
    fn SetRotationX(&self, value: f64) -> ::windows::core::Result<()>;
    fn RotationY(&self) -> ::windows::core::Result<f64>;
    fn SetRotationY(&self, value: f64) -> ::windows::core::Result<()>;
    fn RotationZ(&self) -> ::windows::core::Result<f64>;
    fn SetRotationZ(&self, value: f64) -> ::windows::core::Result<()>;
    fn ScaleX(&self) -> ::windows::core::Result<f64>;
    fn SetScaleX(&self, value: f64) -> ::windows::core::Result<()>;
    fn ScaleY(&self) -> ::windows::core::Result<f64>;
    fn SetScaleY(&self, value: f64) -> ::windows::core::Result<()>;
    fn ScaleZ(&self) -> ::windows::core::Result<f64>;
    fn SetScaleZ(&self, value: f64) -> ::windows::core::Result<()>;
    fn TranslateX(&self) -> ::windows::core::Result<f64>;
    fn SetTranslateX(&self, value: f64) -> ::windows::core::Result<()>;
    fn TranslateY(&self) -> ::windows::core::Result<f64>;
    fn SetTranslateY(&self, value: f64) -> ::windows::core::Result<()>;
    fn TranslateZ(&self) -> ::windows::core::Result<f64>;
    fn SetTranslateZ(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositeTransform3DStaticsImpl: Sized {
    fn CenterXProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CenterYProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CenterZProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RotationXProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RotationYProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RotationZProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ScaleXProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ScaleYProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ScaleZProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TranslateXProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TranslateYProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TranslateZProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMatrix3DHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMatrix3DHelperStaticsImpl: Sized {
    fn Identity(&self) -> ::windows::core::Result<Matrix3D>;
    fn Multiply(&self, matrix1: &Matrix3D, matrix2: &Matrix3D) -> ::windows::core::Result<Matrix3D>;
    fn FromElements(&self, m11: f64, m12: f64, m13: f64, m14: f64, m21: f64, m22: f64, m23: f64, m24: f64, m31: f64, m32: f64, m33: f64, m34: f64, offsetx: f64, offsety: f64, offsetz: f64, m44: f64) -> ::windows::core::Result<Matrix3D>;
    fn GetHasInverse(&self, target: &Matrix3D) -> ::windows::core::Result<bool>;
    fn GetIsIdentity(&self, target: &Matrix3D) -> ::windows::core::Result<bool>;
    fn Invert(&self, target: &Matrix3D) -> ::windows::core::Result<Matrix3D>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPerspectiveTransform3DImpl: Sized {
    fn Depth(&self) -> ::windows::core::Result<f64>;
    fn SetDepth(&self, value: f64) -> ::windows::core::Result<()>;
    fn OffsetX(&self) -> ::windows::core::Result<f64>;
    fn SetOffsetX(&self, value: f64) -> ::windows::core::Result<()>;
    fn OffsetY(&self) -> ::windows::core::Result<f64>;
    fn SetOffsetY(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPerspectiveTransform3DStaticsImpl: Sized {
    fn DepthProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn OffsetXProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn OffsetYProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITransform3DImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ITransform3DFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Transform3D>;
}
