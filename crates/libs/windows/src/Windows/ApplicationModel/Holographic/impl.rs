#[cfg(feature = "implement_exclusive")]
pub trait IHolographicKeyboardImpl: Sized {
    fn SetPlacementOverride(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>, topcenterposition: &super::super::Foundation::Numerics::Vector3, orientation: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn SetPlacementOverrideWithMaxSize(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>, topcenterposition: &super::super::Foundation::Numerics::Vector3, orientation: &super::super::Foundation::Numerics::Quaternion, maxsize: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn ResetPlacementOverride(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicKeyboardStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<HolographicKeyboard>;
}
