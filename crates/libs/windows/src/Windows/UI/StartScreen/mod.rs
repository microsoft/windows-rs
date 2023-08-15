#[doc(hidden)]
#[repr(transparent)]
pub struct IJumpList(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJumpList {
    type Vtable = IJumpList_Vtbl;
}
impl ::core::clone::Clone for IJumpList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IJumpList {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb0234c3e_cd6f_4cb6_a611_61fd505f3ed1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpList_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
    pub SystemGroupKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut JumpListSystemGroupKind) -> ::windows_core::HRESULT,
    pub SetSystemGroupKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: JumpListSystemGroupKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJumpListItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJumpListItem {
    type Vtable = IJumpListItem_Vtbl;
}
impl ::core::clone::Clone for IJumpListItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IJumpListItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7adb6717_8b5d_4820_995b_9b418dbe48b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListItem_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut JumpListItemKind) -> ::windows_core::HRESULT,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemovedByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GroupName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetGroupName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Logo: usize,
    #[cfg(feature = "Foundation")]
    pub SetLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLogo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJumpListItemStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJumpListItemStatics {
    type Vtable = IJumpListItemStatics_Vtbl;
}
impl ::core::clone::Clone for IJumpListItemStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IJumpListItemStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf1bfc4e8_c7aa_49cb_8dde_ecfccd7ad7e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListItemStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateWithArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arguments: ::std::mem::MaybeUninit<::windows_core::HSTRING>, displayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSeparator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJumpListStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJumpListStatics {
    type Vtable = IJumpListStatics_Vtbl;
}
impl ::core::clone::Clone for IJumpListStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IJumpListStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7e0c681_e67e_4b74_8250_3f322c4d92c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub LoadCurrentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadCurrentAsync: usize,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryTile(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISecondaryTile {
    type Vtable = ISecondaryTile_Vtbl;
}
impl ::core::clone::Clone for ISecondaryTile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISecondaryTile {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e9e51e0_2bb5_4bc0_bb8d_42b23abcc88d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTile_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetTileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub SetShortName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetShortName: usize,
    #[cfg(feature = "deprecated")]
    pub ShortName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ShortName: usize,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetLogo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Logo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetSmallLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetSmallLogo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SmallLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SmallLogo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetWideLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetWideLogo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub WideLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    WideLogo: usize,
    #[cfg(feature = "Foundation")]
    pub SetLockScreenBadgeLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLockScreenBadgeLogo: usize,
    #[cfg(feature = "Foundation")]
    pub LockScreenBadgeLogo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LockScreenBadgeLogo: usize,
    pub SetLockScreenDisplayBadgeAndTileText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub LockScreenDisplayBadgeAndTileText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub SetTileOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TileOptions) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetTileOptions: usize,
    #[cfg(feature = "deprecated")]
    pub TileOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TileOptions) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TileOptions: usize,
    #[cfg(feature = "deprecated")]
    pub SetForegroundText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ForegroundText) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetForegroundText: usize,
    #[cfg(feature = "deprecated")]
    pub ForegroundText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ForegroundText) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ForegroundText: usize,
    #[cfg(feature = "deprecated")]
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetBackgroundColor: usize,
    #[cfg(feature = "deprecated")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub RequestCreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCreateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestCreateAsyncWithPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, invocationpoint: super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCreateAsyncWithPoint: usize,
    #[cfg(feature = "Foundation")]
    pub RequestCreateAsyncWithRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCreateAsyncWithRect: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub RequestCreateAsyncWithRectAndPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    RequestCreateAsyncWithRectAndPlacement: usize,
    #[cfg(feature = "Foundation")]
    pub RequestDeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestDeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestDeleteAsyncWithPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, invocationpoint: super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestDeleteAsyncWithPoint: usize,
    #[cfg(feature = "Foundation")]
    pub RequestDeleteAsyncWithRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestDeleteAsyncWithRect: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub RequestDeleteAsyncWithRectAndPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    RequestDeleteAsyncWithRectAndPlacement: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryTile2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISecondaryTile2 {
    type Vtable = ISecondaryTile2_Vtbl;
}
impl ::core::clone::Clone for ISecondaryTile2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISecondaryTile2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2f6cc35_3250_4990_923c_294ab4b694dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTile2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetPhoneticName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PhoneticName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VisualElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetRoamingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub RoamingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub VisualElementsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VisualElementsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVisualElementsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVisualElementsRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryTileFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISecondaryTileFactory {
    type Vtable = ISecondaryTileFactory_Vtbl;
}
impl ::core::clone::Clone for ISecondaryTileFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISecondaryTileFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x57f52ca0_51bc_4abf_8ebf_627a0398b05a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CreateTile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, shortname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, displayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, arguments: ::std::mem::MaybeUninit<::windows_core::HSTRING>, tileoptions: TileOptions, logoreference: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CreateTile: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CreateWideTile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, shortname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, displayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, arguments: ::std::mem::MaybeUninit<::windows_core::HSTRING>, tileoptions: TileOptions, logoreference: *mut ::core::ffi::c_void, widelogoreference: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CreateWideTile: usize,
    pub CreateWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryTileFactory2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISecondaryTileFactory2 {
    type Vtable = ISecondaryTileFactory2_Vtbl;
}
impl ::core::clone::Clone for ISecondaryTileFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISecondaryTileFactory2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x274b8a3b_522d_448e_9eb2_d0672ab345c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileFactory2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateMinimalTile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, displayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, arguments: ::std::mem::MaybeUninit<::windows_core::HSTRING>, square150x150logo: *mut ::core::ffi::c_void, desiredsize: TileSize, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateMinimalTile: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryTileStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISecondaryTileStatics {
    type Vtable = ISecondaryTileStatics_Vtbl;
}
impl ::core::clone::Clone for ISecondaryTileStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISecondaryTileStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x99908dae_d051_4676_87fe_9ec242d83c74);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Exists: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllForApplicationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllForApplicationAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllForPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllForPackageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryTileVisualElements(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISecondaryTileVisualElements {
    type Vtable = ISecondaryTileVisualElements_Vtbl;
}
impl ::core::clone::Clone for ISecondaryTileVisualElements {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISecondaryTileVisualElements {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d8df333_815e_413f_9f50_a81da70a96b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetSquare30x30Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetSquare30x30Logo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Square30x30Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Square30x30Logo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetSquare70x70Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetSquare70x70Logo: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Square70x70Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Square70x70Logo: usize,
    #[cfg(feature = "Foundation")]
    pub SetSquare150x150Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSquare150x150Logo: usize,
    #[cfg(feature = "Foundation")]
    pub Square150x150Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Square150x150Logo: usize,
    #[cfg(feature = "Foundation")]
    pub SetWide310x150Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetWide310x150Logo: usize,
    #[cfg(feature = "Foundation")]
    pub Wide310x150Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Wide310x150Logo: usize,
    #[cfg(feature = "Foundation")]
    pub SetSquare310x310Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSquare310x310Logo: usize,
    #[cfg(feature = "Foundation")]
    pub Square310x310Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Square310x310Logo: usize,
    pub SetForegroundText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ForegroundText) -> ::windows_core::HRESULT,
    pub ForegroundText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ForegroundText) -> ::windows_core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows_core::HRESULT,
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows_core::HRESULT,
    pub SetShowNameOnSquare150x150Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ShowNameOnSquare150x150Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShowNameOnWide310x150Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ShowNameOnWide310x150Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShowNameOnSquare310x310Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ShowNameOnSquare310x310Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryTileVisualElements2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISecondaryTileVisualElements2 {
    type Vtable = ISecondaryTileVisualElements2_Vtbl;
}
impl ::core::clone::Clone for ISecondaryTileVisualElements2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISecondaryTileVisualElements2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd2e31d0_57dc_4794_8ecf_5682f5f3e6ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetSquare71x71Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSquare71x71Logo: usize,
    #[cfg(feature = "Foundation")]
    pub Square71x71Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Square71x71Logo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryTileVisualElements3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISecondaryTileVisualElements3 {
    type Vtable = ISecondaryTileVisualElements3_Vtbl;
}
impl ::core::clone::Clone for ISecondaryTileVisualElements3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISecondaryTileVisualElements3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x56b55ad6_d15c_40f4_81e7_57ffd8f8a4e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetSquare44x44Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSquare44x44Logo: usize,
    #[cfg(feature = "Foundation")]
    pub Square44x44Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Square44x44Logo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISecondaryTileVisualElements4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISecondaryTileVisualElements4 {
    type Vtable = ISecondaryTileVisualElements4_Vtbl;
}
impl ::core::clone::Clone for ISecondaryTileVisualElements4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISecondaryTileVisualElements4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x66566117_b544_40d2_8d12_74d4ec24d04c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MixedRealityModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStartScreenManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStartScreenManager {
    type Vtable = IStartScreenManager_Vtbl;
}
impl ::core::clone::Clone for IStartScreenManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IStartScreenManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a1dcbcb_26e9_4eb4_8933_859eb6ecdb29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartScreenManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    #[cfg(feature = "ApplicationModel_Core")]
    pub SupportsAppListEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applistentry: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))]
    SupportsAppListEntry: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub ContainsAppListEntryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applistentry: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))]
    ContainsAppListEntryAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub RequestAddAppListEntryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applistentry: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))]
    RequestAddAppListEntryAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStartScreenManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStartScreenManager2 {
    type Vtable = IStartScreenManager2_Vtbl;
}
impl ::core::clone::Clone for IStartScreenManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IStartScreenManager2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08a716b6_316b_4ad9_acb8_fe9cf00bd608);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartScreenManager2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ContainsSecondaryTileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContainsSecondaryTileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryRemoveSecondaryTileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryRemoveSecondaryTileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStartScreenManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStartScreenManagerStatics {
    type Vtable = IStartScreenManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IStartScreenManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IStartScreenManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7865ef0f_b585_464e_8993_34e8f8738d48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartScreenManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileMixedRealityModel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileMixedRealityModel {
    type Vtable = ITileMixedRealityModel_Vtbl;
}
impl ::core::clone::Clone for ITileMixedRealityModel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITileMixedRealityModel {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb0764e5b_887d_4242_9a19_3d0a4ea78031);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileMixedRealityModel_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub SetBoundingBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    SetBoundingBox: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub BoundingBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    BoundingBox: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileMixedRealityModel2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileMixedRealityModel2 {
    type Vtable = ITileMixedRealityModel2_Vtbl;
}
impl ::core::clone::Clone for ITileMixedRealityModel2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITileMixedRealityModel2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x439470b2_d7c5_410b_8319_9486a27b6c67);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileMixedRealityModel2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetActivationBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TileMixedRealityModelActivationBehavior) -> ::windows_core::HRESULT,
    pub ActivationBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TileMixedRealityModelActivationBehavior) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualElementsRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualElementsRequest {
    type Vtable = IVisualElementsRequest_Vtbl;
}
impl ::core::clone::Clone for IVisualElementsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVisualElementsRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc138333a_9308_4072_88cc_d068db347c68);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualElementsRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub VisualElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AlternateVisualElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AlternateVisualElements: usize,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualElementsRequestDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualElementsRequestDeferral {
    type Vtable = IVisualElementsRequestDeferral_Vtbl;
}
impl ::core::clone::Clone for IVisualElementsRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVisualElementsRequestDeferral {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1656eb0_0126_4357_8204_bd82bb2a046d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualElementsRequestDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVisualElementsRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVisualElementsRequestedEventArgs {
    type Vtable = IVisualElementsRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IVisualElementsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVisualElementsRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b6fc982_3a0d_4ece_af96_cd17e1b00b2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualElementsRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct JumpList(::windows_core::IUnknown);
impl JumpList {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<JumpListItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SystemGroupKind(&self) -> ::windows_core::Result<JumpListSystemGroupKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemGroupKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSystemGroupKind(&self, value: JumpListSystemGroupKind) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSystemGroupKind)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SaveAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoadCurrentAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<JumpList>> {
        Self::IJumpListStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadCurrentAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::IJumpListStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IJumpListStatics<R, F: FnOnce(&IJumpListStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<JumpList, IJumpListStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for JumpList {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.JumpList;{b0234c3e-cd6f-4cb6-a611-61fd505f3ed1})");
}
impl ::core::clone::Clone for JumpList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for JumpList {
    type Vtable = IJumpList_Vtbl;
}
unsafe impl ::windows_core::ComInterface for JumpList {
    const IID: ::windows_core::GUID = <IJumpList as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for JumpList {
    const NAME: &'static str = "Windows.UI.StartScreen.JumpList";
}
::windows_core::imp::interface_hierarchy!(JumpList, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for JumpList {}
unsafe impl ::core::marker::Sync for JumpList {}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct JumpListItem(::windows_core::IUnknown);
impl JumpListItem {
    pub fn Kind(&self) -> ::windows_core::Result<JumpListItemKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RemovedByUser(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemovedByUser)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn GroupName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GroupName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetGroupName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGroupName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Logo(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Logo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLogo<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLogo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CreateWithArguments(arguments: &::windows_core::HSTRING, displayname: &::windows_core::HSTRING) -> ::windows_core::Result<JumpListItem> {
        Self::IJumpListItemStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithArguments)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(arguments), ::core::mem::transmute_copy(displayname), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateSeparator() -> ::windows_core::Result<JumpListItem> {
        Self::IJumpListItemStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateSeparator)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IJumpListItemStatics<R, F: FnOnce(&IJumpListItemStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<JumpListItem, IJumpListItemStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for JumpListItem {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.JumpListItem;{7adb6717-8b5d-4820-995b-9b418dbe48b0})");
}
impl ::core::clone::Clone for JumpListItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for JumpListItem {
    type Vtable = IJumpListItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for JumpListItem {
    const IID: ::windows_core::GUID = <IJumpListItem as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for JumpListItem {
    const NAME: &'static str = "Windows.UI.StartScreen.JumpListItem";
}
::windows_core::imp::interface_hierarchy!(JumpListItem, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for JumpListItem {}
unsafe impl ::core::marker::Sync for JumpListItem {}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct SecondaryTile(::windows_core::IUnknown);
impl SecondaryTile {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SecondaryTile, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetTileId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTileId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetArguments(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetArguments)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetShortName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShortName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ShortName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShortName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetLogo<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLogo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Logo(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Logo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetSmallLogo<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSmallLogo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SmallLogo(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SmallLogo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetWideLogo<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWideLogo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn WideLogo(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WideLogo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLockScreenBadgeLogo<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLockScreenBadgeLogo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LockScreenBadgeLogo(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LockScreenBadgeLogo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLockScreenDisplayBadgeAndTileText(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLockScreenDisplayBadgeAndTileText)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LockScreenDisplayBadgeAndTileText(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LockScreenDisplayBadgeAndTileText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetTileOptions(&self, value: TileOptions) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTileOptions)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn TileOptions(&self) -> ::windows_core::Result<TileOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TileOptions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetForegroundText(&self, value: ForegroundText) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetForegroundText)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ForegroundText(&self) -> ::windows_core::Result<ForegroundText> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ForegroundText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetBackgroundColor(&self, value: super::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBackgroundColor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn BackgroundColor(&self) -> ::windows_core::Result<super::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundColor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestCreateAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestCreateAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestCreateAsyncWithPoint(&self, invocationpoint: super::super::Foundation::Point) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestCreateAsyncWithPoint)(::windows_core::Interface::as_raw(this), invocationpoint, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestCreateAsyncWithRect(&self, selection: super::super::Foundation::Rect) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestCreateAsyncWithRect)(::windows_core::Interface::as_raw(this), selection, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn RequestCreateAsyncWithRectAndPlacement(&self, selection: super::super::Foundation::Rect, preferredplacement: super::Popups::Placement) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestCreateAsyncWithRectAndPlacement)(::windows_core::Interface::as_raw(this), selection, preferredplacement, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestDeleteAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestDeleteAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestDeleteAsyncWithPoint(&self, invocationpoint: super::super::Foundation::Point) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestDeleteAsyncWithPoint)(::windows_core::Interface::as_raw(this), invocationpoint, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestDeleteAsyncWithRect(&self, selection: super::super::Foundation::Rect) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestDeleteAsyncWithRect)(::windows_core::Interface::as_raw(this), selection, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn RequestDeleteAsyncWithRectAndPlacement(&self, selection: super::super::Foundation::Rect, preferredplacement: super::Popups::Placement) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestDeleteAsyncWithRectAndPlacement)(::windows_core::Interface::as_raw(this), selection, preferredplacement, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPhoneticName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISecondaryTile2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPhoneticName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn PhoneticName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ISecondaryTile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhoneticName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VisualElements(&self) -> ::windows_core::Result<SecondaryTileVisualElements> {
        let this = &::windows_core::ComInterface::cast::<ISecondaryTile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VisualElements)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRoamingEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISecondaryTile2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRoamingEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RoamingEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ISecondaryTile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoamingEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VisualElementsRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SecondaryTile, VisualElementsRequestedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<ISecondaryTile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VisualElementsRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVisualElementsRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISecondaryTile2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVisualElementsRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn CreateTile<P0>(tileid: &::windows_core::HSTRING, shortname: &::windows_core::HSTRING, displayname: &::windows_core::HSTRING, arguments: &::windows_core::HSTRING, tileoptions: TileOptions, logoreference: P0) -> ::windows_core::Result<SecondaryTile>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::ISecondaryTileFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateTile)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(tileid), ::core::mem::transmute_copy(shortname), ::core::mem::transmute_copy(displayname), ::core::mem::transmute_copy(arguments), tileoptions, logoreference.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn CreateWideTile<P0, P1>(tileid: &::windows_core::HSTRING, shortname: &::windows_core::HSTRING, displayname: &::windows_core::HSTRING, arguments: &::windows_core::HSTRING, tileoptions: TileOptions, logoreference: P0, widelogoreference: P1) -> ::windows_core::Result<SecondaryTile>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
        P1: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::ISecondaryTileFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWideTile)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(tileid), ::core::mem::transmute_copy(shortname), ::core::mem::transmute_copy(displayname), ::core::mem::transmute_copy(arguments), tileoptions, logoreference.into_param().abi(), widelogoreference.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithId(tileid: &::windows_core::HSTRING) -> ::windows_core::Result<SecondaryTile> {
        Self::ISecondaryTileFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(tileid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateMinimalTile<P0>(tileid: &::windows_core::HSTRING, displayname: &::windows_core::HSTRING, arguments: &::windows_core::HSTRING, square150x150logo: P0, desiredsize: TileSize) -> ::windows_core::Result<SecondaryTile>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::ISecondaryTileFactory2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMinimalTile)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(tileid), ::core::mem::transmute_copy(displayname), ::core::mem::transmute_copy(arguments), square150x150logo.into_param().abi(), desiredsize, &mut result__).from_abi(result__)
        })
    }
    pub fn Exists(tileid: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        Self::ISecondaryTileStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Exists)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(tileid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>> {
        Self::ISecondaryTileStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllForApplicationAsync(applicationid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>> {
        Self::ISecondaryTileStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllForApplicationAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(applicationid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllForPackageAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>> {
        Self::ISecondaryTileStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllForPackageAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISecondaryTileFactory<R, F: FnOnce(&ISecondaryTileFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SecondaryTile, ISecondaryTileFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISecondaryTileFactory2<R, F: FnOnce(&ISecondaryTileFactory2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SecondaryTile, ISecondaryTileFactory2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISecondaryTileStatics<R, F: FnOnce(&ISecondaryTileStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SecondaryTile, ISecondaryTileStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for SecondaryTile {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.SecondaryTile;{9e9e51e0-2bb5-4bc0-bb8d-42b23abcc88d})");
}
impl ::core::clone::Clone for SecondaryTile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SecondaryTile {
    type Vtable = ISecondaryTile_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SecondaryTile {
    const IID: ::windows_core::GUID = <ISecondaryTile as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SecondaryTile {
    const NAME: &'static str = "Windows.UI.StartScreen.SecondaryTile";
}
::windows_core::imp::interface_hierarchy!(SecondaryTile, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SecondaryTile {}
unsafe impl ::core::marker::Sync for SecondaryTile {}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct SecondaryTileVisualElements(::windows_core::IUnknown);
impl SecondaryTileVisualElements {
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetSquare30x30Logo<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSquare30x30Logo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Square30x30Logo(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Square30x30Logo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetSquare70x70Logo<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSquare70x70Logo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Square70x70Logo(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Square70x70Logo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSquare150x150Logo<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSquare150x150Logo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Square150x150Logo(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Square150x150Logo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetWide310x150Logo<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWide310x150Logo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Wide310x150Logo(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Wide310x150Logo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSquare310x310Logo<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSquare310x310Logo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Square310x310Logo(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Square310x310Logo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetForegroundText(&self, value: ForegroundText) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetForegroundText)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForegroundText(&self) -> ::windows_core::Result<ForegroundText> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ForegroundText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBackgroundColor(&self, value: super::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBackgroundColor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BackgroundColor(&self) -> ::windows_core::Result<super::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundColor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetShowNameOnSquare150x150Logo(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShowNameOnSquare150x150Logo)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShowNameOnSquare150x150Logo(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShowNameOnSquare150x150Logo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetShowNameOnWide310x150Logo(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShowNameOnWide310x150Logo)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShowNameOnWide310x150Logo(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShowNameOnWide310x150Logo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetShowNameOnSquare310x310Logo(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShowNameOnSquare310x310Logo)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShowNameOnSquare310x310Logo(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShowNameOnSquare310x310Logo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSquare71x71Logo<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<ISecondaryTileVisualElements2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSquare71x71Logo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Square71x71Logo(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<ISecondaryTileVisualElements2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Square71x71Logo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSquare44x44Logo<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<ISecondaryTileVisualElements3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSquare44x44Logo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Square44x44Logo(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<ISecondaryTileVisualElements3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Square44x44Logo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MixedRealityModel(&self) -> ::windows_core::Result<TileMixedRealityModel> {
        let this = &::windows_core::ComInterface::cast::<ISecondaryTileVisualElements4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MixedRealityModel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for SecondaryTileVisualElements {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.SecondaryTileVisualElements;{1d8df333-815e-413f-9f50-a81da70a96b2})");
}
impl ::core::clone::Clone for SecondaryTileVisualElements {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SecondaryTileVisualElements {
    type Vtable = ISecondaryTileVisualElements_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SecondaryTileVisualElements {
    const IID: ::windows_core::GUID = <ISecondaryTileVisualElements as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SecondaryTileVisualElements {
    const NAME: &'static str = "Windows.UI.StartScreen.SecondaryTileVisualElements";
}
::windows_core::imp::interface_hierarchy!(SecondaryTileVisualElements, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SecondaryTileVisualElements {}
unsafe impl ::core::marker::Sync for SecondaryTileVisualElements {}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct StartScreenManager(::windows_core::IUnknown);
impl StartScreenManager {
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`*"]
    #[cfg(feature = "ApplicationModel_Core")]
    pub fn SupportsAppListEntry<P0>(&self, applistentry: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<super::super::ApplicationModel::Core::AppListEntry>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportsAppListEntry)(::windows_core::Interface::as_raw(this), applistentry.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub fn ContainsAppListEntryAsync<P0>(&self, applistentry: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<super::super::ApplicationModel::Core::AppListEntry>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContainsAppListEntryAsync)(::windows_core::Interface::as_raw(this), applistentry.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub fn RequestAddAppListEntryAsync<P0>(&self, applistentry: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<super::super::ApplicationModel::Core::AppListEntry>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAddAppListEntryAsync)(::windows_core::Interface::as_raw(this), applistentry.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContainsSecondaryTileAsync(&self, tileid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<IStartScreenManager2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContainsSecondaryTileAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(tileid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryRemoveSecondaryTileAsync(&self, tileid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<IStartScreenManager2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryRemoveSecondaryTileAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(tileid), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<StartScreenManager> {
        Self::IStartScreenManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser<P0>(user: P0) -> ::windows_core::Result<StartScreenManager>
    where
        P0: ::windows_core::IntoParam<super::super::System::User>,
    {
        Self::IStartScreenManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStartScreenManagerStatics<R, F: FnOnce(&IStartScreenManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<StartScreenManager, IStartScreenManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for StartScreenManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.StartScreenManager;{4a1dcbcb-26e9-4eb4-8933-859eb6ecdb29})");
}
impl ::core::clone::Clone for StartScreenManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for StartScreenManager {
    type Vtable = IStartScreenManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for StartScreenManager {
    const IID: ::windows_core::GUID = <IStartScreenManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for StartScreenManager {
    const NAME: &'static str = "Windows.UI.StartScreen.StartScreenManager";
}
::windows_core::imp::interface_hierarchy!(StartScreenManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for StartScreenManager {}
unsafe impl ::core::marker::Sync for StartScreenManager {}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct TileMixedRealityModel(::windows_core::IUnknown);
impl TileMixedRealityModel {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Perception_Spatial\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn SetBoundingBox<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingBox>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBoundingBox)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Perception_Spatial\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn BoundingBox(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingBox>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BoundingBox)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetActivationBehavior(&self, value: TileMixedRealityModelActivationBehavior) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITileMixedRealityModel2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetActivationBehavior)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActivationBehavior(&self) -> ::windows_core::Result<TileMixedRealityModelActivationBehavior> {
        let this = &::windows_core::ComInterface::cast::<ITileMixedRealityModel2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivationBehavior)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for TileMixedRealityModel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.TileMixedRealityModel;{b0764e5b-887d-4242-9a19-3d0a4ea78031})");
}
impl ::core::clone::Clone for TileMixedRealityModel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TileMixedRealityModel {
    type Vtable = ITileMixedRealityModel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TileMixedRealityModel {
    const IID: ::windows_core::GUID = <ITileMixedRealityModel as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TileMixedRealityModel {
    const NAME: &'static str = "Windows.UI.StartScreen.TileMixedRealityModel";
}
::windows_core::imp::interface_hierarchy!(TileMixedRealityModel, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TileMixedRealityModel {}
unsafe impl ::core::marker::Sync for TileMixedRealityModel {}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct VisualElementsRequest(::windows_core::IUnknown);
impl VisualElementsRequest {
    pub fn VisualElements(&self) -> ::windows_core::Result<SecondaryTileVisualElements> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VisualElements)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AlternateVisualElements(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<SecondaryTileVisualElements>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlternateVisualElements)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Deadline)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<VisualElementsRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for VisualElementsRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.VisualElementsRequest;{c138333a-9308-4072-88cc-d068db347c68})");
}
impl ::core::clone::Clone for VisualElementsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VisualElementsRequest {
    type Vtable = IVisualElementsRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VisualElementsRequest {
    const IID: ::windows_core::GUID = <IVisualElementsRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VisualElementsRequest {
    const NAME: &'static str = "Windows.UI.StartScreen.VisualElementsRequest";
}
::windows_core::imp::interface_hierarchy!(VisualElementsRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VisualElementsRequest {}
unsafe impl ::core::marker::Sync for VisualElementsRequest {}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct VisualElementsRequestDeferral(::windows_core::IUnknown);
impl VisualElementsRequestDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
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
impl ::windows_core::RuntimeType for VisualElementsRequestDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.VisualElementsRequestDeferral;{a1656eb0-0126-4357-8204-bd82bb2a046d})");
}
impl ::core::clone::Clone for VisualElementsRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VisualElementsRequestDeferral {
    type Vtable = IVisualElementsRequestDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VisualElementsRequestDeferral {
    const IID: ::windows_core::GUID = <IVisualElementsRequestDeferral as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VisualElementsRequestDeferral {
    const NAME: &'static str = "Windows.UI.StartScreen.VisualElementsRequestDeferral";
}
::windows_core::imp::interface_hierarchy!(VisualElementsRequestDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VisualElementsRequestDeferral {}
unsafe impl ::core::marker::Sync for VisualElementsRequestDeferral {}
#[doc = "*Required features: `\"UI_StartScreen\"`*"]
#[repr(transparent)]
pub struct VisualElementsRequestedEventArgs(::windows_core::IUnknown);
impl VisualElementsRequestedEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<VisualElementsRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for VisualElementsRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.VisualElementsRequestedEventArgs;{7b6fc982-3a0d-4ece-af96-cd17e1b00b2d})");
}
impl ::core::clone::Clone for VisualElementsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VisualElementsRequestedEventArgs {
    type Vtable = IVisualElementsRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VisualElementsRequestedEventArgs {
    const IID: ::windows_core::GUID = <IVisualElementsRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VisualElementsRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.StartScreen.VisualElementsRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(VisualElementsRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::TypeKind for ForegroundText {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ForegroundText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ForegroundText").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ForegroundText {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.ForegroundText;i4)");
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
impl ::windows_core::TypeKind for JumpListItemKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for JumpListItemKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JumpListItemKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for JumpListItemKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.JumpListItemKind;i4)");
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
impl ::windows_core::TypeKind for JumpListSystemGroupKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for JumpListSystemGroupKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JumpListSystemGroupKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for JumpListSystemGroupKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.JumpListSystemGroupKind;i4)");
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
impl ::windows_core::TypeKind for TileMixedRealityModelActivationBehavior {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TileMixedRealityModelActivationBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileMixedRealityModelActivationBehavior").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TileMixedRealityModelActivationBehavior {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.TileMixedRealityModelActivationBehavior;i4)");
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
impl ::windows_core::TypeKind for TileOptions {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TileOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileOptions").field(&self.0).finish()
    }
}
impl TileOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
impl ::windows_core::RuntimeType for TileOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.TileOptions;u4)");
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
impl ::windows_core::TypeKind for TileSize {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TileSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileSize").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TileSize {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.TileSize;i4)");
}
