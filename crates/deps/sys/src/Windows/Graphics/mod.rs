#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Graphics_Capture")]
pub mod Capture;
#[cfg(feature = "Graphics_DirectX")]
pub mod DirectX;
#[cfg(feature = "Graphics_Display")]
pub mod Display;
#[cfg(feature = "Graphics_Effects")]
pub mod Effects;
#[cfg(feature = "Graphics_Holographic")]
pub mod Holographic;
#[cfg(feature = "Graphics_Imaging")]
pub mod Imaging;
#[cfg(feature = "Graphics_Printing")]
pub mod Printing;
#[cfg(feature = "Graphics_Printing3D")]
pub mod Printing3D;
#[link(name = "windows")]
extern "system" {
    fn DisplayAdapterId();
    fn DisplayId();
    fn IGeometrySource2D();
    fn PointInt32();
    fn RectInt32();
    fn SizeInt32();
}
