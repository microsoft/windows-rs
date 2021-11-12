#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhotoImportDeleteImportedItemsFromSourceResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhotoImportDeleteImportedItemsFromSourceResult {
    type Vtable = IPhotoImportDeleteImportedItemsFromSourceResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4e112f8_843d_428a_a1a6_81510292b0ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportDeleteImportedItemsFromSourceResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhotoImportFindItemsResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhotoImportFindItemsResult {
    type Vtable = IPhotoImportFindItemsResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3915e647_6c78_492b_844e_8fe5e8f6bfb9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportFindItemsResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PhotoImportImportMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhotoImportImportMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhotoImportFindItemsResult2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhotoImportFindItemsResult2 {
    type Vtable = IPhotoImportFindItemsResult2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbdd6a3b_ecf9_406a_815e_5015625b0a88);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportFindItemsResult2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhotoImportImportItemsResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhotoImportImportItemsResult {
    type Vtable = IPhotoImportImportItemsResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4d4f478_d419_4443_a84e_f06a850c0b00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportImportItemsResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhotoImportItem(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhotoImportItem {
    type Vtable = IPhotoImportItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9d07e76_9bfc_43b8_b356_633b6a988c9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportItem_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhotoImportContentType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhotoImportItem2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhotoImportItem2 {
    type Vtable = IPhotoImportItem2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1053505_f53b_46a3_9e30_3610791a9110);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportItem2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhotoImportItemImportedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhotoImportItemImportedEventArgs {
    type Vtable = IPhotoImportItemImportedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42cb2fdd_7d68_47b5_bc7c_ceb73e0c77dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportItemImportedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhotoImportManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhotoImportManagerStatics {
    type Vtable = IPhotoImportManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2771903d_a046_4f06_9b9c_bfd662e83287);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhotoImportOperation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhotoImportOperation {
    type Vtable = IPhotoImportOperation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9f797e4_a09a_4ee4_a4b1_20940277a5be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhotoImportStage) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhotoImportSelectionChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhotoImportSelectionChangedEventArgs {
    type Vtable = IPhotoImportSelectionChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10461782_fa9d_4c30_8bc9_4d64911572d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportSelectionChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhotoImportSession(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhotoImportSession {
    type Vtable = IPhotoImportSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa63916e_ecdb_4efe_94c6_5f5cafe34cfb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportSession_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PhotoImportSubfolderCreationMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhotoImportSubfolderCreationMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, contenttypefilter: PhotoImportContentTypeFilter, itemselectionmode: PhotoImportItemSelectionMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhotoImportSession2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhotoImportSession2 {
    type Vtable = IPhotoImportSession2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a526710_3ec6_469d_a375_2b9f4785391e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportSession2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PhotoImportSubfolderDateFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhotoImportSubfolderDateFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhotoImportSidecar(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhotoImportSidecar {
    type Vtable = IPhotoImportSidecar_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46d7d757_f802_44c7_9c98_7a71f4bc1486);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportSidecar_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhotoImportSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhotoImportSource {
    type Vtable = IPhotoImportSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f8ea35e_145b_4cd6_87f1_54965a982fef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportSource_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhotoImportConnectionTransport) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhotoImportSourceType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhotoImportPowerSource) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhotoImportSourceStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhotoImportSourceStatics {
    type Vtable = IPhotoImportSourceStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0528e586_32d8_467c_8cee_23a1b2f43e85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportSourceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sourceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sourcerootfolder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhotoImportStorageMedium(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhotoImportStorageMedium {
    type Vtable = IPhotoImportStorageMedium_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2b9b093_fc85_487f_87c2_58d675d05b07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportStorageMedium_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhotoImportStorageMediumType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PhotoImportAccessMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPhotoImportVideoSegment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhotoImportVideoSegment {
    type Vtable = IPhotoImportVideoSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x623c0289_321a_41d8_9166_8c62a333276c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportVideoSegment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhotoImportAccessMode(pub i32);
impl PhotoImportAccessMode {
    pub const ReadWrite: PhotoImportAccessMode = PhotoImportAccessMode(0i32);
    pub const ReadOnly: PhotoImportAccessMode = PhotoImportAccessMode(1i32);
    pub const ReadAndDelete: PhotoImportAccessMode = PhotoImportAccessMode(2i32);
}
impl ::core::convert::From<i32> for PhotoImportAccessMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportAccessMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhotoImportAccessMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportAccessMode;i4)");
}
impl ::windows::core::DefaultType for PhotoImportAccessMode {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhotoImportConnectionTransport(pub i32);
impl PhotoImportConnectionTransport {
    pub const Unknown: PhotoImportConnectionTransport = PhotoImportConnectionTransport(0i32);
    pub const Usb: PhotoImportConnectionTransport = PhotoImportConnectionTransport(1i32);
    pub const IP: PhotoImportConnectionTransport = PhotoImportConnectionTransport(2i32);
    pub const Bluetooth: PhotoImportConnectionTransport = PhotoImportConnectionTransport(3i32);
}
impl ::core::convert::From<i32> for PhotoImportConnectionTransport {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportConnectionTransport {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhotoImportConnectionTransport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportConnectionTransport;i4)");
}
impl ::windows::core::DefaultType for PhotoImportConnectionTransport {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhotoImportContentType(pub i32);
impl PhotoImportContentType {
    pub const Unknown: PhotoImportContentType = PhotoImportContentType(0i32);
    pub const Image: PhotoImportContentType = PhotoImportContentType(1i32);
    pub const Video: PhotoImportContentType = PhotoImportContentType(2i32);
}
impl ::core::convert::From<i32> for PhotoImportContentType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportContentType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhotoImportContentType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportContentType;i4)");
}
impl ::windows::core::DefaultType for PhotoImportContentType {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhotoImportContentTypeFilter(pub i32);
impl PhotoImportContentTypeFilter {
    pub const OnlyImages: PhotoImportContentTypeFilter = PhotoImportContentTypeFilter(0i32);
    pub const OnlyVideos: PhotoImportContentTypeFilter = PhotoImportContentTypeFilter(1i32);
    pub const ImagesAndVideos: PhotoImportContentTypeFilter = PhotoImportContentTypeFilter(2i32);
    pub const ImagesAndVideosFromCameraRoll: PhotoImportContentTypeFilter = PhotoImportContentTypeFilter(3i32);
}
impl ::core::convert::From<i32> for PhotoImportContentTypeFilter {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportContentTypeFilter {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhotoImportContentTypeFilter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportContentTypeFilter;i4)");
}
impl ::windows::core::DefaultType for PhotoImportContentTypeFilter {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhotoImportDeleteImportedItemsFromSourceResult(pub ::windows::core::IInspectable);
impl PhotoImportDeleteImportedItemsFromSourceResult {
    pub fn Session(&self) -> ::windows::core::Result<PhotoImportSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoImportSession>(result__)
        }
    }
    pub fn HasSucceeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeletedItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PhotoImportItem>>(result__)
        }
    }
    pub fn PhotosCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn PhotosSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn VideosCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn VideosSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn SidecarsCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SidecarsSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn SiblingsCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SiblingsSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn TotalCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn TotalSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportDeleteImportedItemsFromSourceResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportDeleteImportedItemsFromSourceResult;{f4e112f8-843d-428a-a1a6-81510292b0ae})");
}
unsafe impl ::windows::core::Interface for PhotoImportDeleteImportedItemsFromSourceResult {
    type Vtable = IPhotoImportDeleteImportedItemsFromSourceResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4e112f8_843d_428a_a1a6_81510292b0ae);
}
impl ::windows::core::RuntimeName for PhotoImportDeleteImportedItemsFromSourceResult {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportDeleteImportedItemsFromSourceResult";
}
impl ::core::convert::From<PhotoImportDeleteImportedItemsFromSourceResult> for ::windows::core::IUnknown {
    fn from(value: PhotoImportDeleteImportedItemsFromSourceResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhotoImportDeleteImportedItemsFromSourceResult> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportDeleteImportedItemsFromSourceResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhotoImportDeleteImportedItemsFromSourceResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhotoImportDeleteImportedItemsFromSourceResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhotoImportDeleteImportedItemsFromSourceResult> for ::windows::core::IInspectable {
    fn from(value: PhotoImportDeleteImportedItemsFromSourceResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhotoImportDeleteImportedItemsFromSourceResult> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportDeleteImportedItemsFromSourceResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhotoImportDeleteImportedItemsFromSourceResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhotoImportDeleteImportedItemsFromSourceResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhotoImportDeleteImportedItemsFromSourceResult {}
unsafe impl ::core::marker::Sync for PhotoImportDeleteImportedItemsFromSourceResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhotoImportFindItemsResult(pub ::windows::core::IInspectable);
impl PhotoImportFindItemsResult {
    pub fn Session(&self) -> ::windows::core::Result<PhotoImportSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoImportSession>(result__)
        }
    }
    pub fn HasSucceeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FoundItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PhotoImportItem>>(result__)
        }
    }
    pub fn PhotosCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn PhotosSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn VideosCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn VideosSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn SidecarsCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SidecarsSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn SiblingsCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SiblingsSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn TotalCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn TotalSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn SelectAll(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn SelectNone(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SelectNewAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn SetImportMode(&self, value: PhotoImportImportMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ImportMode(&self) -> ::windows::core::Result<PhotoImportImportMode> {
        let this = self;
        unsafe {
            let mut result__: PhotoImportImportMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoImportImportMode>(result__)
        }
    }
    pub fn SelectedPhotosCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SelectedPhotosSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn SelectedVideosCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SelectedVideosSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn SelectedSidecarsCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SelectedSidecarsSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn SelectedSiblingsCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SelectedSiblingsSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn SelectedTotalCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SelectedTotalSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SelectionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhotoImportFindItemsResult, PhotoImportSelectionChangedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSelectionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ImportItemsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportImportItemsResult, PhotoImportProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportImportItemsResult, PhotoImportProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ItemImported<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PhotoImportFindItemsResult, PhotoImportItemImportedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemImported<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn AddItemsInDateRangeToSelection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, rangestart: Param0, rangelength: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPhotoImportFindItemsResult2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), rangestart.into_param().abi(), rangelength.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportFindItemsResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportFindItemsResult;{3915e647-6c78-492b-844e-8fe5e8f6bfb9})");
}
unsafe impl ::windows::core::Interface for PhotoImportFindItemsResult {
    type Vtable = IPhotoImportFindItemsResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3915e647_6c78_492b_844e_8fe5e8f6bfb9);
}
impl ::windows::core::RuntimeName for PhotoImportFindItemsResult {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportFindItemsResult";
}
impl ::core::convert::From<PhotoImportFindItemsResult> for ::windows::core::IUnknown {
    fn from(value: PhotoImportFindItemsResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhotoImportFindItemsResult> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportFindItemsResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhotoImportFindItemsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhotoImportFindItemsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhotoImportFindItemsResult> for ::windows::core::IInspectable {
    fn from(value: PhotoImportFindItemsResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhotoImportFindItemsResult> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportFindItemsResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhotoImportFindItemsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhotoImportFindItemsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhotoImportFindItemsResult {}
unsafe impl ::core::marker::Sync for PhotoImportFindItemsResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhotoImportImportItemsResult(pub ::windows::core::IInspectable);
impl PhotoImportImportItemsResult {
    pub fn Session(&self) -> ::windows::core::Result<PhotoImportSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoImportSession>(result__)
        }
    }
    pub fn HasSucceeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ImportedItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PhotoImportItem>>(result__)
        }
    }
    pub fn PhotosCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn PhotosSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn VideosCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn VideosSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn SidecarsCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SidecarsSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn SiblingsCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SiblingsSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn TotalCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn TotalSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeleteImportedItemsFromSourceAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportDeleteImportedItemsFromSourceResult, f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportDeleteImportedItemsFromSourceResult, f64>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportImportItemsResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportImportItemsResult;{e4d4f478-d419-4443-a84e-f06a850c0b00})");
}
unsafe impl ::windows::core::Interface for PhotoImportImportItemsResult {
    type Vtable = IPhotoImportImportItemsResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4d4f478_d419_4443_a84e_f06a850c0b00);
}
impl ::windows::core::RuntimeName for PhotoImportImportItemsResult {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportImportItemsResult";
}
impl ::core::convert::From<PhotoImportImportItemsResult> for ::windows::core::IUnknown {
    fn from(value: PhotoImportImportItemsResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhotoImportImportItemsResult> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportImportItemsResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhotoImportImportItemsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhotoImportImportItemsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhotoImportImportItemsResult> for ::windows::core::IInspectable {
    fn from(value: PhotoImportImportItemsResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhotoImportImportItemsResult> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportImportItemsResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhotoImportImportItemsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhotoImportImportItemsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhotoImportImportItemsResult {}
unsafe impl ::core::marker::Sync for PhotoImportImportItemsResult {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhotoImportImportMode(pub i32);
impl PhotoImportImportMode {
    pub const ImportEverything: PhotoImportImportMode = PhotoImportImportMode(0i32);
    pub const IgnoreSidecars: PhotoImportImportMode = PhotoImportImportMode(1i32);
    pub const IgnoreSiblings: PhotoImportImportMode = PhotoImportImportMode(2i32);
    pub const IgnoreSidecarsAndSiblings: PhotoImportImportMode = PhotoImportImportMode(3i32);
}
impl ::core::convert::From<i32> for PhotoImportImportMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportImportMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhotoImportImportMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportImportMode;i4)");
}
impl ::windows::core::DefaultType for PhotoImportImportMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhotoImportItem(pub ::windows::core::IInspectable);
impl PhotoImportItem {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ItemKey(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn ContentType(&self) -> ::windows::core::Result<PhotoImportContentType> {
        let this = self;
        unsafe {
            let mut result__: PhotoImportContentType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoImportContentType>(result__)
        }
    }
    pub fn SizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Date(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn Sibling(&self) -> ::windows::core::Result<PhotoImportSidecar> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoImportSidecar>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Sidecars(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportSidecar>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PhotoImportSidecar>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn VideoSegments(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportVideoSegment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PhotoImportVideoSegment>>(result__)
        }
    }
    pub fn IsSelected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsSelected(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ImportedFileNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeletedFileNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPhotoImportItem2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportItem;{a9d07e76-9bfc-43b8-b356-633b6a988c9e})");
}
unsafe impl ::windows::core::Interface for PhotoImportItem {
    type Vtable = IPhotoImportItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9d07e76_9bfc_43b8_b356_633b6a988c9e);
}
impl ::windows::core::RuntimeName for PhotoImportItem {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportItem";
}
impl ::core::convert::From<PhotoImportItem> for ::windows::core::IUnknown {
    fn from(value: PhotoImportItem) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhotoImportItem> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportItem) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhotoImportItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhotoImportItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhotoImportItem> for ::windows::core::IInspectable {
    fn from(value: PhotoImportItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhotoImportItem> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhotoImportItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhotoImportItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhotoImportItem {}
unsafe impl ::core::marker::Sync for PhotoImportItem {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhotoImportItemImportedEventArgs(pub ::windows::core::IInspectable);
impl PhotoImportItemImportedEventArgs {
    pub fn ImportedItem(&self) -> ::windows::core::Result<PhotoImportItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoImportItem>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportItemImportedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportItemImportedEventArgs;{42cb2fdd-7d68-47b5-bc7c-ceb73e0c77dc})");
}
unsafe impl ::windows::core::Interface for PhotoImportItemImportedEventArgs {
    type Vtable = IPhotoImportItemImportedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42cb2fdd_7d68_47b5_bc7c_ceb73e0c77dc);
}
impl ::windows::core::RuntimeName for PhotoImportItemImportedEventArgs {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportItemImportedEventArgs";
}
impl ::core::convert::From<PhotoImportItemImportedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PhotoImportItemImportedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhotoImportItemImportedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportItemImportedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhotoImportItemImportedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhotoImportItemImportedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhotoImportItemImportedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PhotoImportItemImportedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhotoImportItemImportedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportItemImportedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhotoImportItemImportedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhotoImportItemImportedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhotoImportItemImportedEventArgs {}
unsafe impl ::core::marker::Sync for PhotoImportItemImportedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhotoImportItemSelectionMode(pub i32);
impl PhotoImportItemSelectionMode {
    pub const SelectAll: PhotoImportItemSelectionMode = PhotoImportItemSelectionMode(0i32);
    pub const SelectNone: PhotoImportItemSelectionMode = PhotoImportItemSelectionMode(1i32);
    pub const SelectNew: PhotoImportItemSelectionMode = PhotoImportItemSelectionMode(2i32);
}
impl ::core::convert::From<i32> for PhotoImportItemSelectionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportItemSelectionMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhotoImportItemSelectionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportItemSelectionMode;i4)");
}
impl ::windows::core::DefaultType for PhotoImportItemSelectionMode {
    type DefaultType = Self;
}
pub struct PhotoImportManager {}
impl PhotoImportManager {
    #[cfg(feature = "Foundation")]
    pub fn IsSupportedAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IPhotoImportManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAllSourcesAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PhotoImportSource>>> {
        Self::IPhotoImportManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PhotoImportSource>>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPendingOperations() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportOperation>> {
        Self::IPhotoImportManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PhotoImportOperation>>(result__)
        })
    }
    pub fn IPhotoImportManagerStatics<R, F: FnOnce(&IPhotoImportManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhotoImportManager, IPhotoImportManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PhotoImportManager {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportManager";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhotoImportOperation(pub ::windows::core::IInspectable);
impl PhotoImportOperation {
    pub fn Stage(&self) -> ::windows::core::Result<PhotoImportStage> {
        let this = self;
        unsafe {
            let mut result__: PhotoImportStage = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoImportStage>(result__)
        }
    }
    pub fn Session(&self) -> ::windows::core::Result<PhotoImportSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoImportSession>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ContinueFindingItemsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportFindItemsResult, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportFindItemsResult, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ContinueImportingItemsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportImportItemsResult, PhotoImportProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportImportItemsResult, PhotoImportProgress>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ContinueDeletingImportedItemsFromSourceAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportDeleteImportedItemsFromSourceResult, f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportDeleteImportedItemsFromSourceResult, f64>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportOperation;{d9f797e4-a09a-4ee4-a4b1-20940277a5be})");
}
unsafe impl ::windows::core::Interface for PhotoImportOperation {
    type Vtable = IPhotoImportOperation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9f797e4_a09a_4ee4_a4b1_20940277a5be);
}
impl ::windows::core::RuntimeName for PhotoImportOperation {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportOperation";
}
impl ::core::convert::From<PhotoImportOperation> for ::windows::core::IUnknown {
    fn from(value: PhotoImportOperation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhotoImportOperation> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportOperation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhotoImportOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhotoImportOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhotoImportOperation> for ::windows::core::IInspectable {
    fn from(value: PhotoImportOperation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhotoImportOperation> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhotoImportOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhotoImportOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhotoImportOperation {}
unsafe impl ::core::marker::Sync for PhotoImportOperation {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhotoImportPowerSource(pub i32);
impl PhotoImportPowerSource {
    pub const Unknown: PhotoImportPowerSource = PhotoImportPowerSource(0i32);
    pub const Battery: PhotoImportPowerSource = PhotoImportPowerSource(1i32);
    pub const External: PhotoImportPowerSource = PhotoImportPowerSource(2i32);
}
impl ::core::convert::From<i32> for PhotoImportPowerSource {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportPowerSource {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhotoImportPowerSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportPowerSource;i4)");
}
impl ::windows::core::DefaultType for PhotoImportPowerSource {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PhotoImportProgress {
    pub ItemsImported: u32,
    pub TotalItemsToImport: u32,
    pub BytesImported: u64,
    pub TotalBytesToImport: u64,
    pub ImportProgress: f64,
}
impl PhotoImportProgress {}
impl ::core::default::Default for PhotoImportProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PhotoImportProgress {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PhotoImportProgress").field("ItemsImported", &self.ItemsImported).field("TotalItemsToImport", &self.TotalItemsToImport).field("BytesImported", &self.BytesImported).field("TotalBytesToImport", &self.TotalBytesToImport).field("ImportProgress", &self.ImportProgress).finish()
    }
}
impl ::core::cmp::PartialEq for PhotoImportProgress {
    fn eq(&self, other: &Self) -> bool {
        self.ItemsImported == other.ItemsImported && self.TotalItemsToImport == other.TotalItemsToImport && self.BytesImported == other.BytesImported && self.TotalBytesToImport == other.TotalBytesToImport && self.ImportProgress == other.ImportProgress
    }
}
impl ::core::cmp::Eq for PhotoImportProgress {}
unsafe impl ::windows::core::Abi for PhotoImportProgress {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhotoImportProgress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Media.Import.PhotoImportProgress;u4;u4;u8;u8;f8)");
}
impl ::windows::core::DefaultType for PhotoImportProgress {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhotoImportSelectionChangedEventArgs(pub ::windows::core::IInspectable);
impl PhotoImportSelectionChangedEventArgs {
    pub fn IsSelectionEmpty(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportSelectionChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportSelectionChangedEventArgs;{10461782-fa9d-4c30-8bc9-4d64911572d5})");
}
unsafe impl ::windows::core::Interface for PhotoImportSelectionChangedEventArgs {
    type Vtable = IPhotoImportSelectionChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10461782_fa9d_4c30_8bc9_4d64911572d5);
}
impl ::windows::core::RuntimeName for PhotoImportSelectionChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportSelectionChangedEventArgs";
}
impl ::core::convert::From<PhotoImportSelectionChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PhotoImportSelectionChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhotoImportSelectionChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportSelectionChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhotoImportSelectionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhotoImportSelectionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhotoImportSelectionChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PhotoImportSelectionChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhotoImportSelectionChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportSelectionChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhotoImportSelectionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhotoImportSelectionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhotoImportSelectionChangedEventArgs {}
unsafe impl ::core::marker::Sync for PhotoImportSelectionChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhotoImportSession(pub ::windows::core::IInspectable);
impl PhotoImportSession {
    pub fn Source(&self) -> ::windows::core::Result<PhotoImportSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoImportSource>(result__)
        }
    }
    pub fn SessionId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn SetDestinationFolder<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFolder>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    pub fn DestinationFolder(&self) -> ::windows::core::Result<super::super::Storage::IStorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::IStorageFolder>(result__)
        }
    }
    pub fn SetAppendSessionDateToDestinationFolder(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AppendSessionDateToDestinationFolder(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetSubfolderCreationMode(&self, value: PhotoImportSubfolderCreationMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SubfolderCreationMode(&self) -> ::windows::core::Result<PhotoImportSubfolderCreationMode> {
        let this = self;
        unsafe {
            let mut result__: PhotoImportSubfolderCreationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoImportSubfolderCreationMode>(result__)
        }
    }
    pub fn SetDestinationFileNamePrefix<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DestinationFileNamePrefix(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FindItemsAsync(&self, contenttypefilter: PhotoImportContentTypeFilter, itemselectionmode: PhotoImportItemSelectionMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportFindItemsResult, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), contenttypefilter, itemselectionmode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportFindItemsResult, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn SetSubfolderDateFormat(&self, value: PhotoImportSubfolderDateFormat) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPhotoImportSession2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn SubfolderDateFormat(&self) -> ::windows::core::Result<PhotoImportSubfolderDateFormat> {
        let this = &::windows::core::Interface::cast::<IPhotoImportSession2>(self)?;
        unsafe {
            let mut result__: PhotoImportSubfolderDateFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoImportSubfolderDateFormat>(result__)
        }
    }
    pub fn SetRememberDeselectedItems(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPhotoImportSession2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RememberDeselectedItems(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPhotoImportSession2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportSession;{aa63916e-ecdb-4efe-94c6-5f5cafe34cfb})");
}
unsafe impl ::windows::core::Interface for PhotoImportSession {
    type Vtable = IPhotoImportSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa63916e_ecdb_4efe_94c6_5f5cafe34cfb);
}
impl ::windows::core::RuntimeName for PhotoImportSession {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportSession";
}
impl ::core::convert::From<PhotoImportSession> for ::windows::core::IUnknown {
    fn from(value: PhotoImportSession) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhotoImportSession> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhotoImportSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhotoImportSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhotoImportSession> for ::windows::core::IInspectable {
    fn from(value: PhotoImportSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhotoImportSession> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhotoImportSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhotoImportSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<PhotoImportSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: PhotoImportSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&PhotoImportSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &PhotoImportSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for PhotoImportSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &PhotoImportSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PhotoImportSession {}
unsafe impl ::core::marker::Sync for PhotoImportSession {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhotoImportSidecar(pub ::windows::core::IInspectable);
impl PhotoImportSidecar {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Date(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportSidecar {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportSidecar;{46d7d757-f802-44c7-9c98-7a71f4bc1486})");
}
unsafe impl ::windows::core::Interface for PhotoImportSidecar {
    type Vtable = IPhotoImportSidecar_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46d7d757_f802_44c7_9c98_7a71f4bc1486);
}
impl ::windows::core::RuntimeName for PhotoImportSidecar {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportSidecar";
}
impl ::core::convert::From<PhotoImportSidecar> for ::windows::core::IUnknown {
    fn from(value: PhotoImportSidecar) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhotoImportSidecar> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportSidecar) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhotoImportSidecar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhotoImportSidecar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhotoImportSidecar> for ::windows::core::IInspectable {
    fn from(value: PhotoImportSidecar) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhotoImportSidecar> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportSidecar) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhotoImportSidecar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhotoImportSidecar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhotoImportSidecar {}
unsafe impl ::core::marker::Sync for PhotoImportSidecar {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhotoImportSource(pub ::windows::core::IInspectable);
impl PhotoImportSource {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Manufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Model(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SerialNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ConnectionProtocol(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ConnectionTransport(&self) -> ::windows::core::Result<PhotoImportConnectionTransport> {
        let this = self;
        unsafe {
            let mut result__: PhotoImportConnectionTransport = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoImportConnectionTransport>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<PhotoImportSourceType> {
        let this = self;
        unsafe {
            let mut result__: PhotoImportSourceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoImportSourceType>(result__)
        }
    }
    pub fn PowerSource(&self) -> ::windows::core::Result<PhotoImportPowerSource> {
        let this = self;
        unsafe {
            let mut result__: PhotoImportPowerSource = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoImportPowerSource>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BatteryLevelPercent(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DateTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn StorageMedia(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportStorageMedium>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PhotoImportStorageMedium>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn IsLocked(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<bool>>(result__)
        }
    }
    pub fn IsMassStorage(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    pub fn CreateImportSession(&self) -> ::windows::core::Result<PhotoImportSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoImportSession>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(sourceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhotoImportSource>> {
        Self::IPhotoImportSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), sourceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhotoImportSource>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn FromFolderAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFolder>>(sourcerootfolder: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhotoImportSource>> {
        Self::IPhotoImportSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), sourcerootfolder.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PhotoImportSource>>(result__)
        })
    }
    pub fn IPhotoImportSourceStatics<R, F: FnOnce(&IPhotoImportSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PhotoImportSource, IPhotoImportSourceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportSource;{1f8ea35e-145b-4cd6-87f1-54965a982fef})");
}
unsafe impl ::windows::core::Interface for PhotoImportSource {
    type Vtable = IPhotoImportSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f8ea35e_145b_4cd6_87f1_54965a982fef);
}
impl ::windows::core::RuntimeName for PhotoImportSource {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportSource";
}
impl ::core::convert::From<PhotoImportSource> for ::windows::core::IUnknown {
    fn from(value: PhotoImportSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhotoImportSource> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhotoImportSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhotoImportSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhotoImportSource> for ::windows::core::IInspectable {
    fn from(value: PhotoImportSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhotoImportSource> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhotoImportSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhotoImportSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhotoImportSource {}
unsafe impl ::core::marker::Sync for PhotoImportSource {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhotoImportSourceType(pub i32);
impl PhotoImportSourceType {
    pub const Generic: PhotoImportSourceType = PhotoImportSourceType(0i32);
    pub const Camera: PhotoImportSourceType = PhotoImportSourceType(1i32);
    pub const MediaPlayer: PhotoImportSourceType = PhotoImportSourceType(2i32);
    pub const Phone: PhotoImportSourceType = PhotoImportSourceType(3i32);
    pub const Video: PhotoImportSourceType = PhotoImportSourceType(4i32);
    pub const PersonalInfoManager: PhotoImportSourceType = PhotoImportSourceType(5i32);
    pub const AudioRecorder: PhotoImportSourceType = PhotoImportSourceType(6i32);
}
impl ::core::convert::From<i32> for PhotoImportSourceType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportSourceType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhotoImportSourceType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportSourceType;i4)");
}
impl ::windows::core::DefaultType for PhotoImportSourceType {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhotoImportStage(pub i32);
impl PhotoImportStage {
    pub const NotStarted: PhotoImportStage = PhotoImportStage(0i32);
    pub const FindingItems: PhotoImportStage = PhotoImportStage(1i32);
    pub const ImportingItems: PhotoImportStage = PhotoImportStage(2i32);
    pub const DeletingImportedItemsFromSource: PhotoImportStage = PhotoImportStage(3i32);
}
impl ::core::convert::From<i32> for PhotoImportStage {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportStage {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhotoImportStage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportStage;i4)");
}
impl ::windows::core::DefaultType for PhotoImportStage {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhotoImportStorageMedium(pub ::windows::core::IInspectable);
impl PhotoImportStorageMedium {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SerialNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn StorageMediumType(&self) -> ::windows::core::Result<PhotoImportStorageMediumType> {
        let this = self;
        unsafe {
            let mut result__: PhotoImportStorageMediumType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoImportStorageMediumType>(result__)
        }
    }
    pub fn SupportedAccessMode(&self) -> ::windows::core::Result<PhotoImportAccessMode> {
        let this = self;
        unsafe {
            let mut result__: PhotoImportAccessMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoImportAccessMode>(result__)
        }
    }
    pub fn CapacityInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn AvailableSpaceInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    pub fn Refresh(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportStorageMedium {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportStorageMedium;{f2b9b093-fc85-487f-87c2-58d675d05b07})");
}
unsafe impl ::windows::core::Interface for PhotoImportStorageMedium {
    type Vtable = IPhotoImportStorageMedium_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2b9b093_fc85_487f_87c2_58d675d05b07);
}
impl ::windows::core::RuntimeName for PhotoImportStorageMedium {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportStorageMedium";
}
impl ::core::convert::From<PhotoImportStorageMedium> for ::windows::core::IUnknown {
    fn from(value: PhotoImportStorageMedium) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhotoImportStorageMedium> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportStorageMedium) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhotoImportStorageMedium {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhotoImportStorageMedium {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhotoImportStorageMedium> for ::windows::core::IInspectable {
    fn from(value: PhotoImportStorageMedium) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhotoImportStorageMedium> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportStorageMedium) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhotoImportStorageMedium {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhotoImportStorageMedium {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhotoImportStorageMedium {}
unsafe impl ::core::marker::Sync for PhotoImportStorageMedium {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhotoImportStorageMediumType(pub i32);
impl PhotoImportStorageMediumType {
    pub const Undefined: PhotoImportStorageMediumType = PhotoImportStorageMediumType(0i32);
    pub const Fixed: PhotoImportStorageMediumType = PhotoImportStorageMediumType(1i32);
    pub const Removable: PhotoImportStorageMediumType = PhotoImportStorageMediumType(2i32);
}
impl ::core::convert::From<i32> for PhotoImportStorageMediumType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportStorageMediumType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhotoImportStorageMediumType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportStorageMediumType;i4)");
}
impl ::windows::core::DefaultType for PhotoImportStorageMediumType {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhotoImportSubfolderCreationMode(pub i32);
impl PhotoImportSubfolderCreationMode {
    pub const DoNotCreateSubfolders: PhotoImportSubfolderCreationMode = PhotoImportSubfolderCreationMode(0i32);
    pub const CreateSubfoldersFromFileDate: PhotoImportSubfolderCreationMode = PhotoImportSubfolderCreationMode(1i32);
    pub const CreateSubfoldersFromExifDate: PhotoImportSubfolderCreationMode = PhotoImportSubfolderCreationMode(2i32);
    pub const KeepOriginalFolderStructure: PhotoImportSubfolderCreationMode = PhotoImportSubfolderCreationMode(3i32);
}
impl ::core::convert::From<i32> for PhotoImportSubfolderCreationMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportSubfolderCreationMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhotoImportSubfolderCreationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportSubfolderCreationMode;i4)");
}
impl ::windows::core::DefaultType for PhotoImportSubfolderCreationMode {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PhotoImportSubfolderDateFormat(pub i32);
impl PhotoImportSubfolderDateFormat {
    pub const Year: PhotoImportSubfolderDateFormat = PhotoImportSubfolderDateFormat(0i32);
    pub const YearMonth: PhotoImportSubfolderDateFormat = PhotoImportSubfolderDateFormat(1i32);
    pub const YearMonthDay: PhotoImportSubfolderDateFormat = PhotoImportSubfolderDateFormat(2i32);
}
impl ::core::convert::From<i32> for PhotoImportSubfolderDateFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportSubfolderDateFormat {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhotoImportSubfolderDateFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportSubfolderDateFormat;i4)");
}
impl ::windows::core::DefaultType for PhotoImportSubfolderDateFormat {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhotoImportVideoSegment(pub ::windows::core::IInspectable);
impl PhotoImportVideoSegment {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Date(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn Sibling(&self) -> ::windows::core::Result<PhotoImportSidecar> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoImportSidecar>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Sidecars(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportSidecar>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PhotoImportSidecar>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportVideoSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportVideoSegment;{623c0289-321a-41d8-9166-8c62a333276c})");
}
unsafe impl ::windows::core::Interface for PhotoImportVideoSegment {
    type Vtable = IPhotoImportVideoSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x623c0289_321a_41d8_9166_8c62a333276c);
}
impl ::windows::core::RuntimeName for PhotoImportVideoSegment {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportVideoSegment";
}
impl ::core::convert::From<PhotoImportVideoSegment> for ::windows::core::IUnknown {
    fn from(value: PhotoImportVideoSegment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhotoImportVideoSegment> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportVideoSegment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhotoImportVideoSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhotoImportVideoSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhotoImportVideoSegment> for ::windows::core::IInspectable {
    fn from(value: PhotoImportVideoSegment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhotoImportVideoSegment> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportVideoSegment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhotoImportVideoSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhotoImportVideoSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PhotoImportVideoSegment {}
unsafe impl ::core::marker::Sync for PhotoImportVideoSegment {}
