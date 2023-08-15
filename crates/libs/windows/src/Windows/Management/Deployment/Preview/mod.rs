#[doc(hidden)]
#[repr(transparent)]
pub struct IClassicAppManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IClassicAppManagerStatics {
    type Vtable = IClassicAppManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IClassicAppManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IClassicAppManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe2fad668_882c_4f33_b035_0df7b90d67e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassicAppManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FindInstalledApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appuninstallkey: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInstalledClassicAppInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInstalledClassicAppInfo {
    type Vtable = IInstalledClassicAppInfo_Vtbl;
}
impl ::core::clone::Clone for IInstalledClassicAppInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInstalledClassicAppInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a7d3da3_65d0_4086_80d6_0610d760207d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstalledClassicAppInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Management_Deployment_Preview\"`*"]
pub struct ClassicAppManager;
impl ClassicAppManager {
    pub fn FindInstalledApp(appuninstallkey: &::windows_core::HSTRING) -> ::windows_core::Result<InstalledClassicAppInfo> {
        Self::IClassicAppManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindInstalledApp)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(appuninstallkey), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IClassicAppManagerStatics<R, F: FnOnce(&IClassicAppManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ClassicAppManager, IClassicAppManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for ClassicAppManager {
    const NAME: &'static str = "Windows.Management.Deployment.Preview.ClassicAppManager";
}
#[doc = "*Required features: `\"Management_Deployment_Preview\"`*"]
#[repr(transparent)]
pub struct InstalledClassicAppInfo(::windows_core::IUnknown);
impl InstalledClassicAppInfo {
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayVersion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayVersion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InstalledClassicAppInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InstalledClassicAppInfo {}
impl ::core::fmt::Debug for InstalledClassicAppInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InstalledClassicAppInfo").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InstalledClassicAppInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.Preview.InstalledClassicAppInfo;{0a7d3da3-65d0-4086-80d6-0610d760207d})");
}
impl ::core::clone::Clone for InstalledClassicAppInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for InstalledClassicAppInfo {
    type Vtable = IInstalledClassicAppInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InstalledClassicAppInfo {
    const IID: ::windows_core::GUID = <IInstalledClassicAppInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InstalledClassicAppInfo {
    const NAME: &'static str = "Windows.Management.Deployment.Preview.InstalledClassicAppInfo";
}
::windows_core::imp::interface_hierarchy!(InstalledClassicAppInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for InstalledClassicAppInfo {}
unsafe impl ::core::marker::Sync for InstalledClassicAppInfo {}
