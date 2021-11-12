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
impl ::core::marker::Copy for IJumpList {}
impl ::core::clone::Clone for IJumpList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IJumpListItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IJumpListItem {}
impl ::core::clone::Clone for IJumpListItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IJumpListItemStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IJumpListItemStatics {}
impl ::core::clone::Clone for IJumpListItemStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IJumpListStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IJumpListStatics {}
impl ::core::clone::Clone for IJumpListStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISecondaryTile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISecondaryTile {}
impl ::core::clone::Clone for ISecondaryTile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISecondaryTile2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISecondaryTile2 {}
impl ::core::clone::Clone for ISecondaryTile2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISecondaryTileFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISecondaryTileFactory {}
impl ::core::clone::Clone for ISecondaryTileFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISecondaryTileFactory2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISecondaryTileFactory2 {}
impl ::core::clone::Clone for ISecondaryTileFactory2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISecondaryTileStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISecondaryTileStatics {}
impl ::core::clone::Clone for ISecondaryTileStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISecondaryTileVisualElements(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISecondaryTileVisualElements {}
impl ::core::clone::Clone for ISecondaryTileVisualElements {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISecondaryTileVisualElements2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISecondaryTileVisualElements2 {}
impl ::core::clone::Clone for ISecondaryTileVisualElements2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISecondaryTileVisualElements3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISecondaryTileVisualElements3 {}
impl ::core::clone::Clone for ISecondaryTileVisualElements3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISecondaryTileVisualElements4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISecondaryTileVisualElements4 {}
impl ::core::clone::Clone for ISecondaryTileVisualElements4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStartScreenManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStartScreenManager {}
impl ::core::clone::Clone for IStartScreenManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStartScreenManager2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStartScreenManager2 {}
impl ::core::clone::Clone for IStartScreenManager2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStartScreenManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStartScreenManagerStatics {}
impl ::core::clone::Clone for IStartScreenManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITileMixedRealityModel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITileMixedRealityModel {}
impl ::core::clone::Clone for ITileMixedRealityModel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITileMixedRealityModel2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITileMixedRealityModel2 {}
impl ::core::clone::Clone for ITileMixedRealityModel2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualElementsRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualElementsRequest {}
impl ::core::clone::Clone for IVisualElementsRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualElementsRequestDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualElementsRequestDeferral {}
impl ::core::clone::Clone for IVisualElementsRequestDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualElementsRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualElementsRequestedEventArgs {}
impl ::core::clone::Clone for IVisualElementsRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct JumpList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for JumpList {}
impl ::core::clone::Clone for JumpList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct JumpListItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for JumpListItem {}
impl ::core::clone::Clone for JumpListItem {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SecondaryTile {}
impl ::core::clone::Clone for SecondaryTile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SecondaryTileVisualElements(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SecondaryTileVisualElements {}
impl ::core::clone::Clone for SecondaryTileVisualElements {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StartScreenManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StartScreenManager {}
impl ::core::clone::Clone for StartScreenManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TileMixedRealityModel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TileMixedRealityModel {}
impl ::core::clone::Clone for TileMixedRealityModel {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for VisualElementsRequest {}
impl ::core::clone::Clone for VisualElementsRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VisualElementsRequestDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VisualElementsRequestDeferral {}
impl ::core::clone::Clone for VisualElementsRequestDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VisualElementsRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VisualElementsRequestedEventArgs {}
impl ::core::clone::Clone for VisualElementsRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
