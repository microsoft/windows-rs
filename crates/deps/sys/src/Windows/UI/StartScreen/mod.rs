#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ForegroundText(pub i32);
impl ForegroundText {
    pub const Dark: ForegroundText = ForegroundText(0i32);
    pub const Light: ForegroundText = ForegroundText(1i32);
}
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
#[repr(transparent)]
pub struct JumpListItemKind(pub i32);
impl JumpListItemKind {
    pub const Arguments: JumpListItemKind = JumpListItemKind(0i32);
    pub const Separator: JumpListItemKind = JumpListItemKind(1i32);
}
#[repr(transparent)]
pub struct JumpListSystemGroupKind(pub i32);
impl JumpListSystemGroupKind {
    pub const None: JumpListSystemGroupKind = JumpListSystemGroupKind(0i32);
    pub const Frequent: JumpListSystemGroupKind = JumpListSystemGroupKind(1i32);
    pub const Recent: JumpListSystemGroupKind = JumpListSystemGroupKind(2i32);
}
#[repr(transparent)]
pub struct SecondaryTile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SecondaryTileVisualElements(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StartScreenManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TileMixedRealityModel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TileMixedRealityModelActivationBehavior(pub i32);
impl TileMixedRealityModelActivationBehavior {
    pub const Default: TileMixedRealityModelActivationBehavior = TileMixedRealityModelActivationBehavior(0i32);
    pub const None: TileMixedRealityModelActivationBehavior = TileMixedRealityModelActivationBehavior(1i32);
}
#[repr(transparent)]
pub struct TileOptions(pub u32);
impl TileOptions {
    pub const None: TileOptions = TileOptions(0u32);
    pub const ShowNameOnLogo: TileOptions = TileOptions(1u32);
    pub const ShowNameOnWideLogo: TileOptions = TileOptions(2u32);
    pub const CopyOnDeployment: TileOptions = TileOptions(4u32);
}
#[repr(transparent)]
pub struct TileSize(pub i32);
impl TileSize {
    pub const Default: TileSize = TileSize(0i32);
    pub const Square30x30: TileSize = TileSize(1i32);
    pub const Square70x70: TileSize = TileSize(2i32);
    pub const Square150x150: TileSize = TileSize(3i32);
    pub const Wide310x150: TileSize = TileSize(4i32);
    pub const Square310x310: TileSize = TileSize(5i32);
    pub const Square71x71: TileSize = TileSize(6i32);
    pub const Square44x44: TileSize = TileSize(7i32);
}
#[repr(transparent)]
pub struct VisualElementsRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VisualElementsRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VisualElementsRequestedEventArgs(pub *mut ::core::ffi::c_void);
