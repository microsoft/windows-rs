#[cfg(feature = "implement_exclusive")]
pub trait ICustomMapTileDataSourceImpl: Sized {
    fn BitmapRequested(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<CustomMapTileDataSource, MapTileBitmapRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBitmapRequested(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomMapTileDataSourceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CustomMapTileDataSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMapTileDataSourceImpl: Sized {
    fn UriFormatString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUriFormatString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AdditionalRequestHeaders(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn AllowCaching(&self) -> ::windows::core::Result<bool>;
    fn SetAllowCaching(&self, value: bool) -> ::windows::core::Result<()>;
    fn UriRequested(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<HttpMapTileDataSource, MapTileUriRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUriRequested(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHttpMapTileDataSourceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<HttpMapTileDataSource>;
    fn CreateInstanceWithUriFormatString(&self, uriformatstring: &::windows::core::HSTRING, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<HttpMapTileDataSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocalMapTileDataSourceImpl: Sized {
    fn UriFormatString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUriFormatString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UriRequested(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<LocalMapTileDataSource, MapTileUriRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUriRequested(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocalMapTileDataSourceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<LocalMapTileDataSource>;
    fn CreateInstanceWithUriFormatString(&self, uriformatstring: &::windows::core::HSTRING, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<LocalMapTileDataSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapActualCameraChangedEventArgsImpl: Sized {
    fn Camera(&self) -> ::windows::core::Result<MapCamera>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapActualCameraChangedEventArgs2Impl: Sized {
    fn ChangeReason(&self) -> ::windows::core::Result<MapCameraChangeReason>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapActualCameraChangingEventArgsImpl: Sized {
    fn Camera(&self) -> ::windows::core::Result<MapCamera>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapActualCameraChangingEventArgs2Impl: Sized {
    fn ChangeReason(&self) -> ::windows::core::Result<MapCameraChangeReason>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapBillboardImpl: Sized {
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn SetLocation(&self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn NormalizedAnchorPoint(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetNormalizedAnchorPoint(&self, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn Image(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetImage(&self, value: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn CollisionBehaviorDesired(&self) -> ::windows::core::Result<MapElementCollisionBehavior>;
    fn SetCollisionBehaviorDesired(&self, value: MapElementCollisionBehavior) -> ::windows::core::Result<()>;
    fn ReferenceCamera(&self) -> ::windows::core::Result<MapCamera>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapBillboardFactoryImpl: Sized {
    fn CreateInstanceFromCamera(&self, camera: &::core::option::Option<MapCamera>) -> ::windows::core::Result<MapBillboard>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapBillboardStaticsImpl: Sized {
    fn LocationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn NormalizedAnchorPointProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CollisionBehaviorDesiredProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapCameraImpl: Sized {
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn SetLocation(&self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn Heading(&self) -> ::windows::core::Result<f64>;
    fn SetHeading(&self, value: f64) -> ::windows::core::Result<()>;
    fn Pitch(&self) -> ::windows::core::Result<f64>;
    fn SetPitch(&self, value: f64) -> ::windows::core::Result<()>;
    fn Roll(&self) -> ::windows::core::Result<f64>;
    fn SetRoll(&self, value: f64) -> ::windows::core::Result<()>;
    fn FieldOfView(&self) -> ::windows::core::Result<f64>;
    fn SetFieldOfView(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapCameraFactoryImpl: Sized {
    fn CreateInstanceWithLocation(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<MapCamera>;
    fn CreateInstanceWithLocationAndHeading(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, headingindegrees: f64) -> ::windows::core::Result<MapCamera>;
    fn CreateInstanceWithLocationHeadingAndPitch(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, headingindegrees: f64, pitchindegrees: f64) -> ::windows::core::Result<MapCamera>;
    fn CreateInstanceWithLocationHeadingPitchRollAndFieldOfView(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, headingindegrees: f64, pitchindegrees: f64, rollindegrees: f64, fieldofviewindegrees: f64) -> ::windows::core::Result<MapCamera>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapContextRequestedEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElements(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MapElement>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlImpl: Sized {
    fn Center(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn SetCenter(&self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::super::DependencyObject>>;
    fn ColorScheme(&self) -> ::windows::core::Result<MapColorScheme>;
    fn SetColorScheme(&self, value: MapColorScheme) -> ::windows::core::Result<()>;
    fn DesiredPitch(&self) -> ::windows::core::Result<f64>;
    fn SetDesiredPitch(&self, value: f64) -> ::windows::core::Result<()>;
    fn Heading(&self) -> ::windows::core::Result<f64>;
    fn SetHeading(&self, value: f64) -> ::windows::core::Result<()>;
    fn LandmarksVisible(&self) -> ::windows::core::Result<bool>;
    fn SetLandmarksVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn LoadingStatus(&self) -> ::windows::core::Result<MapLoadingStatus>;
    fn MapServiceToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMapServiceToken(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MaxZoomLevel(&self) -> ::windows::core::Result<f64>;
    fn MinZoomLevel(&self) -> ::windows::core::Result<f64>;
    fn PedestrianFeaturesVisible(&self) -> ::windows::core::Result<bool>;
    fn SetPedestrianFeaturesVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn Pitch(&self) -> ::windows::core::Result<f64>;
    fn Style(&self) -> ::windows::core::Result<MapStyle>;
    fn SetStyle(&self, value: MapStyle) -> ::windows::core::Result<()>;
    fn TrafficFlowVisible(&self) -> ::windows::core::Result<bool>;
    fn SetTrafficFlowVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn TransformOrigin(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetTransformOrigin(&self, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn WatermarkMode(&self) -> ::windows::core::Result<MapWatermarkMode>;
    fn SetWatermarkMode(&self, value: MapWatermarkMode) -> ::windows::core::Result<()>;
    fn ZoomLevel(&self) -> ::windows::core::Result<f64>;
    fn SetZoomLevel(&self, value: f64) -> ::windows::core::Result<()>;
    fn MapElements(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapElement>>;
    fn Routes(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapRouteView>>;
    fn TileSources(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapTileSource>>;
    fn CenterChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCenterChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HeadingChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveHeadingChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LoadingStatusChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLoadingStatusChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapDoubleTapped(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapDoubleTapped(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapHolding(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapHolding(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapTapped(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapInputEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapTapped(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PitchChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePitchChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TransformOriginChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTransformOriginChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ZoomLevelChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveZoomLevelChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FindMapElementsAtOffset(&self, offset: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MapElement>>;
    fn GetLocationFromOffset(&self, offset: &super::super::super::super::Foundation::Point, location: &mut ::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn GetOffsetFromLocation(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, offset: &mut super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn IsLocationInView(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, isinview: &mut bool) -> ::windows::core::Result<()>;
    fn TrySetViewBoundsAsync(&self, bounds: &::core::option::Option<super::super::super::super::Devices::Geolocation::GeoboundingBox>, margin: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::Thickness>>, animation: MapAnimationKind) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetViewWithCenterAsync(&self, center: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetViewWithCenterAndZoomAsync(&self, center: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, zoomlevel: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetViewWithCenterZoomHeadingAndPitchAsync(&self, center: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, zoomlevel: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>, heading: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>, desiredpitch: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetViewWithCenterZoomHeadingPitchAndAnimationAsync(&self, center: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, zoomlevel: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>, heading: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>, desiredpitch: &::core::option::Option<super::super::super::super::Foundation::IReference<f64>>, animation: MapAnimationKind) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControl2Impl: Sized {
    fn BusinessLandmarksVisible(&self) -> ::windows::core::Result<bool>;
    fn SetBusinessLandmarksVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn TransitFeaturesVisible(&self) -> ::windows::core::Result<bool>;
    fn SetTransitFeaturesVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn PanInteractionMode(&self) -> ::windows::core::Result<MapPanInteractionMode>;
    fn SetPanInteractionMode(&self, value: MapPanInteractionMode) -> ::windows::core::Result<()>;
    fn RotateInteractionMode(&self) -> ::windows::core::Result<MapInteractionMode>;
    fn SetRotateInteractionMode(&self, value: MapInteractionMode) -> ::windows::core::Result<()>;
    fn TiltInteractionMode(&self) -> ::windows::core::Result<MapInteractionMode>;
    fn SetTiltInteractionMode(&self, value: MapInteractionMode) -> ::windows::core::Result<()>;
    fn ZoomInteractionMode(&self) -> ::windows::core::Result<MapInteractionMode>;
    fn SetZoomInteractionMode(&self, value: MapInteractionMode) -> ::windows::core::Result<()>;
    fn Is3DSupported(&self) -> ::windows::core::Result<bool>;
    fn IsStreetsideSupported(&self) -> ::windows::core::Result<bool>;
    fn Scene(&self) -> ::windows::core::Result<MapScene>;
    fn SetScene(&self, value: &::core::option::Option<MapScene>) -> ::windows::core::Result<()>;
    fn ActualCamera(&self) -> ::windows::core::Result<MapCamera>;
    fn TargetCamera(&self) -> ::windows::core::Result<MapCamera>;
    fn CustomExperience(&self) -> ::windows::core::Result<MapCustomExperience>;
    fn SetCustomExperience(&self, value: &::core::option::Option<MapCustomExperience>) -> ::windows::core::Result<()>;
    fn MapElementClick(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementClickEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapElementClick(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapElementPointerEntered(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementPointerEnteredEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapElementPointerEntered(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapElementPointerExited(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapElementPointerExitedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapElementPointerExited(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ActualCameraChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapActualCameraChangedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveActualCameraChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ActualCameraChanging(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapActualCameraChangingEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveActualCameraChanging(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TargetCameraChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapTargetCameraChangedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTargetCameraChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CustomExperienceChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapCustomExperienceChangedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCustomExperienceChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StartContinuousRotate(&self, rateindegreespersecond: f64) -> ::windows::core::Result<()>;
    fn StopContinuousRotate(&self) -> ::windows::core::Result<()>;
    fn StartContinuousTilt(&self, rateindegreespersecond: f64) -> ::windows::core::Result<()>;
    fn StopContinuousTilt(&self) -> ::windows::core::Result<()>;
    fn StartContinuousZoom(&self, rateofchangepersecond: f64) -> ::windows::core::Result<()>;
    fn StopContinuousZoom(&self) -> ::windows::core::Result<()>;
    fn TryRotateAsync(&self, degrees: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryRotateToAsync(&self, angleindegrees: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryTiltAsync(&self, degrees: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryTiltToAsync(&self, angleindegrees: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryZoomInAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryZoomOutAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryZoomToAsync(&self, zoomlevel: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetSceneAsync(&self, scene: &::core::option::Option<MapScene>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetSceneWithAnimationAsync(&self, scene: &::core::option::Option<MapScene>, animationkind: MapAnimationKind) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControl3Impl: Sized {
    fn MapRightTapped(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapRightTappedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapRightTapped(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControl4Impl: Sized {
    fn BusinessLandmarksEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetBusinessLandmarksEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn TransitFeaturesEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetTransitFeaturesEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetVisibleRegion(&self, region: MapVisibleRegionKind) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopath>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControl5Impl: Sized {
    fn MapProjection(&self) -> ::windows::core::Result<MapProjection>;
    fn SetMapProjection(&self, value: MapProjection) -> ::windows::core::Result<()>;
    fn StyleSheet(&self) -> ::windows::core::Result<MapStyleSheet>;
    fn SetStyleSheet(&self, value: &::core::option::Option<MapStyleSheet>) -> ::windows::core::Result<()>;
    fn ViewPadding(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetViewPadding(&self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn MapContextRequested(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapContextRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapContextRequested(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FindMapElementsAtOffsetWithRadius(&self, offset: &super::super::super::super::Foundation::Point, radius: f64) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MapElement>>;
    fn GetLocationFromOffsetWithReferenceSystem(&self, offset: &super::super::super::super::Foundation::Point, desiredreferencesystem: super::super::super::super::Devices::Geolocation::AltitudeReferenceSystem, location: &mut ::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn StartContinuousPan(&self, horizontalpixelspersecond: f64, verticalpixelspersecond: f64) -> ::windows::core::Result<()>;
    fn StopContinuousPan(&self) -> ::windows::core::Result<()>;
    fn TryPanAsync(&self, horizontalpixels: f64, verticalpixels: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn TryPanToAsync(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControl6Impl: Sized {
    fn Layers(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapLayer>>;
    fn SetLayers(&self, value: &::core::option::Option<super::super::super::super::Foundation::Collections::IVector<MapLayer>>) -> ::windows::core::Result<()>;
    fn TryGetLocationFromOffset(&self, offset: &super::super::super::super::Foundation::Point, location: &mut ::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<bool>;
    fn TryGetLocationFromOffsetWithReferenceSystem(&self, offset: &super::super::super::super::Foundation::Point, desiredreferencesystem: super::super::super::super::Devices::Geolocation::AltitudeReferenceSystem, location: &mut ::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControl7Impl: Sized {
    fn Region(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRegion(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControl8Impl: Sized {
    fn CanTiltDown(&self) -> ::windows::core::Result<bool>;
    fn CanTiltUp(&self) -> ::windows::core::Result<bool>;
    fn CanZoomIn(&self) -> ::windows::core::Result<bool>;
    fn CanZoomOut(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlBusinessLandmarkClickEventArgsImpl: Sized {
    fn LocalLocations(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlBusinessLandmarkPointerEnteredEventArgsImpl: Sized {
    fn LocalLocations(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlBusinessLandmarkPointerExitedEventArgsImpl: Sized {
    fn LocalLocations(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlBusinessLandmarkRightTappedEventArgsImpl: Sized {
    fn LocalLocations(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Services::Maps::LocalSearch::LocalLocation>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlDataHelperImpl: Sized {
    fn BusinessLandmarkClick(&self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkClickEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBusinessLandmarkClick(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TransitFeatureClick(&self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeatureClickEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTransitFeatureClick(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BusinessLandmarkRightTapped(&self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkRightTappedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBusinessLandmarkRightTapped(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TransitFeatureRightTapped(&self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeatureRightTappedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTransitFeatureRightTapped(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlDataHelper2Impl: Sized {
    fn BusinessLandmarkPointerEntered(&self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkPointerEnteredEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBusinessLandmarkPointerEntered(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TransitFeaturePointerEntered(&self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeaturePointerEnteredEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTransitFeaturePointerEntered(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BusinessLandmarkPointerExited(&self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlBusinessLandmarkPointerExitedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBusinessLandmarkPointerExited(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TransitFeaturePointerExited(&self, value: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapControl, MapControlTransitFeaturePointerExitedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTransitFeaturePointerExited(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlDataHelperFactoryImpl: Sized {
    fn CreateInstance(&self, map: &::core::option::Option<MapControl>) -> ::windows::core::Result<MapControlDataHelper>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlDataHelperStaticsImpl: Sized {
    fn CreateMapControl(&self, rasterrendermode: bool) -> ::windows::core::Result<MapControl>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlStaticsImpl: Sized {
    fn CenterProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ChildrenProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ColorSchemeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DesiredPitchProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn HeadingProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn LandmarksVisibleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn LoadingStatusProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MapServiceTokenProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PedestrianFeaturesVisibleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PitchProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn StyleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TrafficFlowVisibleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TransformOriginProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn WatermarkModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ZoomLevelProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MapElementsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RoutesProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TileSourcesProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn LocationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetLocation(&self, element: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn SetLocation(&self, element: &::core::option::Option<super::super::DependencyObject>, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn NormalizedAnchorPointProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetNormalizedAnchorPoint(&self, element: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetNormalizedAnchorPoint(&self, element: &::core::option::Option<super::super::DependencyObject>, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlStatics2Impl: Sized {
    fn BusinessLandmarksVisibleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TransitFeaturesVisibleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PanInteractionModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RotateInteractionModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TiltInteractionModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ZoomInteractionModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn Is3DSupportedProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsStreetsideSupportedProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SceneProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlStatics4Impl: Sized {
    fn BusinessLandmarksEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TransitFeaturesEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlStatics5Impl: Sized {
    fn MapProjectionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn StyleSheetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ViewPaddingProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlStatics6Impl: Sized {
    fn LayersProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlStatics7Impl: Sized {
    fn RegionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlStatics8Impl: Sized {
    fn CanTiltDownProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CanTiltUpProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CanZoomInProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CanZoomOutProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlTransitFeatureClickEventArgsImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn TransitProperties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlTransitFeaturePointerEnteredEventArgsImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn TransitProperties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlTransitFeaturePointerExitedEventArgsImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn TransitProperties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlTransitFeatureRightTappedEventArgsImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn TransitProperties(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapCustomExperienceImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMapCustomExperienceChangedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMapCustomExperienceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapCustomExperience>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementImpl: Sized {
    fn ZIndex(&self) -> ::windows::core::Result<i32>;
    fn SetZIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn Visible(&self) -> ::windows::core::Result<bool>;
    fn SetVisible(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElement2Impl: Sized {
    fn MapTabIndex(&self) -> ::windows::core::Result<i32>;
    fn SetMapTabIndex(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElement3Impl: Sized {
    fn MapStyleSheetEntry(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMapStyleSheetEntry(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn MapStyleSheetEntryState(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMapStyleSheetEntryState(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetTag(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElement3DImpl: Sized {
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn SetLocation(&self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn Model(&self) -> ::windows::core::Result<MapModel3D>;
    fn SetModel(&self, value: &::core::option::Option<MapModel3D>) -> ::windows::core::Result<()>;
    fn Heading(&self) -> ::windows::core::Result<f64>;
    fn SetHeading(&self, value: f64) -> ::windows::core::Result<()>;
    fn Pitch(&self) -> ::windows::core::Result<f64>;
    fn SetPitch(&self, value: f64) -> ::windows::core::Result<()>;
    fn Roll(&self) -> ::windows::core::Result<f64>;
    fn SetRoll(&self, value: f64) -> ::windows::core::Result<()>;
    fn Scale(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Numerics::Vector3>;
    fn SetScale(&self, value: &super::super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElement3DStaticsImpl: Sized {
    fn LocationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn HeadingProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PitchProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RollProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ScaleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElement4Impl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementClickEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElements(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapElement>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapElement>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementPointerEnteredEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElement(&self) -> ::windows::core::Result<MapElement>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementPointerExitedEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElement(&self) -> ::windows::core::Result<MapElement>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementStaticsImpl: Sized {
    fn ZIndexProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn VisibleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementStatics2Impl: Sized {
    fn MapTabIndexProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementStatics3Impl: Sized {
    fn MapStyleSheetEntryProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MapStyleSheetEntryStateProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TagProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementStatics4Impl: Sized {
    fn IsEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementsLayerImpl: Sized {
    fn MapElements(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapElement>>;
    fn SetMapElements(&self, value: &::core::option::Option<super::super::super::super::Foundation::Collections::IVector<MapElement>>) -> ::windows::core::Result<()>;
    fn MapElementClick(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerClickEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapElementClick(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapElementPointerEntered(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerPointerEnteredEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapElementPointerEntered(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapElementPointerExited(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerPointerExitedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapElementPointerExited(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MapContextRequested(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapElementsLayer, MapElementsLayerContextRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMapContextRequested(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementsLayerClickEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElements(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<MapElement>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementsLayerContextRequestedEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElements(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MapElement>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementsLayerPointerEnteredEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElement(&self) -> ::windows::core::Result<MapElement>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementsLayerPointerExitedEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn MapElement(&self) -> ::windows::core::Result<MapElement>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapElementsLayerStaticsImpl: Sized {
    fn MapElementsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapIconImpl: Sized {
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
    fn SetLocation(&self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<()>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn NormalizedAnchorPoint(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn SetNormalizedAnchorPoint(&self, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn Image(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetImage(&self, value: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapIcon2Impl: Sized {
    fn CollisionBehaviorDesired(&self) -> ::windows::core::Result<MapElementCollisionBehavior>;
    fn SetCollisionBehaviorDesired(&self, value: MapElementCollisionBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapIconStaticsImpl: Sized {
    fn LocationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TitleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn NormalizedAnchorPointProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapIconStatics2Impl: Sized {
    fn CollisionBehaviorDesiredProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapInputEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapItemsControlImpl: Sized {
    fn ItemsSource(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetItemsSource(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Items(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::super::DependencyObject>>;
    fn ItemTemplate(&self) -> ::windows::core::Result<super::super::DataTemplate>;
    fn SetItemTemplate(&self, value: &::core::option::Option<super::super::DataTemplate>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapItemsControlStaticsImpl: Sized {
    fn ItemsSourceProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ItemsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ItemTemplateProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapLayerImpl: Sized {
    fn MapTabIndex(&self) -> ::windows::core::Result<i32>;
    fn SetMapTabIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn Visible(&self) -> ::windows::core::Result<bool>;
    fn SetVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn ZIndex(&self) -> ::windows::core::Result<i32>;
    fn SetZIndex(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapLayerFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapLayer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapLayerStaticsImpl: Sized {
    fn MapTabIndexProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn VisibleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ZIndexProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapModel3DImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMapModel3DFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapModel3D>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapModel3DStaticsImpl: Sized {
    fn CreateFrom3MFAsync(&self, source: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MapModel3D>>;
    fn CreateFrom3MFWithShadingOptionAsync(&self, source: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>, shadingoption: MapModel3DShadingOption) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MapModel3D>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapPolygonImpl: Sized {
    fn Path(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopath>;
    fn SetPath(&self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopath>) -> ::windows::core::Result<()>;
    fn StrokeColor(&self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetStrokeColor(&self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
    fn StrokeThickness(&self) -> ::windows::core::Result<f64>;
    fn SetStrokeThickness(&self, value: f64) -> ::windows::core::Result<()>;
    fn StrokeDashed(&self) -> ::windows::core::Result<bool>;
    fn SetStrokeDashed(&self, value: bool) -> ::windows::core::Result<()>;
    fn FillColor(&self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetFillColor(&self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapPolygon2Impl: Sized {
    fn Paths(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::super::super::super::Devices::Geolocation::Geopath>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapPolygonStaticsImpl: Sized {
    fn PathProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn StrokeThicknessProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn StrokeDashedProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapPolylineImpl: Sized {
    fn Path(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopath>;
    fn SetPath(&self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopath>) -> ::windows::core::Result<()>;
    fn StrokeColor(&self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetStrokeColor(&self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
    fn StrokeThickness(&self) -> ::windows::core::Result<f64>;
    fn SetStrokeThickness(&self, value: f64) -> ::windows::core::Result<()>;
    fn StrokeDashed(&self) -> ::windows::core::Result<bool>;
    fn SetStrokeDashed(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapPolylineStaticsImpl: Sized {
    fn PathProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn StrokeDashedProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRightTappedEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteViewImpl: Sized {
    fn RouteColor(&self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetRouteColor(&self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
    fn OutlineColor(&self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetOutlineColor(&self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
    fn Route(&self) -> ::windows::core::Result<super::super::super::super::Services::Maps::MapRoute>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteViewFactoryImpl: Sized {
    fn CreateInstanceWithMapRoute(&self, route: &::core::option::Option<super::super::super::super::Services::Maps::MapRoute>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapRouteView>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapSceneImpl: Sized {
    fn TargetCamera(&self) -> ::windows::core::Result<MapCamera>;
    fn TargetCameraChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<MapScene, MapTargetCameraChangedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTargetCameraChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapSceneStaticsImpl: Sized {
    fn CreateFromBoundingBox(&self, bounds: &::core::option::Option<super::super::super::super::Devices::Geolocation::GeoboundingBox>) -> ::windows::core::Result<MapScene>;
    fn CreateFromBoundingBoxWithHeadingAndPitch(&self, bounds: &::core::option::Option<super::super::super::super::Devices::Geolocation::GeoboundingBox>, headingindegrees: f64, pitchindegrees: f64) -> ::windows::core::Result<MapScene>;
    fn CreateFromCamera(&self, camera: &::core::option::Option<MapCamera>) -> ::windows::core::Result<MapScene>;
    fn CreateFromLocation(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<MapScene>;
    fn CreateFromLocationWithHeadingAndPitch(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, headingindegrees: f64, pitchindegrees: f64) -> ::windows::core::Result<MapScene>;
    fn CreateFromLocationAndRadius(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, radiusinmeters: f64) -> ::windows::core::Result<MapScene>;
    fn CreateFromLocationAndRadiusWithHeadingAndPitch(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, radiusinmeters: f64, headingindegrees: f64, pitchindegrees: f64) -> ::windows::core::Result<MapScene>;
    fn CreateFromLocations(&self, locations: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<super::super::super::super::Devices::Geolocation::Geopoint>>) -> ::windows::core::Result<MapScene>;
    fn CreateFromLocationsWithHeadingAndPitch(&self, locations: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<super::super::super::super::Devices::Geolocation::Geopoint>>, headingindegrees: f64, pitchindegrees: f64) -> ::windows::core::Result<MapScene>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapStyleSheetImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMapStyleSheetEntriesStaticsImpl: Sized {
    fn Area(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Airport(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Cemetery(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Continent(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Education(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IndigenousPeoplesReserve(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Island(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Medical(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Military(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Nautical(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Neighborhood(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Runway(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Sand(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ShoppingCenter(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Stadium(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Vegetation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Forest(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GolfCourse(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Park(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PlayingField(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Reserve(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Point(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NaturalPoint(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Peak(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VolcanicPeak(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WaterPoint(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PointOfInterest(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Business(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FoodPoint(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PopulatedPlace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Capital(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AdminDistrictCapital(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CountryRegionCapital(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RoadShield(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RoadExit(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Transit(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Political(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CountryRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AdminDistrict(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn District(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Structure(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Building(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EducationBuilding(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MedicalBuilding(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TransitBuilding(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Transportation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Road(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ControlledAccessHighway(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HighSpeedRamp(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Highway(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MajorRoad(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ArterialRoad(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Street(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ramp(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UnpavedStreet(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TollRoad(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Railway(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Trail(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WaterRoute(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Water(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn River(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RouteLine(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WalkingRoute(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DrivingRoute(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapStyleSheetEntryStatesStaticsImpl: Sized {
    fn Disabled(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Hover(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Selected(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapStyleSheetStaticsImpl: Sized {
    fn Aerial(&self) -> ::windows::core::Result<MapStyleSheet>;
    fn AerialWithOverlay(&self) -> ::windows::core::Result<MapStyleSheet>;
    fn RoadLight(&self) -> ::windows::core::Result<MapStyleSheet>;
    fn RoadDark(&self) -> ::windows::core::Result<MapStyleSheet>;
    fn RoadHighContrastLight(&self) -> ::windows::core::Result<MapStyleSheet>;
    fn RoadHighContrastDark(&self) -> ::windows::core::Result<MapStyleSheet>;
    fn Combine(&self, stylesheets: &::core::option::Option<super::super::super::super::Foundation::Collections::IIterable<MapStyleSheet>>) -> ::windows::core::Result<MapStyleSheet>;
    fn ParseFromJson(&self, styleasjson: &::windows::core::HSTRING) -> ::windows::core::Result<MapStyleSheet>;
    fn TryParseFromJson(&self, styleasjson: &::windows::core::HSTRING, stylesheet: &mut ::core::option::Option<MapStyleSheet>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTargetCameraChangedEventArgsImpl: Sized {
    fn Camera(&self) -> ::windows::core::Result<MapCamera>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTargetCameraChangedEventArgs2Impl: Sized {
    fn ChangeReason(&self) -> ::windows::core::Result<MapCameraChangeReason>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileBitmapRequestImpl: Sized {
    fn PixelData(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetPixelData(&self, value: &::core::option::Option<super::super::super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<MapTileBitmapRequestDeferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileBitmapRequestDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileBitmapRequestedEventArgsImpl: Sized {
    fn X(&self) -> ::windows::core::Result<i32>;
    fn Y(&self) -> ::windows::core::Result<i32>;
    fn ZoomLevel(&self) -> ::windows::core::Result<i32>;
    fn Request(&self) -> ::windows::core::Result<MapTileBitmapRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileBitmapRequestedEventArgs2Impl: Sized {
    fn FrameIndex(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileDataSourceImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileDataSourceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileDataSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileSourceImpl: Sized {
    fn DataSource(&self) -> ::windows::core::Result<MapTileDataSource>;
    fn SetDataSource(&self, value: &::core::option::Option<MapTileDataSource>) -> ::windows::core::Result<()>;
    fn Layer(&self) -> ::windows::core::Result<MapTileLayer>;
    fn SetLayer(&self, value: MapTileLayer) -> ::windows::core::Result<()>;
    fn ZoomLevelRange(&self) -> ::windows::core::Result<MapZoomLevelRange>;
    fn SetZoomLevelRange(&self, value: &MapZoomLevelRange) -> ::windows::core::Result<()>;
    fn Bounds(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::GeoboundingBox>;
    fn SetBounds(&self, value: &::core::option::Option<super::super::super::super::Devices::Geolocation::GeoboundingBox>) -> ::windows::core::Result<()>;
    fn AllowOverstretch(&self) -> ::windows::core::Result<bool>;
    fn SetAllowOverstretch(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsFadingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsFadingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsTransparencyEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsTransparencyEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsRetryEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsRetryEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ZIndex(&self) -> ::windows::core::Result<i32>;
    fn SetZIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn TilePixelSize(&self) -> ::windows::core::Result<i32>;
    fn SetTilePixelSize(&self, value: i32) -> ::windows::core::Result<()>;
    fn Visible(&self) -> ::windows::core::Result<bool>;
    fn SetVisible(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileSource2Impl: Sized {
    fn AnimationState(&self) -> ::windows::core::Result<MapTileAnimationState>;
    fn AutoPlay(&self) -> ::windows::core::Result<bool>;
    fn SetAutoPlay(&self, value: bool) -> ::windows::core::Result<()>;
    fn FrameCount(&self) -> ::windows::core::Result<i32>;
    fn SetFrameCount(&self, value: i32) -> ::windows::core::Result<()>;
    fn FrameDuration(&self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan>;
    fn SetFrameDuration(&self, value: &super::super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Play(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileSourceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileSource>;
    fn CreateInstanceWithDataSource(&self, datasource: &::core::option::Option<MapTileDataSource>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileSource>;
    fn CreateInstanceWithDataSourceAndZoomRange(&self, datasource: &::core::option::Option<MapTileDataSource>, zoomlevelrange: &MapZoomLevelRange, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileSource>;
    fn CreateInstanceWithDataSourceZoomRangeAndBounds(&self, datasource: &::core::option::Option<MapTileDataSource>, zoomlevelrange: &MapZoomLevelRange, bounds: &::core::option::Option<super::super::super::super::Devices::Geolocation::GeoboundingBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileSource>;
    fn CreateInstanceWithDataSourceZoomRangeBoundsAndTileSize(&self, datasource: &::core::option::Option<MapTileDataSource>, zoomlevelrange: &MapZoomLevelRange, bounds: &::core::option::Option<super::super::super::super::Devices::Geolocation::GeoboundingBox>, tilesizeinpixels: i32, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MapTileSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileSourceStaticsImpl: Sized {
    fn DataSourceProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn LayerProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ZoomLevelRangeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn BoundsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn AllowOverstretchProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsFadingEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsTransparencyEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsRetryEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ZIndexProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn TilePixelSizeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn VisibleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileSourceStatics2Impl: Sized {
    fn AnimationStateProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn AutoPlayProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FrameCountProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FrameDurationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileUriRequestImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri>;
    fn SetUri(&self, value: &::core::option::Option<super::super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<MapTileUriRequestDeferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileUriRequestDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileUriRequestedEventArgsImpl: Sized {
    fn X(&self) -> ::windows::core::Result<i32>;
    fn Y(&self) -> ::windows::core::Result<i32>;
    fn ZoomLevel(&self) -> ::windows::core::Result<i32>;
    fn Request(&self) -> ::windows::core::Result<MapTileUriRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapTileUriRequestedEventArgs2Impl: Sized {
    fn FrameIndex(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreetsideExperienceImpl: Sized {
    fn AddressTextVisible(&self) -> ::windows::core::Result<bool>;
    fn SetAddressTextVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn CursorVisible(&self) -> ::windows::core::Result<bool>;
    fn SetCursorVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn OverviewMapVisible(&self) -> ::windows::core::Result<bool>;
    fn SetOverviewMapVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn StreetLabelsVisible(&self) -> ::windows::core::Result<bool>;
    fn SetStreetLabelsVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn ExitButtonVisible(&self) -> ::windows::core::Result<bool>;
    fn SetExitButtonVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn ZoomButtonsVisible(&self) -> ::windows::core::Result<bool>;
    fn SetZoomButtonsVisible(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreetsideExperienceFactoryImpl: Sized {
    fn CreateInstanceWithPanorama(&self, panorama: &::core::option::Option<StreetsidePanorama>) -> ::windows::core::Result<StreetsideExperience>;
    fn CreateInstanceWithPanoramaHeadingPitchAndFieldOfView(&self, panorama: &::core::option::Option<StreetsidePanorama>, headingindegrees: f64, pitchindegrees: f64, fieldofviewindegrees: f64) -> ::windows::core::Result<StreetsideExperience>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreetsidePanoramaImpl: Sized {
    fn Location(&self) -> ::windows::core::Result<super::super::super::super::Devices::Geolocation::Geopoint>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreetsidePanoramaStaticsImpl: Sized {
    fn FindNearbyWithLocationAsync(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<StreetsidePanorama>>;
    fn FindNearbyWithLocationAndRadiusAsync(&self, location: &::core::option::Option<super::super::super::super::Devices::Geolocation::Geopoint>, radiusinmeters: f64) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<StreetsidePanorama>>;
}
