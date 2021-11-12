#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CustomMapTileDataSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpMapTileDataSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICustomMapTileDataSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICustomMapTileDataSourceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpMapTileDataSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpMapTileDataSourceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILocalMapTileDataSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILocalMapTileDataSourceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapActualCameraChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapActualCameraChangedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapActualCameraChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapActualCameraChangingEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapBillboard(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapBillboardFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapBillboardStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapCamera(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapCameraFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapContextRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControl2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControl3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControl4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControl5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControl6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControl7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControl8(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControlBusinessLandmarkClickEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControlBusinessLandmarkPointerEnteredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControlBusinessLandmarkPointerExitedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControlBusinessLandmarkRightTappedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControlDataHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControlDataHelper2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControlDataHelperFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControlDataHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControlStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControlStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControlStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControlStatics5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControlStatics6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControlStatics7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControlStatics8(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControlTransitFeatureClickEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControlTransitFeaturePointerEnteredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControlTransitFeaturePointerExitedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapControlTransitFeatureRightTappedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapCustomExperience(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapCustomExperienceChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapCustomExperienceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapElement2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapElement3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapElement3D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapElement3DStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapElement4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapElementClickEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapElementFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapElementPointerEnteredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapElementPointerExitedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapElementStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapElementStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapElementStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapElementStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapElementsLayer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapElementsLayerClickEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapElementsLayerContextRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapElementsLayerPointerEnteredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapElementsLayerPointerExitedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapElementsLayerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapIcon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapIcon2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapIconStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapIconStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapInputEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapItemsControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapItemsControlStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapLayer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapLayerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapLayerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapModel3D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapModel3DFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapModel3DStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapPolygon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapPolygon2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapPolygonStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapPolyline(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapPolylineStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapRightTappedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapRouteView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapRouteViewFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapScene(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapSceneStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapStyleSheet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapStyleSheetEntriesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapStyleSheetEntryStatesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapStyleSheetStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapTargetCameraChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapTargetCameraChangedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapTileBitmapRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapTileBitmapRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapTileBitmapRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapTileBitmapRequestedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapTileDataSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapTileDataSourceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapTileSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapTileSource2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapTileSourceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapTileSourceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapTileSourceStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapTileUriRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapTileUriRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapTileUriRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapTileUriRequestedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreetsideExperience(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreetsideExperienceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreetsidePanorama(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreetsidePanoramaStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LocalMapTileDataSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapActualCameraChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapActualCameraChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapAnimationKind(pub i32);
impl MapAnimationKind {
    pub const Default: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Linear: Self = Self(2i32);
    pub const Bow: Self = Self(3i32);
}
#[repr(transparent)]
pub struct MapBillboard(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapCamera(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapCameraChangeReason(pub i32);
impl MapCameraChangeReason {
    pub const System: Self = Self(0i32);
    pub const UserInteraction: Self = Self(1i32);
    pub const Programmatic: Self = Self(2i32);
}
#[repr(transparent)]
pub struct MapColorScheme(pub i32);
impl MapColorScheme {
    pub const Light: Self = Self(0i32);
    pub const Dark: Self = Self(1i32);
}
#[repr(transparent)]
pub struct MapContextRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapControlBusinessLandmarkClickEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapControlBusinessLandmarkPointerEnteredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapControlBusinessLandmarkPointerExitedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapControlBusinessLandmarkRightTappedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapControlDataHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapControlTransitFeatureClickEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapControlTransitFeaturePointerEnteredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapControlTransitFeaturePointerExitedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapControlTransitFeatureRightTappedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapCustomExperience(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapCustomExperienceChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapElement3D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapElementClickEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapElementCollisionBehavior(pub i32);
impl MapElementCollisionBehavior {
    pub const Hide: Self = Self(0i32);
    pub const RemainVisible: Self = Self(1i32);
}
#[repr(transparent)]
pub struct MapElementPointerEnteredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapElementPointerExitedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapElementsLayer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapElementsLayerClickEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapElementsLayerContextRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapElementsLayerPointerEnteredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapElementsLayerPointerExitedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapIcon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapInputEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapInteractionMode(pub i32);
impl MapInteractionMode {
    pub const Auto: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const GestureOnly: Self = Self(2i32);
    pub const PointerAndKeyboard: Self = Self(2i32);
    pub const ControlOnly: Self = Self(3i32);
    pub const GestureAndControl: Self = Self(4i32);
    pub const PointerKeyboardAndControl: Self = Self(4i32);
    pub const PointerOnly: Self = Self(5i32);
}
#[repr(transparent)]
pub struct MapItemsControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapLayer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapLoadingStatus(pub i32);
impl MapLoadingStatus {
    pub const Loading: Self = Self(0i32);
    pub const Loaded: Self = Self(1i32);
    pub const DataUnavailable: Self = Self(2i32);
    pub const DownloadedMapsManagerUnavailable: Self = Self(3i32);
}
#[repr(transparent)]
pub struct MapModel3D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapModel3DShadingOption(pub i32);
impl MapModel3DShadingOption {
    pub const Default: Self = Self(0i32);
    pub const Flat: Self = Self(1i32);
    pub const Smooth: Self = Self(2i32);
}
#[repr(transparent)]
pub struct MapPanInteractionMode(pub i32);
impl MapPanInteractionMode {
    pub const Auto: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
}
#[repr(transparent)]
pub struct MapPolygon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapPolyline(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapProjection(pub i32);
impl MapProjection {
    pub const WebMercator: Self = Self(0i32);
    pub const Globe: Self = Self(1i32);
}
#[repr(transparent)]
pub struct MapRightTappedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapRouteView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapScene(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapStyle(pub i32);
impl MapStyle {
    pub const None: Self = Self(0i32);
    pub const Road: Self = Self(1i32);
    pub const Aerial: Self = Self(2i32);
    pub const AerialWithRoads: Self = Self(3i32);
    pub const Terrain: Self = Self(4i32);
    pub const Aerial3D: Self = Self(5i32);
    pub const Aerial3DWithRoads: Self = Self(6i32);
    pub const Custom: Self = Self(7i32);
}
#[repr(transparent)]
pub struct MapStyleSheet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapTargetCameraChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapTileAnimationState(pub i32);
impl MapTileAnimationState {
    pub const Stopped: Self = Self(0i32);
    pub const Paused: Self = Self(1i32);
    pub const Playing: Self = Self(2i32);
}
#[repr(transparent)]
pub struct MapTileBitmapRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapTileBitmapRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapTileBitmapRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapTileDataSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapTileLayer(pub i32);
impl MapTileLayer {
    pub const LabelOverlay: Self = Self(0i32);
    pub const RoadOverlay: Self = Self(1i32);
    pub const AreaOverlay: Self = Self(2i32);
    pub const BackgroundOverlay: Self = Self(3i32);
    pub const BackgroundReplacement: Self = Self(4i32);
}
#[repr(transparent)]
pub struct MapTileSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapTileUriRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapTileUriRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapTileUriRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapVisibleRegionKind(pub i32);
impl MapVisibleRegionKind {
    pub const Near: Self = Self(0i32);
    pub const Full: Self = Self(1i32);
}
#[repr(transparent)]
pub struct MapWatermarkMode(pub i32);
impl MapWatermarkMode {
    pub const Automatic: Self = Self(0i32);
    pub const On: Self = Self(1i32);
}
#[repr(C)]
pub struct MapZoomLevelRange(i32);
#[repr(transparent)]
pub struct StreetsideExperience(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StreetsidePanorama(pub *mut ::core::ffi::c_void);
