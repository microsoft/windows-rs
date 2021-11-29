#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct INDClient(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INDClient {
    type Vtable = INDClient_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bd6781b_61b8_46e2_99a5_8abcb6b9f7d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct INDClient_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, contenturl: ::windows::core::RawPtr, startasyncoptions: u32, registrationcustomdata: ::windows::core::RawPtr, licensefetchdescriptor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, licensefetchdescriptor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, registrationcustomdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INDClientFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INDClientFactory {
    type Vtable = INDClientFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e53dd62_fee8_451f_b0d4_f706cca3e037);
}
#[repr(C)]
#[doc(hidden)]
pub struct INDClientFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, downloadengine: ::windows::core::RawPtr, streamparser: ::windows::core::RawPtr, pmessenger: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INDClosedCaptionDataReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INDClosedCaptionDataReceivedEventArgs {
    type Vtable = INDClosedCaptionDataReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4738d29f_c345_4649_8468_b8c5fc357190);
}
impl INDClosedCaptionDataReceivedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn ClosedCaptionDataFormat(&self) -> ::windows::core::Result<NDClosedCaptionFormat> {
        let this = self;
        unsafe {
            let mut result__: NDClosedCaptionFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NDClosedCaptionFormat>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn PresentationTimestamp(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ClosedCaptionData(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for INDClosedCaptionDataReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4738d29f-c345-4649-8468-b8c5fc357190}");
}
impl ::core::convert::From<INDClosedCaptionDataReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: INDClosedCaptionDataReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INDClosedCaptionDataReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &INDClosedCaptionDataReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INDClosedCaptionDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INDClosedCaptionDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INDClosedCaptionDataReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: INDClosedCaptionDataReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INDClosedCaptionDataReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &INDClosedCaptionDataReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INDClosedCaptionDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INDClosedCaptionDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDClosedCaptionDataReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut NDClosedCaptionFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INDCustomData(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INDCustomData {
    type Vtable = INDCustomData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5cb0fdc_2d09_4f19_b5e1_76a0b3ee9267);
}
impl INDCustomData {
    #[cfg(feature = "deprecated")]
    pub fn CustomDataTypeID(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn CustomData(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for INDCustomData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f5cb0fdc-2d09-4f19-b5e1-76a0b3ee9267}");
}
impl ::core::convert::From<INDCustomData> for ::windows::core::IUnknown {
    fn from(value: INDCustomData) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INDCustomData> for ::windows::core::IUnknown {
    fn from(value: &INDCustomData) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INDCustomData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INDCustomData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INDCustomData> for ::windows::core::IInspectable {
    fn from(value: INDCustomData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INDCustomData> for ::windows::core::IInspectable {
    fn from(value: &INDCustomData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INDCustomData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INDCustomData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDCustomData_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INDCustomDataFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INDCustomDataFactory {
    type Vtable = INDCustomDataFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd65405ab_3424_4833_8c9a_af5fdeb22872);
}
#[repr(C)]
#[doc(hidden)]
pub struct INDCustomDataFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, customDataTypeIDBytes_array_size: u32, customdatatypeidbytes: *const u8, customDataBytes_array_size: u32, customdatabytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INDDownloadEngine(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INDDownloadEngine {
    type Vtable = INDDownloadEngine_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d223d65_c4b6_4438_8d46_b96e6d0fb21f);
}
impl INDDownloadEngine {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn Open<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, uri: Param0, sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), uri.into_param().abi(), sessionidbytes.len() as u32, ::core::mem::transmute(sessionidbytes.as_ptr())).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn Pause(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn Resume(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn Seek<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, startposition: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), startposition.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn CanSeek(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn BufferFullMinThresholdInSamples(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn BufferFullMaxThresholdInSamples(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Notifier(&self) -> ::windows::core::Result<NDDownloadEngineNotifier> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NDDownloadEngineNotifier>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for INDDownloadEngine {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2d223d65-c4b6-4438-8d46-b96e6d0fb21f}");
}
impl ::core::convert::From<INDDownloadEngine> for ::windows::core::IUnknown {
    fn from(value: INDDownloadEngine) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INDDownloadEngine> for ::windows::core::IUnknown {
    fn from(value: &INDDownloadEngine) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INDDownloadEngine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INDDownloadEngine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INDDownloadEngine> for ::windows::core::IInspectable {
    fn from(value: INDDownloadEngine) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INDDownloadEngine> for ::windows::core::IInspectable {
    fn from(value: &INDDownloadEngine) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INDDownloadEngine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INDDownloadEngine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDDownloadEngine_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, sessionIDBytes_array_size: u32, sessionidbytes: *const u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, startposition: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INDDownloadEngineNotifier(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INDDownloadEngineNotifier {
    type Vtable = INDDownloadEngineNotifier_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd720b4d4_f4b8_4530_a809_9193a571e7fc);
}
impl INDDownloadEngineNotifier {
    #[cfg(feature = "deprecated")]
    pub fn OnStreamOpened(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn OnPlayReadyObjectReceived(&self, databytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), databytes.len() as u32, ::core::mem::transmute(databytes.as_ptr())).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn OnContentIDReceived<'a, Param0: ::windows::core::IntoParam<'a, INDLicenseFetchDescriptor>>(&self, licensefetchdescriptor: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), licensefetchdescriptor.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn OnDataReceived(&self, databytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], bytesreceived: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), databytes.len() as u32, ::core::mem::transmute(databytes.as_ptr()), bytesreceived).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn OnEndOfStream(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn OnNetworkError(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for INDDownloadEngineNotifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d720b4d4-f4b8-4530-a809-9193a571e7fc}");
}
impl ::core::convert::From<INDDownloadEngineNotifier> for ::windows::core::IUnknown {
    fn from(value: INDDownloadEngineNotifier) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INDDownloadEngineNotifier> for ::windows::core::IUnknown {
    fn from(value: &INDDownloadEngineNotifier) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INDDownloadEngineNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INDDownloadEngineNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INDDownloadEngineNotifier> for ::windows::core::IInspectable {
    fn from(value: INDDownloadEngineNotifier) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INDDownloadEngineNotifier> for ::windows::core::IInspectable {
    fn from(value: &INDDownloadEngineNotifier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INDDownloadEngineNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INDDownloadEngineNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDDownloadEngineNotifier_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dataBytes_array_size: u32, databytes: *const u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, licensefetchdescriptor: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dataBytes_array_size: u32, databytes: *const u8, bytesreceived: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INDLicenseFetchCompletedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INDLicenseFetchCompletedEventArgs {
    type Vtable = INDLicenseFetchCompletedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ee30a1a_11b2_4558_8865_e3a516922517);
}
impl INDLicenseFetchCompletedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<INDCustomData>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for INDLicenseFetchCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1ee30a1a-11b2-4558-8865-e3a516922517}");
}
impl ::core::convert::From<INDLicenseFetchCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: INDLicenseFetchCompletedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INDLicenseFetchCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &INDLicenseFetchCompletedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INDLicenseFetchCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INDLicenseFetchCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INDLicenseFetchCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: INDLicenseFetchCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INDLicenseFetchCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &INDLicenseFetchCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INDLicenseFetchCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INDLicenseFetchCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INDLicenseFetchDescriptor(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INDLicenseFetchDescriptor {
    type Vtable = INDLicenseFetchDescriptor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5498d33a_e686_4935_a567_7ca77ad20fa4);
}
impl INDLicenseFetchDescriptor {
    #[cfg(feature = "deprecated")]
    pub fn ContentIDType(&self) -> ::windows::core::Result<NDContentIDType> {
        let this = self;
        unsafe {
            let mut result__: NDContentIDType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NDContentIDType>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ContentID(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn LicenseFetchChallengeCustomData(&self) -> ::windows::core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<INDCustomData>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetLicenseFetchChallengeCustomData<'a, Param0: ::windows::core::IntoParam<'a, INDCustomData>>(&self, licensefetchchallengecustomdata: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), licensefetchchallengecustomdata.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for INDLicenseFetchDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5498d33a-e686-4935-a567-7ca77ad20fa4}");
}
impl ::core::convert::From<INDLicenseFetchDescriptor> for ::windows::core::IUnknown {
    fn from(value: INDLicenseFetchDescriptor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INDLicenseFetchDescriptor> for ::windows::core::IUnknown {
    fn from(value: &INDLicenseFetchDescriptor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INDLicenseFetchDescriptor> for ::windows::core::IInspectable {
    fn from(value: INDLicenseFetchDescriptor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INDLicenseFetchDescriptor> for ::windows::core::IInspectable {
    fn from(value: &INDLicenseFetchDescriptor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchDescriptor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut NDContentIDType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, licensefetchchallengecustomdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INDLicenseFetchDescriptorFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INDLicenseFetchDescriptorFactory {
    type Vtable = INDLicenseFetchDescriptorFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0031202_cfac_4f00_ae6a_97af80b848f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchDescriptorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, contentidtype: NDContentIDType, contentIDBytes_array_size: u32, contentidbytes: *const u8, licensefetchchallengecustomdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INDLicenseFetchResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INDLicenseFetchResult {
    type Vtable = INDLicenseFetchResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21d39698_aa62_45ff_a5ff_8037e5433825);
}
impl INDLicenseFetchResult {
    #[cfg(feature = "deprecated")]
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<INDCustomData>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for INDLicenseFetchResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{21d39698-aa62-45ff-a5ff-8037e5433825}");
}
impl ::core::convert::From<INDLicenseFetchResult> for ::windows::core::IUnknown {
    fn from(value: INDLicenseFetchResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INDLicenseFetchResult> for ::windows::core::IUnknown {
    fn from(value: &INDLicenseFetchResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INDLicenseFetchResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INDLicenseFetchResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INDLicenseFetchResult> for ::windows::core::IInspectable {
    fn from(value: INDLicenseFetchResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INDLicenseFetchResult> for ::windows::core::IInspectable {
    fn from(value: &INDLicenseFetchResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INDLicenseFetchResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INDLicenseFetchResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INDMessenger(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INDMessenger {
    type Vtable = INDMessenger_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd42df95d_a75b_47bf_8249_bc83820da38a);
}
impl INDMessenger {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn SendRegistrationRequestAsync(&self, sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], challengedatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), sessionidbytes.len() as u32, ::core::mem::transmute(sessionidbytes.as_ptr()), challengedatabytes.len() as u32, ::core::mem::transmute(challengedatabytes.as_ptr()), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn SendProximityDetectionStartAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], challengedatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), pdtype, transmitterchannelbytes.len() as u32, ::core::mem::transmute(transmitterchannelbytes.as_ptr()), sessionidbytes.len() as u32, ::core::mem::transmute(sessionidbytes.as_ptr()), challengedatabytes.len() as u32, ::core::mem::transmute(challengedatabytes.as_ptr()), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn SendProximityDetectionResponseAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], responsedatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), pdtype, transmitterchannelbytes.len() as u32, ::core::mem::transmute(transmitterchannelbytes.as_ptr()), sessionidbytes.len() as u32, ::core::mem::transmute(sessionidbytes.as_ptr()), responsedatabytes.len() as u32, ::core::mem::transmute(responsedatabytes.as_ptr()), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn SendLicenseFetchRequestAsync(&self, sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], challengedatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), sessionidbytes.len() as u32, ::core::mem::transmute(sessionidbytes.as_ptr()), challengedatabytes.len() as u32, ::core::mem::transmute(challengedatabytes.as_ptr()), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for INDMessenger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d42df95d-a75b-47bf-8249-bc83820da38a}");
}
impl ::core::convert::From<INDMessenger> for ::windows::core::IUnknown {
    fn from(value: INDMessenger) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INDMessenger> for ::windows::core::IUnknown {
    fn from(value: &INDMessenger) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INDMessenger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INDMessenger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INDMessenger> for ::windows::core::IInspectable {
    fn from(value: INDMessenger) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INDMessenger> for ::windows::core::IInspectable {
    fn from(value: &INDMessenger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INDMessenger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INDMessenger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDMessenger_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdtype: NDProximityDetectionType, transmitterChannelBytes_array_size: u32, transmitterchannelbytes: *const u8, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdtype: NDProximityDetectionType, transmitterChannelBytes_array_size: u32, transmitterchannelbytes: *const u8, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, responseDataBytes_array_size: u32, responsedatabytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INDProximityDetectionCompletedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INDProximityDetectionCompletedEventArgs {
    type Vtable = INDProximityDetectionCompletedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a706328_da25_4f8c_9eb7_5d0fc3658bca);
}
impl INDProximityDetectionCompletedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn ProximityDetectionRetryCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for INDProximityDetectionCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2a706328-da25-4f8c-9eb7-5d0fc3658bca}");
}
impl ::core::convert::From<INDProximityDetectionCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: INDProximityDetectionCompletedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INDProximityDetectionCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &INDProximityDetectionCompletedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INDProximityDetectionCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INDProximityDetectionCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INDProximityDetectionCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: INDProximityDetectionCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INDProximityDetectionCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &INDProximityDetectionCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INDProximityDetectionCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INDProximityDetectionCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDProximityDetectionCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INDRegistrationCompletedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INDRegistrationCompletedEventArgs {
    type Vtable = INDRegistrationCompletedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e39b64d_ab5b_4905_acdc_787a77c6374d);
}
impl INDRegistrationCompletedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<INDCustomData>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn TransmitterProperties(&self) -> ::windows::core::Result<INDTransmitterProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<INDTransmitterProperties>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn TransmitterCertificateAccepted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetTransmitterCertificateAccepted(&self, accept: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), accept).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for INDRegistrationCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{9e39b64d-ab5b-4905-acdc-787a77c6374d}");
}
impl ::core::convert::From<INDRegistrationCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: INDRegistrationCompletedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INDRegistrationCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &INDRegistrationCompletedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INDRegistrationCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INDRegistrationCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INDRegistrationCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: INDRegistrationCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INDRegistrationCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &INDRegistrationCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INDRegistrationCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INDRegistrationCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDRegistrationCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, accept: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INDSendResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INDSendResult {
    type Vtable = INDSendResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3685517_a584_479d_90b7_d689c7bf7c80);
}
impl INDSendResult {
    #[cfg(feature = "deprecated")]
    pub fn Response(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for INDSendResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e3685517-a584-479d-90b7-d689c7bf7c80}");
}
impl ::core::convert::From<INDSendResult> for ::windows::core::IUnknown {
    fn from(value: INDSendResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INDSendResult> for ::windows::core::IUnknown {
    fn from(value: &INDSendResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INDSendResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INDSendResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INDSendResult> for ::windows::core::IInspectable {
    fn from(value: INDSendResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INDSendResult> for ::windows::core::IInspectable {
    fn from(value: &INDSendResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INDSendResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INDSendResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDSendResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INDStartResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INDStartResult {
    type Vtable = INDStartResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79f6e96e_f50f_4015_8ba4_c2bc344ebd4e);
}
impl INDStartResult {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Media_Core")]
    pub fn MediaStreamSource(&self) -> ::windows::core::Result<super::super::Core::MediaStreamSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::MediaStreamSource>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for INDStartResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{79f6e96e-f50f-4015-8ba4-c2bc344ebd4e}");
}
impl ::core::convert::From<INDStartResult> for ::windows::core::IUnknown {
    fn from(value: INDStartResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INDStartResult> for ::windows::core::IUnknown {
    fn from(value: &INDStartResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INDStartResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INDStartResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INDStartResult> for ::windows::core::IInspectable {
    fn from(value: INDStartResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INDStartResult> for ::windows::core::IInspectable {
    fn from(value: &INDStartResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INDStartResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INDStartResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDStartResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INDStorageFileHelper(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INDStorageFileHelper {
    type Vtable = INDStorageFileHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8f0bef8_91d2_4d47_a3f9_eaff4edb729f);
}
impl INDStorageFileHelper {
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn GetFileURLs<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::IStorageFile>>(&self, file: Param0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for INDStorageFileHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d8f0bef8-91d2-4d47-a3f9-eaff4edb729f}");
}
impl ::core::convert::From<INDStorageFileHelper> for ::windows::core::IUnknown {
    fn from(value: INDStorageFileHelper) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INDStorageFileHelper> for ::windows::core::IUnknown {
    fn from(value: &INDStorageFileHelper) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INDStorageFileHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INDStorageFileHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INDStorageFileHelper> for ::windows::core::IInspectable {
    fn from(value: INDStorageFileHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INDStorageFileHelper> for ::windows::core::IInspectable {
    fn from(value: &INDStorageFileHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INDStorageFileHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INDStorageFileHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDStorageFileHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INDStreamParser(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INDStreamParser {
    type Vtable = INDStreamParser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0baa198_9796_41c9_8695_59437e67e66a);
}
impl INDStreamParser {
    #[cfg(feature = "deprecated")]
    pub fn ParseData(&self, databytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), databytes.len() as u32, ::core::mem::transmute(databytes.as_ptr())).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Media_Core")]
    pub fn GetStreamInformation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Core::IMediaStreamDescriptor>>(&self, descriptor: Param0, streamtype: &mut NDMediaStreamType) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), descriptor.into_param().abi(), streamtype, &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn BeginOfStream(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn EndOfStream(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn Notifier(&self) -> ::windows::core::Result<NDStreamParserNotifier> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NDStreamParserNotifier>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for INDStreamParser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e0baa198-9796-41c9-8695-59437e67e66a}");
}
impl ::core::convert::From<INDStreamParser> for ::windows::core::IUnknown {
    fn from(value: INDStreamParser) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INDStreamParser> for ::windows::core::IUnknown {
    fn from(value: &INDStreamParser) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INDStreamParser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INDStreamParser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INDStreamParser> for ::windows::core::IInspectable {
    fn from(value: INDStreamParser) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INDStreamParser> for ::windows::core::IInspectable {
    fn from(value: &INDStreamParser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INDStreamParser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INDStreamParser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDStreamParser_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dataBytes_array_size: u32, databytes: *const u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, descriptor: ::windows::core::RawPtr, streamtype: *mut NDMediaStreamType, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INDStreamParserNotifier(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INDStreamParserNotifier {
    type Vtable = INDStreamParserNotifier_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc167acd0_2ce6_426c_ace5_5e9275fea715);
}
impl INDStreamParserNotifier {
    #[cfg(feature = "deprecated")]
    pub fn OnContentIDReceived<'a, Param0: ::windows::core::IntoParam<'a, INDLicenseFetchDescriptor>>(&self, licensefetchdescriptor: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), licensefetchdescriptor.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn OnMediaStreamDescriptorCreated<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::super::Core::AudioStreamDescriptor>>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::super::Core::VideoStreamDescriptor>>>(&self, audiostreamdescriptors: Param0, videostreamdescriptors: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), audiostreamdescriptors.into_param().abi(), videostreamdescriptors.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Media_Core")]
    pub fn OnSampleParsed<'a, Param2: ::windows::core::IntoParam<'a, super::super::Core::MediaStreamSample>>(&self, streamid: u32, streamtype: NDMediaStreamType, streamsample: Param2, pts: i64, ccformat: NDClosedCaptionFormat, ccdatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), streamid, streamtype, streamsample.into_param().abi(), pts, ccformat, ccdatabytes.len() as u32, ::core::mem::transmute(ccdatabytes.as_ptr())).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Media_Core")]
    pub fn OnBeginSetupDecryptor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Core::IMediaStreamDescriptor>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, descriptor: Param0, keyid: Param1, probytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), descriptor.into_param().abi(), keyid.into_param().abi(), probytes.len() as u32, ::core::mem::transmute(probytes.as_ptr())).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for INDStreamParserNotifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c167acd0-2ce6-426c-ace5-5e9275fea715}");
}
impl ::core::convert::From<INDStreamParserNotifier> for ::windows::core::IUnknown {
    fn from(value: INDStreamParserNotifier) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INDStreamParserNotifier> for ::windows::core::IUnknown {
    fn from(value: &INDStreamParserNotifier) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INDStreamParserNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INDStreamParserNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INDStreamParserNotifier> for ::windows::core::IInspectable {
    fn from(value: INDStreamParserNotifier) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INDStreamParserNotifier> for ::windows::core::IInspectable {
    fn from(value: &INDStreamParserNotifier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INDStreamParserNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INDStreamParserNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDStreamParserNotifier_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, licensefetchdescriptor: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, audiostreamdescriptors: ::windows::core::RawPtr, videostreamdescriptors: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))] usize,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, streamid: u32, streamtype: NDMediaStreamType, streamsample: ::windows::core::RawPtr, pts: i64, ccformat: NDClosedCaptionFormat, ccDataBytes_array_size: u32, ccdatabytes: *const u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, descriptor: ::windows::core::RawPtr, keyid: ::windows::core::GUID, proBytes_array_size: u32, probytes: *const u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct INDTCPMessengerFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INDTCPMessengerFactory {
    type Vtable = INDTCPMessengerFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7dd85cfe_1b99_4f68_8f82_8177f7cedf2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INDTCPMessengerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, remotehostname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, remotehostport: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INDTransmitterProperties(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for INDTransmitterProperties {
    type Vtable = INDTransmitterProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe536af23_ac4f_4adc_8c66_4ff7c2702dd6);
}
impl INDTransmitterProperties {
    #[cfg(feature = "deprecated")]
    pub fn CertificateType(&self) -> ::windows::core::Result<NDCertificateType> {
        let this = self;
        unsafe {
            let mut result__: NDCertificateType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NDCertificateType>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn PlatformIdentifier(&self) -> ::windows::core::Result<NDCertificatePlatformID> {
        let this = self;
        unsafe {
            let mut result__: NDCertificatePlatformID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NDCertificatePlatformID>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SupportedFeatures(&self) -> ::windows::core::Result<::windows::core::Array<NDCertificateFeature>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<NDCertificateFeature> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), ::windows::core::Array::<NDCertificateFeature>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SecurityLevel(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SecurityVersion(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationDate(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ClientID(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ModelDigest(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ModelManufacturerName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ModelName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ModelNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for INDTransmitterProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e536af23-ac4f-4adc-8c66-4ff7c2702dd6}");
}
impl ::core::convert::From<INDTransmitterProperties> for ::windows::core::IUnknown {
    fn from(value: INDTransmitterProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&INDTransmitterProperties> for ::windows::core::IUnknown {
    fn from(value: &INDTransmitterProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INDTransmitterProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INDTransmitterProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<INDTransmitterProperties> for ::windows::core::IInspectable {
    fn from(value: INDTransmitterProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INDTransmitterProperties> for ::windows::core::IInspectable {
    fn from(value: &INDTransmitterProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INDTransmitterProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INDTransmitterProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INDTransmitterProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut NDCertificateType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut NDCertificatePlatformID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut NDCertificateFeature) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadyContentHeader(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyContentHeader {
    type Vtable = IPlayReadyContentHeader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a438a6a_7f4c_452e_88bd_0148c6387a2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PlayReadyEncryptionAlgorithm) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PlayReadyDecryptorSetup) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadyContentHeader2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyContentHeader2 {
    type Vtable = IPlayReadyContentHeader2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x359c79f4_2180_498c_965b_e754d875eab2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeader2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadyContentHeaderFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyContentHeaderFactory {
    type Vtable = IPlayReadyContentHeaderFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb97c8ff_b758_4776_bf01_217a8b510b2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeaderFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, headerBytes_array_size: u32, headerbytes: *const u8, licenseacquisitionurl: ::windows::core::RawPtr, licenseacquisitionuserinterfaceurl: ::windows::core::RawPtr, customattributes: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, domainserviceid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, contentkeyid: ::windows::core::GUID, contentkeyidstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: ::windows::core::RawPtr, licenseacquisitionuserinterfaceurl: ::windows::core::RawPtr, customattributes: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, domainserviceid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, headerBytes_array_size: u32, headerbytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadyContentHeaderFactory2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyContentHeaderFactory2 {
    type Vtable = IPlayReadyContentHeaderFactory2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1239cf5_ae6d_4778_97fd_6e3a2eeadbeb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeaderFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwflags: u32, contentKeyIds_array_size: u32, contentkeyids: *const ::windows::core::GUID, contentKeyIdStrings_array_size: u32, contentkeyidstrings: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: ::windows::core::RawPtr, licenseacquisitionuserinterfaceurl: ::windows::core::RawPtr, customattributes: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, domainserviceid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadyContentResolver(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyContentResolver {
    type Vtable = IPlayReadyContentResolver_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbfd2523_906d_4982_a6b8_6849565a7ce8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentResolver_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, contentheader: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPlayReadyDomain(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyDomain {
    type Vtable = IPlayReadyDomain_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadcc93ac_97e6_43ef_95e4_d7868f3b16a9);
}
impl IPlayReadyDomain {
    pub fn AccountId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn Revision(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DomainJoinUrl(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IPlayReadyDomain {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{adcc93ac-97e6-43ef-95e4-d7868f3b16a9}");
}
impl ::core::convert::From<IPlayReadyDomain> for ::windows::core::IUnknown {
    fn from(value: IPlayReadyDomain) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPlayReadyDomain> for ::windows::core::IUnknown {
    fn from(value: &IPlayReadyDomain) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPlayReadyDomain {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPlayReadyDomain {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPlayReadyDomain> for ::windows::core::IInspectable {
    fn from(value: IPlayReadyDomain) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPlayReadyDomain> for ::windows::core::IInspectable {
    fn from(value: &IPlayReadyDomain) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPlayReadyDomain {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPlayReadyDomain {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomain_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadyDomainIterableFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyDomainIterableFactory {
    type Vtable = IPlayReadyDomainIterableFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4df384ee_3121_4df3_a5e8_d0c24c0500fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomainIterableFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, domainaccountid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadyDomainJoinServiceRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyDomainJoinServiceRequest {
    type Vtable = IPlayReadyDomainJoinServiceRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x171b4a5a_405f_4739_b040_67b9f0c38758);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomainJoinServiceRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadyDomainLeaveServiceRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyDomainLeaveServiceRequest {
    type Vtable = IPlayReadyDomainLeaveServiceRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x062d58be_97ad_4917_aa03_46d4c252d464);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomainLeaveServiceRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadyITADataGenerator(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyITADataGenerator {
    type Vtable = IPlayReadyITADataGenerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24446b8e_10b9_4530_b25b_901a8029a9b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyITADataGenerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, guidcpsystemid: ::windows::core::GUID, countofstreams: u32, configuration: ::windows::core::RawPtr, format: PlayReadyITADataFormat, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadyIndividualizationServiceRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyIndividualizationServiceRequest {
    type Vtable = IPlayReadyIndividualizationServiceRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21f5a86b_008c_4611_ab2f_aaa6c69f0e24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyIndividualizationServiceRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPlayReadyLicense(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyLicense {
    type Vtable = IPlayReadyLicense_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee474c4e_fa3c_414d_a9f2_3ffc1ef832d4);
}
impl IPlayReadyLicense {
    pub fn FullyEvaluated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn UsableForPlay(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExpirationDate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    pub fn ExpireAfterFirstPlay(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn DomainAccountID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn ChainDepth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn GetKIDAtChainDepth(&self, chaindepth: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), chaindepth, &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IPlayReadyLicense {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ee474c4e-fa3c-414d-a9f2-3ffc1ef832d4}");
}
impl ::core::convert::From<IPlayReadyLicense> for ::windows::core::IUnknown {
    fn from(value: IPlayReadyLicense) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPlayReadyLicense> for ::windows::core::IUnknown {
    fn from(value: &IPlayReadyLicense) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPlayReadyLicense {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPlayReadyLicense {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPlayReadyLicense> for ::windows::core::IInspectable {
    fn from(value: IPlayReadyLicense) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPlayReadyLicense> for ::windows::core::IInspectable {
    fn from(value: &IPlayReadyLicense) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPlayReadyLicense {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPlayReadyLicense {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicense_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, chaindepth: u32, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadyLicense2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyLicense2 {
    type Vtable = IPlayReadyLicense2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30f4e7a7_d8e3_48a0_bcda_ff9f40530436);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicense2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyLicenseAcquisitionServiceRequest {
    type Vtable = IPlayReadyLicenseAcquisitionServiceRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d85ff45_3e9f_4f48_93e1_9530c8d58c3e);
}
impl IPlayReadyLicenseAcquisitionServiceRequest {
    pub fn ContentHeader(&self) -> ::windows::core::Result<PlayReadyContentHeader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadyContentHeader>(result__)
        }
    }
    pub fn SetContentHeader<'a, Param0: ::windows::core::IntoParam<'a, PlayReadyContentHeader>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DomainServiceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn SetDomainServiceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetChallengeCustomData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), responsebytes.len() as u32, ::core::mem::transmute(responsebytes.as_ptr()), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IPlayReadyLicenseAcquisitionServiceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5d85ff45-3e9f-4f48-93e1-9530c8d58c3e}");
}
impl ::core::convert::From<IPlayReadyLicenseAcquisitionServiceRequest> for ::windows::core::IUnknown {
    fn from(value: IPlayReadyLicenseAcquisitionServiceRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPlayReadyLicenseAcquisitionServiceRequest> for ::windows::core::IUnknown {
    fn from(value: &IPlayReadyLicenseAcquisitionServiceRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPlayReadyLicenseAcquisitionServiceRequest> for ::windows::core::IInspectable {
    fn from(value: IPlayReadyLicenseAcquisitionServiceRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPlayReadyLicenseAcquisitionServiceRequest> for ::windows::core::IInspectable {
    fn from(value: &IPlayReadyLicenseAcquisitionServiceRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IPlayReadyLicenseAcquisitionServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: IPlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadyLicenseAcquisitionServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaProtectionServiceRequest> for IPlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaProtectionServiceRequest> for &IPlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::core::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IPlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: IPlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyServiceRequest> for IPlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyServiceRequest> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyServiceRequest> for &IPlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyServiceRequest> {
        ::core::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyLicenseAcquisitionServiceRequest2 {
    type Vtable = IPlayReadyLicenseAcquisitionServiceRequest2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7fa5eb5_fe0c_b225_bc60_5a9edd32ceb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyLicenseAcquisitionServiceRequest3 {
    type Vtable = IPlayReadyLicenseAcquisitionServiceRequest3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x394e5f4d_7f75_430d_b2e7_7f75f34b2d75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, contentheader: ::windows::core::RawPtr, fullyevaluated: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadyLicenseIterableFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyLicenseIterableFactory {
    type Vtable = IPlayReadyLicenseIterableFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4179f08_0837_4978_8e68_be4293c8d7a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseIterableFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, contentheader: ::windows::core::RawPtr, fullyevaluated: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadyLicenseManagement(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyLicenseManagement {
    type Vtable = IPlayReadyLicenseManagement_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaaeb2141_0957_4405_b892_8bf3ec5dadd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseManagement_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, contentheader: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPlayReadyLicenseSession(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyLicenseSession {
    type Vtable = IPlayReadyLicenseSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1723a39_87fa_4fdd_abbb_a9720e845259);
}
impl IPlayReadyLicenseSession {
    pub fn CreateLAServiceRequest(&self) -> ::windows::core::Result<IPlayReadyLicenseAcquisitionServiceRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyLicenseAcquisitionServiceRequest>(result__)
        }
    }
    pub fn ConfigureMediaProtectionManager<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProtectionManager>>(&self, mpm: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mpm.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IPlayReadyLicenseSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a1723a39-87fa-4fdd-abbb-a9720e845259}");
}
impl ::core::convert::From<IPlayReadyLicenseSession> for ::windows::core::IUnknown {
    fn from(value: IPlayReadyLicenseSession) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPlayReadyLicenseSession> for ::windows::core::IUnknown {
    fn from(value: &IPlayReadyLicenseSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPlayReadyLicenseSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPlayReadyLicenseSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPlayReadyLicenseSession> for ::windows::core::IInspectable {
    fn from(value: IPlayReadyLicenseSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPlayReadyLicenseSession> for ::windows::core::IInspectable {
    fn from(value: &IPlayReadyLicenseSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPlayReadyLicenseSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPlayReadyLicenseSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseSession_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mpm: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPlayReadyLicenseSession2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyLicenseSession2 {
    type Vtable = IPlayReadyLicenseSession2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4909be3a_3aed_4656_8ad7_ee0fd7799510);
}
impl IPlayReadyLicenseSession2 {
    pub fn CreateLAServiceRequest(&self) -> ::windows::core::Result<IPlayReadyLicenseAcquisitionServiceRequest> {
        let this = &::windows::core::Interface::cast::<IPlayReadyLicenseSession>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyLicenseAcquisitionServiceRequest>(result__)
        }
    }
    pub fn ConfigureMediaProtectionManager<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProtectionManager>>(&self, mpm: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyLicenseSession>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mpm.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateLicenseIterable<'a, Param0: ::windows::core::IntoParam<'a, PlayReadyContentHeader>>(&self, contentheader: Param0, fullyevaluated: bool) -> ::windows::core::Result<PlayReadyLicenseIterable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), contentheader.into_param().abi(), fullyevaluated, &mut result__).from_abi::<PlayReadyLicenseIterable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IPlayReadyLicenseSession2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4909be3a-3aed-4656-8ad7-ee0fd7799510}");
}
impl ::core::convert::From<IPlayReadyLicenseSession2> for ::windows::core::IUnknown {
    fn from(value: IPlayReadyLicenseSession2) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPlayReadyLicenseSession2> for ::windows::core::IUnknown {
    fn from(value: &IPlayReadyLicenseSession2) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPlayReadyLicenseSession2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPlayReadyLicenseSession2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPlayReadyLicenseSession2> for ::windows::core::IInspectable {
    fn from(value: IPlayReadyLicenseSession2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPlayReadyLicenseSession2> for ::windows::core::IInspectable {
    fn from(value: &IPlayReadyLicenseSession2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPlayReadyLicenseSession2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPlayReadyLicenseSession2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IPlayReadyLicenseSession2> for IPlayReadyLicenseSession {
    type Error = ::windows::core::Error;
    fn try_from(value: IPlayReadyLicenseSession2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadyLicenseSession2> for IPlayReadyLicenseSession {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPlayReadyLicenseSession2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyLicenseSession> for IPlayReadyLicenseSession2 {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyLicenseSession> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyLicenseSession> for &IPlayReadyLicenseSession2 {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyLicenseSession> {
        ::core::convert::TryInto::<IPlayReadyLicenseSession>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseSession2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, contentheader: ::windows::core::RawPtr, fullyevaluated: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadyLicenseSessionFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyLicenseSessionFactory {
    type Vtable = IPlayReadyLicenseSessionFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62492699_6527_429e_98be_48d798ac2739);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseSessionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, configuration: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadyMeteringReportServiceRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyMeteringReportServiceRequest {
    type Vtable = IPlayReadyMeteringReportServiceRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc12b231c_0ecd_4f11_a185_1e24a4a67fb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyMeteringReportServiceRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, meteringCertBytes_array_size: u32, meteringcertbytes: *const u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadyRevocationServiceRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyRevocationServiceRequest {
    type Vtable = IPlayReadyRevocationServiceRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x543d66ac_faf0_4560_84a5_0e4acec939e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyRevocationServiceRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadySecureStopIterableFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadySecureStopIterableFactory {
    type Vtable = IPlayReadySecureStopIterableFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f1f0165_4214_4d9e_81eb_e89f9d294aee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySecureStopIterableFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, publisherCertBytes_array_size: u32, publishercertbytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPlayReadySecureStopServiceRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadySecureStopServiceRequest {
    type Vtable = IPlayReadySecureStopServiceRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5501ee5_01bf_4401_9677_05630a6a4cc8);
}
impl IPlayReadySecureStopServiceRequest {
    pub fn SessionID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UpdateTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn Stopped(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn PublisherCertificate(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    pub fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetChallengeCustomData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), responsebytes.len() as u32, ::core::mem::transmute(responsebytes.as_ptr()), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IPlayReadySecureStopServiceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b5501ee5-01bf-4401-9677-05630a6a4cc8}");
}
impl ::core::convert::From<IPlayReadySecureStopServiceRequest> for ::windows::core::IUnknown {
    fn from(value: IPlayReadySecureStopServiceRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPlayReadySecureStopServiceRequest> for ::windows::core::IUnknown {
    fn from(value: &IPlayReadySecureStopServiceRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPlayReadySecureStopServiceRequest> for ::windows::core::IInspectable {
    fn from(value: IPlayReadySecureStopServiceRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPlayReadySecureStopServiceRequest> for ::windows::core::IInspectable {
    fn from(value: &IPlayReadySecureStopServiceRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IPlayReadySecureStopServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: IPlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadySecureStopServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaProtectionServiceRequest> for IPlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaProtectionServiceRequest> for &IPlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::core::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IPlayReadySecureStopServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: IPlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadySecureStopServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyServiceRequest> for IPlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyServiceRequest> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyServiceRequest> for &IPlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyServiceRequest> {
        ::core::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySecureStopServiceRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadySecureStopServiceRequestFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadySecureStopServiceRequestFactory {
    type Vtable = IPlayReadySecureStopServiceRequestFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e448ac9_e67e_494e_9f49_6285438c76cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySecureStopServiceRequestFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, publisherCertBytes_array_size: u32, publishercertbytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sessionid: ::windows::core::GUID, publisherCertBytes_array_size: u32, publishercertbytes: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPlayReadyServiceRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyServiceRequest {
    type Vtable = IPlayReadyServiceRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bad2836_a703_45a6_a180_76f3565aa725);
}
impl IPlayReadyServiceRequest {
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetChallengeCustomData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), responsebytes.len() as u32, ::core::mem::transmute(responsebytes.as_ptr()), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    pub fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IPlayReadyServiceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{8bad2836-a703-45a6-a180-76f3565aa725}");
}
impl ::core::convert::From<IPlayReadyServiceRequest> for ::windows::core::IUnknown {
    fn from(value: IPlayReadyServiceRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPlayReadyServiceRequest> for ::windows::core::IUnknown {
    fn from(value: &IPlayReadyServiceRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPlayReadyServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPlayReadyServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPlayReadyServiceRequest> for ::windows::core::IInspectable {
    fn from(value: IPlayReadyServiceRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPlayReadyServiceRequest> for ::windows::core::IInspectable {
    fn from(value: &IPlayReadyServiceRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPlayReadyServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPlayReadyServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IPlayReadyServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: IPlayReadyServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPlayReadyServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPlayReadyServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaProtectionServiceRequest> for IPlayReadyServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaProtectionServiceRequest> for &IPlayReadyServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::core::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyServiceRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, responseBytes_array_size: u32, responsebytes: *const u8, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadySoapMessage(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadySoapMessage {
    type Vtable = IPlayReadySoapMessage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb659fcb5_ce41_41ba_8a0d_61df5fffa139);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySoapMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadyStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyStatics {
    type Vtable = IPlayReadyStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e69c00d_247c_469a_8f31_5c1a1571d9c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadyStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyStatics2 {
    type Vtable = IPlayReadyStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f8d6a92_5f9a_423e_9466_b33969af7a3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics2_abi(
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
pub struct IPlayReadyStatics3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyStatics3 {
    type Vtable = IPlayReadyStatics3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fa33f71_2dd3_4bed_ae49_f7148e63e710);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hwdrmfeature: PlayReadyHardwareDRMFeatures, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadyStatics4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyStatics4 {
    type Vtable = IPlayReadyStatics4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50a91300_d824_4231_9d5e_78ef8844c7d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlayReadyStatics5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlayReadyStatics5 {
    type Vtable = IPlayReadyStatics5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x230a7075_dfa0_4f8e_a779_cefea9c6824b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDCertificateFeature(pub i32);
impl NDCertificateFeature {
    pub const Transmitter: NDCertificateFeature = NDCertificateFeature(1i32);
    pub const Receiver: NDCertificateFeature = NDCertificateFeature(2i32);
    pub const SharedCertificate: NDCertificateFeature = NDCertificateFeature(3i32);
    pub const SecureClock: NDCertificateFeature = NDCertificateFeature(4i32);
    pub const AntiRollBackClock: NDCertificateFeature = NDCertificateFeature(5i32);
    pub const CRLS: NDCertificateFeature = NDCertificateFeature(9i32);
    pub const PlayReady3Features: NDCertificateFeature = NDCertificateFeature(13i32);
}
impl ::core::convert::From<i32> for NDCertificateFeature {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NDCertificateFeature {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for NDCertificateFeature {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDCertificateFeature;i4)");
}
impl ::windows::core::DefaultType for NDCertificateFeature {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDCertificatePlatformID(pub i32);
impl NDCertificatePlatformID {
    pub const Windows: NDCertificatePlatformID = NDCertificatePlatformID(0i32);
    pub const OSX: NDCertificatePlatformID = NDCertificatePlatformID(1i32);
    pub const WindowsOnARM: NDCertificatePlatformID = NDCertificatePlatformID(2i32);
    pub const WindowsMobile7: NDCertificatePlatformID = NDCertificatePlatformID(5i32);
    pub const iOSOnARM: NDCertificatePlatformID = NDCertificatePlatformID(6i32);
    pub const XBoxOnPPC: NDCertificatePlatformID = NDCertificatePlatformID(7i32);
    pub const WindowsPhone8OnARM: NDCertificatePlatformID = NDCertificatePlatformID(8i32);
    pub const WindowsPhone8OnX86: NDCertificatePlatformID = NDCertificatePlatformID(9i32);
    pub const XboxOne: NDCertificatePlatformID = NDCertificatePlatformID(10i32);
    pub const AndroidOnARM: NDCertificatePlatformID = NDCertificatePlatformID(11i32);
    pub const WindowsPhone81OnARM: NDCertificatePlatformID = NDCertificatePlatformID(12i32);
    pub const WindowsPhone81OnX86: NDCertificatePlatformID = NDCertificatePlatformID(13i32);
}
impl ::core::convert::From<i32> for NDCertificatePlatformID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NDCertificatePlatformID {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for NDCertificatePlatformID {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDCertificatePlatformID;i4)");
}
impl ::windows::core::DefaultType for NDCertificatePlatformID {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDCertificateType(pub i32);
impl NDCertificateType {
    pub const Unknown: NDCertificateType = NDCertificateType(0i32);
    pub const PC: NDCertificateType = NDCertificateType(1i32);
    pub const Device: NDCertificateType = NDCertificateType(2i32);
    pub const Domain: NDCertificateType = NDCertificateType(3i32);
    pub const Issuer: NDCertificateType = NDCertificateType(4i32);
    pub const CrlSigner: NDCertificateType = NDCertificateType(5i32);
    pub const Service: NDCertificateType = NDCertificateType(6i32);
    pub const Silverlight: NDCertificateType = NDCertificateType(7i32);
    pub const Application: NDCertificateType = NDCertificateType(8i32);
    pub const Metering: NDCertificateType = NDCertificateType(9i32);
    pub const KeyFileSigner: NDCertificateType = NDCertificateType(10i32);
    pub const Server: NDCertificateType = NDCertificateType(11i32);
    pub const LicenseSigner: NDCertificateType = NDCertificateType(12i32);
}
impl ::core::convert::From<i32> for NDCertificateType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NDCertificateType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for NDCertificateType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDCertificateType;i4)");
}
impl ::windows::core::DefaultType for NDCertificateType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NDClient(pub ::windows::core::IInspectable);
impl NDClient {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RegistrationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<NDClient, INDRegistrationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRegistrationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn ProximityDetectionCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<NDClient, INDProximityDetectionCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProximityDetectionCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn LicenseFetchCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<NDClient, INDLicenseFetchCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLicenseFetchCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn ReRegistrationNeeded<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<NDClient, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReRegistrationNeeded<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn ClosedCaptionDataReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<NDClient, INDClosedCaptionDataReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosedCaptionDataReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, INDCustomData>, Param3: ::windows::core::IntoParam<'a, INDLicenseFetchDescriptor>>(&self, contenturl: Param0, startasyncoptions: u32, registrationcustomdata: Param2, licensefetchdescriptor: Param3) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDStartResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), contenturl.into_param().abi(), startasyncoptions, registrationcustomdata.into_param().abi(), licensefetchdescriptor.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<INDStartResult>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn LicenseFetchAsync<'a, Param0: ::windows::core::IntoParam<'a, INDLicenseFetchDescriptor>>(&self, licensefetchdescriptor: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDLicenseFetchResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), licensefetchdescriptor.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<INDLicenseFetchResult>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn ReRegistrationAsync<'a, Param0: ::windows::core::IntoParam<'a, INDCustomData>>(&self, registrationcustomdata: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), registrationcustomdata.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, INDDownloadEngine>, Param1: ::windows::core::IntoParam<'a, INDStreamParser>, Param2: ::windows::core::IntoParam<'a, INDMessenger>>(downloadengine: Param0, streamparser: Param1, pmessenger: Param2) -> ::windows::core::Result<NDClient> {
        Self::INDClientFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), downloadengine.into_param().abi(), streamparser.into_param().abi(), pmessenger.into_param().abi(), &mut result__).from_abi::<NDClient>(result__)
        })
    }
    pub fn INDClientFactory<R, F: FnOnce(&INDClientFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NDClient, INDClientFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for NDClient {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDClient;{3bd6781b-61b8-46e2-99a5-8abcb6b9f7d6})");
}
unsafe impl ::windows::core::Interface for NDClient {
    type Vtable = INDClient_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bd6781b_61b8_46e2_99a5_8abcb6b9f7d6);
}
impl ::windows::core::RuntimeName for NDClient {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDClient";
}
impl ::core::convert::From<NDClient> for ::windows::core::IUnknown {
    fn from(value: NDClient) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NDClient> for ::windows::core::IUnknown {
    fn from(value: &NDClient) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NDClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NDClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NDClient> for ::windows::core::IInspectable {
    fn from(value: NDClient) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NDClient> for ::windows::core::IInspectable {
    fn from(value: &NDClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NDClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NDClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDClosedCaptionFormat(pub i32);
impl NDClosedCaptionFormat {
    pub const ATSC: NDClosedCaptionFormat = NDClosedCaptionFormat(0i32);
    pub const SCTE20: NDClosedCaptionFormat = NDClosedCaptionFormat(1i32);
    pub const Unknown: NDClosedCaptionFormat = NDClosedCaptionFormat(2i32);
}
impl ::core::convert::From<i32> for NDClosedCaptionFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NDClosedCaptionFormat {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for NDClosedCaptionFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDClosedCaptionFormat;i4)");
}
impl ::windows::core::DefaultType for NDClosedCaptionFormat {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDContentIDType(pub i32);
impl NDContentIDType {
    pub const KeyID: NDContentIDType = NDContentIDType(1i32);
    pub const PlayReadyObject: NDContentIDType = NDContentIDType(2i32);
    pub const Custom: NDContentIDType = NDContentIDType(3i32);
}
impl ::core::convert::From<i32> for NDContentIDType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NDContentIDType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for NDContentIDType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDContentIDType;i4)");
}
impl ::windows::core::DefaultType for NDContentIDType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NDCustomData(pub ::windows::core::IInspectable);
impl NDCustomData {
    #[cfg(feature = "deprecated")]
    pub fn CustomDataTypeID(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn CustomData(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn CreateInstance(customdatatypeidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], customdatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<NDCustomData> {
        Self::INDCustomDataFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), customdatatypeidbytes.len() as u32, ::core::mem::transmute(customdatatypeidbytes.as_ptr()), customdatabytes.len() as u32, ::core::mem::transmute(customdatabytes.as_ptr()), &mut result__).from_abi::<NDCustomData>(result__)
        })
    }
    pub fn INDCustomDataFactory<R, F: FnOnce(&INDCustomDataFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NDCustomData, INDCustomDataFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for NDCustomData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDCustomData;{f5cb0fdc-2d09-4f19-b5e1-76a0b3ee9267})");
}
unsafe impl ::windows::core::Interface for NDCustomData {
    type Vtable = INDCustomData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5cb0fdc_2d09_4f19_b5e1_76a0b3ee9267);
}
impl ::windows::core::RuntimeName for NDCustomData {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDCustomData";
}
impl ::core::convert::From<NDCustomData> for ::windows::core::IUnknown {
    fn from(value: NDCustomData) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NDCustomData> for ::windows::core::IUnknown {
    fn from(value: &NDCustomData) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NDCustomData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NDCustomData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NDCustomData> for ::windows::core::IInspectable {
    fn from(value: NDCustomData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NDCustomData> for ::windows::core::IInspectable {
    fn from(value: &NDCustomData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NDCustomData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NDCustomData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<NDCustomData> for INDCustomData {
    fn from(value: NDCustomData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NDCustomData> for INDCustomData {
    fn from(value: &NDCustomData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INDCustomData> for NDCustomData {
    fn into_param(self) -> ::windows::core::Param<'a, INDCustomData> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INDCustomData> for &NDCustomData {
    fn into_param(self) -> ::windows::core::Param<'a, INDCustomData> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NDDownloadEngineNotifier(pub ::windows::core::IInspectable);
impl NDDownloadEngineNotifier {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NDDownloadEngineNotifier, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "deprecated")]
    pub fn OnStreamOpened(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn OnPlayReadyObjectReceived(&self, databytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), databytes.len() as u32, ::core::mem::transmute(databytes.as_ptr())).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn OnContentIDReceived<'a, Param0: ::windows::core::IntoParam<'a, INDLicenseFetchDescriptor>>(&self, licensefetchdescriptor: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), licensefetchdescriptor.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn OnDataReceived(&self, databytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], bytesreceived: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), databytes.len() as u32, ::core::mem::transmute(databytes.as_ptr()), bytesreceived).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn OnEndOfStream(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn OnNetworkError(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for NDDownloadEngineNotifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDDownloadEngineNotifier;{d720b4d4-f4b8-4530-a809-9193a571e7fc})");
}
unsafe impl ::windows::core::Interface for NDDownloadEngineNotifier {
    type Vtable = INDDownloadEngineNotifier_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd720b4d4_f4b8_4530_a809_9193a571e7fc);
}
impl ::windows::core::RuntimeName for NDDownloadEngineNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDDownloadEngineNotifier";
}
impl ::core::convert::From<NDDownloadEngineNotifier> for ::windows::core::IUnknown {
    fn from(value: NDDownloadEngineNotifier) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NDDownloadEngineNotifier> for ::windows::core::IUnknown {
    fn from(value: &NDDownloadEngineNotifier) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NDDownloadEngineNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NDDownloadEngineNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NDDownloadEngineNotifier> for ::windows::core::IInspectable {
    fn from(value: NDDownloadEngineNotifier) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NDDownloadEngineNotifier> for ::windows::core::IInspectable {
    fn from(value: &NDDownloadEngineNotifier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NDDownloadEngineNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NDDownloadEngineNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<NDDownloadEngineNotifier> for INDDownloadEngineNotifier {
    fn from(value: NDDownloadEngineNotifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NDDownloadEngineNotifier> for INDDownloadEngineNotifier {
    fn from(value: &NDDownloadEngineNotifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INDDownloadEngineNotifier> for NDDownloadEngineNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, INDDownloadEngineNotifier> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INDDownloadEngineNotifier> for &NDDownloadEngineNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, INDDownloadEngineNotifier> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NDLicenseFetchDescriptor(pub ::windows::core::IInspectable);
impl NDLicenseFetchDescriptor {
    #[cfg(feature = "deprecated")]
    pub fn ContentIDType(&self) -> ::windows::core::Result<NDContentIDType> {
        let this = self;
        unsafe {
            let mut result__: NDContentIDType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NDContentIDType>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn ContentID(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn LicenseFetchChallengeCustomData(&self) -> ::windows::core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<INDCustomData>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetLicenseFetchChallengeCustomData<'a, Param0: ::windows::core::IntoParam<'a, INDCustomData>>(&self, licensefetchchallengecustomdata: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), licensefetchchallengecustomdata.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn CreateInstance<'a, Param2: ::windows::core::IntoParam<'a, INDCustomData>>(contentidtype: NDContentIDType, contentidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], licensefetchchallengecustomdata: Param2) -> ::windows::core::Result<NDLicenseFetchDescriptor> {
        Self::INDLicenseFetchDescriptorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), contentidtype, contentidbytes.len() as u32, ::core::mem::transmute(contentidbytes.as_ptr()), licensefetchchallengecustomdata.into_param().abi(), &mut result__).from_abi::<NDLicenseFetchDescriptor>(result__)
        })
    }
    pub fn INDLicenseFetchDescriptorFactory<R, F: FnOnce(&INDLicenseFetchDescriptorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NDLicenseFetchDescriptor, INDLicenseFetchDescriptorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for NDLicenseFetchDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDLicenseFetchDescriptor;{5498d33a-e686-4935-a567-7ca77ad20fa4})");
}
unsafe impl ::windows::core::Interface for NDLicenseFetchDescriptor {
    type Vtable = INDLicenseFetchDescriptor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5498d33a_e686_4935_a567_7ca77ad20fa4);
}
impl ::windows::core::RuntimeName for NDLicenseFetchDescriptor {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDLicenseFetchDescriptor";
}
impl ::core::convert::From<NDLicenseFetchDescriptor> for ::windows::core::IUnknown {
    fn from(value: NDLicenseFetchDescriptor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NDLicenseFetchDescriptor> for ::windows::core::IUnknown {
    fn from(value: &NDLicenseFetchDescriptor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NDLicenseFetchDescriptor> for ::windows::core::IInspectable {
    fn from(value: NDLicenseFetchDescriptor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NDLicenseFetchDescriptor> for ::windows::core::IInspectable {
    fn from(value: &NDLicenseFetchDescriptor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<NDLicenseFetchDescriptor> for INDLicenseFetchDescriptor {
    fn from(value: NDLicenseFetchDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NDLicenseFetchDescriptor> for INDLicenseFetchDescriptor {
    fn from(value: &NDLicenseFetchDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INDLicenseFetchDescriptor> for NDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, INDLicenseFetchDescriptor> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INDLicenseFetchDescriptor> for &NDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, INDLicenseFetchDescriptor> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDMediaStreamType(pub i32);
impl NDMediaStreamType {
    pub const Audio: NDMediaStreamType = NDMediaStreamType(1i32);
    pub const Video: NDMediaStreamType = NDMediaStreamType(2i32);
}
impl ::core::convert::From<i32> for NDMediaStreamType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NDMediaStreamType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for NDMediaStreamType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDMediaStreamType;i4)");
}
impl ::windows::core::DefaultType for NDMediaStreamType {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDProximityDetectionType(pub i32);
impl NDProximityDetectionType {
    pub const UDP: NDProximityDetectionType = NDProximityDetectionType(1i32);
    pub const TCP: NDProximityDetectionType = NDProximityDetectionType(2i32);
    pub const TransportAgnostic: NDProximityDetectionType = NDProximityDetectionType(4i32);
}
impl ::core::convert::From<i32> for NDProximityDetectionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NDProximityDetectionType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for NDProximityDetectionType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDProximityDetectionType;i4)");
}
impl ::windows::core::DefaultType for NDProximityDetectionType {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDStartAsyncOptions(pub i32);
impl NDStartAsyncOptions {
    pub const MutualAuthentication: NDStartAsyncOptions = NDStartAsyncOptions(1i32);
    pub const WaitForLicenseDescriptor: NDStartAsyncOptions = NDStartAsyncOptions(2i32);
}
impl ::core::convert::From<i32> for NDStartAsyncOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NDStartAsyncOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for NDStartAsyncOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDStartAsyncOptions;i4)");
}
impl ::windows::core::DefaultType for NDStartAsyncOptions {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NDStorageFileHelper(pub ::windows::core::IInspectable);
impl NDStorageFileHelper {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NDStorageFileHelper, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn GetFileURLs<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::IStorageFile>>(&self, file: Param0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for NDStorageFileHelper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDStorageFileHelper;{d8f0bef8-91d2-4d47-a3f9-eaff4edb729f})");
}
unsafe impl ::windows::core::Interface for NDStorageFileHelper {
    type Vtable = INDStorageFileHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8f0bef8_91d2_4d47_a3f9_eaff4edb729f);
}
impl ::windows::core::RuntimeName for NDStorageFileHelper {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDStorageFileHelper";
}
impl ::core::convert::From<NDStorageFileHelper> for ::windows::core::IUnknown {
    fn from(value: NDStorageFileHelper) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NDStorageFileHelper> for ::windows::core::IUnknown {
    fn from(value: &NDStorageFileHelper) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NDStorageFileHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NDStorageFileHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NDStorageFileHelper> for ::windows::core::IInspectable {
    fn from(value: NDStorageFileHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NDStorageFileHelper> for ::windows::core::IInspectable {
    fn from(value: &NDStorageFileHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NDStorageFileHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NDStorageFileHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<NDStorageFileHelper> for INDStorageFileHelper {
    fn from(value: NDStorageFileHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NDStorageFileHelper> for INDStorageFileHelper {
    fn from(value: &NDStorageFileHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INDStorageFileHelper> for NDStorageFileHelper {
    fn into_param(self) -> ::windows::core::Param<'a, INDStorageFileHelper> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INDStorageFileHelper> for &NDStorageFileHelper {
    fn into_param(self) -> ::windows::core::Param<'a, INDStorageFileHelper> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NDStreamParserNotifier(pub ::windows::core::IInspectable);
impl NDStreamParserNotifier {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NDStreamParserNotifier, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "deprecated")]
    pub fn OnContentIDReceived<'a, Param0: ::windows::core::IntoParam<'a, INDLicenseFetchDescriptor>>(&self, licensefetchdescriptor: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), licensefetchdescriptor.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn OnMediaStreamDescriptorCreated<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::super::Core::AudioStreamDescriptor>>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::super::Core::VideoStreamDescriptor>>>(&self, audiostreamdescriptors: Param0, videostreamdescriptors: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), audiostreamdescriptors.into_param().abi(), videostreamdescriptors.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Media_Core")]
    pub fn OnSampleParsed<'a, Param2: ::windows::core::IntoParam<'a, super::super::Core::MediaStreamSample>>(&self, streamid: u32, streamtype: NDMediaStreamType, streamsample: Param2, pts: i64, ccformat: NDClosedCaptionFormat, ccdatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), streamid, streamtype, streamsample.into_param().abi(), pts, ccformat, ccdatabytes.len() as u32, ::core::mem::transmute(ccdatabytes.as_ptr())).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Media_Core")]
    pub fn OnBeginSetupDecryptor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Core::IMediaStreamDescriptor>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, descriptor: Param0, keyid: Param1, probytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), descriptor.into_param().abi(), keyid.into_param().abi(), probytes.len() as u32, ::core::mem::transmute(probytes.as_ptr())).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for NDStreamParserNotifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDStreamParserNotifier;{c167acd0-2ce6-426c-ace5-5e9275fea715})");
}
unsafe impl ::windows::core::Interface for NDStreamParserNotifier {
    type Vtable = INDStreamParserNotifier_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc167acd0_2ce6_426c_ace5_5e9275fea715);
}
impl ::windows::core::RuntimeName for NDStreamParserNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDStreamParserNotifier";
}
impl ::core::convert::From<NDStreamParserNotifier> for ::windows::core::IUnknown {
    fn from(value: NDStreamParserNotifier) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NDStreamParserNotifier> for ::windows::core::IUnknown {
    fn from(value: &NDStreamParserNotifier) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NDStreamParserNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NDStreamParserNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NDStreamParserNotifier> for ::windows::core::IInspectable {
    fn from(value: NDStreamParserNotifier) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NDStreamParserNotifier> for ::windows::core::IInspectable {
    fn from(value: &NDStreamParserNotifier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NDStreamParserNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NDStreamParserNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<NDStreamParserNotifier> for INDStreamParserNotifier {
    fn from(value: NDStreamParserNotifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NDStreamParserNotifier> for INDStreamParserNotifier {
    fn from(value: &NDStreamParserNotifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INDStreamParserNotifier> for NDStreamParserNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, INDStreamParserNotifier> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INDStreamParserNotifier> for &NDStreamParserNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, INDStreamParserNotifier> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct NDTCPMessenger(pub ::windows::core::IInspectable);
impl NDTCPMessenger {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn SendRegistrationRequestAsync(&self, sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], challengedatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), sessionidbytes.len() as u32, ::core::mem::transmute(sessionidbytes.as_ptr()), challengedatabytes.len() as u32, ::core::mem::transmute(challengedatabytes.as_ptr()), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn SendProximityDetectionStartAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], challengedatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), pdtype, transmitterchannelbytes.len() as u32, ::core::mem::transmute(transmitterchannelbytes.as_ptr()), sessionidbytes.len() as u32, ::core::mem::transmute(sessionidbytes.as_ptr()), challengedatabytes.len() as u32, ::core::mem::transmute(challengedatabytes.as_ptr()), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn SendProximityDetectionResponseAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], responsedatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), pdtype, transmitterchannelbytes.len() as u32, ::core::mem::transmute(transmitterchannelbytes.as_ptr()), sessionidbytes.len() as u32, ::core::mem::transmute(sessionidbytes.as_ptr()), responsedatabytes.len() as u32, ::core::mem::transmute(responsedatabytes.as_ptr()), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn SendLicenseFetchRequestAsync(&self, sessionidbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], challengedatabytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), sessionidbytes.len() as u32, ::core::mem::transmute(sessionidbytes.as_ptr()), challengedatabytes.len() as u32, ::core::mem::transmute(challengedatabytes.as_ptr()), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(remotehostname: Param0, remotehostport: u32) -> ::windows::core::Result<NDTCPMessenger> {
        Self::INDTCPMessengerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), remotehostname.into_param().abi(), remotehostport, &mut result__).from_abi::<NDTCPMessenger>(result__)
        })
    }
    pub fn INDTCPMessengerFactory<R, F: FnOnce(&INDTCPMessengerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NDTCPMessenger, INDTCPMessengerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for NDTCPMessenger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDTCPMessenger;{d42df95d-a75b-47bf-8249-bc83820da38a})");
}
unsafe impl ::windows::core::Interface for NDTCPMessenger {
    type Vtable = INDMessenger_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd42df95d_a75b_47bf_8249_bc83820da38a);
}
impl ::windows::core::RuntimeName for NDTCPMessenger {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDTCPMessenger";
}
impl ::core::convert::From<NDTCPMessenger> for ::windows::core::IUnknown {
    fn from(value: NDTCPMessenger) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&NDTCPMessenger> for ::windows::core::IUnknown {
    fn from(value: &NDTCPMessenger) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NDTCPMessenger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NDTCPMessenger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<NDTCPMessenger> for ::windows::core::IInspectable {
    fn from(value: NDTCPMessenger) -> Self {
        value.0
    }
}
impl ::core::convert::From<&NDTCPMessenger> for ::windows::core::IInspectable {
    fn from(value: &NDTCPMessenger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NDTCPMessenger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NDTCPMessenger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<NDTCPMessenger> for INDMessenger {
    fn from(value: NDTCPMessenger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NDTCPMessenger> for INDMessenger {
    fn from(value: &NDTCPMessenger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, INDMessenger> for NDTCPMessenger {
    fn into_param(self) -> ::windows::core::Param<'a, INDMessenger> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, INDMessenger> for &NDTCPMessenger {
    fn into_param(self) -> ::windows::core::Param<'a, INDMessenger> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlayReadyContentHeader(pub ::windows::core::IInspectable);
impl PlayReadyContentHeader {
    pub fn KeyId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn KeyIdString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LicenseAcquisitionUrl(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LicenseAcquisitionUserInterfaceUrl(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    pub fn DomainServiceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn EncryptionType(&self) -> ::windows::core::Result<PlayReadyEncryptionAlgorithm> {
        let this = self;
        unsafe {
            let mut result__: PlayReadyEncryptionAlgorithm = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadyEncryptionAlgorithm>(result__)
        }
    }
    pub fn CustomAttributes(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DecryptorSetup(&self) -> ::windows::core::Result<PlayReadyDecryptorSetup> {
        let this = self;
        unsafe {
            let mut result__: PlayReadyDecryptorSetup = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadyDecryptorSetup>(result__)
        }
    }
    pub fn GetSerializedHeader(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    pub fn HeaderWithEmbeddedUpdates(&self) -> ::windows::core::Result<PlayReadyContentHeader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadyContentHeader>(result__)
        }
    }
    pub fn KeyIds(&self) -> ::windows::core::Result<::windows::core::Array<::windows::core::GUID>> {
        let this = &::windows::core::Interface::cast::<IPlayReadyContentHeader2>(self)?;
        unsafe {
            let mut result__: ::windows::core::Array<::windows::core::GUID> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::windows::core::Array::<::windows::core::GUID>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    pub fn KeyIdStrings(&self) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IPlayReadyContentHeader2>(self)?;
        unsafe {
            let mut result__: ::windows::core::Array<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), ::windows::core::Array::<::windows::core::HSTRING>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateInstanceFromWindowsMediaDrmHeader<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(headerbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType], licenseacquisitionurl: Param1, licenseacquisitionuserinterfaceurl: Param2, customattributes: Param3, domainserviceid: Param4) -> ::windows::core::Result<PlayReadyContentHeader> {
        Self::IPlayReadyContentHeaderFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), headerbytes.len() as u32, ::core::mem::transmute(headerbytes.as_ptr()), licenseacquisitionurl.into_param().abi(), licenseacquisitionuserinterfaceurl.into_param().abi(), customattributes.into_param().abi(), domainserviceid.into_param().abi(), &mut result__).from_abi::<PlayReadyContentHeader>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateInstanceFromComponents<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param5: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param6: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(contentkeyid: Param0, contentkeyidstring: Param1, contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: Param3, licenseacquisitionuserinterfaceurl: Param4, customattributes: Param5, domainserviceid: Param6) -> ::windows::core::Result<PlayReadyContentHeader> {
        Self::IPlayReadyContentHeaderFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), contentkeyid.into_param().abi(), contentkeyidstring.into_param().abi(), contentencryptionalgorithm, licenseacquisitionurl.into_param().abi(), licenseacquisitionuserinterfaceurl.into_param().abi(), customattributes.into_param().abi(), domainserviceid.into_param().abi(), &mut result__).from_abi::<PlayReadyContentHeader>(result__)
        })
    }
    pub fn CreateInstanceFromPlayReadyHeader(headerbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<PlayReadyContentHeader> {
        Self::IPlayReadyContentHeaderFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), headerbytes.len() as u32, ::core::mem::transmute(headerbytes.as_ptr()), &mut result__).from_abi::<PlayReadyContentHeader>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateInstanceFromComponents2<'a, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param5: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param6: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param7: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(dwflags: u32, contentkeyids: &[<::windows::core::GUID as ::windows::core::DefaultType>::DefaultType], contentkeyidstrings: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType], contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: Param4, licenseacquisitionuserinterfaceurl: Param5, customattributes: Param6, domainserviceid: Param7) -> ::windows::core::Result<PlayReadyContentHeader> {
        Self::IPlayReadyContentHeaderFactory2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dwflags, contentkeyids.len() as u32, ::core::mem::transmute(contentkeyids.as_ptr()), contentkeyidstrings.len() as u32, ::core::mem::transmute(contentkeyidstrings.as_ptr()), contentencryptionalgorithm, licenseacquisitionurl.into_param().abi(), licenseacquisitionuserinterfaceurl.into_param().abi(), customattributes.into_param().abi(), domainserviceid.into_param().abi(), &mut result__).from_abi::<PlayReadyContentHeader>(result__)
        })
    }
    pub fn IPlayReadyContentHeaderFactory<R, F: FnOnce(&IPlayReadyContentHeaderFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayReadyContentHeader, IPlayReadyContentHeaderFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPlayReadyContentHeaderFactory2<R, F: FnOnce(&IPlayReadyContentHeaderFactory2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayReadyContentHeader, IPlayReadyContentHeaderFactory2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyContentHeader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyContentHeader;{9a438a6a-7f4c-452e-88bd-0148c6387a2c})");
}
unsafe impl ::windows::core::Interface for PlayReadyContentHeader {
    type Vtable = IPlayReadyContentHeader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a438a6a_7f4c_452e_88bd_0148c6387a2c);
}
impl ::windows::core::RuntimeName for PlayReadyContentHeader {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyContentHeader";
}
impl ::core::convert::From<PlayReadyContentHeader> for ::windows::core::IUnknown {
    fn from(value: PlayReadyContentHeader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlayReadyContentHeader> for ::windows::core::IUnknown {
    fn from(value: &PlayReadyContentHeader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayReadyContentHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlayReadyContentHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlayReadyContentHeader> for ::windows::core::IInspectable {
    fn from(value: PlayReadyContentHeader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlayReadyContentHeader> for ::windows::core::IInspectable {
    fn from(value: &PlayReadyContentHeader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayReadyContentHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlayReadyContentHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
pub struct PlayReadyContentResolver {}
impl PlayReadyContentResolver {
    pub fn ServiceRequest<'a, Param0: ::windows::core::IntoParam<'a, PlayReadyContentHeader>>(contentheader: Param0) -> ::windows::core::Result<IPlayReadyServiceRequest> {
        Self::IPlayReadyContentResolver(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), contentheader.into_param().abi(), &mut result__).from_abi::<IPlayReadyServiceRequest>(result__)
        })
    }
    pub fn IPlayReadyContentResolver<R, F: FnOnce(&IPlayReadyContentResolver) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayReadyContentResolver, IPlayReadyContentResolver> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PlayReadyContentResolver {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyContentResolver";
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlayReadyDecryptorSetup(pub i32);
impl PlayReadyDecryptorSetup {
    pub const Uninitialized: PlayReadyDecryptorSetup = PlayReadyDecryptorSetup(0i32);
    pub const OnDemand: PlayReadyDecryptorSetup = PlayReadyDecryptorSetup(1i32);
}
impl ::core::convert::From<i32> for PlayReadyDecryptorSetup {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PlayReadyDecryptorSetup {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PlayReadyDecryptorSetup {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyDecryptorSetup;i4)");
}
impl ::windows::core::DefaultType for PlayReadyDecryptorSetup {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlayReadyDomain(pub ::windows::core::IInspectable);
impl PlayReadyDomain {
    pub fn AccountId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn Revision(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DomainJoinUrl(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyDomain {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomain;{adcc93ac-97e6-43ef-95e4-d7868f3b16a9})");
}
unsafe impl ::windows::core::Interface for PlayReadyDomain {
    type Vtable = IPlayReadyDomain_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadcc93ac_97e6_43ef_95e4_d7868f3b16a9);
}
impl ::windows::core::RuntimeName for PlayReadyDomain {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomain";
}
impl ::core::convert::From<PlayReadyDomain> for ::windows::core::IUnknown {
    fn from(value: PlayReadyDomain) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlayReadyDomain> for ::windows::core::IUnknown {
    fn from(value: &PlayReadyDomain) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayReadyDomain {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlayReadyDomain {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlayReadyDomain> for ::windows::core::IInspectable {
    fn from(value: PlayReadyDomain) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlayReadyDomain> for ::windows::core::IInspectable {
    fn from(value: &PlayReadyDomain) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayReadyDomain {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlayReadyDomain {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PlayReadyDomain> for IPlayReadyDomain {
    fn from(value: PlayReadyDomain) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyDomain> for IPlayReadyDomain {
    fn from(value: &PlayReadyDomain) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyDomain> for PlayReadyDomain {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyDomain> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyDomain> for &PlayReadyDomain {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyDomain> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlayReadyDomainIterable(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadyDomainIterable {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(domainaccountid: Param0) -> ::windows::core::Result<PlayReadyDomainIterable> {
        Self::IPlayReadyDomainIterableFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), domainaccountid.into_param().abi(), &mut result__).from_abi::<PlayReadyDomainIterable>(result__)
        })
    }
    pub fn IPlayReadyDomainIterableFactory<R, F: FnOnce(&IPlayReadyDomainIterableFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayReadyDomainIterable, IPlayReadyDomainIterableFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for PlayReadyDomainIterable {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomainIterable;pinterface({faa585ea-6214-4217-afda-7f46de5869b3};{adcc93ac-97e6-43ef-95e4-d7868f3b16a9}))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for PlayReadyDomainIterable {
    type Vtable = super::super::super::Foundation::Collections::IIterable_abi<IPlayReadyDomain>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for PlayReadyDomainIterable {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainIterable";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PlayReadyDomainIterable> for ::windows::core::IUnknown {
    fn from(value: PlayReadyDomainIterable) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PlayReadyDomainIterable> for ::windows::core::IUnknown {
    fn from(value: &PlayReadyDomainIterable) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayReadyDomainIterable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlayReadyDomainIterable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PlayReadyDomainIterable> for ::windows::core::IInspectable {
    fn from(value: PlayReadyDomainIterable) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PlayReadyDomainIterable> for ::windows::core::IInspectable {
    fn from(value: &PlayReadyDomainIterable) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayReadyDomainIterable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlayReadyDomainIterable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PlayReadyDomainIterable> for super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain> {
    fn from(value: PlayReadyDomainIterable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PlayReadyDomainIterable> for super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain> {
    fn from(value: &PlayReadyDomainIterable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain>> for PlayReadyDomainIterable {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain>> for &PlayReadyDomainIterable {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for PlayReadyDomainIterable {
    type Item = IPlayReadyDomain;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &PlayReadyDomainIterable {
    type Item = IPlayReadyDomain;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlayReadyDomainIterator(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadyDomainIterator {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Current(&self) -> ::windows::core::Result<IPlayReadyDomain> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyDomain>(result__)
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
    pub fn GetMany(&self, items: &mut [<IPlayReadyDomain as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for PlayReadyDomainIterator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomainIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};{adcc93ac-97e6-43ef-95e4-d7868f3b16a9}))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for PlayReadyDomainIterator {
    type Vtable = super::super::super::Foundation::Collections::IIterator_abi<IPlayReadyDomain>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for PlayReadyDomainIterator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainIterator";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PlayReadyDomainIterator> for ::windows::core::IUnknown {
    fn from(value: PlayReadyDomainIterator) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PlayReadyDomainIterator> for ::windows::core::IUnknown {
    fn from(value: &PlayReadyDomainIterator) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayReadyDomainIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlayReadyDomainIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PlayReadyDomainIterator> for ::windows::core::IInspectable {
    fn from(value: PlayReadyDomainIterator) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PlayReadyDomainIterator> for ::windows::core::IInspectable {
    fn from(value: &PlayReadyDomainIterator) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayReadyDomainIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlayReadyDomainIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PlayReadyDomainIterator> for super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain> {
    fn from(value: PlayReadyDomainIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PlayReadyDomainIterator> for super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain> {
    fn from(value: &PlayReadyDomainIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain>> for PlayReadyDomainIterator {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain>> for &PlayReadyDomainIterator {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlayReadyDomainJoinServiceRequest(pub ::windows::core::IInspectable);
impl PlayReadyDomainJoinServiceRequest {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayReadyDomainJoinServiceRequest, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn DomainAccountId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn SetDomainAccountId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DomainFriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDomainFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DomainServiceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn SetDomainServiceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetChallengeCustomData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), responsebytes.len() as u32, ::core::mem::transmute(responsebytes.as_ptr()), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyDomainJoinServiceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomainJoinServiceRequest;{171b4a5a-405f-4739-b040-67b9f0c38758})");
}
unsafe impl ::windows::core::Interface for PlayReadyDomainJoinServiceRequest {
    type Vtable = IPlayReadyDomainJoinServiceRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x171b4a5a_405f_4739_b040_67b9f0c38758);
}
impl ::windows::core::RuntimeName for PlayReadyDomainJoinServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainJoinServiceRequest";
}
impl ::core::convert::From<PlayReadyDomainJoinServiceRequest> for ::windows::core::IUnknown {
    fn from(value: PlayReadyDomainJoinServiceRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlayReadyDomainJoinServiceRequest> for ::windows::core::IUnknown {
    fn from(value: &PlayReadyDomainJoinServiceRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlayReadyDomainJoinServiceRequest> for ::windows::core::IInspectable {
    fn from(value: PlayReadyDomainJoinServiceRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlayReadyDomainJoinServiceRequest> for ::windows::core::IInspectable {
    fn from(value: &PlayReadyDomainJoinServiceRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<PlayReadyDomainJoinServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyDomainJoinServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyDomainJoinServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyDomainJoinServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaProtectionServiceRequest> for PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaProtectionServiceRequest> for &PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::core::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PlayReadyDomainJoinServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyDomainJoinServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyDomainJoinServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyDomainJoinServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyServiceRequest> for PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyServiceRequest> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyServiceRequest> for &PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyServiceRequest> {
        ::core::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlayReadyDomainLeaveServiceRequest(pub ::windows::core::IInspectable);
impl PlayReadyDomainLeaveServiceRequest {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayReadyDomainLeaveServiceRequest, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn DomainAccountId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn SetDomainAccountId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DomainServiceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn SetDomainServiceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetChallengeCustomData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), responsebytes.len() as u32, ::core::mem::transmute(responsebytes.as_ptr()), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyDomainLeaveServiceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomainLeaveServiceRequest;{062d58be-97ad-4917-aa03-46d4c252d464})");
}
unsafe impl ::windows::core::Interface for PlayReadyDomainLeaveServiceRequest {
    type Vtable = IPlayReadyDomainLeaveServiceRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x062d58be_97ad_4917_aa03_46d4c252d464);
}
impl ::windows::core::RuntimeName for PlayReadyDomainLeaveServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainLeaveServiceRequest";
}
impl ::core::convert::From<PlayReadyDomainLeaveServiceRequest> for ::windows::core::IUnknown {
    fn from(value: PlayReadyDomainLeaveServiceRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlayReadyDomainLeaveServiceRequest> for ::windows::core::IUnknown {
    fn from(value: &PlayReadyDomainLeaveServiceRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlayReadyDomainLeaveServiceRequest> for ::windows::core::IInspectable {
    fn from(value: PlayReadyDomainLeaveServiceRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlayReadyDomainLeaveServiceRequest> for ::windows::core::IInspectable {
    fn from(value: &PlayReadyDomainLeaveServiceRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<PlayReadyDomainLeaveServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyDomainLeaveServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyDomainLeaveServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyDomainLeaveServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaProtectionServiceRequest> for PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaProtectionServiceRequest> for &PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::core::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PlayReadyDomainLeaveServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyDomainLeaveServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyDomainLeaveServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyDomainLeaveServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyServiceRequest> for PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyServiceRequest> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyServiceRequest> for &PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyServiceRequest> {
        ::core::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlayReadyEncryptionAlgorithm(pub i32);
impl PlayReadyEncryptionAlgorithm {
    pub const Unprotected: PlayReadyEncryptionAlgorithm = PlayReadyEncryptionAlgorithm(0i32);
    pub const Aes128Ctr: PlayReadyEncryptionAlgorithm = PlayReadyEncryptionAlgorithm(1i32);
    pub const Cocktail: PlayReadyEncryptionAlgorithm = PlayReadyEncryptionAlgorithm(4i32);
    pub const Aes128Cbc: PlayReadyEncryptionAlgorithm = PlayReadyEncryptionAlgorithm(5i32);
    pub const Unspecified: PlayReadyEncryptionAlgorithm = PlayReadyEncryptionAlgorithm(65535i32);
    pub const Uninitialized: PlayReadyEncryptionAlgorithm = PlayReadyEncryptionAlgorithm(2147483647i32);
}
impl ::core::convert::From<i32> for PlayReadyEncryptionAlgorithm {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PlayReadyEncryptionAlgorithm {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PlayReadyEncryptionAlgorithm {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyEncryptionAlgorithm;i4)");
}
impl ::windows::core::DefaultType for PlayReadyEncryptionAlgorithm {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlayReadyHardwareDRMFeatures(pub i32);
impl PlayReadyHardwareDRMFeatures {
    pub const HardwareDRM: PlayReadyHardwareDRMFeatures = PlayReadyHardwareDRMFeatures(1i32);
    pub const HEVC: PlayReadyHardwareDRMFeatures = PlayReadyHardwareDRMFeatures(2i32);
    pub const Aes128Cbc: PlayReadyHardwareDRMFeatures = PlayReadyHardwareDRMFeatures(3i32);
}
impl ::core::convert::From<i32> for PlayReadyHardwareDRMFeatures {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PlayReadyHardwareDRMFeatures {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PlayReadyHardwareDRMFeatures {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyHardwareDRMFeatures;i4)");
}
impl ::windows::core::DefaultType for PlayReadyHardwareDRMFeatures {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlayReadyITADataFormat(pub i32);
impl PlayReadyITADataFormat {
    pub const SerializedProperties: PlayReadyITADataFormat = PlayReadyITADataFormat(0i32);
    pub const SerializedProperties_WithContentProtectionWrapper: PlayReadyITADataFormat = PlayReadyITADataFormat(1i32);
}
impl ::core::convert::From<i32> for PlayReadyITADataFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PlayReadyITADataFormat {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PlayReadyITADataFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyITADataFormat;i4)");
}
impl ::windows::core::DefaultType for PlayReadyITADataFormat {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlayReadyITADataGenerator(pub ::windows::core::IInspectable);
impl PlayReadyITADataGenerator {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayReadyITADataGenerator, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GenerateData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IPropertySet>>(&self, guidcpsystemid: Param0, countofstreams: u32, configuration: Param2, format: PlayReadyITADataFormat) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), guidcpsystemid.into_param().abi(), countofstreams, configuration.into_param().abi(), format, ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyITADataGenerator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyITADataGenerator;{24446b8e-10b9-4530-b25b-901a8029a9b2})");
}
unsafe impl ::windows::core::Interface for PlayReadyITADataGenerator {
    type Vtable = IPlayReadyITADataGenerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24446b8e_10b9_4530_b25b_901a8029a9b2);
}
impl ::windows::core::RuntimeName for PlayReadyITADataGenerator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyITADataGenerator";
}
impl ::core::convert::From<PlayReadyITADataGenerator> for ::windows::core::IUnknown {
    fn from(value: PlayReadyITADataGenerator) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlayReadyITADataGenerator> for ::windows::core::IUnknown {
    fn from(value: &PlayReadyITADataGenerator) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayReadyITADataGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlayReadyITADataGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlayReadyITADataGenerator> for ::windows::core::IInspectable {
    fn from(value: PlayReadyITADataGenerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlayReadyITADataGenerator> for ::windows::core::IInspectable {
    fn from(value: &PlayReadyITADataGenerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayReadyITADataGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlayReadyITADataGenerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlayReadyIndividualizationServiceRequest(pub ::windows::core::IInspectable);
impl PlayReadyIndividualizationServiceRequest {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayReadyIndividualizationServiceRequest, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetChallengeCustomData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), responsebytes.len() as u32, ::core::mem::transmute(responsebytes.as_ptr()), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyIndividualizationServiceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyIndividualizationServiceRequest;{21f5a86b-008c-4611-ab2f-aaa6c69f0e24})");
}
unsafe impl ::windows::core::Interface for PlayReadyIndividualizationServiceRequest {
    type Vtable = IPlayReadyIndividualizationServiceRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21f5a86b_008c_4611_ab2f_aaa6c69f0e24);
}
impl ::windows::core::RuntimeName for PlayReadyIndividualizationServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyIndividualizationServiceRequest";
}
impl ::core::convert::From<PlayReadyIndividualizationServiceRequest> for ::windows::core::IUnknown {
    fn from(value: PlayReadyIndividualizationServiceRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlayReadyIndividualizationServiceRequest> for ::windows::core::IUnknown {
    fn from(value: &PlayReadyIndividualizationServiceRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlayReadyIndividualizationServiceRequest> for ::windows::core::IInspectable {
    fn from(value: PlayReadyIndividualizationServiceRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlayReadyIndividualizationServiceRequest> for ::windows::core::IInspectable {
    fn from(value: &PlayReadyIndividualizationServiceRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<PlayReadyIndividualizationServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyIndividualizationServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyIndividualizationServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyIndividualizationServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaProtectionServiceRequest> for PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaProtectionServiceRequest> for &PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::core::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PlayReadyIndividualizationServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyIndividualizationServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyIndividualizationServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyIndividualizationServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyServiceRequest> for PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyServiceRequest> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyServiceRequest> for &PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyServiceRequest> {
        ::core::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlayReadyLicense(pub ::windows::core::IInspectable);
impl PlayReadyLicense {
    pub fn FullyEvaluated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn UsableForPlay(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExpirationDate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    pub fn ExpireAfterFirstPlay(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn DomainAccountID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn ChainDepth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn GetKIDAtChainDepth(&self, chaindepth: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), chaindepth, &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn SecureStopId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn SecurityLevel(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn InMemoryOnly(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ExpiresInRealTime(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyLicense {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicense;{ee474c4e-fa3c-414d-a9f2-3ffc1ef832d4})");
}
unsafe impl ::windows::core::Interface for PlayReadyLicense {
    type Vtable = IPlayReadyLicense_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee474c4e_fa3c_414d_a9f2_3ffc1ef832d4);
}
impl ::windows::core::RuntimeName for PlayReadyLicense {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicense";
}
impl ::core::convert::From<PlayReadyLicense> for ::windows::core::IUnknown {
    fn from(value: PlayReadyLicense) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlayReadyLicense> for ::windows::core::IUnknown {
    fn from(value: &PlayReadyLicense) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayReadyLicense {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlayReadyLicense {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlayReadyLicense> for ::windows::core::IInspectable {
    fn from(value: PlayReadyLicense) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlayReadyLicense> for ::windows::core::IInspectable {
    fn from(value: &PlayReadyLicense) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayReadyLicense {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlayReadyLicense {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PlayReadyLicense> for IPlayReadyLicense {
    fn from(value: PlayReadyLicense) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyLicense> for IPlayReadyLicense {
    fn from(value: &PlayReadyLicense) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyLicense> for PlayReadyLicense {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyLicense> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyLicense> for &PlayReadyLicense {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyLicense> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlayReadyLicenseAcquisitionServiceRequest(pub ::windows::core::IInspectable);
impl PlayReadyLicenseAcquisitionServiceRequest {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayReadyLicenseAcquisitionServiceRequest, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ContentHeader(&self) -> ::windows::core::Result<PlayReadyContentHeader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadyContentHeader>(result__)
        }
    }
    pub fn SetContentHeader<'a, Param0: ::windows::core::IntoParam<'a, PlayReadyContentHeader>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DomainServiceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn SetDomainServiceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetChallengeCustomData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), responsebytes.len() as u32, ::core::mem::transmute(responsebytes.as_ptr()), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    pub fn SessionId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<IPlayReadyLicenseAcquisitionServiceRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateLicenseIterable<'a, Param0: ::windows::core::IntoParam<'a, PlayReadyContentHeader>>(&self, contentheader: Param0, fullyevaluated: bool) -> ::windows::core::Result<PlayReadyLicenseIterable> {
        let this = &::windows::core::Interface::cast::<IPlayReadyLicenseAcquisitionServiceRequest3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), contentheader.into_param().abi(), fullyevaluated, &mut result__).from_abi::<PlayReadyLicenseIterable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyLicenseAcquisitionServiceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicenseAcquisitionServiceRequest;{5d85ff45-3e9f-4f48-93e1-9530c8d58c3e})");
}
unsafe impl ::windows::core::Interface for PlayReadyLicenseAcquisitionServiceRequest {
    type Vtable = IPlayReadyLicenseAcquisitionServiceRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d85ff45_3e9f_4f48_93e1_9530c8d58c3e);
}
impl ::windows::core::RuntimeName for PlayReadyLicenseAcquisitionServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseAcquisitionServiceRequest";
}
impl ::core::convert::From<PlayReadyLicenseAcquisitionServiceRequest> for ::windows::core::IUnknown {
    fn from(value: PlayReadyLicenseAcquisitionServiceRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlayReadyLicenseAcquisitionServiceRequest> for ::windows::core::IUnknown {
    fn from(value: &PlayReadyLicenseAcquisitionServiceRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlayReadyLicenseAcquisitionServiceRequest> for ::windows::core::IInspectable {
    fn from(value: PlayReadyLicenseAcquisitionServiceRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlayReadyLicenseAcquisitionServiceRequest> for ::windows::core::IInspectable {
    fn from(value: &PlayReadyLicenseAcquisitionServiceRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyLicenseAcquisitionServiceRequest {
    fn from(value: PlayReadyLicenseAcquisitionServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyLicenseAcquisitionServiceRequest {
    fn from(value: &PlayReadyLicenseAcquisitionServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyLicenseAcquisitionServiceRequest> for PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyLicenseAcquisitionServiceRequest> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyLicenseAcquisitionServiceRequest> for &PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyLicenseAcquisitionServiceRequest> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PlayReadyLicenseAcquisitionServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyLicenseAcquisitionServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaProtectionServiceRequest> for PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaProtectionServiceRequest> for &PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::core::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyLicenseAcquisitionServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyServiceRequest> for PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyServiceRequest> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyServiceRequest> for &PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyServiceRequest> {
        ::core::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlayReadyLicenseIterable(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadyLicenseIterable {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayReadyLicenseIterable, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, PlayReadyContentHeader>>(contentheader: Param0, fullyevaluated: bool) -> ::windows::core::Result<PlayReadyLicenseIterable> {
        Self::IPlayReadyLicenseIterableFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), contentheader.into_param().abi(), fullyevaluated, &mut result__).from_abi::<PlayReadyLicenseIterable>(result__)
        })
    }
    pub fn IPlayReadyLicenseIterableFactory<R, F: FnOnce(&IPlayReadyLicenseIterableFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayReadyLicenseIterable, IPlayReadyLicenseIterableFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for PlayReadyLicenseIterable {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicenseIterable;pinterface({faa585ea-6214-4217-afda-7f46de5869b3};{ee474c4e-fa3c-414d-a9f2-3ffc1ef832d4}))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for PlayReadyLicenseIterable {
    type Vtable = super::super::super::Foundation::Collections::IIterable_abi<IPlayReadyLicense>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for PlayReadyLicenseIterable {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseIterable";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PlayReadyLicenseIterable> for ::windows::core::IUnknown {
    fn from(value: PlayReadyLicenseIterable) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PlayReadyLicenseIterable> for ::windows::core::IUnknown {
    fn from(value: &PlayReadyLicenseIterable) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayReadyLicenseIterable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlayReadyLicenseIterable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PlayReadyLicenseIterable> for ::windows::core::IInspectable {
    fn from(value: PlayReadyLicenseIterable) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PlayReadyLicenseIterable> for ::windows::core::IInspectable {
    fn from(value: &PlayReadyLicenseIterable) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayReadyLicenseIterable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlayReadyLicenseIterable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PlayReadyLicenseIterable> for super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense> {
    fn from(value: PlayReadyLicenseIterable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PlayReadyLicenseIterable> for super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense> {
    fn from(value: &PlayReadyLicenseIterable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense>> for PlayReadyLicenseIterable {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense>> for &PlayReadyLicenseIterable {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for PlayReadyLicenseIterable {
    type Item = IPlayReadyLicense;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &PlayReadyLicenseIterable {
    type Item = IPlayReadyLicense;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlayReadyLicenseIterator(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadyLicenseIterator {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Current(&self) -> ::windows::core::Result<IPlayReadyLicense> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyLicense>(result__)
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
    pub fn GetMany(&self, items: &mut [<IPlayReadyLicense as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for PlayReadyLicenseIterator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicenseIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};{ee474c4e-fa3c-414d-a9f2-3ffc1ef832d4}))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for PlayReadyLicenseIterator {
    type Vtable = super::super::super::Foundation::Collections::IIterator_abi<IPlayReadyLicense>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for PlayReadyLicenseIterator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseIterator";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PlayReadyLicenseIterator> for ::windows::core::IUnknown {
    fn from(value: PlayReadyLicenseIterator) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PlayReadyLicenseIterator> for ::windows::core::IUnknown {
    fn from(value: &PlayReadyLicenseIterator) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayReadyLicenseIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlayReadyLicenseIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PlayReadyLicenseIterator> for ::windows::core::IInspectable {
    fn from(value: PlayReadyLicenseIterator) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PlayReadyLicenseIterator> for ::windows::core::IInspectable {
    fn from(value: &PlayReadyLicenseIterator) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayReadyLicenseIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlayReadyLicenseIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PlayReadyLicenseIterator> for super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense> {
    fn from(value: PlayReadyLicenseIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PlayReadyLicenseIterator> for super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense> {
    fn from(value: &PlayReadyLicenseIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense>> for PlayReadyLicenseIterator {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense>> for &PlayReadyLicenseIterator {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
pub struct PlayReadyLicenseManagement {}
impl PlayReadyLicenseManagement {
    #[cfg(feature = "Foundation")]
    pub fn DeleteLicenses<'a, Param0: ::windows::core::IntoParam<'a, PlayReadyContentHeader>>(contentheader: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IPlayReadyLicenseManagement(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), contentheader.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn IPlayReadyLicenseManagement<R, F: FnOnce(&IPlayReadyLicenseManagement) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayReadyLicenseManagement, IPlayReadyLicenseManagement> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PlayReadyLicenseManagement {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseManagement";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlayReadyLicenseSession(pub ::windows::core::IInspectable);
impl PlayReadyLicenseSession {
    pub fn CreateLAServiceRequest(&self) -> ::windows::core::Result<IPlayReadyLicenseAcquisitionServiceRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyLicenseAcquisitionServiceRequest>(result__)
        }
    }
    pub fn ConfigureMediaProtectionManager<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProtectionManager>>(&self, mpm: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), mpm.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IPropertySet>>(configuration: Param0) -> ::windows::core::Result<PlayReadyLicenseSession> {
        Self::IPlayReadyLicenseSessionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), configuration.into_param().abi(), &mut result__).from_abi::<PlayReadyLicenseSession>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateLicenseIterable<'a, Param0: ::windows::core::IntoParam<'a, PlayReadyContentHeader>>(&self, contentheader: Param0, fullyevaluated: bool) -> ::windows::core::Result<PlayReadyLicenseIterable> {
        let this = &::windows::core::Interface::cast::<IPlayReadyLicenseSession2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), contentheader.into_param().abi(), fullyevaluated, &mut result__).from_abi::<PlayReadyLicenseIterable>(result__)
        }
    }
    pub fn IPlayReadyLicenseSessionFactory<R, F: FnOnce(&IPlayReadyLicenseSessionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayReadyLicenseSession, IPlayReadyLicenseSessionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyLicenseSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicenseSession;{a1723a39-87fa-4fdd-abbb-a9720e845259})");
}
unsafe impl ::windows::core::Interface for PlayReadyLicenseSession {
    type Vtable = IPlayReadyLicenseSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1723a39_87fa_4fdd_abbb_a9720e845259);
}
impl ::windows::core::RuntimeName for PlayReadyLicenseSession {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseSession";
}
impl ::core::convert::From<PlayReadyLicenseSession> for ::windows::core::IUnknown {
    fn from(value: PlayReadyLicenseSession) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlayReadyLicenseSession> for ::windows::core::IUnknown {
    fn from(value: &PlayReadyLicenseSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayReadyLicenseSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlayReadyLicenseSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlayReadyLicenseSession> for ::windows::core::IInspectable {
    fn from(value: PlayReadyLicenseSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlayReadyLicenseSession> for ::windows::core::IInspectable {
    fn from(value: &PlayReadyLicenseSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayReadyLicenseSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlayReadyLicenseSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PlayReadyLicenseSession> for IPlayReadyLicenseSession {
    fn from(value: PlayReadyLicenseSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadyLicenseSession> for IPlayReadyLicenseSession {
    fn from(value: &PlayReadyLicenseSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyLicenseSession> for PlayReadyLicenseSession {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyLicenseSession> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyLicenseSession> for &PlayReadyLicenseSession {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyLicenseSession> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PlayReadyLicenseSession> for IPlayReadyLicenseSession2 {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyLicenseSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyLicenseSession> for IPlayReadyLicenseSession2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyLicenseSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyLicenseSession2> for PlayReadyLicenseSession {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyLicenseSession2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyLicenseSession2> for &PlayReadyLicenseSession {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyLicenseSession2> {
        ::core::convert::TryInto::<IPlayReadyLicenseSession2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlayReadyMeteringReportServiceRequest(pub ::windows::core::IInspectable);
impl PlayReadyMeteringReportServiceRequest {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayReadyMeteringReportServiceRequest, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn MeteringCertificate(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    pub fn SetMeteringCertificate(&self, meteringcertbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), meteringcertbytes.len() as u32, ::core::mem::transmute(meteringcertbytes.as_ptr())).ok() }
    }
    pub fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetChallengeCustomData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), responsebytes.len() as u32, ::core::mem::transmute(responsebytes.as_ptr()), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyMeteringReportServiceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyMeteringReportServiceRequest;{c12b231c-0ecd-4f11-a185-1e24a4a67fb7})");
}
unsafe impl ::windows::core::Interface for PlayReadyMeteringReportServiceRequest {
    type Vtable = IPlayReadyMeteringReportServiceRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc12b231c_0ecd_4f11_a185_1e24a4a67fb7);
}
impl ::windows::core::RuntimeName for PlayReadyMeteringReportServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyMeteringReportServiceRequest";
}
impl ::core::convert::From<PlayReadyMeteringReportServiceRequest> for ::windows::core::IUnknown {
    fn from(value: PlayReadyMeteringReportServiceRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlayReadyMeteringReportServiceRequest> for ::windows::core::IUnknown {
    fn from(value: &PlayReadyMeteringReportServiceRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlayReadyMeteringReportServiceRequest> for ::windows::core::IInspectable {
    fn from(value: PlayReadyMeteringReportServiceRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlayReadyMeteringReportServiceRequest> for ::windows::core::IInspectable {
    fn from(value: &PlayReadyMeteringReportServiceRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<PlayReadyMeteringReportServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyMeteringReportServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyMeteringReportServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyMeteringReportServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaProtectionServiceRequest> for PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaProtectionServiceRequest> for &PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::core::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PlayReadyMeteringReportServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyMeteringReportServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyMeteringReportServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyMeteringReportServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyServiceRequest> for PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyServiceRequest> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyServiceRequest> for &PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyServiceRequest> {
        ::core::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlayReadyRevocationServiceRequest(pub ::windows::core::IInspectable);
impl PlayReadyRevocationServiceRequest {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayReadyRevocationServiceRequest, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetChallengeCustomData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), responsebytes.len() as u32, ::core::mem::transmute(responsebytes.as_ptr()), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadyRevocationServiceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyRevocationServiceRequest;{543d66ac-faf0-4560-84a5-0e4acec939e4})");
}
unsafe impl ::windows::core::Interface for PlayReadyRevocationServiceRequest {
    type Vtable = IPlayReadyRevocationServiceRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x543d66ac_faf0_4560_84a5_0e4acec939e4);
}
impl ::windows::core::RuntimeName for PlayReadyRevocationServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyRevocationServiceRequest";
}
impl ::core::convert::From<PlayReadyRevocationServiceRequest> for ::windows::core::IUnknown {
    fn from(value: PlayReadyRevocationServiceRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlayReadyRevocationServiceRequest> for ::windows::core::IUnknown {
    fn from(value: &PlayReadyRevocationServiceRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlayReadyRevocationServiceRequest> for ::windows::core::IInspectable {
    fn from(value: PlayReadyRevocationServiceRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlayReadyRevocationServiceRequest> for ::windows::core::IInspectable {
    fn from(value: &PlayReadyRevocationServiceRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<PlayReadyRevocationServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyRevocationServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyRevocationServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyRevocationServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaProtectionServiceRequest> for PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaProtectionServiceRequest> for &PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::core::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PlayReadyRevocationServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadyRevocationServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadyRevocationServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadyRevocationServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyServiceRequest> for PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyServiceRequest> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyServiceRequest> for &PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyServiceRequest> {
        ::core::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlayReadySecureStopIterable(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadySecureStopIterable {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateInstance(publishercertbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<PlayReadySecureStopIterable> {
        Self::IPlayReadySecureStopIterableFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), publishercertbytes.len() as u32, ::core::mem::transmute(publishercertbytes.as_ptr()), &mut result__).from_abi::<PlayReadySecureStopIterable>(result__)
        })
    }
    pub fn IPlayReadySecureStopIterableFactory<R, F: FnOnce(&IPlayReadySecureStopIterableFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayReadySecureStopIterable, IPlayReadySecureStopIterableFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for PlayReadySecureStopIterable {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadySecureStopIterable;pinterface({faa585ea-6214-4217-afda-7f46de5869b3};{b5501ee5-01bf-4401-9677-05630a6a4cc8}))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for PlayReadySecureStopIterable {
    type Vtable = super::super::super::Foundation::Collections::IIterable_abi<IPlayReadySecureStopServiceRequest>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for PlayReadySecureStopIterable {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySecureStopIterable";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PlayReadySecureStopIterable> for ::windows::core::IUnknown {
    fn from(value: PlayReadySecureStopIterable) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PlayReadySecureStopIterable> for ::windows::core::IUnknown {
    fn from(value: &PlayReadySecureStopIterable) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayReadySecureStopIterable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlayReadySecureStopIterable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PlayReadySecureStopIterable> for ::windows::core::IInspectable {
    fn from(value: PlayReadySecureStopIterable) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PlayReadySecureStopIterable> for ::windows::core::IInspectable {
    fn from(value: &PlayReadySecureStopIterable) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayReadySecureStopIterable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlayReadySecureStopIterable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PlayReadySecureStopIterable> for super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest> {
    fn from(value: PlayReadySecureStopIterable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PlayReadySecureStopIterable> for super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest> {
    fn from(value: &PlayReadySecureStopIterable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest>> for PlayReadySecureStopIterable {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest>> for &PlayReadySecureStopIterable {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for PlayReadySecureStopIterable {
    type Item = IPlayReadySecureStopServiceRequest;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &PlayReadySecureStopIterable {
    type Item = IPlayReadySecureStopServiceRequest;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlayReadySecureStopIterator(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadySecureStopIterator {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Current(&self) -> ::windows::core::Result<IPlayReadySecureStopServiceRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadySecureStopServiceRequest>(result__)
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
    pub fn GetMany(&self, items: &mut [<IPlayReadySecureStopServiceRequest as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for PlayReadySecureStopIterator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadySecureStopIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};{b5501ee5-01bf-4401-9677-05630a6a4cc8}))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for PlayReadySecureStopIterator {
    type Vtable = super::super::super::Foundation::Collections::IIterator_abi<IPlayReadySecureStopServiceRequest>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for PlayReadySecureStopIterator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySecureStopIterator";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PlayReadySecureStopIterator> for ::windows::core::IUnknown {
    fn from(value: PlayReadySecureStopIterator) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PlayReadySecureStopIterator> for ::windows::core::IUnknown {
    fn from(value: &PlayReadySecureStopIterator) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayReadySecureStopIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlayReadySecureStopIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PlayReadySecureStopIterator> for ::windows::core::IInspectable {
    fn from(value: PlayReadySecureStopIterator) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PlayReadySecureStopIterator> for ::windows::core::IInspectable {
    fn from(value: &PlayReadySecureStopIterator) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayReadySecureStopIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlayReadySecureStopIterator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PlayReadySecureStopIterator> for super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest> {
    fn from(value: PlayReadySecureStopIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PlayReadySecureStopIterator> for super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest> {
    fn from(value: &PlayReadySecureStopIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>> for PlayReadySecureStopIterator {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>> for &PlayReadySecureStopIterator {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlayReadySecureStopServiceRequest(pub ::windows::core::IInspectable);
impl PlayReadySecureStopServiceRequest {
    pub fn SessionID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UpdateTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn Stopped(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn PublisherCertificate(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    pub fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetChallengeCustomData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows::core::Result<IPlayReadyServiceRequest> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::core::Result<PlayReadySoapMessage> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), responsebytes.len() as u32, ::core::mem::transmute(responsebytes.as_ptr()), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    pub fn CreateInstance(publishercertbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<PlayReadySecureStopServiceRequest> {
        Self::IPlayReadySecureStopServiceRequestFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), publishercertbytes.len() as u32, ::core::mem::transmute(publishercertbytes.as_ptr()), &mut result__).from_abi::<PlayReadySecureStopServiceRequest>(result__)
        })
    }
    pub fn CreateInstanceFromSessionID<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(sessionid: Param0, publishercertbytes: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<PlayReadySecureStopServiceRequest> {
        Self::IPlayReadySecureStopServiceRequestFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), sessionid.into_param().abi(), publishercertbytes.len() as u32, ::core::mem::transmute(publishercertbytes.as_ptr()), &mut result__).from_abi::<PlayReadySecureStopServiceRequest>(result__)
        })
    }
    pub fn IPlayReadySecureStopServiceRequestFactory<R, F: FnOnce(&IPlayReadySecureStopServiceRequestFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayReadySecureStopServiceRequest, IPlayReadySecureStopServiceRequestFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadySecureStopServiceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadySecureStopServiceRequest;{b5501ee5-01bf-4401-9677-05630a6a4cc8})");
}
unsafe impl ::windows::core::Interface for PlayReadySecureStopServiceRequest {
    type Vtable = IPlayReadySecureStopServiceRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5501ee5_01bf_4401_9677_05630a6a4cc8);
}
impl ::windows::core::RuntimeName for PlayReadySecureStopServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySecureStopServiceRequest";
}
impl ::core::convert::From<PlayReadySecureStopServiceRequest> for ::windows::core::IUnknown {
    fn from(value: PlayReadySecureStopServiceRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlayReadySecureStopServiceRequest> for ::windows::core::IUnknown {
    fn from(value: &PlayReadySecureStopServiceRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlayReadySecureStopServiceRequest> for ::windows::core::IInspectable {
    fn from(value: PlayReadySecureStopServiceRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlayReadySecureStopServiceRequest> for ::windows::core::IInspectable {
    fn from(value: &PlayReadySecureStopServiceRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PlayReadySecureStopServiceRequest> for IPlayReadySecureStopServiceRequest {
    fn from(value: PlayReadySecureStopServiceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayReadySecureStopServiceRequest> for IPlayReadySecureStopServiceRequest {
    fn from(value: &PlayReadySecureStopServiceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadySecureStopServiceRequest> for PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadySecureStopServiceRequest> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadySecureStopServiceRequest> for &PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadySecureStopServiceRequest> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PlayReadySecureStopServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadySecureStopServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaProtectionServiceRequest> for PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IMediaProtectionServiceRequest> for &PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, super::IMediaProtectionServiceRequest> {
        ::core::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PlayReadySecureStopServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: PlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PlayReadySecureStopServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlayReadySecureStopServiceRequest) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyServiceRequest> for PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyServiceRequest> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPlayReadyServiceRequest> for &PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, IPlayReadyServiceRequest> {
        ::core::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlayReadySoapMessage(pub ::windows::core::IInspectable);
impl PlayReadySoapMessage {
    pub fn GetMessageBody(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MessageHeaders(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PlayReadySoapMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadySoapMessage;{b659fcb5-ce41-41ba-8a0d-61df5fffa139})");
}
unsafe impl ::windows::core::Interface for PlayReadySoapMessage {
    type Vtable = IPlayReadySoapMessage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb659fcb5_ce41_41ba_8a0d_61df5fffa139);
}
impl ::windows::core::RuntimeName for PlayReadySoapMessage {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySoapMessage";
}
impl ::core::convert::From<PlayReadySoapMessage> for ::windows::core::IUnknown {
    fn from(value: PlayReadySoapMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlayReadySoapMessage> for ::windows::core::IUnknown {
    fn from(value: &PlayReadySoapMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayReadySoapMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlayReadySoapMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlayReadySoapMessage> for ::windows::core::IInspectable {
    fn from(value: PlayReadySoapMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlayReadySoapMessage> for ::windows::core::IInspectable {
    fn from(value: &PlayReadySoapMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayReadySoapMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlayReadySoapMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
pub struct PlayReadyStatics {}
impl PlayReadyStatics {
    pub fn DomainJoinServiceRequestType() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn DomainLeaveServiceRequestType() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn IndividualizationServiceRequestType() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn LicenseAcquirerServiceRequestType() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn MeteringReportServiceRequestType() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn RevocationServiceRequestType() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn MediaProtectionSystemId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn PlayReadySecurityVersion() -> ::windows::core::Result<u32> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    pub fn PlayReadyCertificateSecurityLevel() -> ::windows::core::Result<u32> {
        Self::IPlayReadyStatics2(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    pub fn SecureStopServiceRequestType() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IPlayReadyStatics3(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn CheckSupportedHardware(hwdrmfeature: PlayReadyHardwareDRMFeatures) -> ::windows::core::Result<bool> {
        Self::IPlayReadyStatics3(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), hwdrmfeature, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn InputTrustAuthorityToCreate() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPlayReadyStatics4(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ProtectionSystemId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IPlayReadyStatics4(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn HardwareDRMDisabledAtTime() -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        Self::IPlayReadyStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn HardwareDRMDisabledUntilTime() -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        Self::IPlayReadyStatics5(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        })
    }
    pub fn ResetHardwareDRMDisabled() -> ::windows::core::Result<()> {
        Self::IPlayReadyStatics5(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() })
    }
    pub fn IPlayReadyStatics<R, F: FnOnce(&IPlayReadyStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayReadyStatics, IPlayReadyStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPlayReadyStatics2<R, F: FnOnce(&IPlayReadyStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayReadyStatics, IPlayReadyStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPlayReadyStatics3<R, F: FnOnce(&IPlayReadyStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayReadyStatics, IPlayReadyStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPlayReadyStatics4<R, F: FnOnce(&IPlayReadyStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayReadyStatics, IPlayReadyStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPlayReadyStatics5<R, F: FnOnce(&IPlayReadyStatics5) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayReadyStatics, IPlayReadyStatics5> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PlayReadyStatics {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyStatics";
}
