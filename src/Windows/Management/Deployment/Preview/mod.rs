#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Management_Deployment_Preview`*"]
pub struct ClassicAppManager {}
impl ClassicAppManager {
    #[doc = "*Required features: `Management_Deployment_Preview`*"]
    pub fn FindInstalledApp<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(appuninstallkey: Param0) -> ::windows::core::Result<InstalledClassicAppInfo> {
        Self::IClassicAppManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), appuninstallkey.into_param().abi(), &mut result__).from_abi::<InstalledClassicAppInfo>(result__)
        })
    }
    pub fn IClassicAppManagerStatics<R, F: FnOnce(&IClassicAppManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ClassicAppManager, IClassicAppManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ClassicAppManager {
    const NAME: &'static str = "Windows.Management.Deployment.Preview.ClassicAppManager";
}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct DeploymentPreviewContract(pub u8);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClassicAppManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClassicAppManagerStatics {
    type Vtable = IClassicAppManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2fad668_882c_4f33_b035_0df7b90d67e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassicAppManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appuninstallkey: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInstalledClassicAppInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInstalledClassicAppInfo {
    type Vtable = IInstalledClassicAppInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a7d3da3_65d0_4086_80d6_0610d760207d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstalledClassicAppInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Management_Deployment_Preview`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InstalledClassicAppInfo(pub ::windows::core::IInspectable);
impl InstalledClassicAppInfo {
    #[doc = "*Required features: `Management_Deployment_Preview`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Management_Deployment_Preview`*"]
    pub fn DisplayVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InstalledClassicAppInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.Preview.InstalledClassicAppInfo;{0a7d3da3-65d0-4086-80d6-0610d760207d})");
}
unsafe impl ::windows::core::Interface for InstalledClassicAppInfo {
    type Vtable = IInstalledClassicAppInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a7d3da3_65d0_4086_80d6_0610d760207d);
}
impl ::windows::core::RuntimeName for InstalledClassicAppInfo {
    const NAME: &'static str = "Windows.Management.Deployment.Preview.InstalledClassicAppInfo";
}
impl ::core::convert::From<InstalledClassicAppInfo> for ::windows::core::IUnknown {
    fn from(value: InstalledClassicAppInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InstalledClassicAppInfo> for ::windows::core::IUnknown {
    fn from(value: &InstalledClassicAppInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InstalledClassicAppInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InstalledClassicAppInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InstalledClassicAppInfo> for ::windows::core::IInspectable {
    fn from(value: InstalledClassicAppInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InstalledClassicAppInfo> for ::windows::core::IInspectable {
    fn from(value: &InstalledClassicAppInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InstalledClassicAppInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InstalledClassicAppInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InstalledClassicAppInfo {}
unsafe impl ::core::marker::Sync for InstalledClassicAppInfo {}
