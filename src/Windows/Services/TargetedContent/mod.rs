#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct ITargetedContentAction(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITargetedContentAction {
    type Vtable = ITargetedContentAction_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd75b691e_6cd6_4ca0_9d8f_4728b0b7e6b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentAction_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITargetedContentAvailabilityChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITargetedContentAvailabilityChangedEventArgs {
    type Vtable = ITargetedContentAvailabilityChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0f59d26_5927_4450_965c_1ceb7becde65);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentAvailabilityChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITargetedContentChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITargetedContentChangedEventArgs {
    type Vtable = ITargetedContentChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99d488c9_587e_4586_8ef7_b54ca9453a16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITargetedContentCollection(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITargetedContentCollection {
    type Vtable = ITargetedContentCollection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d4b66c5_f163_44ba_9f6e_e1a4c2bb559d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interaction: TargetedContentInteraction) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, custominteractionname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITargetedContentContainer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITargetedContentContainer {
    type Vtable = ITargetedContentContainer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc2494c9_8837_47c2_850f_d79d64595926);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentContainer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut TargetedContentAvailability) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITargetedContentContainerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITargetedContentContainerStatics {
    type Vtable = ITargetedContentContainerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b47e7fb_2140_4c1f_a736_c59583f227d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentContainerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, contentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITargetedContentImage(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITargetedContentImage {
    type Vtable = ITargetedContentImage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7a585d9_779f_4b1e_bbb1_8eaf53fbeab2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentImage_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITargetedContentItem(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITargetedContentItem {
    type Vtable = ITargetedContentItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38168dc4_276c_4c32_96ba_565c6e406e74);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentItem_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interaction: TargetedContentInteraction) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, custominteractionname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITargetedContentItemState(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITargetedContentItemState {
    type Vtable = ITargetedContentItemState_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73935454_4c65_4b47_a441_472de53c79b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentItemState_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut TargetedContentAppInstallationState) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITargetedContentObject(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITargetedContentObject {
    type Vtable = ITargetedContentObject_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x041d7969_2212_42d1_9dfa_88a8e3033aa3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentObject_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut TargetedContentObjectKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITargetedContentStateChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITargetedContentStateChangedEventArgs {
    type Vtable = ITargetedContentStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a1cef3d_8073_4416_8df2_546835a6414f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITargetedContentSubscription(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITargetedContentSubscription {
    type Vtable = ITargetedContentSubscription_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x882c2c49_c652_4c7a_acad_1f7fa2986c73);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentSubscription_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITargetedContentSubscriptionOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITargetedContentSubscriptionOptions {
    type Vtable = ITargetedContentSubscriptionOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61ee6ad0_2c83_421b_8467_413eaf1aeb97);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentSubscriptionOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITargetedContentSubscriptionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITargetedContentSubscriptionStatics {
    type Vtable = ITargetedContentSubscriptionStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaddfe80_360d_4916_b53c_7ea27090d02a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentSubscriptionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, subscriptionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, subscriptionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITargetedContentValue(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITargetedContentValue {
    type Vtable = ITargetedContentValue_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaafde4b3_4215_4bf8_867f_43f04865f9bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetedContentValue_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut TargetedContentValueKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc = "*Required features: `Services_TargetedContent`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TargetedContentAction(pub ::windows::core::IInspectable);
impl TargetedContentAction {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation`*"]
    pub fn InvokeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentAction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentAction;{d75b691e-6cd6-4ca0-9d8f-4728b0b7e6b6})");
}
unsafe impl ::windows::core::Interface for TargetedContentAction {
    type Vtable = ITargetedContentAction_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd75b691e_6cd6_4ca0_9d8f_4728b0b7e6b6);
}
impl ::windows::core::RuntimeName for TargetedContentAction {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentAction";
}
impl ::core::convert::From<TargetedContentAction> for ::windows::core::IUnknown {
    fn from(value: TargetedContentAction) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TargetedContentAction> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentAction) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TargetedContentAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TargetedContentAction> for ::windows::core::IInspectable {
    fn from(value: TargetedContentAction) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TargetedContentAction> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentAction) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TargetedContentAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TargetedContentAction {}
unsafe impl ::core::marker::Sync for TargetedContentAction {}
#[doc = "*Required features: `Services_TargetedContent`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TargetedContentAppInstallationState(pub i32);
impl TargetedContentAppInstallationState {
    pub const NotApplicable: TargetedContentAppInstallationState = TargetedContentAppInstallationState(0i32);
    pub const NotInstalled: TargetedContentAppInstallationState = TargetedContentAppInstallationState(1i32);
    pub const Installed: TargetedContentAppInstallationState = TargetedContentAppInstallationState(2i32);
}
impl ::core::convert::From<i32> for TargetedContentAppInstallationState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TargetedContentAppInstallationState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TargetedContentAppInstallationState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentAppInstallationState;i4)");
}
impl ::windows::core::DefaultType for TargetedContentAppInstallationState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Services_TargetedContent`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TargetedContentAvailability(pub i32);
impl TargetedContentAvailability {
    pub const None: TargetedContentAvailability = TargetedContentAvailability(0i32);
    pub const Partial: TargetedContentAvailability = TargetedContentAvailability(1i32);
    pub const All: TargetedContentAvailability = TargetedContentAvailability(2i32);
}
impl ::core::convert::From<i32> for TargetedContentAvailability {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TargetedContentAvailability {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TargetedContentAvailability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentAvailability;i4)");
}
impl ::windows::core::DefaultType for TargetedContentAvailability {
    type DefaultType = Self;
}
#[doc = "*Required features: `Services_TargetedContent`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TargetedContentAvailabilityChangedEventArgs(pub ::windows::core::IInspectable);
impl TargetedContentAvailabilityChangedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentAvailabilityChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentAvailabilityChangedEventArgs;{e0f59d26-5927-4450-965c-1ceb7becde65})");
}
unsafe impl ::windows::core::Interface for TargetedContentAvailabilityChangedEventArgs {
    type Vtable = ITargetedContentAvailabilityChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0f59d26_5927_4450_965c_1ceb7becde65);
}
impl ::windows::core::RuntimeName for TargetedContentAvailabilityChangedEventArgs {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentAvailabilityChangedEventArgs";
}
impl ::core::convert::From<TargetedContentAvailabilityChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: TargetedContentAvailabilityChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TargetedContentAvailabilityChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentAvailabilityChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TargetedContentAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TargetedContentAvailabilityChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: TargetedContentAvailabilityChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TargetedContentAvailabilityChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentAvailabilityChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TargetedContentAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TargetedContentAvailabilityChangedEventArgs {}
unsafe impl ::core::marker::Sync for TargetedContentAvailabilityChangedEventArgs {}
#[doc = "*Required features: `Services_TargetedContent`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TargetedContentChangedEventArgs(pub ::windows::core::IInspectable);
impl TargetedContentChangedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn HasPreviousContentExpired(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentChangedEventArgs;{99d488c9-587e-4586-8ef7-b54ca9453a16})");
}
unsafe impl ::windows::core::Interface for TargetedContentChangedEventArgs {
    type Vtable = ITargetedContentChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99d488c9_587e_4586_8ef7_b54ca9453a16);
}
impl ::windows::core::RuntimeName for TargetedContentChangedEventArgs {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentChangedEventArgs";
}
impl ::core::convert::From<TargetedContentChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: TargetedContentChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TargetedContentChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TargetedContentChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TargetedContentChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: TargetedContentChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TargetedContentChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TargetedContentChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TargetedContentChangedEventArgs {}
unsafe impl ::core::marker::Sync for TargetedContentChangedEventArgs {}
#[doc = "*Required features: `Services_TargetedContent`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TargetedContentCollection(pub ::windows::core::IInspectable);
impl TargetedContentCollection {
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn ReportInteraction(&self, interaction: TargetedContentInteraction) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), interaction).ok() }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn ReportCustomInteraction<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, custominteractionname: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), custominteractionname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, TargetedContentValue>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, TargetedContentValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation_Collections`*"]
    pub fn Collections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentCollection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<TargetedContentCollection>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation_Collections`*"]
    pub fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<TargetedContentItem>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentCollection;{2d4b66c5-f163-44ba-9f6e-e1a4c2bb559d})");
}
unsafe impl ::windows::core::Interface for TargetedContentCollection {
    type Vtable = ITargetedContentCollection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d4b66c5_f163_44ba_9f6e_e1a4c2bb559d);
}
impl ::windows::core::RuntimeName for TargetedContentCollection {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentCollection";
}
impl ::core::convert::From<TargetedContentCollection> for ::windows::core::IUnknown {
    fn from(value: TargetedContentCollection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TargetedContentCollection> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentCollection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TargetedContentCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TargetedContentCollection> for ::windows::core::IInspectable {
    fn from(value: TargetedContentCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TargetedContentCollection> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TargetedContentCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TargetedContentCollection {}
unsafe impl ::core::marker::Sync for TargetedContentCollection {}
#[doc = "*Required features: `Services_TargetedContent`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TargetedContentContainer(pub ::windows::core::IInspectable);
impl TargetedContentContainer {
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn Availability(&self) -> ::windows::core::Result<TargetedContentAvailability> {
        let this = self;
        unsafe {
            let mut result__: TargetedContentAvailability = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentAvailability>(result__)
        }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn Content(&self) -> ::windows::core::Result<TargetedContentCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentCollection>(result__)
        }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn SelectSingleObject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, path: Param0) -> ::windows::core::Result<TargetedContentObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), path.into_param().abi(), &mut result__).from_abi::<TargetedContentObject>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation`*"]
    pub fn GetAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(contentid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TargetedContentContainer>> {
        Self::ITargetedContentContainerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), contentid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<TargetedContentContainer>>(result__)
        })
    }
    pub fn ITargetedContentContainerStatics<R, F: FnOnce(&ITargetedContentContainerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TargetedContentContainer, ITargetedContentContainerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentContainer;{bc2494c9-8837-47c2-850f-d79d64595926})");
}
unsafe impl ::windows::core::Interface for TargetedContentContainer {
    type Vtable = ITargetedContentContainer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc2494c9_8837_47c2_850f_d79d64595926);
}
impl ::windows::core::RuntimeName for TargetedContentContainer {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentContainer";
}
impl ::core::convert::From<TargetedContentContainer> for ::windows::core::IUnknown {
    fn from(value: TargetedContentContainer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TargetedContentContainer> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentContainer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TargetedContentContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TargetedContentContainer> for ::windows::core::IInspectable {
    fn from(value: TargetedContentContainer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TargetedContentContainer> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TargetedContentContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TargetedContentContainer {}
unsafe impl ::core::marker::Sync for TargetedContentContainer {}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct TargetedContentContract(pub u8);
#[cfg(feature = "Storage_Streams")]
#[doc = "*Required features: `Services_TargetedContent`, `Storage_Streams`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TargetedContentFile(pub ::windows::core::IInspectable);
#[cfg(feature = "Storage_Streams")]
impl TargetedContentFile {
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation`, `Storage_Streams`*"]
    pub fn OpenReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>>(result__)
        }
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::RuntimeType for TargetedContentFile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentFile;{33ee3134-1dd6-4e3a-8067-d1c162e8642b})");
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::Interface for TargetedContentFile {
    type Vtable = super::super::Storage::Streams::IRandomAccessStreamReference_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33ee3134_1dd6_4e3a_8067_d1c162e8642b);
}
#[cfg(feature = "Storage_Streams")]
impl ::windows::core::RuntimeName for TargetedContentFile {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentFile";
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<TargetedContentFile> for ::windows::core::IUnknown {
    fn from(value: TargetedContentFile) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&TargetedContentFile> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentFile) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TargetedContentFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<TargetedContentFile> for ::windows::core::IInspectable {
    fn from(value: TargetedContentFile) -> Self {
        value.0
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&TargetedContentFile> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentFile) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TargetedContentFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<TargetedContentFile> for super::super::Storage::Streams::IRandomAccessStreamReference {
    fn from(value: TargetedContentFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&TargetedContentFile> for super::super::Storage::Streams::IRandomAccessStreamReference {
    fn from(value: &TargetedContentFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference> for TargetedContentFile {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStreamReference> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference> for &TargetedContentFile {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStreamReference> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::core::marker::Send for TargetedContentFile {}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::core::marker::Sync for TargetedContentFile {}
#[doc = "*Required features: `Services_TargetedContent`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TargetedContentImage(pub ::windows::core::IInspectable);
impl TargetedContentImage {
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation`, `Storage_Streams`*"]
    pub fn OpenReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStreamReference>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentImage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentImage;{a7a585d9-779f-4b1e-bbb1-8eaf53fbeab2})");
}
unsafe impl ::windows::core::Interface for TargetedContentImage {
    type Vtable = ITargetedContentImage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7a585d9_779f_4b1e_bbb1_8eaf53fbeab2);
}
impl ::windows::core::RuntimeName for TargetedContentImage {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentImage";
}
impl ::core::convert::From<TargetedContentImage> for ::windows::core::IUnknown {
    fn from(value: TargetedContentImage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TargetedContentImage> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentImage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentImage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TargetedContentImage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TargetedContentImage> for ::windows::core::IInspectable {
    fn from(value: TargetedContentImage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TargetedContentImage> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentImage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentImage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TargetedContentImage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[doc = "*Required features: `Services_TargetedContent`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TargetedContentInteraction(pub i32);
impl TargetedContentInteraction {
    pub const Impression: TargetedContentInteraction = TargetedContentInteraction(0i32);
    pub const ClickThrough: TargetedContentInteraction = TargetedContentInteraction(1i32);
    pub const Hover: TargetedContentInteraction = TargetedContentInteraction(2i32);
    pub const Like: TargetedContentInteraction = TargetedContentInteraction(3i32);
    pub const Dislike: TargetedContentInteraction = TargetedContentInteraction(4i32);
    pub const Dismiss: TargetedContentInteraction = TargetedContentInteraction(5i32);
    pub const Ineligible: TargetedContentInteraction = TargetedContentInteraction(6i32);
    pub const Accept: TargetedContentInteraction = TargetedContentInteraction(7i32);
    pub const Decline: TargetedContentInteraction = TargetedContentInteraction(8i32);
    pub const Defer: TargetedContentInteraction = TargetedContentInteraction(9i32);
    pub const Canceled: TargetedContentInteraction = TargetedContentInteraction(10i32);
    pub const Conversion: TargetedContentInteraction = TargetedContentInteraction(11i32);
    pub const Opportunity: TargetedContentInteraction = TargetedContentInteraction(12i32);
}
impl ::core::convert::From<i32> for TargetedContentInteraction {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TargetedContentInteraction {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TargetedContentInteraction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentInteraction;i4)");
}
impl ::windows::core::DefaultType for TargetedContentInteraction {
    type DefaultType = Self;
}
#[doc = "*Required features: `Services_TargetedContent`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TargetedContentItem(pub ::windows::core::IInspectable);
impl TargetedContentItem {
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn ReportInteraction(&self, interaction: TargetedContentInteraction) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), interaction).ok() }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn ReportCustomInteraction<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, custominteractionname: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), custominteractionname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn State(&self) -> ::windows::core::Result<TargetedContentItemState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentItemState>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, TargetedContentValue>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, TargetedContentValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation_Collections`*"]
    pub fn Collections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentCollection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<TargetedContentCollection>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentItem;{38168dc4-276c-4c32-96ba-565c6e406e74})");
}
unsafe impl ::windows::core::Interface for TargetedContentItem {
    type Vtable = ITargetedContentItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38168dc4_276c_4c32_96ba_565c6e406e74);
}
impl ::windows::core::RuntimeName for TargetedContentItem {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentItem";
}
impl ::core::convert::From<TargetedContentItem> for ::windows::core::IUnknown {
    fn from(value: TargetedContentItem) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TargetedContentItem> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentItem) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TargetedContentItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TargetedContentItem> for ::windows::core::IInspectable {
    fn from(value: TargetedContentItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TargetedContentItem> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TargetedContentItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TargetedContentItem {}
unsafe impl ::core::marker::Sync for TargetedContentItem {}
#[doc = "*Required features: `Services_TargetedContent`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TargetedContentItemState(pub ::windows::core::IInspectable);
impl TargetedContentItemState {
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn ShouldDisplay(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn AppInstallationState(&self) -> ::windows::core::Result<TargetedContentAppInstallationState> {
        let this = self;
        unsafe {
            let mut result__: TargetedContentAppInstallationState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentAppInstallationState>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentItemState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentItemState;{73935454-4c65-4b47-a441-472de53c79b6})");
}
unsafe impl ::windows::core::Interface for TargetedContentItemState {
    type Vtable = ITargetedContentItemState_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73935454_4c65_4b47_a441_472de53c79b6);
}
impl ::windows::core::RuntimeName for TargetedContentItemState {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentItemState";
}
impl ::core::convert::From<TargetedContentItemState> for ::windows::core::IUnknown {
    fn from(value: TargetedContentItemState) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TargetedContentItemState> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentItemState) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentItemState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TargetedContentItemState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TargetedContentItemState> for ::windows::core::IInspectable {
    fn from(value: TargetedContentItemState) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TargetedContentItemState> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentItemState) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentItemState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TargetedContentItemState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TargetedContentItemState {}
unsafe impl ::core::marker::Sync for TargetedContentItemState {}
#[doc = "*Required features: `Services_TargetedContent`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TargetedContentObject(pub ::windows::core::IInspectable);
impl TargetedContentObject {
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn ObjectKind(&self) -> ::windows::core::Result<TargetedContentObjectKind> {
        let this = self;
        unsafe {
            let mut result__: TargetedContentObjectKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentObjectKind>(result__)
        }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn Collection(&self) -> ::windows::core::Result<TargetedContentCollection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentCollection>(result__)
        }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn Item(&self) -> ::windows::core::Result<TargetedContentItem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentItem>(result__)
        }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn Value(&self) -> ::windows::core::Result<TargetedContentValue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentValue>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentObject {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentObject;{041d7969-2212-42d1-9dfa-88a8e3033aa3})");
}
unsafe impl ::windows::core::Interface for TargetedContentObject {
    type Vtable = ITargetedContentObject_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x041d7969_2212_42d1_9dfa_88a8e3033aa3);
}
impl ::windows::core::RuntimeName for TargetedContentObject {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentObject";
}
impl ::core::convert::From<TargetedContentObject> for ::windows::core::IUnknown {
    fn from(value: TargetedContentObject) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TargetedContentObject> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentObject) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TargetedContentObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TargetedContentObject> for ::windows::core::IInspectable {
    fn from(value: TargetedContentObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TargetedContentObject> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TargetedContentObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TargetedContentObject {}
unsafe impl ::core::marker::Sync for TargetedContentObject {}
#[doc = "*Required features: `Services_TargetedContent`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TargetedContentObjectKind(pub i32);
impl TargetedContentObjectKind {
    pub const Collection: TargetedContentObjectKind = TargetedContentObjectKind(0i32);
    pub const Item: TargetedContentObjectKind = TargetedContentObjectKind(1i32);
    pub const Value: TargetedContentObjectKind = TargetedContentObjectKind(2i32);
}
impl ::core::convert::From<i32> for TargetedContentObjectKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TargetedContentObjectKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TargetedContentObjectKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentObjectKind;i4)");
}
impl ::windows::core::DefaultType for TargetedContentObjectKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Services_TargetedContent`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TargetedContentStateChangedEventArgs(pub ::windows::core::IInspectable);
impl TargetedContentStateChangedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentStateChangedEventArgs;{9a1cef3d-8073-4416-8df2-546835a6414f})");
}
unsafe impl ::windows::core::Interface for TargetedContentStateChangedEventArgs {
    type Vtable = ITargetedContentStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a1cef3d_8073_4416_8df2_546835a6414f);
}
impl ::windows::core::RuntimeName for TargetedContentStateChangedEventArgs {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentStateChangedEventArgs";
}
impl ::core::convert::From<TargetedContentStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: TargetedContentStateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TargetedContentStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentStateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TargetedContentStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TargetedContentStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: TargetedContentStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TargetedContentStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TargetedContentStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TargetedContentStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for TargetedContentStateChangedEventArgs {}
#[doc = "*Required features: `Services_TargetedContent`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TargetedContentSubscription(pub ::windows::core::IInspectable);
impl TargetedContentSubscription {
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation`*"]
    pub fn GetContentContainerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TargetedContentContainer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<TargetedContentContainer>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation`*"]
    pub fn ContentChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation`*"]
    pub fn RemoveContentChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation`*"]
    pub fn AvailabilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentAvailabilityChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation`*"]
    pub fn RemoveAvailabilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation`*"]
    pub fn StateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation`*"]
    pub fn RemoveStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation`*"]
    pub fn GetAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(subscriptionid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TargetedContentSubscription>> {
        Self::ITargetedContentSubscriptionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), subscriptionid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<TargetedContentSubscription>>(result__)
        })
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn GetOptions<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(subscriptionid: Param0) -> ::windows::core::Result<TargetedContentSubscriptionOptions> {
        Self::ITargetedContentSubscriptionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), subscriptionid.into_param().abi(), &mut result__).from_abi::<TargetedContentSubscriptionOptions>(result__)
        })
    }
    pub fn ITargetedContentSubscriptionStatics<R, F: FnOnce(&ITargetedContentSubscriptionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<TargetedContentSubscription, ITargetedContentSubscriptionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentSubscription {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentSubscription;{882c2c49-c652-4c7a-acad-1f7fa2986c73})");
}
unsafe impl ::windows::core::Interface for TargetedContentSubscription {
    type Vtable = ITargetedContentSubscription_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x882c2c49_c652_4c7a_acad_1f7fa2986c73);
}
impl ::windows::core::RuntimeName for TargetedContentSubscription {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentSubscription";
}
impl ::core::convert::From<TargetedContentSubscription> for ::windows::core::IUnknown {
    fn from(value: TargetedContentSubscription) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TargetedContentSubscription> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentSubscription) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentSubscription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TargetedContentSubscription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TargetedContentSubscription> for ::windows::core::IInspectable {
    fn from(value: TargetedContentSubscription) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TargetedContentSubscription> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentSubscription) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentSubscription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TargetedContentSubscription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TargetedContentSubscription {}
unsafe impl ::core::marker::Sync for TargetedContentSubscription {}
#[doc = "*Required features: `Services_TargetedContent`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TargetedContentSubscriptionOptions(pub ::windows::core::IInspectable);
impl TargetedContentSubscriptionOptions {
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn SubscriptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn AllowPartialContentAvailability(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn SetAllowPartialContentAvailability(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation_Collections`*"]
    pub fn CloudQueryParameters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation_Collections`*"]
    pub fn LocalFilters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn Update(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentSubscriptionOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentSubscriptionOptions;{61ee6ad0-2c83-421b-8467-413eaf1aeb97})");
}
unsafe impl ::windows::core::Interface for TargetedContentSubscriptionOptions {
    type Vtable = ITargetedContentSubscriptionOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61ee6ad0_2c83_421b_8467_413eaf1aeb97);
}
impl ::windows::core::RuntimeName for TargetedContentSubscriptionOptions {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentSubscriptionOptions";
}
impl ::core::convert::From<TargetedContentSubscriptionOptions> for ::windows::core::IUnknown {
    fn from(value: TargetedContentSubscriptionOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TargetedContentSubscriptionOptions> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentSubscriptionOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentSubscriptionOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TargetedContentSubscriptionOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TargetedContentSubscriptionOptions> for ::windows::core::IInspectable {
    fn from(value: TargetedContentSubscriptionOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TargetedContentSubscriptionOptions> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentSubscriptionOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentSubscriptionOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TargetedContentSubscriptionOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TargetedContentSubscriptionOptions {}
unsafe impl ::core::marker::Sync for TargetedContentSubscriptionOptions {}
#[doc = "*Required features: `Services_TargetedContent`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TargetedContentValue(pub ::windows::core::IInspectable);
impl TargetedContentValue {
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn ValueKind(&self) -> ::windows::core::Result<TargetedContentValueKind> {
        let this = self;
        unsafe {
            let mut result__: TargetedContentValueKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentValueKind>(result__)
        }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn String(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn Number(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn Boolean(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Services_TargetedContent`, `Storage_Streams`*"]
    pub fn File(&self) -> ::windows::core::Result<TargetedContentFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentFile>(result__)
        }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn ImageFile(&self) -> ::windows::core::Result<TargetedContentImage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentImage>(result__)
        }
    }
    #[doc = "*Required features: `Services_TargetedContent`*"]
    pub fn Action(&self) -> ::windows::core::Result<TargetedContentAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TargetedContentAction>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation_Collections`*"]
    pub fn Strings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation`, `Foundation_Collections`*"]
    pub fn Uris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation_Collections`*"]
    pub fn Numbers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation_Collections`*"]
    pub fn Booleans(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<TargetedContentFile>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation_Collections`*"]
    pub fn ImageFiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentImage>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<TargetedContentImage>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_TargetedContent`, `Foundation_Collections`*"]
    pub fn Actions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TargetedContentAction>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<TargetedContentAction>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TargetedContentValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.TargetedContent.TargetedContentValue;{aafde4b3-4215-4bf8-867f-43f04865f9bf})");
}
unsafe impl ::windows::core::Interface for TargetedContentValue {
    type Vtable = ITargetedContentValue_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaafde4b3_4215_4bf8_867f_43f04865f9bf);
}
impl ::windows::core::RuntimeName for TargetedContentValue {
    const NAME: &'static str = "Windows.Services.TargetedContent.TargetedContentValue";
}
impl ::core::convert::From<TargetedContentValue> for ::windows::core::IUnknown {
    fn from(value: TargetedContentValue) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TargetedContentValue> for ::windows::core::IUnknown {
    fn from(value: &TargetedContentValue) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TargetedContentValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TargetedContentValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TargetedContentValue> for ::windows::core::IInspectable {
    fn from(value: TargetedContentValue) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TargetedContentValue> for ::windows::core::IInspectable {
    fn from(value: &TargetedContentValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TargetedContentValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TargetedContentValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TargetedContentValue {}
unsafe impl ::core::marker::Sync for TargetedContentValue {}
#[doc = "*Required features: `Services_TargetedContent`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TargetedContentValueKind(pub i32);
impl TargetedContentValueKind {
    pub const String: TargetedContentValueKind = TargetedContentValueKind(0i32);
    pub const Uri: TargetedContentValueKind = TargetedContentValueKind(1i32);
    pub const Number: TargetedContentValueKind = TargetedContentValueKind(2i32);
    pub const Boolean: TargetedContentValueKind = TargetedContentValueKind(3i32);
    pub const File: TargetedContentValueKind = TargetedContentValueKind(4i32);
    pub const ImageFile: TargetedContentValueKind = TargetedContentValueKind(5i32);
    pub const Action: TargetedContentValueKind = TargetedContentValueKind(6i32);
    pub const Strings: TargetedContentValueKind = TargetedContentValueKind(7i32);
    pub const Uris: TargetedContentValueKind = TargetedContentValueKind(8i32);
    pub const Numbers: TargetedContentValueKind = TargetedContentValueKind(9i32);
    pub const Booleans: TargetedContentValueKind = TargetedContentValueKind(10i32);
    pub const Files: TargetedContentValueKind = TargetedContentValueKind(11i32);
    pub const ImageFiles: TargetedContentValueKind = TargetedContentValueKind(12i32);
    pub const Actions: TargetedContentValueKind = TargetedContentValueKind(13i32);
}
impl ::core::convert::From<i32> for TargetedContentValueKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TargetedContentValueKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TargetedContentValueKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.TargetedContent.TargetedContentValueKind;i4)");
}
impl ::windows::core::DefaultType for TargetedContentValueKind {
    type DefaultType = Self;
}
