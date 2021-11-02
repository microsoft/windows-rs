#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct INDClient(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INDClient {
    type Vtable = INDClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1003911195, 25016, 18146, [153, 165, 138, 188, 182, 185, 247, 214]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INDClient_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contenturl: ::windows::runtime::RawPtr, startasyncoptions: u32, registrationcustomdata: ::windows::runtime::RawPtr, licensefetchdescriptor: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, licensefetchdescriptor: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, registrationcustomdata: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct INDClientFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INDClientFactory {
    type Vtable = INDClientFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1045683554, 65256, 17695, [176, 212, 247, 6, 204, 163, 224, 55]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INDClientFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, downloadengine: ::windows::runtime::RawPtr, streamparser: ::windows::runtime::RawPtr, pmessenger: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct INDClosedCaptionDataReceivedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INDClosedCaptionDataReceivedEventArgs {
    type Vtable = INDClosedCaptionDataReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1194906271, 49989, 17993, [132, 104, 184, 197, 252, 53, 113, 144]);
}
impl INDClosedCaptionDataReceivedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ClosedCaptionDataFormat(&self) -> ::windows::runtime::Result<NDClosedCaptionFormat> {
        let this = self;
        unsafe {
            let mut result__: NDClosedCaptionFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NDClosedCaptionFormat>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn PresentationTimestamp(&self) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ClosedCaptionData(&self) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INDClosedCaptionDataReceivedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{4738d29f-c345-4649-8468-b8c5fc357190}");
}
#[repr(C)]
#[doc(hidden)]
pub struct INDClosedCaptionDataReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut NDClosedCaptionFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct INDCustomData(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INDCustomData {
    type Vtable = INDCustomData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4123725788, 11529, 20249, [181, 225, 118, 160, 179, 238, 146, 103]);
}
impl INDCustomData {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn CustomDataTypeID(&self) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn CustomData(&self) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INDCustomData {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{f5cb0fdc-2d09-4f19-b5e1-76a0b3ee9267}");
}
#[repr(C)]
#[doc(hidden)]
pub struct INDCustomData_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct INDCustomDataFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INDCustomDataFactory {
    type Vtable = INDCustomDataFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3595830699, 13348, 18483, [140, 154, 175, 95, 222, 178, 40, 114]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INDCustomDataFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, customDataTypeIDBytes_array_size: u32, customdatatypeidbytes: *const u8, customDataBytes_array_size: u32, customdatabytes: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct INDDownloadEngine(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INDDownloadEngine {
    type Vtable = INDDownloadEngine_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(757218661, 50358, 17464, [141, 70, 185, 110, 109, 15, 178, 31]);
}
impl INDDownloadEngine {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn Open<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, uri: Param0, sessionidbytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), uri.into_param().abi(), sessionidbytes.len() as u32, ::std::mem::transmute(sessionidbytes.as_ptr())).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn Pause(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn Resume(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn Seek<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, startposition: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), startposition.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn CanSeek(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn BufferFullMinThresholdInSamples(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn BufferFullMaxThresholdInSamples(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn Notifier(&self) -> ::windows::runtime::Result<NDDownloadEngineNotifier> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NDDownloadEngineNotifier>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INDDownloadEngine {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{2d223d65-c4b6-4438-8d46-b96e6d0fb21f}");
}
#[repr(C)]
#[doc(hidden)]
pub struct INDDownloadEngine_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, sessionIDBytes_array_size: u32, sessionidbytes: *const u8) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startposition: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct INDDownloadEngineNotifier(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INDDownloadEngineNotifier {
    type Vtable = INDDownloadEngineNotifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3609244884, 62648, 17712, [168, 9, 145, 147, 165, 113, 231, 252]);
}
impl INDDownloadEngineNotifier {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn OnStreamOpened(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn OnPlayReadyObjectReceived(&self, databytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), databytes.len() as u32, ::std::mem::transmute(databytes.as_ptr())).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn OnContentIDReceived<'a, Param0: ::windows::runtime::IntoParam<'a, INDLicenseFetchDescriptor>>(&self, licensefetchdescriptor: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), licensefetchdescriptor.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn OnDataReceived(&self, databytes: &[<u8 as ::windows::runtime::Abi>::DefaultType], bytesreceived: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), databytes.len() as u32, ::std::mem::transmute(databytes.as_ptr()), bytesreceived).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn OnEndOfStream(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn OnNetworkError(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INDDownloadEngineNotifier {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{d720b4d4-f4b8-4530-a809-9193a571e7fc}");
}
#[repr(C)]
#[doc(hidden)]
pub struct INDDownloadEngineNotifier_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dataBytes_array_size: u32, databytes: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, licensefetchdescriptor: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dataBytes_array_size: u32, databytes: *const u8, bytesreceived: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct INDLicenseFetchCompletedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INDLicenseFetchCompletedEventArgs {
    type Vtable = INDLicenseFetchCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(518195738, 4530, 17752, [136, 101, 227, 165, 22, 146, 37, 23]);
}
impl INDLicenseFetchCompletedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ResponseCustomData(&self) -> ::windows::runtime::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<INDCustomData>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INDLicenseFetchCompletedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{1ee30a1a-11b2-4558-8865-e3a516922517}");
}
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchCompletedEventArgs_abi(
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
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct INDLicenseFetchDescriptor(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INDLicenseFetchDescriptor {
    type Vtable = INDLicenseFetchDescriptor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1419301690, 59014, 18741, [165, 103, 124, 167, 122, 210, 15, 164]);
}
impl INDLicenseFetchDescriptor {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ContentIDType(&self) -> ::windows::runtime::Result<NDContentIDType> {
        let this = self;
        unsafe {
            let mut result__: NDContentIDType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NDContentIDType>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ContentID(&self) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn LicenseFetchChallengeCustomData(&self) -> ::windows::runtime::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<INDCustomData>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetLicenseFetchChallengeCustomData<'a, Param0: ::windows::runtime::IntoParam<'a, INDCustomData>>(&self, licensefetchchallengecustomdata: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), licensefetchchallengecustomdata.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INDLicenseFetchDescriptor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{5498d33a-e686-4935-a567-7ca77ad20fa4}");
}
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchDescriptor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut NDContentIDType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, licensefetchchallengecustomdata: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct INDLicenseFetchDescriptorFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INDLicenseFetchDescriptorFactory {
    type Vtable = INDLicenseFetchDescriptorFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3489862146, 53164, 20224, [174, 106, 151, 175, 128, 184, 72, 242]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchDescriptorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contentidtype: NDContentIDType, contentIDBytes_array_size: u32, contentidbytes: *const u8, licensefetchchallengecustomdata: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct INDLicenseFetchResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INDLicenseFetchResult {
    type Vtable = INDLicenseFetchResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(567514776, 43618, 17919, [165, 255, 128, 55, 229, 67, 56, 37]);
}
impl INDLicenseFetchResult {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ResponseCustomData(&self) -> ::windows::runtime::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<INDCustomData>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INDLicenseFetchResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{21d39698-aa62-45ff-a5ff-8037e5433825}");
}
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchResult_abi(
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
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct INDMessenger(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INDMessenger {
    type Vtable = INDMessenger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3559782749, 42843, 18367, [130, 73, 188, 131, 130, 13, 163, 138]);
}
impl INDMessenger {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn SendRegistrationRequestAsync(&self, sessionidbytes: &[<u8 as ::windows::runtime::Abi>::DefaultType], challengedatabytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), sessionidbytes.len() as u32, ::std::mem::transmute(sessionidbytes.as_ptr()), challengedatabytes.len() as u32, ::std::mem::transmute(challengedatabytes.as_ptr()), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn SendProximityDetectionStartAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[<u8 as ::windows::runtime::Abi>::DefaultType], sessionidbytes: &[<u8 as ::windows::runtime::Abi>::DefaultType], challengedatabytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                pdtype,
                transmitterchannelbytes.len() as u32,
                ::std::mem::transmute(transmitterchannelbytes.as_ptr()),
                sessionidbytes.len() as u32,
                ::std::mem::transmute(sessionidbytes.as_ptr()),
                challengedatabytes.len() as u32,
                ::std::mem::transmute(challengedatabytes.as_ptr()),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn SendProximityDetectionResponseAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[<u8 as ::windows::runtime::Abi>::DefaultType], sessionidbytes: &[<u8 as ::windows::runtime::Abi>::DefaultType], responsedatabytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                pdtype,
                transmitterchannelbytes.len() as u32,
                ::std::mem::transmute(transmitterchannelbytes.as_ptr()),
                sessionidbytes.len() as u32,
                ::std::mem::transmute(sessionidbytes.as_ptr()),
                responsedatabytes.len() as u32,
                ::std::mem::transmute(responsedatabytes.as_ptr()),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn SendLicenseFetchRequestAsync(&self, sessionidbytes: &[<u8 as ::windows::runtime::Abi>::DefaultType], challengedatabytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), sessionidbytes.len() as u32, ::std::mem::transmute(sessionidbytes.as_ptr()), challengedatabytes.len() as u32, ::std::mem::transmute(challengedatabytes.as_ptr()), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INDMessenger {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{d42df95d-a75b-47bf-8249-bc83820da38a}");
}
#[repr(C)]
#[doc(hidden)]
pub struct INDMessenger_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdtype: NDProximityDetectionType, transmitterChannelBytes_array_size: u32, transmitterchannelbytes: *const u8, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdtype: NDProximityDetectionType, transmitterChannelBytes_array_size: u32, transmitterchannelbytes: *const u8, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, responseDataBytes_array_size: u32, responsedatabytes: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct INDProximityDetectionCompletedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INDProximityDetectionCompletedEventArgs {
    type Vtable = INDProximityDetectionCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(712008488, 55845, 20364, [158, 183, 93, 15, 195, 101, 139, 202]);
}
impl INDProximityDetectionCompletedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ProximityDetectionRetryCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INDProximityDetectionCompletedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{2a706328-da25-4f8c-9eb7-5d0fc3658bca}");
}
#[repr(C)]
#[doc(hidden)]
pub struct INDProximityDetectionCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct INDRegistrationCompletedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INDRegistrationCompletedEventArgs {
    type Vtable = INDRegistrationCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2654582349, 43867, 18693, [172, 220, 120, 122, 119, 198, 55, 77]);
}
impl INDRegistrationCompletedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ResponseCustomData(&self) -> ::windows::runtime::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<INDCustomData>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn TransmitterProperties(&self) -> ::windows::runtime::Result<INDTransmitterProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<INDTransmitterProperties>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn TransmitterCertificateAccepted(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetTransmitterCertificateAccepted(&self, accept: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), accept).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INDRegistrationCompletedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{9e39b64d-ab5b-4905-acdc-787a77c6374d}");
}
#[repr(C)]
#[doc(hidden)]
pub struct INDRegistrationCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, accept: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct INDSendResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INDSendResult {
    type Vtable = INDSendResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3815265559, 42372, 18333, [144, 183, 214, 137, 199, 191, 124, 128]);
}
impl INDSendResult {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn Response(&self) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INDSendResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{e3685517-a584-479d-90b7-d689c7bf7c80}");
}
#[repr(C)]
#[doc(hidden)]
pub struct INDSendResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct INDStartResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INDStartResult {
    type Vtable = INDStartResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2046224750, 62735, 16405, [139, 164, 194, 188, 52, 78, 189, 78]);
}
impl INDStartResult {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Media_Core")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Media_Core`*"]
    pub fn MediaStreamSource(&self) -> ::windows::runtime::Result<super::super::Core::MediaStreamSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::MediaStreamSource>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INDStartResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{79f6e96e-f50f-4015-8ba4-c2bc344ebd4e}");
}
#[repr(C)]
#[doc(hidden)]
pub struct INDStartResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct INDStorageFileHelper(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INDStorageFileHelper {
    type Vtable = INDStorageFileHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3639656184, 37330, 19783, [163, 249, 234, 255, 78, 219, 114, 159]);
}
impl INDStorageFileHelper {
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`, `Storage`*"]
    pub fn GetFileURLs<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::IStorageFile>>(&self, file: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INDStorageFileHelper {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{d8f0bef8-91d2-4d47-a3f9-eaff4edb729f}");
}
#[repr(C)]
#[doc(hidden)]
pub struct INDStorageFileHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, file: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct INDStreamParser(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INDStreamParser {
    type Vtable = INDStreamParser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3770327448, 38806, 16841, [134, 149, 89, 67, 126, 103, 230, 106]);
}
impl INDStreamParser {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ParseData(&self, databytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), databytes.len() as u32, ::std::mem::transmute(databytes.as_ptr())).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Media_Core")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Media_Core`*"]
    pub fn GetStreamInformation<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Core::IMediaStreamDescriptor>>(&self, descriptor: Param0, streamtype: &mut NDMediaStreamType) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), descriptor.into_param().abi(), streamtype, &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn BeginOfStream(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn EndOfStream(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn Notifier(&self) -> ::windows::runtime::Result<NDStreamParserNotifier> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NDStreamParserNotifier>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INDStreamParser {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{e0baa198-9796-41c9-8695-59437e67e66a}");
}
#[repr(C)]
#[doc(hidden)]
pub struct INDStreamParser_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dataBytes_array_size: u32, databytes: *const u8) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, descriptor: ::windows::runtime::RawPtr, streamtype: *mut NDMediaStreamType, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct INDStreamParserNotifier(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INDStreamParserNotifier {
    type Vtable = INDStreamParserNotifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3244797136, 11494, 17004, [172, 229, 94, 146, 117, 254, 167, 21]);
}
impl INDStreamParserNotifier {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn OnContentIDReceived<'a, Param0: ::windows::runtime::IntoParam<'a, INDLicenseFetchDescriptor>>(&self, licensefetchdescriptor: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), licensefetchdescriptor.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`, `Media_Core`*"]
    pub fn OnMediaStreamDescriptorCreated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::super::Core::AudioStreamDescriptor>>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::super::Core::VideoStreamDescriptor>>>(&self, audiostreamdescriptors: Param0, videostreamdescriptors: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), audiostreamdescriptors.into_param().abi(), videostreamdescriptors.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Media_Core")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Media_Core`*"]
    pub fn OnSampleParsed<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Core::MediaStreamSample>>(&self, streamid: u32, streamtype: NDMediaStreamType, streamsample: Param2, pts: i64, ccformat: NDClosedCaptionFormat, ccdatabytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), streamid, streamtype, streamsample.into_param().abi(), pts, ccformat, ccdatabytes.len() as u32, ::std::mem::transmute(ccdatabytes.as_ptr())).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Media_Core")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Media_Core`*"]
    pub fn OnBeginSetupDecryptor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Core::IMediaStreamDescriptor>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, descriptor: Param0, keyid: Param1, probytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), descriptor.into_param().abi(), keyid.into_param().abi(), probytes.len() as u32, ::std::mem::transmute(probytes.as_ptr())).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INDStreamParserNotifier {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{c167acd0-2ce6-426c-ace5-5e9275fea715}");
}
#[repr(C)]
#[doc(hidden)]
pub struct INDStreamParserNotifier_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, licensefetchdescriptor: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, audiostreamdescriptors: ::windows::runtime::RawPtr, videostreamdescriptors: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))] usize,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, streamid: u32, streamtype: NDMediaStreamType, streamsample: ::windows::runtime::RawPtr, pts: i64, ccformat: NDClosedCaptionFormat, ccDataBytes_array_size: u32, ccdatabytes: *const u8) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
    #[cfg(feature = "Media_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, descriptor: ::windows::runtime::RawPtr, keyid: ::windows::runtime::GUID, proBytes_array_size: u32, probytes: *const u8) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Core"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct INDTCPMessengerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INDTCPMessengerFactory {
    type Vtable = INDTCPMessengerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2111331582, 7065, 20328, [143, 130, 129, 119, 247, 206, 223, 43]);
}
#[repr(C)]
#[doc(hidden)]
pub struct INDTCPMessengerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remotehostname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, remotehostport: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct INDTransmitterProperties(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INDTransmitterProperties {
    type Vtable = INDTransmitterProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3845566243, 44111, 19164, [140, 102, 79, 247, 194, 112, 45, 214]);
}
impl INDTransmitterProperties {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn CertificateType(&self) -> ::windows::runtime::Result<NDCertificateType> {
        let this = self;
        unsafe {
            let mut result__: NDCertificateType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NDCertificateType>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn PlatformIdentifier(&self) -> ::windows::runtime::Result<NDCertificatePlatformID> {
        let this = self;
        unsafe {
            let mut result__: NDCertificatePlatformID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NDCertificatePlatformID>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SupportedFeatures(&self) -> ::windows::runtime::Result<::windows::runtime::Array<NDCertificateFeature>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<NDCertificateFeature> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<NDCertificateFeature>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SecurityLevel(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SecurityVersion(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn ExpirationDate(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ClientID(&self) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ModelDigest(&self) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ModelManufacturerName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ModelName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ModelNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for INDTransmitterProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{e536af23-ac4f-4adc-8c66-4ff7c2702dd6}");
}
#[repr(C)]
#[doc(hidden)]
pub struct INDTransmitterProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut NDCertificateType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut NDCertificatePlatformID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut NDCertificateFeature) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyContentHeader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyContentHeader {
    type Vtable = IPlayReadyContentHeader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2588117610, 32588, 17710, [136, 189, 1, 72, 198, 56, 122, 44]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PlayReadyEncryptionAlgorithm) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PlayReadyDecryptorSetup) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyContentHeader2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyContentHeader2 {
    type Vtable = IPlayReadyContentHeader2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(899447284, 8576, 18828, [150, 91, 231, 84, 216, 117, 234, 178]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeader2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyContentHeaderFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyContentHeaderFactory {
    type Vtable = IPlayReadyContentHeaderFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3415722239, 46936, 18294, [191, 1, 33, 122, 139, 81, 11, 44]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeaderFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, headerBytes_array_size: u32, headerbytes: *const u8, licenseacquisitionurl: ::windows::runtime::RawPtr, licenseacquisitionuserinterfaceurl: ::windows::runtime::RawPtr, customattributes: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, domainserviceid: ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        contentkeyid: ::windows::runtime::GUID,
        contentkeyidstring: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        contentencryptionalgorithm: PlayReadyEncryptionAlgorithm,
        licenseacquisitionurl: ::windows::runtime::RawPtr,
        licenseacquisitionuserinterfaceurl: ::windows::runtime::RawPtr,
        customattributes: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        domainserviceid: ::windows::runtime::GUID,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, headerBytes_array_size: u32, headerbytes: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyContentHeaderFactory2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyContentHeaderFactory2 {
    type Vtable = IPlayReadyContentHeaderFactory2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3508772085, 44653, 18296, [151, 253, 110, 58, 46, 234, 219, 235]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeaderFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dwflags: u32,
        contentKeyIds_array_size: u32,
        contentkeyids: *const ::windows::runtime::GUID,
        contentKeyIdStrings_array_size: u32,
        contentkeyidstrings: *const ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        contentencryptionalgorithm: PlayReadyEncryptionAlgorithm,
        licenseacquisitionurl: ::windows::runtime::RawPtr,
        licenseacquisitionuserinterfaceurl: ::windows::runtime::RawPtr,
        customattributes: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        domainserviceid: ::windows::runtime::GUID,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyContentResolver(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyContentResolver {
    type Vtable = IPlayReadyContentResolver_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4227671331, 36973, 18818, [166, 184, 104, 73, 86, 90, 124, 232]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentResolver_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contentheader: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct IPlayReadyDomain(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyDomain {
    type Vtable = IPlayReadyDomain_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2915865516, 38886, 17391, [149, 228, 215, 134, 143, 59, 22, 169]);
}
impl IPlayReadyDomain {
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn AccountId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ServiceId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn Revision(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn FriendlyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn DomainJoinUrl(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPlayReadyDomain {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{adcc93ac-97e6-43ef-95e4-d7868f3b16a9}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomain_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyDomainIterableFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyDomainIterableFactory {
    type Vtable = IPlayReadyDomainIterableFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1307804910, 12577, 19955, [165, 232, 208, 194, 76, 5, 0, 252]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomainIterableFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, domainaccountid: ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyDomainJoinServiceRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyDomainJoinServiceRequest {
    type Vtable = IPlayReadyDomainJoinServiceRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(387664474, 16479, 18233, [176, 64, 103, 185, 240, 195, 135, 88]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomainJoinServiceRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyDomainLeaveServiceRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyDomainLeaveServiceRequest {
    type Vtable = IPlayReadyDomainLeaveServiceRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(103635134, 38829, 18711, [170, 3, 70, 212, 194, 82, 212, 100]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomainLeaveServiceRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyITADataGenerator(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyITADataGenerator {
    type Vtable = IPlayReadyITADataGenerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(608463758, 4281, 17712, [178, 91, 144, 26, 128, 41, 169, 178]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyITADataGenerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidcpsystemid: ::windows::runtime::GUID, countofstreams: u32, configuration: ::windows::runtime::RawPtr, format: PlayReadyITADataFormat, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyIndividualizationServiceRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyIndividualizationServiceRequest {
    type Vtable = IPlayReadyIndividualizationServiceRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(569747563, 140, 17937, [171, 47, 170, 166, 198, 159, 14, 36]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyIndividualizationServiceRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct IPlayReadyLicense(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyLicense {
    type Vtable = IPlayReadyLicense_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3997649998, 64060, 16717, [169, 242, 63, 252, 30, 248, 50, 212]);
}
impl IPlayReadyLicense {
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn FullyEvaluated(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn UsableForPlay(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn ExpirationDate(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ExpireAfterFirstPlay(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn DomainAccountID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ChainDepth(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn GetKIDAtChainDepth(&self, chaindepth: u32) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), chaindepth, &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPlayReadyLicense {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{ee474c4e-fa3c-414d-a9f2-3ffc1ef832d4}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicense_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, chaindepth: u32, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyLicense2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyLicense2 {
    type Vtable = IPlayReadyLicense2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(821356455, 55523, 18592, [188, 218, 255, 159, 64, 83, 4, 54]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicense2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct IPlayReadyLicenseAcquisitionServiceRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyLicenseAcquisitionServiceRequest {
    type Vtable = IPlayReadyLicenseAcquisitionServiceRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1569062725, 16031, 20296, [147, 225, 149, 48, 200, 213, 140, 62]);
}
impl IPlayReadyLicenseAcquisitionServiceRequest {
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ContentHeader(&self) -> ::windows::runtime::Result<PlayReadyContentHeader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadyContentHeader>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetContentHeader<'a, Param0: ::windows::runtime::IntoParam<'a, PlayReadyContentHeader>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn DomainServiceId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetDomainServiceId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ProtectionSystem(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn SetUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ResponseCustomData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ChallengeCustomData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetChallengeCustomData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn BeginServiceRequest(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn NextServiceRequest(&self) -> ::windows::runtime::Result<IPlayReadyServiceRequest> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::runtime::Result<PlayReadySoapMessage> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), responsebytes.len() as u32, ::std::mem::transmute(responsebytes.as_ptr()), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPlayReadyLicenseAcquisitionServiceRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{5d85ff45-3e9f-4f48-93e1-9530c8d58c3e}");
}
impl ::std::convert::TryFrom<IPlayReadyLicenseAcquisitionServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IPlayReadyLicenseAcquisitionServiceRequest) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IPlayReadyLicenseAcquisitionServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IPlayReadyLicenseAcquisitionServiceRequest) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaProtectionServiceRequest> for IPlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaProtectionServiceRequest> for &IPlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaProtectionServiceRequest> {
        ::std::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<IPlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IPlayReadyLicenseAcquisitionServiceRequest) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IPlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IPlayReadyLicenseAcquisitionServiceRequest) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyServiceRequest> for IPlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyServiceRequest> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyServiceRequest> for &IPlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyServiceRequest> {
        ::std::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyLicenseAcquisitionServiceRequest2 {
    type Vtable = IPlayReadyLicenseAcquisitionServiceRequest2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3086638773, 65036, 45605, [188, 96, 90, 158, 221, 50, 206, 181]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyLicenseAcquisitionServiceRequest3 {
    type Vtable = IPlayReadyLicenseAcquisitionServiceRequest3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(961437517, 32629, 17165, [178, 231, 127, 117, 243, 75, 45, 117]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contentheader: ::windows::runtime::RawPtr, fullyevaluated: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyLicenseIterableFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyLicenseIterableFactory {
    type Vtable = IPlayReadyLicenseIterableFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3558317832, 2103, 18808, [142, 104, 190, 66, 147, 200, 215, 166]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseIterableFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contentheader: ::windows::runtime::RawPtr, fullyevaluated: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyLicenseManagement(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyLicenseManagement {
    type Vtable = IPlayReadyLicenseManagement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2867536193, 2391, 17413, [184, 146, 139, 243, 236, 93, 173, 217]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseManagement_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contentheader: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct IPlayReadyLicenseSession(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyLicenseSession {
    type Vtable = IPlayReadyLicenseSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2708617785, 34810, 20445, [171, 187, 169, 114, 14, 132, 82, 89]);
}
impl IPlayReadyLicenseSession {
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn CreateLAServiceRequest(&self) -> ::windows::runtime::Result<IPlayReadyLicenseAcquisitionServiceRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyLicenseAcquisitionServiceRequest>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ConfigureMediaProtectionManager<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProtectionManager>>(&self, mpm: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), mpm.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPlayReadyLicenseSession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{a1723a39-87fa-4fdd-abbb-a9720e845259}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseSession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mpm: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct IPlayReadyLicenseSession2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyLicenseSession2 {
    type Vtable = IPlayReadyLicenseSession2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1225375290, 15085, 18006, [138, 215, 238, 15, 215, 121, 149, 16]);
}
impl IPlayReadyLicenseSession2 {
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn CreateLAServiceRequest(&self) -> ::windows::runtime::Result<IPlayReadyLicenseAcquisitionServiceRequest> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyLicenseSession>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyLicenseAcquisitionServiceRequest>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ConfigureMediaProtectionManager<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProtectionManager>>(&self, mpm: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyLicenseSession>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), mpm.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn CreateLicenseIterable<'a, Param0: ::windows::runtime::IntoParam<'a, PlayReadyContentHeader>>(&self, contentheader: Param0, fullyevaluated: bool) -> ::windows::runtime::Result<PlayReadyLicenseIterable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), contentheader.into_param().abi(), fullyevaluated, &mut result__).from_abi::<PlayReadyLicenseIterable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPlayReadyLicenseSession2 {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{4909be3a-3aed-4656-8ad7-ee0fd7799510}");
}
impl ::std::convert::TryFrom<IPlayReadyLicenseSession2> for IPlayReadyLicenseSession {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IPlayReadyLicenseSession2) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IPlayReadyLicenseSession2> for IPlayReadyLicenseSession {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IPlayReadyLicenseSession2) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyLicenseSession> for IPlayReadyLicenseSession2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyLicenseSession> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyLicenseSession> for &IPlayReadyLicenseSession2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyLicenseSession> {
        ::std::convert::TryInto::<IPlayReadyLicenseSession>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseSession2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contentheader: ::windows::runtime::RawPtr, fullyevaluated: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyLicenseSessionFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyLicenseSessionFactory {
    type Vtable = IPlayReadyLicenseSessionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1648961177, 25895, 17054, [152, 190, 72, 215, 152, 172, 39, 57]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseSessionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, configuration: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyMeteringReportServiceRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyMeteringReportServiceRequest {
    type Vtable = IPlayReadyMeteringReportServiceRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3240829724, 3789, 20241, [161, 133, 30, 36, 164, 166, 127, 183]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyMeteringReportServiceRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, meteringCertBytes_array_size: u32, meteringcertbytes: *const u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyRevocationServiceRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyRevocationServiceRequest {
    type Vtable = IPlayReadyRevocationServiceRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1413310124, 64240, 17760, [132, 165, 14, 74, 206, 201, 57, 228]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyRevocationServiceRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadySecureStopIterableFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadySecureStopIterableFactory {
    type Vtable = IPlayReadySecureStopIterableFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1595867493, 16916, 19870, [129, 235, 232, 159, 157, 41, 74, 238]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySecureStopIterableFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, publisherCertBytes_array_size: u32, publishercertbytes: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct IPlayReadySecureStopServiceRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadySecureStopServiceRequest {
    type Vtable = IPlayReadySecureStopServiceRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3041926885, 447, 17409, [150, 119, 5, 99, 10, 106, 76, 200]);
}
impl IPlayReadySecureStopServiceRequest {
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SessionID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn StartTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn UpdateTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn Stopped(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn PublisherCertificate(&self) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ProtectionSystem(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn SetUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ResponseCustomData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ChallengeCustomData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetChallengeCustomData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn BeginServiceRequest(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn NextServiceRequest(&self) -> ::windows::runtime::Result<IPlayReadyServiceRequest> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::runtime::Result<PlayReadySoapMessage> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), responsebytes.len() as u32, ::std::mem::transmute(responsebytes.as_ptr()), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPlayReadySecureStopServiceRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{b5501ee5-01bf-4401-9677-05630a6a4cc8}");
}
impl ::std::convert::TryFrom<IPlayReadySecureStopServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IPlayReadySecureStopServiceRequest) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IPlayReadySecureStopServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IPlayReadySecureStopServiceRequest) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaProtectionServiceRequest> for IPlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaProtectionServiceRequest> for &IPlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaProtectionServiceRequest> {
        ::std::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<IPlayReadySecureStopServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IPlayReadySecureStopServiceRequest) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IPlayReadySecureStopServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IPlayReadySecureStopServiceRequest) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyServiceRequest> for IPlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyServiceRequest> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyServiceRequest> for &IPlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyServiceRequest> {
        ::std::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySecureStopServiceRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadySecureStopServiceRequestFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadySecureStopServiceRequestFactory {
    type Vtable = IPlayReadySecureStopServiceRequestFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(239373001, 59006, 18766, [159, 73, 98, 133, 67, 140, 118, 207]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySecureStopServiceRequestFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, publisherCertBytes_array_size: u32, publishercertbytes: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionid: ::windows::runtime::GUID, publisherCertBytes_array_size: u32, publishercertbytes: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct IPlayReadyServiceRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyServiceRequest {
    type Vtable = IPlayReadyServiceRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2343381046, 42755, 17830, [161, 128, 118, 243, 86, 90, 167, 37]);
}
impl IPlayReadyServiceRequest {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn SetUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ResponseCustomData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ChallengeCustomData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetChallengeCustomData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn BeginServiceRequest(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn NextServiceRequest(&self) -> ::windows::runtime::Result<IPlayReadyServiceRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::runtime::Result<PlayReadySoapMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), responsebytes.len() as u32, ::std::mem::transmute(responsebytes.as_ptr()), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ProtectionSystem(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPlayReadyServiceRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{8bad2836-a703-45a6-a180-76f3565aa725}");
}
impl ::std::convert::TryFrom<IPlayReadyServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IPlayReadyServiceRequest) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IPlayReadyServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IPlayReadyServiceRequest) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaProtectionServiceRequest> for IPlayReadyServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaProtectionServiceRequest> for &IPlayReadyServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaProtectionServiceRequest> {
        ::std::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyServiceRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, responseBytes_array_size: u32, responsebytes: *const u8, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadySoapMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadySoapMessage {
    type Vtable = IPlayReadySoapMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3059350709, 52801, 16826, [138, 13, 97, 223, 95, 255, 161, 57]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySoapMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyStatics {
    type Vtable = IPlayReadyStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1583988749, 9340, 18074, [143, 49, 92, 26, 21, 113, 217, 198]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyStatics2 {
    type Vtable = IPlayReadyStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(529361554, 24474, 16958, [148, 102, 179, 57, 105, 175, 122, 61]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyStatics3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyStatics3 {
    type Vtable = IPlayReadyStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1067663217, 11731, 19437, [174, 73, 247, 20, 142, 99, 231, 16]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwdrmfeature: PlayReadyHardwareDRMFeatures, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyStatics4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyStatics4 {
    type Vtable = IPlayReadyStatics4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1353257728, 55332, 16945, [157, 94, 120, 239, 136, 68, 199, 215]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlayReadyStatics5(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlayReadyStatics5 {
    type Vtable = IPlayReadyStatics5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(587886709, 57248, 20366, [167, 121, 206, 254, 169, 198, 130, 75]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for NDCertificateFeature {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDCertificateFeature {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NDCertificateFeature {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDCertificateFeature;i4)");
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for NDCertificatePlatformID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDCertificatePlatformID {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NDCertificatePlatformID {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDCertificatePlatformID;i4)");
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for NDCertificateType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDCertificateType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NDCertificateType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDCertificateType;i4)");
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NDClient(::windows::runtime::IInspectable);
impl NDClient {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn RegistrationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<NDClient, INDRegistrationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn RemoveRegistrationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn ProximityDetectionCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<NDClient, INDProximityDetectionCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn RemoveProximityDetectionCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn LicenseFetchCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<NDClient, INDLicenseFetchCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn RemoveLicenseFetchCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn ReRegistrationNeeded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<NDClient, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn RemoveReRegistrationNeeded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn ClosedCaptionDataReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<NDClient, INDClosedCaptionDataReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn RemoveClosedCaptionDataReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn StartAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>, Param2: ::windows::runtime::IntoParam<'a, INDCustomData>, Param3: ::windows::runtime::IntoParam<'a, INDLicenseFetchDescriptor>>(&self, contenturl: Param0, startasyncoptions: u32, registrationcustomdata: Param2, licensefetchdescriptor: Param3) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<INDStartResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), contenturl.into_param().abi(), startasyncoptions, registrationcustomdata.into_param().abi(), licensefetchdescriptor.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<INDStartResult>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn LicenseFetchAsync<'a, Param0: ::windows::runtime::IntoParam<'a, INDLicenseFetchDescriptor>>(&self, licensefetchdescriptor: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<INDLicenseFetchResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), licensefetchdescriptor.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<INDLicenseFetchResult>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn ReRegistrationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, INDCustomData>>(&self, registrationcustomdata: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), registrationcustomdata.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, INDDownloadEngine>, Param1: ::windows::runtime::IntoParam<'a, INDStreamParser>, Param2: ::windows::runtime::IntoParam<'a, INDMessenger>>(downloadengine: Param0, streamparser: Param1, pmessenger: Param2) -> ::windows::runtime::Result<NDClient> {
        Self::INDClientFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), downloadengine.into_param().abi(), streamparser.into_param().abi(), pmessenger.into_param().abi(), &mut result__).from_abi::<NDClient>(result__)
        })
    }
    pub fn INDClientFactory<R, F: FnOnce(&INDClientFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<NDClient, INDClientFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NDClient {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDClient;{3bd6781b-61b8-46e2-99a5-8abcb6b9f7d6})");
}
unsafe impl ::windows::runtime::Interface for NDClient {
    type Vtable = INDClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1003911195, 25016, 18146, [153, 165, 138, 188, 182, 185, 247, 214]);
}
impl ::windows::runtime::RuntimeName for NDClient {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDClient";
}
impl ::std::convert::From<NDClient> for ::windows::runtime::IUnknown {
    fn from(value: NDClient) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NDClient> for ::windows::runtime::IUnknown {
    fn from(value: &NDClient) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NDClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NDClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<NDClient> for ::windows::runtime::IInspectable {
    fn from(value: NDClient) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NDClient> for ::windows::runtime::IInspectable {
    fn from(value: &NDClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NDClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NDClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDClosedCaptionFormat(pub i32);
impl NDClosedCaptionFormat {
    pub const ATSC: NDClosedCaptionFormat = NDClosedCaptionFormat(0i32);
    pub const SCTE20: NDClosedCaptionFormat = NDClosedCaptionFormat(1i32);
    pub const Unknown: NDClosedCaptionFormat = NDClosedCaptionFormat(2i32);
}
impl ::std::convert::From<i32> for NDClosedCaptionFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDClosedCaptionFormat {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NDClosedCaptionFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDClosedCaptionFormat;i4)");
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDContentIDType(pub i32);
impl NDContentIDType {
    pub const KeyID: NDContentIDType = NDContentIDType(1i32);
    pub const PlayReadyObject: NDContentIDType = NDContentIDType(2i32);
    pub const Custom: NDContentIDType = NDContentIDType(3i32);
}
impl ::std::convert::From<i32> for NDContentIDType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDContentIDType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NDContentIDType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDContentIDType;i4)");
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NDCustomData(::windows::runtime::IInspectable);
impl NDCustomData {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn CustomDataTypeID(&self) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn CustomData(&self) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn CreateInstance(customdatatypeidbytes: &[<u8 as ::windows::runtime::Abi>::DefaultType], customdatabytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<NDCustomData> {
        Self::INDCustomDataFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), customdatatypeidbytes.len() as u32, ::std::mem::transmute(customdatatypeidbytes.as_ptr()), customdatabytes.len() as u32, ::std::mem::transmute(customdatabytes.as_ptr()), &mut result__).from_abi::<NDCustomData>(result__)
        })
    }
    pub fn INDCustomDataFactory<R, F: FnOnce(&INDCustomDataFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<NDCustomData, INDCustomDataFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NDCustomData {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDCustomData;{f5cb0fdc-2d09-4f19-b5e1-76a0b3ee9267})");
}
unsafe impl ::windows::runtime::Interface for NDCustomData {
    type Vtable = INDCustomData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4123725788, 11529, 20249, [181, 225, 118, 160, 179, 238, 146, 103]);
}
impl ::windows::runtime::RuntimeName for NDCustomData {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDCustomData";
}
impl ::std::convert::From<NDCustomData> for ::windows::runtime::IUnknown {
    fn from(value: NDCustomData) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NDCustomData> for ::windows::runtime::IUnknown {
    fn from(value: &NDCustomData) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NDCustomData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NDCustomData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<NDCustomData> for ::windows::runtime::IInspectable {
    fn from(value: NDCustomData) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NDCustomData> for ::windows::runtime::IInspectable {
    fn from(value: &NDCustomData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NDCustomData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NDCustomData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<NDCustomData> for INDCustomData {
    fn from(value: NDCustomData) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NDCustomData> for INDCustomData {
    fn from(value: &NDCustomData) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INDCustomData> for NDCustomData {
    fn into_param(self) -> ::windows::runtime::Param<'a, INDCustomData> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INDCustomData>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INDCustomData> for &NDCustomData {
    fn into_param(self) -> ::windows::runtime::Param<'a, INDCustomData> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INDCustomData>::into(::std::clone::Clone::clone(self)))
    }
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NDDownloadEngineNotifier(::windows::runtime::IInspectable);
impl NDDownloadEngineNotifier {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<NDDownloadEngineNotifier, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn OnStreamOpened(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn OnPlayReadyObjectReceived(&self, databytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), databytes.len() as u32, ::std::mem::transmute(databytes.as_ptr())).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn OnContentIDReceived<'a, Param0: ::windows::runtime::IntoParam<'a, INDLicenseFetchDescriptor>>(&self, licensefetchdescriptor: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), licensefetchdescriptor.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn OnDataReceived(&self, databytes: &[<u8 as ::windows::runtime::Abi>::DefaultType], bytesreceived: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), databytes.len() as u32, ::std::mem::transmute(databytes.as_ptr()), bytesreceived).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn OnEndOfStream(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn OnNetworkError(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NDDownloadEngineNotifier {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDDownloadEngineNotifier;{d720b4d4-f4b8-4530-a809-9193a571e7fc})");
}
unsafe impl ::windows::runtime::Interface for NDDownloadEngineNotifier {
    type Vtable = INDDownloadEngineNotifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3609244884, 62648, 17712, [168, 9, 145, 147, 165, 113, 231, 252]);
}
impl ::windows::runtime::RuntimeName for NDDownloadEngineNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDDownloadEngineNotifier";
}
impl ::std::convert::From<NDDownloadEngineNotifier> for ::windows::runtime::IUnknown {
    fn from(value: NDDownloadEngineNotifier) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NDDownloadEngineNotifier> for ::windows::runtime::IUnknown {
    fn from(value: &NDDownloadEngineNotifier) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NDDownloadEngineNotifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NDDownloadEngineNotifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<NDDownloadEngineNotifier> for ::windows::runtime::IInspectable {
    fn from(value: NDDownloadEngineNotifier) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NDDownloadEngineNotifier> for ::windows::runtime::IInspectable {
    fn from(value: &NDDownloadEngineNotifier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NDDownloadEngineNotifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NDDownloadEngineNotifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<NDDownloadEngineNotifier> for INDDownloadEngineNotifier {
    fn from(value: NDDownloadEngineNotifier) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NDDownloadEngineNotifier> for INDDownloadEngineNotifier {
    fn from(value: &NDDownloadEngineNotifier) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INDDownloadEngineNotifier> for NDDownloadEngineNotifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, INDDownloadEngineNotifier> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INDDownloadEngineNotifier>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INDDownloadEngineNotifier> for &NDDownloadEngineNotifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, INDDownloadEngineNotifier> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INDDownloadEngineNotifier>::into(::std::clone::Clone::clone(self)))
    }
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NDLicenseFetchDescriptor(::windows::runtime::IInspectable);
impl NDLicenseFetchDescriptor {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ContentIDType(&self) -> ::windows::runtime::Result<NDContentIDType> {
        let this = self;
        unsafe {
            let mut result__: NDContentIDType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<NDContentIDType>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ContentID(&self) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn LicenseFetchChallengeCustomData(&self) -> ::windows::runtime::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<INDCustomData>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetLicenseFetchChallengeCustomData<'a, Param0: ::windows::runtime::IntoParam<'a, INDCustomData>>(&self, licensefetchchallengecustomdata: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), licensefetchchallengecustomdata.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn CreateInstance<'a, Param2: ::windows::runtime::IntoParam<'a, INDCustomData>>(contentidtype: NDContentIDType, contentidbytes: &[<u8 as ::windows::runtime::Abi>::DefaultType], licensefetchchallengecustomdata: Param2) -> ::windows::runtime::Result<NDLicenseFetchDescriptor> {
        Self::INDLicenseFetchDescriptorFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), contentidtype, contentidbytes.len() as u32, ::std::mem::transmute(contentidbytes.as_ptr()), licensefetchchallengecustomdata.into_param().abi(), &mut result__).from_abi::<NDLicenseFetchDescriptor>(result__)
        })
    }
    pub fn INDLicenseFetchDescriptorFactory<R, F: FnOnce(&INDLicenseFetchDescriptorFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<NDLicenseFetchDescriptor, INDLicenseFetchDescriptorFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NDLicenseFetchDescriptor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDLicenseFetchDescriptor;{5498d33a-e686-4935-a567-7ca77ad20fa4})");
}
unsafe impl ::windows::runtime::Interface for NDLicenseFetchDescriptor {
    type Vtable = INDLicenseFetchDescriptor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1419301690, 59014, 18741, [165, 103, 124, 167, 122, 210, 15, 164]);
}
impl ::windows::runtime::RuntimeName for NDLicenseFetchDescriptor {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDLicenseFetchDescriptor";
}
impl ::std::convert::From<NDLicenseFetchDescriptor> for ::windows::runtime::IUnknown {
    fn from(value: NDLicenseFetchDescriptor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NDLicenseFetchDescriptor> for ::windows::runtime::IUnknown {
    fn from(value: &NDLicenseFetchDescriptor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<NDLicenseFetchDescriptor> for ::windows::runtime::IInspectable {
    fn from(value: NDLicenseFetchDescriptor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NDLicenseFetchDescriptor> for ::windows::runtime::IInspectable {
    fn from(value: &NDLicenseFetchDescriptor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<NDLicenseFetchDescriptor> for INDLicenseFetchDescriptor {
    fn from(value: NDLicenseFetchDescriptor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NDLicenseFetchDescriptor> for INDLicenseFetchDescriptor {
    fn from(value: &NDLicenseFetchDescriptor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INDLicenseFetchDescriptor> for NDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, INDLicenseFetchDescriptor> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INDLicenseFetchDescriptor>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INDLicenseFetchDescriptor> for &NDLicenseFetchDescriptor {
    fn into_param(self) -> ::windows::runtime::Param<'a, INDLicenseFetchDescriptor> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INDLicenseFetchDescriptor>::into(::std::clone::Clone::clone(self)))
    }
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDMediaStreamType(pub i32);
impl NDMediaStreamType {
    pub const Audio: NDMediaStreamType = NDMediaStreamType(1i32);
    pub const Video: NDMediaStreamType = NDMediaStreamType(2i32);
}
impl ::std::convert::From<i32> for NDMediaStreamType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDMediaStreamType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NDMediaStreamType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDMediaStreamType;i4)");
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDProximityDetectionType(pub i32);
impl NDProximityDetectionType {
    pub const UDP: NDProximityDetectionType = NDProximityDetectionType(1i32);
    pub const TCP: NDProximityDetectionType = NDProximityDetectionType(2i32);
    pub const TransportAgnostic: NDProximityDetectionType = NDProximityDetectionType(4i32);
}
impl ::std::convert::From<i32> for NDProximityDetectionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDProximityDetectionType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NDProximityDetectionType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDProximityDetectionType;i4)");
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDStartAsyncOptions(pub i32);
impl NDStartAsyncOptions {
    pub const MutualAuthentication: NDStartAsyncOptions = NDStartAsyncOptions(1i32);
    pub const WaitForLicenseDescriptor: NDStartAsyncOptions = NDStartAsyncOptions(2i32);
}
impl ::std::convert::From<i32> for NDStartAsyncOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDStartAsyncOptions {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for NDStartAsyncOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDStartAsyncOptions;i4)");
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NDStorageFileHelper(::windows::runtime::IInspectable);
impl NDStorageFileHelper {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<NDStorageFileHelper, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`, `Storage`*"]
    pub fn GetFileURLs<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::IStorageFile>>(&self, file: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NDStorageFileHelper {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDStorageFileHelper;{d8f0bef8-91d2-4d47-a3f9-eaff4edb729f})");
}
unsafe impl ::windows::runtime::Interface for NDStorageFileHelper {
    type Vtable = INDStorageFileHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3639656184, 37330, 19783, [163, 249, 234, 255, 78, 219, 114, 159]);
}
impl ::windows::runtime::RuntimeName for NDStorageFileHelper {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDStorageFileHelper";
}
impl ::std::convert::From<NDStorageFileHelper> for ::windows::runtime::IUnknown {
    fn from(value: NDStorageFileHelper) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NDStorageFileHelper> for ::windows::runtime::IUnknown {
    fn from(value: &NDStorageFileHelper) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NDStorageFileHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NDStorageFileHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<NDStorageFileHelper> for ::windows::runtime::IInspectable {
    fn from(value: NDStorageFileHelper) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NDStorageFileHelper> for ::windows::runtime::IInspectable {
    fn from(value: &NDStorageFileHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NDStorageFileHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NDStorageFileHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<NDStorageFileHelper> for INDStorageFileHelper {
    fn from(value: NDStorageFileHelper) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NDStorageFileHelper> for INDStorageFileHelper {
    fn from(value: &NDStorageFileHelper) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INDStorageFileHelper> for NDStorageFileHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, INDStorageFileHelper> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INDStorageFileHelper>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INDStorageFileHelper> for &NDStorageFileHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, INDStorageFileHelper> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INDStorageFileHelper>::into(::std::clone::Clone::clone(self)))
    }
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NDStreamParserNotifier(::windows::runtime::IInspectable);
impl NDStreamParserNotifier {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<NDStreamParserNotifier, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn OnContentIDReceived<'a, Param0: ::windows::runtime::IntoParam<'a, INDLicenseFetchDescriptor>>(&self, licensefetchdescriptor: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), licensefetchdescriptor.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`, `Media_Core`*"]
    pub fn OnMediaStreamDescriptorCreated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::super::Core::AudioStreamDescriptor>>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVector<super::super::Core::VideoStreamDescriptor>>>(&self, audiostreamdescriptors: Param0, videostreamdescriptors: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), audiostreamdescriptors.into_param().abi(), videostreamdescriptors.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Media_Core")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Media_Core`*"]
    pub fn OnSampleParsed<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Core::MediaStreamSample>>(&self, streamid: u32, streamtype: NDMediaStreamType, streamsample: Param2, pts: i64, ccformat: NDClosedCaptionFormat, ccdatabytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), streamid, streamtype, streamsample.into_param().abi(), pts, ccformat, ccdatabytes.len() as u32, ::std::mem::transmute(ccdatabytes.as_ptr())).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Media_Core")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Media_Core`*"]
    pub fn OnBeginSetupDecryptor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Core::IMediaStreamDescriptor>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, descriptor: Param0, keyid: Param1, probytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), descriptor.into_param().abi(), keyid.into_param().abi(), probytes.len() as u32, ::std::mem::transmute(probytes.as_ptr())).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NDStreamParserNotifier {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDStreamParserNotifier;{c167acd0-2ce6-426c-ace5-5e9275fea715})");
}
unsafe impl ::windows::runtime::Interface for NDStreamParserNotifier {
    type Vtable = INDStreamParserNotifier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3244797136, 11494, 17004, [172, 229, 94, 146, 117, 254, 167, 21]);
}
impl ::windows::runtime::RuntimeName for NDStreamParserNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDStreamParserNotifier";
}
impl ::std::convert::From<NDStreamParserNotifier> for ::windows::runtime::IUnknown {
    fn from(value: NDStreamParserNotifier) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NDStreamParserNotifier> for ::windows::runtime::IUnknown {
    fn from(value: &NDStreamParserNotifier) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NDStreamParserNotifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NDStreamParserNotifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<NDStreamParserNotifier> for ::windows::runtime::IInspectable {
    fn from(value: NDStreamParserNotifier) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NDStreamParserNotifier> for ::windows::runtime::IInspectable {
    fn from(value: &NDStreamParserNotifier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NDStreamParserNotifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NDStreamParserNotifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<NDStreamParserNotifier> for INDStreamParserNotifier {
    fn from(value: NDStreamParserNotifier) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NDStreamParserNotifier> for INDStreamParserNotifier {
    fn from(value: &NDStreamParserNotifier) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INDStreamParserNotifier> for NDStreamParserNotifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, INDStreamParserNotifier> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INDStreamParserNotifier>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INDStreamParserNotifier> for &NDStreamParserNotifier {
    fn into_param(self) -> ::windows::runtime::Param<'a, INDStreamParserNotifier> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INDStreamParserNotifier>::into(::std::clone::Clone::clone(self)))
    }
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct NDTCPMessenger(::windows::runtime::IInspectable);
impl NDTCPMessenger {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn SendRegistrationRequestAsync(&self, sessionidbytes: &[<u8 as ::windows::runtime::Abi>::DefaultType], challengedatabytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), sessionidbytes.len() as u32, ::std::mem::transmute(sessionidbytes.as_ptr()), challengedatabytes.len() as u32, ::std::mem::transmute(challengedatabytes.as_ptr()), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn SendProximityDetectionStartAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[<u8 as ::windows::runtime::Abi>::DefaultType], sessionidbytes: &[<u8 as ::windows::runtime::Abi>::DefaultType], challengedatabytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                pdtype,
                transmitterchannelbytes.len() as u32,
                ::std::mem::transmute(transmitterchannelbytes.as_ptr()),
                sessionidbytes.len() as u32,
                ::std::mem::transmute(sessionidbytes.as_ptr()),
                challengedatabytes.len() as u32,
                ::std::mem::transmute(challengedatabytes.as_ptr()),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn SendProximityDetectionResponseAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[<u8 as ::windows::runtime::Abi>::DefaultType], sessionidbytes: &[<u8 as ::windows::runtime::Abi>::DefaultType], responsedatabytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                pdtype,
                transmitterchannelbytes.len() as u32,
                ::std::mem::transmute(transmitterchannelbytes.as_ptr()),
                sessionidbytes.len() as u32,
                ::std::mem::transmute(sessionidbytes.as_ptr()),
                responsedatabytes.len() as u32,
                ::std::mem::transmute(responsedatabytes.as_ptr()),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn SendLicenseFetchRequestAsync(&self, sessionidbytes: &[<u8 as ::windows::runtime::Abi>::DefaultType], challengedatabytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), sessionidbytes.len() as u32, ::std::mem::transmute(sessionidbytes.as_ptr()), challengedatabytes.len() as u32, ::std::mem::transmute(challengedatabytes.as_ptr()), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<INDSendResult>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(remotehostname: Param0, remotehostport: u32) -> ::windows::runtime::Result<NDTCPMessenger> {
        Self::INDTCPMessengerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), remotehostname.into_param().abi(), remotehostport, &mut result__).from_abi::<NDTCPMessenger>(result__)
        })
    }
    pub fn INDTCPMessengerFactory<R, F: FnOnce(&INDTCPMessengerFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<NDTCPMessenger, INDTCPMessengerFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NDTCPMessenger {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDTCPMessenger;{d42df95d-a75b-47bf-8249-bc83820da38a})");
}
unsafe impl ::windows::runtime::Interface for NDTCPMessenger {
    type Vtable = INDMessenger_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3559782749, 42843, 18367, [130, 73, 188, 131, 130, 13, 163, 138]);
}
impl ::windows::runtime::RuntimeName for NDTCPMessenger {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDTCPMessenger";
}
impl ::std::convert::From<NDTCPMessenger> for ::windows::runtime::IUnknown {
    fn from(value: NDTCPMessenger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NDTCPMessenger> for ::windows::runtime::IUnknown {
    fn from(value: &NDTCPMessenger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for NDTCPMessenger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &NDTCPMessenger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<NDTCPMessenger> for ::windows::runtime::IInspectable {
    fn from(value: NDTCPMessenger) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NDTCPMessenger> for ::windows::runtime::IInspectable {
    fn from(value: &NDTCPMessenger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for NDTCPMessenger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a NDTCPMessenger {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<NDTCPMessenger> for INDMessenger {
    fn from(value: NDTCPMessenger) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NDTCPMessenger> for INDMessenger {
    fn from(value: &NDTCPMessenger) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INDMessenger> for NDTCPMessenger {
    fn into_param(self) -> ::windows::runtime::Param<'a, INDMessenger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INDMessenger>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, INDMessenger> for &NDTCPMessenger {
    fn into_param(self) -> ::windows::runtime::Param<'a, INDMessenger> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<INDMessenger>::into(::std::clone::Clone::clone(self)))
    }
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PlayReadyContentHeader(::windows::runtime::IInspectable);
impl PlayReadyContentHeader {
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn KeyId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn KeyIdString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn LicenseAcquisitionUrl(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn LicenseAcquisitionUserInterfaceUrl(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn DomainServiceId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn EncryptionType(&self) -> ::windows::runtime::Result<PlayReadyEncryptionAlgorithm> {
        let this = self;
        unsafe {
            let mut result__: PlayReadyEncryptionAlgorithm = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadyEncryptionAlgorithm>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn CustomAttributes(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn DecryptorSetup(&self) -> ::windows::runtime::Result<PlayReadyDecryptorSetup> {
        let this = self;
        unsafe {
            let mut result__: PlayReadyDecryptorSetup = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadyDecryptorSetup>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn GetSerializedHeader(&self) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn HeaderWithEmbeddedUpdates(&self) -> ::windows::runtime::Result<PlayReadyContentHeader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadyContentHeader>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn KeyIds(&self) -> ::windows::runtime::Result<::windows::runtime::Array<::windows::runtime::GUID>> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyContentHeader2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::Array<::windows::runtime::GUID> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<::windows::runtime::GUID>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn KeyIdStrings(&self) -> ::windows::runtime::Result<::windows::runtime::Array<::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyContentHeader2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::Array<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<::windows::runtime::HSTRING>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn CreateInstanceFromWindowsMediaDrmHeader<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(
        headerbytes: &[<u8 as ::windows::runtime::Abi>::DefaultType],
        licenseacquisitionurl: Param1,
        licenseacquisitionuserinterfaceurl: Param2,
        customattributes: Param3,
        domainserviceid: Param4,
    ) -> ::windows::runtime::Result<PlayReadyContentHeader> {
        Self::IPlayReadyContentHeaderFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), headerbytes.len() as u32, ::std::mem::transmute(headerbytes.as_ptr()), licenseacquisitionurl.into_param().abi(), licenseacquisitionuserinterfaceurl.into_param().abi(), customattributes.into_param().abi(), domainserviceid.into_param().abi(), &mut result__).from_abi::<PlayReadyContentHeader>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn CreateInstanceFromComponents<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>, Param5: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param6: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(
        contentkeyid: Param0,
        contentkeyidstring: Param1,
        contentencryptionalgorithm: PlayReadyEncryptionAlgorithm,
        licenseacquisitionurl: Param3,
        licenseacquisitionuserinterfaceurl: Param4,
        customattributes: Param5,
        domainserviceid: Param6,
    ) -> ::windows::runtime::Result<PlayReadyContentHeader> {
        Self::IPlayReadyContentHeaderFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                contentkeyid.into_param().abi(),
                contentkeyidstring.into_param().abi(),
                contentencryptionalgorithm,
                licenseacquisitionurl.into_param().abi(),
                licenseacquisitionuserinterfaceurl.into_param().abi(),
                customattributes.into_param().abi(),
                domainserviceid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PlayReadyContentHeader>(result__)
        })
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn CreateInstanceFromPlayReadyHeader(headerbytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<PlayReadyContentHeader> {
        Self::IPlayReadyContentHeaderFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), headerbytes.len() as u32, ::std::mem::transmute(headerbytes.as_ptr()), &mut result__).from_abi::<PlayReadyContentHeader>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn CreateInstanceFromComponents2<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>, Param5: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>, Param6: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param7: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(
        dwflags: u32,
        contentkeyids: &[<::windows::runtime::GUID as ::windows::runtime::Abi>::DefaultType],
        contentkeyidstrings: &[<::windows::runtime::HSTRING as ::windows::runtime::Abi>::DefaultType],
        contentencryptionalgorithm: PlayReadyEncryptionAlgorithm,
        licenseacquisitionurl: Param4,
        licenseacquisitionuserinterfaceurl: Param5,
        customattributes: Param6,
        domainserviceid: Param7,
    ) -> ::windows::runtime::Result<PlayReadyContentHeader> {
        Self::IPlayReadyContentHeaderFactory2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                dwflags,
                contentkeyids.len() as u32,
                ::std::mem::transmute(contentkeyids.as_ptr()),
                contentkeyidstrings.len() as u32,
                ::std::mem::transmute(contentkeyidstrings.as_ptr()),
                contentencryptionalgorithm,
                licenseacquisitionurl.into_param().abi(),
                licenseacquisitionuserinterfaceurl.into_param().abi(),
                customattributes.into_param().abi(),
                domainserviceid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<PlayReadyContentHeader>(result__)
        })
    }
    pub fn IPlayReadyContentHeaderFactory<R, F: FnOnce(&IPlayReadyContentHeaderFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayReadyContentHeader, IPlayReadyContentHeaderFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPlayReadyContentHeaderFactory2<R, F: FnOnce(&IPlayReadyContentHeaderFactory2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayReadyContentHeader, IPlayReadyContentHeaderFactory2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayReadyContentHeader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyContentHeader;{9a438a6a-7f4c-452e-88bd-0148c6387a2c})");
}
unsafe impl ::windows::runtime::Interface for PlayReadyContentHeader {
    type Vtable = IPlayReadyContentHeader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2588117610, 32588, 17710, [136, 189, 1, 72, 198, 56, 122, 44]);
}
impl ::windows::runtime::RuntimeName for PlayReadyContentHeader {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyContentHeader";
}
impl ::std::convert::From<PlayReadyContentHeader> for ::windows::runtime::IUnknown {
    fn from(value: PlayReadyContentHeader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PlayReadyContentHeader> for ::windows::runtime::IUnknown {
    fn from(value: &PlayReadyContentHeader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlayReadyContentHeader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PlayReadyContentHeader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PlayReadyContentHeader> for ::windows::runtime::IInspectable {
    fn from(value: PlayReadyContentHeader) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PlayReadyContentHeader> for ::windows::runtime::IInspectable {
    fn from(value: &PlayReadyContentHeader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlayReadyContentHeader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlayReadyContentHeader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct PlayReadyContentResolver {}
impl PlayReadyContentResolver {
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ServiceRequest<'a, Param0: ::windows::runtime::IntoParam<'a, PlayReadyContentHeader>>(contentheader: Param0) -> ::windows::runtime::Result<IPlayReadyServiceRequest> {
        Self::IPlayReadyContentResolver(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), contentheader.into_param().abi(), &mut result__).from_abi::<IPlayReadyServiceRequest>(result__)
        })
    }
    pub fn IPlayReadyContentResolver<R, F: FnOnce(&IPlayReadyContentResolver) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayReadyContentResolver, IPlayReadyContentResolver> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for PlayReadyContentResolver {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyContentResolver";
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlayReadyDecryptorSetup(pub i32);
impl PlayReadyDecryptorSetup {
    pub const Uninitialized: PlayReadyDecryptorSetup = PlayReadyDecryptorSetup(0i32);
    pub const OnDemand: PlayReadyDecryptorSetup = PlayReadyDecryptorSetup(1i32);
}
impl ::std::convert::From<i32> for PlayReadyDecryptorSetup {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PlayReadyDecryptorSetup {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PlayReadyDecryptorSetup {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyDecryptorSetup;i4)");
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PlayReadyDomain(::windows::runtime::IInspectable);
impl PlayReadyDomain {
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn AccountId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ServiceId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn Revision(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn FriendlyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn DomainJoinUrl(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayReadyDomain {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomain;{adcc93ac-97e6-43ef-95e4-d7868f3b16a9})");
}
unsafe impl ::windows::runtime::Interface for PlayReadyDomain {
    type Vtable = IPlayReadyDomain_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2915865516, 38886, 17391, [149, 228, 215, 134, 143, 59, 22, 169]);
}
impl ::windows::runtime::RuntimeName for PlayReadyDomain {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomain";
}
impl ::std::convert::From<PlayReadyDomain> for ::windows::runtime::IUnknown {
    fn from(value: PlayReadyDomain) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PlayReadyDomain> for ::windows::runtime::IUnknown {
    fn from(value: &PlayReadyDomain) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlayReadyDomain {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PlayReadyDomain {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PlayReadyDomain> for ::windows::runtime::IInspectable {
    fn from(value: PlayReadyDomain) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PlayReadyDomain> for ::windows::runtime::IInspectable {
    fn from(value: &PlayReadyDomain) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlayReadyDomain {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlayReadyDomain {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<PlayReadyDomain> for IPlayReadyDomain {
    fn from(value: PlayReadyDomain) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PlayReadyDomain> for IPlayReadyDomain {
    fn from(value: &PlayReadyDomain) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyDomain> for PlayReadyDomain {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyDomain> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPlayReadyDomain>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyDomain> for &PlayReadyDomain {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyDomain> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPlayReadyDomain>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PlayReadyDomainIterable(::windows::runtime::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadyDomainIterable {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(domainaccountid: Param0) -> ::windows::runtime::Result<PlayReadyDomainIterable> {
        Self::IPlayReadyDomainIterableFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), domainaccountid.into_param().abi(), &mut result__).from_abi::<PlayReadyDomainIterable>(result__)
        })
    }
    pub fn IPlayReadyDomainIterableFactory<R, F: FnOnce(&IPlayReadyDomainIterableFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayReadyDomainIterable, IPlayReadyDomainIterableFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for PlayReadyDomainIterable {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomainIterable;pinterface({faa585ea-6214-4217-afda-7f46de5869b3};{adcc93ac-97e6-43ef-95e4-d7868f3b16a9}))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for PlayReadyDomainIterable {
    type Vtable = super::super::super::Foundation::Collections::IIterable_abi<IPlayReadyDomain>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::runtime::RuntimeName for PlayReadyDomainIterable {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainIterable";
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PlayReadyDomainIterable> for ::windows::runtime::IUnknown {
    fn from(value: PlayReadyDomainIterable) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PlayReadyDomainIterable> for ::windows::runtime::IUnknown {
    fn from(value: &PlayReadyDomainIterable) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlayReadyDomainIterable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PlayReadyDomainIterable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PlayReadyDomainIterable> for ::windows::runtime::IInspectable {
    fn from(value: PlayReadyDomainIterable) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PlayReadyDomainIterable> for ::windows::runtime::IInspectable {
    fn from(value: &PlayReadyDomainIterable) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlayReadyDomainIterable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlayReadyDomainIterable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PlayReadyDomainIterable> for super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain> {
    fn from(value: PlayReadyDomainIterable) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PlayReadyDomainIterable> for super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain> {
    fn from(value: &PlayReadyDomainIterable) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain>> for PlayReadyDomainIterable {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain>>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain>> for &PlayReadyDomainIterable {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain>>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for PlayReadyDomainIterable {
    type Item = IPlayReadyDomain;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &PlayReadyDomainIterable {
    type Item = IPlayReadyDomain;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PlayReadyDomainIterator(::windows::runtime::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadyDomainIterator {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn Current(&self) -> ::windows::runtime::Result<IPlayReadyDomain> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyDomain>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn HasCurrent(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn MoveNext(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn GetMany(&self, items: &mut [<IPlayReadyDomain as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for PlayReadyDomainIterator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomainIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};{adcc93ac-97e6-43ef-95e4-d7868f3b16a9}))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for PlayReadyDomainIterator {
    type Vtable = super::super::super::Foundation::Collections::IIterator_abi<IPlayReadyDomain>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::runtime::RuntimeName for PlayReadyDomainIterator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainIterator";
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PlayReadyDomainIterator> for ::windows::runtime::IUnknown {
    fn from(value: PlayReadyDomainIterator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PlayReadyDomainIterator> for ::windows::runtime::IUnknown {
    fn from(value: &PlayReadyDomainIterator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlayReadyDomainIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PlayReadyDomainIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PlayReadyDomainIterator> for ::windows::runtime::IInspectable {
    fn from(value: PlayReadyDomainIterator) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PlayReadyDomainIterator> for ::windows::runtime::IInspectable {
    fn from(value: &PlayReadyDomainIterator) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlayReadyDomainIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlayReadyDomainIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PlayReadyDomainIterator> for super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain> {
    fn from(value: PlayReadyDomainIterator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PlayReadyDomainIterator> for super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain> {
    fn from(value: &PlayReadyDomainIterator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain>> for PlayReadyDomainIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain>>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain>> for &PlayReadyDomainIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain>>::into(::std::clone::Clone::clone(self)))
    }
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PlayReadyDomainJoinServiceRequest(::windows::runtime::IInspectable);
impl PlayReadyDomainJoinServiceRequest {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayReadyDomainJoinServiceRequest, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn DomainAccountId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetDomainAccountId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn DomainFriendlyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetDomainFriendlyName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn DomainServiceId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetDomainServiceId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ProtectionSystem(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn SetUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ResponseCustomData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ChallengeCustomData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetChallengeCustomData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn BeginServiceRequest(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn NextServiceRequest(&self) -> ::windows::runtime::Result<IPlayReadyServiceRequest> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::runtime::Result<PlayReadySoapMessage> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), responsebytes.len() as u32, ::std::mem::transmute(responsebytes.as_ptr()), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayReadyDomainJoinServiceRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomainJoinServiceRequest;{171b4a5a-405f-4739-b040-67b9f0c38758})");
}
unsafe impl ::windows::runtime::Interface for PlayReadyDomainJoinServiceRequest {
    type Vtable = IPlayReadyDomainJoinServiceRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(387664474, 16479, 18233, [176, 64, 103, 185, 240, 195, 135, 88]);
}
impl ::windows::runtime::RuntimeName for PlayReadyDomainJoinServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainJoinServiceRequest";
}
impl ::std::convert::From<PlayReadyDomainJoinServiceRequest> for ::windows::runtime::IUnknown {
    fn from(value: PlayReadyDomainJoinServiceRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PlayReadyDomainJoinServiceRequest> for ::windows::runtime::IUnknown {
    fn from(value: &PlayReadyDomainJoinServiceRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PlayReadyDomainJoinServiceRequest> for ::windows::runtime::IInspectable {
    fn from(value: PlayReadyDomainJoinServiceRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PlayReadyDomainJoinServiceRequest> for ::windows::runtime::IInspectable {
    fn from(value: &PlayReadyDomainJoinServiceRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<PlayReadyDomainJoinServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PlayReadyDomainJoinServiceRequest) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PlayReadyDomainJoinServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PlayReadyDomainJoinServiceRequest) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaProtectionServiceRequest> for PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaProtectionServiceRequest> for &PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaProtectionServiceRequest> {
        ::std::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<PlayReadyDomainJoinServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PlayReadyDomainJoinServiceRequest) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PlayReadyDomainJoinServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PlayReadyDomainJoinServiceRequest) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyServiceRequest> for PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyServiceRequest> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyServiceRequest> for &PlayReadyDomainJoinServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyServiceRequest> {
        ::std::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PlayReadyDomainLeaveServiceRequest(::windows::runtime::IInspectable);
impl PlayReadyDomainLeaveServiceRequest {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayReadyDomainLeaveServiceRequest, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn DomainAccountId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetDomainAccountId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn DomainServiceId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetDomainServiceId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ProtectionSystem(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn SetUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ResponseCustomData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ChallengeCustomData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetChallengeCustomData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn BeginServiceRequest(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn NextServiceRequest(&self) -> ::windows::runtime::Result<IPlayReadyServiceRequest> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::runtime::Result<PlayReadySoapMessage> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), responsebytes.len() as u32, ::std::mem::transmute(responsebytes.as_ptr()), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayReadyDomainLeaveServiceRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomainLeaveServiceRequest;{062d58be-97ad-4917-aa03-46d4c252d464})");
}
unsafe impl ::windows::runtime::Interface for PlayReadyDomainLeaveServiceRequest {
    type Vtable = IPlayReadyDomainLeaveServiceRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(103635134, 38829, 18711, [170, 3, 70, 212, 194, 82, 212, 100]);
}
impl ::windows::runtime::RuntimeName for PlayReadyDomainLeaveServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainLeaveServiceRequest";
}
impl ::std::convert::From<PlayReadyDomainLeaveServiceRequest> for ::windows::runtime::IUnknown {
    fn from(value: PlayReadyDomainLeaveServiceRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PlayReadyDomainLeaveServiceRequest> for ::windows::runtime::IUnknown {
    fn from(value: &PlayReadyDomainLeaveServiceRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PlayReadyDomainLeaveServiceRequest> for ::windows::runtime::IInspectable {
    fn from(value: PlayReadyDomainLeaveServiceRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PlayReadyDomainLeaveServiceRequest> for ::windows::runtime::IInspectable {
    fn from(value: &PlayReadyDomainLeaveServiceRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<PlayReadyDomainLeaveServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PlayReadyDomainLeaveServiceRequest) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PlayReadyDomainLeaveServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PlayReadyDomainLeaveServiceRequest) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaProtectionServiceRequest> for PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaProtectionServiceRequest> for &PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaProtectionServiceRequest> {
        ::std::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<PlayReadyDomainLeaveServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PlayReadyDomainLeaveServiceRequest) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PlayReadyDomainLeaveServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PlayReadyDomainLeaveServiceRequest) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyServiceRequest> for PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyServiceRequest> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyServiceRequest> for &PlayReadyDomainLeaveServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyServiceRequest> {
        ::std::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for PlayReadyEncryptionAlgorithm {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PlayReadyEncryptionAlgorithm {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PlayReadyEncryptionAlgorithm {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyEncryptionAlgorithm;i4)");
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlayReadyHardwareDRMFeatures(pub i32);
impl PlayReadyHardwareDRMFeatures {
    pub const HardwareDRM: PlayReadyHardwareDRMFeatures = PlayReadyHardwareDRMFeatures(1i32);
    pub const HEVC: PlayReadyHardwareDRMFeatures = PlayReadyHardwareDRMFeatures(2i32);
    pub const Aes128Cbc: PlayReadyHardwareDRMFeatures = PlayReadyHardwareDRMFeatures(3i32);
}
impl ::std::convert::From<i32> for PlayReadyHardwareDRMFeatures {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PlayReadyHardwareDRMFeatures {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PlayReadyHardwareDRMFeatures {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyHardwareDRMFeatures;i4)");
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlayReadyITADataFormat(pub i32);
impl PlayReadyITADataFormat {
    pub const SerializedProperties: PlayReadyITADataFormat = PlayReadyITADataFormat(0i32);
    pub const SerializedProperties_WithContentProtectionWrapper: PlayReadyITADataFormat = PlayReadyITADataFormat(1i32);
}
impl ::std::convert::From<i32> for PlayReadyITADataFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PlayReadyITADataFormat {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PlayReadyITADataFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyITADataFormat;i4)");
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PlayReadyITADataGenerator(::windows::runtime::IInspectable);
impl PlayReadyITADataGenerator {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayReadyITADataGenerator, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn GenerateData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IPropertySet>>(&self, guidcpsystemid: Param0, countofstreams: u32, configuration: Param2, format: PlayReadyITADataFormat) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), guidcpsystemid.into_param().abi(), countofstreams, configuration.into_param().abi(), format, ::windows::runtime::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayReadyITADataGenerator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyITADataGenerator;{24446b8e-10b9-4530-b25b-901a8029a9b2})");
}
unsafe impl ::windows::runtime::Interface for PlayReadyITADataGenerator {
    type Vtable = IPlayReadyITADataGenerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(608463758, 4281, 17712, [178, 91, 144, 26, 128, 41, 169, 178]);
}
impl ::windows::runtime::RuntimeName for PlayReadyITADataGenerator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyITADataGenerator";
}
impl ::std::convert::From<PlayReadyITADataGenerator> for ::windows::runtime::IUnknown {
    fn from(value: PlayReadyITADataGenerator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PlayReadyITADataGenerator> for ::windows::runtime::IUnknown {
    fn from(value: &PlayReadyITADataGenerator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlayReadyITADataGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PlayReadyITADataGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PlayReadyITADataGenerator> for ::windows::runtime::IInspectable {
    fn from(value: PlayReadyITADataGenerator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PlayReadyITADataGenerator> for ::windows::runtime::IInspectable {
    fn from(value: &PlayReadyITADataGenerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlayReadyITADataGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlayReadyITADataGenerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PlayReadyIndividualizationServiceRequest(::windows::runtime::IInspectable);
impl PlayReadyIndividualizationServiceRequest {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayReadyIndividualizationServiceRequest, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ProtectionSystem(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn SetUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ResponseCustomData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ChallengeCustomData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetChallengeCustomData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn BeginServiceRequest(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn NextServiceRequest(&self) -> ::windows::runtime::Result<IPlayReadyServiceRequest> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::runtime::Result<PlayReadySoapMessage> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), responsebytes.len() as u32, ::std::mem::transmute(responsebytes.as_ptr()), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayReadyIndividualizationServiceRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyIndividualizationServiceRequest;{21f5a86b-008c-4611-ab2f-aaa6c69f0e24})");
}
unsafe impl ::windows::runtime::Interface for PlayReadyIndividualizationServiceRequest {
    type Vtable = IPlayReadyIndividualizationServiceRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(569747563, 140, 17937, [171, 47, 170, 166, 198, 159, 14, 36]);
}
impl ::windows::runtime::RuntimeName for PlayReadyIndividualizationServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyIndividualizationServiceRequest";
}
impl ::std::convert::From<PlayReadyIndividualizationServiceRequest> for ::windows::runtime::IUnknown {
    fn from(value: PlayReadyIndividualizationServiceRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PlayReadyIndividualizationServiceRequest> for ::windows::runtime::IUnknown {
    fn from(value: &PlayReadyIndividualizationServiceRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PlayReadyIndividualizationServiceRequest> for ::windows::runtime::IInspectable {
    fn from(value: PlayReadyIndividualizationServiceRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PlayReadyIndividualizationServiceRequest> for ::windows::runtime::IInspectable {
    fn from(value: &PlayReadyIndividualizationServiceRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<PlayReadyIndividualizationServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PlayReadyIndividualizationServiceRequest) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PlayReadyIndividualizationServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PlayReadyIndividualizationServiceRequest) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaProtectionServiceRequest> for PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaProtectionServiceRequest> for &PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaProtectionServiceRequest> {
        ::std::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<PlayReadyIndividualizationServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PlayReadyIndividualizationServiceRequest) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PlayReadyIndividualizationServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PlayReadyIndividualizationServiceRequest) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyServiceRequest> for PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyServiceRequest> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyServiceRequest> for &PlayReadyIndividualizationServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyServiceRequest> {
        ::std::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PlayReadyLicense(::windows::runtime::IInspectable);
impl PlayReadyLicense {
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn FullyEvaluated(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn UsableForPlay(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn ExpirationDate(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ExpireAfterFirstPlay(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn DomainAccountID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ChainDepth(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn GetKIDAtChainDepth(&self, chaindepth: u32) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), chaindepth, &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SecureStopId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SecurityLevel(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn InMemoryOnly(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ExpiresInRealTime(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayReadyLicense {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicense;{ee474c4e-fa3c-414d-a9f2-3ffc1ef832d4})");
}
unsafe impl ::windows::runtime::Interface for PlayReadyLicense {
    type Vtable = IPlayReadyLicense_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3997649998, 64060, 16717, [169, 242, 63, 252, 30, 248, 50, 212]);
}
impl ::windows::runtime::RuntimeName for PlayReadyLicense {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicense";
}
impl ::std::convert::From<PlayReadyLicense> for ::windows::runtime::IUnknown {
    fn from(value: PlayReadyLicense) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PlayReadyLicense> for ::windows::runtime::IUnknown {
    fn from(value: &PlayReadyLicense) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlayReadyLicense {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PlayReadyLicense {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PlayReadyLicense> for ::windows::runtime::IInspectable {
    fn from(value: PlayReadyLicense) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PlayReadyLicense> for ::windows::runtime::IInspectable {
    fn from(value: &PlayReadyLicense) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlayReadyLicense {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlayReadyLicense {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<PlayReadyLicense> for IPlayReadyLicense {
    fn from(value: PlayReadyLicense) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PlayReadyLicense> for IPlayReadyLicense {
    fn from(value: &PlayReadyLicense) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyLicense> for PlayReadyLicense {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyLicense> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPlayReadyLicense>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyLicense> for &PlayReadyLicense {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyLicense> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPlayReadyLicense>::into(::std::clone::Clone::clone(self)))
    }
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PlayReadyLicenseAcquisitionServiceRequest(::windows::runtime::IInspectable);
impl PlayReadyLicenseAcquisitionServiceRequest {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayReadyLicenseAcquisitionServiceRequest, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ContentHeader(&self) -> ::windows::runtime::Result<PlayReadyContentHeader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadyContentHeader>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetContentHeader<'a, Param0: ::windows::runtime::IntoParam<'a, PlayReadyContentHeader>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn DomainServiceId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetDomainServiceId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ProtectionSystem(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn SetUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ResponseCustomData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ChallengeCustomData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetChallengeCustomData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn BeginServiceRequest(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn NextServiceRequest(&self) -> ::windows::runtime::Result<IPlayReadyServiceRequest> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::runtime::Result<PlayReadySoapMessage> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), responsebytes.len() as u32, ::std::mem::transmute(responsebytes.as_ptr()), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SessionId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyLicenseAcquisitionServiceRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn CreateLicenseIterable<'a, Param0: ::windows::runtime::IntoParam<'a, PlayReadyContentHeader>>(&self, contentheader: Param0, fullyevaluated: bool) -> ::windows::runtime::Result<PlayReadyLicenseIterable> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyLicenseAcquisitionServiceRequest3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), contentheader.into_param().abi(), fullyevaluated, &mut result__).from_abi::<PlayReadyLicenseIterable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayReadyLicenseAcquisitionServiceRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicenseAcquisitionServiceRequest;{5d85ff45-3e9f-4f48-93e1-9530c8d58c3e})");
}
unsafe impl ::windows::runtime::Interface for PlayReadyLicenseAcquisitionServiceRequest {
    type Vtable = IPlayReadyLicenseAcquisitionServiceRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1569062725, 16031, 20296, [147, 225, 149, 48, 200, 213, 140, 62]);
}
impl ::windows::runtime::RuntimeName for PlayReadyLicenseAcquisitionServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseAcquisitionServiceRequest";
}
impl ::std::convert::From<PlayReadyLicenseAcquisitionServiceRequest> for ::windows::runtime::IUnknown {
    fn from(value: PlayReadyLicenseAcquisitionServiceRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PlayReadyLicenseAcquisitionServiceRequest> for ::windows::runtime::IUnknown {
    fn from(value: &PlayReadyLicenseAcquisitionServiceRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PlayReadyLicenseAcquisitionServiceRequest> for ::windows::runtime::IInspectable {
    fn from(value: PlayReadyLicenseAcquisitionServiceRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PlayReadyLicenseAcquisitionServiceRequest> for ::windows::runtime::IInspectable {
    fn from(value: &PlayReadyLicenseAcquisitionServiceRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<PlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyLicenseAcquisitionServiceRequest {
    fn from(value: PlayReadyLicenseAcquisitionServiceRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyLicenseAcquisitionServiceRequest {
    fn from(value: &PlayReadyLicenseAcquisitionServiceRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyLicenseAcquisitionServiceRequest> for PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyLicenseAcquisitionServiceRequest> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPlayReadyLicenseAcquisitionServiceRequest>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyLicenseAcquisitionServiceRequest> for &PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyLicenseAcquisitionServiceRequest> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPlayReadyLicenseAcquisitionServiceRequest>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::TryFrom<PlayReadyLicenseAcquisitionServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PlayReadyLicenseAcquisitionServiceRequest) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PlayReadyLicenseAcquisitionServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PlayReadyLicenseAcquisitionServiceRequest) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaProtectionServiceRequest> for PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaProtectionServiceRequest> for &PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaProtectionServiceRequest> {
        ::std::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<PlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PlayReadyLicenseAcquisitionServiceRequest) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PlayReadyLicenseAcquisitionServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PlayReadyLicenseAcquisitionServiceRequest) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyServiceRequest> for PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyServiceRequest> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyServiceRequest> for &PlayReadyLicenseAcquisitionServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyServiceRequest> {
        ::std::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PlayReadyLicenseIterable(::windows::runtime::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadyLicenseIterable {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayReadyLicenseIterable, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, PlayReadyContentHeader>>(contentheader: Param0, fullyevaluated: bool) -> ::windows::runtime::Result<PlayReadyLicenseIterable> {
        Self::IPlayReadyLicenseIterableFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), contentheader.into_param().abi(), fullyevaluated, &mut result__).from_abi::<PlayReadyLicenseIterable>(result__)
        })
    }
    pub fn IPlayReadyLicenseIterableFactory<R, F: FnOnce(&IPlayReadyLicenseIterableFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayReadyLicenseIterable, IPlayReadyLicenseIterableFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for PlayReadyLicenseIterable {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicenseIterable;pinterface({faa585ea-6214-4217-afda-7f46de5869b3};{ee474c4e-fa3c-414d-a9f2-3ffc1ef832d4}))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for PlayReadyLicenseIterable {
    type Vtable = super::super::super::Foundation::Collections::IIterable_abi<IPlayReadyLicense>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::runtime::RuntimeName for PlayReadyLicenseIterable {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseIterable";
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PlayReadyLicenseIterable> for ::windows::runtime::IUnknown {
    fn from(value: PlayReadyLicenseIterable) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PlayReadyLicenseIterable> for ::windows::runtime::IUnknown {
    fn from(value: &PlayReadyLicenseIterable) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlayReadyLicenseIterable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PlayReadyLicenseIterable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PlayReadyLicenseIterable> for ::windows::runtime::IInspectable {
    fn from(value: PlayReadyLicenseIterable) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PlayReadyLicenseIterable> for ::windows::runtime::IInspectable {
    fn from(value: &PlayReadyLicenseIterable) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlayReadyLicenseIterable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlayReadyLicenseIterable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PlayReadyLicenseIterable> for super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense> {
    fn from(value: PlayReadyLicenseIterable) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PlayReadyLicenseIterable> for super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense> {
    fn from(value: &PlayReadyLicenseIterable) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense>> for PlayReadyLicenseIterable {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense>>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense>> for &PlayReadyLicenseIterable {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense>>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for PlayReadyLicenseIterable {
    type Item = IPlayReadyLicense;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &PlayReadyLicenseIterable {
    type Item = IPlayReadyLicense;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PlayReadyLicenseIterator(::windows::runtime::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadyLicenseIterator {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn Current(&self) -> ::windows::runtime::Result<IPlayReadyLicense> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyLicense>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn HasCurrent(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn MoveNext(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn GetMany(&self, items: &mut [<IPlayReadyLicense as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for PlayReadyLicenseIterator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicenseIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};{ee474c4e-fa3c-414d-a9f2-3ffc1ef832d4}))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for PlayReadyLicenseIterator {
    type Vtable = super::super::super::Foundation::Collections::IIterator_abi<IPlayReadyLicense>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::runtime::RuntimeName for PlayReadyLicenseIterator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseIterator";
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PlayReadyLicenseIterator> for ::windows::runtime::IUnknown {
    fn from(value: PlayReadyLicenseIterator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PlayReadyLicenseIterator> for ::windows::runtime::IUnknown {
    fn from(value: &PlayReadyLicenseIterator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlayReadyLicenseIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PlayReadyLicenseIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PlayReadyLicenseIterator> for ::windows::runtime::IInspectable {
    fn from(value: PlayReadyLicenseIterator) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PlayReadyLicenseIterator> for ::windows::runtime::IInspectable {
    fn from(value: &PlayReadyLicenseIterator) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlayReadyLicenseIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlayReadyLicenseIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PlayReadyLicenseIterator> for super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense> {
    fn from(value: PlayReadyLicenseIterator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PlayReadyLicenseIterator> for super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense> {
    fn from(value: &PlayReadyLicenseIterator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense>> for PlayReadyLicenseIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense>>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense>> for &PlayReadyLicenseIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense>>::into(::std::clone::Clone::clone(self)))
    }
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct PlayReadyLicenseManagement {}
impl PlayReadyLicenseManagement {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn DeleteLicenses<'a, Param0: ::windows::runtime::IntoParam<'a, PlayReadyContentHeader>>(contentheader: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IPlayReadyLicenseManagement(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), contentheader.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn IPlayReadyLicenseManagement<R, F: FnOnce(&IPlayReadyLicenseManagement) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayReadyLicenseManagement, IPlayReadyLicenseManagement> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for PlayReadyLicenseManagement {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseManagement";
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PlayReadyLicenseSession(::windows::runtime::IInspectable);
impl PlayReadyLicenseSession {
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn CreateLAServiceRequest(&self) -> ::windows::runtime::Result<IPlayReadyLicenseAcquisitionServiceRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyLicenseAcquisitionServiceRequest>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ConfigureMediaProtectionManager<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaProtectionManager>>(&self, mpm: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), mpm.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IPropertySet>>(configuration: Param0) -> ::windows::runtime::Result<PlayReadyLicenseSession> {
        Self::IPlayReadyLicenseSessionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), configuration.into_param().abi(), &mut result__).from_abi::<PlayReadyLicenseSession>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn CreateLicenseIterable<'a, Param0: ::windows::runtime::IntoParam<'a, PlayReadyContentHeader>>(&self, contentheader: Param0, fullyevaluated: bool) -> ::windows::runtime::Result<PlayReadyLicenseIterable> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyLicenseSession2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), contentheader.into_param().abi(), fullyevaluated, &mut result__).from_abi::<PlayReadyLicenseIterable>(result__)
        }
    }
    pub fn IPlayReadyLicenseSessionFactory<R, F: FnOnce(&IPlayReadyLicenseSessionFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayReadyLicenseSession, IPlayReadyLicenseSessionFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayReadyLicenseSession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicenseSession;{a1723a39-87fa-4fdd-abbb-a9720e845259})");
}
unsafe impl ::windows::runtime::Interface for PlayReadyLicenseSession {
    type Vtable = IPlayReadyLicenseSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2708617785, 34810, 20445, [171, 187, 169, 114, 14, 132, 82, 89]);
}
impl ::windows::runtime::RuntimeName for PlayReadyLicenseSession {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseSession";
}
impl ::std::convert::From<PlayReadyLicenseSession> for ::windows::runtime::IUnknown {
    fn from(value: PlayReadyLicenseSession) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PlayReadyLicenseSession> for ::windows::runtime::IUnknown {
    fn from(value: &PlayReadyLicenseSession) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlayReadyLicenseSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PlayReadyLicenseSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PlayReadyLicenseSession> for ::windows::runtime::IInspectable {
    fn from(value: PlayReadyLicenseSession) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PlayReadyLicenseSession> for ::windows::runtime::IInspectable {
    fn from(value: &PlayReadyLicenseSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlayReadyLicenseSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlayReadyLicenseSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<PlayReadyLicenseSession> for IPlayReadyLicenseSession {
    fn from(value: PlayReadyLicenseSession) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PlayReadyLicenseSession> for IPlayReadyLicenseSession {
    fn from(value: &PlayReadyLicenseSession) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyLicenseSession> for PlayReadyLicenseSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyLicenseSession> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPlayReadyLicenseSession>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyLicenseSession> for &PlayReadyLicenseSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyLicenseSession> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPlayReadyLicenseSession>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::TryFrom<PlayReadyLicenseSession> for IPlayReadyLicenseSession2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PlayReadyLicenseSession) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PlayReadyLicenseSession> for IPlayReadyLicenseSession2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PlayReadyLicenseSession) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyLicenseSession2> for PlayReadyLicenseSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyLicenseSession2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyLicenseSession2> for &PlayReadyLicenseSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyLicenseSession2> {
        ::std::convert::TryInto::<IPlayReadyLicenseSession2>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PlayReadyMeteringReportServiceRequest(::windows::runtime::IInspectable);
impl PlayReadyMeteringReportServiceRequest {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayReadyMeteringReportServiceRequest, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn MeteringCertificate(&self) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetMeteringCertificate(&self, meteringcertbytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), meteringcertbytes.len() as u32, ::std::mem::transmute(meteringcertbytes.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ProtectionSystem(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn SetUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ResponseCustomData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ChallengeCustomData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetChallengeCustomData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn BeginServiceRequest(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn NextServiceRequest(&self) -> ::windows::runtime::Result<IPlayReadyServiceRequest> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::runtime::Result<PlayReadySoapMessage> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), responsebytes.len() as u32, ::std::mem::transmute(responsebytes.as_ptr()), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayReadyMeteringReportServiceRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyMeteringReportServiceRequest;{c12b231c-0ecd-4f11-a185-1e24a4a67fb7})");
}
unsafe impl ::windows::runtime::Interface for PlayReadyMeteringReportServiceRequest {
    type Vtable = IPlayReadyMeteringReportServiceRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3240829724, 3789, 20241, [161, 133, 30, 36, 164, 166, 127, 183]);
}
impl ::windows::runtime::RuntimeName for PlayReadyMeteringReportServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyMeteringReportServiceRequest";
}
impl ::std::convert::From<PlayReadyMeteringReportServiceRequest> for ::windows::runtime::IUnknown {
    fn from(value: PlayReadyMeteringReportServiceRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PlayReadyMeteringReportServiceRequest> for ::windows::runtime::IUnknown {
    fn from(value: &PlayReadyMeteringReportServiceRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PlayReadyMeteringReportServiceRequest> for ::windows::runtime::IInspectable {
    fn from(value: PlayReadyMeteringReportServiceRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PlayReadyMeteringReportServiceRequest> for ::windows::runtime::IInspectable {
    fn from(value: &PlayReadyMeteringReportServiceRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<PlayReadyMeteringReportServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PlayReadyMeteringReportServiceRequest) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PlayReadyMeteringReportServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PlayReadyMeteringReportServiceRequest) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaProtectionServiceRequest> for PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaProtectionServiceRequest> for &PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaProtectionServiceRequest> {
        ::std::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<PlayReadyMeteringReportServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PlayReadyMeteringReportServiceRequest) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PlayReadyMeteringReportServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PlayReadyMeteringReportServiceRequest) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyServiceRequest> for PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyServiceRequest> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyServiceRequest> for &PlayReadyMeteringReportServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyServiceRequest> {
        ::std::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PlayReadyRevocationServiceRequest(::windows::runtime::IInspectable);
impl PlayReadyRevocationServiceRequest {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayReadyRevocationServiceRequest, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ProtectionSystem(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn SetUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ResponseCustomData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ChallengeCustomData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetChallengeCustomData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn BeginServiceRequest(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn NextServiceRequest(&self) -> ::windows::runtime::Result<IPlayReadyServiceRequest> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::runtime::Result<PlayReadySoapMessage> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), responsebytes.len() as u32, ::std::mem::transmute(responsebytes.as_ptr()), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayReadyRevocationServiceRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyRevocationServiceRequest;{543d66ac-faf0-4560-84a5-0e4acec939e4})");
}
unsafe impl ::windows::runtime::Interface for PlayReadyRevocationServiceRequest {
    type Vtable = IPlayReadyRevocationServiceRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1413310124, 64240, 17760, [132, 165, 14, 74, 206, 201, 57, 228]);
}
impl ::windows::runtime::RuntimeName for PlayReadyRevocationServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyRevocationServiceRequest";
}
impl ::std::convert::From<PlayReadyRevocationServiceRequest> for ::windows::runtime::IUnknown {
    fn from(value: PlayReadyRevocationServiceRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PlayReadyRevocationServiceRequest> for ::windows::runtime::IUnknown {
    fn from(value: &PlayReadyRevocationServiceRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PlayReadyRevocationServiceRequest> for ::windows::runtime::IInspectable {
    fn from(value: PlayReadyRevocationServiceRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PlayReadyRevocationServiceRequest> for ::windows::runtime::IInspectable {
    fn from(value: &PlayReadyRevocationServiceRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<PlayReadyRevocationServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PlayReadyRevocationServiceRequest) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PlayReadyRevocationServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PlayReadyRevocationServiceRequest) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaProtectionServiceRequest> for PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaProtectionServiceRequest> for &PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaProtectionServiceRequest> {
        ::std::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<PlayReadyRevocationServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PlayReadyRevocationServiceRequest) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PlayReadyRevocationServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PlayReadyRevocationServiceRequest) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyServiceRequest> for PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyServiceRequest> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyServiceRequest> for &PlayReadyRevocationServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyServiceRequest> {
        ::std::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PlayReadySecureStopIterable(::windows::runtime::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadySecureStopIterable {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn CreateInstance(publishercertbytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<PlayReadySecureStopIterable> {
        Self::IPlayReadySecureStopIterableFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), publishercertbytes.len() as u32, ::std::mem::transmute(publishercertbytes.as_ptr()), &mut result__).from_abi::<PlayReadySecureStopIterable>(result__)
        })
    }
    pub fn IPlayReadySecureStopIterableFactory<R, F: FnOnce(&IPlayReadySecureStopIterableFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayReadySecureStopIterable, IPlayReadySecureStopIterableFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for PlayReadySecureStopIterable {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadySecureStopIterable;pinterface({faa585ea-6214-4217-afda-7f46de5869b3};{b5501ee5-01bf-4401-9677-05630a6a4cc8}))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for PlayReadySecureStopIterable {
    type Vtable = super::super::super::Foundation::Collections::IIterable_abi<IPlayReadySecureStopServiceRequest>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::runtime::RuntimeName for PlayReadySecureStopIterable {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySecureStopIterable";
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PlayReadySecureStopIterable> for ::windows::runtime::IUnknown {
    fn from(value: PlayReadySecureStopIterable) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PlayReadySecureStopIterable> for ::windows::runtime::IUnknown {
    fn from(value: &PlayReadySecureStopIterable) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlayReadySecureStopIterable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PlayReadySecureStopIterable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PlayReadySecureStopIterable> for ::windows::runtime::IInspectable {
    fn from(value: PlayReadySecureStopIterable) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PlayReadySecureStopIterable> for ::windows::runtime::IInspectable {
    fn from(value: &PlayReadySecureStopIterable) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlayReadySecureStopIterable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlayReadySecureStopIterable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PlayReadySecureStopIterable> for super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest> {
    fn from(value: PlayReadySecureStopIterable) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PlayReadySecureStopIterable> for super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest> {
    fn from(value: &PlayReadySecureStopIterable) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest>> for PlayReadySecureStopIterable {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest>>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest>> for &PlayReadySecureStopIterable {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest>>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for PlayReadySecureStopIterable {
    type Item = IPlayReadySecureStopServiceRequest;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &PlayReadySecureStopIterable {
    type Item = IPlayReadySecureStopServiceRequest;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PlayReadySecureStopIterator(::windows::runtime::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadySecureStopIterator {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn Current(&self) -> ::windows::runtime::Result<IPlayReadySecureStopServiceRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadySecureStopServiceRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn HasCurrent(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn MoveNext(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn GetMany(&self, items: &mut [<IPlayReadySecureStopServiceRequest as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for PlayReadySecureStopIterator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadySecureStopIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};{b5501ee5-01bf-4401-9677-05630a6a4cc8}))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for PlayReadySecureStopIterator {
    type Vtable = super::super::super::Foundation::Collections::IIterator_abi<IPlayReadySecureStopServiceRequest>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::runtime::RuntimeName for PlayReadySecureStopIterator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySecureStopIterator";
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PlayReadySecureStopIterator> for ::windows::runtime::IUnknown {
    fn from(value: PlayReadySecureStopIterator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PlayReadySecureStopIterator> for ::windows::runtime::IUnknown {
    fn from(value: &PlayReadySecureStopIterator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlayReadySecureStopIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PlayReadySecureStopIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PlayReadySecureStopIterator> for ::windows::runtime::IInspectable {
    fn from(value: PlayReadySecureStopIterator) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PlayReadySecureStopIterator> for ::windows::runtime::IInspectable {
    fn from(value: &PlayReadySecureStopIterator) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlayReadySecureStopIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlayReadySecureStopIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PlayReadySecureStopIterator> for super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest> {
    fn from(value: PlayReadySecureStopIterator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PlayReadySecureStopIterator> for super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest> {
    fn from(value: &PlayReadySecureStopIterator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>> for PlayReadySecureStopIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>> for &PlayReadySecureStopIterator {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>>::into(::std::clone::Clone::clone(self)))
    }
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PlayReadySecureStopServiceRequest(::windows::runtime::IInspectable);
impl PlayReadySecureStopServiceRequest {
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SessionID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn StartTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn UpdateTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn Stopped(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn PublisherCertificate(&self) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ProtectionSystem(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn SetUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ResponseCustomData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ChallengeCustomData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SetChallengeCustomData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn BeginServiceRequest(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn NextServiceRequest(&self) -> ::windows::runtime::Result<IPlayReadyServiceRequest> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IPlayReadyServiceRequest>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows::runtime::Result<PlayReadySoapMessage> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlayReadySoapMessage>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), responsebytes.len() as u32, ::std::mem::transmute(responsebytes.as_ptr()), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn CreateInstance(publishercertbytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<PlayReadySecureStopServiceRequest> {
        Self::IPlayReadySecureStopServiceRequestFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), publishercertbytes.len() as u32, ::std::mem::transmute(publishercertbytes.as_ptr()), &mut result__).from_abi::<PlayReadySecureStopServiceRequest>(result__)
        })
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn CreateInstanceFromSessionID<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(sessionid: Param0, publishercertbytes: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<PlayReadySecureStopServiceRequest> {
        Self::IPlayReadySecureStopServiceRequestFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), sessionid.into_param().abi(), publishercertbytes.len() as u32, ::std::mem::transmute(publishercertbytes.as_ptr()), &mut result__).from_abi::<PlayReadySecureStopServiceRequest>(result__)
        })
    }
    pub fn IPlayReadySecureStopServiceRequestFactory<R, F: FnOnce(&IPlayReadySecureStopServiceRequestFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayReadySecureStopServiceRequest, IPlayReadySecureStopServiceRequestFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayReadySecureStopServiceRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadySecureStopServiceRequest;{b5501ee5-01bf-4401-9677-05630a6a4cc8})");
}
unsafe impl ::windows::runtime::Interface for PlayReadySecureStopServiceRequest {
    type Vtable = IPlayReadySecureStopServiceRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3041926885, 447, 17409, [150, 119, 5, 99, 10, 106, 76, 200]);
}
impl ::windows::runtime::RuntimeName for PlayReadySecureStopServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySecureStopServiceRequest";
}
impl ::std::convert::From<PlayReadySecureStopServiceRequest> for ::windows::runtime::IUnknown {
    fn from(value: PlayReadySecureStopServiceRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PlayReadySecureStopServiceRequest> for ::windows::runtime::IUnknown {
    fn from(value: &PlayReadySecureStopServiceRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PlayReadySecureStopServiceRequest> for ::windows::runtime::IInspectable {
    fn from(value: PlayReadySecureStopServiceRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PlayReadySecureStopServiceRequest> for ::windows::runtime::IInspectable {
    fn from(value: &PlayReadySecureStopServiceRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<PlayReadySecureStopServiceRequest> for IPlayReadySecureStopServiceRequest {
    fn from(value: PlayReadySecureStopServiceRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PlayReadySecureStopServiceRequest> for IPlayReadySecureStopServiceRequest {
    fn from(value: &PlayReadySecureStopServiceRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadySecureStopServiceRequest> for PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadySecureStopServiceRequest> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPlayReadySecureStopServiceRequest>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadySecureStopServiceRequest> for &PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadySecureStopServiceRequest> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPlayReadySecureStopServiceRequest>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::TryFrom<PlayReadySecureStopServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PlayReadySecureStopServiceRequest) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PlayReadySecureStopServiceRequest> for super::IMediaProtectionServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PlayReadySecureStopServiceRequest) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaProtectionServiceRequest> for PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaProtectionServiceRequest> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IMediaProtectionServiceRequest> for &PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IMediaProtectionServiceRequest> {
        ::std::convert::TryInto::<super::IMediaProtectionServiceRequest>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<PlayReadySecureStopServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PlayReadySecureStopServiceRequest) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&PlayReadySecureStopServiceRequest> for IPlayReadyServiceRequest {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PlayReadySecureStopServiceRequest) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyServiceRequest> for PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyServiceRequest> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPlayReadyServiceRequest> for &PlayReadySecureStopServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPlayReadyServiceRequest> {
        ::std::convert::TryInto::<IPlayReadyServiceRequest>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PlayReadySoapMessage(::windows::runtime::IInspectable);
impl PlayReadySoapMessage {
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn GetMessageBody(&self) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation_Collections`*"]
    pub fn MessageHeaders(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PlayReadySoapMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadySoapMessage;{b659fcb5-ce41-41ba-8a0d-61df5fffa139})");
}
unsafe impl ::windows::runtime::Interface for PlayReadySoapMessage {
    type Vtable = IPlayReadySoapMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3059350709, 52801, 16826, [138, 13, 97, 223, 95, 255, 161, 57]);
}
impl ::windows::runtime::RuntimeName for PlayReadySoapMessage {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySoapMessage";
}
impl ::std::convert::From<PlayReadySoapMessage> for ::windows::runtime::IUnknown {
    fn from(value: PlayReadySoapMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PlayReadySoapMessage> for ::windows::runtime::IUnknown {
    fn from(value: &PlayReadySoapMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PlayReadySoapMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PlayReadySoapMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PlayReadySoapMessage> for ::windows::runtime::IInspectable {
    fn from(value: PlayReadySoapMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PlayReadySoapMessage> for ::windows::runtime::IInspectable {
    fn from(value: &PlayReadySoapMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PlayReadySoapMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PlayReadySoapMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Media_Protection_PlayReady`*"]
pub struct PlayReadyStatics {}
impl PlayReadyStatics {
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn DomainJoinServiceRequestType() -> ::windows::runtime::Result<::windows::runtime::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn DomainLeaveServiceRequestType() -> ::windows::runtime::Result<::windows::runtime::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn IndividualizationServiceRequestType() -> ::windows::runtime::Result<::windows::runtime::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn LicenseAcquirerServiceRequestType() -> ::windows::runtime::Result<::windows::runtime::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn MeteringReportServiceRequestType() -> ::windows::runtime::Result<::windows::runtime::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn RevocationServiceRequestType() -> ::windows::runtime::Result<::windows::runtime::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn MediaProtectionSystemId() -> ::windows::runtime::Result<::windows::runtime::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn PlayReadySecurityVersion() -> ::windows::runtime::Result<u32> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn PlayReadyCertificateSecurityLevel() -> ::windows::runtime::Result<u32> {
        Self::IPlayReadyStatics2(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn SecureStopServiceRequestType() -> ::windows::runtime::Result<::windows::runtime::GUID> {
        Self::IPlayReadyStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn CheckSupportedHardware(hwdrmfeature: PlayReadyHardwareDRMFeatures) -> ::windows::runtime::Result<bool> {
        Self::IPlayReadyStatics3(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), hwdrmfeature, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn InputTrustAuthorityToCreate() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IPlayReadyStatics4(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ProtectionSystemId() -> ::windows::runtime::Result<::windows::runtime::GUID> {
        Self::IPlayReadyStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn HardwareDRMDisabledAtTime() -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        Self::IPlayReadyStatics5(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Protection_PlayReady`, `Foundation`*"]
    pub fn HardwareDRMDisabledUntilTime() -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        Self::IPlayReadyStatics5(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        })
    }
    #[doc = "*Required features: `Media_Protection_PlayReady`*"]
    pub fn ResetHardwareDRMDisabled() -> ::windows::runtime::Result<()> {
        Self::IPlayReadyStatics5(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() })
    }
    pub fn IPlayReadyStatics<R, F: FnOnce(&IPlayReadyStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayReadyStatics, IPlayReadyStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPlayReadyStatics2<R, F: FnOnce(&IPlayReadyStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayReadyStatics, IPlayReadyStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPlayReadyStatics3<R, F: FnOnce(&IPlayReadyStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayReadyStatics, IPlayReadyStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPlayReadyStatics4<R, F: FnOnce(&IPlayReadyStatics4) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayReadyStatics, IPlayReadyStatics4> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPlayReadyStatics5<R, F: FnOnce(&IPlayReadyStatics5) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlayReadyStatics, IPlayReadyStatics5> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for PlayReadyStatics {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyStatics";
}
