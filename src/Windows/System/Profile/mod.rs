#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "System_Profile_SystemManufacturers")]
pub mod SystemManufacturers;
#[doc = "*Required features: `System_Profile`*"]
pub struct AnalyticsInfo {}
impl AnalyticsInfo {
    #[doc = "*Required features: `System_Profile`*"]
    pub fn VersionInfo() -> ::windows::runtime::Result<AnalyticsVersionInfo> {
        Self::IAnalyticsInfoStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AnalyticsVersionInfo>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn DeviceForm() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IAnalyticsInfoStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `System_Profile`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetSystemPropertiesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(attributenames: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>> {
        Self::IAnalyticsInfoStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), attributenames.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>(result__)
        })
    }
    pub fn IAnalyticsInfoStatics<R, F: FnOnce(&IAnalyticsInfoStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AnalyticsInfo, IAnalyticsInfoStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAnalyticsInfoStatics2<R, F: FnOnce(&IAnalyticsInfoStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AnalyticsInfo, IAnalyticsInfoStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for AnalyticsInfo {
    const NAME: &'static str = "Windows.System.Profile.AnalyticsInfo";
}
#[doc = "*Required features: `System_Profile`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AnalyticsVersionInfo(::windows::runtime::IInspectable);
impl AnalyticsVersionInfo {
    #[doc = "*Required features: `System_Profile`*"]
    pub fn DeviceFamily(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn DeviceFamilyVersion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn ProductName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAnalyticsVersionInfo2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AnalyticsVersionInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Profile.AnalyticsVersionInfo;{926130b8-9955-4c74-bdc1-7cd0decf9b03})");
}
unsafe impl ::windows::runtime::Interface for AnalyticsVersionInfo {
    type Vtable = IAnalyticsVersionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2455843000, 39253, 19572, [189, 193, 124, 208, 222, 207, 155, 3]);
}
impl ::windows::runtime::RuntimeName for AnalyticsVersionInfo {
    const NAME: &'static str = "Windows.System.Profile.AnalyticsVersionInfo";
}
impl ::std::convert::From<AnalyticsVersionInfo> for ::windows::runtime::IUnknown {
    fn from(value: AnalyticsVersionInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AnalyticsVersionInfo> for ::windows::runtime::IUnknown {
    fn from(value: &AnalyticsVersionInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AnalyticsVersionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AnalyticsVersionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AnalyticsVersionInfo> for ::windows::runtime::IInspectable {
    fn from(value: AnalyticsVersionInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AnalyticsVersionInfo> for ::windows::runtime::IInspectable {
    fn from(value: &AnalyticsVersionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AnalyticsVersionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AnalyticsVersionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AnalyticsVersionInfo {}
unsafe impl ::std::marker::Sync for AnalyticsVersionInfo {}
#[doc = "*Required features: `System_Profile`*"]
pub struct AppApplicability {}
impl AppApplicability {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_Profile`, `Foundation_Collections`*"]
    pub fn GetUnsupportedAppRequirements<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(capabilities: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<UnsupportedAppRequirement>> {
        Self::IAppApplicabilityStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), capabilities.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<UnsupportedAppRequirement>>(result__)
        })
    }
    pub fn IAppApplicabilityStatics<R, F: FnOnce(&IAppApplicabilityStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppApplicability, IAppApplicabilityStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for AppApplicability {
    const NAME: &'static str = "Windows.System.Profile.AppApplicability";
}
#[doc = "*Required features: `System_Profile`*"]
pub struct EducationSettings {}
impl EducationSettings {
    #[doc = "*Required features: `System_Profile`*"]
    pub fn IsEducationEnvironment() -> ::windows::runtime::Result<bool> {
        Self::IEducationSettingsStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IEducationSettingsStatics<R, F: FnOnce(&IEducationSettingsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<EducationSettings, IEducationSettingsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for EducationSettings {
    const NAME: &'static str = "Windows.System.Profile.EducationSettings";
}
#[doc = "*Required features: `System_Profile`*"]
pub struct HardwareIdentification {}
impl HardwareIdentification {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `System_Profile`, `Storage_Streams`*"]
    pub fn GetPackageSpecificToken<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(nonce: Param0) -> ::windows::runtime::Result<HardwareToken> {
        Self::IHardwareIdentificationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), nonce.into_param().abi(), &mut result__).from_abi::<HardwareToken>(result__)
        })
    }
    pub fn IHardwareIdentificationStatics<R, F: FnOnce(&IHardwareIdentificationStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HardwareIdentification, IHardwareIdentificationStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for HardwareIdentification {
    const NAME: &'static str = "Windows.System.Profile.HardwareIdentification";
}
#[doc = "*Required features: `System_Profile`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HardwareToken(::windows::runtime::IInspectable);
impl HardwareToken {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `System_Profile`, `Storage_Streams`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `System_Profile`, `Storage_Streams`*"]
    pub fn Signature(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `System_Profile`, `Storage_Streams`*"]
    pub fn Certificate(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HardwareToken {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Profile.HardwareToken;{28f6d4c0-fb12-40a4-8167-7f4e03d2724c})");
}
unsafe impl ::windows::runtime::Interface for HardwareToken {
    type Vtable = IHardwareToken_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(687264960, 64274, 16548, [129, 103, 127, 78, 3, 210, 114, 76]);
}
impl ::windows::runtime::RuntimeName for HardwareToken {
    const NAME: &'static str = "Windows.System.Profile.HardwareToken";
}
impl ::std::convert::From<HardwareToken> for ::windows::runtime::IUnknown {
    fn from(value: HardwareToken) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HardwareToken> for ::windows::runtime::IUnknown {
    fn from(value: &HardwareToken) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HardwareToken {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HardwareToken {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HardwareToken> for ::windows::runtime::IInspectable {
    fn from(value: HardwareToken) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HardwareToken> for ::windows::runtime::IInspectable {
    fn from(value: &HardwareToken) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HardwareToken {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HardwareToken {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for HardwareToken {}
unsafe impl ::std::marker::Sync for HardwareToken {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IAnalyticsInfoStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAnalyticsInfoStatics {
    type Vtable = IAnalyticsInfoStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(492757094, 6285, 23465, [67, 135, 172, 174, 176, 231, 227, 5]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnalyticsInfoStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IAnalyticsInfoStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAnalyticsInfoStatics2 {
    type Vtable = IAnalyticsInfoStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(269944042, 43001, 18130, [171, 148, 1, 104, 101, 175, 219, 37]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnalyticsInfoStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attributenames: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IAnalyticsVersionInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAnalyticsVersionInfo {
    type Vtable = IAnalyticsVersionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2455843000, 39253, 19572, [189, 193, 124, 208, 222, 207, 155, 3]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnalyticsVersionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IAnalyticsVersionInfo2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAnalyticsVersionInfo2 {
    type Vtable = IAnalyticsVersionInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1994986929, 65334, 16508, [159, 87, 22, 13, 62, 84, 7, 71]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnalyticsVersionInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IAppApplicabilityStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppApplicabilityStatics {
    type Vtable = IAppApplicabilityStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(375693442, 3896, 23705, [131, 228, 72, 153, 89, 112, 134, 28]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppApplicabilityStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, capabilities: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IEducationSettingsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEducationSettingsStatics {
    type Vtable = IEducationSettingsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4233359599, 19774, 19987, [155, 35, 80, 95, 77, 9, 30, 146]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEducationSettingsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IHardwareIdentificationStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHardwareIdentificationStatics {
    type Vtable = IHardwareIdentificationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2534564064, 61808, 19010, [189, 85, 169, 0, 178, 18, 218, 226]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHardwareIdentificationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nonce: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IHardwareToken(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHardwareToken {
    type Vtable = IHardwareToken_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(687264960, 64274, 16548, [129, 103, 127, 78, 3, 210, 114, 76]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHardwareToken_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IKnownRetailInfoPropertiesStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKnownRetailInfoPropertiesStatics {
    type Vtable = IKnownRetailInfoPropertiesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2572620152, 20495, 18558, [142, 117, 41, 229, 81, 114, 135, 18]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownRetailInfoPropertiesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPlatformDiagnosticsAndUsageDataSettingsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPlatformDiagnosticsAndUsageDataSettingsStatics {
    type Vtable = IPlatformDiagnosticsAndUsageDataSettingsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3068283931, 31516, 19250, [140, 98, 166, 101, 151, 206, 114, 58]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlatformDiagnosticsAndUsageDataSettingsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PlatformDataCollectionLevel) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, level: PlatformDataCollectionLevel, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IRetailInfoStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRetailInfoStatics {
    type Vtable = IRetailInfoStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(118671032, 35730, 20266, [132, 153, 3, 31, 23, 152, 214, 239]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRetailInfoStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISharedModeSettingsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISharedModeSettingsStatics {
    type Vtable = ISharedModeSettingsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2302538766, 51926, 19792, [140, 73, 111, 207, 192, 62, 219, 41]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedModeSettingsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISharedModeSettingsStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISharedModeSettingsStatics2 {
    type Vtable = ISharedModeSettingsStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1619626148, 52465, 20200, [165, 226, 253, 106, 29, 12, 250, 200]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISharedModeSettingsStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISystemIdentificationInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemIdentificationInfo {
    type Vtable = ISystemIdentificationInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(207986301, 50114, 19763, [162, 223, 33, 188, 65, 145, 110, 179]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemIdentificationInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SystemIdentificationSource) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISystemIdentificationStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemIdentificationStatics {
    type Vtable = ISystemIdentificationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1434580010, 54239, 19859, [163, 125, 196, 26, 97, 108, 109, 1]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemIdentificationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISystemSetupInfoStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISystemSetupInfoStatics {
    type Vtable = ISystemSetupInfoStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(748036264, 7560, 24109, [163, 36, 165, 67, 175, 66, 71, 238]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemSetupInfoStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SystemOutOfBoxExperienceState) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IUnsupportedAppRequirement(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUnsupportedAppRequirement {
    type Vtable = IUnsupportedAppRequirement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1635927132, 35147, 23740, [137, 118, 169, 142, 10, 155, 153, 141]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnsupportedAppRequirement_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UnsupportedAppRequirementReasons) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWindowsIntegrityPolicyStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWindowsIntegrityPolicyStatics {
    type Vtable = IWindowsIntegrityPolicyStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2099085787, 36195, 18313, [158, 165, 221, 207, 101, 169, 79, 60]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsIntegrityPolicyStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: `System_Profile`*"]
pub struct KnownRetailInfoProperties {}
impl KnownRetailInfoProperties {
    #[doc = "*Required features: `System_Profile`*"]
    pub fn RetailAccessCode() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn ManufacturerName() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn ModelName() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn DisplayModelName() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn Price() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn IsFeatured() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn FormFactor() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn ScreenSize() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn Weight() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn DisplayDescription() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn BatteryLifeDescription() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn ProcessorDescription() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn Memory() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn StorageDescription() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn GraphicsDescription() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn FrontCameraDescription() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn RearCameraDescription() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn HasNfc() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn HasSdSlot() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn HasOpticalDrive() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn IsOfficeInstalled() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn WindowsEdition() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRetailInfoPropertiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IKnownRetailInfoPropertiesStatics<R, F: FnOnce(&IKnownRetailInfoPropertiesStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<KnownRetailInfoProperties, IKnownRetailInfoPropertiesStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for KnownRetailInfoProperties {
    const NAME: &'static str = "Windows.System.Profile.KnownRetailInfoProperties";
}
#[doc = "*Required features: `System_Profile`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PlatformDataCollectionLevel(pub i32);
impl PlatformDataCollectionLevel {
    pub const Security: PlatformDataCollectionLevel = PlatformDataCollectionLevel(0i32);
    pub const Basic: PlatformDataCollectionLevel = PlatformDataCollectionLevel(1i32);
    pub const Enhanced: PlatformDataCollectionLevel = PlatformDataCollectionLevel(2i32);
    pub const Full: PlatformDataCollectionLevel = PlatformDataCollectionLevel(3i32);
}
impl ::std::convert::From<i32> for PlatformDataCollectionLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PlatformDataCollectionLevel {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PlatformDataCollectionLevel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.Profile.PlatformDataCollectionLevel;i4)");
}
#[doc = "*Required features: `System_Profile`*"]
pub struct PlatformDiagnosticsAndUsageDataSettings {}
impl PlatformDiagnosticsAndUsageDataSettings {
    #[doc = "*Required features: `System_Profile`*"]
    pub fn CollectionLevel() -> ::windows::runtime::Result<PlatformDataCollectionLevel> {
        Self::IPlatformDiagnosticsAndUsageDataSettingsStatics(|this| unsafe {
            let mut result__: PlatformDataCollectionLevel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PlatformDataCollectionLevel>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Profile`, `Foundation`*"]
    pub fn CollectionLevelChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPlatformDiagnosticsAndUsageDataSettingsStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Profile`, `Foundation`*"]
    pub fn RemoveCollectionLevelChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IPlatformDiagnosticsAndUsageDataSettingsStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn CanCollectDiagnostics(level: PlatformDataCollectionLevel) -> ::windows::runtime::Result<bool> {
        Self::IPlatformDiagnosticsAndUsageDataSettingsStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), level, &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IPlatformDiagnosticsAndUsageDataSettingsStatics<R, F: FnOnce(&IPlatformDiagnosticsAndUsageDataSettingsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PlatformDiagnosticsAndUsageDataSettings, IPlatformDiagnosticsAndUsageDataSettingsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for PlatformDiagnosticsAndUsageDataSettings {
    const NAME: &'static str = "Windows.System.Profile.PlatformDiagnosticsAndUsageDataSettings";
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct ProfileHardwareTokenContract(pub u8);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct ProfileRetailInfoContract(pub u8);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct ProfileSharedModeContract(pub u8);
#[doc = "*Required features: `System_Profile`*"]
pub struct RetailInfo {}
impl RetailInfo {
    #[doc = "*Required features: `System_Profile`*"]
    pub fn IsDemoModeEnabled() -> ::windows::runtime::Result<bool> {
        Self::IRetailInfoStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `System_Profile`, `Foundation_Collections`*"]
    pub fn Properties() -> ::windows::runtime::Result<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        Self::IRetailInfoStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(result__)
        })
    }
    pub fn IRetailInfoStatics<R, F: FnOnce(&IRetailInfoStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RetailInfo, IRetailInfoStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for RetailInfo {
    const NAME: &'static str = "Windows.System.Profile.RetailInfo";
}
#[doc = "*Required features: `System_Profile`*"]
pub struct SharedModeSettings {}
impl SharedModeSettings {
    #[doc = "*Required features: `System_Profile`*"]
    pub fn IsEnabled() -> ::windows::runtime::Result<bool> {
        Self::ISharedModeSettingsStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn ShouldAvoidLocalStorage() -> ::windows::runtime::Result<bool> {
        Self::ISharedModeSettingsStatics2(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn ISharedModeSettingsStatics<R, F: FnOnce(&ISharedModeSettingsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SharedModeSettings, ISharedModeSettingsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISharedModeSettingsStatics2<R, F: FnOnce(&ISharedModeSettingsStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SharedModeSettings, ISharedModeSettingsStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for SharedModeSettings {
    const NAME: &'static str = "Windows.System.Profile.SharedModeSettings";
}
#[doc = "*Required features: `System_Profile`*"]
pub struct SystemIdentification {}
impl SystemIdentification {
    #[doc = "*Required features: `System_Profile`*"]
    pub fn GetSystemIdForPublisher() -> ::windows::runtime::Result<SystemIdentificationInfo> {
        Self::ISystemIdentificationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SystemIdentificationInfo>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn GetSystemIdForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::User>>(user: Param0) -> ::windows::runtime::Result<SystemIdentificationInfo> {
        Self::ISystemIdentificationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<SystemIdentificationInfo>(result__)
        })
    }
    pub fn ISystemIdentificationStatics<R, F: FnOnce(&ISystemIdentificationStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SystemIdentification, ISystemIdentificationStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for SystemIdentification {
    const NAME: &'static str = "Windows.System.Profile.SystemIdentification";
}
#[doc = "*Required features: `System_Profile`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SystemIdentificationInfo(::windows::runtime::IInspectable);
impl SystemIdentificationInfo {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `System_Profile`, `Storage_Streams`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn Source(&self) -> ::windows::runtime::Result<SystemIdentificationSource> {
        let this = self;
        unsafe {
            let mut result__: SystemIdentificationSource = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SystemIdentificationSource>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SystemIdentificationInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Profile.SystemIdentificationInfo;{0c659e7d-c3c2-4d33-a2df-21bc41916eb3})");
}
unsafe impl ::windows::runtime::Interface for SystemIdentificationInfo {
    type Vtable = ISystemIdentificationInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(207986301, 50114, 19763, [162, 223, 33, 188, 65, 145, 110, 179]);
}
impl ::windows::runtime::RuntimeName for SystemIdentificationInfo {
    const NAME: &'static str = "Windows.System.Profile.SystemIdentificationInfo";
}
impl ::std::convert::From<SystemIdentificationInfo> for ::windows::runtime::IUnknown {
    fn from(value: SystemIdentificationInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SystemIdentificationInfo> for ::windows::runtime::IUnknown {
    fn from(value: &SystemIdentificationInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SystemIdentificationInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SystemIdentificationInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SystemIdentificationInfo> for ::windows::runtime::IInspectable {
    fn from(value: SystemIdentificationInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SystemIdentificationInfo> for ::windows::runtime::IInspectable {
    fn from(value: &SystemIdentificationInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SystemIdentificationInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SystemIdentificationInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SystemIdentificationInfo {}
unsafe impl ::std::marker::Sync for SystemIdentificationInfo {}
#[doc = "*Required features: `System_Profile`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SystemIdentificationSource(pub i32);
impl SystemIdentificationSource {
    pub const None: SystemIdentificationSource = SystemIdentificationSource(0i32);
    pub const Tpm: SystemIdentificationSource = SystemIdentificationSource(1i32);
    pub const Uefi: SystemIdentificationSource = SystemIdentificationSource(2i32);
    pub const Registry: SystemIdentificationSource = SystemIdentificationSource(3i32);
}
impl ::std::convert::From<i32> for SystemIdentificationSource {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SystemIdentificationSource {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SystemIdentificationSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.Profile.SystemIdentificationSource;i4)");
}
#[doc = "*Required features: `System_Profile`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SystemOutOfBoxExperienceState(pub i32);
impl SystemOutOfBoxExperienceState {
    pub const NotStarted: SystemOutOfBoxExperienceState = SystemOutOfBoxExperienceState(0i32);
    pub const InProgress: SystemOutOfBoxExperienceState = SystemOutOfBoxExperienceState(1i32);
    pub const Completed: SystemOutOfBoxExperienceState = SystemOutOfBoxExperienceState(2i32);
}
impl ::std::convert::From<i32> for SystemOutOfBoxExperienceState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SystemOutOfBoxExperienceState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SystemOutOfBoxExperienceState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.Profile.SystemOutOfBoxExperienceState;i4)");
}
#[doc = "*Required features: `System_Profile`*"]
pub struct SystemSetupInfo {}
impl SystemSetupInfo {
    #[doc = "*Required features: `System_Profile`*"]
    pub fn OutOfBoxExperienceState() -> ::windows::runtime::Result<SystemOutOfBoxExperienceState> {
        Self::ISystemSetupInfoStatics(|this| unsafe {
            let mut result__: SystemOutOfBoxExperienceState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SystemOutOfBoxExperienceState>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Profile`, `Foundation`*"]
    pub fn OutOfBoxExperienceStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::ISystemSetupInfoStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Profile`, `Foundation`*"]
    pub fn RemoveOutOfBoxExperienceStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::ISystemSetupInfoStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    pub fn ISystemSetupInfoStatics<R, F: FnOnce(&ISystemSetupInfoStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SystemSetupInfo, ISystemSetupInfoStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for SystemSetupInfo {
    const NAME: &'static str = "Windows.System.Profile.SystemSetupInfo";
}
#[doc = "*Required features: `System_Profile`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UnsupportedAppRequirement(::windows::runtime::IInspectable);
impl UnsupportedAppRequirement {
    #[doc = "*Required features: `System_Profile`*"]
    pub fn Requirement(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn Reasons(&self) -> ::windows::runtime::Result<UnsupportedAppRequirementReasons> {
        let this = self;
        unsafe {
            let mut result__: UnsupportedAppRequirementReasons = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UnsupportedAppRequirementReasons>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UnsupportedAppRequirement {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Profile.UnsupportedAppRequirement;{6182445c-894b-5cbc-8976-a98e0a9b998d})");
}
unsafe impl ::windows::runtime::Interface for UnsupportedAppRequirement {
    type Vtable = IUnsupportedAppRequirement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1635927132, 35147, 23740, [137, 118, 169, 142, 10, 155, 153, 141]);
}
impl ::windows::runtime::RuntimeName for UnsupportedAppRequirement {
    const NAME: &'static str = "Windows.System.Profile.UnsupportedAppRequirement";
}
impl ::std::convert::From<UnsupportedAppRequirement> for ::windows::runtime::IUnknown {
    fn from(value: UnsupportedAppRequirement) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UnsupportedAppRequirement> for ::windows::runtime::IUnknown {
    fn from(value: &UnsupportedAppRequirement) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UnsupportedAppRequirement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UnsupportedAppRequirement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UnsupportedAppRequirement> for ::windows::runtime::IInspectable {
    fn from(value: UnsupportedAppRequirement) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UnsupportedAppRequirement> for ::windows::runtime::IInspectable {
    fn from(value: &UnsupportedAppRequirement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UnsupportedAppRequirement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UnsupportedAppRequirement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for UnsupportedAppRequirement {}
unsafe impl ::std::marker::Sync for UnsupportedAppRequirement {}
#[doc = "*Required features: `System_Profile`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UnsupportedAppRequirementReasons(pub u32);
impl UnsupportedAppRequirementReasons {
    pub const Unknown: UnsupportedAppRequirementReasons = UnsupportedAppRequirementReasons(0u32);
    pub const DeniedBySystem: UnsupportedAppRequirementReasons = UnsupportedAppRequirementReasons(1u32);
}
impl ::std::convert::From<u32> for UnsupportedAppRequirementReasons {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UnsupportedAppRequirementReasons {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UnsupportedAppRequirementReasons {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.Profile.UnsupportedAppRequirementReasons;u4)");
}
impl ::std::ops::BitOr for UnsupportedAppRequirementReasons {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for UnsupportedAppRequirementReasons {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for UnsupportedAppRequirementReasons {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for UnsupportedAppRequirementReasons {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for UnsupportedAppRequirementReasons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `System_Profile`*"]
pub struct WindowsIntegrityPolicy {}
impl WindowsIntegrityPolicy {
    #[doc = "*Required features: `System_Profile`*"]
    pub fn IsEnabled() -> ::windows::runtime::Result<bool> {
        Self::IWindowsIntegrityPolicyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn IsEnabledForTrial() -> ::windows::runtime::Result<bool> {
        Self::IWindowsIntegrityPolicyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn CanDisable() -> ::windows::runtime::Result<bool> {
        Self::IWindowsIntegrityPolicyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `System_Profile`*"]
    pub fn IsDisableSupported() -> ::windows::runtime::Result<bool> {
        Self::IWindowsIntegrityPolicyStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Profile`, `Foundation`*"]
    pub fn PolicyChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IWindowsIntegrityPolicyStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Profile`, `Foundation`*"]
    pub fn RemovePolicyChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IWindowsIntegrityPolicyStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    pub fn IWindowsIntegrityPolicyStatics<R, F: FnOnce(&IWindowsIntegrityPolicyStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WindowsIntegrityPolicy, IWindowsIntegrityPolicyStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for WindowsIntegrityPolicy {
    const NAME: &'static str = "Windows.System.Profile.WindowsIntegrityPolicy";
}
