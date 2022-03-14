#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "System_Profile_SystemManufacturers")]
pub mod SystemManufacturers;
#[doc = "*Required features: `\"System_Profile\"`*"]
pub struct AnalyticsInfo {}
impl AnalyticsInfo {
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn VersionInfo() -> ::windows::core::Result<AnalyticsVersionInfo> {
        Self::IAnalyticsInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VersionInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AnalyticsVersionInfo>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn DeviceForm() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAnalyticsInfoStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceForm)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSystemPropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(attributenames: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>> {
        Self::IAnalyticsInfoStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSystemPropertiesAsync)(::core::mem::transmute_copy(this), attributenames.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAnalyticsInfoStatics<R, F: FnOnce(&IAnalyticsInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AnalyticsInfo, IAnalyticsInfoStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAnalyticsInfoStatics2<R, F: FnOnce(&IAnalyticsInfoStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AnalyticsInfo, IAnalyticsInfoStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for AnalyticsInfo {
    const NAME: &'static str = "Windows.System.Profile.AnalyticsInfo";
}
#[doc = "*Required features: `\"System_Profile\"`*"]
#[repr(transparent)]
pub struct AnalyticsVersionInfo(::windows::core::IUnknown);
impl AnalyticsVersionInfo {
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn DeviceFamily(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceFamily)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn DeviceFamilyVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceFamilyVersion)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn ProductName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAnalyticsVersionInfo2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProductName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AnalyticsVersionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AnalyticsVersionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnalyticsVersionInfo {}
impl ::core::fmt::Debug for AnalyticsVersionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnalyticsVersionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AnalyticsVersionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Profile.AnalyticsVersionInfo;{926130b8-9955-4c74-bdc1-7cd0decf9b03})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AnalyticsVersionInfo {
    type Vtable = IAnalyticsVersionInfo_Vtbl;
    const IID: ::windows::core::GUID = <IAnalyticsVersionInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AnalyticsVersionInfo {
    const NAME: &'static str = "Windows.System.Profile.AnalyticsVersionInfo";
}
impl ::core::convert::From<AnalyticsVersionInfo> for ::windows::core::IUnknown {
    fn from(value: AnalyticsVersionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnalyticsVersionInfo> for ::windows::core::IUnknown {
    fn from(value: &AnalyticsVersionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AnalyticsVersionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AnalyticsVersionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AnalyticsVersionInfo> for ::windows::core::IInspectable {
    fn from(value: AnalyticsVersionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnalyticsVersionInfo> for ::windows::core::IInspectable {
    fn from(value: &AnalyticsVersionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AnalyticsVersionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AnalyticsVersionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AnalyticsVersionInfo {}
unsafe impl ::core::marker::Sync for AnalyticsVersionInfo {}
#[doc = "*Required features: `\"System_Profile\"`*"]
pub struct AppApplicability {}
impl AppApplicability {
    #[doc = "*Required features: `\"System_Profile\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUnsupportedAppRequirements<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(capabilities: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UnsupportedAppRequirement>> {
        Self::IAppApplicabilityStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetUnsupportedAppRequirements)(::core::mem::transmute_copy(this), capabilities.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<UnsupportedAppRequirement>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppApplicabilityStatics<R, F: FnOnce(&IAppApplicabilityStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppApplicability, IAppApplicabilityStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for AppApplicability {
    const NAME: &'static str = "Windows.System.Profile.AppApplicability";
}
#[doc = "*Required features: `\"System_Profile\"`*"]
pub struct EducationSettings {}
impl EducationSettings {
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn IsEducationEnvironment() -> ::windows::core::Result<bool> {
        Self::IEducationSettingsStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEducationEnvironment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEducationSettingsStatics<R, F: FnOnce(&IEducationSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EducationSettings, IEducationSettingsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for EducationSettings {
    const NAME: &'static str = "Windows.System.Profile.EducationSettings";
}
#[doc = "*Required features: `\"System_Profile\"`*"]
pub struct HardwareIdentification {}
impl HardwareIdentification {
    #[doc = "*Required features: `\"System_Profile\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetPackageSpecificToken<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(nonce: Param0) -> ::windows::core::Result<HardwareToken> {
        Self::IHardwareIdentificationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPackageSpecificToken)(::core::mem::transmute_copy(this), nonce.into_param().abi(), &mut result__).from_abi::<HardwareToken>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHardwareIdentificationStatics<R, F: FnOnce(&IHardwareIdentificationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HardwareIdentification, IHardwareIdentificationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for HardwareIdentification {
    const NAME: &'static str = "Windows.System.Profile.HardwareIdentification";
}
#[doc = "*Required features: `\"System_Profile\"`*"]
#[repr(transparent)]
pub struct HardwareToken(::windows::core::IUnknown);
impl HardwareToken {
    #[doc = "*Required features: `\"System_Profile\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Id(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Profile\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Signature(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Signature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Profile\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Certificate(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Certificate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for HardwareToken {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HardwareToken {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HardwareToken {}
impl ::core::fmt::Debug for HardwareToken {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HardwareToken").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HardwareToken {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Profile.HardwareToken;{28f6d4c0-fb12-40a4-8167-7f4e03d2724c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HardwareToken {
    type Vtable = IHardwareToken_Vtbl;
    const IID: ::windows::core::GUID = <IHardwareToken as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HardwareToken {
    const NAME: &'static str = "Windows.System.Profile.HardwareToken";
}
impl ::core::convert::From<HardwareToken> for ::windows::core::IUnknown {
    fn from(value: HardwareToken) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HardwareToken> for ::windows::core::IUnknown {
    fn from(value: &HardwareToken) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HardwareToken {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HardwareToken {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HardwareToken> for ::windows::core::IInspectable {
    fn from(value: HardwareToken) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HardwareToken> for ::windows::core::IInspectable {
    fn from(value: &HardwareToken) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HardwareToken {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HardwareToken {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HardwareToken {}
unsafe impl ::core::marker::Sync for HardwareToken {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAnalyticsInfoStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAnalyticsInfoStatics {
    type Vtable = IAnalyticsInfoStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d5ee066_188d_5ba9_4387_acaeb0e7e305);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnalyticsInfoStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub VersionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DeviceForm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAnalyticsInfoStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAnalyticsInfoStatics2 {
    type Vtable = IAnalyticsInfoStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x101704ea_a7f9_46d2_ab94_016865afdb25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnalyticsInfoStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSystemPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributenames: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSystemPropertiesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAnalyticsVersionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAnalyticsVersionInfo {
    type Vtable = IAnalyticsVersionInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x926130b8_9955_4c74_bdc1_7cd0decf9b03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnalyticsVersionInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DeviceFamilyVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAnalyticsVersionInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAnalyticsVersionInfo2 {
    type Vtable = IAnalyticsVersionInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76e915b1_ff36_407c_9f57_160d3e540747);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnalyticsVersionInfo2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ProductName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppApplicabilityStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppApplicabilityStatics {
    type Vtable = IAppApplicabilityStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1664a082_0f38_5c99_83e4_48995970861c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppApplicabilityStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUnsupportedAppRequirements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilities: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUnsupportedAppRequirements: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEducationSettingsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEducationSettingsStatics {
    type Vtable = IEducationSettingsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc53f0ef_4d3e_4e13_9b23_505f4d091e92);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEducationSettingsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsEducationEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHardwareIdentificationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHardwareIdentificationStatics {
    type Vtable = IHardwareIdentificationStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x971260e0_f170_4a42_bd55_a900b212dae2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHardwareIdentificationStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub GetPackageSpecificToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nonce: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetPackageSpecificToken: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHardwareToken(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHardwareToken {
    type Vtable = IHardwareToken_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28f6d4c0_fb12_40a4_8167_7f4e03d2724c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHardwareToken_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Id: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Signature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Signature: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Certificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Certificate: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownRetailInfoPropertiesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKnownRetailInfoPropertiesStatics {
    type Vtable = IKnownRetailInfoPropertiesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99571178_500f_487e_8e75_29e551728712);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownRetailInfoPropertiesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RetailAccessCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ManufacturerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ModelName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayModelName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Price: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsFeatured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FormFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ScreenSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Weight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BatteryLifeDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ProcessorDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Memory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub StorageDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GraphicsDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FrontCameraDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RearCameraDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HasNfc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HasSdSlot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HasOpticalDrive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsOfficeInstalled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WindowsEdition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlatformDiagnosticsAndUsageDataSettingsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPlatformDiagnosticsAndUsageDataSettingsStatics {
    type Vtable = IPlatformDiagnosticsAndUsageDataSettingsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6e24c1b_7b1c_4b32_8c62_a66597ce723a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformDiagnosticsAndUsageDataSettingsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CollectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlatformDataCollectionLevel) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CollectionLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CollectionLevelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCollectionLevelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCollectionLevelChanged: usize,
    pub CanCollectDiagnostics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: PlatformDataCollectionLevel, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRetailInfoStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRetailInfoStatics {
    type Vtable = IRetailInfoStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0712c6b8_8b92_4f2a_8499_031f1798d6ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRetailInfoStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsDemoModeEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISharedModeSettingsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISharedModeSettingsStatics {
    type Vtable = ISharedModeSettingsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x893df40e_cad6_4d50_8c49_6fcfc03edb29);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedModeSettingsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISharedModeSettingsStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISharedModeSettingsStatics2 {
    type Vtable = ISharedModeSettingsStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x608988a4_ccf1_4ee8_a5e2_fd6a1d0cfac8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedModeSettingsStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ShouldAvoidLocalStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemIdentificationInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemIdentificationInfo {
    type Vtable = ISystemIdentificationInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c659e7d_c3c2_4d33_a2df_21bc41916eb3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemIdentificationInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Id: usize,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemIdentificationSource) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemIdentificationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemIdentificationStatics {
    type Vtable = ISystemIdentificationStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5581f42a_d3df_4d93_a37d_c41a616c6d01);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemIdentificationStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetSystemIdForPublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetSystemIdForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemSetupInfoStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemSetupInfoStatics {
    type Vtable = ISystemSetupInfoStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c9620a8_1d88_5e2d_a324_a543af4247ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemSetupInfoStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub OutOfBoxExperienceState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemOutOfBoxExperienceState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OutOfBoxExperienceStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OutOfBoxExperienceStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOutOfBoxExperienceStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOutOfBoxExperienceStateChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUnsupportedAppRequirement(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUnsupportedAppRequirement {
    type Vtable = IUnsupportedAppRequirement_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6182445c_894b_5cbc_8976_a98e0a9b998d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnsupportedAppRequirement_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Requirement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Reasons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnsupportedAppRequirementReasons) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowsIntegrityPolicyStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowsIntegrityPolicyStatics {
    type Vtable = IWindowsIntegrityPolicyStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d1d81db_8d63_4789_9ea5_ddcf65a94f3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsIntegrityPolicyStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsEnabledForTrial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDisableSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PolicyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PolicyChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePolicyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePolicyChanged: usize,
}
#[doc = "*Required features: `\"System_Profile\"`*"]
pub struct KnownRetailInfoProperties {}
impl KnownRetailInfoProperties {
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn RetailAccessCode() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RetailAccessCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn ManufacturerName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ManufacturerName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn ModelName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ModelName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn DisplayModelName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayModelName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn Price() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Price)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn IsFeatured() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsFeatured)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn FormFactor() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FormFactor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn ScreenSize() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScreenSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn Weight() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Weight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn DisplayDescription() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn BatteryLifeDescription() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BatteryLifeDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn ProcessorDescription() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProcessorDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn Memory() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Memory)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn StorageDescription() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StorageDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn GraphicsDescription() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GraphicsDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn FrontCameraDescription() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrontCameraDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn RearCameraDescription() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RearCameraDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn HasNfc() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasNfc)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn HasSdSlot() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasSdSlot)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn HasOpticalDrive() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasOpticalDrive)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn IsOfficeInstalled() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOfficeInstalled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn WindowsEdition() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WindowsEdition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownRetailInfoPropertiesStatics<R, F: FnOnce(&IKnownRetailInfoPropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownRetailInfoProperties, IKnownRetailInfoPropertiesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for KnownRetailInfoProperties {
    const NAME: &'static str = "Windows.System.Profile.KnownRetailInfoProperties";
}
#[doc = "*Required features: `\"System_Profile\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PlatformDataCollectionLevel(pub i32);
impl PlatformDataCollectionLevel {
    pub const Security: Self = Self(0i32);
    pub const Basic: Self = Self(1i32);
    pub const Enhanced: Self = Self(2i32);
    pub const Full: Self = Self(3i32);
}
impl ::core::marker::Copy for PlatformDataCollectionLevel {}
impl ::core::clone::Clone for PlatformDataCollectionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlatformDataCollectionLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PlatformDataCollectionLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlatformDataCollectionLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlatformDataCollectionLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlatformDataCollectionLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Profile.PlatformDataCollectionLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_Profile\"`*"]
pub struct PlatformDiagnosticsAndUsageDataSettings {}
impl PlatformDiagnosticsAndUsageDataSettings {
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn CollectionLevel() -> ::windows::core::Result<PlatformDataCollectionLevel> {
        Self::IPlatformDiagnosticsAndUsageDataSettingsStatics(|this| unsafe {
            let mut result__: PlatformDataCollectionLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CollectionLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlatformDataCollectionLevel>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CollectionLevelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPlatformDiagnosticsAndUsageDataSettingsStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CollectionLevelChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCollectionLevelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IPlatformDiagnosticsAndUsageDataSettingsStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveCollectionLevelChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn CanCollectDiagnostics(level: PlatformDataCollectionLevel) -> ::windows::core::Result<bool> {
        Self::IPlatformDiagnosticsAndUsageDataSettingsStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanCollectDiagnostics)(::core::mem::transmute_copy(this), level, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlatformDiagnosticsAndUsageDataSettingsStatics<R, F: FnOnce(&IPlatformDiagnosticsAndUsageDataSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlatformDiagnosticsAndUsageDataSettings, IPlatformDiagnosticsAndUsageDataSettingsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PlatformDiagnosticsAndUsageDataSettings {
    const NAME: &'static str = "Windows.System.Profile.PlatformDiagnosticsAndUsageDataSettings";
}
#[doc = "*Required features: `\"System_Profile\"`*"]
pub struct RetailInfo {}
impl RetailInfo {
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn IsDemoModeEnabled() -> ::windows::core::Result<bool> {
        Self::IRetailInfoStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDemoModeEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties() -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        Self::IRetailInfoStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRetailInfoStatics<R, F: FnOnce(&IRetailInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RetailInfo, IRetailInfoStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for RetailInfo {
    const NAME: &'static str = "Windows.System.Profile.RetailInfo";
}
#[doc = "*Required features: `\"System_Profile\"`*"]
pub struct SharedModeSettings {}
impl SharedModeSettings {
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn IsEnabled() -> ::windows::core::Result<bool> {
        Self::ISharedModeSettingsStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn ShouldAvoidLocalStorage() -> ::windows::core::Result<bool> {
        Self::ISharedModeSettingsStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShouldAvoidLocalStorage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISharedModeSettingsStatics<R, F: FnOnce(&ISharedModeSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SharedModeSettings, ISharedModeSettingsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ISharedModeSettingsStatics2<R, F: FnOnce(&ISharedModeSettingsStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SharedModeSettings, ISharedModeSettingsStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for SharedModeSettings {
    const NAME: &'static str = "Windows.System.Profile.SharedModeSettings";
}
#[doc = "*Required features: `\"System_Profile\"`*"]
pub struct SystemIdentification {}
impl SystemIdentification {
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn GetSystemIdForPublisher() -> ::windows::core::Result<SystemIdentificationInfo> {
        Self::ISystemIdentificationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSystemIdForPublisher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemIdentificationInfo>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn GetSystemIdForUser<'a, Param0: ::windows::core::IntoParam<'a, super::User>>(user: Param0) -> ::windows::core::Result<SystemIdentificationInfo> {
        Self::ISystemIdentificationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSystemIdForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<SystemIdentificationInfo>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISystemIdentificationStatics<R, F: FnOnce(&ISystemIdentificationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SystemIdentification, ISystemIdentificationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for SystemIdentification {
    const NAME: &'static str = "Windows.System.Profile.SystemIdentification";
}
#[doc = "*Required features: `\"System_Profile\"`*"]
#[repr(transparent)]
pub struct SystemIdentificationInfo(::windows::core::IUnknown);
impl SystemIdentificationInfo {
    #[doc = "*Required features: `\"System_Profile\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Id(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn Source(&self) -> ::windows::core::Result<SystemIdentificationSource> {
        let this = self;
        unsafe {
            let mut result__: SystemIdentificationSource = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Source)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemIdentificationSource>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemIdentificationInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemIdentificationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemIdentificationInfo {}
impl ::core::fmt::Debug for SystemIdentificationInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemIdentificationInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemIdentificationInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Profile.SystemIdentificationInfo;{0c659e7d-c3c2-4d33-a2df-21bc41916eb3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SystemIdentificationInfo {
    type Vtable = ISystemIdentificationInfo_Vtbl;
    const IID: ::windows::core::GUID = <ISystemIdentificationInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SystemIdentificationInfo {
    const NAME: &'static str = "Windows.System.Profile.SystemIdentificationInfo";
}
impl ::core::convert::From<SystemIdentificationInfo> for ::windows::core::IUnknown {
    fn from(value: SystemIdentificationInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemIdentificationInfo> for ::windows::core::IUnknown {
    fn from(value: &SystemIdentificationInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SystemIdentificationInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SystemIdentificationInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemIdentificationInfo> for ::windows::core::IInspectable {
    fn from(value: SystemIdentificationInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemIdentificationInfo> for ::windows::core::IInspectable {
    fn from(value: &SystemIdentificationInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SystemIdentificationInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SystemIdentificationInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemIdentificationInfo {}
unsafe impl ::core::marker::Sync for SystemIdentificationInfo {}
#[doc = "*Required features: `\"System_Profile\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SystemIdentificationSource(pub i32);
impl SystemIdentificationSource {
    pub const None: Self = Self(0i32);
    pub const Tpm: Self = Self(1i32);
    pub const Uefi: Self = Self(2i32);
    pub const Registry: Self = Self(3i32);
}
impl ::core::marker::Copy for SystemIdentificationSource {}
impl ::core::clone::Clone for SystemIdentificationSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemIdentificationSource {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SystemIdentificationSource {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemIdentificationSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemIdentificationSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemIdentificationSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Profile.SystemIdentificationSource;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_Profile\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SystemOutOfBoxExperienceState(pub i32);
impl SystemOutOfBoxExperienceState {
    pub const NotStarted: Self = Self(0i32);
    pub const InProgress: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
}
impl ::core::marker::Copy for SystemOutOfBoxExperienceState {}
impl ::core::clone::Clone for SystemOutOfBoxExperienceState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemOutOfBoxExperienceState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SystemOutOfBoxExperienceState {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemOutOfBoxExperienceState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemOutOfBoxExperienceState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SystemOutOfBoxExperienceState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Profile.SystemOutOfBoxExperienceState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_Profile\"`*"]
pub struct SystemSetupInfo {}
impl SystemSetupInfo {
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn OutOfBoxExperienceState() -> ::windows::core::Result<SystemOutOfBoxExperienceState> {
        Self::ISystemSetupInfoStatics(|this| unsafe {
            let mut result__: SystemOutOfBoxExperienceState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutOfBoxExperienceState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SystemOutOfBoxExperienceState>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OutOfBoxExperienceStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ISystemSetupInfoStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutOfBoxExperienceStateChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOutOfBoxExperienceStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::ISystemSetupInfoStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveOutOfBoxExperienceStateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn ISystemSetupInfoStatics<R, F: FnOnce(&ISystemSetupInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SystemSetupInfo, ISystemSetupInfoStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for SystemSetupInfo {
    const NAME: &'static str = "Windows.System.Profile.SystemSetupInfo";
}
#[doc = "*Required features: `\"System_Profile\"`*"]
#[repr(transparent)]
pub struct UnsupportedAppRequirement(::windows::core::IUnknown);
impl UnsupportedAppRequirement {
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn Requirement(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Requirement)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn Reasons(&self) -> ::windows::core::Result<UnsupportedAppRequirementReasons> {
        let this = self;
        unsafe {
            let mut result__: UnsupportedAppRequirementReasons = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Reasons)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnsupportedAppRequirementReasons>(result__)
        }
    }
}
impl ::core::clone::Clone for UnsupportedAppRequirement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UnsupportedAppRequirement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UnsupportedAppRequirement {}
impl ::core::fmt::Debug for UnsupportedAppRequirement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnsupportedAppRequirement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UnsupportedAppRequirement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Profile.UnsupportedAppRequirement;{6182445c-894b-5cbc-8976-a98e0a9b998d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UnsupportedAppRequirement {
    type Vtable = IUnsupportedAppRequirement_Vtbl;
    const IID: ::windows::core::GUID = <IUnsupportedAppRequirement as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UnsupportedAppRequirement {
    const NAME: &'static str = "Windows.System.Profile.UnsupportedAppRequirement";
}
impl ::core::convert::From<UnsupportedAppRequirement> for ::windows::core::IUnknown {
    fn from(value: UnsupportedAppRequirement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UnsupportedAppRequirement> for ::windows::core::IUnknown {
    fn from(value: &UnsupportedAppRequirement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UnsupportedAppRequirement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UnsupportedAppRequirement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UnsupportedAppRequirement> for ::windows::core::IInspectable {
    fn from(value: UnsupportedAppRequirement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UnsupportedAppRequirement> for ::windows::core::IInspectable {
    fn from(value: &UnsupportedAppRequirement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UnsupportedAppRequirement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UnsupportedAppRequirement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UnsupportedAppRequirement {}
unsafe impl ::core::marker::Sync for UnsupportedAppRequirement {}
#[doc = "*Required features: `\"System_Profile\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UnsupportedAppRequirementReasons(pub u32);
impl UnsupportedAppRequirementReasons {
    pub const Unknown: Self = Self(0u32);
    pub const DeniedBySystem: Self = Self(1u32);
}
impl ::core::marker::Copy for UnsupportedAppRequirementReasons {}
impl ::core::clone::Clone for UnsupportedAppRequirementReasons {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UnsupportedAppRequirementReasons {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UnsupportedAppRequirementReasons {
    type Abi = Self;
}
impl ::core::fmt::Debug for UnsupportedAppRequirementReasons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnsupportedAppRequirementReasons").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for UnsupportedAppRequirementReasons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for UnsupportedAppRequirementReasons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for UnsupportedAppRequirementReasons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for UnsupportedAppRequirementReasons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for UnsupportedAppRequirementReasons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for UnsupportedAppRequirementReasons {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Profile.UnsupportedAppRequirementReasons;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_Profile\"`*"]
pub struct WindowsIntegrityPolicy {}
impl WindowsIntegrityPolicy {
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn IsEnabled() -> ::windows::core::Result<bool> {
        Self::IWindowsIntegrityPolicyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn IsEnabledForTrial() -> ::windows::core::Result<bool> {
        Self::IWindowsIntegrityPolicyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabledForTrial)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn CanDisable() -> ::windows::core::Result<bool> {
        Self::IWindowsIntegrityPolicyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanDisable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`*"]
    pub fn IsDisableSupported() -> ::windows::core::Result<bool> {
        Self::IWindowsIntegrityPolicyStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDisableSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PolicyChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IWindowsIntegrityPolicyStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PolicyChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Profile\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePolicyChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IWindowsIntegrityPolicyStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemovePolicyChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IWindowsIntegrityPolicyStatics<R, F: FnOnce(&IWindowsIntegrityPolicyStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WindowsIntegrityPolicy, IWindowsIntegrityPolicyStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for WindowsIntegrityPolicy {
    const NAME: &'static str = "Windows.System.Profile.WindowsIntegrityPolicy";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
