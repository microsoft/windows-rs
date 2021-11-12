#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlatformTelemetryClientStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlatformTelemetryClientStatics {
    type Vtable = IPlatformTelemetryClientStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bf3f25d_d5c3_4eea_8dbe_9c8dbb0d9d8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformTelemetryClientStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlatformTelemetryRegistrationResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlatformTelemetryRegistrationResult {
    type Vtable = IPlatformTelemetryRegistrationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d8518ab_2292_49bd_a15a_3d71d2145112);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformTelemetryRegistrationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PlatformTelemetryRegistrationStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPlatformTelemetryRegistrationSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPlatformTelemetryRegistrationSettings {
    type Vtable = IPlatformTelemetryRegistrationSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x819a8582_ca19_415e_bb79_9c224bfa3a73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformTelemetryRegistrationSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
);
pub struct PlatformTelemetryClient {}
impl PlatformTelemetryClient {
    pub fn Register<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(id: Param0) -> ::windows::core::Result<PlatformTelemetryRegistrationResult> {
        Self::IPlatformTelemetryClientStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<PlatformTelemetryRegistrationResult>(result__)
        })
    }
    pub fn RegisterWithSettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, PlatformTelemetryRegistrationSettings>>(id: Param0, settings: Param1) -> ::windows::core::Result<PlatformTelemetryRegistrationResult> {
        Self::IPlatformTelemetryClientStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), id.into_param().abi(), settings.into_param().abi(), &mut result__).from_abi::<PlatformTelemetryRegistrationResult>(result__)
        })
    }
    pub fn IPlatformTelemetryClientStatics<R, F: FnOnce(&IPlatformTelemetryClientStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlatformTelemetryClient, IPlatformTelemetryClientStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PlatformTelemetryClient {
    const NAME: &'static str = "Windows.System.Diagnostics.Telemetry.PlatformTelemetryClient";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlatformTelemetryRegistrationResult(pub ::windows::core::IInspectable);
impl PlatformTelemetryRegistrationResult {
    pub fn Status(&self) -> ::windows::core::Result<PlatformTelemetryRegistrationStatus> {
        let this = self;
        unsafe {
            let mut result__: PlatformTelemetryRegistrationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlatformTelemetryRegistrationStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PlatformTelemetryRegistrationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationResult;{4d8518ab-2292-49bd-a15a-3d71d2145112})");
}
unsafe impl ::windows::core::Interface for PlatformTelemetryRegistrationResult {
    type Vtable = IPlatformTelemetryRegistrationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d8518ab_2292_49bd_a15a_3d71d2145112);
}
impl ::windows::core::RuntimeName for PlatformTelemetryRegistrationResult {
    const NAME: &'static str = "Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationResult";
}
impl ::core::convert::From<PlatformTelemetryRegistrationResult> for ::windows::core::IUnknown {
    fn from(value: PlatformTelemetryRegistrationResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlatformTelemetryRegistrationResult> for ::windows::core::IUnknown {
    fn from(value: &PlatformTelemetryRegistrationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlatformTelemetryRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlatformTelemetryRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlatformTelemetryRegistrationResult> for ::windows::core::IInspectable {
    fn from(value: PlatformTelemetryRegistrationResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlatformTelemetryRegistrationResult> for ::windows::core::IInspectable {
    fn from(value: &PlatformTelemetryRegistrationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlatformTelemetryRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlatformTelemetryRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PlatformTelemetryRegistrationResult {}
unsafe impl ::core::marker::Sync for PlatformTelemetryRegistrationResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PlatformTelemetryRegistrationSettings(pub ::windows::core::IInspectable);
impl PlatformTelemetryRegistrationSettings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlatformTelemetryRegistrationSettings, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn StorageSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetStorageSize(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn UploadQuotaSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetUploadQuotaSize(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for PlatformTelemetryRegistrationSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationSettings;{819a8582-ca19-415e-bb79-9c224bfa3a73})");
}
unsafe impl ::windows::core::Interface for PlatformTelemetryRegistrationSettings {
    type Vtable = IPlatformTelemetryRegistrationSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x819a8582_ca19_415e_bb79_9c224bfa3a73);
}
impl ::windows::core::RuntimeName for PlatformTelemetryRegistrationSettings {
    const NAME: &'static str = "Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationSettings";
}
impl ::core::convert::From<PlatformTelemetryRegistrationSettings> for ::windows::core::IUnknown {
    fn from(value: PlatformTelemetryRegistrationSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PlatformTelemetryRegistrationSettings> for ::windows::core::IUnknown {
    fn from(value: &PlatformTelemetryRegistrationSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlatformTelemetryRegistrationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlatformTelemetryRegistrationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PlatformTelemetryRegistrationSettings> for ::windows::core::IInspectable {
    fn from(value: PlatformTelemetryRegistrationSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PlatformTelemetryRegistrationSettings> for ::windows::core::IInspectable {
    fn from(value: &PlatformTelemetryRegistrationSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlatformTelemetryRegistrationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlatformTelemetryRegistrationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PlatformTelemetryRegistrationSettings {}
unsafe impl ::core::marker::Sync for PlatformTelemetryRegistrationSettings {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlatformTelemetryRegistrationStatus(pub i32);
impl PlatformTelemetryRegistrationStatus {
    pub const Success: PlatformTelemetryRegistrationStatus = PlatformTelemetryRegistrationStatus(0i32);
    pub const SettingsOutOfRange: PlatformTelemetryRegistrationStatus = PlatformTelemetryRegistrationStatus(1i32);
    pub const UnknownFailure: PlatformTelemetryRegistrationStatus = PlatformTelemetryRegistrationStatus(2i32);
}
impl ::core::convert::From<i32> for PlatformTelemetryRegistrationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PlatformTelemetryRegistrationStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PlatformTelemetryRegistrationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationStatus;i4)");
}
impl ::windows::core::DefaultType for PlatformTelemetryRegistrationStatus {
    type DefaultType = Self;
}
