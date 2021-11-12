#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ForegroundText(i32);
pub struct IJumpList(pub *mut ::core::ffi::c_void);
pub struct IJumpListItem(pub *mut ::core::ffi::c_void);
pub struct IJumpListItemStatics(pub *mut ::core::ffi::c_void);
pub struct IJumpListStatics(pub *mut ::core::ffi::c_void);
pub struct ISecondaryTile(pub *mut ::core::ffi::c_void);
pub struct ISecondaryTile2(pub *mut ::core::ffi::c_void);
pub struct ISecondaryTileFactory(pub *mut ::core::ffi::c_void);
pub struct ISecondaryTileFactory2(pub *mut ::core::ffi::c_void);
pub struct ISecondaryTileStatics(pub *mut ::core::ffi::c_void);
pub struct ISecondaryTileVisualElements(pub *mut ::core::ffi::c_void);
pub struct ISecondaryTileVisualElements2(pub *mut ::core::ffi::c_void);
pub struct ISecondaryTileVisualElements3(pub *mut ::core::ffi::c_void);
pub struct ISecondaryTileVisualElements4(pub *mut ::core::ffi::c_void);
pub struct IStartScreenManager(pub *mut ::core::ffi::c_void);
pub struct IStartScreenManager2(pub *mut ::core::ffi::c_void);
pub struct IStartScreenManagerStatics(pub *mut ::core::ffi::c_void);
pub struct ITileMixedRealityModel(pub *mut ::core::ffi::c_void);
pub struct ITileMixedRealityModel2(pub *mut ::core::ffi::c_void);
pub struct IVisualElementsRequest(pub *mut ::core::ffi::c_void);
pub struct IVisualElementsRequestDeferral(pub *mut ::core::ffi::c_void);
pub struct IVisualElementsRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct JumpList(i32);
pub struct JumpListItem(i32);
pub struct JumpListItemKind(i32);
pub struct JumpListSystemGroupKind(i32);
pub struct SecondaryTile(i32);
pub struct SecondaryTileVisualElements(i32);
pub struct StartScreenManager(i32);
pub struct TileMixedRealityModel(i32);
pub struct TileMixedRealityModelActivationBehavior(i32);
pub struct TileOptions(i32);
pub struct TileSize(i32);
pub struct VisualElementsRequest(i32);
pub struct VisualElementsRequestDeferral(i32);
pub struct VisualElementsRequestedEventArgs(i32);
