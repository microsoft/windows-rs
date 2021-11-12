#![allow(non_snake_case, non_camel_case_types)]
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
pub struct Matrix3D(i32);
#[repr(transparent)]
pub struct Matrix3DHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerspectiveTransform3D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Transform3D(pub *mut ::core::ffi::c_void);
