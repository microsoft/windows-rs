#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct INamedResource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INamedResource {
    type Vtable = INamedResourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c98c219_0b13_4240_89a5_d495dc189a00);
}
#[repr(C)]
#[doc(hidden)]
pub struct INamedResourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcecontext: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcecontext: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceCandidate(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceCandidate {
    type Vtable = IResourceCandidateVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf5207d9_c433_4764_b3fd_8fa6bfbcbadc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCandidateVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, qualifiername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceCandidate2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceCandidate2 {
    type Vtable = IResourceCandidate2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69e5b468_f6fc_4013_aaa2_d53f1757d3b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCandidate2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceCandidate3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceCandidate3 {
    type Vtable = IResourceCandidate3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08ae97f8_517a_4674_958c_4a3c7cd2cc6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCandidate3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ResourceCandidateKind) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceContext(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceContext {
    type Vtable = IResourceContextVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2fa22f4b_707e_4b27_ad0d_d0d8cd468fd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContextVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, qualifiernames: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languages: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceContextStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceContextStatics {
    type Vtable = IResourceContextStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98be9d6c_6338_4b31_99df_b2b442f17149);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContextStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceContextStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceContextStatics2 {
    type Vtable = IResourceContextStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41f752ef_12af_41b9_ab36_b1eb4b512460);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContextStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, qualifiernames: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceContextStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceContextStatics3 {
    type Vtable = IResourceContextStatics3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20cf492c_af0f_450b_9da6_106dd0c29a39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContextStatics3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, persistence: ResourceQualifierPersistence) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceContextStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceContextStatics4 {
    type Vtable = IResourceContextStatics4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22eb9ccd_fb31_4bfa_b86b_df9d9d7bdc39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContextStatics4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceManager {
    type Vtable = IResourceManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf744d97b_9988_44fb_abd6_5378844cfa8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, files: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, files: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceManager2 {
    type Vtable = IResourceManager2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d66fe6c_a4d7_4c23_9e85_675f304c252d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, resourcelayoutinfo: ResourceLayoutInfo, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, resourcelayoutinfo: ResourceLayoutInfo, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceManagerStatics {
    type Vtable = IResourceManagerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cc0fdfc_69ee_4e43_9901_47f12687baf7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcereference: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceMap(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceMap {
    type Vtable = IResourceMapVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72284824_db8c_42f8_b08c_53ff357dad82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceMapVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reference: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IResourceQualifier(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResourceQualifier {
    type Vtable = IResourceQualifierVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x785da5b2_4afd_4376_a888_c5f9a6b7a05c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceQualifierVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct NamedResource(::windows::core::IUnknown);
impl NamedResource {
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Candidates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>>(result__)
        }
    }
    pub fn Resolve(&self) -> ::windows::core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResourceCandidate>(result__)
        }
    }
    pub fn ResolveForContext<'a, Param0: ::windows::core::IntoParam<'a, ResourceContext>>(&self, resourcecontext: Param0) -> ::windows::core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), resourcecontext.into_param().abi(), &mut result__).from_abi::<ResourceCandidate>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResolveAll(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResolveAllForContext<'a, Param0: ::windows::core::IntoParam<'a, ResourceContext>>(&self, resourcecontext: Param0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), resourcecontext.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>>(result__)
        }
    }
}
impl ::core::clone::Clone for NamedResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NamedResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NamedResource {}
unsafe impl ::windows::core::RuntimeType for NamedResource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.NamedResource;{1c98c219-0b13-4240-89a5-d495dc189a00})");
}
unsafe impl ::windows::core::Interface for NamedResource {
    type Vtable = INamedResourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c98c219_0b13_4240_89a5_d495dc189a00);
}
impl ::windows::core::RuntimeName for NamedResource {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.NamedResource";
}
impl ::core::convert::From<NamedResource> for ::windows::core::IUnknown {
    fn from(value: NamedResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NamedResource> for ::windows::core::IUnknown {
    fn from(value: &NamedResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NamedResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &NamedResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NamedResource> for ::windows::core::IInspectable {
    fn from(value: NamedResource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NamedResource> for ::windows::core::IInspectable {
    fn from(value: &NamedResource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NamedResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &NamedResource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NamedResource {}
unsafe impl ::core::marker::Sync for NamedResource {}
#[repr(transparent)]
pub struct ResourceCandidate(::windows::core::IUnknown);
impl ResourceCandidate {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Qualifiers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceQualifier>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<ResourceQualifier>>(result__)
        }
    }
    pub fn IsMatch(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsMatchAsDefault(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsDefault(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ValueAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn GetValueAsFileAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::StorageFile>>(result__)
        }
    }
    pub fn GetQualifierValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, qualifiername: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), qualifiername.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetValueAsStreamAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStream>> {
        let this = &::windows::core::Interface::cast::<IResourceCandidate2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStream>>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ResourceCandidateKind> {
        let this = &::windows::core::Interface::cast::<IResourceCandidate3>(self)?;
        unsafe {
            let mut result__: ResourceCandidateKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResourceCandidateKind>(result__)
        }
    }
}
impl ::core::clone::Clone for ResourceCandidate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceCandidate {}
unsafe impl ::windows::core::RuntimeType for ResourceCandidate {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceCandidate;{af5207d9-c433-4764-b3fd-8fa6bfbcbadc})");
}
unsafe impl ::windows::core::Interface for ResourceCandidate {
    type Vtable = IResourceCandidateVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf5207d9_c433_4764_b3fd_8fa6bfbcbadc);
}
impl ::windows::core::RuntimeName for ResourceCandidate {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceCandidate";
}
impl ::core::convert::From<ResourceCandidate> for ::windows::core::IUnknown {
    fn from(value: ResourceCandidate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceCandidate> for ::windows::core::IUnknown {
    fn from(value: &ResourceCandidate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceCandidate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ResourceCandidate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ResourceCandidate> for ::windows::core::IInspectable {
    fn from(value: ResourceCandidate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceCandidate> for ::windows::core::IInspectable {
    fn from(value: &ResourceCandidate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceCandidate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ResourceCandidate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ResourceCandidate {}
unsafe impl ::core::marker::Sync for ResourceCandidate {}
#[repr(transparent)]
pub struct ResourceCandidateKind(pub i32);
impl ResourceCandidateKind {
    pub const String: Self = Self(0i32);
    pub const File: Self = Self(1i32);
    pub const EmbeddedData: Self = Self(2i32);
}
impl ::core::marker::Copy for ResourceCandidateKind {}
impl ::core::clone::Clone for ResourceCandidateKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ResourceCandidateKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ResourceCandidateKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceCandidateKind {}
unsafe impl ::windows::core::RuntimeType for ResourceCandidateKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Resources.Core.ResourceCandidateKind;i4)");
}
impl ::windows::core::DefaultType for ResourceCandidateKind {
    type DefaultType = Self;
}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct ResourceCandidateVectorView(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl ResourceCandidateVectorView {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<ResourceCandidate>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<ResourceCandidate>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<ResourceCandidate>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<ResourceCandidate>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, ResourceCandidate>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<ResourceCandidate as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for ResourceCandidateVectorView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for ResourceCandidateVectorView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for ResourceCandidateVectorView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for ResourceCandidateVectorView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceCandidateVectorView;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.ApplicationModel.Resources.Core.ResourceCandidate;{af5207d9-c433-4764-b3fd-8fa6bfbcbadc})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for ResourceCandidateVectorView {
    type Vtable = super::super::super::Foundation::Collections::IVectorViewVtbl<ResourceCandidate>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ResourceCandidateVectorView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceCandidateVectorView";
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for ResourceCandidateVectorView {
    type Item = ResourceCandidate;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &ResourceCandidateVectorView {
    type Item = ResourceCandidate;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceCandidateVectorView> for ::windows::core::IUnknown {
    fn from(value: ResourceCandidateVectorView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceCandidateVectorView> for ::windows::core::IUnknown {
    fn from(value: &ResourceCandidateVectorView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceCandidateVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ResourceCandidateVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceCandidateVectorView> for ::windows::core::IInspectable {
    fn from(value: ResourceCandidateVectorView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceCandidateVectorView> for ::windows::core::IInspectable {
    fn from(value: &ResourceCandidateVectorView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceCandidateVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ResourceCandidateVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceCandidateVectorView> for super::super::super::Foundation::Collections::IIterable<ResourceCandidate> {
    type Error = ::windows::core::Error;
    fn try_from(value: ResourceCandidateVectorView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceCandidateVectorView> for super::super::super::Foundation::Collections::IIterable<ResourceCandidate> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceCandidateVectorView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<ResourceCandidate>> for ResourceCandidateVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<ResourceCandidate>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<ResourceCandidate>> for &ResourceCandidateVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<ResourceCandidate>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<ResourceCandidate>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceCandidateVectorView> for super::super::super::Foundation::Collections::IVectorView<ResourceCandidate> {
    type Error = ::windows::core::Error;
    fn try_from(value: ResourceCandidateVectorView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceCandidateVectorView> for super::super::super::Foundation::Collections::IVectorView<ResourceCandidate> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceCandidateVectorView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>> for ResourceCandidateVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>> for &ResourceCandidateVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceCandidateVectorView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceCandidateVectorView {}
#[repr(transparent)]
pub struct ResourceContext(::windows::core::IUnknown);
impl ResourceContext {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ResourceContext, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn QualifierValues(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResetQualifierValues<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, qualifiernames: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), qualifiernames.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OverrideToMatch<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<ResourceQualifier>>>(&self, result: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), result.into_param().abi()).ok() }
    }
    pub fn Clone(&self) -> ::windows::core::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResourceContext>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetLanguages<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(&self, languages: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), languages.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateMatchingContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<ResourceQualifier>>>(result: Param0) -> ::windows::core::Result<ResourceContext> {
        Self::IResourceContextStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), result.into_param().abi(), &mut result__).from_abi::<ResourceContext>(result__)
        })
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<ResourceContext> {
        Self::IResourceContextStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResourceContext>(result__)
        })
    }
    pub fn SetGlobalQualifierValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(key: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IResourceContextStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn ResetGlobalQualifierValues() -> ::windows::core::Result<()> {
        Self::IResourceContextStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResetGlobalQualifierValuesForSpecifiedQualifiers<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(qualifiernames: Param0) -> ::windows::core::Result<()> {
        Self::IResourceContextStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), qualifiernames.into_param().abi()).ok() })
    }
    pub fn GetForViewIndependentUse() -> ::windows::core::Result<ResourceContext> {
        Self::IResourceContextStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResourceContext>(result__)
        })
    }
    pub fn SetGlobalQualifierValueWithPersistence<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(key: Param0, value: Param1, persistence: ResourceQualifierPersistence) -> ::windows::core::Result<()> {
        Self::IResourceContextStatics3(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), persistence).ok() })
    }
    #[cfg(feature = "UI")]
    pub fn GetForUIContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::UI::UIContext>>(context: Param0) -> ::windows::core::Result<ResourceContext> {
        Self::IResourceContextStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), context.into_param().abi(), &mut result__).from_abi::<ResourceContext>(result__)
        })
    }
    pub fn IResourceContextStatics<R, F: FnOnce(&IResourceContextStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ResourceContext, IResourceContextStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceContextStatics2<R, F: FnOnce(&IResourceContextStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ResourceContext, IResourceContextStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceContextStatics3<R, F: FnOnce(&IResourceContextStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ResourceContext, IResourceContextStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IResourceContextStatics4<R, F: FnOnce(&IResourceContextStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ResourceContext, IResourceContextStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ResourceContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceContext {}
unsafe impl ::windows::core::RuntimeType for ResourceContext {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceContext;{2fa22f4b-707e-4b27-ad0d-d0d8cd468fd2})");
}
unsafe impl ::windows::core::Interface for ResourceContext {
    type Vtable = IResourceContextVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2fa22f4b_707e_4b27_ad0d_d0d8cd468fd2);
}
impl ::windows::core::RuntimeName for ResourceContext {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceContext";
}
impl ::core::convert::From<ResourceContext> for ::windows::core::IUnknown {
    fn from(value: ResourceContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceContext> for ::windows::core::IUnknown {
    fn from(value: &ResourceContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ResourceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ResourceContext> for ::windows::core::IInspectable {
    fn from(value: ResourceContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceContext> for ::windows::core::IInspectable {
    fn from(value: &ResourceContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ResourceContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ResourceContext {}
unsafe impl ::core::marker::Sync for ResourceContext {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct ResourceContextLanguagesVectorView(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl ResourceContextLanguagesVectorView {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for ResourceContextLanguagesVectorView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for ResourceContextLanguagesVectorView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for ResourceContextLanguagesVectorView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for ResourceContextLanguagesVectorView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceContextLanguagesVectorView;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};string))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for ResourceContextLanguagesVectorView {
    type Vtable = super::super::super::Foundation::Collections::IVectorViewVtbl<::windows::core::HSTRING>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ResourceContextLanguagesVectorView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceContextLanguagesVectorView";
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for ResourceContextLanguagesVectorView {
    type Item = ::windows::core::HSTRING;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &ResourceContextLanguagesVectorView {
    type Item = ::windows::core::HSTRING;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceContextLanguagesVectorView> for ::windows::core::IUnknown {
    fn from(value: ResourceContextLanguagesVectorView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceContextLanguagesVectorView> for ::windows::core::IUnknown {
    fn from(value: &ResourceContextLanguagesVectorView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceContextLanguagesVectorView> for ::windows::core::IInspectable {
    fn from(value: ResourceContextLanguagesVectorView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceContextLanguagesVectorView> for ::windows::core::IInspectable {
    fn from(value: &ResourceContextLanguagesVectorView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceContextLanguagesVectorView> for super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: ResourceContextLanguagesVectorView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceContextLanguagesVectorView> for super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceContextLanguagesVectorView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>> for ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>> for &ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceContextLanguagesVectorView> for super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: ResourceContextLanguagesVectorView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceContextLanguagesVectorView> for super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceContextLanguagesVectorView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> for ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> for &ResourceContextLanguagesVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceContextLanguagesVectorView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceContextLanguagesVectorView {}
#[repr(C)]
pub struct ResourceLayoutInfo {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ResourceSubtreeCount: u32,
    pub NamedResourceCount: u32,
    pub Checksum: i32,
}
impl ::core::marker::Copy for ResourceLayoutInfo {}
impl ::core::clone::Clone for ResourceLayoutInfo {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ResourceLayoutInfo {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ResourceLayoutInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.ApplicationModel.Resources.Core.ResourceLayoutInfo;u4;u4;u4;u4;i4)");
}
impl ::windows::core::DefaultType for ResourceLayoutInfo {
    type DefaultType = Self;
}
impl ::core::cmp::PartialEq for ResourceLayoutInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ResourceLayoutInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for ResourceLayoutInfo {}
impl ::core::default::Default for ResourceLayoutInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct ResourceManager(::windows::core::IUnknown);
impl ResourceManager {
    pub fn MainResourceMap(&self) -> ::windows::core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResourceMap>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllResourceMaps(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ResourceMap>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ResourceMap>>(result__)
        }
    }
    pub fn DefaultContext(&self) -> ::windows::core::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResourceContext>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn LoadPriFiles<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile>>>(&self, files: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), files.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn UnloadPriFiles<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile>>>(&self, files: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), files.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllNamedResourcesForPackage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ResourceLayoutInfo>>(&self, packagename: Param0, resourcelayoutinfo: Param1) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<NamedResource>> {
        let this = &::windows::core::Interface::cast::<IResourceManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), packagename.into_param().abi(), resourcelayoutinfo.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<NamedResource>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllSubtreesForPackage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ResourceLayoutInfo>>(&self, packagename: Param0, resourcelayoutinfo: Param1) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceMap>> {
        let this = &::windows::core::Interface::cast::<IResourceManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), packagename.into_param().abi(), resourcelayoutinfo.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<ResourceMap>>(result__)
        }
    }
    pub fn Current() -> ::windows::core::Result<ResourceManager> {
        Self::IResourceManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResourceManager>(result__)
        })
    }
    pub fn IsResourceReference<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(resourcereference: Param0) -> ::windows::core::Result<bool> {
        Self::IResourceManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), resourcereference.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IResourceManagerStatics<R, F: FnOnce(&IResourceManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ResourceManager, IResourceManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ResourceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceManager {}
unsafe impl ::windows::core::RuntimeType for ResourceManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceManager;{f744d97b-9988-44fb-abd6-5378844cfa8b})");
}
unsafe impl ::windows::core::Interface for ResourceManager {
    type Vtable = IResourceManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf744d97b_9988_44fb_abd6_5378844cfa8b);
}
impl ::windows::core::RuntimeName for ResourceManager {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceManager";
}
impl ::core::convert::From<ResourceManager> for ::windows::core::IUnknown {
    fn from(value: ResourceManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceManager> for ::windows::core::IUnknown {
    fn from(value: &ResourceManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ResourceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ResourceManager> for ::windows::core::IInspectable {
    fn from(value: ResourceManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceManager> for ::windows::core::IInspectable {
    fn from(value: &ResourceManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ResourceManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ResourceManager {}
unsafe impl ::core::marker::Sync for ResourceManager {}
#[repr(transparent)]
pub struct ResourceMap(::windows::core::IUnknown);
impl ResourceMap {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<NamedResource> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, NamedResource>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<NamedResource>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, NamedResource>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, NamedResource>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Split(&self, first: &mut ::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, NamedResource>>, second: &mut ::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, NamedResource>>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, NamedResource>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, resource: Param0) -> ::windows::core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), resource.into_param().abi(), &mut result__).from_abi::<ResourceCandidate>(result__)
        }
    }
    pub fn GetValueForContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ResourceContext>>(&self, resource: Param0, context: Param1) -> ::windows::core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), resource.into_param().abi(), context.into_param().abi(), &mut result__).from_abi::<ResourceCandidate>(result__)
        }
    }
    pub fn GetSubtree<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, reference: Param0) -> ::windows::core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), reference.into_param().abi(), &mut result__).from_abi::<ResourceMap>(result__)
        }
    }
}
impl ::core::clone::Clone for ResourceMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceMap {}
unsafe impl ::windows::core::RuntimeType for ResourceMap {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceMap;{72284824-db8c-42f8-b08c-53ff357dad82})");
}
unsafe impl ::windows::core::Interface for ResourceMap {
    type Vtable = IResourceMapVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72284824_db8c_42f8_b08c_53ff357dad82);
}
impl ::windows::core::RuntimeName for ResourceMap {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceMap";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for ResourceMap {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &ResourceMap {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl ::core::convert::From<ResourceMap> for ::windows::core::IUnknown {
    fn from(value: ResourceMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceMap> for ::windows::core::IUnknown {
    fn from(value: &ResourceMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ResourceMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ResourceMap> for ::windows::core::IInspectable {
    fn from(value: ResourceMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceMap> for ::windows::core::IInspectable {
    fn from(value: &ResourceMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ResourceMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceMap> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>> {
    type Error = ::windows::core::Error;
    fn try_from(value: ResourceMap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceMap> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceMap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>>> for ResourceMap {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>>> for &ResourceMap {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceMap> for super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, NamedResource> {
    type Error = ::windows::core::Error;
    fn try_from(value: ResourceMap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceMap> for super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, NamedResource> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceMap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, NamedResource>> for ResourceMap {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, NamedResource>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, NamedResource>> for &ResourceMap {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, NamedResource>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, NamedResource>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ResourceMap {}
unsafe impl ::core::marker::Sync for ResourceMap {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct ResourceMapIterator(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl ResourceMapIterator {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Current(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasCurrent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MoveNext(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, items: &mut [<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource> as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for ResourceMapIterator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for ResourceMapIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for ResourceMapIterator {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for ResourceMapIterator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceMapIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};pinterface({02b51929-c1c4-4a7e-8940-0312b5c18500};string;rc(Windows.ApplicationModel.Resources.Core.NamedResource;{1c98c219-0b13-4240-89a5-d495dc189a00}))))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for ResourceMapIterator {
    type Vtable = super::super::super::Foundation::Collections::IIteratorVtbl<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ResourceMapIterator {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceMapIterator";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceMapIterator> for ::windows::core::IUnknown {
    fn from(value: ResourceMapIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceMapIterator> for ::windows::core::IUnknown {
    fn from(value: &ResourceMapIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceMapIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ResourceMapIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceMapIterator> for ::windows::core::IInspectable {
    fn from(value: ResourceMapIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceMapIterator> for ::windows::core::IInspectable {
    fn from(value: &ResourceMapIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceMapIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ResourceMapIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceMapIterator> for super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>> {
    type Error = ::windows::core::Error;
    fn try_from(value: ResourceMapIterator) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceMapIterator> for super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceMapIterator) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>>> for ResourceMapIterator {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>>> for &ResourceMapIterator {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, NamedResource>>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceMapIterator {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceMapIterator {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct ResourceMapMapView(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl ResourceMapMapView {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap>>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap>>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<ResourceMap>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Split(&self, first: &mut ::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ResourceMap>>, second: &mut ::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ResourceMap>>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for ResourceMapMapView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for ResourceMapMapView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for ResourceMapMapView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for ResourceMapMapView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceMapMapView;pinterface({e480ce40-a338-4ada-adcf-272272e48cb9};string;rc(Windows.ApplicationModel.Resources.Core.ResourceMap;{72284824-db8c-42f8-b08c-53ff357dad82})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for ResourceMapMapView {
    type Vtable = super::super::super::Foundation::Collections::IMapViewVtbl<::windows::core::HSTRING, ResourceMap>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ResourceMapMapView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceMapMapView";
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for ResourceMapMapView {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &ResourceMapMapView {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceMapMapView> for ::windows::core::IUnknown {
    fn from(value: ResourceMapMapView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceMapMapView> for ::windows::core::IUnknown {
    fn from(value: &ResourceMapMapView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceMapMapView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ResourceMapMapView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceMapMapView> for ::windows::core::IInspectable {
    fn from(value: ResourceMapMapView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceMapMapView> for ::windows::core::IInspectable {
    fn from(value: &ResourceMapMapView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceMapMapView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ResourceMapMapView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceMapMapView> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap>> {
    type Error = ::windows::core::Error;
    fn try_from(value: ResourceMapMapView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceMapMapView> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceMapMapView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap>>> for ResourceMapMapView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap>>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap>>> for &ResourceMapMapView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap>>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap>>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceMapMapView> for super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ResourceMap> {
    type Error = ::windows::core::Error;
    fn try_from(value: ResourceMapMapView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceMapMapView> for super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ResourceMap> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceMapMapView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ResourceMap>> for ResourceMapMapView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ResourceMap>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ResourceMap>> for &ResourceMapMapView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ResourceMap>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ResourceMap>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceMapMapView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceMapMapView {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct ResourceMapMapViewIterator(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl ResourceMapMapViewIterator {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Current(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasCurrent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MoveNext(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, items: &mut [<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap> as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for ResourceMapMapViewIterator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for ResourceMapMapViewIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for ResourceMapMapViewIterator {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for ResourceMapMapViewIterator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceMapMapViewIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};pinterface({02b51929-c1c4-4a7e-8940-0312b5c18500};string;rc(Windows.ApplicationModel.Resources.Core.ResourceMap;{72284824-db8c-42f8-b08c-53ff357dad82}))))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for ResourceMapMapViewIterator {
    type Vtable = super::super::super::Foundation::Collections::IIteratorVtbl<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap>>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ResourceMapMapViewIterator {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceMapMapViewIterator";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceMapMapViewIterator> for ::windows::core::IUnknown {
    fn from(value: ResourceMapMapViewIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceMapMapViewIterator> for ::windows::core::IUnknown {
    fn from(value: &ResourceMapMapViewIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceMapMapViewIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ResourceMapMapViewIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceMapMapViewIterator> for ::windows::core::IInspectable {
    fn from(value: ResourceMapMapViewIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceMapMapViewIterator> for ::windows::core::IInspectable {
    fn from(value: &ResourceMapMapViewIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceMapMapViewIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ResourceMapMapViewIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceMapMapViewIterator> for super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap>> {
    type Error = ::windows::core::Error;
    fn try_from(value: ResourceMapMapViewIterator) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceMapMapViewIterator> for super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceMapMapViewIterator) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap>>> for ResourceMapMapViewIterator {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap>>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap>>> for &ResourceMapMapViewIterator {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap>>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ResourceMap>>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceMapMapViewIterator {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceMapMapViewIterator {}
#[repr(transparent)]
pub struct ResourceQualifier(::windows::core::IUnknown);
impl ResourceQualifier {
    pub fn QualifierName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn QualifierValue(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IsDefault(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsMatch(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Score(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for ResourceQualifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResourceQualifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceQualifier {}
unsafe impl ::windows::core::RuntimeType for ResourceQualifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceQualifier;{785da5b2-4afd-4376-a888-c5f9a6b7a05c})");
}
unsafe impl ::windows::core::Interface for ResourceQualifier {
    type Vtable = IResourceQualifierVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x785da5b2_4afd_4376_a888_c5f9a6b7a05c);
}
impl ::windows::core::RuntimeName for ResourceQualifier {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceQualifier";
}
impl ::core::convert::From<ResourceQualifier> for ::windows::core::IUnknown {
    fn from(value: ResourceQualifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceQualifier> for ::windows::core::IUnknown {
    fn from(value: &ResourceQualifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceQualifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ResourceQualifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ResourceQualifier> for ::windows::core::IInspectable {
    fn from(value: ResourceQualifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResourceQualifier> for ::windows::core::IInspectable {
    fn from(value: &ResourceQualifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceQualifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ResourceQualifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ResourceQualifier {}
unsafe impl ::core::marker::Sync for ResourceQualifier {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct ResourceQualifierMapView(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl ResourceQualifierMapView {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Split(&self, first: &mut ::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>, second: &mut ::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for ResourceQualifierMapView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for ResourceQualifierMapView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for ResourceQualifierMapView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for ResourceQualifierMapView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceQualifierMapView;pinterface({e480ce40-a338-4ada-adcf-272272e48cb9};string;string))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for ResourceQualifierMapView {
    type Vtable = super::super::super::Foundation::Collections::IMapViewVtbl<::windows::core::HSTRING, ::windows::core::HSTRING>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ResourceQualifierMapView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceQualifierMapView";
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for ResourceQualifierMapView {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &ResourceQualifierMapView {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceQualifierMapView> for ::windows::core::IUnknown {
    fn from(value: ResourceQualifierMapView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceQualifierMapView> for ::windows::core::IUnknown {
    fn from(value: &ResourceQualifierMapView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceQualifierMapView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ResourceQualifierMapView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceQualifierMapView> for ::windows::core::IInspectable {
    fn from(value: ResourceQualifierMapView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceQualifierMapView> for ::windows::core::IInspectable {
    fn from(value: &ResourceQualifierMapView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceQualifierMapView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ResourceQualifierMapView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceQualifierMapView> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: ResourceQualifierMapView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceQualifierMapView> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceQualifierMapView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> for ResourceQualifierMapView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> for &ResourceQualifierMapView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceQualifierMapView> for super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: ResourceQualifierMapView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceQualifierMapView> for super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceQualifierMapView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> for ResourceQualifierMapView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> for &ResourceQualifierMapView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceQualifierMapView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceQualifierMapView {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct ResourceQualifierObservableMap(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl ResourceQualifierObservableMap {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0, value: Param1) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn MapChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::MapChangedEventHandler<::windows::core::HSTRING, ::windows::core::HSTRING>>>(&self, vhnd: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), vhnd.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RemoveMapChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for ResourceQualifierObservableMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for ResourceQualifierObservableMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for ResourceQualifierObservableMap {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for ResourceQualifierObservableMap {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceQualifierObservableMap;pinterface({65df2bf5-bf39-41b5-aebc-5a9d865e472b};string;string))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for ResourceQualifierObservableMap {
    type Vtable = super::super::super::Foundation::Collections::IObservableMapVtbl<::windows::core::HSTRING, ::windows::core::HSTRING>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ResourceQualifierObservableMap {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceQualifierObservableMap";
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for ResourceQualifierObservableMap {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &ResourceQualifierObservableMap {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceQualifierObservableMap> for ::windows::core::IUnknown {
    fn from(value: ResourceQualifierObservableMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceQualifierObservableMap> for ::windows::core::IUnknown {
    fn from(value: &ResourceQualifierObservableMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceQualifierObservableMap> for ::windows::core::IInspectable {
    fn from(value: ResourceQualifierObservableMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceQualifierObservableMap> for ::windows::core::IInspectable {
    fn from(value: &ResourceQualifierObservableMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceQualifierObservableMap> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: ResourceQualifierObservableMap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceQualifierObservableMap> for super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceQualifierObservableMap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> for ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> for &ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceQualifierObservableMap> for super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: ResourceQualifierObservableMap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceQualifierObservableMap> for super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceQualifierObservableMap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> for ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> for &ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceQualifierObservableMap> for super::super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: ResourceQualifierObservableMap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceQualifierObservableMap> for super::super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::HSTRING> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceQualifierObservableMap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::HSTRING>> for ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::HSTRING>> for &ResourceQualifierObservableMap {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::HSTRING>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceQualifierObservableMap {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceQualifierObservableMap {}
#[repr(transparent)]
pub struct ResourceQualifierPersistence(pub i32);
impl ResourceQualifierPersistence {
    pub const None: Self = Self(0i32);
    pub const LocalMachine: Self = Self(1i32);
}
impl ::core::marker::Copy for ResourceQualifierPersistence {}
impl ::core::clone::Clone for ResourceQualifierPersistence {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ResourceQualifierPersistence {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ResourceQualifierPersistence {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResourceQualifierPersistence {}
unsafe impl ::windows::core::RuntimeType for ResourceQualifierPersistence {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Resources.Core.ResourceQualifierPersistence;i4)");
}
impl ::windows::core::DefaultType for ResourceQualifierPersistence {
    type DefaultType = Self;
}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct ResourceQualifierVectorView(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl ResourceQualifierVectorView {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<ResourceQualifier>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<ResourceQualifier>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<ResourceQualifier>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<ResourceQualifier> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<ResourceQualifier>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, ResourceQualifier>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [<ResourceQualifier as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for ResourceQualifierVectorView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for ResourceQualifierVectorView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for ResourceQualifierVectorView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for ResourceQualifierVectorView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceQualifierVectorView;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.ApplicationModel.Resources.Core.ResourceQualifier;{785da5b2-4afd-4376-a888-c5f9a6b7a05c})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for ResourceQualifierVectorView {
    type Vtable = super::super::super::Foundation::Collections::IVectorViewVtbl<ResourceQualifier>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ResourceQualifierVectorView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceQualifierVectorView";
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for ResourceQualifierVectorView {
    type Item = ResourceQualifier;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &ResourceQualifierVectorView {
    type Item = ResourceQualifier;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceQualifierVectorView> for ::windows::core::IUnknown {
    fn from(value: ResourceQualifierVectorView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceQualifierVectorView> for ::windows::core::IUnknown {
    fn from(value: &ResourceQualifierVectorView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResourceQualifierVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ResourceQualifierVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<ResourceQualifierVectorView> for ::windows::core::IInspectable {
    fn from(value: ResourceQualifierVectorView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&ResourceQualifierVectorView> for ::windows::core::IInspectable {
    fn from(value: &ResourceQualifierVectorView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResourceQualifierVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ResourceQualifierVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceQualifierVectorView> for super::super::super::Foundation::Collections::IIterable<ResourceQualifier> {
    type Error = ::windows::core::Error;
    fn try_from(value: ResourceQualifierVectorView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceQualifierVectorView> for super::super::super::Foundation::Collections::IIterable<ResourceQualifier> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceQualifierVectorView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<ResourceQualifier>> for ResourceQualifierVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<ResourceQualifier>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<ResourceQualifier>> for &ResourceQualifierVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<ResourceQualifier>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<ResourceQualifier>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ResourceQualifierVectorView> for super::super::super::Foundation::Collections::IVectorView<ResourceQualifier> {
    type Error = ::windows::core::Error;
    fn try_from(value: ResourceQualifierVectorView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ResourceQualifierVectorView> for super::super::super::Foundation::Collections::IVectorView<ResourceQualifier> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ResourceQualifierVectorView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<ResourceQualifier>> for ResourceQualifierVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVectorView<ResourceQualifier>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<ResourceQualifier>> for &ResourceQualifierVectorView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVectorView<ResourceQualifier>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVectorView<ResourceQualifier>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceQualifierVectorView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceQualifierVectorView {}
