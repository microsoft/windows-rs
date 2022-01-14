#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSCDefaultProduct_Impl: Sized + super::Com::IDispatch_Impl {
    fn SetDefaultProduct(&mut self, etype: SECURITY_PRODUCT_TYPE, pguid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSCDefaultProduct_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSCDefaultProduct_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSCDefaultProduct_Vtbl {
        unsafe extern "system" fn SetDefaultProduct<Impl: IWSCDefaultProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, etype: SECURITY_PRODUCT_TYPE, pguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultProduct(::core::mem::transmute_copy(&etype), ::core::mem::transmute_copy(&pguid)).into()
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetDefaultProduct: SetDefaultProduct::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSCDefaultProduct as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSCProductList_Impl: Sized + super::Com::IDispatch_Impl {
    fn Initialize(&mut self, provider: WSC_SECURITY_PROVIDER) -> ::windows::core::Result<()>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: u32) -> ::windows::core::Result<IWscProduct>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSCProductList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSCProductList_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSCProductList_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IWSCProductList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: WSC_SECURITY_PROVIDER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&provider)).into()
        }
        unsafe extern "system" fn Count<Impl: IWSCProductList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IWSCProductList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSCProductList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWscProduct_Impl: Sized + super::Com::IDispatch_Impl {
    fn ProductName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ProductState(&mut self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_STATE>;
    fn SignatureStatus(&mut self) -> ::windows::core::Result<WSC_SECURITY_SIGNATURE_STATUS>;
    fn RemediationPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ProductStateTimestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ProductGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ProductIsDefault(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWscProduct_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWscProduct_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWscProduct_Vtbl {
        unsafe extern "system" fn ProductName<Impl: IWscProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductState<Impl: IWscProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut WSC_SECURITY_PRODUCT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductState() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignatureStatus<Impl: IWscProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut WSC_SECURITY_SIGNATURE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignatureStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemediationPath<Impl: IWscProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemediationPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductStateTimestamp<Impl: IWscProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductStateTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductGuid<Impl: IWscProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductIsDefault<Impl: IWscProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductIsDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ProductName: ProductName::<Impl, IMPL_OFFSET>,
            ProductState: ProductState::<Impl, IMPL_OFFSET>,
            SignatureStatus: SignatureStatus::<Impl, IMPL_OFFSET>,
            RemediationPath: RemediationPath::<Impl, IMPL_OFFSET>,
            ProductStateTimestamp: ProductStateTimestamp::<Impl, IMPL_OFFSET>,
            ProductGuid: ProductGuid::<Impl, IMPL_OFFSET>,
            ProductIsDefault: ProductIsDefault::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWscProduct as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWscProduct2_Impl: Sized + super::Com::IDispatch_Impl + IWscProduct_Impl {
    fn AntivirusScanSubstatus(&mut self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn AntivirusSettingsSubstatus(&mut self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn AntivirusProtectionUpdateSubstatus(&mut self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn FirewallDomainProfileSubstatus(&mut self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn FirewallPrivateProfileSubstatus(&mut self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn FirewallPublicProfileSubstatus(&mut self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWscProduct2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWscProduct2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWscProduct2_Vtbl {
        unsafe extern "system" fn AntivirusScanSubstatus<Impl: IWscProduct2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AntivirusScanSubstatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pestatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AntivirusSettingsSubstatus<Impl: IWscProduct2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AntivirusSettingsSubstatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pestatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AntivirusProtectionUpdateSubstatus<Impl: IWscProduct2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AntivirusProtectionUpdateSubstatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pestatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirewallDomainProfileSubstatus<Impl: IWscProduct2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirewallDomainProfileSubstatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pestatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirewallPrivateProfileSubstatus<Impl: IWscProduct2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirewallPrivateProfileSubstatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pestatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirewallPublicProfileSubstatus<Impl: IWscProduct2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirewallPublicProfileSubstatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pestatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWscProduct_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AntivirusScanSubstatus: AntivirusScanSubstatus::<Impl, IMPL_OFFSET>,
            AntivirusSettingsSubstatus: AntivirusSettingsSubstatus::<Impl, IMPL_OFFSET>,
            AntivirusProtectionUpdateSubstatus: AntivirusProtectionUpdateSubstatus::<Impl, IMPL_OFFSET>,
            FirewallDomainProfileSubstatus: FirewallDomainProfileSubstatus::<Impl, IMPL_OFFSET>,
            FirewallPrivateProfileSubstatus: FirewallPrivateProfileSubstatus::<Impl, IMPL_OFFSET>,
            FirewallPublicProfileSubstatus: FirewallPublicProfileSubstatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWscProduct2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWscProduct3_Impl: Sized + super::Com::IDispatch_Impl + IWscProduct_Impl + IWscProduct2_Impl {
    fn AntivirusDaysUntilExpired(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWscProduct3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWscProduct3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWscProduct3_Vtbl {
        unsafe extern "system" fn AntivirusDaysUntilExpired<Impl: IWscProduct3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdays: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AntivirusDaysUntilExpired() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwdays = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWscProduct2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AntivirusDaysUntilExpired: AntivirusDaysUntilExpired::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWscProduct3 as ::windows::core::Interface>::IID
    }
}
