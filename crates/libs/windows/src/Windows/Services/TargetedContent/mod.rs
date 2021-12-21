#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentAction(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITargetedContentAction {
    type Vtable = ITargetedContentActionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd75b691e_6cd6_4ca0_9d8f_4728b0b7e6b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentActionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentAvailabilityChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITargetedContentAvailabilityChangedEventArgs {
    type Vtable = ITargetedContentAvailabilityChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0f59d26_5927_4450_965c_1ceb7becde65);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentAvailabilityChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITargetedContentChangedEventArgs {
    type Vtable = ITargetedContentChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99d488c9_587e_4586_8ef7_b54ca9453a16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITargetedContentCollection {
    type Vtable = ITargetedContentCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d4b66c5_f163_44ba_9f6e_e1a4c2bb559d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interaction: TargetedContentInteraction) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, custominteractionname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentContainer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITargetedContentContainer {
    type Vtable = ITargetedContentContainerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc2494c9_8837_47c2_850f_d79d64595926);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentContainerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TargetedContentAvailability) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentContainerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITargetedContentContainerStatics {
    type Vtable = ITargetedContentContainerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b47e7fb_2140_4c1f_a736_c59583f227d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentContainerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentImage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITargetedContentImage {
    type Vtable = ITargetedContentImageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7a585d9_779f_4b1e_bbb1_8eaf53fbeab2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentImageVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITargetedContentItem {
    type Vtable = ITargetedContentItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38168dc4_276c_4c32_96ba_565c6e406e74);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentItemVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interaction: TargetedContentInteraction) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, custominteractionname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentItemState(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITargetedContentItemState {
    type Vtable = ITargetedContentItemStateVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73935454_4c65_4b47_a441_472de53c79b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentItemStateVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TargetedContentAppInstallationState) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentObject(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITargetedContentObject {
    type Vtable = ITargetedContentObjectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x041d7969_2212_42d1_9dfa_88a8e3033aa3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentObjectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TargetedContentObjectKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITargetedContentStateChangedEventArgs {
    type Vtable = ITargetedContentStateChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a1cef3d_8073_4416_8df2_546835a6414f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentStateChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentSubscription(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITargetedContentSubscription {
    type Vtable = ITargetedContentSubscriptionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x882c2c49_c652_4c7a_acad_1f7fa2986c73);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentSubscriptionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentSubscriptionOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITargetedContentSubscriptionOptions {
    type Vtable = ITargetedContentSubscriptionOptionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61ee6ad0_2c83_421b_8467_413eaf1aeb97);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentSubscriptionOptionsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentSubscriptionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITargetedContentSubscriptionStatics {
    type Vtable = ITargetedContentSubscriptionStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaddfe80_360d_4916_b53c_7ea27090d02a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentSubscriptionStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subscriptionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subscriptionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ITargetedContentValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITargetedContentValue {
    type Vtable = ITargetedContentValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaafde4b3_4215_4bf8_867f_43f04865f9bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentValueVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TargetedContentValueKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc = "*Required features: 'Services_TargetedContent'*"]
#[repr(transparent)]
pub struct TargetedContentAction(::windows::core::IUnknown);
impl TargetedContentAction {
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn InvokeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for TargetedContentAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentAction {}
impl ::core::fmt::Debug for TargetedContentAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentAction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentAction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentAction;{d75b691e-6cd6-4ca0-9d8f-4728b0b7e6b6})");
}
unsafe impl ::windows::core::Interface for TargetedContentAction {
    type Vtable = ITargetedContentActionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd75b691e_6cd6_4ca0_9d8f_4728b0b7e6b6);
}
impl ::windows::core::RuntimeName for TargetedContentAction {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentAction";
}
impl ::core::convert::From<TargetedContentAction> for ::windows::core::IUnknown {
    fn from(value: TargetedContentAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentAction> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TargetedContentAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentAction> for ::windows::core::IInspectable {
    fn from(value: TargetedContentAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentAction> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TargetedContentAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentAction {}
unsafe impl ::core::marker::Sync for TargetedContentAction {}
#[doc = "*Required features: 'Services_TargetedContent'*"]
#[repr(transparent)]
pub struct TargetedContentAppInstallationState(pub i32);
impl TargetedContentAppInstallationState {
    pub const NotApplicable: Self = Self(0i32);
    pub const NotInstalled: Self = Self(1i32);
    pub const Installed: Self = Self(2i32);
}
impl ::core::marker::Copy for TargetedContentAppInstallationState {}
impl ::core::clone::Clone for TargetedContentAppInstallationState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TargetedContentAppInstallationState {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TargetedContentAppInstallationState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentAppInstallationState {}
impl ::core::fmt::Debug for TargetedContentAppInstallationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentAppInstallationState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentAppInstallationState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentAppInstallationState;i4)");
}
impl ::windows::core::DefaultType for TargetedContentAppInstallationState {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Services_TargetedContent'*"]
#[repr(transparent)]
pub struct TargetedContentAvailability(pub i32);
impl TargetedContentAvailability {
    pub const None: Self = Self(0i32);
    pub const Partial: Self = Self(1i32);
    pub const All: Self = Self(2i32);
}
impl ::core::marker::Copy for TargetedContentAvailability {}
impl ::core::clone::Clone for TargetedContentAvailability {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TargetedContentAvailability {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TargetedContentAvailability {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentAvailability {}
impl ::core::fmt::Debug for TargetedContentAvailability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentAvailability").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentAvailability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentAvailability;i4)");
}
impl ::windows::core::DefaultType for TargetedContentAvailability {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Services_TargetedContent'*"]
#[repr(transparent)]
pub struct TargetedContentAvailabilityChangedEventArgs(::windows::core::IUnknown);
impl TargetedContentAvailabilityChangedEventArgs {
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for TargetedContentAvailabilityChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentAvailabilityChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentAvailabilityChangedEventArgs {}
impl ::core::fmt::Debug for TargetedContentAvailabilityChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentAvailabilityChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentAvailabilityChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentAvailabilityChangedEventArgs;{e0f59d26-5927-4450-965c-1ceb7becde65})");
}
unsafe impl ::windows::core::Interface for TargetedContentAvailabilityChangedEventArgs {
    type Vtable = ITargetedContentAvailabilityChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0f59d26_5927_4450_965c_1ceb7becde65);
}
impl ::windows::core::RuntimeName for TargetedContentAvailabilityChangedEventArgs {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentAvailabilityChangedEventArgs";
}
impl ::core::convert::From<TargetedContentAvailabilityChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: TargetedContentAvailabilityChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentAvailabilityChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentAvailabilityChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TargetedContentAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentAvailabilityChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: TargetedContentAvailabilityChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentAvailabilityChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentAvailabilityChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TargetedContentAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentAvailabilityChangedEventArgs {}
unsafe impl ::core::marker::Sync for TargetedContentAvailabilityChangedEventArgs {}
#[doc = "*Required features: 'Services_TargetedContent'*"]
#[repr(transparent)]
pub struct TargetedContentChangedEventArgs(::windows::core::IUnknown);
impl TargetedContentChangedEventArgs {
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn HasPreviousContentExpired(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for TargetedContentChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentChangedEventArgs {}
impl ::core::fmt::Debug for TargetedContentChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentChangedEventArgs;{99d488c9-587e-4586-8ef7-b54ca9453a16})");
}
unsafe impl ::windows::core::Interface for TargetedContentChangedEventArgs {
    type Vtable = ITargetedContentChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99d488c9_587e_4586_8ef7_b54ca9453a16);
}
impl ::windows::core::RuntimeName for TargetedContentChangedEventArgs {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentChangedEventArgs";
}
impl ::core::convert::From<TargetedContentChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: TargetedContentChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TargetedContentChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: TargetedContentChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TargetedContentChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentChangedEventArgs {}
unsafe impl ::core::marker::Sync for TargetedContentChangedEventArgs {}
#[doc = "*Required features: 'Services_TargetedContent'*"]
#[repr(transparent)]
pub struct TargetedContentCollection(::windows::core::IUnknown);
impl TargetedContentCollection {
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn ReportInteraction(&self, interaction: TargetedContentInteraction) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), interaction).ok() }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn ReportCustomInteraction<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, custominteractionname: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), custominteractionname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, TargetedContentValue>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, TargetedContentValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Collections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentCollection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<TargetedContentCollection>>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<TargetedContentItem>>(result__)
        }
    }
}
impl ::core::clone::Clone for TargetedContentCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentCollection {}
impl ::core::fmt::Debug for TargetedContentCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentCollection;{2d4b66c5-f163-44ba-9f6e-e1a4c2bb559d})");
}
unsafe impl ::windows::core::Interface for TargetedContentCollection {
    type Vtable = ITargetedContentCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d4b66c5_f163_44ba_9f6e_e1a4c2bb559d);
}
impl ::windows::core::RuntimeName for TargetedContentCollection {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentCollection";
}
impl ::core::convert::From<TargetedContentCollection> for ::windows::core::IUnknown {
    fn from(value: TargetedContentCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentCollection> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TargetedContentCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentCollection> for ::windows::core::IInspectable {
    fn from(value: TargetedContentCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentCollection> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TargetedContentCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentCollection {}
unsafe impl ::core::marker::Sync for TargetedContentCollection {}
#[doc = "*Required features: 'Services_TargetedContent'*"]
#[repr(transparent)]
pub struct TargetedContentContainer(::windows::core::IUnknown);
impl TargetedContentContainer {
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn Availability(&self) -> ::windows::core::Result<TargetedContentAvailability> {
        let this = self;
        unsafe {
            let mut result__: TargetedContentAvailability = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentAvailability>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn Content(&self) -> ::windows::core::Result<TargetedContentCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn SelectSingleObject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, path: Param0) -> ::windows::core::Result<TargetedContentObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), path.into_param().abi(), &mut result__).from_abi::<TargetedContentObject>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(contentid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TargetedContentContainer>> {
        Self::ITargetedContentContainerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), contentid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<TargetedContentContainer>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITargetedContentContainerStatics<R, F: FnOnce(&ITargetedContentContainerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TargetedContentContainer, ITargetedContentContainerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TargetedContentContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentContainer {}
impl ::core::fmt::Debug for TargetedContentContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentContainer;{bc2494c9-8837-47c2-850f-d79d64595926})");
}
unsafe impl ::windows::core::Interface for TargetedContentContainer {
    type Vtable = ITargetedContentContainerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc2494c9_8837_47c2_850f_d79d64595926);
}
impl ::windows::core::RuntimeName for TargetedContentContainer {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentContainer";
}
impl ::core::convert::From<TargetedContentContainer> for ::windows::core::IUnknown {
    fn from(value: TargetedContentContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentContainer> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TargetedContentContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentContainer> for ::windows::core::IInspectable {
    fn from(value: TargetedContentContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentContainer> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TargetedContentContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentContainer {}
unsafe impl ::core::marker::Sync for TargetedContentContainer {}
#[doc = "*Required features: 'Services_TargetedContent', 'Storage_Streams'*"]
#[cfg(feature = "Storage_Streams")]
#[repr(transparent)]
pub struct TargetedContentFile(::windows::core::IUnknown);
#[cfg(feature = "Storage_Streams")]
impl TargetedContentFile {
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>>(result__)
        }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::clone::Clone for TargetedContentFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::cmp::PartialEq for TargetedContentFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::cmp::Eq for TargetedContentFile {}
#[cfg(feature = "Storage_Streams")]
impl ::core::fmt::Debug for TargetedContentFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentFile").field(&self.0).finish()
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::RuntimeType for TargetedContentFile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentFile;{33ee3134-1dd6-4e3a-8067-d1c162e8642b})");
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::Interface for TargetedContentFile {
    type Vtable = super::super::Storage::Streams::IRandomAccessStreamReferenceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33ee3134_1dd6_4e3a_8067_d1c162e8642b);
}
#[cfg(feature = "Storage_Streams")]
impl ::windows::core::RuntimeName for TargetedContentFile {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentFile";
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<TargetedContentFile> for ::windows::core::IUnknown {
    fn from(value: TargetedContentFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&TargetedContentFile> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TargetedContentFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<TargetedContentFile> for ::windows::core::IInspectable {
    fn from(value: TargetedContentFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&TargetedContentFile> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TargetedContentFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<TargetedContentFile> for super::super::Storage::Streams::IRandomAccessStreamReference {
    type Error = ::windows::core::Error;
    fn try_from(value: TargetedContentFile) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&TargetedContentFile> for super::super::Storage::Streams::IRandomAccessStreamReference {
    type Error = ::windows::core::Error;
    fn try_from(value: &TargetedContentFile) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference> for TargetedContentFile {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStreamReference> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference> for &TargetedContentFile {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStreamReference> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IRandomAccessStreamReference>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::core::marker::Send for TargetedContentFile {}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::core::marker::Sync for TargetedContentFile {}
#[doc = "*Required features: 'Services_TargetedContent'*"]
#[repr(transparent)]
pub struct TargetedContentImage(::windows::core::IUnknown);
impl TargetedContentImage {
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStreamReference>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for TargetedContentImage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentImage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentImage {}
impl ::core::fmt::Debug for TargetedContentImage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentImage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentImage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentImage;{a7a585d9-779f-4b1e-bbb1-8eaf53fbeab2})");
}
unsafe impl ::windows::core::Interface for TargetedContentImage {
    type Vtable = ITargetedContentImageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7a585d9_779f_4b1e_bbb1_8eaf53fbeab2);
}
impl ::windows::core::RuntimeName for TargetedContentImage {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentImage";
}
impl ::core::convert::From<TargetedContentImage> for ::windows::core::IUnknown {
    fn from(value: TargetedContentImage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentImage> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentImage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentImage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TargetedContentImage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentImage> for ::windows::core::IInspectable {
    fn from(value: TargetedContentImage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentImage> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentImage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentImage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TargetedContentImage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<TargetedContentImage> for super::super::Storage::Streams::IRandomAccessStreamReference {
    type Error = ::windows::core::Error;
    fn try_from(value: TargetedContentImage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&TargetedContentImage> for super::super::Storage::Streams::IRandomAccessStreamReference {
    type Error = ::windows::core::Error;
    fn try_from(value: &TargetedContentImage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference> for TargetedContentImage {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStreamReference> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference> for &TargetedContentImage {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStreamReference> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IRandomAccessStreamReference>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for TargetedContentImage {}
unsafe impl ::core::marker::Sync for TargetedContentImage {}
#[doc = "*Required features: 'Services_TargetedContent'*"]
#[repr(transparent)]
pub struct TargetedContentInteraction(pub i32);
impl TargetedContentInteraction {
    pub const Impression: Self = Self(0i32);
    pub const ClickThrough: Self = Self(1i32);
    pub const Hover: Self = Self(2i32);
    pub const Like: Self = Self(3i32);
    pub const Dislike: Self = Self(4i32);
    pub const Dismiss: Self = Self(5i32);
    pub const Ineligible: Self = Self(6i32);
    pub const Accept: Self = Self(7i32);
    pub const Decline: Self = Self(8i32);
    pub const Defer: Self = Self(9i32);
    pub const Canceled: Self = Self(10i32);
    pub const Conversion: Self = Self(11i32);
    pub const Opportunity: Self = Self(12i32);
}
impl ::core::marker::Copy for TargetedContentInteraction {}
impl ::core::clone::Clone for TargetedContentInteraction {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TargetedContentInteraction {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TargetedContentInteraction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentInteraction {}
impl ::core::fmt::Debug for TargetedContentInteraction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentInteraction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentInteraction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentInteraction;i4)");
}
impl ::windows::core::DefaultType for TargetedContentInteraction {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Services_TargetedContent'*"]
#[repr(transparent)]
pub struct TargetedContentItem(::windows::core::IUnknown);
impl TargetedContentItem {
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn ReportInteraction(&self, interaction: TargetedContentInteraction) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), interaction).ok() }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn ReportCustomInteraction<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, custominteractionname: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), custominteractionname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn State(&self) -> ::windows::core::Result<TargetedContentItemState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentItemState>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, TargetedContentValue>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, TargetedContentValue>>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Collections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentCollection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<TargetedContentCollection>>(result__)
        }
    }
}
impl ::core::clone::Clone for TargetedContentItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentItem {}
impl ::core::fmt::Debug for TargetedContentItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentItem;{38168dc4-276c-4c32-96ba-565c6e406e74})");
}
unsafe impl ::windows::core::Interface for TargetedContentItem {
    type Vtable = ITargetedContentItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38168dc4_276c_4c32_96ba_565c6e406e74);
}
impl ::windows::core::RuntimeName for TargetedContentItem {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentItem";
}
impl ::core::convert::From<TargetedContentItem> for ::windows::core::IUnknown {
    fn from(value: TargetedContentItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentItem> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TargetedContentItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentItem> for ::windows::core::IInspectable {
    fn from(value: TargetedContentItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentItem> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TargetedContentItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentItem {}
unsafe impl ::core::marker::Sync for TargetedContentItem {}
#[doc = "*Required features: 'Services_TargetedContent'*"]
#[repr(transparent)]
pub struct TargetedContentItemState(::windows::core::IUnknown);
impl TargetedContentItemState {
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn ShouldDisplay(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn AppInstallationState(&self) -> ::windows::core::Result<TargetedContentAppInstallationState> {
        let this = self;
        unsafe {
            let mut result__: TargetedContentAppInstallationState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentAppInstallationState>(result__)
        }
    }
}
impl ::core::clone::Clone for TargetedContentItemState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentItemState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentItemState {}
impl ::core::fmt::Debug for TargetedContentItemState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentItemState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentItemState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentItemState;{73935454-4c65-4b47-a441-472de53c79b6})");
}
unsafe impl ::windows::core::Interface for TargetedContentItemState {
    type Vtable = ITargetedContentItemStateVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73935454_4c65_4b47_a441_472de53c79b6);
}
impl ::windows::core::RuntimeName for TargetedContentItemState {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentItemState";
}
impl ::core::convert::From<TargetedContentItemState> for ::windows::core::IUnknown {
    fn from(value: TargetedContentItemState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentItemState> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentItemState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentItemState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TargetedContentItemState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentItemState> for ::windows::core::IInspectable {
    fn from(value: TargetedContentItemState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentItemState> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentItemState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentItemState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TargetedContentItemState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentItemState {}
unsafe impl ::core::marker::Sync for TargetedContentItemState {}
#[doc = "*Required features: 'Services_TargetedContent'*"]
#[repr(transparent)]
pub struct TargetedContentObject(::windows::core::IUnknown);
impl TargetedContentObject {
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn ObjectKind(&self) -> ::windows::core::Result<TargetedContentObjectKind> {
        let this = self;
        unsafe {
            let mut result__: TargetedContentObjectKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentObjectKind>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn Collection(&self) -> ::windows::core::Result<TargetedContentCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentCollection>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn Item(&self) -> ::windows::core::Result<TargetedContentItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentItem>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn Value(&self) -> ::windows::core::Result<TargetedContentValue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentValue>(result__)
        }
    }
}
impl ::core::clone::Clone for TargetedContentObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentObject {}
impl ::core::fmt::Debug for TargetedContentObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentObject {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentObject;{041d7969-2212-42d1-9dfa-88a8e3033aa3})");
}
unsafe impl ::windows::core::Interface for TargetedContentObject {
    type Vtable = ITargetedContentObjectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x041d7969_2212_42d1_9dfa_88a8e3033aa3);
}
impl ::windows::core::RuntimeName for TargetedContentObject {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentObject";
}
impl ::core::convert::From<TargetedContentObject> for ::windows::core::IUnknown {
    fn from(value: TargetedContentObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentObject> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TargetedContentObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentObject> for ::windows::core::IInspectable {
    fn from(value: TargetedContentObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentObject> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TargetedContentObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentObject {}
unsafe impl ::core::marker::Sync for TargetedContentObject {}
#[doc = "*Required features: 'Services_TargetedContent'*"]
#[repr(transparent)]
pub struct TargetedContentObjectKind(pub i32);
impl TargetedContentObjectKind {
    pub const Collection: Self = Self(0i32);
    pub const Item: Self = Self(1i32);
    pub const Value: Self = Self(2i32);
}
impl ::core::marker::Copy for TargetedContentObjectKind {}
impl ::core::clone::Clone for TargetedContentObjectKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TargetedContentObjectKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TargetedContentObjectKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentObjectKind {}
impl ::core::fmt::Debug for TargetedContentObjectKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentObjectKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentObjectKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentObjectKind;i4)");
}
impl ::windows::core::DefaultType for TargetedContentObjectKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Services_TargetedContent'*"]
#[repr(transparent)]
pub struct TargetedContentStateChangedEventArgs(::windows::core::IUnknown);
impl TargetedContentStateChangedEventArgs {
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for TargetedContentStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentStateChangedEventArgs {}
impl ::core::fmt::Debug for TargetedContentStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentStateChangedEventArgs;{9a1cef3d-8073-4416-8df2-546835a6414f})");
}
unsafe impl ::windows::core::Interface for TargetedContentStateChangedEventArgs {
    type Vtable = ITargetedContentStateChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a1cef3d_8073_4416_8df2_546835a6414f);
}
impl ::windows::core::RuntimeName for TargetedContentStateChangedEventArgs {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentStateChangedEventArgs";
}
impl ::core::convert::From<TargetedContentStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: TargetedContentStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TargetedContentStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: TargetedContentStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TargetedContentStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for TargetedContentStateChangedEventArgs {}
#[doc = "*Required features: 'Services_TargetedContent'*"]
#[repr(transparent)]
pub struct TargetedContentSubscription(::windows::core::IUnknown);
impl TargetedContentSubscription {
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetContentContainerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TargetedContentContainer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<TargetedContentContainer>>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContentChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn AvailabilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentAvailabilityChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAvailabilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(subscriptionid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TargetedContentSubscription>> {
        Self::ITargetedContentSubscriptionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), subscriptionid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<TargetedContentSubscription>>(result__)
        })
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn GetOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(subscriptionid: Param0) -> ::windows::core::Result<TargetedContentSubscriptionOptions> {
        Self::ITargetedContentSubscriptionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), subscriptionid.into_param().abi(), &mut result__).from_abi::<TargetedContentSubscriptionOptions>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITargetedContentSubscriptionStatics<R, F: FnOnce(&ITargetedContentSubscriptionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TargetedContentSubscription, ITargetedContentSubscriptionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TargetedContentSubscription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentSubscription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentSubscription {}
impl ::core::fmt::Debug for TargetedContentSubscription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentSubscription").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentSubscription {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentSubscription;{882c2c49-c652-4c7a-acad-1f7fa2986c73})");
}
unsafe impl ::windows::core::Interface for TargetedContentSubscription {
    type Vtable = ITargetedContentSubscriptionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x882c2c49_c652_4c7a_acad_1f7fa2986c73);
}
impl ::windows::core::RuntimeName for TargetedContentSubscription {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentSubscription";
}
impl ::core::convert::From<TargetedContentSubscription> for ::windows::core::IUnknown {
    fn from(value: TargetedContentSubscription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentSubscription> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentSubscription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentSubscription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TargetedContentSubscription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentSubscription> for ::windows::core::IInspectable {
    fn from(value: TargetedContentSubscription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentSubscription> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentSubscription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentSubscription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TargetedContentSubscription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentSubscription {}
unsafe impl ::core::marker::Sync for TargetedContentSubscription {}
#[doc = "*Required features: 'Services_TargetedContent'*"]
#[repr(transparent)]
pub struct TargetedContentSubscriptionOptions(::windows::core::IUnknown);
impl TargetedContentSubscriptionOptions {
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn SubscriptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn AllowPartialContentAvailability(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn SetAllowPartialContentAvailability(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CloudQueryParameters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LocalFilters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn Update(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for TargetedContentSubscriptionOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentSubscriptionOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentSubscriptionOptions {}
impl ::core::fmt::Debug for TargetedContentSubscriptionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentSubscriptionOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentSubscriptionOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentSubscriptionOptions;{61ee6ad0-2c83-421b-8467-413eaf1aeb97})");
}
unsafe impl ::windows::core::Interface for TargetedContentSubscriptionOptions {
    type Vtable = ITargetedContentSubscriptionOptionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61ee6ad0_2c83_421b_8467_413eaf1aeb97);
}
impl ::windows::core::RuntimeName for TargetedContentSubscriptionOptions {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentSubscriptionOptions";
}
impl ::core::convert::From<TargetedContentSubscriptionOptions> for ::windows::core::IUnknown {
    fn from(value: TargetedContentSubscriptionOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentSubscriptionOptions> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentSubscriptionOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentSubscriptionOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TargetedContentSubscriptionOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentSubscriptionOptions> for ::windows::core::IInspectable {
    fn from(value: TargetedContentSubscriptionOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentSubscriptionOptions> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentSubscriptionOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentSubscriptionOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TargetedContentSubscriptionOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentSubscriptionOptions {}
unsafe impl ::core::marker::Sync for TargetedContentSubscriptionOptions {}
#[doc = "*Required features: 'Services_TargetedContent'*"]
#[repr(transparent)]
pub struct TargetedContentValue(::windows::core::IUnknown);
impl TargetedContentValue {
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn ValueKind(&self) -> ::windows::core::Result<TargetedContentValueKind> {
        let this = self;
        unsafe {
            let mut result__: TargetedContentValueKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentValueKind>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn String(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn Number(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn Boolean(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn File(&self) -> ::windows::core::Result<TargetedContentFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentFile>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn ImageFile(&self) -> ::windows::core::Result<TargetedContentImage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentImage>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent'*"]
    pub fn Action(&self) -> ::windows::core::Result<TargetedContentAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentAction>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Strings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn Uris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Numbers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<f64>>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Booleans(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation_Collections', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<TargetedContentFile>>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ImageFiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentImage>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<TargetedContentImage>>(result__)
        }
    }
    #[doc = "*Required features: 'Services_TargetedContent', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Actions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentAction>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<TargetedContentAction>>(result__)
        }
    }
}
impl ::core::clone::Clone for TargetedContentValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TargetedContentValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentValue {}
impl ::core::fmt::Debug for TargetedContentValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentValue;{aafde4b3-4215-4bf8-867f-43f04865f9bf})");
}
unsafe impl ::windows::core::Interface for TargetedContentValue {
    type Vtable = ITargetedContentValueVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaafde4b3_4215_4bf8_867f_43f04865f9bf);
}
impl ::windows::core::RuntimeName for TargetedContentValue {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentValue";
}
impl ::core::convert::From<TargetedContentValue> for ::windows::core::IUnknown {
    fn from(value: TargetedContentValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentValue> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &TargetedContentValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TargetedContentValue> for ::windows::core::IInspectable {
    fn from(value: TargetedContentValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TargetedContentValue> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &TargetedContentValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TargetedContentValue {}
unsafe impl ::core::marker::Sync for TargetedContentValue {}
#[doc = "*Required features: 'Services_TargetedContent'*"]
#[repr(transparent)]
pub struct TargetedContentValueKind(pub i32);
impl TargetedContentValueKind {
    pub const String: Self = Self(0i32);
    pub const Uri: Self = Self(1i32);
    pub const Number: Self = Self(2i32);
    pub const Boolean: Self = Self(3i32);
    pub const File: Self = Self(4i32);
    pub const ImageFile: Self = Self(5i32);
    pub const Action: Self = Self(6i32);
    pub const Strings: Self = Self(7i32);
    pub const Uris: Self = Self(8i32);
    pub const Numbers: Self = Self(9i32);
    pub const Booleans: Self = Self(10i32);
    pub const Files: Self = Self(11i32);
    pub const ImageFiles: Self = Self(12i32);
    pub const Actions: Self = Self(13i32);
}
impl ::core::marker::Copy for TargetedContentValueKind {}
impl ::core::clone::Clone for TargetedContentValueKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TargetedContentValueKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TargetedContentValueKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TargetedContentValueKind {}
impl ::core::fmt::Debug for TargetedContentValueKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TargetedContentValueKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentValueKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentValueKind;i4)");
}
impl ::windows::core::DefaultType for TargetedContentValueKind {
    type DefaultType = Self;
}
