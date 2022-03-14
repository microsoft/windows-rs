#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Management_Core\"`*"]
#[repr(transparent)]
pub struct ApplicationDataManager(::windows::core::IUnknown);
impl ApplicationDataManager {
    #[doc = "*Required features: `\"Management_Core\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn CreateForPackageFamily<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(packagefamilyname: Param0) -> ::windows::core::Result<super::super::Storage::ApplicationData> {
        Self::IApplicationDataManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateForPackageFamily)(::core::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::Storage::ApplicationData>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IApplicationDataManagerStatics<R, F: FnOnce(&IApplicationDataManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ApplicationDataManager, IApplicationDataManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ApplicationDataManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for ApplicationDataManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Core.ApplicationDataManager;{74d10432-2e99-4000-9a3a-64307e858129})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ApplicationDataManager {
    type Vtable = IApplicationDataManager_Vtbl;
    const IID: ::windows::core::GUID = <IApplicationDataManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ApplicationDataManager {
    const NAME: &'static str = "Windows.Management.Core.ApplicationDataManager";
}
impl ::core::convert::From<ApplicationDataManager> for ::windows::core::IUnknown {
    fn from(value: ApplicationDataManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ApplicationDataManager> for ::windows::core::IUnknown {
    fn from(value: &ApplicationDataManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ApplicationDataManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ApplicationDataManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ApplicationDataManager> for ::windows::core::IInspectable {
    fn from(value: ApplicationDataManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ApplicationDataManager> for ::windows::core::IInspectable {
    fn from(value: &ApplicationDataManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ApplicationDataManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ApplicationDataManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ApplicationDataManager {}
unsafe impl ::core::marker::Sync for ApplicationDataManager {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationDataManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationDataManager {
    type Vtable = IApplicationDataManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74d10432_2e99_4000_9a3a_64307e858129);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationDataManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationDataManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationDataManagerStatics {
    type Vtable = IApplicationDataManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e1862e3_698e_49a1_9752_dee94925b9b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationDataManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage")]
    pub CreateForPackageFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CreateForPackageFamily: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
