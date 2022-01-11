#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSCDefaultProductImpl: Sized + IDispatchImpl {
    fn SetDefaultProduct();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSCDefaultProductVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSCDefaultProductImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSCDefaultProductVtbl {
        unsafe extern "system" fn SetDefaultProduct<Impl: IWSCDefaultProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, etype: SECURITY_PRODUCT_TYPE, pguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, SetDefaultProduct::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSCDefaultProduct as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSCProductListImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn Count();
    fn Item();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSCProductListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSCProductListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSCProductListVtbl {
        unsafe extern "system" fn Initialize<Impl: IWSCProductListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: WSC_SECURITY_PROVIDER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IWSCProductListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IWSCProductListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSCProductList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWscProductImpl: Sized + IDispatchImpl {
    fn ProductName();
    fn ProductState();
    fn SignatureStatus();
    fn RemediationPath();
    fn ProductStateTimestamp();
    fn ProductGuid();
    fn ProductIsDefault();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWscProductVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWscProductImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWscProductVtbl {
        unsafe extern "system" fn ProductName<Impl: IWscProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProductState<Impl: IWscProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut WSC_SECURITY_PRODUCT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SignatureStatus<Impl: IWscProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut WSC_SECURITY_SIGNATURE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemediationPath<Impl: IWscProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProductStateTimestamp<Impl: IWscProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProductGuid<Impl: IWscProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProductIsDefault<Impl: IWscProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            ProductName::<Impl, IMPL_OFFSET>,
            ProductState::<Impl, IMPL_OFFSET>,
            SignatureStatus::<Impl, IMPL_OFFSET>,
            RemediationPath::<Impl, IMPL_OFFSET>,
            ProductStateTimestamp::<Impl, IMPL_OFFSET>,
            ProductGuid::<Impl, IMPL_OFFSET>,
            ProductIsDefault::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWscProduct as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWscProduct2Impl: Sized + IWscProductImpl + IDispatchImpl {
    fn AntivirusScanSubstatus();
    fn AntivirusSettingsSubstatus();
    fn AntivirusProtectionUpdateSubstatus();
    fn FirewallDomainProfileSubstatus();
    fn FirewallPrivateProfileSubstatus();
    fn FirewallPublicProfileSubstatus();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWscProduct2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWscProduct2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWscProduct2Vtbl {
        unsafe extern "system" fn AntivirusScanSubstatus<Impl: IWscProduct2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AntivirusSettingsSubstatus<Impl: IWscProduct2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AntivirusProtectionUpdateSubstatus<Impl: IWscProduct2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FirewallDomainProfileSubstatus<Impl: IWscProduct2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FirewallPrivateProfileSubstatus<Impl: IWscProduct2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FirewallPublicProfileSubstatus<Impl: IWscProduct2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            ProductName::<Impl, IMPL_OFFSET>,
            ProductState::<Impl, IMPL_OFFSET>,
            SignatureStatus::<Impl, IMPL_OFFSET>,
            RemediationPath::<Impl, IMPL_OFFSET>,
            ProductStateTimestamp::<Impl, IMPL_OFFSET>,
            ProductGuid::<Impl, IMPL_OFFSET>,
            ProductIsDefault::<Impl, IMPL_OFFSET>,
            AntivirusScanSubstatus::<Impl, IMPL_OFFSET>,
            AntivirusSettingsSubstatus::<Impl, IMPL_OFFSET>,
            AntivirusProtectionUpdateSubstatus::<Impl, IMPL_OFFSET>,
            FirewallDomainProfileSubstatus::<Impl, IMPL_OFFSET>,
            FirewallPrivateProfileSubstatus::<Impl, IMPL_OFFSET>,
            FirewallPublicProfileSubstatus::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWscProduct2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWscProduct3Impl: Sized + IWscProduct2Impl + IWscProductImpl + IDispatchImpl {
    fn AntivirusDaysUntilExpired();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWscProduct3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWscProduct3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWscProduct3Vtbl {
        unsafe extern "system" fn AntivirusDaysUntilExpired<Impl: IWscProduct3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdays: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            ProductName::<Impl, IMPL_OFFSET>,
            ProductState::<Impl, IMPL_OFFSET>,
            SignatureStatus::<Impl, IMPL_OFFSET>,
            RemediationPath::<Impl, IMPL_OFFSET>,
            ProductStateTimestamp::<Impl, IMPL_OFFSET>,
            ProductGuid::<Impl, IMPL_OFFSET>,
            ProductIsDefault::<Impl, IMPL_OFFSET>,
            AntivirusScanSubstatus::<Impl, IMPL_OFFSET>,
            AntivirusSettingsSubstatus::<Impl, IMPL_OFFSET>,
            AntivirusProtectionUpdateSubstatus::<Impl, IMPL_OFFSET>,
            FirewallDomainProfileSubstatus::<Impl, IMPL_OFFSET>,
            FirewallPrivateProfileSubstatus::<Impl, IMPL_OFFSET>,
            FirewallPublicProfileSubstatus::<Impl, IMPL_OFFSET>,
            AntivirusDaysUntilExpired::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWscProduct3 as ::windows::core::Interface>::IID
    }
}
