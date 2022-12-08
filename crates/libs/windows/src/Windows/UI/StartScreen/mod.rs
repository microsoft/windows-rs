#[doc(hidden)]
#[repr(transparent)]
pub struct IJumpList(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IJumpList {
    type Vtable = IJumpList_Vtbl;
}
unsafe impl ::windows::core::Interface for IJumpList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0234c3e_cd6f_4cb6_a611_61fd505f3ed1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpList_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
    pub SystemGroupKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut JumpListSystemGroupKind) -> ::windows::core::HRESULT,
    pub SetSystemGroupKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: JumpListSystemGroupKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJumpListItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IJumpListItem {
    type Vtable = IJumpListItem_Vtbl;
}
unsafe impl ::windows::core::Interface for IJumpListItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7adb6717_8b5d_4820_995b_9b418dbe48b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListItem_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut JumpListItemKind) -> ::windows::core::HRESULT,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemovedByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GroupName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetGroupName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Logo: usize,
    #[cfg(feature = "Foundation")]
    pub SetLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLogo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJumpListItemStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IJumpListItemStatics {
    type Vtable = IJumpListItemStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IJumpListItemStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1bfc4e8_c7aa_49cb_8dde_ecfccd7ad7e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListItemStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWithArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arguments: *mut ::core::ffi::c_void, displayname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSeparator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJumpListStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IJumpListStatics {
    type Vtable = IJumpListStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IJumpListStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7e0c681_e67e_4b74_8250_3f322c4d92c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub LoadCurrentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadCurrentAsync: usize,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryTile(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISecondaryTile {
    type Vtable = ISecondaryTile_Vtbl;
}
unsafe impl ::windows::core::Interface for ISecondaryTile {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e9e51e0_2bb5_4bc0_bb8d_42b23abcc88d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTile_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetTileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub SetShortName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetShortName: usize,
    #[cfg(feature = "deprecated")]
    pub ShortName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShortName: usize,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetLogo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Logo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetSmallLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetSmallLogo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SmallLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SmallLogo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetWideLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetWideLogo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub WideLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    WideLogo: usize,
    #[cfg(feature = "Foundation")]
    pub SetLockScreenBadgeLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLockScreenBadgeLogo: usize,
    #[cfg(feature = "Foundation")]
    pub LockScreenBadgeLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LockScreenBadgeLogo: usize,
    pub SetLockScreenDisplayBadgeAndTileText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub LockScreenDisplayBadgeAndTileText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub SetTileOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TileOptions) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetTileOptions: usize,
    #[cfg(feature = "deprecated")]
    pub TileOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TileOptions) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TileOptions: usize,
    #[cfg(feature = "deprecated")]
    pub SetForegroundText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ForegroundText) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetForegroundText: usize,
    #[cfg(feature = "deprecated")]
    pub ForegroundText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ForegroundText) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ForegroundText: usize,
    #[cfg(feature = "deprecated")]
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetBackgroundColor: usize,
    #[cfg(feature = "deprecated")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub RequestCreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCreateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestCreateAsyncWithPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, invocationpoint: super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCreateAsyncWithPoint: usize,
    #[cfg(feature = "Foundation")]
    pub RequestCreateAsyncWithRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCreateAsyncWithRect: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub RequestCreateAsyncWithRectAndPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    RequestCreateAsyncWithRectAndPlacement: usize,
    #[cfg(feature = "Foundation")]
    pub RequestDeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestDeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestDeleteAsyncWithPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, invocationpoint: super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestDeleteAsyncWithPoint: usize,
    #[cfg(feature = "Foundation")]
    pub RequestDeleteAsyncWithRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestDeleteAsyncWithRect: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub RequestDeleteAsyncWithRectAndPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    RequestDeleteAsyncWithRectAndPlacement: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryTile2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISecondaryTile2 {
    type Vtable = ISecondaryTile2_Vtbl;
}
unsafe impl ::windows::core::Interface for ISecondaryTile2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2f6cc35_3250_4990_923c_294ab4b694dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTile2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetPhoneticName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PhoneticName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub VisualElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRoamingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub RoamingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub VisualElementsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VisualElementsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVisualElementsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVisualElementsRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryTileFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISecondaryTileFactory {
    type Vtable = ISecondaryTileFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ISecondaryTileFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57f52ca0_51bc_4abf_8ebf_627a0398b05a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CreateTile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: *mut ::core::ffi::c_void, shortname: *mut ::core::ffi::c_void, displayname: *mut ::core::ffi::c_void, arguments: *mut ::core::ffi::c_void, tileoptions: TileOptions, logoreference: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CreateTile: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CreateWideTile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: *mut ::core::ffi::c_void, shortname: *mut ::core::ffi::c_void, displayname: *mut ::core::ffi::c_void, arguments: *mut ::core::ffi::c_void, tileoptions: TileOptions, logoreference: *mut ::core::ffi::c_void, widelogoreference: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CreateWideTile: usize,
    pub CreateWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryTileFactory2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISecondaryTileFactory2 {
    type Vtable = ISecondaryTileFactory2_Vtbl;
}
unsafe impl ::windows::core::Interface for ISecondaryTileFactory2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x274b8a3b_522d_448e_9eb2_d0672ab345c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileFactory2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateMinimalTile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: *mut ::core::ffi::c_void, displayname: *mut ::core::ffi::c_void, arguments: *mut ::core::ffi::c_void, square150x150logo: *mut ::core::ffi::c_void, desiredsize: TileSize, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateMinimalTile: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryTileStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISecondaryTileStatics {
    type Vtable = ISecondaryTileStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ISecondaryTileStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99908dae_d051_4676_87fe_9ec242d83c74);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Exists: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllForApplicationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllForApplicationAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllForPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllForPackageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryTileVisualElements(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISecondaryTileVisualElements {
    type Vtable = ISecondaryTileVisualElements_Vtbl;
}
unsafe impl ::windows::core::Interface for ISecondaryTileVisualElements {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d8df333_815e_413f_9f50_a81da70a96b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetSquare30x30Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetSquare30x30Logo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Square30x30Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Square30x30Logo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetSquare70x70Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetSquare70x70Logo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Square70x70Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Square70x70Logo: usize,
    #[cfg(feature = "Foundation")]
    pub SetSquare150x150Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSquare150x150Logo: usize,
    #[cfg(feature = "Foundation")]
    pub Square150x150Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Square150x150Logo: usize,
    #[cfg(feature = "Foundation")]
    pub SetWide310x150Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetWide310x150Logo: usize,
    #[cfg(feature = "Foundation")]
    pub Wide310x150Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Wide310x150Logo: usize,
    #[cfg(feature = "Foundation")]
    pub SetSquare310x310Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSquare310x310Logo: usize,
    #[cfg(feature = "Foundation")]
    pub Square310x310Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Square310x310Logo: usize,
    pub SetForegroundText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ForegroundText) -> ::windows::core::HRESULT,
    pub ForegroundText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ForegroundText) -> ::windows::core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows::core::HRESULT,
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows::core::HRESULT,
    pub SetShowNameOnSquare150x150Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ShowNameOnSquare150x150Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetShowNameOnWide310x150Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ShowNameOnWide310x150Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetShowNameOnSquare310x310Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ShowNameOnSquare310x310Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryTileVisualElements2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISecondaryTileVisualElements2 {
    type Vtable = ISecondaryTileVisualElements2_Vtbl;
}
unsafe impl ::windows::core::Interface for ISecondaryTileVisualElements2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd2e31d0_57dc_4794_8ecf_5682f5f3e6ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetSquare71x71Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSquare71x71Logo: usize,
    #[cfg(feature = "Foundation")]
    pub Square71x71Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Square71x71Logo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryTileVisualElements3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISecondaryTileVisualElements3 {
    type Vtable = ISecondaryTileVisualElements3_Vtbl;
}
unsafe impl ::windows::core::Interface for ISecondaryTileVisualElements3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56b55ad6_d15c_40f4_81e7_57ffd8f8a4e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetSquare44x44Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSquare44x44Logo: usize,
    #[cfg(feature = "Foundation")]
    pub Square44x44Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Square44x44Logo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryTileVisualElements4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISecondaryTileVisualElements4 {
    type Vtable = ISecondaryTileVisualElements4_Vtbl;
}
unsafe impl ::windows::core::Interface for ISecondaryTileVisualElements4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66566117_b544_40d2_8d12_74d4ec24d04c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MixedRealityModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStartScreenManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStartScreenManager {
    type Vtable = IStartScreenManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IStartScreenManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a1dcbcb_26e9_4eb4_8933_859eb6ecdb29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartScreenManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    #[cfg(feature = "ApplicationModel_Core")]
    pub SupportsAppListEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applistentry: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))]
    SupportsAppListEntry: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub ContainsAppListEntryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applistentry: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))]
    ContainsAppListEntryAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub RequestAddAppListEntryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applistentry: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))]
    RequestAddAppListEntryAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStartScreenManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStartScreenManager2 {
    type Vtable = IStartScreenManager2_Vtbl;
}
unsafe impl ::windows::core::Interface for IStartScreenManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08a716b6_316b_4ad9_acb8_fe9cf00bd608);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartScreenManager2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ContainsSecondaryTileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContainsSecondaryTileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryRemoveSecondaryTileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryRemoveSecondaryTileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStartScreenManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStartScreenManagerStatics {
    type Vtable = IStartScreenManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IStartScreenManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7865ef0f_b585_464e_8993_34e8f8738d48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartScreenManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileMixedRealityModel(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITileMixedRealityModel {
    type Vtable = ITileMixedRealityModel_Vtbl;
}
unsafe impl ::windows::core::Interface for ITileMixedRealityModel {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0764e5b_887d_4242_9a19_3d0a4ea78031);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileMixedRealityModel_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetBoundingBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetBoundingBox: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub BoundingBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    BoundingBox: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileMixedRealityModel2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITileMixedRealityModel2 {
    type Vtable = ITileMixedRealityModel2_Vtbl;
}
unsafe impl ::windows::core::Interface for ITileMixedRealityModel2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x439470b2_d7c5_410b_8319_9486a27b6c67);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileMixedRealityModel2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetActivationBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TileMixedRealityModelActivationBehavior) -> ::windows::core::HRESULT,
    pub ActivationBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TileMixedRealityModelActivationBehavior) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualElementsRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IVisualElementsRequest {
    type Vtable = IVisualElementsRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IVisualElementsRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc138333a_9308_4072_88cc_d068db347c68);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualElementsRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub VisualElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AlternateVisualElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AlternateVisualElements: usize,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualElementsRequestDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IVisualElementsRequestDeferral {
    type Vtable = IVisualElementsRequestDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for IVisualElementsRequestDeferral {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1656eb0_0126_4357_8204_bd82bb2a046d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualElementsRequestDeferral_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualElementsRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IVisualElementsRequestedEventArgs {
    type Vtable = IVisualElementsRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IVisualElementsRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b6fc982_3a0d_4ece_af96_cd17e1b00b2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualElementsRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct JumpList(::windows::core::IUnknown);
impl JumpList {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<JumpListItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Items)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SystemGroupKind(&self) -> ::windows::core::Result<JumpListSystemGroupKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemGroupKind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSystemGroupKind(&self, value: JumpListSystemGroupKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSystemGroupKind)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SaveAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoadCurrentAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<JumpList>> {
        Self::IJumpListStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LoadCurrentAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IJumpListStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IJumpListStatics<R, F: FnOnce(&IJumpListStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<JumpList, IJumpListStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for JumpList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for JumpList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JumpList {}
impl ::core::fmt::Debug for JumpList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JumpList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for JumpList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.JumpList;{b0234c3e-cd6f-4cb6-a611-61fd505f3ed1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for JumpList {
    type Vtable = IJumpList_Vtbl;
}
unsafe impl ::windows::core::Interface for JumpList {
    const IID: ::windows::core::GUID = <IJumpList as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for JumpList {
    const NAME: &'static str = "Windows.UI.StartScreen.JumpList";
}
::windows::core::interface_hierarchy!(JumpList, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for JumpList {}
unsafe impl ::core::marker::Sync for JumpList {}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct JumpListItem(::windows::core::IUnknown);
impl JumpListItem {
    pub fn Kind(&self) -> ::windows::core::Result<JumpListItemKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Arguments)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemovedByUser(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemovedByUser)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDescription)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDisplayName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn GroupName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GroupName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetGroupName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetGroupName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Logo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLogo(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLogo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CreateWithArguments(arguments: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<JumpListItem> {
        Self::IJumpListItemStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithArguments)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(arguments), ::core::mem::transmute_copy(displayname), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateSeparator() -> ::windows::core::Result<JumpListItem> {
        Self::IJumpListItemStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSeparator)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IJumpListItemStatics<R, F: FnOnce(&IJumpListItemStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<JumpListItem, IJumpListItemStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for JumpListItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for JumpListItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JumpListItem {}
impl ::core::fmt::Debug for JumpListItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JumpListItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for JumpListItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.JumpListItem;{7adb6717-8b5d-4820-995b-9b418dbe48b0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for JumpListItem {
    type Vtable = IJumpListItem_Vtbl;
}
unsafe impl ::windows::core::Interface for JumpListItem {
    const IID: ::windows::core::GUID = <IJumpListItem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for JumpListItem {
    const NAME: &'static str = "Windows.UI.StartScreen.JumpListItem";
}
::windows::core::interface_hierarchy!(JumpListItem, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for JumpListItem {}
unsafe impl ::core::marker::Sync for JumpListItem {}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct SecondaryTile(::windows::core::IUnknown);
impl SecondaryTile {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SecondaryTile, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetTileId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTileId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TileId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetArguments(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetArguments)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Arguments)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetShortName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetShortName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ShortName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShortName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDisplayName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetLogo(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLogo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Logo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetSmallLogo(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSmallLogo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SmallLogo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SmallLogo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetWideLogo(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetWideLogo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn WideLogo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WideLogo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLockScreenBadgeLogo(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLockScreenBadgeLogo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LockScreenBadgeLogo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LockScreenBadgeLogo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLockScreenDisplayBadgeAndTileText(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLockScreenDisplayBadgeAndTileText)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn LockScreenDisplayBadgeAndTileText(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LockScreenDisplayBadgeAndTileText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetTileOptions(&self, value: TileOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTileOptions)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn TileOptions(&self) -> ::windows::core::Result<TileOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TileOptions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetForegroundText(&self, value: ForegroundText) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetForegroundText)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ForegroundText(&self) -> ::windows::core::Result<ForegroundText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ForegroundText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetBackgroundColor(&self, value: super::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBackgroundColor)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BackgroundColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestCreateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestCreateAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestCreateAsyncWithPoint(&self, invocationpoint: super::super::Foundation::Point) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestCreateAsyncWithPoint)(::windows::core::Vtable::as_raw(this), invocationpoint, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestCreateAsyncWithRect(&self, selection: super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestCreateAsyncWithRect)(::windows::core::Vtable::as_raw(this), selection, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn RequestCreateAsyncWithRectAndPlacement(&self, selection: super::super::Foundation::Rect, preferredplacement: super::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestCreateAsyncWithRectAndPlacement)(::windows::core::Vtable::as_raw(this), selection, preferredplacement, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestDeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestDeleteAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestDeleteAsyncWithPoint(&self, invocationpoint: super::super::Foundation::Point) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestDeleteAsyncWithPoint)(::windows::core::Vtable::as_raw(this), invocationpoint, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestDeleteAsyncWithRect(&self, selection: super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestDeleteAsyncWithRect)(::windows::core::Vtable::as_raw(this), selection, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn RequestDeleteAsyncWithRectAndPlacement(&self, selection: super::super::Foundation::Rect, preferredplacement: super::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestDeleteAsyncWithRectAndPlacement)(::windows::core::Vtable::as_raw(this), selection, preferredplacement, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPhoneticName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISecondaryTile2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPhoneticName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn PhoneticName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ISecondaryTile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhoneticName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn VisualElements(&self) -> ::windows::core::Result<SecondaryTileVisualElements> {
        let this = &::windows::core::Interface::cast::<ISecondaryTile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VisualElements)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRoamingEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISecondaryTile2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetRoamingEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn RoamingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISecondaryTile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RoamingEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VisualElementsRequested(&self, handler: &super::super::Foundation::TypedEventHandler<SecondaryTile, VisualElementsRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<ISecondaryTile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VisualElementsRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVisualElementsRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISecondaryTile2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveVisualElementsRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn CreateTile(tileid: &::windows::core::HSTRING, shortname: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING, arguments: &::windows::core::HSTRING, tileoptions: TileOptions, logoreference: &super::super::Foundation::Uri) -> ::windows::core::Result<SecondaryTile> {
        Self::ISecondaryTileFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateTile)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tileid), ::core::mem::transmute_copy(shortname), ::core::mem::transmute_copy(displayname), ::core::mem::transmute_copy(arguments), tileoptions, ::core::mem::transmute_copy(logoreference), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn CreateWideTile(tileid: &::windows::core::HSTRING, shortname: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING, arguments: &::windows::core::HSTRING, tileoptions: TileOptions, logoreference: &super::super::Foundation::Uri, widelogoreference: &super::super::Foundation::Uri) -> ::windows::core::Result<SecondaryTile> {
        Self::ISecondaryTileFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWideTile)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tileid), ::core::mem::transmute_copy(shortname), ::core::mem::transmute_copy(displayname), ::core::mem::transmute_copy(arguments), tileoptions, ::core::mem::transmute_copy(logoreference), ::core::mem::transmute_copy(widelogoreference), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateWithId(tileid: &::windows::core::HSTRING) -> ::windows::core::Result<SecondaryTile> {
        Self::ISecondaryTileFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tileid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateMinimalTile(tileid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING, arguments: &::windows::core::HSTRING, square150x150logo: &super::super::Foundation::Uri, desiredsize: TileSize) -> ::windows::core::Result<SecondaryTile> {
        Self::ISecondaryTileFactory2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateMinimalTile)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tileid), ::core::mem::transmute_copy(displayname), ::core::mem::transmute_copy(arguments), ::core::mem::transmute_copy(square150x150logo), desiredsize, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Exists(tileid: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        Self::ISecondaryTileStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Exists)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tileid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>> {
        Self::ISecondaryTileStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindAllAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllForApplicationAsync(applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>> {
        Self::ISecondaryTileStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindAllForApplicationAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(applicationid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllForPackageAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>> {
        Self::ISecondaryTileStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindAllForPackageAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISecondaryTileFactory<R, F: FnOnce(&ISecondaryTileFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SecondaryTile, ISecondaryTileFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISecondaryTileFactory2<R, F: FnOnce(&ISecondaryTileFactory2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SecondaryTile, ISecondaryTileFactory2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISecondaryTileStatics<R, F: FnOnce(&ISecondaryTileStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SecondaryTile, ISecondaryTileStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SecondaryTile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SecondaryTile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecondaryTile {}
impl ::core::fmt::Debug for SecondaryTile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryTile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SecondaryTile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.SecondaryTile;{9e9e51e0-2bb5-4bc0-bb8d-42b23abcc88d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SecondaryTile {
    type Vtable = ISecondaryTile_Vtbl;
}
unsafe impl ::windows::core::Interface for SecondaryTile {
    const IID: ::windows::core::GUID = <ISecondaryTile as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SecondaryTile {
    const NAME: &'static str = "Windows.UI.StartScreen.SecondaryTile";
}
::windows::core::interface_hierarchy!(SecondaryTile, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SecondaryTile {}
unsafe impl ::core::marker::Sync for SecondaryTile {}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct SecondaryTileVisualElements(::windows::core::IUnknown);
impl SecondaryTileVisualElements {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetSquare30x30Logo(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSquare30x30Logo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Square30x30Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Square30x30Logo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetSquare70x70Logo(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSquare70x70Logo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Square70x70Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Square70x70Logo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSquare150x150Logo(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSquare150x150Logo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Square150x150Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Square150x150Logo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetWide310x150Logo(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetWide310x150Logo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Wide310x150Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Wide310x150Logo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSquare310x310Logo(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSquare310x310Logo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Square310x310Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Square310x310Logo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetForegroundText(&self, value: ForegroundText) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetForegroundText)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ForegroundText(&self) -> ::windows::core::Result<ForegroundText> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ForegroundText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetBackgroundColor(&self, value: super::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBackgroundColor)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BackgroundColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetShowNameOnSquare150x150Logo(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetShowNameOnSquare150x150Logo)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ShowNameOnSquare150x150Logo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShowNameOnSquare150x150Logo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetShowNameOnWide310x150Logo(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetShowNameOnWide310x150Logo)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ShowNameOnWide310x150Logo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShowNameOnWide310x150Logo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetShowNameOnSquare310x310Logo(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetShowNameOnSquare310x310Logo)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ShowNameOnSquare310x310Logo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShowNameOnSquare310x310Logo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSquare71x71Logo(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISecondaryTileVisualElements2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetSquare71x71Logo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Square71x71Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<ISecondaryTileVisualElements2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Square71x71Logo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSquare44x44Logo(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISecondaryTileVisualElements3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetSquare44x44Logo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Square44x44Logo(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<ISecondaryTileVisualElements3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Square44x44Logo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MixedRealityModel(&self) -> ::windows::core::Result<TileMixedRealityModel> {
        let this = &::windows::core::Interface::cast::<ISecondaryTileVisualElements4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MixedRealityModel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for SecondaryTileVisualElements {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SecondaryTileVisualElements {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SecondaryTileVisualElements {}
impl ::core::fmt::Debug for SecondaryTileVisualElements {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryTileVisualElements").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SecondaryTileVisualElements {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.SecondaryTileVisualElements;{1d8df333-815e-413f-9f50-a81da70a96b2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SecondaryTileVisualElements {
    type Vtable = ISecondaryTileVisualElements_Vtbl;
}
unsafe impl ::windows::core::Interface for SecondaryTileVisualElements {
    const IID: ::windows::core::GUID = <ISecondaryTileVisualElements as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SecondaryTileVisualElements {
    const NAME: &'static str = "Windows.UI.StartScreen.SecondaryTileVisualElements";
}
::windows::core::interface_hierarchy!(SecondaryTileVisualElements, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SecondaryTileVisualElements {}
unsafe impl ::core::marker::Sync for SecondaryTileVisualElements {}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct StartScreenManager(::windows::core::IUnknown);
impl StartScreenManager {
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    #[cfg(feature = "ApplicationModel_Core")]
    pub fn SupportsAppListEntry(&self, applistentry: &super::super::ApplicationModel::Core::AppListEntry) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportsAppListEntry)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(applistentry), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub fn ContainsAppListEntryAsync(&self, applistentry: &super::super::ApplicationModel::Core::AppListEntry) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContainsAppListEntryAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(applistentry), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub fn RequestAddAppListEntryAsync(&self, applistentry: &super::super::ApplicationModel::Core::AppListEntry) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestAddAppListEntryAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(applistentry), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContainsSecondaryTileAsync(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IStartScreenManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContainsSecondaryTileAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tileid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryRemoveSecondaryTileAsync(&self, tileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IStartScreenManager2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryRemoveSecondaryTileAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tileid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<StartScreenManager> {
        Self::IStartScreenManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser(user: &super::super::System::User) -> ::windows::core::Result<StartScreenManager> {
        Self::IStartScreenManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStartScreenManagerStatics<R, F: FnOnce(&IStartScreenManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StartScreenManager, IStartScreenManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for StartScreenManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StartScreenManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StartScreenManager {}
impl ::core::fmt::Debug for StartScreenManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StartScreenManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StartScreenManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.StartScreenManager;{4a1dcbcb-26e9-4eb4-8933-859eb6ecdb29})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StartScreenManager {
    type Vtable = IStartScreenManager_Vtbl;
}
unsafe impl ::windows::core::Interface for StartScreenManager {
    const IID: ::windows::core::GUID = <IStartScreenManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StartScreenManager {
    const NAME: &'static str = "Windows.UI.StartScreen.StartScreenManager";
}
::windows::core::interface_hierarchy!(StartScreenManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StartScreenManager {}
unsafe impl ::core::marker::Sync for StartScreenManager {}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct TileMixedRealityModel(::windows::core::IUnknown);
impl TileMixedRealityModel {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Perception_Spatial\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetBoundingBox<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingBox>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBoundingBox)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Perception_Spatial\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn BoundingBox(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingBox>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BoundingBox)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetActivationBehavior(&self, value: TileMixedRealityModelActivationBehavior) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITileMixedRealityModel2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetActivationBehavior)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ActivationBehavior(&self) -> ::windows::core::Result<TileMixedRealityModelActivationBehavior> {
        let this = &::windows::core::Interface::cast::<ITileMixedRealityModel2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActivationBehavior)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for TileMixedRealityModel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TileMixedRealityModel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileMixedRealityModel {}
impl ::core::fmt::Debug for TileMixedRealityModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileMixedRealityModel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TileMixedRealityModel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.TileMixedRealityModel;{b0764e5b-887d-4242-9a19-3d0a4ea78031})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for TileMixedRealityModel {
    type Vtable = ITileMixedRealityModel_Vtbl;
}
unsafe impl ::windows::core::Interface for TileMixedRealityModel {
    const IID: ::windows::core::GUID = <ITileMixedRealityModel as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TileMixedRealityModel {
    const NAME: &'static str = "Windows.UI.StartScreen.TileMixedRealityModel";
}
::windows::core::interface_hierarchy!(TileMixedRealityModel, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for TileMixedRealityModel {}
unsafe impl ::core::marker::Sync for TileMixedRealityModel {}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct VisualElementsRequest(::windows::core::IUnknown);
impl VisualElementsRequest {
    pub fn VisualElements(&self) -> ::windows::core::Result<SecondaryTileVisualElements> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VisualElements)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AlternateVisualElements(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SecondaryTileVisualElements>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AlternateVisualElements)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Deadline)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<VisualElementsRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for VisualElementsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VisualElementsRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VisualElementsRequest {}
impl ::core::fmt::Debug for VisualElementsRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisualElementsRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VisualElementsRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.VisualElementsRequest;{c138333a-9308-4072-88cc-d068db347c68})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for VisualElementsRequest {
    type Vtable = IVisualElementsRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for VisualElementsRequest {
    const IID: ::windows::core::GUID = <IVisualElementsRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VisualElementsRequest {
    const NAME: &'static str = "Windows.UI.StartScreen.VisualElementsRequest";
}
::windows::core::interface_hierarchy!(VisualElementsRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for VisualElementsRequest {}
unsafe impl ::core::marker::Sync for VisualElementsRequest {}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct VisualElementsRequestDeferral(::windows::core::IUnknown);
impl VisualElementsRequestDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Complete)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for VisualElementsRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VisualElementsRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VisualElementsRequestDeferral {}
impl ::core::fmt::Debug for VisualElementsRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisualElementsRequestDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VisualElementsRequestDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.VisualElementsRequestDeferral;{a1656eb0-0126-4357-8204-bd82bb2a046d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for VisualElementsRequestDeferral {
    type Vtable = IVisualElementsRequestDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for VisualElementsRequestDeferral {
    const IID: ::windows::core::GUID = <IVisualElementsRequestDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VisualElementsRequestDeferral {
    const NAME: &'static str = "Windows.UI.StartScreen.VisualElementsRequestDeferral";
}
::windows::core::interface_hierarchy!(VisualElementsRequestDeferral, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for VisualElementsRequestDeferral {}
unsafe impl ::core::marker::Sync for VisualElementsRequestDeferral {}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct VisualElementsRequestedEventArgs(::windows::core::IUnknown);
impl VisualElementsRequestedEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<VisualElementsRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for VisualElementsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VisualElementsRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VisualElementsRequestedEventArgs {}
impl ::core::fmt::Debug for VisualElementsRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisualElementsRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VisualElementsRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.VisualElementsRequestedEventArgs;{7b6fc982-3a0d-4ece-af96-cd17e1b00b2d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for VisualElementsRequestedEventArgs {
    type Vtable = IVisualElementsRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for VisualElementsRequestedEventArgs {
    const IID: ::windows::core::GUID = <IVisualElementsRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VisualElementsRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.StartScreen.VisualElementsRequestedEventArgs";
}
::windows::core::interface_hierarchy!(VisualElementsRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for VisualElementsRequestedEventArgs {}
unsafe impl ::core::marker::Sync for VisualElementsRequestedEventArgs {}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for ForegroundText {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ForegroundText {
    type Abi = Self;
}
impl ::core::fmt::Debug for ForegroundText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ForegroundText").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ForegroundText {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.ForegroundText;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for JumpListItemKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JumpListItemKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for JumpListItemKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JumpListItemKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for JumpListItemKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.JumpListItemKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for JumpListSystemGroupKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JumpListSystemGroupKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for JumpListSystemGroupKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JumpListSystemGroupKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for JumpListSystemGroupKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.JumpListSystemGroupKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for TileMixedRealityModelActivationBehavior {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TileMixedRealityModelActivationBehavior {
    type Abi = Self;
}
impl ::core::fmt::Debug for TileMixedRealityModelActivationBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileMixedRealityModelActivationBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TileMixedRealityModelActivationBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.TileMixedRealityModelActivationBehavior;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for TileOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TileOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for TileOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TileOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TileOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TileOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TileOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TileOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for TileOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.TileOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for TileSize {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TileSize {
    type Abi = Self;
}
impl ::core::fmt::Debug for TileSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileSize").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TileSize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.TileSize;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
