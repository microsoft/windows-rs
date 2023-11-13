#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INDClient(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for INDClient {
    type Vtable = INDClient_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for INDClient {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3bd6781b_61b8_46e2_99a5_8abcb6b9f7d6);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDClient_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RegistrationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RegistrationCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveRegistrationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveRegistrationCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ProximityDetectionCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ProximityDetectionCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveProximityDetectionCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveProximityDetectionCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub LicenseFetchCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    LicenseFetchCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveLicenseFetchCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveLicenseFetchCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ReRegistrationNeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ReRegistrationNeeded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveReRegistrationNeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveReRegistrationNeeded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ClosedCaptionDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ClosedCaptionDataReceived: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveClosedCaptionDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveClosedCaptionDataReceived: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenturl: *mut ::core::ffi::c_void, startasyncoptions: u32, registrationcustomdata: *mut ::core::ffi::c_void, licensefetchdescriptor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    StartAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub LicenseFetchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, licensefetchdescriptor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    LicenseFetchAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ReRegistrationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, registrationcustomdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ReRegistrationAsync: usize,
    #[cfg(feature = "deprecated")]
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Close: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INDClientFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for INDClientFactory {
    type Vtable = INDClientFactory_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for INDClientFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e53dd62_fee8_451f_b0d4_f706cca3e037);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDClientFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, downloadengine: *mut ::core::ffi::c_void, streamparser: *mut ::core::ffi::c_void, pmessenger: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateInstance: usize,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INDClosedCaptionDataReceivedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDClosedCaptionDataReceivedEventArgs {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn ClosedCaptionDataFormat(&self) -> ::windows_core::Result<NDClosedCaptionFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClosedCaptionDataFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn PresentationTimestamp(&self) -> ::windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PresentationTimestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn ClosedCaptionData(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).ClosedCaptionData)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(INDClosedCaptionDataReceivedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for INDClosedCaptionDataReceivedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{4738d29f-c345-4649-8468-b8c5fc357190}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for INDClosedCaptionDataReceivedEventArgs {
    type Vtable = INDClosedCaptionDataReceivedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for INDClosedCaptionDataReceivedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4738d29f_c345_4649_8468_b8c5fc357190);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDClosedCaptionDataReceivedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ClosedCaptionDataFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NDClosedCaptionFormat) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ClosedCaptionDataFormat: usize,
    #[cfg(feature = "deprecated")]
    pub PresentationTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PresentationTimestamp: usize,
    #[cfg(feature = "deprecated")]
    pub ClosedCaptionData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ClosedCaptionData: usize,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INDCustomData(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDCustomData {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn CustomDataTypeID(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).CustomDataTypeID)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn CustomData(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).CustomData)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(INDCustomData, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for INDCustomData {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{f5cb0fdc-2d09-4f19-b5e1-76a0b3ee9267}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for INDCustomData {
    type Vtable = INDCustomData_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for INDCustomData {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5cb0fdc_2d09_4f19_b5e1_76a0b3ee9267);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDCustomData_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CustomDataTypeID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CustomDataTypeID: usize,
    #[cfg(feature = "deprecated")]
    pub CustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CustomData: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INDCustomDataFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for INDCustomDataFactory {
    type Vtable = INDCustomDataFactory_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for INDCustomDataFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd65405ab_3424_4833_8c9a_af5fdeb22872);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDCustomDataFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customDataTypeIDBytes_array_size: u32, customdatatypeidbytes: *const u8, customDataBytes_array_size: u32, customdatabytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateInstance: usize,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INDDownloadEngine(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDDownloadEngine {
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Open<P0>(&self, uri: P0, sessionidbytes: &[u8]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Open)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr()).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn Pause(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Pause)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn Resume(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Resume)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Seek(&self, startposition: super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Seek)(::windows_core::Interface::as_raw(this), startposition).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn CanSeek(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanSeek)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn BufferFullMinThresholdInSamples(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BufferFullMinThresholdInSamples)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn BufferFullMaxThresholdInSamples(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BufferFullMaxThresholdInSamples)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn Notifier(&self) -> ::windows_core::Result<NDDownloadEngineNotifier> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Notifier)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(INDDownloadEngine, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for INDDownloadEngine {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{2d223d65-c4b6-4438-8d46-b96e6d0fb21f}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for INDDownloadEngine {
    type Vtable = INDDownloadEngine_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for INDDownloadEngine {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2d223d65_c4b6_4438_8d46_b96e6d0fb21f);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDDownloadEngine_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, sessionIDBytes_array_size: u32, sessionidbytes: *const u8) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Open: usize,
    #[cfg(feature = "deprecated")]
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Pause: usize,
    #[cfg(feature = "deprecated")]
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Resume: usize,
    #[cfg(feature = "deprecated")]
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Close: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startposition: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Seek: usize,
    #[cfg(feature = "deprecated")]
    pub CanSeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CanSeek: usize,
    #[cfg(feature = "deprecated")]
    pub BufferFullMinThresholdInSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BufferFullMinThresholdInSamples: usize,
    #[cfg(feature = "deprecated")]
    pub BufferFullMaxThresholdInSamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BufferFullMaxThresholdInSamples: usize,
    #[cfg(feature = "deprecated")]
    pub Notifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Notifier: usize,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INDDownloadEngineNotifier(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDDownloadEngineNotifier {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn OnStreamOpened(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnStreamOpened)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn OnPlayReadyObjectReceived(&self, databytes: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnPlayReadyObjectReceived)(::windows_core::Interface::as_raw(this), databytes.len().try_into().unwrap(), databytes.as_ptr()).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn OnContentIDReceived<P0>(&self, licensefetchdescriptor: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<INDLicenseFetchDescriptor>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnContentIDReceived)(::windows_core::Interface::as_raw(this), licensefetchdescriptor.try_into_param()?.abi()).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn OnDataReceived(&self, databytes: &[u8], bytesreceived: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnDataReceived)(::windows_core::Interface::as_raw(this), databytes.len().try_into().unwrap(), databytes.as_ptr(), bytesreceived).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn OnEndOfStream(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnEndOfStream)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn OnNetworkError(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnNetworkError)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(INDDownloadEngineNotifier, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for INDDownloadEngineNotifier {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{d720b4d4-f4b8-4530-a809-9193a571e7fc}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for INDDownloadEngineNotifier {
    type Vtable = INDDownloadEngineNotifier_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for INDDownloadEngineNotifier {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd720b4d4_f4b8_4530_a809_9193a571e7fc);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDDownloadEngineNotifier_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub OnStreamOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnStreamOpened: usize,
    #[cfg(feature = "deprecated")]
    pub OnPlayReadyObjectReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnPlayReadyObjectReceived: usize,
    #[cfg(feature = "deprecated")]
    pub OnContentIDReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, licensefetchdescriptor: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnContentIDReceived: usize,
    #[cfg(feature = "deprecated")]
    pub OnDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8, bytesreceived: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnDataReceived: usize,
    #[cfg(feature = "deprecated")]
    pub OnEndOfStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnEndOfStream: usize,
    #[cfg(feature = "deprecated")]
    pub OnNetworkError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnNetworkError: usize,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INDLicenseFetchCompletedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDLicenseFetchCompletedEventArgs {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(INDLicenseFetchCompletedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for INDLicenseFetchCompletedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{1ee30a1a-11b2-4558-8865-e3a516922517}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for INDLicenseFetchCompletedEventArgs {
    type Vtable = INDLicenseFetchCompletedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for INDLicenseFetchCompletedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1ee30a1a_11b2_4558_8865_e3a516922517);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ResponseCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ResponseCustomData: usize,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INDLicenseFetchDescriptor(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDLicenseFetchDescriptor {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn ContentIDType(&self) -> ::windows_core::Result<NDContentIDType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentIDType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn ContentID(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).ContentID)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn LicenseFetchChallengeCustomData(&self) -> ::windows_core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LicenseFetchChallengeCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn SetLicenseFetchChallengeCustomData<P0>(&self, licensefetchchallengecustomdata: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<INDCustomData>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLicenseFetchChallengeCustomData)(::windows_core::Interface::as_raw(this), licensefetchchallengecustomdata.try_into_param()?.abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(INDLicenseFetchDescriptor, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for INDLicenseFetchDescriptor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{5498d33a-e686-4935-a567-7ca77ad20fa4}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for INDLicenseFetchDescriptor {
    type Vtable = INDLicenseFetchDescriptor_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for INDLicenseFetchDescriptor {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5498d33a_e686_4935_a567_7ca77ad20fa4);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchDescriptor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ContentIDType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NDContentIDType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ContentIDType: usize,
    #[cfg(feature = "deprecated")]
    pub ContentID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ContentID: usize,
    #[cfg(feature = "deprecated")]
    pub LicenseFetchChallengeCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LicenseFetchChallengeCustomData: usize,
    #[cfg(feature = "deprecated")]
    pub SetLicenseFetchChallengeCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, licensefetchchallengecustomdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetLicenseFetchChallengeCustomData: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INDLicenseFetchDescriptorFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for INDLicenseFetchDescriptorFactory {
    type Vtable = INDLicenseFetchDescriptorFactory_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for INDLicenseFetchDescriptorFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd0031202_cfac_4f00_ae6a_97af80b848f2);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchDescriptorFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentidtype: NDContentIDType, contentIDBytes_array_size: u32, contentidbytes: *const u8, licensefetchchallengecustomdata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateInstance: usize,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INDLicenseFetchResult(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDLicenseFetchResult {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(INDLicenseFetchResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for INDLicenseFetchResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{21d39698-aa62-45ff-a5ff-8037e5433825}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for INDLicenseFetchResult {
    type Vtable = INDLicenseFetchResult_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for INDLicenseFetchResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x21d39698_aa62_45ff_a5ff_8037e5433825);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDLicenseFetchResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ResponseCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ResponseCustomData: usize,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INDMessenger(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDMessenger {
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SendRegistrationRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SendRegistrationRequestAsync)(::windows_core::Interface::as_raw(this), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), challengedatabytes.len().try_into().unwrap(), challengedatabytes.as_ptr(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SendProximityDetectionStartAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SendProximityDetectionStartAsync)(::windows_core::Interface::as_raw(this), pdtype, transmitterchannelbytes.len().try_into().unwrap(), transmitterchannelbytes.as_ptr(), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), challengedatabytes.len().try_into().unwrap(), challengedatabytes.as_ptr(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SendProximityDetectionResponseAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], responsedatabytes: &[u8]) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SendProximityDetectionResponseAsync)(::windows_core::Interface::as_raw(this), pdtype, transmitterchannelbytes.len().try_into().unwrap(), transmitterchannelbytes.as_ptr(), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), responsedatabytes.len().try_into().unwrap(), responsedatabytes.as_ptr(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SendLicenseFetchRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SendLicenseFetchRequestAsync)(::windows_core::Interface::as_raw(this), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), challengedatabytes.len().try_into().unwrap(), challengedatabytes.as_ptr(), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(INDMessenger, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for INDMessenger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{d42df95d-a75b-47bf-8249-bc83820da38a}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for INDMessenger {
    type Vtable = INDMessenger_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for INDMessenger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd42df95d_a75b_47bf_8249_bc83820da38a);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDMessenger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SendRegistrationRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SendRegistrationRequestAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SendProximityDetectionStartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdtype: NDProximityDetectionType, transmitterChannelBytes_array_size: u32, transmitterchannelbytes: *const u8, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SendProximityDetectionStartAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SendProximityDetectionResponseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdtype: NDProximityDetectionType, transmitterChannelBytes_array_size: u32, transmitterchannelbytes: *const u8, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, responseDataBytes_array_size: u32, responsedatabytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SendProximityDetectionResponseAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SendLicenseFetchRequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionIDBytes_array_size: u32, sessionidbytes: *const u8, challengeDataBytes_array_size: u32, challengedatabytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SendLicenseFetchRequestAsync: usize,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INDProximityDetectionCompletedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDProximityDetectionCompletedEventArgs {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn ProximityDetectionRetryCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProximityDetectionRetryCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(INDProximityDetectionCompletedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for INDProximityDetectionCompletedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{2a706328-da25-4f8c-9eb7-5d0fc3658bca}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for INDProximityDetectionCompletedEventArgs {
    type Vtable = INDProximityDetectionCompletedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for INDProximityDetectionCompletedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a706328_da25_4f8c_9eb7_5d0fc3658bca);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDProximityDetectionCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ProximityDetectionRetryCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ProximityDetectionRetryCount: usize,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INDRegistrationCompletedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDRegistrationCompletedEventArgs {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn TransmitterProperties(&self) -> ::windows_core::Result<INDTransmitterProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransmitterProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn TransmitterCertificateAccepted(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransmitterCertificateAccepted)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn SetTransmitterCertificateAccepted(&self, accept: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTransmitterCertificateAccepted)(::windows_core::Interface::as_raw(this), accept).ok() }
    }
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(INDRegistrationCompletedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for INDRegistrationCompletedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{9e39b64d-ab5b-4905-acdc-787a77c6374d}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for INDRegistrationCompletedEventArgs {
    type Vtable = INDRegistrationCompletedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for INDRegistrationCompletedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e39b64d_ab5b_4905_acdc_787a77c6374d);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDRegistrationCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ResponseCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ResponseCustomData: usize,
    #[cfg(feature = "deprecated")]
    pub TransmitterProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TransmitterProperties: usize,
    #[cfg(feature = "deprecated")]
    pub TransmitterCertificateAccepted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TransmitterCertificateAccepted: usize,
    #[cfg(feature = "deprecated")]
    pub SetTransmitterCertificateAccepted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accept: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetTransmitterCertificateAccepted: usize,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INDSendResult(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDSendResult {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn Response(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).Response)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(INDSendResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for INDSendResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{e3685517-a584-479d-90b7-d689c7bf7c80}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for INDSendResult {
    type Vtable = INDSendResult_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for INDSendResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3685517_a584_479d_90b7_d689c7bf7c80);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDSendResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Response: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Response: usize,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INDStartResult(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDStartResult {
    #[doc = "Required features: `\"Media_Core\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub fn MediaStreamSource(&self) -> ::windows_core::Result<super::super::Core::MediaStreamSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaStreamSource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(INDStartResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for INDStartResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{79f6e96e-f50f-4015-8ba4-c2bc344ebd4e}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for INDStartResult {
    type Vtable = INDStartResult_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for INDStartResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79f6e96e_f50f_4015_8ba4_c2bc344ebd4e);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDStartResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub MediaStreamSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "deprecated")))]
    MediaStreamSource: usize,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INDStorageFileHelper(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDStorageFileHelper {
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub fn GetFileURLs<P0>(&self, file: P0) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Storage::IStorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFileURLs)(::windows_core::Interface::as_raw(this), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(INDStorageFileHelper, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for INDStorageFileHelper {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{d8f0bef8-91d2-4d47-a3f9-eaff4edb729f}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for INDStorageFileHelper {
    type Vtable = INDStorageFileHelper_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for INDStorageFileHelper {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8f0bef8_91d2_4d47_a3f9_eaff4edb729f);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDStorageFileHelper_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub GetFileURLs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated")))]
    GetFileURLs: usize,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INDStreamParser(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDStreamParser {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn ParseData(&self, databytes: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ParseData)(::windows_core::Interface::as_raw(this), databytes.len().try_into().unwrap(), databytes.as_ptr()).ok() }
    }
    #[doc = "Required features: `\"Media_Core\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub fn GetStreamInformation<P0>(&self, descriptor: P0, streamtype: &mut NDMediaStreamType) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::TryIntoParam<super::super::Core::IMediaStreamDescriptor>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStreamInformation)(::windows_core::Interface::as_raw(this), descriptor.try_into_param()?.abi(), streamtype, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn BeginOfStream(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).BeginOfStream)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn EndOfStream(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).EndOfStream)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn Notifier(&self) -> ::windows_core::Result<NDStreamParserNotifier> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Notifier)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(INDStreamParser, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for INDStreamParser {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{e0baa198-9796-41c9-8695-59437e67e66a}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for INDStreamParser {
    type Vtable = INDStreamParser_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for INDStreamParser {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0baa198_9796_41c9_8695_59437e67e66a);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDStreamParser_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ParseData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dataBytes_array_size: u32, databytes: *const u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ParseData: usize,
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub GetStreamInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: *mut ::core::ffi::c_void, streamtype: *mut NDMediaStreamType, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "deprecated")))]
    GetStreamInformation: usize,
    #[cfg(feature = "deprecated")]
    pub BeginOfStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BeginOfStream: usize,
    #[cfg(feature = "deprecated")]
    pub EndOfStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    EndOfStream: usize,
    #[cfg(feature = "deprecated")]
    pub Notifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Notifier: usize,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INDStreamParserNotifier(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDStreamParserNotifier {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn OnContentIDReceived<P0>(&self, licensefetchdescriptor: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<INDLicenseFetchDescriptor>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnContentIDReceived)(::windows_core::Interface::as_raw(this), licensefetchdescriptor.try_into_param()?.abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated"))]
    pub fn OnMediaStreamDescriptorCreated<P0, P1>(&self, audiostreamdescriptors: P0, videostreamdescriptors: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IVector<super::super::Core::AudioStreamDescriptor>>,
        P1: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IVector<super::super::Core::VideoStreamDescriptor>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnMediaStreamDescriptorCreated)(::windows_core::Interface::as_raw(this), audiostreamdescriptors.try_into_param()?.abi(), videostreamdescriptors.try_into_param()?.abi()).ok() }
    }
    #[doc = "Required features: `\"Media_Core\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub fn OnSampleParsed<P0>(&self, streamid: u32, streamtype: NDMediaStreamType, streamsample: P0, pts: i64, ccformat: NDClosedCaptionFormat, ccdatabytes: &[u8]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Core::MediaStreamSample>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnSampleParsed)(::windows_core::Interface::as_raw(this), streamid, streamtype, streamsample.into_param().abi(), pts, ccformat, ccdatabytes.len().try_into().unwrap(), ccdatabytes.as_ptr()).ok() }
    }
    #[doc = "Required features: `\"Media_Core\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub fn OnBeginSetupDecryptor<P0>(&self, descriptor: P0, keyid: ::windows_core::GUID, probytes: &[u8]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Core::IMediaStreamDescriptor>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnBeginSetupDecryptor)(::windows_core::Interface::as_raw(this), descriptor.try_into_param()?.abi(), keyid, probytes.len().try_into().unwrap(), probytes.as_ptr()).ok() }
    }
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(INDStreamParserNotifier, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for INDStreamParserNotifier {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{c167acd0-2ce6-426c-ace5-5e9275fea715}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for INDStreamParserNotifier {
    type Vtable = INDStreamParserNotifier_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for INDStreamParserNotifier {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc167acd0_2ce6_426c_ace5_5e9275fea715);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDStreamParserNotifier_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub OnContentIDReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, licensefetchdescriptor: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OnContentIDReceived: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated"))]
    pub OnMediaStreamDescriptorCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiostreamdescriptors: *mut ::core::ffi::c_void, videostreamdescriptors: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated")))]
    OnMediaStreamDescriptorCreated: usize,
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub OnSampleParsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamid: u32, streamtype: NDMediaStreamType, streamsample: *mut ::core::ffi::c_void, pts: i64, ccformat: NDClosedCaptionFormat, ccDataBytes_array_size: u32, ccdatabytes: *const u8) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "deprecated")))]
    OnSampleParsed: usize,
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub OnBeginSetupDecryptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: *mut ::core::ffi::c_void, keyid: ::windows_core::GUID, proBytes_array_size: u32, probytes: *const u8) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "deprecated")))]
    OnBeginSetupDecryptor: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INDTCPMessengerFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for INDTCPMessengerFactory {
    type Vtable = INDTCPMessengerFactory_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for INDTCPMessengerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7dd85cfe_1b99_4f68_8f82_8177f7cedf2b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDTCPMessengerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotehostname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, remotehostport: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateInstance: usize,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INDTransmitterProperties(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl INDTransmitterProperties {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn CertificateType(&self) -> ::windows_core::Result<NDCertificateType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CertificateType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn PlatformIdentifier(&self) -> ::windows_core::Result<NDCertificatePlatformID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlatformIdentifier)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn SupportedFeatures(&self) -> ::windows_core::Result<::windows_core::Array<NDCertificateFeature>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).SupportedFeatures)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<NDCertificateFeature>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn SecurityLevel(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SecurityLevel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn SecurityVersion(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SecurityVersion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ExpirationDate(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationDate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn ClientID(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).ClientID)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn ModelDigest(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).ModelDigest)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn ModelManufacturerName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ModelManufacturerName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn ModelName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ModelName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn ModelNumber(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ModelNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(INDTransmitterProperties, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for INDTransmitterProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{e536af23-ac4f-4adc-8c66-4ff7c2702dd6}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for INDTransmitterProperties {
    type Vtable = INDTransmitterProperties_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for INDTransmitterProperties {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe536af23_ac4f_4adc_8c66_4ff7c2702dd6);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct INDTransmitterProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CertificateType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NDCertificateType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CertificateType: usize,
    #[cfg(feature = "deprecated")]
    pub PlatformIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NDCertificatePlatformID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlatformIdentifier: usize,
    #[cfg(feature = "deprecated")]
    pub SupportedFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut NDCertificateFeature) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SupportedFeatures: usize,
    #[cfg(feature = "deprecated")]
    pub SecurityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SecurityLevel: usize,
    #[cfg(feature = "deprecated")]
    pub SecurityVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SecurityVersion: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    ExpirationDate: usize,
    #[cfg(feature = "deprecated")]
    pub ClientID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ClientID: usize,
    #[cfg(feature = "deprecated")]
    pub ModelDigest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ModelDigest: usize,
    #[cfg(feature = "deprecated")]
    pub ModelManufacturerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ModelManufacturerName: usize,
    #[cfg(feature = "deprecated")]
    pub ModelName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ModelName: usize,
    #[cfg(feature = "deprecated")]
    pub ModelNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ModelNumber: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyContentHeader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyContentHeader {
    type Vtable = IPlayReadyContentHeader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyContentHeader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a438a6a_7f4c_452e_88bd_0148c6387a2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub KeyId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub KeyIdString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LicenseAcquisitionUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LicenseAcquisitionUrl: usize,
    #[cfg(feature = "Foundation")]
    pub LicenseAcquisitionUserInterfaceUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LicenseAcquisitionUserInterfaceUrl: usize,
    pub DomainServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub EncryptionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlayReadyEncryptionAlgorithm) -> ::windows_core::HRESULT,
    pub CustomAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DecryptorSetup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlayReadyDecryptorSetup) -> ::windows_core::HRESULT,
    pub GetSerializedHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    pub HeaderWithEmbeddedUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyContentHeader2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyContentHeader2 {
    type Vtable = IPlayReadyContentHeader2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyContentHeader2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x359c79f4_2180_498c_965b_e754d875eab2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeader2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub KeyIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub KeyIdStrings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyContentHeaderFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyContentHeaderFactory {
    type Vtable = IPlayReadyContentHeaderFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyContentHeaderFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb97c8ff_b758_4776_bf01_217a8b510b2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeaderFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateInstanceFromWindowsMediaDrmHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headerBytes_array_size: u32, headerbytes: *const u8, licenseacquisitionurl: *mut ::core::ffi::c_void, licenseacquisitionuserinterfaceurl: *mut ::core::ffi::c_void, customattributes: ::std::mem::MaybeUninit<::windows_core::HSTRING>, domainserviceid: ::windows_core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstanceFromWindowsMediaDrmHeader: usize,
    #[cfg(feature = "Foundation")]
    pub CreateInstanceFromComponents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentkeyid: ::windows_core::GUID, contentkeyidstring: ::std::mem::MaybeUninit<::windows_core::HSTRING>, contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: *mut ::core::ffi::c_void, licenseacquisitionuserinterfaceurl: *mut ::core::ffi::c_void, customattributes: ::std::mem::MaybeUninit<::windows_core::HSTRING>, domainserviceid: ::windows_core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstanceFromComponents: usize,
    pub CreateInstanceFromPlayReadyHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headerBytes_array_size: u32, headerbytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyContentHeaderFactory2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyContentHeaderFactory2 {
    type Vtable = IPlayReadyContentHeaderFactory2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyContentHeaderFactory2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd1239cf5_ae6d_4778_97fd_6e3a2eeadbeb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentHeaderFactory2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateInstanceFromComponents2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, contentKeyIds_array_size: u32, contentkeyids: *const ::windows_core::GUID, contentKeyIdStrings_array_size: u32, contentkeyidstrings: *const ::std::mem::MaybeUninit<::windows_core::HSTRING>, contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: *mut ::core::ffi::c_void, licenseacquisitionuserinterfaceurl: *mut ::core::ffi::c_void, customattributes: ::std::mem::MaybeUninit<::windows_core::HSTRING>, domainserviceid: ::windows_core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstanceFromComponents2: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyContentResolver(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyContentResolver {
    type Vtable = IPlayReadyContentResolver_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyContentResolver {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfbfd2523_906d_4982_a6b8_6849565a7ce8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyContentResolver_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ServiceRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentheader: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyDomain(::windows_core::IUnknown);
impl IPlayReadyDomain {
    pub fn AccountId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccountId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Revision(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Revision)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FriendlyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DomainJoinUrl(&self) -> ::windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DomainJoinUrl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IPlayReadyDomain, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IPlayReadyDomain {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{adcc93ac-97e6-43ef-95e4-d7868f3b16a9}");
}
unsafe impl ::windows_core::Interface for IPlayReadyDomain {
    type Vtable = IPlayReadyDomain_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyDomain {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xadcc93ac_97e6_43ef_95e4_d7868f3b16a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomain_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Revision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DomainJoinUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DomainJoinUrl: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyDomainIterableFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyDomainIterableFactory {
    type Vtable = IPlayReadyDomainIterableFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyDomainIterableFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4df384ee_3121_4df3_a5e8_d0c24c0500fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomainIterableFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, domainaccountid: ::windows_core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyDomainJoinServiceRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyDomainJoinServiceRequest {
    type Vtable = IPlayReadyDomainJoinServiceRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyDomainJoinServiceRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x171b4a5a_405f_4739_b040_67b9f0c38758);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomainJoinServiceRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DomainAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetDomainAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub DomainFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDomainFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DomainServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetDomainServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyDomainLeaveServiceRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyDomainLeaveServiceRequest {
    type Vtable = IPlayReadyDomainLeaveServiceRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyDomainLeaveServiceRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x062d58be_97ad_4917_aa03_46d4c252d464);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyDomainLeaveServiceRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DomainAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetDomainAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub DomainServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetDomainServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyITADataGenerator(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyITADataGenerator {
    type Vtable = IPlayReadyITADataGenerator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyITADataGenerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24446b8e_10b9_4530_b25b_901a8029a9b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyITADataGenerator_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GenerateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidcpsystemid: ::windows_core::GUID, countofstreams: u32, configuration: *mut ::core::ffi::c_void, format: PlayReadyITADataFormat, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GenerateData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyIndividualizationServiceRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyIndividualizationServiceRequest {
    type Vtable = IPlayReadyIndividualizationServiceRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyIndividualizationServiceRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x21f5a86b_008c_4611_ab2f_aaa6c69f0e24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyIndividualizationServiceRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyLicense(::windows_core::IUnknown);
impl IPlayReadyLicense {
    pub fn FullyEvaluated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FullyEvaluated)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UsableForPlay(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UsableForPlay)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationDate(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationDate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExpireAfterFirstPlay(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExpireAfterFirstPlay)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DomainAccountID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DomainAccountID)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChainDepth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChainDepth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetKIDAtChainDepth(&self, chaindepth: u32) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetKIDAtChainDepth)(::windows_core::Interface::as_raw(this), chaindepth, &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IPlayReadyLicense, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IPlayReadyLicense {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{ee474c4e-fa3c-414d-a9f2-3ffc1ef832d4}");
}
unsafe impl ::windows_core::Interface for IPlayReadyLicense {
    type Vtable = IPlayReadyLicense_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyLicense {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee474c4e_fa3c_414d_a9f2_3ffc1ef832d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicense_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FullyEvaluated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub UsableForPlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationDate: usize,
    pub ExpireAfterFirstPlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub DomainAccountID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub ChainDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub GetKIDAtChainDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, chaindepth: u32, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyLicense2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyLicense2 {
    type Vtable = IPlayReadyLicense2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyLicense2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30f4e7a7_d8e3_48a0_bcda_ff9f40530436);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicense2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SecureStopId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SecurityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub InMemoryOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ExpiresInRealTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest(::windows_core::IUnknown);
impl IPlayReadyLicenseAcquisitionServiceRequest {
    pub fn ContentHeader(&self) -> ::windows_core::Result<PlayReadyContentHeader> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentHeader)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContentHeader<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PlayReadyContentHeader>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentHeader)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DomainServiceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DomainServiceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDomainServiceId(&self, value: ::windows_core::GUID) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDomainServiceId)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChallengeCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetChallengeCustomData(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetChallengeCustomData)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BeginServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows_core::Result<IPlayReadyServiceRequest> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows_core::Result<PlayReadySoapMessage> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(::windows_core::Interface::as_raw(this), responsebytes.len().try_into().unwrap(), responsebytes.as_ptr(), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IPlayReadyLicenseAcquisitionServiceRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IMediaProtectionServiceRequest> for IPlayReadyLicenseAcquisitionServiceRequest {}
impl ::windows_core::CanTryInto<IPlayReadyServiceRequest> for IPlayReadyLicenseAcquisitionServiceRequest {}
impl ::windows_core::RuntimeType for IPlayReadyLicenseAcquisitionServiceRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{5d85ff45-3e9f-4f48-93e1-9530c8d58c3e}");
}
unsafe impl ::windows_core::Interface for IPlayReadyLicenseAcquisitionServiceRequest {
    type Vtable = IPlayReadyLicenseAcquisitionServiceRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyLicenseAcquisitionServiceRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d85ff45_3e9f_4f48_93e1_9530c8d58c3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ContentHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetContentHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DomainServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetDomainServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyLicenseAcquisitionServiceRequest2 {
    type Vtable = IPlayReadyLicenseAcquisitionServiceRequest2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyLicenseAcquisitionServiceRequest2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7fa5eb5_fe0c_b225_bc60_5a9edd32ceb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyLicenseAcquisitionServiceRequest3 {
    type Vtable = IPlayReadyLicenseAcquisitionServiceRequest3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyLicenseAcquisitionServiceRequest3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x394e5f4d_7f75_430d_b2e7_7f75f34b2d75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseAcquisitionServiceRequest3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateLicenseIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentheader: *mut ::core::ffi::c_void, fullyevaluated: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateLicenseIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyLicenseIterableFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyLicenseIterableFactory {
    type Vtable = IPlayReadyLicenseIterableFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyLicenseIterableFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4179f08_0837_4978_8e68_be4293c8d7a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseIterableFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentheader: *mut ::core::ffi::c_void, fullyevaluated: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyLicenseManagement(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyLicenseManagement {
    type Vtable = IPlayReadyLicenseManagement_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyLicenseManagement {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaaeb2141_0957_4405_b892_8bf3ec5dadd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseManagement_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub DeleteLicenses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentheader: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteLicenses: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyLicenseSession(::windows_core::IUnknown);
impl IPlayReadyLicenseSession {
    pub fn CreateLAServiceRequest(&self) -> ::windows_core::Result<IPlayReadyLicenseAcquisitionServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateLAServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConfigureMediaProtectionManager<P0>(&self, mpm: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::MediaProtectionManager>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureMediaProtectionManager)(::windows_core::Interface::as_raw(this), mpm.into_param().abi()).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IPlayReadyLicenseSession, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IPlayReadyLicenseSession {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{a1723a39-87fa-4fdd-abbb-a9720e845259}");
}
unsafe impl ::windows_core::Interface for IPlayReadyLicenseSession {
    type Vtable = IPlayReadyLicenseSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyLicenseSession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1723a39_87fa_4fdd_abbb_a9720e845259);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseSession_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateLAServiceRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ConfigureMediaProtectionManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mpm: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyLicenseSession2(::windows_core::IUnknown);
impl IPlayReadyLicenseSession2 {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateLicenseIterable<P0>(&self, contentheader: P0, fullyevaluated: bool) -> ::windows_core::Result<PlayReadyLicenseIterable>
    where
        P0: ::windows_core::IntoParam<PlayReadyContentHeader>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateLicenseIterable)(::windows_core::Interface::as_raw(this), contentheader.into_param().abi(), fullyevaluated, &mut result__).from_abi(result__)
        }
    }
    pub fn CreateLAServiceRequest(&self) -> ::windows_core::Result<IPlayReadyLicenseAcquisitionServiceRequest> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyLicenseSession>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateLAServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConfigureMediaProtectionManager<P0>(&self, mpm: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::MediaProtectionManager>,
    {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyLicenseSession>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureMediaProtectionManager)(::windows_core::Interface::as_raw(this), mpm.into_param().abi()).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IPlayReadyLicenseSession2, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPlayReadyLicenseSession> for IPlayReadyLicenseSession2 {}
impl ::windows_core::RuntimeType for IPlayReadyLicenseSession2 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{4909be3a-3aed-4656-8ad7-ee0fd7799510}");
}
unsafe impl ::windows_core::Interface for IPlayReadyLicenseSession2 {
    type Vtable = IPlayReadyLicenseSession2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyLicenseSession2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4909be3a_3aed_4656_8ad7_ee0fd7799510);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseSession2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateLicenseIterable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentheader: *mut ::core::ffi::c_void, fullyevaluated: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateLicenseIterable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyLicenseSessionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyLicenseSessionFactory {
    type Vtable = IPlayReadyLicenseSessionFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyLicenseSessionFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x62492699_6527_429e_98be_48d798ac2739);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyLicenseSessionFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyMeteringReportServiceRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyMeteringReportServiceRequest {
    type Vtable = IPlayReadyMeteringReportServiceRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyMeteringReportServiceRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc12b231c_0ecd_4f11_a185_1e24a4a67fb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyMeteringReportServiceRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MeteringCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    pub SetMeteringCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, meteringCertBytes_array_size: u32, meteringcertbytes: *const u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyRevocationServiceRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyRevocationServiceRequest {
    type Vtable = IPlayReadyRevocationServiceRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyRevocationServiceRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x543d66ac_faf0_4560_84a5_0e4acec939e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyRevocationServiceRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadySecureStopIterableFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadySecureStopIterableFactory {
    type Vtable = IPlayReadySecureStopIterableFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadySecureStopIterableFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f1f0165_4214_4d9e_81eb_e89f9d294aee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySecureStopIterableFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, publisherCertBytes_array_size: u32, publishercertbytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadySecureStopServiceRequest(::windows_core::IUnknown);
impl IPlayReadySecureStopServiceRequest {
    pub fn SessionID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SessionID)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateTime(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Stopped(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PublisherCertificate(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).PublisherCertificate)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChallengeCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetChallengeCustomData(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetChallengeCustomData)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BeginServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows_core::Result<IPlayReadyServiceRequest> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows_core::Result<PlayReadySoapMessage> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(::windows_core::Interface::as_raw(this), responsebytes.len().try_into().unwrap(), responsebytes.as_ptr(), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IPlayReadySecureStopServiceRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IMediaProtectionServiceRequest> for IPlayReadySecureStopServiceRequest {}
impl ::windows_core::CanTryInto<IPlayReadyServiceRequest> for IPlayReadySecureStopServiceRequest {}
impl ::windows_core::RuntimeType for IPlayReadySecureStopServiceRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{b5501ee5-01bf-4401-9677-05630a6a4cc8}");
}
unsafe impl ::windows_core::Interface for IPlayReadySecureStopServiceRequest {
    type Vtable = IPlayReadySecureStopServiceRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadySecureStopServiceRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5501ee5_01bf_4401_9677_05630a6a4cc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySecureStopServiceRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SessionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateTime: usize,
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub PublisherCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadySecureStopServiceRequestFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadySecureStopServiceRequestFactory {
    type Vtable = IPlayReadySecureStopServiceRequestFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadySecureStopServiceRequestFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e448ac9_e67e_494e_9f49_6285438c76cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySecureStopServiceRequestFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, publisherCertBytes_array_size: u32, publishercertbytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateInstanceFromSessionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: ::windows_core::GUID, publisherCertBytes_array_size: u32, publishercertbytes: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyServiceRequest(::windows_core::IUnknown);
impl IPlayReadyServiceRequest {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChallengeCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetChallengeCustomData(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetChallengeCustomData)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BeginServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows_core::Result<IPlayReadyServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows_core::Result<PlayReadySoapMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(::windows_core::Interface::as_raw(this), responsebytes.len().try_into().unwrap(), responsebytes.as_ptr(), &mut result__).from_abi(result__)
        }
    }
    pub fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IPlayReadyServiceRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IMediaProtectionServiceRequest> for IPlayReadyServiceRequest {}
impl ::windows_core::RuntimeType for IPlayReadyServiceRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{8bad2836-a703-45a6-a180-76f3565aa725}");
}
unsafe impl ::windows_core::Interface for IPlayReadyServiceRequest {
    type Vtable = IPlayReadyServiceRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyServiceRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8bad2836_a703_45a6_a180_76f3565aa725);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyServiceRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    pub ResponseCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ChallengeCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetChallengeCustomData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BeginServiceRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BeginServiceRequest: usize,
    pub NextServiceRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GenerateManualEnablingChallenge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ProcessManualEnablingResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, responseBytes_array_size: u32, responsebytes: *const u8, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadySoapMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadySoapMessage {
    type Vtable = IPlayReadySoapMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadySoapMessage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb659fcb5_ce41_41ba_8a0d_61df5fffa139);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadySoapMessage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetMessageBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MessageHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MessageHeaders: usize,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyStatics {
    type Vtable = IPlayReadyStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e69c00d_247c_469a_8f31_5c1a1571d9c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DomainJoinServiceRequestType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub DomainLeaveServiceRequestType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub IndividualizationServiceRequestType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub LicenseAcquirerServiceRequestType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub MeteringReportServiceRequestType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub RevocationServiceRequestType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub MediaProtectionSystemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub PlayReadySecurityVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyStatics2 {
    type Vtable = IPlayReadyStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1f8d6a92_5f9a_423e_9466_b33969af7a3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PlayReadyCertificateSecurityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyStatics3 {
    type Vtable = IPlayReadyStatics3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3fa33f71_2dd3_4bed_ae49_f7148e63e710);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SecureStopServiceRequestType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub CheckSupportedHardware: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwdrmfeature: PlayReadyHardwareDRMFeatures, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyStatics4 {
    type Vtable = IPlayReadyStatics4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyStatics4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50a91300_d824_4231_9d5e_78ef8844c7d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub InputTrustAuthorityToCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ProtectionSystemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPlayReadyStatics5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPlayReadyStatics5 {
    type Vtable = IPlayReadyStatics5_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPlayReadyStatics5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x230a7075_dfa0_4f8e_a779_cefea9c6824b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayReadyStatics5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub HardwareDRMDisabledAtTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HardwareDRMDisabledAtTime: usize,
    #[cfg(feature = "Foundation")]
    pub HardwareDRMDisabledUntilTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HardwareDRMDisabledUntilTime: usize,
    pub ResetHardwareDRMDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NDClient(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl NDClient {
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RegistrationCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<NDClient, INDRegistrationCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegistrationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveRegistrationCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRegistrationCompleted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ProximityDetectionCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<NDClient, INDProximityDetectionCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProximityDetectionCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveProximityDetectionCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveProximityDetectionCompleted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn LicenseFetchCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<NDClient, INDLicenseFetchCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LicenseFetchCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveLicenseFetchCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLicenseFetchCompleted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ReRegistrationNeeded<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<NDClient, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReRegistrationNeeded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveReRegistrationNeeded(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveReRegistrationNeeded)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ClosedCaptionDataReceived<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<NDClient, INDClosedCaptionDataReceivedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClosedCaptionDataReceived)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveClosedCaptionDataReceived(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosedCaptionDataReceived)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn StartAsync<P0, P1, P2>(&self, contenturl: P0, startasyncoptions: u32, registrationcustomdata: P1, licensefetchdescriptor: P2) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDStartResult>>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
        P1: ::windows_core::TryIntoParam<INDCustomData>,
        P2: ::windows_core::TryIntoParam<INDLicenseFetchDescriptor>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartAsync)(::windows_core::Interface::as_raw(this), contenturl.into_param().abi(), startasyncoptions, registrationcustomdata.try_into_param()?.abi(), licensefetchdescriptor.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn LicenseFetchAsync<P0>(&self, licensefetchdescriptor: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDLicenseFetchResult>>
    where
        P0: ::windows_core::TryIntoParam<INDLicenseFetchDescriptor>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LicenseFetchAsync)(::windows_core::Interface::as_raw(this), licensefetchdescriptor.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn ReRegistrationAsync<P0>(&self, registrationcustomdata: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<INDCustomData>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReRegistrationAsync)(::windows_core::Interface::as_raw(this), registrationcustomdata.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn CreateInstance<P0, P1, P2>(downloadengine: P0, streamparser: P1, pmessenger: P2) -> ::windows_core::Result<NDClient>
    where
        P0: ::windows_core::TryIntoParam<INDDownloadEngine>,
        P1: ::windows_core::TryIntoParam<INDStreamParser>,
        P2: ::windows_core::TryIntoParam<INDMessenger>,
    {
        Self::INDClientFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), downloadengine.try_into_param()?.abi(), streamparser.try_into_param()?.abi(), pmessenger.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn INDClientFactory<R, F: FnOnce(&INDClientFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NDClient, INDClientFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for NDClient {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDClient;{3bd6781b-61b8-46e2-99a5-8abcb6b9f7d6})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for NDClient {
    type Vtable = INDClient_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for NDClient {
    const IID: ::windows_core::GUID = <INDClient as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for NDClient {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDClient";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(NDClient, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NDCustomData(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl NDCustomData {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn CustomDataTypeID(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).CustomDataTypeID)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn CustomData(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).CustomData)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn CreateInstance(customdatatypeidbytes: &[u8], customdatabytes: &[u8]) -> ::windows_core::Result<NDCustomData> {
        Self::INDCustomDataFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), customdatatypeidbytes.len().try_into().unwrap(), customdatatypeidbytes.as_ptr(), customdatabytes.len().try_into().unwrap(), customdatabytes.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn INDCustomDataFactory<R, F: FnOnce(&INDCustomDataFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NDCustomData, INDCustomDataFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for NDCustomData {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDCustomData;{f5cb0fdc-2d09-4f19-b5e1-76a0b3ee9267})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for NDCustomData {
    type Vtable = INDCustomData_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for NDCustomData {
    const IID: ::windows_core::GUID = <INDCustomData as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for NDCustomData {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDCustomData";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(NDCustomData, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<INDCustomData> for NDCustomData {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NDDownloadEngineNotifier(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl NDDownloadEngineNotifier {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NDDownloadEngineNotifier, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn OnStreamOpened(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnStreamOpened)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn OnPlayReadyObjectReceived(&self, databytes: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnPlayReadyObjectReceived)(::windows_core::Interface::as_raw(this), databytes.len().try_into().unwrap(), databytes.as_ptr()).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn OnContentIDReceived<P0>(&self, licensefetchdescriptor: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<INDLicenseFetchDescriptor>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnContentIDReceived)(::windows_core::Interface::as_raw(this), licensefetchdescriptor.try_into_param()?.abi()).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn OnDataReceived(&self, databytes: &[u8], bytesreceived: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnDataReceived)(::windows_core::Interface::as_raw(this), databytes.len().try_into().unwrap(), databytes.as_ptr(), bytesreceived).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn OnEndOfStream(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnEndOfStream)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn OnNetworkError(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnNetworkError)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for NDDownloadEngineNotifier {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDDownloadEngineNotifier;{d720b4d4-f4b8-4530-a809-9193a571e7fc})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for NDDownloadEngineNotifier {
    type Vtable = INDDownloadEngineNotifier_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for NDDownloadEngineNotifier {
    const IID: ::windows_core::GUID = <INDDownloadEngineNotifier as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for NDDownloadEngineNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDDownloadEngineNotifier";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(NDDownloadEngineNotifier, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<INDDownloadEngineNotifier> for NDDownloadEngineNotifier {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NDLicenseFetchDescriptor(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl NDLicenseFetchDescriptor {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn ContentIDType(&self) -> ::windows_core::Result<NDContentIDType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentIDType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn ContentID(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).ContentID)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn LicenseFetchChallengeCustomData(&self) -> ::windows_core::Result<INDCustomData> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LicenseFetchChallengeCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn SetLicenseFetchChallengeCustomData<P0>(&self, licensefetchchallengecustomdata: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<INDCustomData>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLicenseFetchChallengeCustomData)(::windows_core::Interface::as_raw(this), licensefetchchallengecustomdata.try_into_param()?.abi()).ok() }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn CreateInstance<P0>(contentidtype: NDContentIDType, contentidbytes: &[u8], licensefetchchallengecustomdata: P0) -> ::windows_core::Result<NDLicenseFetchDescriptor>
    where
        P0: ::windows_core::TryIntoParam<INDCustomData>,
    {
        Self::INDLicenseFetchDescriptorFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), contentidtype, contentidbytes.len().try_into().unwrap(), contentidbytes.as_ptr(), licensefetchchallengecustomdata.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn INDLicenseFetchDescriptorFactory<R, F: FnOnce(&INDLicenseFetchDescriptorFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NDLicenseFetchDescriptor, INDLicenseFetchDescriptorFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for NDLicenseFetchDescriptor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDLicenseFetchDescriptor;{5498d33a-e686-4935-a567-7ca77ad20fa4})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for NDLicenseFetchDescriptor {
    type Vtable = INDLicenseFetchDescriptor_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for NDLicenseFetchDescriptor {
    const IID: ::windows_core::GUID = <INDLicenseFetchDescriptor as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for NDLicenseFetchDescriptor {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDLicenseFetchDescriptor";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(NDLicenseFetchDescriptor, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<INDLicenseFetchDescriptor> for NDLicenseFetchDescriptor {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NDStorageFileHelper(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl NDStorageFileHelper {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NDStorageFileHelper, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub fn GetFileURLs<P0>(&self, file: P0) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVector<::windows_core::HSTRING>>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Storage::IStorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFileURLs)(::windows_core::Interface::as_raw(this), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for NDStorageFileHelper {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDStorageFileHelper;{d8f0bef8-91d2-4d47-a3f9-eaff4edb729f})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for NDStorageFileHelper {
    type Vtable = INDStorageFileHelper_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for NDStorageFileHelper {
    const IID: ::windows_core::GUID = <INDStorageFileHelper as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for NDStorageFileHelper {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDStorageFileHelper";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(NDStorageFileHelper, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<INDStorageFileHelper> for NDStorageFileHelper {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NDStreamParserNotifier(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl NDStreamParserNotifier {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NDStreamParserNotifier, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn OnContentIDReceived<P0>(&self, licensefetchdescriptor: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<INDLicenseFetchDescriptor>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnContentIDReceived)(::windows_core::Interface::as_raw(this), licensefetchdescriptor.try_into_param()?.abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "deprecated"))]
    pub fn OnMediaStreamDescriptorCreated<P0, P1>(&self, audiostreamdescriptors: P0, videostreamdescriptors: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IVector<super::super::Core::AudioStreamDescriptor>>,
        P1: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IVector<super::super::Core::VideoStreamDescriptor>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnMediaStreamDescriptorCreated)(::windows_core::Interface::as_raw(this), audiostreamdescriptors.try_into_param()?.abi(), videostreamdescriptors.try_into_param()?.abi()).ok() }
    }
    #[doc = "Required features: `\"Media_Core\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub fn OnSampleParsed<P0>(&self, streamid: u32, streamtype: NDMediaStreamType, streamsample: P0, pts: i64, ccformat: NDClosedCaptionFormat, ccdatabytes: &[u8]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Core::MediaStreamSample>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnSampleParsed)(::windows_core::Interface::as_raw(this), streamid, streamtype, streamsample.into_param().abi(), pts, ccformat, ccdatabytes.len().try_into().unwrap(), ccdatabytes.as_ptr()).ok() }
    }
    #[doc = "Required features: `\"Media_Core\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub fn OnBeginSetupDecryptor<P0>(&self, descriptor: P0, keyid: ::windows_core::GUID, probytes: &[u8]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Core::IMediaStreamDescriptor>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnBeginSetupDecryptor)(::windows_core::Interface::as_raw(this), descriptor.try_into_param()?.abi(), keyid, probytes.len().try_into().unwrap(), probytes.as_ptr()).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for NDStreamParserNotifier {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDStreamParserNotifier;{c167acd0-2ce6-426c-ace5-5e9275fea715})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for NDStreamParserNotifier {
    type Vtable = INDStreamParserNotifier_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for NDStreamParserNotifier {
    const IID: ::windows_core::GUID = <INDStreamParserNotifier as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for NDStreamParserNotifier {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDStreamParserNotifier";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(NDStreamParserNotifier, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<INDStreamParserNotifier> for NDStreamParserNotifier {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NDTCPMessenger(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl NDTCPMessenger {
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SendRegistrationRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SendRegistrationRequestAsync)(::windows_core::Interface::as_raw(this), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), challengedatabytes.len().try_into().unwrap(), challengedatabytes.as_ptr(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SendProximityDetectionStartAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SendProximityDetectionStartAsync)(::windows_core::Interface::as_raw(this), pdtype, transmitterchannelbytes.len().try_into().unwrap(), transmitterchannelbytes.as_ptr(), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), challengedatabytes.len().try_into().unwrap(), challengedatabytes.as_ptr(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SendProximityDetectionResponseAsync(&self, pdtype: NDProximityDetectionType, transmitterchannelbytes: &[u8], sessionidbytes: &[u8], responsedatabytes: &[u8]) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SendProximityDetectionResponseAsync)(::windows_core::Interface::as_raw(this), pdtype, transmitterchannelbytes.len().try_into().unwrap(), transmitterchannelbytes.as_ptr(), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), responsedatabytes.len().try_into().unwrap(), responsedatabytes.as_ptr(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SendLicenseFetchRequestAsync(&self, sessionidbytes: &[u8], challengedatabytes: &[u8]) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<INDSendResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SendLicenseFetchRequestAsync)(::windows_core::Interface::as_raw(this), sessionidbytes.len().try_into().unwrap(), sessionidbytes.as_ptr(), challengedatabytes.len().try_into().unwrap(), challengedatabytes.as_ptr(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn CreateInstance(remotehostname: &::windows_core::HSTRING, remotehostport: u32) -> ::windows_core::Result<NDTCPMessenger> {
        Self::INDTCPMessengerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(remotehostname), remotehostport, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn INDTCPMessengerFactory<R, F: FnOnce(&INDTCPMessengerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NDTCPMessenger, INDTCPMessengerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for NDTCPMessenger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.NDTCPMessenger;{d42df95d-a75b-47bf-8249-bc83820da38a})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for NDTCPMessenger {
    type Vtable = INDMessenger_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for NDTCPMessenger {
    const IID: ::windows_core::GUID = <INDMessenger as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for NDTCPMessenger {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.NDTCPMessenger";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(NDTCPMessenger, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<INDMessenger> for NDTCPMessenger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayReadyContentHeader(::windows_core::IUnknown);
impl PlayReadyContentHeader {
    pub fn KeyId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn KeyIdString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyIdString)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LicenseAcquisitionUrl(&self) -> ::windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LicenseAcquisitionUrl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LicenseAcquisitionUserInterfaceUrl(&self) -> ::windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LicenseAcquisitionUserInterfaceUrl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DomainServiceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DomainServiceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EncryptionType(&self) -> ::windows_core::Result<PlayReadyEncryptionAlgorithm> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EncryptionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CustomAttributes(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CustomAttributes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DecryptorSetup(&self) -> ::windows_core::Result<PlayReadyDecryptorSetup> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DecryptorSetup)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetSerializedHeader(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetSerializedHeader)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn HeaderWithEmbeddedUpdates(&self) -> ::windows_core::Result<PlayReadyContentHeader> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HeaderWithEmbeddedUpdates)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn KeyIds(&self) -> ::windows_core::Result<::windows_core::Array<::windows_core::GUID>> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyContentHeader2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).KeyIds)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<::windows_core::GUID>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn KeyIdStrings(&self) -> ::windows_core::Result<::windows_core::Array<::windows_core::HSTRING>> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyContentHeader2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).KeyIdStrings)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<::windows_core::HSTRING>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInstanceFromWindowsMediaDrmHeader<P0, P1>(headerbytes: &[u8], licenseacquisitionurl: P0, licenseacquisitionuserinterfaceurl: P1, customattributes: &::windows_core::HSTRING, domainserviceid: ::windows_core::GUID) -> ::windows_core::Result<PlayReadyContentHeader>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        Self::IPlayReadyContentHeaderFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceFromWindowsMediaDrmHeader)(::windows_core::Interface::as_raw(this), headerbytes.len().try_into().unwrap(), headerbytes.as_ptr(), licenseacquisitionurl.into_param().abi(), licenseacquisitionuserinterfaceurl.into_param().abi(), ::core::mem::transmute_copy(customattributes), domainserviceid, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInstanceFromComponents<P0, P1>(contentkeyid: ::windows_core::GUID, contentkeyidstring: &::windows_core::HSTRING, contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: P0, licenseacquisitionuserinterfaceurl: P1, customattributes: &::windows_core::HSTRING, domainserviceid: ::windows_core::GUID) -> ::windows_core::Result<PlayReadyContentHeader>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        Self::IPlayReadyContentHeaderFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceFromComponents)(::windows_core::Interface::as_raw(this), contentkeyid, ::core::mem::transmute_copy(contentkeyidstring), contentencryptionalgorithm, licenseacquisitionurl.into_param().abi(), licenseacquisitionuserinterfaceurl.into_param().abi(), ::core::mem::transmute_copy(customattributes), domainserviceid, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateInstanceFromPlayReadyHeader(headerbytes: &[u8]) -> ::windows_core::Result<PlayReadyContentHeader> {
        Self::IPlayReadyContentHeaderFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceFromPlayReadyHeader)(::windows_core::Interface::as_raw(this), headerbytes.len().try_into().unwrap(), headerbytes.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInstanceFromComponents2<P0, P1>(dwflags: u32, contentkeyids: &[::windows_core::GUID], contentkeyidstrings: &[::windows_core::HSTRING], contentencryptionalgorithm: PlayReadyEncryptionAlgorithm, licenseacquisitionurl: P0, licenseacquisitionuserinterfaceurl: P1, customattributes: &::windows_core::HSTRING, domainserviceid: ::windows_core::GUID) -> ::windows_core::Result<PlayReadyContentHeader>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        Self::IPlayReadyContentHeaderFactory2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceFromComponents2)(::windows_core::Interface::as_raw(this), dwflags, contentkeyids.len().try_into().unwrap(), contentkeyids.as_ptr(), contentkeyidstrings.len().try_into().unwrap(), ::core::mem::transmute(contentkeyidstrings.as_ptr()), contentencryptionalgorithm, licenseacquisitionurl.into_param().abi(), licenseacquisitionuserinterfaceurl.into_param().abi(), ::core::mem::transmute_copy(customattributes), domainserviceid, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlayReadyContentHeaderFactory<R, F: FnOnce(&IPlayReadyContentHeaderFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayReadyContentHeader, IPlayReadyContentHeaderFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPlayReadyContentHeaderFactory2<R, F: FnOnce(&IPlayReadyContentHeaderFactory2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayReadyContentHeader, IPlayReadyContentHeaderFactory2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PlayReadyContentHeader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyContentHeader;{9a438a6a-7f4c-452e-88bd-0148c6387a2c})");
}
unsafe impl ::windows_core::Interface for PlayReadyContentHeader {
    type Vtable = IPlayReadyContentHeader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PlayReadyContentHeader {
    const IID: ::windows_core::GUID = <IPlayReadyContentHeader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadyContentHeader {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyContentHeader";
}
::windows_core::imp::interface_hierarchy!(PlayReadyContentHeader, ::windows_core::IUnknown, ::windows_core::IInspectable);
pub struct PlayReadyContentResolver;
impl PlayReadyContentResolver {
    pub fn ServiceRequest<P0>(contentheader: P0) -> ::windows_core::Result<IPlayReadyServiceRequest>
    where
        P0: ::windows_core::IntoParam<PlayReadyContentHeader>,
    {
        Self::IPlayReadyContentResolver(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceRequest)(::windows_core::Interface::as_raw(this), contentheader.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlayReadyContentResolver<R, F: FnOnce(&IPlayReadyContentResolver) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayReadyContentResolver, IPlayReadyContentResolver> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for PlayReadyContentResolver {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyContentResolver";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayReadyDomain(::windows_core::IUnknown);
impl PlayReadyDomain {
    pub fn AccountId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccountId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Revision(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Revision)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FriendlyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DomainJoinUrl(&self) -> ::windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DomainJoinUrl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PlayReadyDomain {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomain;{adcc93ac-97e6-43ef-95e4-d7868f3b16a9})");
}
unsafe impl ::windows_core::Interface for PlayReadyDomain {
    type Vtable = IPlayReadyDomain_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PlayReadyDomain {
    const IID: ::windows_core::GUID = <IPlayReadyDomain as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadyDomain {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomain";
}
::windows_core::imp::interface_hierarchy!(PlayReadyDomain, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPlayReadyDomain> for PlayReadyDomain {}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayReadyDomainIterable(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadyDomainIterable {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateInstance(domainaccountid: ::windows_core::GUID) -> ::windows_core::Result<PlayReadyDomainIterable> {
        Self::IPlayReadyDomainIterableFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), domainaccountid, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlayReadyDomainIterableFactory<R, F: FnOnce(&IPlayReadyDomainIterableFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayReadyDomainIterable, IPlayReadyDomainIterableFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for PlayReadyDomainIterable {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomainIterable;pinterface({faa585ea-6214-4217-afda-7f46de5869b3};{adcc93ac-97e6-43ef-95e4-d7868f3b16a9}))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for PlayReadyDomainIterable {
    type Vtable = super::super::super::Foundation::Collections::IIterable_Vtbl<IPlayReadyDomain>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for PlayReadyDomainIterable {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for PlayReadyDomainIterable {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainIterable";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for PlayReadyDomainIterable {
    type Item = IPlayReadyDomain;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &PlayReadyDomainIterable {
    type Item = IPlayReadyDomain;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(PlayReadyDomainIterable, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterable<IPlayReadyDomain>> for PlayReadyDomainIterable {}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayReadyDomainIterator(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadyDomainIterator {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Current(&self) -> ::windows_core::Result<IPlayReadyDomain> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasCurrent(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasCurrent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MoveNext(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MoveNext)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, items: &mut [::core::option::Option<IPlayReadyDomain>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for PlayReadyDomainIterator {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomainIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};{adcc93ac-97e6-43ef-95e4-d7868f3b16a9}))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for PlayReadyDomainIterator {
    type Vtable = super::super::super::Foundation::Collections::IIterator_Vtbl<IPlayReadyDomain>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for PlayReadyDomainIterator {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for PlayReadyDomainIterator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainIterator";
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(PlayReadyDomainIterator, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterator<IPlayReadyDomain>> for PlayReadyDomainIterator {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayReadyDomainJoinServiceRequest(::windows_core::IUnknown);
impl PlayReadyDomainJoinServiceRequest {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayReadyDomainJoinServiceRequest, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DomainAccountId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DomainAccountId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDomainAccountId(&self, value: ::windows_core::GUID) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDomainAccountId)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DomainFriendlyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DomainFriendlyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDomainFriendlyName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDomainFriendlyName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DomainServiceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DomainServiceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDomainServiceId(&self, value: ::windows_core::GUID) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDomainServiceId)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChallengeCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetChallengeCustomData(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetChallengeCustomData)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BeginServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows_core::Result<IPlayReadyServiceRequest> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows_core::Result<PlayReadySoapMessage> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(::windows_core::Interface::as_raw(this), responsebytes.len().try_into().unwrap(), responsebytes.as_ptr(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PlayReadyDomainJoinServiceRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomainJoinServiceRequest;{171b4a5a-405f-4739-b040-67b9f0c38758})");
}
unsafe impl ::windows_core::Interface for PlayReadyDomainJoinServiceRequest {
    type Vtable = IPlayReadyDomainJoinServiceRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PlayReadyDomainJoinServiceRequest {
    const IID: ::windows_core::GUID = <IPlayReadyDomainJoinServiceRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadyDomainJoinServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainJoinServiceRequest";
}
::windows_core::imp::interface_hierarchy!(PlayReadyDomainJoinServiceRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IMediaProtectionServiceRequest> for PlayReadyDomainJoinServiceRequest {}
impl ::windows_core::CanTryInto<IPlayReadyServiceRequest> for PlayReadyDomainJoinServiceRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayReadyDomainLeaveServiceRequest(::windows_core::IUnknown);
impl PlayReadyDomainLeaveServiceRequest {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayReadyDomainLeaveServiceRequest, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DomainAccountId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DomainAccountId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDomainAccountId(&self, value: ::windows_core::GUID) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDomainAccountId)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DomainServiceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DomainServiceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDomainServiceId(&self, value: ::windows_core::GUID) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDomainServiceId)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChallengeCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetChallengeCustomData(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetChallengeCustomData)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BeginServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows_core::Result<IPlayReadyServiceRequest> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows_core::Result<PlayReadySoapMessage> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(::windows_core::Interface::as_raw(this), responsebytes.len().try_into().unwrap(), responsebytes.as_ptr(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PlayReadyDomainLeaveServiceRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyDomainLeaveServiceRequest;{062d58be-97ad-4917-aa03-46d4c252d464})");
}
unsafe impl ::windows_core::Interface for PlayReadyDomainLeaveServiceRequest {
    type Vtable = IPlayReadyDomainLeaveServiceRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PlayReadyDomainLeaveServiceRequest {
    const IID: ::windows_core::GUID = <IPlayReadyDomainLeaveServiceRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadyDomainLeaveServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyDomainLeaveServiceRequest";
}
::windows_core::imp::interface_hierarchy!(PlayReadyDomainLeaveServiceRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IMediaProtectionServiceRequest> for PlayReadyDomainLeaveServiceRequest {}
impl ::windows_core::CanTryInto<IPlayReadyServiceRequest> for PlayReadyDomainLeaveServiceRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayReadyITADataGenerator(::windows_core::IUnknown);
impl PlayReadyITADataGenerator {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayReadyITADataGenerator, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GenerateData<P0>(&self, guidcpsystemid: ::windows_core::GUID, countofstreams: u32, configuration: P0, format: PlayReadyITADataFormat) -> ::windows_core::Result<::windows_core::Array<u8>>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GenerateData)(::windows_core::Interface::as_raw(this), guidcpsystemid, countofstreams, configuration.try_into_param()?.abi(), format, ::windows_core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
impl ::windows_core::RuntimeType for PlayReadyITADataGenerator {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyITADataGenerator;{24446b8e-10b9-4530-b25b-901a8029a9b2})");
}
unsafe impl ::windows_core::Interface for PlayReadyITADataGenerator {
    type Vtable = IPlayReadyITADataGenerator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PlayReadyITADataGenerator {
    const IID: ::windows_core::GUID = <IPlayReadyITADataGenerator as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadyITADataGenerator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyITADataGenerator";
}
::windows_core::imp::interface_hierarchy!(PlayReadyITADataGenerator, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayReadyIndividualizationServiceRequest(::windows_core::IUnknown);
impl PlayReadyIndividualizationServiceRequest {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayReadyIndividualizationServiceRequest, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChallengeCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetChallengeCustomData(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetChallengeCustomData)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BeginServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows_core::Result<IPlayReadyServiceRequest> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows_core::Result<PlayReadySoapMessage> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(::windows_core::Interface::as_raw(this), responsebytes.len().try_into().unwrap(), responsebytes.as_ptr(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PlayReadyIndividualizationServiceRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyIndividualizationServiceRequest;{21f5a86b-008c-4611-ab2f-aaa6c69f0e24})");
}
unsafe impl ::windows_core::Interface for PlayReadyIndividualizationServiceRequest {
    type Vtable = IPlayReadyIndividualizationServiceRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PlayReadyIndividualizationServiceRequest {
    const IID: ::windows_core::GUID = <IPlayReadyIndividualizationServiceRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadyIndividualizationServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyIndividualizationServiceRequest";
}
::windows_core::imp::interface_hierarchy!(PlayReadyIndividualizationServiceRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IMediaProtectionServiceRequest> for PlayReadyIndividualizationServiceRequest {}
impl ::windows_core::CanTryInto<IPlayReadyServiceRequest> for PlayReadyIndividualizationServiceRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayReadyLicense(::windows_core::IUnknown);
impl PlayReadyLicense {
    pub fn FullyEvaluated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FullyEvaluated)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UsableForPlay(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UsableForPlay)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationDate(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationDate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExpireAfterFirstPlay(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExpireAfterFirstPlay)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DomainAccountID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DomainAccountID)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChainDepth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChainDepth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetKIDAtChainDepth(&self, chaindepth: u32) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetKIDAtChainDepth)(::windows_core::Interface::as_raw(this), chaindepth, &mut result__).from_abi(result__)
        }
    }
    pub fn SecureStopId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SecureStopId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SecurityLevel(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SecurityLevel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InMemoryOnly(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InMemoryOnly)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExpiresInRealTime(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyLicense2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExpiresInRealTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PlayReadyLicense {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicense;{ee474c4e-fa3c-414d-a9f2-3ffc1ef832d4})");
}
unsafe impl ::windows_core::Interface for PlayReadyLicense {
    type Vtable = IPlayReadyLicense_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PlayReadyLicense {
    const IID: ::windows_core::GUID = <IPlayReadyLicense as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadyLicense {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicense";
}
::windows_core::imp::interface_hierarchy!(PlayReadyLicense, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPlayReadyLicense> for PlayReadyLicense {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayReadyLicenseAcquisitionServiceRequest(::windows_core::IUnknown);
impl PlayReadyLicenseAcquisitionServiceRequest {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayReadyLicenseAcquisitionServiceRequest, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ContentHeader(&self) -> ::windows_core::Result<PlayReadyContentHeader> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentHeader)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContentHeader<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PlayReadyContentHeader>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentHeader)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DomainServiceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DomainServiceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDomainServiceId(&self, value: ::windows_core::GUID) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDomainServiceId)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SessionId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyLicenseAcquisitionServiceRequest2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SessionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateLicenseIterable<P0>(&self, contentheader: P0, fullyevaluated: bool) -> ::windows_core::Result<PlayReadyLicenseIterable>
    where
        P0: ::windows_core::IntoParam<PlayReadyContentHeader>,
    {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyLicenseAcquisitionServiceRequest3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateLicenseIterable)(::windows_core::Interface::as_raw(this), contentheader.into_param().abi(), fullyevaluated, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChallengeCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetChallengeCustomData(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetChallengeCustomData)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BeginServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows_core::Result<IPlayReadyServiceRequest> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows_core::Result<PlayReadySoapMessage> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(::windows_core::Interface::as_raw(this), responsebytes.len().try_into().unwrap(), responsebytes.as_ptr(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PlayReadyLicenseAcquisitionServiceRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicenseAcquisitionServiceRequest;{5d85ff45-3e9f-4f48-93e1-9530c8d58c3e})");
}
unsafe impl ::windows_core::Interface for PlayReadyLicenseAcquisitionServiceRequest {
    type Vtable = IPlayReadyLicenseAcquisitionServiceRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PlayReadyLicenseAcquisitionServiceRequest {
    const IID: ::windows_core::GUID = <IPlayReadyLicenseAcquisitionServiceRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadyLicenseAcquisitionServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseAcquisitionServiceRequest";
}
::windows_core::imp::interface_hierarchy!(PlayReadyLicenseAcquisitionServiceRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IMediaProtectionServiceRequest> for PlayReadyLicenseAcquisitionServiceRequest {}
impl ::windows_core::CanTryInto<IPlayReadyLicenseAcquisitionServiceRequest> for PlayReadyLicenseAcquisitionServiceRequest {}
impl ::windows_core::CanTryInto<IPlayReadyServiceRequest> for PlayReadyLicenseAcquisitionServiceRequest {}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayReadyLicenseIterable(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadyLicenseIterable {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayReadyLicenseIterable, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateInstance<P0>(contentheader: P0, fullyevaluated: bool) -> ::windows_core::Result<PlayReadyLicenseIterable>
    where
        P0: ::windows_core::IntoParam<PlayReadyContentHeader>,
    {
        Self::IPlayReadyLicenseIterableFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), contentheader.into_param().abi(), fullyevaluated, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlayReadyLicenseIterableFactory<R, F: FnOnce(&IPlayReadyLicenseIterableFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayReadyLicenseIterable, IPlayReadyLicenseIterableFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for PlayReadyLicenseIterable {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicenseIterable;pinterface({faa585ea-6214-4217-afda-7f46de5869b3};{ee474c4e-fa3c-414d-a9f2-3ffc1ef832d4}))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for PlayReadyLicenseIterable {
    type Vtable = super::super::super::Foundation::Collections::IIterable_Vtbl<IPlayReadyLicense>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for PlayReadyLicenseIterable {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for PlayReadyLicenseIterable {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseIterable";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for PlayReadyLicenseIterable {
    type Item = IPlayReadyLicense;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &PlayReadyLicenseIterable {
    type Item = IPlayReadyLicense;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(PlayReadyLicenseIterable, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterable<IPlayReadyLicense>> for PlayReadyLicenseIterable {}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayReadyLicenseIterator(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadyLicenseIterator {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Current(&self) -> ::windows_core::Result<IPlayReadyLicense> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasCurrent(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasCurrent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MoveNext(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MoveNext)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, items: &mut [::core::option::Option<IPlayReadyLicense>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for PlayReadyLicenseIterator {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicenseIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};{ee474c4e-fa3c-414d-a9f2-3ffc1ef832d4}))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for PlayReadyLicenseIterator {
    type Vtable = super::super::super::Foundation::Collections::IIterator_Vtbl<IPlayReadyLicense>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for PlayReadyLicenseIterator {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for PlayReadyLicenseIterator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseIterator";
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(PlayReadyLicenseIterator, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterator<IPlayReadyLicense>> for PlayReadyLicenseIterator {}
pub struct PlayReadyLicenseManagement;
impl PlayReadyLicenseManagement {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteLicenses<P0>(contentheader: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<PlayReadyContentHeader>,
    {
        Self::IPlayReadyLicenseManagement(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeleteLicenses)(::windows_core::Interface::as_raw(this), contentheader.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlayReadyLicenseManagement<R, F: FnOnce(&IPlayReadyLicenseManagement) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayReadyLicenseManagement, IPlayReadyLicenseManagement> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for PlayReadyLicenseManagement {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseManagement";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayReadyLicenseSession(::windows_core::IUnknown);
impl PlayReadyLicenseSession {
    pub fn CreateLAServiceRequest(&self) -> ::windows_core::Result<IPlayReadyLicenseAcquisitionServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateLAServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConfigureMediaProtectionManager<P0>(&self, mpm: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::MediaProtectionManager>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ConfigureMediaProtectionManager)(::windows_core::Interface::as_raw(this), mpm.into_param().abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateLicenseIterable<P0>(&self, contentheader: P0, fullyevaluated: bool) -> ::windows_core::Result<PlayReadyLicenseIterable>
    where
        P0: ::windows_core::IntoParam<PlayReadyContentHeader>,
    {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyLicenseSession2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateLicenseIterable)(::windows_core::Interface::as_raw(this), contentheader.into_param().abi(), fullyevaluated, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateInstance<P0>(configuration: P0) -> ::windows_core::Result<PlayReadyLicenseSession>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IPropertySet>,
    {
        Self::IPlayReadyLicenseSessionFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), configuration.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlayReadyLicenseSessionFactory<R, F: FnOnce(&IPlayReadyLicenseSessionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayReadyLicenseSession, IPlayReadyLicenseSessionFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PlayReadyLicenseSession {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyLicenseSession;{a1723a39-87fa-4fdd-abbb-a9720e845259})");
}
unsafe impl ::windows_core::Interface for PlayReadyLicenseSession {
    type Vtable = IPlayReadyLicenseSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PlayReadyLicenseSession {
    const IID: ::windows_core::GUID = <IPlayReadyLicenseSession as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadyLicenseSession {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyLicenseSession";
}
::windows_core::imp::interface_hierarchy!(PlayReadyLicenseSession, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IPlayReadyLicenseSession> for PlayReadyLicenseSession {}
impl ::windows_core::CanTryInto<IPlayReadyLicenseSession2> for PlayReadyLicenseSession {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayReadyMeteringReportServiceRequest(::windows_core::IUnknown);
impl PlayReadyMeteringReportServiceRequest {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayReadyMeteringReportServiceRequest, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MeteringCertificate(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).MeteringCertificate)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn SetMeteringCertificate(&self, meteringcertbytes: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMeteringCertificate)(::windows_core::Interface::as_raw(this), meteringcertbytes.len().try_into().unwrap(), meteringcertbytes.as_ptr()).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChallengeCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetChallengeCustomData(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetChallengeCustomData)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BeginServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows_core::Result<IPlayReadyServiceRequest> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows_core::Result<PlayReadySoapMessage> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(::windows_core::Interface::as_raw(this), responsebytes.len().try_into().unwrap(), responsebytes.as_ptr(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PlayReadyMeteringReportServiceRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyMeteringReportServiceRequest;{c12b231c-0ecd-4f11-a185-1e24a4a67fb7})");
}
unsafe impl ::windows_core::Interface for PlayReadyMeteringReportServiceRequest {
    type Vtable = IPlayReadyMeteringReportServiceRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PlayReadyMeteringReportServiceRequest {
    const IID: ::windows_core::GUID = <IPlayReadyMeteringReportServiceRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadyMeteringReportServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyMeteringReportServiceRequest";
}
::windows_core::imp::interface_hierarchy!(PlayReadyMeteringReportServiceRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IMediaProtectionServiceRequest> for PlayReadyMeteringReportServiceRequest {}
impl ::windows_core::CanTryInto<IPlayReadyServiceRequest> for PlayReadyMeteringReportServiceRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayReadyRevocationServiceRequest(::windows_core::IUnknown);
impl PlayReadyRevocationServiceRequest {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayReadyRevocationServiceRequest, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChallengeCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetChallengeCustomData(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetChallengeCustomData)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BeginServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows_core::Result<IPlayReadyServiceRequest> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows_core::Result<PlayReadySoapMessage> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(::windows_core::Interface::as_raw(this), responsebytes.len().try_into().unwrap(), responsebytes.as_ptr(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PlayReadyRevocationServiceRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadyRevocationServiceRequest;{543d66ac-faf0-4560-84a5-0e4acec939e4})");
}
unsafe impl ::windows_core::Interface for PlayReadyRevocationServiceRequest {
    type Vtable = IPlayReadyRevocationServiceRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PlayReadyRevocationServiceRequest {
    const IID: ::windows_core::GUID = <IPlayReadyRevocationServiceRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadyRevocationServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyRevocationServiceRequest";
}
::windows_core::imp::interface_hierarchy!(PlayReadyRevocationServiceRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IMediaProtectionServiceRequest> for PlayReadyRevocationServiceRequest {}
impl ::windows_core::CanTryInto<IPlayReadyServiceRequest> for PlayReadyRevocationServiceRequest {}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayReadySecureStopIterable(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadySecureStopIterable {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateInstance(publishercertbytes: &[u8]) -> ::windows_core::Result<PlayReadySecureStopIterable> {
        Self::IPlayReadySecureStopIterableFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), publishercertbytes.len().try_into().unwrap(), publishercertbytes.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlayReadySecureStopIterableFactory<R, F: FnOnce(&IPlayReadySecureStopIterableFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayReadySecureStopIterable, IPlayReadySecureStopIterableFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for PlayReadySecureStopIterable {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadySecureStopIterable;pinterface({faa585ea-6214-4217-afda-7f46de5869b3};{b5501ee5-01bf-4401-9677-05630a6a4cc8}))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for PlayReadySecureStopIterable {
    type Vtable = super::super::super::Foundation::Collections::IIterable_Vtbl<IPlayReadySecureStopServiceRequest>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for PlayReadySecureStopIterable {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for PlayReadySecureStopIterable {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySecureStopIterable";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for PlayReadySecureStopIterable {
    type Item = IPlayReadySecureStopServiceRequest;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &PlayReadySecureStopIterable {
    type Item = IPlayReadySecureStopServiceRequest;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(PlayReadySecureStopIterable, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterable<IPlayReadySecureStopServiceRequest>> for PlayReadySecureStopIterable {}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayReadySecureStopIterator(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl PlayReadySecureStopIterator {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Current(&self) -> ::windows_core::Result<IPlayReadySecureStopServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasCurrent(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasCurrent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MoveNext(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MoveNext)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, items: &mut [::core::option::Option<IPlayReadySecureStopServiceRequest>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for PlayReadySecureStopIterator {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadySecureStopIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};{b5501ee5-01bf-4401-9677-05630a6a4cc8}))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for PlayReadySecureStopIterator {
    type Vtable = super::super::super::Foundation::Collections::IIterator_Vtbl<IPlayReadySecureStopServiceRequest>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for PlayReadySecureStopIterator {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for PlayReadySecureStopIterator {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySecureStopIterator";
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(PlayReadySecureStopIterator, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterator<IPlayReadySecureStopServiceRequest>> for PlayReadySecureStopIterator {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayReadySecureStopServiceRequest(::windows_core::IUnknown);
impl PlayReadySecureStopServiceRequest {
    pub fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaProtectionServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SessionID(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SessionID)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateTime(&self) -> ::windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Stopped(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PublisherCertificate(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).PublisherCertificate)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn CreateInstance(publishercertbytes: &[u8]) -> ::windows_core::Result<PlayReadySecureStopServiceRequest> {
        Self::IPlayReadySecureStopServiceRequestFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), publishercertbytes.len().try_into().unwrap(), publishercertbytes.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateInstanceFromSessionID(sessionid: ::windows_core::GUID, publishercertbytes: &[u8]) -> ::windows_core::Result<PlayReadySecureStopServiceRequest> {
        Self::IPlayReadySecureStopServiceRequestFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceFromSessionID)(::windows_core::Interface::as_raw(this), sessionid, publishercertbytes.len().try_into().unwrap(), publishercertbytes.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ResponseCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResponseCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChallengeCustomData(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChallengeCustomData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetChallengeCustomData(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetChallengeCustomData)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn BeginServiceRequest(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BeginServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NextServiceRequest(&self) -> ::windows_core::Result<IPlayReadyServiceRequest> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NextServiceRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GenerateManualEnablingChallenge(&self) -> ::windows_core::Result<PlayReadySoapMessage> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GenerateManualEnablingChallenge)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProcessManualEnablingResponse(&self, responsebytes: &[u8]) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = &::windows_core::ComInterface::cast::<IPlayReadyServiceRequest>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProcessManualEnablingResponse)(::windows_core::Interface::as_raw(this), responsebytes.len().try_into().unwrap(), responsebytes.as_ptr(), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IPlayReadySecureStopServiceRequestFactory<R, F: FnOnce(&IPlayReadySecureStopServiceRequestFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayReadySecureStopServiceRequest, IPlayReadySecureStopServiceRequestFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PlayReadySecureStopServiceRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadySecureStopServiceRequest;{b5501ee5-01bf-4401-9677-05630a6a4cc8})");
}
unsafe impl ::windows_core::Interface for PlayReadySecureStopServiceRequest {
    type Vtable = IPlayReadySecureStopServiceRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PlayReadySecureStopServiceRequest {
    const IID: ::windows_core::GUID = <IPlayReadySecureStopServiceRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadySecureStopServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySecureStopServiceRequest";
}
::windows_core::imp::interface_hierarchy!(PlayReadySecureStopServiceRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IMediaProtectionServiceRequest> for PlayReadySecureStopServiceRequest {}
impl ::windows_core::CanTryInto<IPlayReadySecureStopServiceRequest> for PlayReadySecureStopServiceRequest {}
impl ::windows_core::CanTryInto<IPlayReadyServiceRequest> for PlayReadySecureStopServiceRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PlayReadySoapMessage(::windows_core::IUnknown);
impl PlayReadySoapMessage {
    pub fn GetMessageBody(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).GetMessageBody)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MessageHeaders(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MessageHeaders)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PlayReadySoapMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.PlayReady.PlayReadySoapMessage;{b659fcb5-ce41-41ba-8a0d-61df5fffa139})");
}
unsafe impl ::windows_core::Interface for PlayReadySoapMessage {
    type Vtable = IPlayReadySoapMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PlayReadySoapMessage {
    const IID: ::windows_core::GUID = <IPlayReadySoapMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PlayReadySoapMessage {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadySoapMessage";
}
::windows_core::imp::interface_hierarchy!(PlayReadySoapMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
pub struct PlayReadyStatics;
impl PlayReadyStatics {
    pub fn DomainJoinServiceRequestType() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DomainJoinServiceRequestType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn DomainLeaveServiceRequestType() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DomainLeaveServiceRequestType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IndividualizationServiceRequestType() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndividualizationServiceRequestType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn LicenseAcquirerServiceRequestType() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LicenseAcquirerServiceRequestType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MeteringReportServiceRequestType() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MeteringReportServiceRequestType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn RevocationServiceRequestType() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RevocationServiceRequestType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MediaProtectionSystemId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaProtectionSystemId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn PlayReadySecurityVersion() -> ::windows_core::Result<u32> {
        Self::IPlayReadyStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlayReadySecurityVersion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn PlayReadyCertificateSecurityLevel() -> ::windows_core::Result<u32> {
        Self::IPlayReadyStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlayReadyCertificateSecurityLevel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SecureStopServiceRequestType() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IPlayReadyStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SecureStopServiceRequestType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CheckSupportedHardware(hwdrmfeature: PlayReadyHardwareDRMFeatures) -> ::windows_core::Result<bool> {
        Self::IPlayReadyStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CheckSupportedHardware)(::windows_core::Interface::as_raw(this), hwdrmfeature, &mut result__).from_abi(result__)
        })
    }
    pub fn InputTrustAuthorityToCreate() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IPlayReadyStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InputTrustAuthorityToCreate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ProtectionSystemId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IPlayReadyStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionSystemId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn HardwareDRMDisabledAtTime() -> ::windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        Self::IPlayReadyStatics5(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HardwareDRMDisabledAtTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn HardwareDRMDisabledUntilTime() -> ::windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        Self::IPlayReadyStatics5(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HardwareDRMDisabledUntilTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ResetHardwareDRMDisabled() -> ::windows_core::Result<()> {
        Self::IPlayReadyStatics5(|this| unsafe { (::windows_core::Interface::vtable(this).ResetHardwareDRMDisabled)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[doc(hidden)]
    pub fn IPlayReadyStatics<R, F: FnOnce(&IPlayReadyStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayReadyStatics, IPlayReadyStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPlayReadyStatics2<R, F: FnOnce(&IPlayReadyStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayReadyStatics, IPlayReadyStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPlayReadyStatics3<R, F: FnOnce(&IPlayReadyStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayReadyStatics, IPlayReadyStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPlayReadyStatics4<R, F: FnOnce(&IPlayReadyStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayReadyStatics, IPlayReadyStatics4> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPlayReadyStatics5<R, F: FnOnce(&IPlayReadyStatics5) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PlayReadyStatics, IPlayReadyStatics5> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for PlayReadyStatics {
    const NAME: &'static str = "Windows.Media.Protection.PlayReady.PlayReadyStatics";
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDCertificateFeature(pub i32);
#[cfg(feature = "deprecated")]
impl NDCertificateFeature {
    pub const Transmitter: Self = Self(1i32);
    pub const Receiver: Self = Self(2i32);
    pub const SharedCertificate: Self = Self(3i32);
    pub const SecureClock: Self = Self(4i32);
    pub const AntiRollBackClock: Self = Self(5i32);
    pub const CRLS: Self = Self(9i32);
    pub const PlayReady3Features: Self = Self(13i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDCertificateFeature {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDCertificateFeature {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for NDCertificateFeature {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::TypeKind for NDCertificateFeature {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDCertificateFeature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDCertificateFeature").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for NDCertificateFeature {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDCertificateFeature;i4)");
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDCertificatePlatformID(pub i32);
#[cfg(feature = "deprecated")]
impl NDCertificatePlatformID {
    pub const Windows: Self = Self(0i32);
    pub const OSX: Self = Self(1i32);
    pub const WindowsOnARM: Self = Self(2i32);
    pub const WindowsMobile7: Self = Self(5i32);
    pub const iOSOnARM: Self = Self(6i32);
    pub const XBoxOnPPC: Self = Self(7i32);
    pub const WindowsPhone8OnARM: Self = Self(8i32);
    pub const WindowsPhone8OnX86: Self = Self(9i32);
    pub const XboxOne: Self = Self(10i32);
    pub const AndroidOnARM: Self = Self(11i32);
    pub const WindowsPhone81OnARM: Self = Self(12i32);
    pub const WindowsPhone81OnX86: Self = Self(13i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDCertificatePlatformID {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDCertificatePlatformID {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for NDCertificatePlatformID {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::TypeKind for NDCertificatePlatformID {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDCertificatePlatformID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDCertificatePlatformID").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for NDCertificatePlatformID {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDCertificatePlatformID;i4)");
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDCertificateType(pub i32);
#[cfg(feature = "deprecated")]
impl NDCertificateType {
    pub const Unknown: Self = Self(0i32);
    pub const PC: Self = Self(1i32);
    pub const Device: Self = Self(2i32);
    pub const Domain: Self = Self(3i32);
    pub const Issuer: Self = Self(4i32);
    pub const CrlSigner: Self = Self(5i32);
    pub const Service: Self = Self(6i32);
    pub const Silverlight: Self = Self(7i32);
    pub const Application: Self = Self(8i32);
    pub const Metering: Self = Self(9i32);
    pub const KeyFileSigner: Self = Self(10i32);
    pub const Server: Self = Self(11i32);
    pub const LicenseSigner: Self = Self(12i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDCertificateType {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDCertificateType {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for NDCertificateType {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::TypeKind for NDCertificateType {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDCertificateType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDCertificateType").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for NDCertificateType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDCertificateType;i4)");
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDClosedCaptionFormat(pub i32);
#[cfg(feature = "deprecated")]
impl NDClosedCaptionFormat {
    pub const ATSC: Self = Self(0i32);
    pub const SCTE20: Self = Self(1i32);
    pub const Unknown: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDClosedCaptionFormat {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDClosedCaptionFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for NDClosedCaptionFormat {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::TypeKind for NDClosedCaptionFormat {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDClosedCaptionFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDClosedCaptionFormat").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for NDClosedCaptionFormat {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDClosedCaptionFormat;i4)");
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDContentIDType(pub i32);
#[cfg(feature = "deprecated")]
impl NDContentIDType {
    pub const KeyID: Self = Self(1i32);
    pub const PlayReadyObject: Self = Self(2i32);
    pub const Custom: Self = Self(3i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDContentIDType {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDContentIDType {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for NDContentIDType {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::TypeKind for NDContentIDType {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDContentIDType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDContentIDType").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for NDContentIDType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDContentIDType;i4)");
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDMediaStreamType(pub i32);
#[cfg(feature = "deprecated")]
impl NDMediaStreamType {
    pub const Audio: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDMediaStreamType {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDMediaStreamType {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for NDMediaStreamType {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::TypeKind for NDMediaStreamType {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDMediaStreamType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDMediaStreamType").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for NDMediaStreamType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDMediaStreamType;i4)");
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDProximityDetectionType(pub i32);
#[cfg(feature = "deprecated")]
impl NDProximityDetectionType {
    pub const UDP: Self = Self(1i32);
    pub const TCP: Self = Self(2i32);
    pub const TransportAgnostic: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDProximityDetectionType {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDProximityDetectionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for NDProximityDetectionType {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::TypeKind for NDProximityDetectionType {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDProximityDetectionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDProximityDetectionType").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for NDProximityDetectionType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDProximityDetectionType;i4)");
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDStartAsyncOptions(pub i32);
#[cfg(feature = "deprecated")]
impl NDStartAsyncOptions {
    pub const MutualAuthentication: Self = Self(1i32);
    pub const WaitForLicenseDescriptor: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for NDStartAsyncOptions {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for NDStartAsyncOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for NDStartAsyncOptions {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::TypeKind for NDStartAsyncOptions {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for NDStartAsyncOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDStartAsyncOptions").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for NDStartAsyncOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.NDStartAsyncOptions;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlayReadyDecryptorSetup(pub i32);
impl PlayReadyDecryptorSetup {
    pub const Uninitialized: Self = Self(0i32);
    pub const OnDemand: Self = Self(1i32);
}
impl ::core::marker::Copy for PlayReadyDecryptorSetup {}
impl ::core::clone::Clone for PlayReadyDecryptorSetup {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlayReadyDecryptorSetup {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PlayReadyDecryptorSetup {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PlayReadyDecryptorSetup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyDecryptorSetup").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PlayReadyDecryptorSetup {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyDecryptorSetup;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlayReadyEncryptionAlgorithm(pub i32);
impl PlayReadyEncryptionAlgorithm {
    pub const Unprotected: Self = Self(0i32);
    pub const Aes128Ctr: Self = Self(1i32);
    pub const Cocktail: Self = Self(4i32);
    pub const Aes128Cbc: Self = Self(5i32);
    pub const Unspecified: Self = Self(65535i32);
    pub const Uninitialized: Self = Self(2147483647i32);
}
impl ::core::marker::Copy for PlayReadyEncryptionAlgorithm {}
impl ::core::clone::Clone for PlayReadyEncryptionAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlayReadyEncryptionAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PlayReadyEncryptionAlgorithm {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PlayReadyEncryptionAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyEncryptionAlgorithm").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PlayReadyEncryptionAlgorithm {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyEncryptionAlgorithm;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlayReadyHardwareDRMFeatures(pub i32);
impl PlayReadyHardwareDRMFeatures {
    pub const HardwareDRM: Self = Self(1i32);
    pub const HEVC: Self = Self(2i32);
    pub const Aes128Cbc: Self = Self(3i32);
}
impl ::core::marker::Copy for PlayReadyHardwareDRMFeatures {}
impl ::core::clone::Clone for PlayReadyHardwareDRMFeatures {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlayReadyHardwareDRMFeatures {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PlayReadyHardwareDRMFeatures {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PlayReadyHardwareDRMFeatures {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyHardwareDRMFeatures").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PlayReadyHardwareDRMFeatures {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyHardwareDRMFeatures;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlayReadyITADataFormat(pub i32);
impl PlayReadyITADataFormat {
    pub const SerializedProperties: Self = Self(0i32);
    pub const SerializedProperties_WithContentProtectionWrapper: Self = Self(1i32);
}
impl ::core::marker::Copy for PlayReadyITADataFormat {}
impl ::core::clone::Clone for PlayReadyITADataFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlayReadyITADataFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PlayReadyITADataFormat {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PlayReadyITADataFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayReadyITADataFormat").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PlayReadyITADataFormat {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.PlayReady.PlayReadyITADataFormat;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
