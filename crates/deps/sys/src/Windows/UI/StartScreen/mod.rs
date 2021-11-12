#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ForegroundText(pub i32);
impl ForegroundText {
    pub const Dark: Self = Self(0i32);
    pub const Light: Self = Self(1i32);
}
impl ::core::marker::Copy for ForegroundText {}
impl ::core::clone::Clone for ForegroundText {
    fn clone(&self) -> Self {
        *self
    }
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
    pub const Arguments: Self = Self(0i32);
    pub const Separator: Self = Self(1i32);
}
impl ::core::marker::Copy for JumpListItemKind {}
impl ::core::clone::Clone for JumpListItemKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct JumpListSystemGroupKind(pub i32);
impl JumpListSystemGroupKind {
    pub const None: Self = Self(0i32);
    pub const Frequent: Self = Self(1i32);
    pub const Recent: Self = Self(2i32);
}
impl ::core::marker::Copy for JumpListSystemGroupKind {}
impl ::core::clone::Clone for JumpListSystemGroupKind {
    fn clone(&self) -> Self {
        *self
    }
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
    pub const Default: Self = Self(0i32);
    pub const None: Self = Self(1i32);
}
impl ::core::marker::Copy for TileMixedRealityModelActivationBehavior {}
impl ::core::clone::Clone for TileMixedRealityModelActivationBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TileOptions(pub u32);
impl TileOptions {
    pub const None: Self = Self(0u32);
    pub const ShowNameOnLogo: Self = Self(1u32);
    pub const ShowNameOnWideLogo: Self = Self(2u32);
    pub const CopyOnDeployment: Self = Self(4u32);
}
impl ::core::marker::Copy for TileOptions {}
impl ::core::clone::Clone for TileOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TileSize(pub i32);
impl TileSize {
    pub const Default: Self = Self(0i32);
    pub const Square30x30: Self = Self(1i32);
    pub const Square70x70: Self = Self(2i32);
    pub const Square150x150: Self = Self(3i32);
    pub const Wide310x150: Self = Self(4i32);
    pub const Square310x310: Self = Self(5i32);
    pub const Square71x71: Self = Self(6i32);
    pub const Square44x44: Self = Self(7i32);
}
impl ::core::marker::Copy for TileSize {}
impl ::core::clone::Clone for TileSize {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VisualElementsRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VisualElementsRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VisualElementsRequestedEventArgs(pub *mut ::core::ffi::c_void);
