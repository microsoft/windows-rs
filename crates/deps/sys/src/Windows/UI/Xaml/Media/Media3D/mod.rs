#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CompositeTransform3D(i32);
pub struct ICompositeTransform3D(pub *mut ::core::ffi::c_void);
pub struct ICompositeTransform3DStatics(pub *mut ::core::ffi::c_void);
pub struct IMatrix3DHelper(pub *mut ::core::ffi::c_void);
pub struct IMatrix3DHelperStatics(pub *mut ::core::ffi::c_void);
pub struct IPerspectiveTransform3D(pub *mut ::core::ffi::c_void);
pub struct IPerspectiveTransform3DStatics(pub *mut ::core::ffi::c_void);
pub struct ITransform3D(pub *mut ::core::ffi::c_void);
pub struct ITransform3DFactory(pub *mut ::core::ffi::c_void);
pub struct Matrix3D(i32);
pub struct Matrix3DHelper(i32);
pub struct PerspectiveTransform3D(i32);
pub struct Transform3D(i32);
