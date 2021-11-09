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
#[doc = "*Required features: `Graphics_Printing`*"]
pub struct IPrintDocumentSource(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintDocumentSource {
    type Vtable = IPrintDocumentSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3738962992, 61931, 18399, [170, 230, 237, 84, 39, 81, 31, 1]);
}
impl IPrintDocumentSource {}
unsafe impl ::windows::runtime::RuntimeType for IPrintDocumentSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{dedc0c30-f1eb-47df-aae6-ed5427511f01}");
}
impl ::core::convert::From<IPrintDocumentSource> for ::windows::runtime::IUnknown {
    fn from(value: IPrintDocumentSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPrintDocumentSource> for ::windows::runtime::IUnknown {
    fn from(value: &IPrintDocumentSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrintDocumentSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPrintDocumentSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPrintDocumentSource> for ::windows::runtime::IInspectable {
    fn from(value: IPrintDocumentSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintDocumentSource> for ::windows::runtime::IInspectable {
    fn from(value: &IPrintDocumentSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IPrintDocumentSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IPrintDocumentSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintManager(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintManager {
    type Vtable = IPrintManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4280981140, 35993, 17661, [174, 74, 25, 217, 170, 154, 15, 10]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintManagerStatic(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintManagerStatic {
    type Vtable = IPrintManagerStatic_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1477991885, 58932, 18004, [132, 240, 224, 21, 42, 130, 23, 172]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintManagerStatic_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintManagerStatic2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintManagerStatic2 {
    type Vtable = IPrintManagerStatic2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(900307285, 59051, 16697, [154, 189, 184, 106, 114, 155, 53, 152]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintManagerStatic2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintPageInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintPageInfo {
    type Vtable = IPrintPageInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3712739785, 42657, 19162, [147, 14, 218, 135, 42, 79, 35, 211]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPageInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PrintMediaSize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PrintMediaSize) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PrintOrientation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PrintOrientation) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintPageRange(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintPageRange {
    type Vtable = IPrintPageRange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4171263060, 28284, 20933, [87, 253, 6, 96, 194, 215, 21, 19]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPageRange_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintPageRangeFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintPageRangeFactory {
    type Vtable = IPrintPageRangeFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1083167839, 57415, 24453, [113, 41, 251, 8, 90, 79, 173, 20]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPageRangeFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, firstpage: i32, lastpage: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, page: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintPageRangeOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintPageRangeOptions {
    type Vtable = IPrintPageRangeOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3463296808, 4951, 18098, [169, 35, 121, 249, 149, 244, 72, 252]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPageRangeOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTask(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintTask {
    type Vtable = IPrintTask_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1641546311, 27894, 20397, [132, 226, 165, 232, 46, 45, 76, 235]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTask_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_DataTransfer")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTask2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintTask2 {
    type Vtable = IPrintTask2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(908281975, 15955, 19869, [143, 94, 49, 106, 200, 222, 218, 225]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTask2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskCompletedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintTaskCompletedEventArgs {
    type Vtable = IPrintTaskCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1540175023, 9449, 19472, [141, 7, 20, 195, 70, 186, 63, 206]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PrintTaskCompletion) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintTaskOptions {
    type Vtable = IPrintTaskOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1510631099, 53897, 16827, [150, 221, 87, 226, 131, 56, 174, 63]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PrintBordering) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PrintBordering) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, printpageinfo: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskOptions2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintTaskOptions2 {
    type Vtable = IPrintTaskOptions2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3952809478, 39478, 19289, [134, 23, 178, 23, 132, 146, 98, 225]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptions2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Graphics_Printing`*"]
pub struct IPrintTaskOptionsCore(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintTaskOptionsCore {
    type Vtable = IPrintTaskOptionsCore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(467383412, 20177, 16875, [190, 60, 114, 209, 142, 214, 115, 55]);
}
impl IPrintTaskOptionsCore {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing`, `Foundation`*"]
    pub fn GetPageDescription(&self, jobpagenumber: u32) -> ::windows::runtime::Result<PrintPageDescription> {
        let this = self;
        unsafe {
            let mut result__: PrintPageDescription = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), jobpagenumber, &mut result__).from_abi::<PrintPageDescription>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPrintTaskOptionsCore {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{1bdbb474-4ed1-41eb-be3c-72d18ed67337}");
}
impl ::core::convert::From<IPrintTaskOptionsCore> for ::windows::runtime::IUnknown {
    fn from(value: IPrintTaskOptionsCore) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPrintTaskOptionsCore> for ::windows::runtime::IUnknown {
    fn from(value: &IPrintTaskOptionsCore) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrintTaskOptionsCore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPrintTaskOptionsCore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPrintTaskOptionsCore> for ::windows::runtime::IInspectable {
    fn from(value: IPrintTaskOptionsCore) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintTaskOptionsCore> for ::windows::runtime::IInspectable {
    fn from(value: &IPrintTaskOptionsCore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IPrintTaskOptionsCore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IPrintTaskOptionsCore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptionsCore_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, jobpagenumber: u32, result__: *mut PrintPageDescription) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Graphics_Printing`*"]
pub struct IPrintTaskOptionsCoreProperties(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintTaskOptionsCoreProperties {
    type Vtable = IPrintTaskOptionsCoreProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3250001970, 40595, 20053, [129, 75, 51, 38, 165, 158, 252, 225]);
}
impl IPrintTaskOptionsCoreProperties {
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetMediaSize(&self, value: PrintMediaSize) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn MediaSize(&self) -> ::windows::runtime::Result<PrintMediaSize> {
        let this = self;
        unsafe {
            let mut result__: PrintMediaSize = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintMediaSize>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetMediaType(&self, value: PrintMediaType) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn MediaType(&self) -> ::windows::runtime::Result<PrintMediaType> {
        let this = self;
        unsafe {
            let mut result__: PrintMediaType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintMediaType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetOrientation(&self, value: PrintOrientation) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Orientation(&self) -> ::windows::runtime::Result<PrintOrientation> {
        let this = self;
        unsafe {
            let mut result__: PrintOrientation = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOrientation>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetPrintQuality(&self, value: PrintQuality) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn PrintQuality(&self) -> ::windows::runtime::Result<PrintQuality> {
        let this = self;
        unsafe {
            let mut result__: PrintQuality = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintQuality>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetColorMode(&self, value: PrintColorMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn ColorMode(&self) -> ::windows::runtime::Result<PrintColorMode> {
        let this = self;
        unsafe {
            let mut result__: PrintColorMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintColorMode>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetDuplex(&self, value: PrintDuplex) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Duplex(&self) -> ::windows::runtime::Result<PrintDuplex> {
        let this = self;
        unsafe {
            let mut result__: PrintDuplex = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintDuplex>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetCollation(&self, value: PrintCollation) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Collation(&self) -> ::windows::runtime::Result<PrintCollation> {
        let this = self;
        unsafe {
            let mut result__: PrintCollation = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintCollation>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetStaple(&self, value: PrintStaple) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Staple(&self) -> ::windows::runtime::Result<PrintStaple> {
        let this = self;
        unsafe {
            let mut result__: PrintStaple = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintStaple>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetHolePunch(&self, value: PrintHolePunch) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn HolePunch(&self) -> ::windows::runtime::Result<PrintHolePunch> {
        let this = self;
        unsafe {
            let mut result__: PrintHolePunch = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintHolePunch>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetBinding(&self, value: PrintBinding) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Binding(&self) -> ::windows::runtime::Result<PrintBinding> {
        let this = self;
        unsafe {
            let mut result__: PrintBinding = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintBinding>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn MinCopies(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn MaxCopies(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetNumberOfCopies(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn NumberOfCopies(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPrintTaskOptionsCoreProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{c1b71832-9e93-4e55-814b-3326a59efce1}");
}
impl ::core::convert::From<IPrintTaskOptionsCoreProperties> for ::windows::runtime::IUnknown {
    fn from(value: IPrintTaskOptionsCoreProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPrintTaskOptionsCoreProperties> for ::windows::runtime::IUnknown {
    fn from(value: &IPrintTaskOptionsCoreProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrintTaskOptionsCoreProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPrintTaskOptionsCoreProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPrintTaskOptionsCoreProperties> for ::windows::runtime::IInspectable {
    fn from(value: IPrintTaskOptionsCoreProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintTaskOptionsCoreProperties> for ::windows::runtime::IInspectable {
    fn from(value: &IPrintTaskOptionsCoreProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IPrintTaskOptionsCoreProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IPrintTaskOptionsCoreProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptionsCoreProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PrintMediaSize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PrintMediaSize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PrintMediaType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PrintMediaType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PrintOrientation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PrintOrientation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PrintQuality) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PrintQuality) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PrintColorMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PrintColorMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PrintDuplex) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PrintDuplex) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PrintCollation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PrintCollation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PrintStaple) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PrintStaple) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PrintHolePunch) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PrintHolePunch) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PrintBinding) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PrintBinding) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Graphics_Printing`*"]
pub struct IPrintTaskOptionsCoreUIConfiguration(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintTaskOptionsCoreUIConfiguration {
    type Vtable = IPrintTaskOptionsCoreUIConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1659280931, 39454, 17206, [183, 79, 60, 199, 244, 207, 247, 9]);
}
impl IPrintTaskOptionsCoreUIConfiguration {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing`, `Foundation_Collections`*"]
    pub fn DisplayedOptions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPrintTaskOptionsCoreUIConfiguration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{62e69e23-9a1e-4336-b74f-3cc7f4cff709}");
}
impl ::core::convert::From<IPrintTaskOptionsCoreUIConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: IPrintTaskOptionsCoreUIConfiguration) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPrintTaskOptionsCoreUIConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: &IPrintTaskOptionsCoreUIConfiguration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrintTaskOptionsCoreUIConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPrintTaskOptionsCoreUIConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPrintTaskOptionsCoreUIConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: IPrintTaskOptionsCoreUIConfiguration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintTaskOptionsCoreUIConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: &IPrintTaskOptionsCoreUIConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IPrintTaskOptionsCoreUIConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IPrintTaskOptionsCoreUIConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptionsCoreUIConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskProgressingEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintTaskProgressingEventArgs {
    type Vtable = IPrintTaskProgressingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2165101515, 46096, 17026, [160, 115, 90, 195, 120, 35, 65, 116]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskProgressingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskRequest(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintTaskRequest {
    type Vtable = IPrintTaskRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1878400558, 10018, 16960, [166, 124, 243, 100, 132, 154, 23, 243]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, title: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, handler: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskRequestedDeferral(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintTaskRequestedDeferral {
    type Vtable = IPrintTaskRequestedDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3488592880, 52798, 17095, [148, 150, 100, 128, 12, 98, 44, 68]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskRequestedDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskRequestedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintTaskRequestedEventArgs {
    type Vtable = IPrintTaskRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3501193508, 41755, 17740, [167, 182, 93, 12, 197, 34, 252, 22]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskSourceRequestedArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintTaskSourceRequestedArgs {
    type Vtable = IPrintTaskSourceRequestedArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4193281982, 62550, 16880, [156, 152, 92, 231, 62, 133, 20, 16]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskSourceRequestedArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, source: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskSourceRequestedDeferral(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintTaskSourceRequestedDeferral {
    type Vtable = IPrintTaskSourceRequestedDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1242915025, 27026, 19869, [133, 85, 76, 164, 86, 63, 177, 102]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskSourceRequestedDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskTargetDeviceSupport(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintTaskTargetDeviceSupport {
    type Vtable = IPrintTaskTargetDeviceSupport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(693989568, 49867, 19325, [176, 234, 147, 9, 80, 145, 162, 32]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskTargetDeviceSupport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStandardPrintTaskOptionsStatic(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStandardPrintTaskOptionsStatic {
    type Vtable = IStandardPrintTaskOptionsStatic_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3024633126, 3536, 19668, [186, 255, 147, 15, 199, 214, 165, 116]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardPrintTaskOptionsStatic_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStandardPrintTaskOptionsStatic2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStandardPrintTaskOptionsStatic2 {
    type Vtable = IStandardPrintTaskOptionsStatic2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1004768244, 31300, 17001, [154, 82, 129, 38, 30, 40, 158, 233]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardPrintTaskOptionsStatic2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStandardPrintTaskOptionsStatic3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStandardPrintTaskOptionsStatic3 {
    type Vtable = IStandardPrintTaskOptionsStatic3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3153497734, 14424, 16819, [167, 153, 85, 221, 152, 136, 212, 117]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardPrintTaskOptionsStatic3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Graphics_Printing`*"]
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
unsafe impl ::windows::runtime::Abi for PrintBinding {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PrintBinding {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintBinding;i4)");
}
impl ::windows::runtime::DefaultType for PrintBinding {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing`*"]
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
unsafe impl ::windows::runtime::Abi for PrintBordering {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PrintBordering {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintBordering;i4)");
}
impl ::windows::runtime::DefaultType for PrintBordering {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing`*"]
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
unsafe impl ::windows::runtime::Abi for PrintCollation {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PrintCollation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintCollation;i4)");
}
impl ::windows::runtime::DefaultType for PrintCollation {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing`*"]
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
unsafe impl ::windows::runtime::Abi for PrintColorMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PrintColorMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintColorMode;i4)");
}
impl ::windows::runtime::DefaultType for PrintColorMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing`*"]
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
unsafe impl ::windows::runtime::Abi for PrintDuplex {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PrintDuplex {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintDuplex;i4)");
}
impl ::windows::runtime::DefaultType for PrintDuplex {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing`*"]
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
unsafe impl ::windows::runtime::Abi for PrintHolePunch {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PrintHolePunch {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintHolePunch;i4)");
}
impl ::windows::runtime::DefaultType for PrintHolePunch {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintManager(pub ::windows::runtime::IInspectable);
impl PrintManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing`, `Foundation`*"]
    pub fn PrintTaskRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PrintManager, PrintTaskRequestedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing`, `Foundation`*"]
    pub fn RemovePrintTaskRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<PrintManager> {
        Self::IPrintManagerStatic(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintManager>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing`, `Foundation`*"]
    pub fn ShowPrintUIAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IPrintManagerStatic(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn IsSupported() -> ::windows::runtime::Result<bool> {
        Self::IPrintManagerStatic2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IPrintManagerStatic<R, F: FnOnce(&IPrintManagerStatic) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PrintManager, IPrintManagerStatic> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPrintManagerStatic2<R, F: FnOnce(&IPrintManagerStatic2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PrintManager, IPrintManagerStatic2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintManager;{ff2a9694-8c99-44fd-ae4a-19d9aa9a0f0a})");
}
unsafe impl ::windows::runtime::Interface for PrintManager {
    type Vtable = IPrintManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4280981140, 35993, 17661, [174, 74, 25, 217, 170, 154, 15, 10]);
}
impl ::windows::runtime::RuntimeName for PrintManager {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintManager";
}
impl ::core::convert::From<PrintManager> for ::windows::runtime::IUnknown {
    fn from(value: PrintManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintManager> for ::windows::runtime::IUnknown {
    fn from(value: &PrintManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PrintManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PrintManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintManager> for ::windows::runtime::IInspectable {
    fn from(value: PrintManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintManager> for ::windows::runtime::IInspectable {
    fn from(value: &PrintManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PrintManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PrintManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintManager {}
unsafe impl ::core::marker::Sync for PrintManager {}
#[doc = "*Required features: `Graphics_Printing`*"]
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
unsafe impl ::windows::runtime::Abi for PrintMediaSize {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PrintMediaSize {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintMediaSize;i4)");
}
impl ::windows::runtime::DefaultType for PrintMediaSize {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing`*"]
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
unsafe impl ::windows::runtime::Abi for PrintMediaType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PrintMediaType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintMediaType;i4)");
}
impl ::windows::runtime::DefaultType for PrintMediaType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing`*"]
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
unsafe impl ::windows::runtime::Abi for PrintOrientation {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PrintOrientation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintOrientation;i4)");
}
impl ::windows::runtime::DefaultType for PrintOrientation {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation")]
#[doc = "*Required features: `Graphics_Printing`, `Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for PrintPageDescription {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::RuntimeType for PrintPageDescription {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Graphics.Printing.PrintPageDescription;struct(Windows.Foundation.Size;f4;f4);struct(Windows.Foundation.Rect;f4;f4;f4;f4);u4;u4)");
}
#[cfg(feature = "Foundation")]
impl ::windows::runtime::DefaultType for PrintPageDescription {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintPageInfo(pub ::windows::runtime::IInspectable);
impl PrintPageInfo {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PrintPageInfo, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetMediaSize(&self, value: PrintMediaSize) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn MediaSize(&self) -> ::windows::runtime::Result<PrintMediaSize> {
        let this = self;
        unsafe {
            let mut result__: PrintMediaSize = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintMediaSize>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing`, `Foundation`*"]
    pub fn SetPageSize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Size>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing`, `Foundation`*"]
    pub fn PageSize(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetDpiX(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn DpiX(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetDpiY(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn DpiY(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetOrientation(&self, value: PrintOrientation) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Orientation(&self) -> ::windows::runtime::Result<PrintOrientation> {
        let this = self;
        unsafe {
            let mut result__: PrintOrientation = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOrientation>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintPageInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintPageInfo;{dd4be9c9-a6a1-4ada-930e-da872a4f23d3})");
}
unsafe impl ::windows::runtime::Interface for PrintPageInfo {
    type Vtable = IPrintPageInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3712739785, 42657, 19162, [147, 14, 218, 135, 42, 79, 35, 211]);
}
impl ::windows::runtime::RuntimeName for PrintPageInfo {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintPageInfo";
}
impl ::core::convert::From<PrintPageInfo> for ::windows::runtime::IUnknown {
    fn from(value: PrintPageInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintPageInfo> for ::windows::runtime::IUnknown {
    fn from(value: &PrintPageInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PrintPageInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PrintPageInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintPageInfo> for ::windows::runtime::IInspectable {
    fn from(value: PrintPageInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintPageInfo> for ::windows::runtime::IInspectable {
    fn from(value: &PrintPageInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PrintPageInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PrintPageInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintPageInfo {}
unsafe impl ::core::marker::Sync for PrintPageInfo {}
#[doc = "*Required features: `Graphics_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintPageRange(pub ::windows::runtime::IInspectable);
impl PrintPageRange {
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn FirstPageNumber(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn LastPageNumber(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Create(firstpage: i32, lastpage: i32) -> ::windows::runtime::Result<PrintPageRange> {
        Self::IPrintPageRangeFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), firstpage, lastpage, &mut result__).from_abi::<PrintPageRange>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn CreateWithSinglePage(page: i32) -> ::windows::runtime::Result<PrintPageRange> {
        Self::IPrintPageRangeFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), page, &mut result__).from_abi::<PrintPageRange>(result__)
        })
    }
    pub fn IPrintPageRangeFactory<R, F: FnOnce(&IPrintPageRangeFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PrintPageRange, IPrintPageRangeFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintPageRange {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintPageRange;{f8a06c54-6e7c-51c5-57fd-0660c2d71513})");
}
unsafe impl ::windows::runtime::Interface for PrintPageRange {
    type Vtable = IPrintPageRange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4171263060, 28284, 20933, [87, 253, 6, 96, 194, 215, 21, 19]);
}
impl ::windows::runtime::RuntimeName for PrintPageRange {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintPageRange";
}
impl ::core::convert::From<PrintPageRange> for ::windows::runtime::IUnknown {
    fn from(value: PrintPageRange) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintPageRange> for ::windows::runtime::IUnknown {
    fn from(value: &PrintPageRange) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PrintPageRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PrintPageRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintPageRange> for ::windows::runtime::IInspectable {
    fn from(value: PrintPageRange) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintPageRange> for ::windows::runtime::IInspectable {
    fn from(value: &PrintPageRange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PrintPageRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PrintPageRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintPageRange {}
unsafe impl ::core::marker::Sync for PrintPageRange {}
#[doc = "*Required features: `Graphics_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintPageRangeOptions(pub ::windows::runtime::IInspectable);
impl PrintPageRangeOptions {
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetAllowAllPages(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn AllowAllPages(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetAllowCurrentPage(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn AllowCurrentPage(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetAllowCustomSetOfPages(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn AllowCustomSetOfPages(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintPageRangeOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintPageRangeOptions;{ce6db728-1357-46b2-a923-79f995f448fc})");
}
unsafe impl ::windows::runtime::Interface for PrintPageRangeOptions {
    type Vtable = IPrintPageRangeOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3463296808, 4951, 18098, [169, 35, 121, 249, 149, 244, 72, 252]);
}
impl ::windows::runtime::RuntimeName for PrintPageRangeOptions {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintPageRangeOptions";
}
impl ::core::convert::From<PrintPageRangeOptions> for ::windows::runtime::IUnknown {
    fn from(value: PrintPageRangeOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintPageRangeOptions> for ::windows::runtime::IUnknown {
    fn from(value: &PrintPageRangeOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PrintPageRangeOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PrintPageRangeOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintPageRangeOptions> for ::windows::runtime::IInspectable {
    fn from(value: PrintPageRangeOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintPageRangeOptions> for ::windows::runtime::IInspectable {
    fn from(value: &PrintPageRangeOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PrintPageRangeOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PrintPageRangeOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintPageRangeOptions {}
unsafe impl ::core::marker::Sync for PrintPageRangeOptions {}
#[doc = "*Required features: `Graphics_Printing`*"]
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
unsafe impl ::windows::runtime::Abi for PrintQuality {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PrintQuality {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintQuality;i4)");
}
impl ::windows::runtime::DefaultType for PrintQuality {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing`*"]
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
unsafe impl ::windows::runtime::Abi for PrintStaple {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PrintStaple {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintStaple;i4)");
}
impl ::windows::runtime::DefaultType for PrintStaple {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTask(pub ::windows::runtime::IInspectable);
impl PrintTask {
    #[cfg(feature = "ApplicationModel_DataTransfer")]
    #[doc = "*Required features: `Graphics_Printing`, `ApplicationModel_DataTransfer`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::ApplicationModel::DataTransfer::DataPackagePropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::ApplicationModel::DataTransfer::DataPackagePropertySet>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Source(&self) -> ::windows::runtime::Result<IPrintDocumentSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IPrintDocumentSource>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Options(&self) -> ::windows::runtime::Result<PrintTaskOptions> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTaskOptions>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing`, `Foundation`*"]
    pub fn Previewing<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PrintTask, ::windows::runtime::IInspectable>>>(&self, eventhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing`, `Foundation`*"]
    pub fn RemovePreviewing<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing`, `Foundation`*"]
    pub fn Submitting<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PrintTask, ::windows::runtime::IInspectable>>>(&self, eventhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing`, `Foundation`*"]
    pub fn RemoveSubmitting<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing`, `Foundation`*"]
    pub fn Progressing<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PrintTask, PrintTaskProgressingEventArgs>>>(&self, eventhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing`, `Foundation`*"]
    pub fn RemoveProgressing<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing`, `Foundation`*"]
    pub fn Completed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PrintTask, PrintTaskCompletedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing`, `Foundation`*"]
    pub fn RemoveCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetIsPrinterTargetEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskTargetDeviceSupport>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn IsPrinterTargetEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskTargetDeviceSupport>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetIs3DManufacturingTargetEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskTargetDeviceSupport>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Is3DManufacturingTargetEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskTargetDeviceSupport>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetIsPreviewEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPrintTask2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn IsPreviewEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IPrintTask2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintTask {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTask;{61d80247-6cf6-4fad-84e2-a5e82e2d4ceb})");
}
unsafe impl ::windows::runtime::Interface for PrintTask {
    type Vtable = IPrintTask_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1641546311, 27894, 20397, [132, 226, 165, 232, 46, 45, 76, 235]);
}
impl ::windows::runtime::RuntimeName for PrintTask {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTask";
}
impl ::core::convert::From<PrintTask> for ::windows::runtime::IUnknown {
    fn from(value: PrintTask) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintTask> for ::windows::runtime::IUnknown {
    fn from(value: &PrintTask) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PrintTask {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PrintTask {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintTask> for ::windows::runtime::IInspectable {
    fn from(value: PrintTask) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintTask> for ::windows::runtime::IInspectable {
    fn from(value: &PrintTask) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PrintTask {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PrintTask {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintTask {}
unsafe impl ::core::marker::Sync for PrintTask {}
#[doc = "*Required features: `Graphics_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTaskCompletedEventArgs(pub ::windows::runtime::IInspectable);
impl PrintTaskCompletedEventArgs {
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Completion(&self) -> ::windows::runtime::Result<PrintTaskCompletion> {
        let this = self;
        unsafe {
            let mut result__: PrintTaskCompletion = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTaskCompletion>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintTaskCompletedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskCompletedEventArgs;{5bcd34af-24e9-4c10-8d07-14c346ba3fce})");
}
unsafe impl ::windows::runtime::Interface for PrintTaskCompletedEventArgs {
    type Vtable = IPrintTaskCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1540175023, 9449, 19472, [141, 7, 20, 195, 70, 186, 63, 206]);
}
impl ::windows::runtime::RuntimeName for PrintTaskCompletedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskCompletedEventArgs";
}
impl ::core::convert::From<PrintTaskCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: PrintTaskCompletedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintTaskCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &PrintTaskCompletedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PrintTaskCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PrintTaskCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintTaskCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: PrintTaskCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintTaskCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &PrintTaskCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PrintTaskCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PrintTaskCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintTaskCompletedEventArgs {}
unsafe impl ::core::marker::Sync for PrintTaskCompletedEventArgs {}
#[doc = "*Required features: `Graphics_Printing`*"]
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
unsafe impl ::windows::runtime::Abi for PrintTaskCompletion {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PrintTaskCompletion {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintTaskCompletion;i4)");
}
impl ::windows::runtime::DefaultType for PrintTaskCompletion {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTaskOptions(pub ::windows::runtime::IInspectable);
impl PrintTaskOptions {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing`, `Foundation`*"]
    pub fn GetPageDescription(&self, jobpagenumber: u32) -> ::windows::runtime::Result<PrintPageDescription> {
        let this = self;
        unsafe {
            let mut result__: PrintPageDescription = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), jobpagenumber, &mut result__).from_abi::<PrintPageDescription>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetMediaSize(&self, value: PrintMediaSize) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn MediaSize(&self) -> ::windows::runtime::Result<PrintMediaSize> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: PrintMediaSize = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintMediaSize>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetMediaType(&self, value: PrintMediaType) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn MediaType(&self) -> ::windows::runtime::Result<PrintMediaType> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: PrintMediaType = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintMediaType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetOrientation(&self, value: PrintOrientation) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Orientation(&self) -> ::windows::runtime::Result<PrintOrientation> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: PrintOrientation = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOrientation>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetPrintQuality(&self, value: PrintQuality) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn PrintQuality(&self) -> ::windows::runtime::Result<PrintQuality> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: PrintQuality = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintQuality>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetColorMode(&self, value: PrintColorMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn ColorMode(&self) -> ::windows::runtime::Result<PrintColorMode> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: PrintColorMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintColorMode>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetDuplex(&self, value: PrintDuplex) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Duplex(&self) -> ::windows::runtime::Result<PrintDuplex> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: PrintDuplex = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintDuplex>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetCollation(&self, value: PrintCollation) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Collation(&self) -> ::windows::runtime::Result<PrintCollation> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: PrintCollation = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintCollation>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetStaple(&self, value: PrintStaple) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Staple(&self) -> ::windows::runtime::Result<PrintStaple> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: PrintStaple = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintStaple>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetHolePunch(&self, value: PrintHolePunch) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn HolePunch(&self) -> ::windows::runtime::Result<PrintHolePunch> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: PrintHolePunch = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintHolePunch>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetBinding(&self, value: PrintBinding) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Binding(&self) -> ::windows::runtime::Result<PrintBinding> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: PrintBinding = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintBinding>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn MinCopies(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn MaxCopies(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetNumberOfCopies(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn NumberOfCopies(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreProperties>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing`, `Foundation_Collections`*"]
    pub fn DisplayedOptions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptionsCoreUIConfiguration>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetBordering(&self, value: PrintBordering) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptions>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Bordering(&self) -> ::windows::runtime::Result<PrintBordering> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptions>(self)?;
        unsafe {
            let mut result__: PrintBordering = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintBordering>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Graphics_Printing`, `Storage_Streams`*"]
    pub fn GetPagePrintTicket<'a, Param0: ::windows::runtime::IntoParam<'a, PrintPageInfo>>(&self, printpageinfo: Param0) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptions>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), printpageinfo.into_param().abi(), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn PageRangeOptions(&self) -> ::windows::runtime::Result<PrintPageRangeOptions> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptions2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintPageRangeOptions>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing`, `Foundation_Collections`*"]
    pub fn CustomPageRanges(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<PrintPageRange>> {
        let this = &::windows::runtime::Interface::cast::<IPrintTaskOptions2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<PrintPageRange>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintTaskOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskOptions;{1bdbb474-4ed1-41eb-be3c-72d18ed67337})");
}
unsafe impl ::windows::runtime::Interface for PrintTaskOptions {
    type Vtable = IPrintTaskOptionsCore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(467383412, 20177, 16875, [190, 60, 114, 209, 142, 214, 115, 55]);
}
impl ::windows::runtime::RuntimeName for PrintTaskOptions {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskOptions";
}
impl ::core::convert::From<PrintTaskOptions> for ::windows::runtime::IUnknown {
    fn from(value: PrintTaskOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintTaskOptions> for ::windows::runtime::IUnknown {
    fn from(value: &PrintTaskOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PrintTaskOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PrintTaskOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintTaskOptions> for ::windows::runtime::IInspectable {
    fn from(value: PrintTaskOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintTaskOptions> for ::windows::runtime::IInspectable {
    fn from(value: &PrintTaskOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PrintTaskOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PrintTaskOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IPrintTaskOptionsCore> for PrintTaskOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPrintTaskOptionsCore> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPrintTaskOptionsCore> for &PrintTaskOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPrintTaskOptionsCore> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PrintTaskOptions> for IPrintTaskOptionsCoreProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PrintTaskOptions) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintTaskOptions> for IPrintTaskOptionsCoreProperties {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PrintTaskOptions) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPrintTaskOptionsCoreProperties> for PrintTaskOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPrintTaskOptionsCoreProperties> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPrintTaskOptionsCoreProperties> for &PrintTaskOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPrintTaskOptionsCoreProperties> {
        ::core::convert::TryInto::<IPrintTaskOptionsCoreProperties>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<PrintTaskOptions> for IPrintTaskOptionsCoreUIConfiguration {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PrintTaskOptions) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintTaskOptions> for IPrintTaskOptionsCoreUIConfiguration {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PrintTaskOptions) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPrintTaskOptionsCoreUIConfiguration> for PrintTaskOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPrintTaskOptionsCoreUIConfiguration> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPrintTaskOptionsCoreUIConfiguration> for &PrintTaskOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPrintTaskOptionsCoreUIConfiguration> {
        ::core::convert::TryInto::<IPrintTaskOptionsCoreUIConfiguration>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintTaskOptions {}
unsafe impl ::core::marker::Sync for PrintTaskOptions {}
#[doc = "*Required features: `Graphics_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTaskProgressingEventArgs(pub ::windows::runtime::IInspectable);
impl PrintTaskProgressingEventArgs {
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn DocumentPageCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintTaskProgressingEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskProgressingEventArgs;{810cd3cb-b410-4282-a073-5ac378234174})");
}
unsafe impl ::windows::runtime::Interface for PrintTaskProgressingEventArgs {
    type Vtable = IPrintTaskProgressingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2165101515, 46096, 17026, [160, 115, 90, 195, 120, 35, 65, 116]);
}
impl ::windows::runtime::RuntimeName for PrintTaskProgressingEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskProgressingEventArgs";
}
impl ::core::convert::From<PrintTaskProgressingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: PrintTaskProgressingEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintTaskProgressingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &PrintTaskProgressingEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PrintTaskProgressingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PrintTaskProgressingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintTaskProgressingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: PrintTaskProgressingEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintTaskProgressingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &PrintTaskProgressingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PrintTaskProgressingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PrintTaskProgressingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintTaskProgressingEventArgs {}
unsafe impl ::core::marker::Sync for PrintTaskProgressingEventArgs {}
#[doc = "*Required features: `Graphics_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTaskRequest(pub ::windows::runtime::IInspectable);
impl PrintTaskRequest {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing`, `Foundation`*"]
    pub fn Deadline(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn CreatePrintTask<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, PrintTaskSourceRequestedHandler>>(&self, title: Param0, handler: Param1) -> ::windows::runtime::Result<PrintTask> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), title.into_param().abi(), handler.into_param().abi(), &mut result__).from_abi::<PrintTask>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<PrintTaskRequestedDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTaskRequestedDeferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintTaskRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskRequest;{6ff61e2e-2722-4240-a67c-f364849a17f3})");
}
unsafe impl ::windows::runtime::Interface for PrintTaskRequest {
    type Vtable = IPrintTaskRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1878400558, 10018, 16960, [166, 124, 243, 100, 132, 154, 23, 243]);
}
impl ::windows::runtime::RuntimeName for PrintTaskRequest {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskRequest";
}
impl ::core::convert::From<PrintTaskRequest> for ::windows::runtime::IUnknown {
    fn from(value: PrintTaskRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintTaskRequest> for ::windows::runtime::IUnknown {
    fn from(value: &PrintTaskRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PrintTaskRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PrintTaskRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintTaskRequest> for ::windows::runtime::IInspectable {
    fn from(value: PrintTaskRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintTaskRequest> for ::windows::runtime::IInspectable {
    fn from(value: &PrintTaskRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PrintTaskRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PrintTaskRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintTaskRequest {}
unsafe impl ::core::marker::Sync for PrintTaskRequest {}
#[doc = "*Required features: `Graphics_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTaskRequestedDeferral(pub ::windows::runtime::IInspectable);
impl PrintTaskRequestedDeferral {
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintTaskRequestedDeferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskRequestedDeferral;{cfefb3f0-ce3e-42c7-9496-64800c622c44})");
}
unsafe impl ::windows::runtime::Interface for PrintTaskRequestedDeferral {
    type Vtable = IPrintTaskRequestedDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3488592880, 52798, 17095, [148, 150, 100, 128, 12, 98, 44, 68]);
}
impl ::windows::runtime::RuntimeName for PrintTaskRequestedDeferral {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskRequestedDeferral";
}
impl ::core::convert::From<PrintTaskRequestedDeferral> for ::windows::runtime::IUnknown {
    fn from(value: PrintTaskRequestedDeferral) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintTaskRequestedDeferral> for ::windows::runtime::IUnknown {
    fn from(value: &PrintTaskRequestedDeferral) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PrintTaskRequestedDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PrintTaskRequestedDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintTaskRequestedDeferral> for ::windows::runtime::IInspectable {
    fn from(value: PrintTaskRequestedDeferral) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintTaskRequestedDeferral> for ::windows::runtime::IInspectable {
    fn from(value: &PrintTaskRequestedDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PrintTaskRequestedDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PrintTaskRequestedDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintTaskRequestedDeferral {}
unsafe impl ::core::marker::Sync for PrintTaskRequestedDeferral {}
#[doc = "*Required features: `Graphics_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTaskRequestedEventArgs(pub ::windows::runtime::IInspectable);
impl PrintTaskRequestedEventArgs {
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<PrintTaskRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTaskRequest>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintTaskRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskRequestedEventArgs;{d0aff924-a31b-454c-a7b6-5d0cc522fc16})");
}
unsafe impl ::windows::runtime::Interface for PrintTaskRequestedEventArgs {
    type Vtable = IPrintTaskRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3501193508, 41755, 17740, [167, 182, 93, 12, 197, 34, 252, 22]);
}
impl ::windows::runtime::RuntimeName for PrintTaskRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskRequestedEventArgs";
}
impl ::core::convert::From<PrintTaskRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: PrintTaskRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintTaskRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &PrintTaskRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PrintTaskRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PrintTaskRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintTaskRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: PrintTaskRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintTaskRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &PrintTaskRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PrintTaskRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PrintTaskRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintTaskRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PrintTaskRequestedEventArgs {}
#[doc = "*Required features: `Graphics_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTaskSourceRequestedArgs(pub ::windows::runtime::IInspectable);
impl PrintTaskSourceRequestedArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing`, `Foundation`*"]
    pub fn Deadline(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn SetSource<'a, Param0: ::windows::runtime::IntoParam<'a, IPrintDocumentSource>>(&self, source: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), source.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<PrintTaskSourceRequestedDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTaskSourceRequestedDeferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintTaskSourceRequestedArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskSourceRequestedArgs;{f9f067be-f456-41f0-9c98-5ce73e851410})");
}
unsafe impl ::windows::runtime::Interface for PrintTaskSourceRequestedArgs {
    type Vtable = IPrintTaskSourceRequestedArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4193281982, 62550, 16880, [156, 152, 92, 231, 62, 133, 20, 16]);
}
impl ::windows::runtime::RuntimeName for PrintTaskSourceRequestedArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskSourceRequestedArgs";
}
impl ::core::convert::From<PrintTaskSourceRequestedArgs> for ::windows::runtime::IUnknown {
    fn from(value: PrintTaskSourceRequestedArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintTaskSourceRequestedArgs> for ::windows::runtime::IUnknown {
    fn from(value: &PrintTaskSourceRequestedArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PrintTaskSourceRequestedArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PrintTaskSourceRequestedArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintTaskSourceRequestedArgs> for ::windows::runtime::IInspectable {
    fn from(value: PrintTaskSourceRequestedArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintTaskSourceRequestedArgs> for ::windows::runtime::IInspectable {
    fn from(value: &PrintTaskSourceRequestedArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PrintTaskSourceRequestedArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PrintTaskSourceRequestedArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintTaskSourceRequestedArgs {}
unsafe impl ::core::marker::Sync for PrintTaskSourceRequestedArgs {}
#[doc = "*Required features: `Graphics_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTaskSourceRequestedDeferral(pub ::windows::runtime::IInspectable);
impl PrintTaskSourceRequestedDeferral {
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintTaskSourceRequestedDeferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTaskSourceRequestedDeferral;{4a1560d1-6992-4d9d-8555-4ca4563fb166})");
}
unsafe impl ::windows::runtime::Interface for PrintTaskSourceRequestedDeferral {
    type Vtable = IPrintTaskSourceRequestedDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1242915025, 27026, 19869, [133, 85, 76, 164, 86, 63, 177, 102]);
}
impl ::windows::runtime::RuntimeName for PrintTaskSourceRequestedDeferral {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTaskSourceRequestedDeferral";
}
impl ::core::convert::From<PrintTaskSourceRequestedDeferral> for ::windows::runtime::IUnknown {
    fn from(value: PrintTaskSourceRequestedDeferral) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintTaskSourceRequestedDeferral> for ::windows::runtime::IUnknown {
    fn from(value: &PrintTaskSourceRequestedDeferral) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PrintTaskSourceRequestedDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PrintTaskSourceRequestedDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintTaskSourceRequestedDeferral> for ::windows::runtime::IInspectable {
    fn from(value: PrintTaskSourceRequestedDeferral) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintTaskSourceRequestedDeferral> for ::windows::runtime::IInspectable {
    fn from(value: &PrintTaskSourceRequestedDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PrintTaskSourceRequestedDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PrintTaskSourceRequestedDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintTaskSourceRequestedDeferral {}
unsafe impl ::core::marker::Sync for PrintTaskSourceRequestedDeferral {}
#[doc = "*Required features: `Graphics_Printing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTaskSourceRequestedHandler(::windows::runtime::IUnknown);
impl PrintTaskSourceRequestedHandler {
    pub fn new<F: FnMut(&::core::option::Option<PrintTaskSourceRequestedArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        unsafe {
            let object = ::windows::runtime::heap_alloc(core::mem::size_of::<PrintTaskSourceRequestedHandler_box<F>>()).expect("Could not successfully allocate delegate") as *mut PrintTaskSourceRequestedHandler_box<F>;
            *object = PrintTaskSourceRequestedHandler_box::<F> {
                vtable: &PrintTaskSourceRequestedHandler_box::<F>::VTABLE,
                count: ::windows::runtime::RefCount::new(1),
                invoke,
            };
            core::mem::transmute(object)
        }
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, PrintTaskSourceRequestedArgs>>(&self, args: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::core::mem::transmute_copy(this), args.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintTaskSourceRequestedHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({6c109fa8-5cb6-4b3a-8663-f39cb02dc9b4})");
}
unsafe impl ::windows::runtime::Interface for PrintTaskSourceRequestedHandler {
    type Vtable = PrintTaskSourceRequestedHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1813028776, 23734, 19258, [134, 99, 243, 156, 176, 45, 201, 180]);
}
#[repr(C)]
#[doc(hidden)]
pub struct PrintTaskSourceRequestedHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, args: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct PrintTaskSourceRequestedHandler_box<F: FnMut(&::core::option::Option<PrintTaskSourceRequestedArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const PrintTaskSourceRequestedHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::core::option::Option<PrintTaskSourceRequestedArgs>) -> ::windows::runtime::Result<()> + 'static> PrintTaskSourceRequestedHandler_box<F> {
    const VTABLE: PrintTaskSourceRequestedHandler_abi = PrintTaskSourceRequestedHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<PrintTaskSourceRequestedHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(ptr: ::windows::runtime::RawPtr) -> u32 {
        let this = ptr as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::runtime::heap_free(ptr);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, args: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&args as *const <PrintTaskSourceRequestedArgs as ::windows::runtime::Abi>::Abi as *const <PrintTaskSourceRequestedArgs as ::windows::runtime::DefaultType>::DefaultType)).into()
    }
}
#[doc = "*Required features: `Graphics_Printing`*"]
pub struct StandardPrintTaskOptions {}
impl StandardPrintTaskOptions {
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn MediaSize() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn MediaType() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Orientation() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn PrintQuality() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn ColorMode() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Duplex() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Collation() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Staple() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn HolePunch() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Binding() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Copies() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn NUp() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn InputBin() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn Bordering() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Printing`*"]
    pub fn CustomPageRanges() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStandardPrintTaskOptionsStatic3(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IStandardPrintTaskOptionsStatic<R, F: FnOnce(&IStandardPrintTaskOptionsStatic) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<StandardPrintTaskOptions, IStandardPrintTaskOptionsStatic> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IStandardPrintTaskOptionsStatic2<R, F: FnOnce(&IStandardPrintTaskOptionsStatic2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<StandardPrintTaskOptions, IStandardPrintTaskOptionsStatic2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IStandardPrintTaskOptionsStatic3<R, F: FnOnce(&IStandardPrintTaskOptionsStatic3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<StandardPrintTaskOptions, IStandardPrintTaskOptionsStatic3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for StandardPrintTaskOptions {
    const NAME: &'static str = "Windows.Graphics.Printing.StandardPrintTaskOptions";
}
