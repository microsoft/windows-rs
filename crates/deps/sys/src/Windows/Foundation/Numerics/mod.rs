#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct Matrix3x2(i32);
#[repr(C)]
pub struct Matrix4x4(i32);
#[repr(C)]
pub struct Plane(i32);
#[repr(C)]
pub struct Quaternion(i32);
#[repr(C)]
pub struct Rational(i32);
#[repr(C)]
pub struct Vector2(i32);
#[repr(C)]
pub struct Vector3(i32);
#[repr(C)]
pub struct Vector4(i32);
