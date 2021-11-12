#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CompositeTransform3D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositeTransform3D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositeTransform3DStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMatrix3DHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMatrix3DHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerspectiveTransform3D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerspectiveTransform3DStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransform3D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransform3DFactory(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct Matrix3D {
    pub M11: f64,
    pub M12: f64,
    pub M13: f64,
    pub M14: f64,
    pub M21: f64,
    pub M22: f64,
    pub M23: f64,
    pub M24: f64,
    pub M31: f64,
    pub M32: f64,
    pub M33: f64,
    pub M34: f64,
    pub OffsetX: f64,
    pub OffsetY: f64,
    pub OffsetZ: f64,
    pub M44: f64,
}
impl ::core::marker::Copy for Matrix3D {}
impl ::core::clone::Clone for Matrix3D {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Matrix3DHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerspectiveTransform3D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Transform3D(pub *mut ::core::ffi::c_void);
