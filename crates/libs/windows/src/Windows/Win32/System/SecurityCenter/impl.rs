#[cfg(feature = "Win32_System_Com")]
pub trait IWSCDefaultProductImpl: Sized + IDispatchImpl {
    fn SetDefaultProduct();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWSCDefaultProduct {
    const NAME: &'static str = "Windows.Win32.System.SecurityCenter.IWSCDefaultProduct";
}
#[cfg(feature = "Win32_System_Com")]
impl IWSCDefaultProductVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSCDefaultProductImpl, const OFFSET: isize>() -> IWSCDefaultProductVtbl {
        unsafe extern "system" fn SetDefaultProduct<Impl: IWSCDefaultProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, etype: SECURITY_PRODUCT_TYPE, pguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDefaultProduct(etype, &*(&pguid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSCDefaultProduct>, ::windows::core::GetTrustLevel, SetDefaultProduct::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWSCProductListImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn Count();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWSCProductList {
    const NAME: &'static str = "Windows.Win32.System.SecurityCenter.IWSCProductList";
}
#[cfg(feature = "Win32_System_Com")]
impl IWSCProductListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSCProductListImpl, const OFFSET: isize>() -> IWSCProductListVtbl {
        unsafe extern "system" fn Initialize<Impl: IWSCProductListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: WSC_SECURITY_PROVIDER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(provider) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IWSCProductListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IWSCProductListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSCProductList>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>, Count::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWscProductImpl: Sized + IDispatchImpl {
    fn ProductName();
    fn ProductState();
    fn SignatureStatus();
    fn RemediationPath();
    fn ProductStateTimestamp();
    fn ProductGuid();
    fn ProductIsDefault();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWscProduct {
    const NAME: &'static str = "Windows.Win32.System.SecurityCenter.IWscProduct";
}
#[cfg(feature = "Win32_System_Com")]
impl IWscProductVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWscProductImpl, const OFFSET: isize>() -> IWscProductVtbl {
        unsafe extern "system" fn ProductName<Impl: IWscProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductName(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductState<Impl: IWscProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut WSC_SECURITY_PRODUCT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductState(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignatureStatus<Impl: IWscProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut WSC_SECURITY_SIGNATURE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignatureStatus(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemediationPath<Impl: IWscProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemediationPath(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductStateTimestamp<Impl: IWscProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductStateTimestamp(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductGuid<Impl: IWscProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductGuid(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductIsDefault<Impl: IWscProductImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductIsDefault(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWscProduct>, ::windows::core::GetTrustLevel, ProductName::<Impl, OFFSET>, ProductState::<Impl, OFFSET>, SignatureStatus::<Impl, OFFSET>, RemediationPath::<Impl, OFFSET>, ProductStateTimestamp::<Impl, OFFSET>, ProductGuid::<Impl, OFFSET>, ProductIsDefault::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWscProduct2Impl: Sized + IWscProductImpl + IDispatchImpl {
    fn AntivirusScanSubstatus();
    fn AntivirusSettingsSubstatus();
    fn AntivirusProtectionUpdateSubstatus();
    fn FirewallDomainProfileSubstatus();
    fn FirewallPrivateProfileSubstatus();
    fn FirewallPublicProfileSubstatus();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWscProduct2 {
    const NAME: &'static str = "Windows.Win32.System.SecurityCenter.IWscProduct2";
}
#[cfg(feature = "Win32_System_Com")]
impl IWscProduct2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWscProduct2Impl, const OFFSET: isize>() -> IWscProduct2Vtbl {
        unsafe extern "system" fn AntivirusScanSubstatus<Impl: IWscProduct2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AntivirusScanSubstatus(::core::mem::transmute_copy(&pestatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AntivirusSettingsSubstatus<Impl: IWscProduct2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AntivirusSettingsSubstatus(::core::mem::transmute_copy(&pestatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AntivirusProtectionUpdateSubstatus<Impl: IWscProduct2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AntivirusProtectionUpdateSubstatus(::core::mem::transmute_copy(&pestatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirewallDomainProfileSubstatus<Impl: IWscProduct2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirewallDomainProfileSubstatus(::core::mem::transmute_copy(&pestatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirewallPrivateProfileSubstatus<Impl: IWscProduct2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirewallPrivateProfileSubstatus(::core::mem::transmute_copy(&pestatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirewallPublicProfileSubstatus<Impl: IWscProduct2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirewallPublicProfileSubstatus(::core::mem::transmute_copy(&pestatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWscProduct2>,
            ::windows::core::GetTrustLevel,
            AntivirusScanSubstatus::<Impl, OFFSET>,
            AntivirusSettingsSubstatus::<Impl, OFFSET>,
            AntivirusProtectionUpdateSubstatus::<Impl, OFFSET>,
            FirewallDomainProfileSubstatus::<Impl, OFFSET>,
            FirewallPrivateProfileSubstatus::<Impl, OFFSET>,
            FirewallPublicProfileSubstatus::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWscProduct3Impl: Sized + IWscProduct2Impl + IWscProductImpl + IDispatchImpl {
    fn AntivirusDaysUntilExpired();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWscProduct3 {
    const NAME: &'static str = "Windows.Win32.System.SecurityCenter.IWscProduct3";
}
#[cfg(feature = "Win32_System_Com")]
impl IWscProduct3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWscProduct3Impl, const OFFSET: isize>() -> IWscProduct3Vtbl {
        unsafe extern "system" fn AntivirusDaysUntilExpired<Impl: IWscProduct3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdays: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AntivirusDaysUntilExpired(::core::mem::transmute_copy(&pdwdays)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWscProduct3>, ::windows::core::GetTrustLevel, AntivirusDaysUntilExpired::<Impl, OFFSET>)
    }
}
