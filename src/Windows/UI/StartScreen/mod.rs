#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `UI_StartScreen`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ForegroundText(pub i32);
impl ForegroundText {
    pub const Dark: ForegroundText = ForegroundText(0i32);
    pub const Light: ForegroundText = ForegroundText(1i32);
}
impl ::std::convert::From<i32> for ForegroundText {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ForegroundText {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ForegroundText {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.ForegroundText;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IJumpList(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IJumpList {
    type Vtable = IJumpList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2955103294, 52591, 19638, [166, 17, 97, 253, 80, 95, 62, 209]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpList_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut JumpListSystemGroupKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: JumpListSystemGroupKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IJumpListItem(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IJumpListItem {
    type Vtable = IJumpListItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2061199127, 35677, 18464, [153, 91, 155, 65, 141, 190, 72, 176]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut JumpListItemKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IJumpListItemStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IJumpListItemStatics {
    type Vtable = IJumpListItemStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4055876840, 51114, 18891, [141, 222, 236, 252, 205, 122, 215, 228]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListItemStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, arguments: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, displayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IJumpListStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IJumpListStatics {
    type Vtable = IJumpListStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2816525953, 59006, 19316, [130, 80, 63, 50, 44, 77, 146, 195]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISecondaryTile(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISecondaryTile {
    type Vtable = ISecondaryTile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2661175776, 11189, 19392, [187, 141, 66, 178, 58, 188, 200, 141]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTile_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: TileOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut TileOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ForegroundText) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ForegroundText) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Color) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, invocationpoint: super::super::Foundation::Point, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::Popups::Placement, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, invocationpoint: super::super::Foundation::Point, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::Popups::Placement, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISecondaryTile2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISecondaryTile2 {
    type Vtable = ISecondaryTile2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3002518581, 12880, 18832, [146, 60, 41, 74, 180, 182, 148, 221]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTile2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISecondaryTileFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISecondaryTileFactory {
    type Vtable = ISecondaryTileFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1475685536, 20924, 19135, [142, 191, 98, 122, 3, 152, 176, 90]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tileid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, shortname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, displayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, arguments: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, tileoptions: TileOptions, logoreference: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        tileid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        shortname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        displayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        arguments: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        tileoptions: TileOptions,
        logoreference: ::windows::runtime::RawPtr,
        widelogoreference: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tileid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISecondaryTileFactory2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISecondaryTileFactory2 {
    type Vtable = ISecondaryTileFactory2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(659262011, 21037, 17550, [158, 178, 208, 103, 42, 179, 69, 200]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tileid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, displayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, arguments: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, square150x150logo: ::windows::runtime::RawPtr, desiredsize: TileSize, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISecondaryTileStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISecondaryTileStatics {
    type Vtable = ISecondaryTileStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2576387502, 53329, 18038, [135, 254, 158, 194, 66, 216, 60, 116]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tileid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, applicationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISecondaryTileVisualElements {
    type Vtable = ISecondaryTileVisualElements_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(495842099, 33118, 16703, [159, 80, 168, 29, 167, 10, 150, 178]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ForegroundText) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ForegroundText) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISecondaryTileVisualElements2 {
    type Vtable = ISecondaryTileVisualElements2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4247663056, 22492, 18324, [142, 207, 86, 130, 245, 243, 230, 239]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISecondaryTileVisualElements3 {
    type Vtable = ISecondaryTileVisualElements3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1454725846, 53596, 16628, [129, 231, 87, 255, 216, 248, 164, 233]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISecondaryTileVisualElements4 {
    type Vtable = ISecondaryTileVisualElements4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1716936983, 46404, 16594, [141, 18, 116, 212, 236, 36, 208, 76]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryTileVisualElements4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IStartScreenManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStartScreenManager {
    type Vtable = IStartScreenManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1243466699, 9961, 20148, [137, 51, 133, 158, 182, 236, 219, 41]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartScreenManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "ApplicationModel_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, applistentry: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))] usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, applistentry: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))] usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, applistentry: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IStartScreenManager2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStartScreenManager2 {
    type Vtable = IStartScreenManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(145168054, 12651, 19161, [172, 184, 254, 156, 240, 11, 214, 8]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartScreenManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tileid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tileid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IStartScreenManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStartScreenManagerStatics {
    type Vtable = IStartScreenManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2019946255, 46469, 17998, [137, 147, 52, 232, 248, 115, 141, 72]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartScreenManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ITileMixedRealityModel(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITileMixedRealityModel {
    type Vtable = ITileMixedRealityModel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2960543323, 34941, 16962, [154, 25, 61, 10, 78, 167, 128, 49]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileMixedRealityModel_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ITileMixedRealityModel2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITileMixedRealityModel2 {
    type Vtable = ITileMixedRealityModel2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1133801650, 55237, 16651, [131, 25, 148, 134, 162, 123, 108, 103]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileMixedRealityModel2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: TileMixedRealityModelActivationBehavior) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut TileMixedRealityModelActivationBehavior) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IVisualElementsRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVisualElementsRequest {
    type Vtable = IVisualElementsRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3241685818, 37640, 16498, [136, 204, 208, 104, 219, 52, 124, 104]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualElementsRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IVisualElementsRequestDeferral(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVisualElementsRequestDeferral {
    type Vtable = IVisualElementsRequestDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2707779248, 294, 17239, [130, 4, 189, 130, 187, 42, 4, 109]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualElementsRequestDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IVisualElementsRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVisualElementsRequestedEventArgs {
    type Vtable = IVisualElementsRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2070923650, 14861, 20174, [175, 150, 205, 23, 225, 176, 11, 45]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualElementsRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_StartScreen`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct JumpList(::windows::runtime::IInspectable);
impl JumpList {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation_Collections`*"]
    pub fn Items(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<JumpListItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<JumpListItem>>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn SystemGroupKind(&self) -> ::windows::runtime::Result<JumpListSystemGroupKind> {
        let this = self;
        unsafe {
            let mut result__: JumpListSystemGroupKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<JumpListSystemGroupKind>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn SetSystemGroupKind(&self, value: JumpListSystemGroupKind) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn SaveAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn LoadCurrentAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<JumpList>> {
        Self::IJumpListStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<JumpList>>(result__)
        })
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn IsSupported() -> ::windows::runtime::Result<bool> {
        Self::IJumpListStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IJumpListStatics<R, F: FnOnce(&IJumpListStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<JumpList, IJumpListStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for JumpList {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.JumpList;{b0234c3e-cd6f-4cb6-a611-61fd505f3ed1})");
}
unsafe impl ::windows::runtime::Interface for JumpList {
    type Vtable = IJumpList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2955103294, 52591, 19638, [166, 17, 97, 253, 80, 95, 62, 209]);
}
impl ::windows::runtime::RuntimeName for JumpList {
    const NAME: &'static str = "Windows.UI.StartScreen.JumpList";
}
impl ::std::convert::From<JumpList> for ::windows::runtime::IUnknown {
    fn from(value: JumpList) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&JumpList> for ::windows::runtime::IUnknown {
    fn from(value: &JumpList) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for JumpList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &JumpList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<JumpList> for ::windows::runtime::IInspectable {
    fn from(value: JumpList) -> Self {
        value.0
    }
}
impl ::std::convert::From<&JumpList> for ::windows::runtime::IInspectable {
    fn from(value: &JumpList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for JumpList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a JumpList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for JumpList {}
unsafe impl ::std::marker::Sync for JumpList {}
#[doc = "*Required features: `UI_StartScreen`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct JumpListItem(::windows::runtime::IInspectable);
impl JumpListItem {
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<JumpListItemKind> {
        let this = self;
        unsafe {
            let mut result__: JumpListItemKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<JumpListItemKind>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn Arguments(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn RemovedByUser(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn GroupName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn SetGroupName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn Logo(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn SetLogo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn CreateWithArguments<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(arguments: Param0, displayname: Param1) -> ::windows::runtime::Result<JumpListItem> {
        Self::IJumpListItemStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), arguments.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<JumpListItem>(result__)
        })
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn CreateSeparator() -> ::windows::runtime::Result<JumpListItem> {
        Self::IJumpListItemStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<JumpListItem>(result__)
        })
    }
    pub fn IJumpListItemStatics<R, F: FnOnce(&IJumpListItemStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<JumpListItem, IJumpListItemStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for JumpListItem {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.JumpListItem;{7adb6717-8b5d-4820-995b-9b418dbe48b0})");
}
unsafe impl ::windows::runtime::Interface for JumpListItem {
    type Vtable = IJumpListItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2061199127, 35677, 18464, [153, 91, 155, 65, 141, 190, 72, 176]);
}
impl ::windows::runtime::RuntimeName for JumpListItem {
    const NAME: &'static str = "Windows.UI.StartScreen.JumpListItem";
}
impl ::std::convert::From<JumpListItem> for ::windows::runtime::IUnknown {
    fn from(value: JumpListItem) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&JumpListItem> for ::windows::runtime::IUnknown {
    fn from(value: &JumpListItem) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for JumpListItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &JumpListItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<JumpListItem> for ::windows::runtime::IInspectable {
    fn from(value: JumpListItem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&JumpListItem> for ::windows::runtime::IInspectable {
    fn from(value: &JumpListItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for JumpListItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a JumpListItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for JumpListItem {}
unsafe impl ::std::marker::Sync for JumpListItem {}
#[doc = "*Required features: `UI_StartScreen`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct JumpListItemKind(pub i32);
impl JumpListItemKind {
    pub const Arguments: JumpListItemKind = JumpListItemKind(0i32);
    pub const Separator: JumpListItemKind = JumpListItemKind(1i32);
}
impl ::std::convert::From<i32> for JumpListItemKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for JumpListItemKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for JumpListItemKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.JumpListItemKind;i4)");
}
#[doc = "*Required features: `UI_StartScreen`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct JumpListSystemGroupKind(pub i32);
impl JumpListSystemGroupKind {
    pub const None: JumpListSystemGroupKind = JumpListSystemGroupKind(0i32);
    pub const Frequent: JumpListSystemGroupKind = JumpListSystemGroupKind(1i32);
    pub const Recent: JumpListSystemGroupKind = JumpListSystemGroupKind(2i32);
}
impl ::std::convert::From<i32> for JumpListSystemGroupKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for JumpListSystemGroupKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for JumpListSystemGroupKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.JumpListSystemGroupKind;i4)");
}
#[doc = "*Required features: `UI_StartScreen`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SecondaryTile(::windows::runtime::IInspectable);
impl SecondaryTile {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SecondaryTile, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn SetTileId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn TileId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn SetArguments<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn Arguments(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn SetShortName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn ShortName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn SetLogo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn Logo(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn SetSmallLogo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn SmallLogo(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn SetWideLogo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn WideLogo(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn SetLockScreenBadgeLogo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn LockScreenBadgeLogo(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn SetLockScreenDisplayBadgeAndTileText(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn LockScreenDisplayBadgeAndTileText(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn SetTileOptions(&self, value: TileOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn TileOptions(&self) -> ::windows::runtime::Result<TileOptions> {
        let this = self;
        unsafe {
            let mut result__: TileOptions = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TileOptions>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn SetForegroundText(&self, value: ForegroundText) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn ForegroundText(&self) -> ::windows::runtime::Result<ForegroundText> {
        let this = self;
        unsafe {
            let mut result__: ForegroundText = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ForegroundText>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn SetBackgroundColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn BackgroundColor(&self) -> ::windows::runtime::Result<super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Color>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn RequestCreateAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn RequestCreateAsyncWithPoint<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Point>>(&self, invocationpoint: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), invocationpoint.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn RequestCreateAsyncWithRect<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`, `UI_Popups`*"]
    pub fn RequestCreateAsyncWithRectAndPlacement<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0, preferredplacement: super::Popups::Placement) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), selection.into_param().abi(), preferredplacement, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn RequestDeleteAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn RequestDeleteAsyncWithPoint<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Point>>(&self, invocationpoint: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), invocationpoint.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn RequestDeleteAsyncWithRect<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`, `UI_Popups`*"]
    pub fn RequestDeleteAsyncWithRectAndPlacement<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0, preferredplacement: super::Popups::Placement) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), selection.into_param().abi(), preferredplacement, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn UpdateAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn SetPhoneticName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISecondaryTile2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn PhoneticName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISecondaryTile2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn VisualElements(&self) -> ::windows::runtime::Result<SecondaryTileVisualElements> {
        let this = &::windows::runtime::Interface::cast::<ISecondaryTile2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryTileVisualElements>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn SetRoamingEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISecondaryTile2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn RoamingEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISecondaryTile2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn VisualElementsRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SecondaryTile, VisualElementsRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ISecondaryTile2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn RemoveVisualElementsRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISecondaryTile2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn CreateTile<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(
        tileid: Param0,
        shortname: Param1,
        displayname: Param2,
        arguments: Param3,
        tileoptions: TileOptions,
        logoreference: Param5,
    ) -> ::windows::runtime::Result<SecondaryTile> {
        Self::ISecondaryTileFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), tileid.into_param().abi(), shortname.into_param().abi(), displayname.into_param().abi(), arguments.into_param().abi(), tileoptions, logoreference.into_param().abi(), &mut result__).from_abi::<SecondaryTile>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn CreateWideTile<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(
        tileid: Param0,
        shortname: Param1,
        displayname: Param2,
        arguments: Param3,
        tileoptions: TileOptions,
        logoreference: Param5,
        widelogoreference: Param6,
    ) -> ::windows::runtime::Result<SecondaryTile> {
        Self::ISecondaryTileFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), tileid.into_param().abi(), shortname.into_param().abi(), displayname.into_param().abi(), arguments.into_param().abi(), tileoptions, logoreference.into_param().abi(), widelogoreference.into_param().abi(), &mut result__).from_abi::<SecondaryTile>(result__)
        })
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn CreateWithId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(tileid: Param0) -> ::windows::runtime::Result<SecondaryTile> {
        Self::ISecondaryTileFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), tileid.into_param().abi(), &mut result__).from_abi::<SecondaryTile>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn CreateMinimalTile<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(tileid: Param0, displayname: Param1, arguments: Param2, square150x150logo: Param3, desiredsize: TileSize) -> ::windows::runtime::Result<SecondaryTile> {
        Self::ISecondaryTileFactory2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), tileid.into_param().abi(), displayname.into_param().abi(), arguments.into_param().abi(), square150x150logo.into_param().abi(), desiredsize, &mut result__).from_abi::<SecondaryTile>(result__)
        })
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn Exists<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(tileid: Param0) -> ::windows::runtime::Result<bool> {
        Self::ISecondaryTileStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), tileid.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>> {
        Self::ISecondaryTileStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllForApplicationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(applicationid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>> {
        Self::ISecondaryTileStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), applicationid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAllForPackageAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>> {
        Self::ISecondaryTileStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SecondaryTile>>>(result__)
        })
    }
    pub fn ISecondaryTileFactory<R, F: FnOnce(&ISecondaryTileFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SecondaryTile, ISecondaryTileFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISecondaryTileFactory2<R, F: FnOnce(&ISecondaryTileFactory2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SecondaryTile, ISecondaryTileFactory2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISecondaryTileStatics<R, F: FnOnce(&ISecondaryTileStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SecondaryTile, ISecondaryTileStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SecondaryTile {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.SecondaryTile;{9e9e51e0-2bb5-4bc0-bb8d-42b23abcc88d})");
}
unsafe impl ::windows::runtime::Interface for SecondaryTile {
    type Vtable = ISecondaryTile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2661175776, 11189, 19392, [187, 141, 66, 178, 58, 188, 200, 141]);
}
impl ::windows::runtime::RuntimeName for SecondaryTile {
    const NAME: &'static str = "Windows.UI.StartScreen.SecondaryTile";
}
impl ::std::convert::From<SecondaryTile> for ::windows::runtime::IUnknown {
    fn from(value: SecondaryTile) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SecondaryTile> for ::windows::runtime::IUnknown {
    fn from(value: &SecondaryTile) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SecondaryTile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SecondaryTile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SecondaryTile> for ::windows::runtime::IInspectable {
    fn from(value: SecondaryTile) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SecondaryTile> for ::windows::runtime::IInspectable {
    fn from(value: &SecondaryTile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SecondaryTile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SecondaryTile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SecondaryTile {}
unsafe impl ::std::marker::Sync for SecondaryTile {}
#[doc = "*Required features: `UI_StartScreen`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SecondaryTileVisualElements(::windows::runtime::IInspectable);
impl SecondaryTileVisualElements {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn SetSquare30x30Logo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn Square30x30Logo(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn SetSquare70x70Logo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn Square70x70Logo(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn SetSquare150x150Logo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn Square150x150Logo(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn SetWide310x150Logo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn Wide310x150Logo(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn SetSquare310x310Logo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn Square310x310Logo(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn SetForegroundText(&self, value: ForegroundText) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn ForegroundText(&self) -> ::windows::runtime::Result<ForegroundText> {
        let this = self;
        unsafe {
            let mut result__: ForegroundText = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ForegroundText>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn SetBackgroundColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn BackgroundColor(&self) -> ::windows::runtime::Result<super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Color>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn SetShowNameOnSquare150x150Logo(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn ShowNameOnSquare150x150Logo(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn SetShowNameOnWide310x150Logo(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn ShowNameOnWide310x150Logo(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn SetShowNameOnSquare310x310Logo(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn ShowNameOnSquare310x310Logo(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn SetSquare71x71Logo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISecondaryTileVisualElements2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn Square71x71Logo(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<ISecondaryTileVisualElements2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn SetSquare44x44Logo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISecondaryTileVisualElements3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn Square44x44Logo(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<ISecondaryTileVisualElements3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn MixedRealityModel(&self) -> ::windows::runtime::Result<TileMixedRealityModel> {
        let this = &::windows::runtime::Interface::cast::<ISecondaryTileVisualElements4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TileMixedRealityModel>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SecondaryTileVisualElements {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.SecondaryTileVisualElements;{1d8df333-815e-413f-9f50-a81da70a96b2})");
}
unsafe impl ::windows::runtime::Interface for SecondaryTileVisualElements {
    type Vtable = ISecondaryTileVisualElements_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(495842099, 33118, 16703, [159, 80, 168, 29, 167, 10, 150, 178]);
}
impl ::windows::runtime::RuntimeName for SecondaryTileVisualElements {
    const NAME: &'static str = "Windows.UI.StartScreen.SecondaryTileVisualElements";
}
impl ::std::convert::From<SecondaryTileVisualElements> for ::windows::runtime::IUnknown {
    fn from(value: SecondaryTileVisualElements) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SecondaryTileVisualElements> for ::windows::runtime::IUnknown {
    fn from(value: &SecondaryTileVisualElements) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SecondaryTileVisualElements {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SecondaryTileVisualElements {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SecondaryTileVisualElements> for ::windows::runtime::IInspectable {
    fn from(value: SecondaryTileVisualElements) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SecondaryTileVisualElements> for ::windows::runtime::IInspectable {
    fn from(value: &SecondaryTileVisualElements) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SecondaryTileVisualElements {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SecondaryTileVisualElements {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SecondaryTileVisualElements {}
unsafe impl ::std::marker::Sync for SecondaryTileVisualElements {}
#[doc = "*Required features: `UI_StartScreen`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct StartScreenManager(::windows::runtime::IInspectable);
impl StartScreenManager {
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_StartScreen`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Core")]
    #[doc = "*Required features: `UI_StartScreen`, `ApplicationModel_Core`*"]
    pub fn SupportsAppListEntry<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::Core::AppListEntry>>(&self, applistentry: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), applistentry.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    #[doc = "*Required features: `UI_StartScreen`, `ApplicationModel_Core`, `Foundation`*"]
    pub fn ContainsAppListEntryAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::Core::AppListEntry>>(&self, applistentry: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), applistentry.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    #[doc = "*Required features: `UI_StartScreen`, `ApplicationModel_Core`, `Foundation`*"]
    pub fn RequestAddAppListEntryAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::Core::AppListEntry>>(&self, applistentry: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), applistentry.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<StartScreenManager> {
        Self::IStartScreenManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<StartScreenManager>(result__)
        })
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `UI_StartScreen`, `System`*"]
    pub fn GetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::runtime::Result<StartScreenManager> {
        Self::IStartScreenManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<StartScreenManager>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn ContainsSecondaryTileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, tileid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IStartScreenManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), tileid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn TryRemoveSecondaryTileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, tileid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IStartScreenManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), tileid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn IStartScreenManagerStatics<R, F: FnOnce(&IStartScreenManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<StartScreenManager, IStartScreenManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for StartScreenManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.StartScreenManager;{4a1dcbcb-26e9-4eb4-8933-859eb6ecdb29})");
}
unsafe impl ::windows::runtime::Interface for StartScreenManager {
    type Vtable = IStartScreenManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1243466699, 9961, 20148, [137, 51, 133, 158, 182, 236, 219, 41]);
}
impl ::windows::runtime::RuntimeName for StartScreenManager {
    const NAME: &'static str = "Windows.UI.StartScreen.StartScreenManager";
}
impl ::std::convert::From<StartScreenManager> for ::windows::runtime::IUnknown {
    fn from(value: StartScreenManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&StartScreenManager> for ::windows::runtime::IUnknown {
    fn from(value: &StartScreenManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for StartScreenManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &StartScreenManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<StartScreenManager> for ::windows::runtime::IInspectable {
    fn from(value: StartScreenManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&StartScreenManager> for ::windows::runtime::IInspectable {
    fn from(value: &StartScreenManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for StartScreenManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a StartScreenManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for StartScreenManager {}
unsafe impl ::std::marker::Sync for StartScreenManager {}
#[doc = "*Required features: `UI_StartScreen`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TileMixedRealityModel(::windows::runtime::IInspectable);
impl TileMixedRealityModel {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn SetUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`, `Foundation_Numerics`, `Perception_Spatial`*"]
    pub fn SetBoundingBox<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingBox>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`, `Foundation_Numerics`, `Perception_Spatial`*"]
    pub fn BoundingBox(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingBox>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingBox>>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn SetActivationBehavior(&self, value: TileMixedRealityModelActivationBehavior) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITileMixedRealityModel2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn ActivationBehavior(&self) -> ::windows::runtime::Result<TileMixedRealityModelActivationBehavior> {
        let this = &::windows::runtime::Interface::cast::<ITileMixedRealityModel2>(self)?;
        unsafe {
            let mut result__: TileMixedRealityModelActivationBehavior = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TileMixedRealityModelActivationBehavior>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TileMixedRealityModel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.TileMixedRealityModel;{b0764e5b-887d-4242-9a19-3d0a4ea78031})");
}
unsafe impl ::windows::runtime::Interface for TileMixedRealityModel {
    type Vtable = ITileMixedRealityModel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2960543323, 34941, 16962, [154, 25, 61, 10, 78, 167, 128, 49]);
}
impl ::windows::runtime::RuntimeName for TileMixedRealityModel {
    const NAME: &'static str = "Windows.UI.StartScreen.TileMixedRealityModel";
}
impl ::std::convert::From<TileMixedRealityModel> for ::windows::runtime::IUnknown {
    fn from(value: TileMixedRealityModel) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TileMixedRealityModel> for ::windows::runtime::IUnknown {
    fn from(value: &TileMixedRealityModel) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TileMixedRealityModel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &TileMixedRealityModel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<TileMixedRealityModel> for ::windows::runtime::IInspectable {
    fn from(value: TileMixedRealityModel) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TileMixedRealityModel> for ::windows::runtime::IInspectable {
    fn from(value: &TileMixedRealityModel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TileMixedRealityModel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TileMixedRealityModel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for TileMixedRealityModel {}
unsafe impl ::std::marker::Sync for TileMixedRealityModel {}
#[doc = "*Required features: `UI_StartScreen`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TileMixedRealityModelActivationBehavior(pub i32);
impl TileMixedRealityModelActivationBehavior {
    pub const Default: TileMixedRealityModelActivationBehavior = TileMixedRealityModelActivationBehavior(0i32);
    pub const None: TileMixedRealityModelActivationBehavior = TileMixedRealityModelActivationBehavior(1i32);
}
impl ::std::convert::From<i32> for TileMixedRealityModelActivationBehavior {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TileMixedRealityModelActivationBehavior {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TileMixedRealityModelActivationBehavior {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.TileMixedRealityModelActivationBehavior;i4)");
}
#[doc = "*Required features: `UI_StartScreen`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TileOptions(pub u32);
impl TileOptions {
    pub const None: TileOptions = TileOptions(0u32);
    pub const ShowNameOnLogo: TileOptions = TileOptions(1u32);
    pub const ShowNameOnWideLogo: TileOptions = TileOptions(2u32);
    pub const CopyOnDeployment: TileOptions = TileOptions(4u32);
}
impl ::std::convert::From<u32> for TileOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TileOptions {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TileOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.TileOptions;u4)");
}
impl ::std::ops::BitOr for TileOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TileOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TileOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TileOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TileOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `UI_StartScreen`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for TileSize {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TileSize {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TileSize {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.StartScreen.TileSize;i4)");
}
#[doc = "*Required features: `UI_StartScreen`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct VisualElementsRequest(::windows::runtime::IInspectable);
impl VisualElementsRequest {
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn VisualElements(&self) -> ::windows::runtime::Result<SecondaryTileVisualElements> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SecondaryTileVisualElements>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation_Collections`*"]
    pub fn AlternateVisualElements(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<SecondaryTileVisualElements>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SecondaryTileVisualElements>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_StartScreen`, `Foundation`*"]
    pub fn Deadline(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<VisualElementsRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<VisualElementsRequestDeferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VisualElementsRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.VisualElementsRequest;{c138333a-9308-4072-88cc-d068db347c68})");
}
unsafe impl ::windows::runtime::Interface for VisualElementsRequest {
    type Vtable = IVisualElementsRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3241685818, 37640, 16498, [136, 204, 208, 104, 219, 52, 124, 104]);
}
impl ::windows::runtime::RuntimeName for VisualElementsRequest {
    const NAME: &'static str = "Windows.UI.StartScreen.VisualElementsRequest";
}
impl ::std::convert::From<VisualElementsRequest> for ::windows::runtime::IUnknown {
    fn from(value: VisualElementsRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VisualElementsRequest> for ::windows::runtime::IUnknown {
    fn from(value: &VisualElementsRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VisualElementsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &VisualElementsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<VisualElementsRequest> for ::windows::runtime::IInspectable {
    fn from(value: VisualElementsRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VisualElementsRequest> for ::windows::runtime::IInspectable {
    fn from(value: &VisualElementsRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for VisualElementsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a VisualElementsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for VisualElementsRequest {}
unsafe impl ::std::marker::Sync for VisualElementsRequest {}
#[doc = "*Required features: `UI_StartScreen`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct VisualElementsRequestDeferral(::windows::runtime::IInspectable);
impl VisualElementsRequestDeferral {
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VisualElementsRequestDeferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.VisualElementsRequestDeferral;{a1656eb0-0126-4357-8204-bd82bb2a046d})");
}
unsafe impl ::windows::runtime::Interface for VisualElementsRequestDeferral {
    type Vtable = IVisualElementsRequestDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2707779248, 294, 17239, [130, 4, 189, 130, 187, 42, 4, 109]);
}
impl ::windows::runtime::RuntimeName for VisualElementsRequestDeferral {
    const NAME: &'static str = "Windows.UI.StartScreen.VisualElementsRequestDeferral";
}
impl ::std::convert::From<VisualElementsRequestDeferral> for ::windows::runtime::IUnknown {
    fn from(value: VisualElementsRequestDeferral) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VisualElementsRequestDeferral> for ::windows::runtime::IUnknown {
    fn from(value: &VisualElementsRequestDeferral) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VisualElementsRequestDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &VisualElementsRequestDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<VisualElementsRequestDeferral> for ::windows::runtime::IInspectable {
    fn from(value: VisualElementsRequestDeferral) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VisualElementsRequestDeferral> for ::windows::runtime::IInspectable {
    fn from(value: &VisualElementsRequestDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for VisualElementsRequestDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a VisualElementsRequestDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for VisualElementsRequestDeferral {}
unsafe impl ::std::marker::Sync for VisualElementsRequestDeferral {}
#[doc = "*Required features: `UI_StartScreen`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct VisualElementsRequestedEventArgs(::windows::runtime::IInspectable);
impl VisualElementsRequestedEventArgs {
    #[doc = "*Required features: `UI_StartScreen`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<VisualElementsRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<VisualElementsRequest>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VisualElementsRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.StartScreen.VisualElementsRequestedEventArgs;{7b6fc982-3a0d-4ece-af96-cd17e1b00b2d})");
}
unsafe impl ::windows::runtime::Interface for VisualElementsRequestedEventArgs {
    type Vtable = IVisualElementsRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2070923650, 14861, 20174, [175, 150, 205, 23, 225, 176, 11, 45]);
}
impl ::windows::runtime::RuntimeName for VisualElementsRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.StartScreen.VisualElementsRequestedEventArgs";
}
impl ::std::convert::From<VisualElementsRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: VisualElementsRequestedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VisualElementsRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &VisualElementsRequestedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VisualElementsRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &VisualElementsRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<VisualElementsRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: VisualElementsRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VisualElementsRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &VisualElementsRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for VisualElementsRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a VisualElementsRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for VisualElementsRequestedEventArgs {}
unsafe impl ::std::marker::Sync for VisualElementsRequestedEventArgs {}
