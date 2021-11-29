#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Graphics_Printing_OptionDetails")]
pub mod OptionDetails;
#[cfg(feature = "Graphics_Printing_PrintSupport")]
pub mod PrintSupport;
#[cfg(feature = "Graphics_Printing_PrintTicket")]
pub mod PrintTicket;
#[cfg(feature = "Graphics_Printing_Workflow")]
pub mod Workflow;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPrintDocumentSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintDocumentSource {
    type Vtable = IPrintDocumentSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdedc0c30_f1eb_47df_aae6_ed5427511f01);
}
impl IPrintDocumentSource {}
unsafe impl ::windows::core::RuntimeType for IPrintDocumentSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{dedc0c30-f1eb-47df-aae6-ed5427511f01}");
}
impl ::core::convert::From<IPrintDocumentSource> for ::windows::core::IUnknown {
    fn from(value: IPrintDocumentSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPrintDocumentSource> for ::windows::core::IUnknown {
    fn from(value: &IPrintDocumentSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrintDocumentSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrintDocumentSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPrintDocumentSource> for ::windows::core::IInspectable {
    fn from(value: IPrintDocumentSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintDocumentSource> for ::windows::core::IInspectable {
    fn from(value: &IPrintDocumentSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPrintDocumentSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPrintDocumentSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentSource_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintManager {
    type Vtable = IPrintManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff2a9694_8c99_44fd_ae4a_19d9aa9a0f0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintManagerStatic(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintManagerStatic {
    type Vtable = IPrintManagerStatic_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58185dcd_e634_4654_84f0_e0152a8217ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintManagerStatic_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintManagerStatic2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintManagerStatic2 {
    type Vtable = IPrintManagerStatic2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35a99955_e6ab_4139_9abd_b86a729b3598);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintManagerStatic2_abi(
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
pub struct IPrintPageInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintPageInfo {
    type Vtable = IPrintPageInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd4be9c9_a6a1_4ada_930e_da872a4f23d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPageInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PrintMediaSize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PrintMediaSize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PrintOrientation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PrintOrientation) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintPageRange(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintPageRange {
    type Vtable = IPrintPageRange_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8a06c54_6e7c_51c5_57fd_0660c2d71513);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPageRange_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintPageRangeFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintPageRangeFactory {
    type Vtable = IPrintPageRangeFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x408fd45f_e047_5f85_7129_fb085a4fad14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPageRangeFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, firstpage: i32, lastpage: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, page: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintPageRangeOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintPageRangeOptions {
    type Vtable = IPrintPageRangeOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce6db728_1357_46b2_a923_79f995f448fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPageRangeOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTask(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintTask {
    type Vtable = IPrintTask_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61d80247_6cf6_4fad_84e2_a5e82e2d4ceb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTask_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_DataTransfer")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTask2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintTask2 {
    type Vtable = IPrintTask2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36234877_3e53_4d9d_8f5e_316ac8dedae1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTask2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskCompletedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintTaskCompletedEventArgs {
    type Vtable = IPrintTaskCompletedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bcd34af_24e9_4c10_8d07_14c346ba3fce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PrintTaskCompletion) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintTaskOptions {
    type Vtable = IPrintTaskOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a0a66bb_d289_41bb_96dd_57e28338ae3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PrintBordering) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PrintBordering) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, printpageinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskOptions2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintTaskOptions2 {
    type Vtable = IPrintTaskOptions2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb9b1606_9a36_4b59_8617_b217849262e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptions2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPrintTaskOptionsCore(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintTaskOptionsCore {
    type Vtable = IPrintTaskOptionsCore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bdbb474_4ed1_41eb_be3c_72d18ed67337);
}
impl IPrintTaskOptionsCore {
    #[cfg(feature = "Foundation")]
    pub fn GetPageDescription(&self, jobpagenumber: u32) -> ::windows::core::Result<PrintPageDescription> {
        let this = self;
        unsafe {
            let mut result__: PrintPageDescription = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), jobpagenumber, &mut result__).from_abi::<PrintPageDescription>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IPrintTaskOptionsCore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1bdbb474-4ed1-41eb-be3c-72d18ed67337}");
}
impl ::core::convert::From<IPrintTaskOptionsCore> for ::windows::core::IUnknown {
    fn from(value: IPrintTaskOptionsCore) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPrintTaskOptionsCore> for ::windows::core::IUnknown {
    fn from(value: &IPrintTaskOptionsCore) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrintTaskOptionsCore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrintTaskOptionsCore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPrintTaskOptionsCore> for ::windows::core::IInspectable {
    fn from(value: IPrintTaskOptionsCore) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintTaskOptionsCore> for ::windows::core::IInspectable {
    fn from(value: &IPrintTaskOptionsCore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPrintTaskOptionsCore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPrintTaskOptionsCore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptionsCore_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, jobpagenumber: u32, result__: *mut PrintPageDescription) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPrintTaskOptionsCoreProperties(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintTaskOptionsCoreProperties {
    type Vtable = IPrintTaskOptionsCoreProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1b71832_9e93_4e55_814b_3326a59efce1);
}
impl IPrintTaskOptionsCoreProperties {
    pub fn SetMediaSize(&self, value: PrintMediaSize) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MediaSize(&self) -> ::windows::core::Result<PrintMediaSize> {
        let this = self;
        unsafe {
            let mut result__: PrintMediaSize = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintMediaSize>(result__)
        }
    }
    pub fn SetMediaType(&self, value: PrintMediaType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MediaType(&self) -> ::windows::core::Result<PrintMediaType> {
        let this = self;
        unsafe {
            let mut result__: PrintMediaType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintMediaType>(result__)
        }
    }
    pub fn SetOrientation(&self, value: PrintOrientation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Orientation(&self) -> ::windows::core::Result<PrintOrientation> {
        let this = self;
        unsafe {
            let mut result__: PrintOrientation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOrientation>(result__)
        }
    }
    pub fn SetPrintQuality(&self, value: PrintQuality) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PrintQuality(&self) -> ::windows::core::Result<PrintQuality> {
        let this = self;
        unsafe {
            let mut result__: PrintQuality = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintQuality>(result__)
        }
    }
    pub fn SetColorMode(&self, value: PrintColorMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ColorMode(&self) -> ::windows::core::Result<PrintColorMode> {
        let this = self;
        unsafe {
            let mut result__: PrintColorMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintColorMode>(result__)
        }
    }
    pub fn SetDuplex(&self, value: PrintDuplex) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Duplex(&self) -> ::windows::core::Result<PrintDuplex> {
        let this = self;
        unsafe {
            let mut result__: PrintDuplex = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintDuplex>(result__)
        }
    }
    pub fn SetCollation(&self, value: PrintCollation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Collation(&self) -> ::windows::core::Result<PrintCollation> {
        let this = self;
        unsafe {
            let mut result__: PrintCollation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintCollation>(result__)
        }
    }
    pub fn SetStaple(&self, value: PrintStaple) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Staple(&self) -> ::windows::core::Result<PrintStaple> {
        let this = self;
        unsafe {
            let mut result__: PrintStaple = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintStaple>(result__)
        }
    }
    pub fn SetHolePunch(&self, value: PrintHolePunch) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn HolePunch(&self) -> ::windows::core::Result<PrintHolePunch> {
        let this = self;
        unsafe {
            let mut result__: PrintHolePunch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintHolePunch>(result__)
        }
    }
    pub fn SetBinding(&self, value: PrintBinding) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Binding(&self) -> ::windows::core::Result<PrintBinding> {
        let this = self;
        unsafe {
            let mut result__: PrintBinding = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintBinding>(result__)
        }
    }
    pub fn MinCopies(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn MaxCopies(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetNumberOfCopies(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn NumberOfCopies(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IPrintTaskOptionsCoreProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c1b71832-9e93-4e55-814b-3326a59efce1}");
}
impl ::core::convert::From<IPrintTaskOptionsCoreProperties> for ::windows::core::IUnknown {
    fn from(value: IPrintTaskOptionsCoreProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPrintTaskOptionsCoreProperties> for ::windows::core::IUnknown {
    fn from(value: &IPrintTaskOptionsCoreProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrintTaskOptionsCoreProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrintTaskOptionsCoreProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPrintTaskOptionsCoreProperties> for ::windows::core::IInspectable {
    fn from(value: IPrintTaskOptionsCoreProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintTaskOptionsCoreProperties> for ::windows::core::IInspectable {
    fn from(value: &IPrintTaskOptionsCoreProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPrintTaskOptionsCoreProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPrintTaskOptionsCoreProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptionsCoreProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PrintMediaSize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PrintMediaSize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PrintMediaType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PrintMediaType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PrintOrientation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PrintOrientation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PrintQuality) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PrintQuality) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PrintColorMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PrintColorMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PrintDuplex) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PrintDuplex) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PrintCollation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PrintCollation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PrintStaple) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PrintStaple) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PrintHolePunch) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PrintHolePunch) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PrintBinding) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PrintBinding) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPrintTaskOptionsCoreUIConfiguration(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintTaskOptionsCoreUIConfiguration {
    type Vtable = IPrintTaskOptionsCoreUIConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62e69e23_9a1e_4336_b74f_3cc7f4cff709);
}
impl IPrintTaskOptionsCoreUIConfiguration {
    #[cfg(feature = "Foundation_Collections")]
    pub fn DisplayedOptions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IPrintTaskOptionsCoreUIConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{62e69e23-9a1e-4336-b74f-3cc7f4cff709}");
}
impl ::core::convert::From<IPrintTaskOptionsCoreUIConfiguration> for ::windows::core::IUnknown {
    fn from(value: IPrintTaskOptionsCoreUIConfiguration) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPrintTaskOptionsCoreUIConfiguration> for ::windows::core::IUnknown {
    fn from(value: &IPrintTaskOptionsCoreUIConfiguration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrintTaskOptionsCoreUIConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrintTaskOptionsCoreUIConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPrintTaskOptionsCoreUIConfiguration> for ::windows::core::IInspectable {
    fn from(value: IPrintTaskOptionsCoreUIConfiguration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintTaskOptionsCoreUIConfiguration> for ::windows::core::IInspectable {
    fn from(value: &IPrintTaskOptionsCoreUIConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPrintTaskOptionsCoreUIConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPrintTaskOptionsCoreUIConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptionsCoreUIConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskProgressingEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintTaskProgressingEventArgs {
    type Vtable = IPrintTaskProgressingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x810cd3cb_b410_4282_a073_5ac378234174);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskProgressingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintTaskRequest {
    type Vtable = IPrintTaskRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ff61e2e_2722_4240_a67c_f364849a17f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskRequestedDeferral(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintTaskRequestedDeferral {
    type Vtable = IPrintTaskRequestedDeferral_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfefb3f0_ce3e_42c7_9496_64800c622c44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskRequestedDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintTaskRequestedEventArgs {
    type Vtable = IPrintTaskRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0aff924_a31b_454c_a7b6_5d0cc522fc16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskRequestedEventArgs_abi(
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
pub struct IPrintTaskSourceRequestedArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintTaskSourceRequestedArgs {
    type Vtable = IPrintTaskSourceRequestedArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9f067be_f456_41f0_9c98_5ce73e851410);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskSourceRequestedArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskSourceRequestedDeferral(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintTaskSourceRequestedDeferral {
    type Vtable = IPrintTaskSourceRequestedDeferral_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a1560d1_6992_4d9d_8555_4ca4563fb166);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskSourceRequestedDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskTargetDeviceSupport(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintTaskTargetDeviceSupport {
    type Vtable = IPrintTaskTargetDeviceSupport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x295d70c0_c2cb_4b7d_b0ea_93095091a220);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskTargetDeviceSupport_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStandardPrintTaskOptionsStatic(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStandardPrintTaskOptionsStatic {
    type Vtable = IStandardPrintTaskOptionsStatic_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4483d26_0dd0_4cd4_baff_930fc7d6a574);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardPrintTaskOptionsStatic_abi(
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
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStandardPrintTaskOptionsStatic2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStandardPrintTaskOptionsStatic2 {
    type Vtable = IStandardPrintTaskOptionsStatic2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3be38bf4_7a44_4269_9a52_81261e289ee9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardPrintTaskOptionsStatic2_abi(
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
pub struct IStandardPrintTaskOptionsStatic3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStandardPrintTaskOptionsStatic3 {
    type Vtable = IStandardPrintTaskOptionsStatic3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbf68e86_3858_41b3_a799_55dd9888d475);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardPrintTaskOptionsStatic3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PrintBinding(pub i32);
impl PrintBinding {
    pub const Default: PrintBinding = PrintBinding(0i32);
    pub const NotAvailable: PrintBinding = PrintBinding(1i32);
    pub const PrinterCustom: PrintBinding = PrintBinding(2i32);
    pub const None: PrintBinding = PrintBinding(3i32);
    pub const Bale: PrintBinding = PrintBinding(4i32);
    pub const BindBottom: PrintBinding = PrintBinding(5i32);
    pub const BindLeft: PrintBinding = PrintBinding(6i32);
    pub const BindRight: PrintBinding = PrintBinding(7i32);
    pub const BindTop: PrintBinding = PrintBinding(8i32);
    pub const Booklet: PrintBinding = PrintBinding(9i32);
    pub const EdgeStitchBottom: PrintBinding = PrintBinding(10i32);
    pub const EdgeStitchLeft: PrintBinding = PrintBinding(11i32);
    pub const EdgeStitchRight: PrintBinding = PrintBinding(12i32);
    pub const EdgeStitchTop: PrintBinding = PrintBinding(13i32);
    pub const Fold: PrintBinding = PrintBinding(14i32);
    pub const JogOffset: PrintBinding = PrintBinding(15i32);
    pub const Trim: PrintBinding = PrintBinding(16i32);
}
impl ::core::convert::From<i32> for PrintBinding {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PrintBinding {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PrintBinding {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintBinding;i4)");
}
impl ::windows::core::DefaultType for PrintBinding {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PrintBordering(pub i32);
impl PrintBordering {
    pub const Default: PrintBordering = PrintBordering(0i32);
    pub const NotAvailable: PrintBordering = PrintBordering(1i32);
    pub const PrinterCustom: PrintBordering = PrintBordering(2i32);
    pub const Bordered: PrintBordering = PrintBordering(3i32);
    pub const Borderless: PrintBordering = PrintBordering(4i32);
}
impl ::core::convert::From<i32> for PrintBordering {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PrintBordering {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PrintBordering {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintBordering;i4)");
}
impl ::windows::core::DefaultType for PrintBordering {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PrintCollation(pub i32);
impl PrintCollation {
    pub const Default: PrintCollation = PrintCollation(0i32);
    pub const NotAvailable: PrintCollation = PrintCollation(1i32);
    pub const PrinterCustom: PrintCollation = PrintCollation(2i32);
    pub const Collated: PrintCollation = PrintCollation(3i32);
    pub const Uncollated: PrintCollation = PrintCollation(4i32);
}
impl ::core::convert::From<i32> for PrintCollation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PrintCollation {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PrintCollation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintCollation;i4)");
}
impl ::windows::core::DefaultType for PrintCollation {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PrintColorMode(pub i32);
impl PrintColorMode {
    pub const Default: PrintColorMode = PrintColorMode(0i32);
    pub const NotAvailable: PrintColorMode = PrintColorMode(1i32);
    pub const PrinterCustom: PrintColorMode = PrintColorMode(2i32);
    pub const Color: PrintColorMode = PrintColorMode(3i32);
    pub const Grayscale: PrintColorMode = PrintColorMode(4i32);
    pub const Monochrome: PrintColorMode = PrintColorMode(5i32);
}
impl ::core::convert::From<i32> for PrintColorMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PrintColorMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PrintColorMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintColorMode;i4)");
}
impl ::windows::core::DefaultType for PrintColorMode {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PrintDuplex(pub i32);
impl PrintDuplex {
    pub const Default: PrintDuplex = PrintDuplex(0i32);
    pub const NotAvailable: PrintDuplex = PrintDuplex(1i32);
    pub const PrinterCustom: PrintDuplex = PrintDuplex(2i32);
    pub const OneSided: PrintDuplex = PrintDuplex(3i32);
    pub const TwoSidedShortEdge: PrintDuplex = PrintDuplex(4i32);
    pub const TwoSidedLongEdge: PrintDuplex = PrintDuplex(5i32);
}
impl ::core::convert::From<i32> for PrintDuplex {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PrintDuplex {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PrintDuplex {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintDuplex;i4)");
}
impl ::windows::core::DefaultType for PrintDuplex {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PrintHolePunch(pub i32);
impl PrintHolePunch {
    pub const Default: PrintHolePunch = PrintHolePunch(0i32);
    pub const NotAvailable: PrintHolePunch = PrintHolePunch(1i32);
    pub const PrinterCustom: PrintHolePunch = PrintHolePunch(2i32);
    pub const None: PrintHolePunch = PrintHolePunch(3i32);
    pub const LeftEdge: PrintHolePunch = PrintHolePunch(4i32);
    pub const RightEdge: PrintHolePunch = PrintHolePunch(5i32);
    pub const TopEdge: PrintHolePunch = PrintHolePunch(6i32);
    pub const BottomEdge: PrintHolePunch = PrintHolePunch(7i32);
}
impl ::core::convert::From<i32> for PrintHolePunch {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PrintHolePunch {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PrintHolePunch {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintHolePunch;i4)");
}
impl ::windows::core::DefaultType for PrintHolePunch {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintManager(pub ::windows::core::IInspectable);
impl PrintManager {
    #[cfg(feature = "Foundation")]
    pub fn PrintTaskRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PrintManager, PrintTaskRequestedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePrintTaskRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<PrintManager> {
        Self::IPrintManagerStatic(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintManager>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn ShowPrintUIAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IPrintManagerStatic(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IPrintManagerStatic2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IPrintManagerStatic<R, F: FnOnce(&IPrintManagerStatic) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PrintManager, IPrintManagerStatic> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPrintManagerStatic2<R, F: FnOnce(&IPrintManagerStatic2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PrintManager, IPrintManagerStatic2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintManager;{ff2a9694-8c99-44fd-ae4a-19d9aa9a0f0a})");
}
unsafe impl ::windows::core::Interface for PrintManager {
    type Vtable = IPrintManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff2a9694_8c99_44fd_ae4a_19d9aa9a0f0a);
}
impl ::windows::core::RuntimeName for PrintManager {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintManager";
}
impl ::core::convert::From<PrintManager> for ::windows::core::IUnknown {
    fn from(value: PrintManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintManager> for ::windows::core::IUnknown {
    fn from(value: &PrintManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintManager> for ::windows::core::IInspectable {
    fn from(value: PrintManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintManager> for ::windows::core::IInspectable {
    fn from(value: &PrintManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintManager {}
unsafe impl ::core::marker::Sync for PrintManager {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PrintMediaSize(pub i32);
impl PrintMediaSize {
    pub const Default: PrintMediaSize = PrintMediaSize(0i32);
    pub const NotAvailable: PrintMediaSize = PrintMediaSize(1i32);
    pub const PrinterCustom: PrintMediaSize = PrintMediaSize(2i32);
    pub const BusinessCard: PrintMediaSize = PrintMediaSize(3i32);
    pub const CreditCard: PrintMediaSize = PrintMediaSize(4i32);
    pub const IsoA0: PrintMediaSize = PrintMediaSize(5i32);
    pub const IsoA1: PrintMediaSize = PrintMediaSize(6i32);
    pub const IsoA10: PrintMediaSize = PrintMediaSize(7i32);
    pub const IsoA2: PrintMediaSize = PrintMediaSize(8i32);
    pub const IsoA3: PrintMediaSize = PrintMediaSize(9i32);
    pub const IsoA3Extra: PrintMediaSize = PrintMediaSize(10i32);
    pub const IsoA3Rotated: PrintMediaSize = PrintMediaSize(11i32);
    pub const IsoA4: PrintMediaSize = PrintMediaSize(12i32);
    pub const IsoA4Extra: PrintMediaSize = PrintMediaSize(13i32);
    pub const IsoA4Rotated: PrintMediaSize = PrintMediaSize(14i32);
    pub const IsoA5: PrintMediaSize = PrintMediaSize(15i32);
    pub const IsoA5Extra: PrintMediaSize = PrintMediaSize(16i32);
    pub const IsoA5Rotated: PrintMediaSize = PrintMediaSize(17i32);
    pub const IsoA6: PrintMediaSize = PrintMediaSize(18i32);
    pub const IsoA6Rotated: PrintMediaSize = PrintMediaSize(19i32);
    pub const IsoA7: PrintMediaSize = PrintMediaSize(20i32);
    pub const IsoA8: PrintMediaSize = PrintMediaSize(21i32);
    pub const IsoA9: PrintMediaSize = PrintMediaSize(22i32);
    pub const IsoB0: PrintMediaSize = PrintMediaSize(23i32);
    pub const IsoB1: PrintMediaSize = PrintMediaSize(24i32);
    pub const IsoB10: PrintMediaSize = PrintMediaSize(25i32);
    pub const IsoB2: PrintMediaSize = PrintMediaSize(26i32);
    pub const IsoB3: PrintMediaSize = PrintMediaSize(27i32);
    pub const IsoB4: PrintMediaSize = PrintMediaSize(28i32);
    pub const IsoB4Envelope: PrintMediaSize = PrintMediaSize(29i32);
    pub const IsoB5Envelope: PrintMediaSize = PrintMediaSize(30i32);
    pub const IsoB5Extra: PrintMediaSize = PrintMediaSize(31i32);
    pub const IsoB7: PrintMediaSize = PrintMediaSize(32i32);
    pub const IsoB8: PrintMediaSize = PrintMediaSize(33i32);
    pub const IsoB9: PrintMediaSize = PrintMediaSize(34i32);
    pub const IsoC0: PrintMediaSize = PrintMediaSize(35i32);
    pub const IsoC1: PrintMediaSize = PrintMediaSize(36i32);
    pub const IsoC10: PrintMediaSize = PrintMediaSize(37i32);
    pub const IsoC2: PrintMediaSize = PrintMediaSize(38i32);
    pub const IsoC3: PrintMediaSize = PrintMediaSize(39i32);
    pub const IsoC3Envelope: PrintMediaSize = PrintMediaSize(40i32);
    pub const IsoC4: PrintMediaSize = PrintMediaSize(41i32);
    pub const IsoC4Envelope: PrintMediaSize = PrintMediaSize(42i32);
    pub const IsoC5: PrintMediaSize = PrintMediaSize(43i32);
    pub const IsoC5Envelope: PrintMediaSize = PrintMediaSize(44i32);
    pub const IsoC6: PrintMediaSize = PrintMediaSize(45i32);
    pub const IsoC6C5Envelope: PrintMediaSize = PrintMediaSize(46i32);
    pub const IsoC6Envelope: PrintMediaSize = PrintMediaSize(47i32);
    pub const IsoC7: PrintMediaSize = PrintMediaSize(48i32);
    pub const IsoC8: PrintMediaSize = PrintMediaSize(49i32);
    pub const IsoC9: PrintMediaSize = PrintMediaSize(50i32);
    pub const IsoDLEnvelope: PrintMediaSize = PrintMediaSize(51i32);
    pub const IsoDLEnvelopeRotated: PrintMediaSize = PrintMediaSize(52i32);
    pub const IsoSRA3: PrintMediaSize = PrintMediaSize(53i32);
    pub const Japan2LPhoto: PrintMediaSize = PrintMediaSize(54i32);
    pub const JapanChou3Envelope: PrintMediaSize = PrintMediaSize(55i32);
    pub const JapanChou3EnvelopeRotated: PrintMediaSize = PrintMediaSize(56i32);
    pub const JapanChou4Envelope: PrintMediaSize = PrintMediaSize(57i32);
    pub const JapanChou4EnvelopeRotated: PrintMediaSize = PrintMediaSize(58i32);
    pub const JapanDoubleHagakiPostcard: PrintMediaSize = PrintMediaSize(59i32);
    pub const JapanDoubleHagakiPostcardRotated: PrintMediaSize = PrintMediaSize(60i32);
    pub const JapanHagakiPostcard: PrintMediaSize = PrintMediaSize(61i32);
    pub const JapanHagakiPostcardRotated: PrintMediaSize = PrintMediaSize(62i32);
    pub const JapanKaku2Envelope: PrintMediaSize = PrintMediaSize(63i32);
    pub const JapanKaku2EnvelopeRotated: PrintMediaSize = PrintMediaSize(64i32);
    pub const JapanKaku3Envelope: PrintMediaSize = PrintMediaSize(65i32);
    pub const JapanKaku3EnvelopeRotated: PrintMediaSize = PrintMediaSize(66i32);
    pub const JapanLPhoto: PrintMediaSize = PrintMediaSize(67i32);
    pub const JapanQuadrupleHagakiPostcard: PrintMediaSize = PrintMediaSize(68i32);
    pub const JapanYou1Envelope: PrintMediaSize = PrintMediaSize(69i32);
    pub const JapanYou2Envelope: PrintMediaSize = PrintMediaSize(70i32);
    pub const JapanYou3Envelope: PrintMediaSize = PrintMediaSize(71i32);
    pub const JapanYou4Envelope: PrintMediaSize = PrintMediaSize(72i32);
    pub const JapanYou4EnvelopeRotated: PrintMediaSize = PrintMediaSize(73i32);
    pub const JapanYou6Envelope: PrintMediaSize = PrintMediaSize(74i32);
    pub const JapanYou6EnvelopeRotated: PrintMediaSize = PrintMediaSize(75i32);
    pub const JisB0: PrintMediaSize = PrintMediaSize(76i32);
    pub const JisB1: PrintMediaSize = PrintMediaSize(77i32);
    pub const JisB10: PrintMediaSize = PrintMediaSize(78i32);
    pub const JisB2: PrintMediaSize = PrintMediaSize(79i32);
    pub const JisB3: PrintMediaSize = PrintMediaSize(80i32);
    pub const JisB4: PrintMediaSize = PrintMediaSize(81i32);
    pub const JisB4Rotated: PrintMediaSize = PrintMediaSize(82i32);
    pub const JisB5: PrintMediaSize = PrintMediaSize(83i32);
    pub const JisB5Rotated: PrintMediaSize = PrintMediaSize(84i32);
    pub const JisB6: PrintMediaSize = PrintMediaSize(85i32);
    pub const JisB6Rotated: PrintMediaSize = PrintMediaSize(86i32);
    pub const JisB7: PrintMediaSize = PrintMediaSize(87i32);
    pub const JisB8: PrintMediaSize = PrintMediaSize(88i32);
    pub const JisB9: PrintMediaSize = PrintMediaSize(89i32);
    pub const NorthAmerica10x11: PrintMediaSize = PrintMediaSize(90i32);
    pub const NorthAmerica10x12: PrintMediaSize = PrintMediaSize(91i32);
    pub const NorthAmerica10x14: PrintMediaSize = PrintMediaSize(92i32);
    pub const NorthAmerica11x17: PrintMediaSize = PrintMediaSize(93i32);
    pub const NorthAmerica14x17: PrintMediaSize = PrintMediaSize(94i32);
    pub const NorthAmerica4x6: PrintMediaSize = PrintMediaSize(95i32);
    pub const NorthAmerica4x8: PrintMediaSize = PrintMediaSize(96i32);
    pub const NorthAmerica5x7: PrintMediaSize = PrintMediaSize(97i32);
    pub const NorthAmerica8x10: PrintMediaSize = PrintMediaSize(98i32);
    pub const NorthAmerica9x11: PrintMediaSize = PrintMediaSize(99i32);
    pub const NorthAmericaArchitectureASheet: PrintMediaSize = PrintMediaSize(100i32);
    pub const NorthAmericaArchitectureBSheet: PrintMediaSize = PrintMediaSize(101i32);
    pub const NorthAmericaArchitectureCSheet: PrintMediaSize = PrintMediaSize(102i32);
    pub const NorthAmericaArchitectureDSheet: PrintMediaSize = PrintMediaSize(103i32);
    pub const NorthAmericaArchitectureESheet: PrintMediaSize = PrintMediaSize(104i32);
    pub const NorthAmericaCSheet: PrintMediaSize = PrintMediaSize(105i32);
    pub const NorthAmericaDSheet: PrintMediaSize = PrintMediaSize(106i32);
    pub const NorthAmericaESheet: PrintMediaSize = PrintMediaSize(107i32);
    pub const NorthAmericaExecutive: PrintMediaSize = PrintMediaSize(108i32);
    pub const NorthAmericaGermanLegalFanfold: PrintMediaSize = PrintMediaSize(109i32);
    pub const NorthAmericaGermanStandardFanfold: PrintMediaSize = PrintMediaSize(110i32);
    pub const NorthAmericaLegal: PrintMediaSize = PrintMediaSize(111i32);
    pub const NorthAmericaLegalExtra: PrintMediaSize = PrintMediaSize(112i32);
    pub const NorthAmericaLetter: PrintMediaSize = PrintMediaSize(113i32);
    pub const NorthAmericaLetterExtra: PrintMediaSize = PrintMediaSize(114i32);
    pub const NorthAmericaLetterPlus: PrintMediaSize = PrintMediaSize(115i32);
    pub const NorthAmericaLetterRotated: PrintMediaSize = PrintMediaSize(116i32);
    pub const NorthAmericaMonarchEnvelope: PrintMediaSize = PrintMediaSize(117i32);
    pub const NorthAmericaNote: PrintMediaSize = PrintMediaSize(118i32);
    pub const NorthAmericaNumber10Envelope: PrintMediaSize = PrintMediaSize(119i32);
    pub const NorthAmericaNumber10EnvelopeRotated: PrintMediaSize = PrintMediaSize(120i32);
    pub const NorthAmericaNumber11Envelope: PrintMediaSize = PrintMediaSize(121i32);
    pub const NorthAmericaNumber12Envelope: PrintMediaSize = PrintMediaSize(122i32);
    pub const NorthAmericaNumber14Envelope: PrintMediaSize = PrintMediaSize(123i32);
    pub const NorthAmericaNumber9Envelope: PrintMediaSize = PrintMediaSize(124i32);
    pub const NorthAmericaPersonalEnvelope: PrintMediaSize = PrintMediaSize(125i32);
    pub const NorthAmericaQuarto: PrintMediaSize = PrintMediaSize(126i32);
    pub const NorthAmericaStatement: PrintMediaSize = PrintMediaSize(127i32);
    pub const NorthAmericaSuperA: PrintMediaSize = PrintMediaSize(128i32);
    pub const NorthAmericaSuperB: PrintMediaSize = PrintMediaSize(129i32);
    pub const NorthAmericaTabloid: PrintMediaSize = PrintMediaSize(130i32);
    pub const NorthAmericaTabloidExtra: PrintMediaSize = PrintMediaSize(131i32);
    pub const OtherMetricA3Plus: PrintMediaSize = PrintMediaSize(132i32);
    pub const OtherMetricA4Plus: PrintMediaSize = PrintMediaSize(133i32);
    pub const OtherMetricFolio: PrintMediaSize = PrintMediaSize(134i32);
    pub const OtherMetricInviteEnvelope: PrintMediaSize = PrintMediaSize(135i32);
    pub const OtherMetricItalianEnvelope: PrintMediaSize = PrintMediaSize(136i32);
    pub const Prc10Envelope: PrintMediaSize = PrintMediaSize(137i32);
    pub const Prc10EnvelopeRotated: PrintMediaSize = PrintMediaSize(138i32);
    pub const Prc16K: PrintMediaSize = PrintMediaSize(139i32);
    pub const Prc16KRotated: PrintMediaSize = PrintMediaSize(140i32);
    pub const Prc1Envelope: PrintMediaSize = PrintMediaSize(141i32);
    pub const Prc1EnvelopeRotated: PrintMediaSize = PrintMediaSize(142i32);
    pub const Prc2Envelope: PrintMediaSize = PrintMediaSize(143i32);
    pub const Prc2EnvelopeRotated: PrintMediaSize = PrintMediaSize(144i32);
    pub const Prc32K: PrintMediaSize = PrintMediaSize(145i32);
    pub const Prc32KBig: PrintMediaSize = PrintMediaSize(146i32);
    pub const Prc32KRotated: PrintMediaSize = PrintMediaSize(147i32);
    pub const Prc3Envelope: PrintMediaSize = PrintMediaSize(148i32);
    pub const Prc3EnvelopeRotated: PrintMediaSize = PrintMediaSize(149i32);
    pub const Prc4Envelope: PrintMediaSize = PrintMediaSize(150i32);
    pub const Prc4EnvelopeRotated: PrintMediaSize = PrintMediaSize(151i32);
    pub const Prc5Envelope: PrintMediaSize = PrintMediaSize(152i32);
    pub const Prc5EnvelopeRotated: PrintMediaSize = PrintMediaSize(153i32);
    pub const Prc6Envelope: PrintMediaSize = PrintMediaSize(154i32);
    pub const Prc6EnvelopeRotated: PrintMediaSize = PrintMediaSize(155i32);
    pub const Prc7Envelope: PrintMediaSize = PrintMediaSize(156i32);
    pub const Prc7EnvelopeRotated: PrintMediaSize = PrintMediaSize(157i32);
    pub const Prc8Envelope: PrintMediaSize = PrintMediaSize(158i32);
    pub const Prc8EnvelopeRotated: PrintMediaSize = PrintMediaSize(159i32);
    pub const Prc9Envelope: PrintMediaSize = PrintMediaSize(160i32);
    pub const Prc9EnvelopeRotated: PrintMediaSize = PrintMediaSize(161i32);
    pub const Roll04Inch: PrintMediaSize = PrintMediaSize(162i32);
    pub const Roll06Inch: PrintMediaSize = PrintMediaSize(163i32);
    pub const Roll08Inch: PrintMediaSize = PrintMediaSize(164i32);
    pub const Roll12Inch: PrintMediaSize = PrintMediaSize(165i32);
    pub const Roll15Inch: PrintMediaSize = PrintMediaSize(166i32);
    pub const Roll18Inch: PrintMediaSize = PrintMediaSize(167i32);
    pub const Roll22Inch: PrintMediaSize = PrintMediaSize(168i32);
    pub const Roll24Inch: PrintMediaSize = PrintMediaSize(169i32);
    pub const Roll30Inch: PrintMediaSize = PrintMediaSize(170i32);
    pub const Roll36Inch: PrintMediaSize = PrintMediaSize(171i32);
    pub const Roll54Inch: PrintMediaSize = PrintMediaSize(172i32);
}
impl ::core::convert::From<i32> for PrintMediaSize {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PrintMediaSize {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PrintMediaSize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintMediaSize;i4)");
}
impl ::windows::core::DefaultType for PrintMediaSize {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PrintMediaType(pub i32);
impl PrintMediaType {
    pub const Default: PrintMediaType = PrintMediaType(0i32);
    pub const NotAvailable: PrintMediaType = PrintMediaType(1i32);
    pub const PrinterCustom: PrintMediaType = PrintMediaType(2i32);
    pub const AutoSelect: PrintMediaType = PrintMediaType(3i32);
    pub const Archival: PrintMediaType = PrintMediaType(4i32);
    pub const BackPrintFilm: PrintMediaType = PrintMediaType(5i32);
    pub const Bond: PrintMediaType = PrintMediaType(6i32);
    pub const CardStock: PrintMediaType = PrintMediaType(7i32);
    pub const Continuous: PrintMediaType = PrintMediaType(8i32);
    pub const EnvelopePlain: PrintMediaType = PrintMediaType(9i32);
    pub const EnvelopeWindow: PrintMediaType = PrintMediaType(10i32);
    pub const Fabric: PrintMediaType = PrintMediaType(11i32);
    pub const HighResolution: PrintMediaType = PrintMediaType(12i32);
    pub const Label: PrintMediaType = PrintMediaType(13i32);
    pub const MultiLayerForm: PrintMediaType = PrintMediaType(14i32);
    pub const MultiPartForm: PrintMediaType = PrintMediaType(15i32);
    pub const Photographic: PrintMediaType = PrintMediaType(16i32);
    pub const PhotographicFilm: PrintMediaType = PrintMediaType(17i32);
    pub const PhotographicGlossy: PrintMediaType = PrintMediaType(18i32);
    pub const PhotographicHighGloss: PrintMediaType = PrintMediaType(19i32);
    pub const PhotographicMatte: PrintMediaType = PrintMediaType(20i32);
    pub const PhotographicSatin: PrintMediaType = PrintMediaType(21i32);
    pub const PhotographicSemiGloss: PrintMediaType = PrintMediaType(22i32);
    pub const Plain: PrintMediaType = PrintMediaType(23i32);
    pub const Screen: PrintMediaType = PrintMediaType(24i32);
    pub const ScreenPaged: PrintMediaType = PrintMediaType(25i32);
    pub const Stationery: PrintMediaType = PrintMediaType(26i32);
    pub const TabStockFull: PrintMediaType = PrintMediaType(27i32);
    pub const TabStockPreCut: PrintMediaType = PrintMediaType(28i32);
    pub const Transparency: PrintMediaType = PrintMediaType(29i32);
    pub const TShirtTransfer: PrintMediaType = PrintMediaType(30i32);
    pub const None: PrintMediaType = PrintMediaType(31i32);
}
impl ::core::convert::From<i32> for PrintMediaType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PrintMediaType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PrintMediaType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintMediaType;i4)");
}
impl ::windows::core::DefaultType for PrintMediaType {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PrintOrientation(pub i32);
impl PrintOrientation {
    pub const Default: PrintOrientation = PrintOrientation(0i32);
    pub const NotAvailable: PrintOrientation = PrintOrientation(1i32);
    pub const PrinterCustom: PrintOrientation = PrintOrientation(2i32);
    pub const Portrait: PrintOrientation = PrintOrientation(3i32);
    pub const PortraitFlipped: PrintOrientation = PrintOrientation(4i32);
    pub const Landscape: PrintOrientation = PrintOrientation(5i32);
    pub const LandscapeFlipped: PrintOrientation = PrintOrientation(6i32);
}
impl ::core::convert::From<i32> for PrintOrientation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PrintOrientation {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PrintOrientation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintOrientation;i4)");
}
impl ::windows::core::DefaultType for PrintOrientation {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation")]
pub struct PrintPageDescription {
    pub PageSize: super::super::Foundation::Size,
    pub ImageableRect: super::super::Foundation::Rect,
    pub DpiX: u32,
    pub DpiY: u32,
}
#[cfg(feature = "Foundation")]
impl PrintPageDescription {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for PrintPageDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for PrintPageDescription {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PrintPageDescription").field("PageSize", &self.PageSize).field("ImageableRect", &self.ImageableRect).field("DpiX", &self.DpiX).field("DpiY", &self.DpiY).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for PrintPageDescription {
    fn eq(&self, other: &Self) -> bool {
        self.PageSize == other.PageSize && self.ImageableRect == other.ImageableRect && self.DpiX == other.DpiX && self.DpiY == other.DpiY
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for PrintPageDescription {}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Abi for PrintPageDescription {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for PrintPageDescription {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.Printing.PrintPageDescription;struct(Windows.Foundation.Size;f4;f4);struct(Windows.Foundation.Rect;f4;f4;f4;f4);u4;u4)");
}
#[cfg(feature = "Foundation")]
impl ::windows::core::DefaultType for PrintPageDescription {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintPageInfo(pub ::windows::core::IInspectable);
impl PrintPageInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PrintPageInfo, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetMediaSize(&self, value: PrintMediaSize) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MediaSize(&self) -> ::windows::core::Result<PrintMediaSize> {
        let this = self;
        unsafe {
            let mut result__: PrintMediaSize = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintMediaSize>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPageSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Size>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PageSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    pub fn SetDpiX(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DpiX(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetDpiY(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DpiY(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetOrientation(&self, value: PrintOrientation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Orientation(&self) -> ::windows::core::Result<PrintOrientation> {
        let this = self;
        unsafe {
            let mut result__: PrintOrientation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOrientation>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintPageInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintPageInfo;{dd4be9c9-a6a1-4ada-930e-da872a4f23d3})");
}
unsafe impl ::windows::core::Interface for PrintPageInfo {
    type Vtable = IPrintPageInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd4be9c9_a6a1_4ada_930e_da872a4f23d3);
}
impl ::windows::core::RuntimeName for PrintPageInfo {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintPageInfo";
}
impl ::core::convert::From<PrintPageInfo> for ::windows::core::IUnknown {
    fn from(value: PrintPageInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintPageInfo> for ::windows::core::IUnknown {
    fn from(value: &PrintPageInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintPageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintPageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintPageInfo> for ::windows::core::IInspectable {
    fn from(value: PrintPageInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintPageInfo> for ::windows::core::IInspectable {
    fn from(value: &PrintPageInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintPageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintPageInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintPageInfo {}
unsafe impl ::core::marker::Sync for PrintPageInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintPageRange(pub ::windows::core::IInspectable);
impl PrintPageRange {
    pub fn FirstPageNumber(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn LastPageNumber(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Create(firstpage: i32, lastpage: i32) -> ::windows::core::Result<PrintPageRange> {
        Self::IPrintPageRangeFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), firstpage, lastpage, &mut result__).from_abi::<PrintPageRange>(result__)
        })
    }
    pub fn CreateWithSinglePage(page: i32) -> ::windows::core::Result<PrintPageRange> {
        Self::IPrintPageRangeFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), page, &mut result__).from_abi::<PrintPageRange>(result__)
        })
    }
    pub fn IPrintPageRangeFactory<R, F: FnOnce(&IPrintPageRangeFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PrintPageRange, IPrintPageRangeFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintPageRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintPageRange;{f8a06c54-6e7c-51c5-57fd-0660c2d71513})");
}
unsafe impl ::windows::core::Interface for PrintPageRange {
    type Vtable = IPrintPageRange_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8a06c54_6e7c_51c5_57fd_0660c2d71513);
}
impl ::windows::core::RuntimeName for PrintPageRange {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintPageRange";
}
impl ::core::convert::From<PrintPageRange> for ::windows::core::IUnknown {
    fn from(value: PrintPageRange) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintPageRange> for ::windows::core::IUnknown {
    fn from(value: &PrintPageRange) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintPageRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintPageRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintPageRange> for ::windows::core::IInspectable {
    fn from(value: PrintPageRange) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintPageRange> for ::windows::core::IInspectable {
    fn from(value: &PrintPageRange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintPageRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintPageRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintPageRange {}
unsafe impl ::core::marker::Sync for PrintPageRange {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintPageRangeOptions(pub ::windows::core::IInspectable);
impl PrintPageRangeOptions {
    pub fn SetAllowAllPages(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AllowAllPages(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowCurrentPage(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AllowCurrentPage(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowCustomSetOfPages(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AllowCustomSetOfPages(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintPageRangeOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintPageRangeOptions;{ce6db728-1357-46b2-a923-79f995f448fc})");
}
unsafe impl ::windows::core::Interface for PrintPageRangeOptions {
    type Vtable = IPrintPageRangeOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce6db728_1357_46b2_a923_79f995f448fc);
}
impl ::windows::core::RuntimeName for PrintPageRangeOptions {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintPageRangeOptions";
}
impl ::core::convert::From<PrintPageRangeOptions> for ::windows::core::IUnknown {
    fn from(value: PrintPageRangeOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintPageRangeOptions> for ::windows::core::IUnknown {
    fn from(value: &PrintPageRangeOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintPageRangeOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintPageRangeOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintPageRangeOptions> for ::windows::core::IInspectable {
    fn from(value: PrintPageRangeOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintPageRangeOptions> for ::windows::core::IInspectable {
    fn from(value: &PrintPageRangeOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintPageRangeOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintPageRangeOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintPageRangeOptions {}
unsafe impl ::core::marker::Sync for PrintPageRangeOptions {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PrintQuality(pub i32);
impl PrintQuality {
    pub const Default: PrintQuality = PrintQuality(0i32);
    pub const NotAvailable: PrintQuality = PrintQuality(1i32);
    pub const PrinterCustom: PrintQuality = PrintQuality(2i32);
    pub const Automatic: PrintQuality = PrintQuality(3i32);
    pub const Draft: PrintQuality = PrintQuality(4i32);
    pub const Fax: PrintQuality = PrintQuality(5i32);
    pub const High: PrintQuality = PrintQuality(6i32);
    pub const Normal: PrintQuality = PrintQuality(7i32);
    pub const Photographic: PrintQuality = PrintQuality(8i32);
    pub const Text: PrintQuality = PrintQuality(9i32);
}
impl ::core::convert::From<i32> for PrintQuality {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PrintQuality {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PrintQuality {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintQuality;i4)");
}
impl ::windows::core::DefaultType for PrintQuality {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PrintStaple(pub i32);
impl PrintStaple {
    pub const Default: PrintStaple = PrintStaple(0i32);
    pub const NotAvailable: PrintStaple = PrintStaple(1i32);
    pub const PrinterCustom: PrintStaple = PrintStaple(2i32);
    pub const None: PrintStaple = PrintStaple(3i32);
    pub const StapleTopLeft: PrintStaple = PrintStaple(4i32);
    pub const StapleTopRight: PrintStaple = PrintStaple(5i32);
    pub const StapleBottomLeft: PrintStaple = PrintStaple(6i32);
    pub const StapleBottomRight: PrintStaple = PrintStaple(7i32);
    pub const StapleDualLeft: PrintStaple = PrintStaple(8i32);
    pub const StapleDualRight: PrintStaple = PrintStaple(9i32);
    pub const StapleDualTop: PrintStaple = PrintStaple(10i32);
    pub const StapleDualBottom: PrintStaple = PrintStaple(11i32);
    pub const SaddleStitch: PrintStaple = PrintStaple(12i32);
}
impl ::core::convert::From<i32> for PrintStaple {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PrintStaple {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PrintStaple {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintStaple;i4)");
}
impl ::windows::core::DefaultType for PrintStaple {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTask(pub ::windows::core::IInspectable);
impl PrintTask {
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackagePropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::ApplicationModel::DataTransfer::DataPackagePropertySet>(result__)
        }
    }
    pub fn Source(&self) -> ::windows::core::Result<IPrintDocumentSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IPrintDocumentSource>(result__)
        }
    }
    pub fn Options(&self) -> ::windows::core::Result<PrintTaskOptions> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTaskOptions>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Previewing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PrintTask, ::windows::core::IInspectable>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePreviewing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Submitting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PrintTask, ::windows::core::IInspectable>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSubmitting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Progressing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PrintTask, PrintTaskProgressingEventArgs>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveProgressing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Completed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PrintTask, PrintTaskCompletedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    pub fn SetIsPrinterTargetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskTargetDeviceSupport>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsPrinterTargetEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPrintTaskTargetDeviceSupport>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIs3DManufacturingTargetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskTargetDeviceSupport>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Is3DManufacturingTargetEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPrintTaskTargetDeviceSupport>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPreviewEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTask2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsPreviewEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPrintTask2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTask {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTask;{61d80247-6cf6-4fad-84e2-a5e82e2d4ceb})");
}
unsafe impl ::windows::core::Interface for PrintTask {
    type Vtable = IPrintTask_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61d80247_6cf6_4fad_84e2_a5e82e2d4ceb);
}
impl ::windows::core::RuntimeName for PrintTask {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTask";
}
impl ::core::convert::From<PrintTask> for ::windows::core::IUnknown {
    fn from(value: PrintTask) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintTask> for ::windows::core::IUnknown {
    fn from(value: &PrintTask) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintTask> for ::windows::core::IInspectable {
    fn from(value: PrintTask) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintTask> for ::windows::core::IInspectable {
    fn from(value: &PrintTask) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintTask {}
unsafe impl ::core::marker::Sync for PrintTask {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTaskCompletedEventArgs(pub ::windows::core::IInspectable);
impl PrintTaskCompletedEventArgs {
    pub fn Completion(&self) -> ::windows::core::Result<PrintTaskCompletion> {
        let this = self;
        unsafe {
            let mut result__: PrintTaskCompletion = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTaskCompletion>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskCompletedEventArgs;{5bcd34af-24e9-4c10-8d07-14c346ba3fce})");
}
unsafe impl ::windows::core::Interface for PrintTaskCompletedEventArgs {
    type Vtable = IPrintTaskCompletedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bcd34af_24e9_4c10_8d07_14c346ba3fce);
}
impl ::windows::core::RuntimeName for PrintTaskCompletedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskCompletedEventArgs";
}
impl ::core::convert::From<PrintTaskCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintTaskCompletedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintTaskCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskCompletedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTaskCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintTaskCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintTaskCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintTaskCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintTaskCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTaskCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintTaskCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintTaskCompletedEventArgs {}
unsafe impl ::core::marker::Sync for PrintTaskCompletedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PrintTaskCompletion(pub i32);
impl PrintTaskCompletion {
    pub const Abandoned: PrintTaskCompletion = PrintTaskCompletion(0i32);
    pub const Canceled: PrintTaskCompletion = PrintTaskCompletion(1i32);
    pub const Failed: PrintTaskCompletion = PrintTaskCompletion(2i32);
    pub const Submitted: PrintTaskCompletion = PrintTaskCompletion(3i32);
}
impl ::core::convert::From<i32> for PrintTaskCompletion {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PrintTaskCompletion {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PrintTaskCompletion {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintTaskCompletion;i4)");
}
impl ::windows::core::DefaultType for PrintTaskCompletion {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTaskOptions(pub ::windows::core::IInspectable);
impl PrintTaskOptions {
    #[cfg(feature = "Foundation")]
    pub fn GetPageDescription(&self, jobpagenumber: u32) -> ::windows::core::Result<PrintPageDescription> {
        let this = self;
        unsafe {
            let mut result__: PrintPageDescription = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), jobpagenumber, &mut result__).from_abi::<PrintPageDescription>(result__)
        }
    }
    pub fn SetMediaSize(&self, value: PrintMediaSize) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MediaSize(&self) -> ::windows::core::Result<PrintMediaSize> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: PrintMediaSize = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintMediaSize>(result__)
        }
    }
    pub fn SetMediaType(&self, value: PrintMediaType) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MediaType(&self) -> ::windows::core::Result<PrintMediaType> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: PrintMediaType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintMediaType>(result__)
        }
    }
    pub fn SetOrientation(&self, value: PrintOrientation) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Orientation(&self) -> ::windows::core::Result<PrintOrientation> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: PrintOrientation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOrientation>(result__)
        }
    }
    pub fn SetPrintQuality(&self, value: PrintQuality) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn PrintQuality(&self) -> ::windows::core::Result<PrintQuality> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: PrintQuality = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintQuality>(result__)
        }
    }
    pub fn SetColorMode(&self, value: PrintColorMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ColorMode(&self) -> ::windows::core::Result<PrintColorMode> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: PrintColorMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintColorMode>(result__)
        }
    }
    pub fn SetDuplex(&self, value: PrintDuplex) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Duplex(&self) -> ::windows::core::Result<PrintDuplex> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: PrintDuplex = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintDuplex>(result__)
        }
    }
    pub fn SetCollation(&self, value: PrintCollation) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Collation(&self) -> ::windows::core::Result<PrintCollation> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: PrintCollation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintCollation>(result__)
        }
    }
    pub fn SetStaple(&self, value: PrintStaple) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Staple(&self) -> ::windows::core::Result<PrintStaple> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: PrintStaple = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintStaple>(result__)
        }
    }
    pub fn SetHolePunch(&self, value: PrintHolePunch) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn HolePunch(&self) -> ::windows::core::Result<PrintHolePunch> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: PrintHolePunch = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintHolePunch>(result__)
        }
    }
    pub fn SetBinding(&self, value: PrintBinding) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Binding(&self) -> ::windows::core::Result<PrintBinding> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: PrintBinding = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintBinding>(result__)
        }
    }
    pub fn MinCopies(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn MaxCopies(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetNumberOfCopies(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn NumberOfCopies(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DisplayedOptions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionsCoreUIConfiguration>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn SetBordering(&self, value: PrintBordering) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptions>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Bordering(&self) -> ::windows::core::Result<PrintBordering> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptions>(self)?;
        unsafe {
            let mut result__: PrintBordering = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintBordering>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetPagePrintTicket<'a, Param0: ::windows::core::IntoParam<'a, PrintPageInfo>>(&self, printpageinfo: Param0) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptions>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), printpageinfo.into_param().abi(), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    pub fn PageRangeOptions(&self) -> ::windows::core::Result<PrintPageRangeOptions> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptions2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintPageRangeOptions>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CustomPageRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<PrintPageRange>> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptions2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<PrintPageRange>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskOptions;{1bdbb474-4ed1-41eb-be3c-72d18ed67337})");
}
unsafe impl ::windows::core::Interface for PrintTaskOptions {
    type Vtable = IPrintTaskOptionsCore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bdbb474_4ed1_41eb_be3c_72d18ed67337);
}
impl ::windows::core::RuntimeName for PrintTaskOptions {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskOptions";
}
impl ::core::convert::From<PrintTaskOptions> for ::windows::core::IUnknown {
    fn from(value: PrintTaskOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintTaskOptions> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTaskOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintTaskOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintTaskOptions> for ::windows::core::IInspectable {
    fn from(value: PrintTaskOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintTaskOptions> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTaskOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintTaskOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PrintTaskOptions> for IPrintTaskOptionsCore {
    fn from(value: PrintTaskOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskOptions> for IPrintTaskOptionsCore {
    fn from(value: &PrintTaskOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintTaskOptionsCore> for PrintTaskOptions {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintTaskOptionsCore> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintTaskOptionsCore> for &PrintTaskOptions {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintTaskOptionsCore> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PrintTaskOptions> for IPrintTaskOptionsCoreProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintTaskOptions) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintTaskOptions> for IPrintTaskOptionsCoreProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintTaskOptions) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintTaskOptionsCoreProperties> for PrintTaskOptions {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintTaskOptionsCoreProperties> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintTaskOptionsCoreProperties> for &PrintTaskOptions {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintTaskOptionsCoreProperties> {
        ::core::convert::TryInto::<IPrintTaskOptionsCoreProperties>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PrintTaskOptions> for IPrintTaskOptionsCoreUIConfiguration {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintTaskOptions) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintTaskOptions> for IPrintTaskOptionsCoreUIConfiguration {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintTaskOptions) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintTaskOptionsCoreUIConfiguration> for PrintTaskOptions {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintTaskOptionsCoreUIConfiguration> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintTaskOptionsCoreUIConfiguration> for &PrintTaskOptions {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintTaskOptionsCoreUIConfiguration> {
        ::core::convert::TryInto::<IPrintTaskOptionsCoreUIConfiguration>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintTaskOptions {}
unsafe impl ::core::marker::Sync for PrintTaskOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTaskProgressingEventArgs(pub ::windows::core::IInspectable);
impl PrintTaskProgressingEventArgs {
    pub fn DocumentPageCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskProgressingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskProgressingEventArgs;{810cd3cb-b410-4282-a073-5ac378234174})");
}
unsafe impl ::windows::core::Interface for PrintTaskProgressingEventArgs {
    type Vtable = IPrintTaskProgressingEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x810cd3cb_b410_4282_a073_5ac378234174);
}
impl ::windows::core::RuntimeName for PrintTaskProgressingEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskProgressingEventArgs";
}
impl ::core::convert::From<PrintTaskProgressingEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintTaskProgressingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintTaskProgressingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskProgressingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTaskProgressingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintTaskProgressingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintTaskProgressingEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintTaskProgressingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintTaskProgressingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskProgressingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTaskProgressingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintTaskProgressingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintTaskProgressingEventArgs {}
unsafe impl ::core::marker::Sync for PrintTaskProgressingEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTaskRequest(pub ::windows::core::IInspectable);
impl PrintTaskRequest {
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn CreatePrintTask<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, PrintTaskSourceRequestedHandler>>(&self, title: Param0, handler: Param1) -> ::windows::core::Result<PrintTask> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), title.into_param().abi(), handler.into_param().abi(), &mut result__).from_abi::<PrintTask>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<PrintTaskRequestedDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTaskRequestedDeferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskRequest;{6ff61e2e-2722-4240-a67c-f364849a17f3})");
}
unsafe impl ::windows::core::Interface for PrintTaskRequest {
    type Vtable = IPrintTaskRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ff61e2e_2722_4240_a67c_f364849a17f3);
}
impl ::windows::core::RuntimeName for PrintTaskRequest {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskRequest";
}
impl ::core::convert::From<PrintTaskRequest> for ::windows::core::IUnknown {
    fn from(value: PrintTaskRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintTaskRequest> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTaskRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintTaskRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintTaskRequest> for ::windows::core::IInspectable {
    fn from(value: PrintTaskRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintTaskRequest> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTaskRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintTaskRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintTaskRequest {}
unsafe impl ::core::marker::Sync for PrintTaskRequest {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTaskRequestedDeferral(pub ::windows::core::IInspectable);
impl PrintTaskRequestedDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskRequestedDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskRequestedDeferral;{cfefb3f0-ce3e-42c7-9496-64800c622c44})");
}
unsafe impl ::windows::core::Interface for PrintTaskRequestedDeferral {
    type Vtable = IPrintTaskRequestedDeferral_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfefb3f0_ce3e_42c7_9496_64800c622c44);
}
impl ::windows::core::RuntimeName for PrintTaskRequestedDeferral {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskRequestedDeferral";
}
impl ::core::convert::From<PrintTaskRequestedDeferral> for ::windows::core::IUnknown {
    fn from(value: PrintTaskRequestedDeferral) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintTaskRequestedDeferral> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskRequestedDeferral) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTaskRequestedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintTaskRequestedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintTaskRequestedDeferral> for ::windows::core::IInspectable {
    fn from(value: PrintTaskRequestedDeferral) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintTaskRequestedDeferral> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskRequestedDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTaskRequestedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintTaskRequestedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintTaskRequestedDeferral {}
unsafe impl ::core::marker::Sync for PrintTaskRequestedDeferral {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTaskRequestedEventArgs(pub ::windows::core::IInspectable);
impl PrintTaskRequestedEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<PrintTaskRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTaskRequest>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskRequestedEventArgs;{d0aff924-a31b-454c-a7b6-5d0cc522fc16})");
}
unsafe impl ::windows::core::Interface for PrintTaskRequestedEventArgs {
    type Vtable = IPrintTaskRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0aff924_a31b_454c_a7b6_5d0cc522fc16);
}
impl ::windows::core::RuntimeName for PrintTaskRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskRequestedEventArgs";
}
impl ::core::convert::From<PrintTaskRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintTaskRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintTaskRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTaskRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintTaskRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintTaskRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintTaskRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintTaskRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTaskRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintTaskRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintTaskRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PrintTaskRequestedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTaskSourceRequestedArgs(pub ::windows::core::IInspectable);
impl PrintTaskSourceRequestedArgs {
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn SetSource<'a, Param0: ::windows::core::IntoParam<'a, IPrintDocumentSource>>(&self, source: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), source.into_param().abi()).ok() }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<PrintTaskSourceRequestedDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTaskSourceRequestedDeferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskSourceRequestedArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskSourceRequestedArgs;{f9f067be-f456-41f0-9c98-5ce73e851410})");
}
unsafe impl ::windows::core::Interface for PrintTaskSourceRequestedArgs {
    type Vtable = IPrintTaskSourceRequestedArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9f067be_f456_41f0_9c98_5ce73e851410);
}
impl ::windows::core::RuntimeName for PrintTaskSourceRequestedArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskSourceRequestedArgs";
}
impl ::core::convert::From<PrintTaskSourceRequestedArgs> for ::windows::core::IUnknown {
    fn from(value: PrintTaskSourceRequestedArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintTaskSourceRequestedArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskSourceRequestedArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTaskSourceRequestedArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintTaskSourceRequestedArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintTaskSourceRequestedArgs> for ::windows::core::IInspectable {
    fn from(value: PrintTaskSourceRequestedArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintTaskSourceRequestedArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskSourceRequestedArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTaskSourceRequestedArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintTaskSourceRequestedArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintTaskSourceRequestedArgs {}
unsafe impl ::core::marker::Sync for PrintTaskSourceRequestedArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTaskSourceRequestedDeferral(pub ::windows::core::IInspectable);
impl PrintTaskSourceRequestedDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskSourceRequestedDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskSourceRequestedDeferral;{4a1560d1-6992-4d9d-8555-4ca4563fb166})");
}
unsafe impl ::windows::core::Interface for PrintTaskSourceRequestedDeferral {
    type Vtable = IPrintTaskSourceRequestedDeferral_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a1560d1_6992_4d9d_8555_4ca4563fb166);
}
impl ::windows::core::RuntimeName for PrintTaskSourceRequestedDeferral {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskSourceRequestedDeferral";
}
impl ::core::convert::From<PrintTaskSourceRequestedDeferral> for ::windows::core::IUnknown {
    fn from(value: PrintTaskSourceRequestedDeferral) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintTaskSourceRequestedDeferral> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskSourceRequestedDeferral) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTaskSourceRequestedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintTaskSourceRequestedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintTaskSourceRequestedDeferral> for ::windows::core::IInspectable {
    fn from(value: PrintTaskSourceRequestedDeferral) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintTaskSourceRequestedDeferral> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskSourceRequestedDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTaskSourceRequestedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintTaskSourceRequestedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintTaskSourceRequestedDeferral {}
unsafe impl ::core::marker::Sync for PrintTaskSourceRequestedDeferral {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTaskSourceRequestedHandler(::windows::core::IUnknown);
impl PrintTaskSourceRequestedHandler {
    pub fn new<F: FnMut(&::core::option::Option<PrintTaskSourceRequestedArgs>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = PrintTaskSourceRequestedHandler_box::<F> {
            vtable: &PrintTaskSourceRequestedHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, PrintTaskSourceRequestedArgs>>(&self, args: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), args.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskSourceRequestedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({6c109fa8-5cb6-4b3a-8663-f39cb02dc9b4})");
}
unsafe impl ::windows::core::Interface for PrintTaskSourceRequestedHandler {
    type Vtable = PrintTaskSourceRequestedHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c109fa8_5cb6_4b3a_8663_f39cb02dc9b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct PrintTaskSourceRequestedHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct PrintTaskSourceRequestedHandler_box<F: FnMut(&::core::option::Option<PrintTaskSourceRequestedArgs>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const PrintTaskSourceRequestedHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<PrintTaskSourceRequestedArgs>) -> ::windows::core::Result<()> + 'static> PrintTaskSourceRequestedHandler_box<F> {
    const VTABLE: PrintTaskSourceRequestedHandler_abi = PrintTaskSourceRequestedHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<PrintTaskSourceRequestedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&args as *const <PrintTaskSourceRequestedArgs as ::windows::core::Abi>::Abi as *const <PrintTaskSourceRequestedArgs as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
pub struct StandardPrintTaskOptions {}
impl StandardPrintTaskOptions {
    pub fn MediaSize() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn MediaType() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Orientation() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn PrintQuality() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ColorMode() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Duplex() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Collation() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Staple() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn HolePunch() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Binding() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Copies() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn NUp() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn InputBin() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn Bordering() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn CustomPageRanges() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic3(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IStandardPrintTaskOptionsStatic<R, F: FnOnce(&IStandardPrintTaskOptionsStatic) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StandardPrintTaskOptions, IStandardPrintTaskOptionsStatic> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IStandardPrintTaskOptionsStatic2<R, F: FnOnce(&IStandardPrintTaskOptionsStatic2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StandardPrintTaskOptions, IStandardPrintTaskOptionsStatic2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IStandardPrintTaskOptionsStatic3<R, F: FnOnce(&IStandardPrintTaskOptionsStatic3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StandardPrintTaskOptions, IStandardPrintTaskOptionsStatic3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for StandardPrintTaskOptions {
    const NAME: &'static str = "Windows.Graphics.Printing.StandardPrintTaskOptions";
}
