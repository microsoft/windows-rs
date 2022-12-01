#[doc(hidden)]
#[repr(transparent)]
pub struct IClassicAppManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IClassicAppManagerStatics {
    type Vtable = IClassicAppManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IClassicAppManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2fad668_882c_4f33_b035_0df7b90d67e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassicAppManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FindInstalledApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appuninstallkey: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInstalledClassicAppInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IInstalledClassicAppInfo {
    type Vtable = IInstalledClassicAppInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IInstalledClassicAppInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a7d3da3_65d0_4086_80d6_0610d760207d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstalledClassicAppInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Management_Deployment_Preview\"`*"]
pub struct ClassicAppManager;
impl ClassicAppManager {
    pub fn FindInstalledApp(appuninstallkey: &::windows::core::HSTRING) -> ::windows::core::Result<InstalledClassicAppInfo> {
        Self::IClassicAppManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindInstalledApp)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(appuninstallkey), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IClassicAppManagerStatics<R, F: FnOnce(&IClassicAppManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ClassicAppManager, IClassicAppManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for ClassicAppManager {
    const NAME: &'static str = "Windows.Management.Deployment.Preview.ClassicAppManager";
}
#[doc = "*Required features: `\"Management_Deployment_Preview\"`*"]
#[repr(transparent)]
pub struct InstalledClassicAppInfo(::windows::core::IUnknown);
impl InstalledClassicAppInfo {
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DisplayVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayVersion)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for InstalledClassicAppInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for InstalledClassicAppInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Deployment.Preview.InstalledClassicAppInfo;{0a7d3da3-65d0-4086-80d6-0610d760207d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for InstalledClassicAppInfo {
    type Vtable = IInstalledClassicAppInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for InstalledClassicAppInfo {
    const IID: ::windows::core::GUID = <IInstalledClassicAppInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InstalledClassicAppInfo {
    const NAME: &'static str = "Windows.Management.Deployment.Preview.InstalledClassicAppInfo";
}
::windows::core::interface_hierarchy!(InstalledClassicAppInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InstalledClassicAppInfo {}
unsafe impl ::core::marker::Sync for InstalledClassicAppInfo {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
