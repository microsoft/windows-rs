#[cfg(feature = "implement_exclusive")]
pub trait IHolographicApplicationPreviewStaticsImpl: Sized {
    fn IsCurrentViewPresentedOnHolographicDisplay(&self) -> ::windows::core::Result<bool>;
    fn IsHolographicActivation(&self, activatedeventargs: &::core::option::Option<super::super::Activation::IActivatedEventArgs>) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IHolographicKeyboardPlacementOverridePreviewImpl: Sized {
    fn SetPlacementOverride(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>, topcenterposition: &super::super::super::Foundation::Numerics::Vector3, normal: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn SetPlacementOverrideWithMaxSize(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>, topcenterposition: &super::super::super::Foundation::Numerics::Vector3, normal: &super::super::super::Foundation::Numerics::Vector3, maxsize: &super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn ResetPlacementOverride(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IHolographicKeyboardPlacementOverridePreviewStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<HolographicKeyboardPlacementOverridePreview>;
}
