#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn Matrix3x2();
    fn Matrix4x4();
    fn Plane();
    fn Quaternion();
    fn Rational();
    fn Vector2();
    fn Vector3();
    fn Vector4();
}
