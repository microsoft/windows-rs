#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CompositeTransform3D();
    fn ICompositeTransform3D();
    fn ICompositeTransform3DStatics();
    fn IMatrix3DHelper();
    fn IMatrix3DHelperStatics();
    fn IPerspectiveTransform3D();
    fn IPerspectiveTransform3DStatics();
    fn ITransform3D();
    fn ITransform3DFactory();
    fn Matrix3D();
    fn Matrix3DHelper();
    fn PerspectiveTransform3D();
    fn Transform3D();
}
