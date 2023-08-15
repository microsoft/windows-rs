#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationDataManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IApplicationDataManager {
    type Vtable = IApplicationDataManager_Vtbl;
}
impl ::core::clone::Clone for IApplicationDataManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IApplicationDataManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74d10432_2e99_4000_9a3a_64307e858129);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationDataManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationDataManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IApplicationDataManagerStatics {
    type Vtable = IApplicationDataManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IApplicationDataManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IApplicationDataManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1e1862e3_698e_49a1_9752_dee94925b9b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationDataManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub CreateForPackageFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CreateForPackageFamily: usize,
}
#[doc = "*Required features: `\"Management_Core\"`*"]
#[repr(transparent)]
pub struct ApplicationDataManager(::windows_core::IUnknown);
impl ApplicationDataManager {
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn CreateForPackageFamily(packagefamilyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Storage::ApplicationData> {
        Self::IApplicationDataManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateForPackageFamily)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IApplicationDataManagerStatics<R, F: FnOnce(&IApplicationDataManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ApplicationDataManager, IApplicationDataManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ApplicationDataManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationDataManager {}
impl ::core::fmt::Debug for ApplicationDataManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationDataManager").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ApplicationDataManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Core.ApplicationDataManager;{74d10432-2e99-4000-9a3a-64307e858129})");
}
impl ::core::clone::Clone for ApplicationDataManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ApplicationDataManager {
    type Vtable = IApplicationDataManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ApplicationDataManager {
    const IID: ::windows_core::GUID = <IApplicationDataManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ApplicationDataManager {
    const NAME: &'static str = "Windows.Management.Core.ApplicationDataManager";
}
::windows_core::imp::interface_hierarchy!(ApplicationDataManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ApplicationDataManager {}
unsafe impl ::core::marker::Sync for ApplicationDataManager {}
