#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSCDefaultProduct_Impl: Sized + super::Com::IDispatch_Impl {
    fn SetDefaultProduct(&self, etype: SECURITY_PRODUCT_TYPE, pguid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWSCDefaultProduct {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSCDefaultProduct_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSCDefaultProduct_Impl, const OFFSET: isize>() -> IWSCDefaultProduct_Vtbl {
        unsafe extern "system" fn SetDefaultProduct<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSCDefaultProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, etype: SECURITY_PRODUCT_TYPE, pguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultProduct(::core::mem::transmute_copy(&etype), ::core::mem::transmute(&pguid)).into()
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), SetDefaultProduct: SetDefaultProduct::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSCDefaultProduct as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSCProductList_Impl: Sized + super::Com::IDispatch_Impl {
    fn Initialize(&self, provider: WSC_SECURITY_PROVIDER) -> ::windows::core::Result<()>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn get_Item(&self, index: u32) -> ::windows::core::Result<IWscProduct>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWSCProductList {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSCProductList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSCProductList_Impl, const OFFSET: isize>() -> IWSCProductList_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSCProductList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: WSC_SECURITY_PROVIDER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&provider)).into()
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSCProductList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSCProductList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSCProductList as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWscProduct_Impl: Sized + super::Com::IDispatch_Impl {
    fn ProductName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ProductState(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_STATE>;
    fn SignatureStatus(&self) -> ::windows::core::Result<WSC_SECURITY_SIGNATURE_STATUS>;
    fn RemediationPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ProductStateTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ProductGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ProductIsDefault(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWscProduct {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWscProduct_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: isize>() -> IWscProduct_Vtbl {
        unsafe extern "system" fn ProductName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProductName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut WSC_SECURITY_PRODUCT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProductState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignatureStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut WSC_SECURITY_SIGNATURE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SignatureStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemediationPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RemediationPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductStateTimestamp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProductStateTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProductGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductIsDefault<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProductIsDefault() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ProductName: ProductName::<Identity, Impl, OFFSET>,
            ProductState: ProductState::<Identity, Impl, OFFSET>,
            SignatureStatus: SignatureStatus::<Identity, Impl, OFFSET>,
            RemediationPath: RemediationPath::<Identity, Impl, OFFSET>,
            ProductStateTimestamp: ProductStateTimestamp::<Identity, Impl, OFFSET>,
            ProductGuid: ProductGuid::<Identity, Impl, OFFSET>,
            ProductIsDefault: ProductIsDefault::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWscProduct as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWscProduct2_Impl: Sized + super::Com::IDispatch_Impl + IWscProduct_Impl {
    fn AntivirusScanSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn AntivirusSettingsSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn AntivirusProtectionUpdateSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn FirewallDomainProfileSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn FirewallPrivateProfileSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
    fn FirewallPublicProfileSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWscProduct2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWscProduct2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct2_Impl, const OFFSET: isize>() -> IWscProduct2_Vtbl {
        unsafe extern "system" fn AntivirusScanSubstatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AntivirusScanSubstatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pestatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AntivirusSettingsSubstatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AntivirusSettingsSubstatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pestatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AntivirusProtectionUpdateSubstatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AntivirusProtectionUpdateSubstatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pestatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirewallDomainProfileSubstatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FirewallDomainProfileSubstatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pestatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirewallPrivateProfileSubstatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FirewallPrivateProfileSubstatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pestatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirewallPublicProfileSubstatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FirewallPublicProfileSubstatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pestatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWscProduct_Vtbl::new::<Identity, Impl, OFFSET>(),
            AntivirusScanSubstatus: AntivirusScanSubstatus::<Identity, Impl, OFFSET>,
            AntivirusSettingsSubstatus: AntivirusSettingsSubstatus::<Identity, Impl, OFFSET>,
            AntivirusProtectionUpdateSubstatus: AntivirusProtectionUpdateSubstatus::<Identity, Impl, OFFSET>,
            FirewallDomainProfileSubstatus: FirewallDomainProfileSubstatus::<Identity, Impl, OFFSET>,
            FirewallPrivateProfileSubstatus: FirewallPrivateProfileSubstatus::<Identity, Impl, OFFSET>,
            FirewallPublicProfileSubstatus: FirewallPublicProfileSubstatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWscProduct2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWscProduct as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWscProduct3_Impl: Sized + super::Com::IDispatch_Impl + IWscProduct_Impl + IWscProduct2_Impl {
    fn AntivirusDaysUntilExpired(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWscProduct3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWscProduct3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct3_Impl, const OFFSET: isize>() -> IWscProduct3_Vtbl {
        unsafe extern "system" fn AntivirusDaysUntilExpired<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWscProduct3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdays: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AntivirusDaysUntilExpired() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwdays, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IWscProduct2_Vtbl::new::<Identity, Impl, OFFSET>(), AntivirusDaysUntilExpired: AntivirusDaysUntilExpired::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWscProduct3 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWscProduct as ::windows::core::Interface>::IID || iid == &<IWscProduct2 as ::windows::core::Interface>::IID
    }
}
