#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ForegroundText(i32);
#[repr(transparent)]
pub struct IJumpList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJumpListItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJumpListItemStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJumpListStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryTile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryTile2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryTileFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryTileFactory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryTileStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryTileVisualElements(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryTileVisualElements2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryTileVisualElements3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryTileVisualElements4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStartScreenManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStartScreenManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStartScreenManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileMixedRealityModel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITileMixedRealityModel2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualElementsRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualElementsRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualElementsRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct JumpList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct JumpListItem(pub *mut ::core::ffi::c_void);
pub struct JumpListItemKind(i32);
pub struct JumpListSystemGroupKind(i32);
#[repr(transparent)]
pub struct SecondaryTile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SecondaryTileVisualElements(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StartScreenManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TileMixedRealityModel(pub *mut ::core::ffi::c_void);
pub struct TileMixedRealityModelActivationBehavior(i32);
pub struct TileOptions(i32);
pub struct TileSize(i32);
#[repr(transparent)]
pub struct VisualElementsRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VisualElementsRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VisualElementsRequestedEventArgs(pub *mut ::core::ffi::c_void);
