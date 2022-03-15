#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlatformTelemetryClientStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPlatformTelemetryClientStatics {
    type Vtable = IPlatformTelemetryClientStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bf3f25d_d5c3_4eea_8dbe_9c8dbb0d9d8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformTelemetryClientStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RegisterWithSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, settings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlatformTelemetryRegistrationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPlatformTelemetryRegistrationResult {
    type Vtable = IPlatformTelemetryRegistrationResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d8518ab_2292_49bd_a15a_3d71d2145112);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformTelemetryRegistrationResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlatformTelemetryRegistrationStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlatformTelemetryRegistrationSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPlatformTelemetryRegistrationSettings {
    type Vtable = IPlatformTelemetryRegistrationSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x819a8582_ca19_415e_bb79_9c224bfa3a73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformTelemetryRegistrationSettings_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub StorageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetStorageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub UploadQuotaSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetUploadQuotaSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
pub struct PlatformTelemetryClient {}
impl PlatformTelemetryClient {
    #[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
    pub fn Register<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(id: Param0) -> ::windows::core::Result<PlatformTelemetryRegistrationResult> {
        Self::IPlatformTelemetryClientStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Register)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<PlatformTelemetryRegistrationResult>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
    pub fn RegisterWithSettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, PlatformTelemetryRegistrationSettings>>(id: Param0, settings: Param1) -> ::windows::core::Result<PlatformTelemetryRegistrationResult> {
        Self::IPlatformTelemetryClientStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RegisterWithSettings)(::core::mem::transmute_copy(this), id.into_param().abi(), settings.into_param().abi(), &mut result__).from_abi::<PlatformTelemetryRegistrationResult>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlatformTelemetryClientStatics<R, F: FnOnce(&IPlatformTelemetryClientStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlatformTelemetryClient, IPlatformTelemetryClientStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PlatformTelemetryClient {
    const NAME: &'static str = "Windows.System.Diagnostics.Telemetry.PlatformTelemetryClient";
}
#[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
#[repr(transparent)]
pub struct PlatformTelemetryRegistrationResult(::windows::core::IUnknown);
impl PlatformTelemetryRegistrationResult {
    #[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<PlatformTelemetryRegistrationStatus> {
        let this = self;
        unsafe {
            let mut result__: PlatformTelemetryRegistrationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlatformTelemetryRegistrationStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for PlatformTelemetryRegistrationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlatformTelemetryRegistrationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlatformTelemetryRegistrationResult {}
impl ::core::fmt::Debug for PlatformTelemetryRegistrationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlatformTelemetryRegistrationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlatformTelemetryRegistrationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationResult;{4d8518ab-2292-49bd-a15a-3d71d2145112})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PlatformTelemetryRegistrationResult {
    type Vtable = IPlatformTelemetryRegistrationResult_Vtbl;
    const IID: ::windows::core::GUID = <IPlatformTelemetryRegistrationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlatformTelemetryRegistrationResult {
    const NAME: &'static str = "Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationResult";
}
impl ::core::convert::From<PlatformTelemetryRegistrationResult> for ::windows::core::IUnknown {
    fn from(value: PlatformTelemetryRegistrationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlatformTelemetryRegistrationResult> for ::windows::core::IUnknown {
    fn from(value: &PlatformTelemetryRegistrationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlatformTelemetryRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlatformTelemetryRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlatformTelemetryRegistrationResult> for ::windows::core::IInspectable {
    fn from(value: PlatformTelemetryRegistrationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlatformTelemetryRegistrationResult> for ::windows::core::IInspectable {
    fn from(value: &PlatformTelemetryRegistrationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlatformTelemetryRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlatformTelemetryRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PlatformTelemetryRegistrationResult {}
unsafe impl ::core::marker::Sync for PlatformTelemetryRegistrationResult {}
#[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
#[repr(transparent)]
pub struct PlatformTelemetryRegistrationSettings(::windows::core::IUnknown);
impl PlatformTelemetryRegistrationSettings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlatformTelemetryRegistrationSettings, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
    pub fn StorageSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StorageSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
    pub fn SetStorageSize(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStorageSize)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
    pub fn UploadQuotaSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UploadQuotaSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
    pub fn SetUploadQuotaSize(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUploadQuotaSize)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for PlatformTelemetryRegistrationSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlatformTelemetryRegistrationSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlatformTelemetryRegistrationSettings {}
impl ::core::fmt::Debug for PlatformTelemetryRegistrationSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlatformTelemetryRegistrationSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlatformTelemetryRegistrationSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationSettings;{819a8582-ca19-415e-bb79-9c224bfa3a73})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PlatformTelemetryRegistrationSettings {
    type Vtable = IPlatformTelemetryRegistrationSettings_Vtbl;
    const IID: ::windows::core::GUID = <IPlatformTelemetryRegistrationSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlatformTelemetryRegistrationSettings {
    const NAME: &'static str = "Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationSettings";
}
impl ::core::convert::From<PlatformTelemetryRegistrationSettings> for ::windows::core::IUnknown {
    fn from(value: PlatformTelemetryRegistrationSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlatformTelemetryRegistrationSettings> for ::windows::core::IUnknown {
    fn from(value: &PlatformTelemetryRegistrationSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlatformTelemetryRegistrationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PlatformTelemetryRegistrationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlatformTelemetryRegistrationSettings> for ::windows::core::IInspectable {
    fn from(value: PlatformTelemetryRegistrationSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlatformTelemetryRegistrationSettings> for ::windows::core::IInspectable {
    fn from(value: &PlatformTelemetryRegistrationSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlatformTelemetryRegistrationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PlatformTelemetryRegistrationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PlatformTelemetryRegistrationSettings {}
unsafe impl ::core::marker::Sync for PlatformTelemetryRegistrationSettings {}
#[doc = "*Required features: `\"System_Diagnostics_Telemetry\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PlatformTelemetryRegistrationStatus(pub i32);
impl PlatformTelemetryRegistrationStatus {
    pub const Success: Self = Self(0i32);
    pub const SettingsOutOfRange: Self = Self(1i32);
    pub const UnknownFailure: Self = Self(2i32);
}
impl ::core::marker::Copy for PlatformTelemetryRegistrationStatus {}
impl ::core::clone::Clone for PlatformTelemetryRegistrationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlatformTelemetryRegistrationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PlatformTelemetryRegistrationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlatformTelemetryRegistrationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlatformTelemetryRegistrationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlatformTelemetryRegistrationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Diagnostics.Telemetry.PlatformTelemetryRegistrationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
