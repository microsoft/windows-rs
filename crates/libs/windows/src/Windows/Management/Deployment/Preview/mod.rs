#[doc = "*Required features: `\"Management_Deployment_Preview\"`*"]
pub struct ClassicAppManager;
impl ClassicAppManager {
    pub fn FindInstalledApp(appuninstallkey: &::windows::core::HSTRING) -> ::windows::core::Result<InstalledClassicAppInfo> {
        Self::IClassicAppManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FindInstalledApp)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(appuninstallkey), result__.as_mut_ptr()).from_abi::<InstalledClassicAppInfo>(result__)
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
#[doc(hidden)]
#[repr(transparent)]
pub struct IClassicAppManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClassicAppManagerStatics {
    type Vtable = IClassicAppManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2fad668_882c_4f33_b035_0df7b90d67e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassicAppManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub FindInstalledApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appuninstallkey: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInstalledClassicAppInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInstalledClassicAppInfo {
    type Vtable = IInstalledClassicAppInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a7d3da3_65d0_4086_80d6_0610d760207d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstalledClassicAppInfo_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Management_Deployment_Preview\"`*"]
#[repr(transparent)]
pub struct InstalledClassicAppInfo(::windows::core::IUnknown);
impl InstalledClassicAppInfo {
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DisplayVersion)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
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
unsafe impl ::windows::core::Interface for InstalledClassicAppInfo {
    type Vtable = IInstalledClassicAppInfo_Vtbl;
    const IID: ::windows::core::GUID = <IInstalledClassicAppInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InstalledClassicAppInfo {
    const NAME: &'static str = "Windows.Management.Deployment.Preview.InstalledClassicAppInfo";
}
impl ::core::convert::From<InstalledClassicAppInfo> for ::windows::core::IUnknown {
    fn from(value: InstalledClassicAppInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InstalledClassicAppInfo> for ::windows::core::IUnknown {
    fn from(value: &InstalledClassicAppInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&InstalledClassicAppInfo> for &::windows::core::IUnknown {
    fn from(value: &InstalledClassicAppInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<InstalledClassicAppInfo> for ::windows::core::IInspectable {
    fn from(value: InstalledClassicAppInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InstalledClassicAppInfo> for ::windows::core::IInspectable {
    fn from(value: &InstalledClassicAppInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&InstalledClassicAppInfo> for &::windows::core::IInspectable {
    fn from(value: &InstalledClassicAppInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for InstalledClassicAppInfo {}
unsafe impl ::core::marker::Sync for InstalledClassicAppInfo {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
