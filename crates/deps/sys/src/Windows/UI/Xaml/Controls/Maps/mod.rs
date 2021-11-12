#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct MapAnimationKind(i32);
#[repr(transparent)]
pub struct MapBillboard(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapCamera(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MapCameraChangeReason(i32);
#[repr(C)]
pub struct MapColorScheme(i32);
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
#[repr(C)]
pub struct MapElementCollisionBehavior(i32);
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
#[repr(C)]
pub struct MapInteractionMode(i32);
#[repr(transparent)]
pub struct MapItemsControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapLayer(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MapLoadingStatus(i32);
#[repr(transparent)]
pub struct MapModel3D(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MapModel3DShadingOption(i32);
#[repr(C)]
pub struct MapPanInteractionMode(i32);
#[repr(transparent)]
pub struct MapPolygon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapPolyline(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MapProjection(i32);
#[repr(transparent)]
pub struct MapRightTappedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapRouteView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapScene(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MapStyle(i32);
#[repr(transparent)]
pub struct MapStyleSheet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapTargetCameraChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MapTileAnimationState(i32);
#[repr(transparent)]
pub struct MapTileBitmapRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapTileBitmapRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapTileBitmapRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapTileDataSource(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MapTileLayer(i32);
#[repr(transparent)]
pub struct MapTileSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapTileUriRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapTileUriRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapTileUriRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MapVisibleRegionKind(i32);
#[repr(C)]
pub struct MapWatermarkMode(i32);
#[repr(C)]
pub struct MapZoomLevelRange(i32);
#[repr(transparent)]
pub struct StreetsideExperience(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StreetsidePanorama(pub *mut ::core::ffi::c_void);
