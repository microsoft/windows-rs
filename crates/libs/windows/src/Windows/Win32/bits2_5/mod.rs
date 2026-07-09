pub type BG_CERT_STORE_LOCATION = i32;
pub const BG_CERT_STORE_LOCATION_CURRENT_SERVICE: BG_CERT_STORE_LOCATION = 2;
pub const BG_CERT_STORE_LOCATION_CURRENT_USER: BG_CERT_STORE_LOCATION = 0;
pub const BG_CERT_STORE_LOCATION_CURRENT_USER_GROUP_POLICY: BG_CERT_STORE_LOCATION = 5;
pub const BG_CERT_STORE_LOCATION_LOCAL_MACHINE: BG_CERT_STORE_LOCATION = 1;
pub const BG_CERT_STORE_LOCATION_LOCAL_MACHINE_ENTERPRISE: BG_CERT_STORE_LOCATION = 7;
pub const BG_CERT_STORE_LOCATION_LOCAL_MACHINE_GROUP_POLICY: BG_CERT_STORE_LOCATION = 6;
pub const BG_CERT_STORE_LOCATION_SERVICES: BG_CERT_STORE_LOCATION = 3;
pub const BG_CERT_STORE_LOCATION_USERS: BG_CERT_STORE_LOCATION = 4;
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_HTTPS_TO_HTTP: u32 = 2048;
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_REPORT: u32 = 256;
pub const BG_HTTP_REDIRECT_POLICY_ALLOW_SILENT: u32 = 0;
pub const BG_HTTP_REDIRECT_POLICY_DISALLOW: u32 = 512;
pub const BG_HTTP_REDIRECT_POLICY_MASK: u32 = 1792;
pub const BG_SSL_ENABLE_CRL_CHECK: u32 = 1;
pub const BG_SSL_IGNORE_CERT_CN_INVALID: u32 = 2;
pub const BG_SSL_IGNORE_CERT_DATE_INVALID: u32 = 4;
pub const BG_SSL_IGNORE_CERT_WRONG_USAGE: u32 = 16;
pub const BG_SSL_IGNORE_UNKNOWN_CA: u32 = 8;
pub const BackgroundCopyManager2_5: windows_core::GUID = windows_core::GUID::from_u128(0x03ca98d6_ff5d_49b8_abc6_03dd84127020);
windows_core::imp::define_interface!(IBackgroundCopyJobHttpOptions, IBackgroundCopyJobHttpOptions_Vtbl, 0xf1bd1079_9f01_4bdc_8036_f09b70095066);
windows_core::imp::interface_hierarchy!(IBackgroundCopyJobHttpOptions, windows_core::IUnknown);
impl IBackgroundCopyJobHttpOptions {
    #[cfg(feature = "Win32_rpcndr")]
    pub unsafe fn SetClientCertificateByID<P1>(&self, storelocation: BG_CERT_STORE_LOCATION, storename: P1, pcerthashblob: *const super::rpcndr::byte) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetClientCertificateByID)(windows_core::Interface::as_raw(self), storelocation, storename.param().abi(), pcerthashblob) }
    }
    pub unsafe fn SetClientCertificateByName<P1, P2>(&self, storelocation: BG_CERT_STORE_LOCATION, storename: P1, subjectname: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetClientCertificateByName)(windows_core::Interface::as_raw(self), storelocation, storename.param().abi(), subjectname.param().abi()) }
    }
    pub unsafe fn RemoveClientCertificate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveClientCertificate)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_rpcndr")]
    pub unsafe fn GetClientCertificate(&self, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut windows_core::PWSTR, ppcerthashblob: *mut *mut super::rpcndr::byte, psubjectname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetClientCertificate)(windows_core::Interface::as_raw(self), pstorelocation as _, pstorename as _, ppcerthashblob as _, psubjectname as _) }
    }
    pub unsafe fn SetCustomHeaders<P0>(&self, requestheaders: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCustomHeaders)(windows_core::Interface::as_raw(self), requestheaders.param().abi()) }
    }
    pub unsafe fn GetCustomHeaders(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCustomHeaders)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSecurityFlags(&self, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSecurityFlags)(windows_core::Interface::as_raw(self), flags) }
    }
    pub unsafe fn GetSecurityFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSecurityFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJobHttpOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_rpcndr")]
    pub SetClientCertificateByID: unsafe extern "system" fn(*mut core::ffi::c_void, BG_CERT_STORE_LOCATION, windows_core::PCWSTR, *const super::rpcndr::byte) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_rpcndr"))]
    SetClientCertificateByID: usize,
    pub SetClientCertificateByName: unsafe extern "system" fn(*mut core::ffi::c_void, BG_CERT_STORE_LOCATION, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RemoveClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_rpcndr")]
    pub GetClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_CERT_STORE_LOCATION, *mut windows_core::PWSTR, *mut *mut super::rpcndr::byte, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_rpcndr"))]
    GetClientCertificate: usize,
    pub SetCustomHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetCustomHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetSecurityFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetSecurityFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_rpcndr")]
pub trait IBackgroundCopyJobHttpOptions_Impl: windows_core::IUnknownImpl {
    fn SetClientCertificateByID(&self, storelocation: BG_CERT_STORE_LOCATION, storename: &windows_core::PCWSTR, pcerthashblob: *const super::rpcndr::byte) -> windows_core::Result<()>;
    fn SetClientCertificateByName(&self, storelocation: BG_CERT_STORE_LOCATION, storename: &windows_core::PCWSTR, subjectname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RemoveClientCertificate(&self) -> windows_core::Result<()>;
    fn GetClientCertificate(&self, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut windows_core::PWSTR, ppcerthashblob: *mut *mut super::rpcndr::byte, psubjectname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetCustomHeaders(&self, requestheaders: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetCustomHeaders(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetSecurityFlags(&self, flags: u32) -> windows_core::Result<()>;
    fn GetSecurityFlags(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_rpcndr")]
impl IBackgroundCopyJobHttpOptions_Vtbl {
    pub const fn new<Identity: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetClientCertificateByID<Identity: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storelocation: BG_CERT_STORE_LOCATION, storename: windows_core::PCWSTR, pcerthashblob: *const super::rpcndr::byte) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJobHttpOptions_Impl::SetClientCertificateByID(this, core::mem::transmute_copy(&storelocation), core::mem::transmute(&storename), core::mem::transmute_copy(&pcerthashblob)).into()
            }
        }
        unsafe extern "system" fn SetClientCertificateByName<Identity: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storelocation: BG_CERT_STORE_LOCATION, storename: windows_core::PCWSTR, subjectname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJobHttpOptions_Impl::SetClientCertificateByName(this, core::mem::transmute_copy(&storelocation), core::mem::transmute(&storename), core::mem::transmute(&subjectname)).into()
            }
        }
        unsafe extern "system" fn RemoveClientCertificate<Identity: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJobHttpOptions_Impl::RemoveClientCertificate(this).into()
            }
        }
        unsafe extern "system" fn GetClientCertificate<Identity: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstorelocation: *mut BG_CERT_STORE_LOCATION, pstorename: *mut windows_core::PWSTR, ppcerthashblob: *mut *mut super::rpcndr::byte, psubjectname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJobHttpOptions_Impl::GetClientCertificate(this, core::mem::transmute_copy(&pstorelocation), core::mem::transmute_copy(&pstorename), core::mem::transmute_copy(&ppcerthashblob), core::mem::transmute_copy(&psubjectname)).into()
            }
        }
        unsafe extern "system" fn SetCustomHeaders<Identity: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestheaders: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJobHttpOptions_Impl::SetCustomHeaders(this, core::mem::transmute(&requestheaders)).into()
            }
        }
        unsafe extern "system" fn GetCustomHeaders<Identity: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequestheaders: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJobHttpOptions_Impl::GetCustomHeaders(this) {
                    Ok(ok__) => {
                        prequestheaders.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSecurityFlags<Identity: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJobHttpOptions_Impl::SetSecurityFlags(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetSecurityFlags<Identity: IBackgroundCopyJobHttpOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJobHttpOptions_Impl::GetSecurityFlags(this) {
                    Ok(ok__) => {
                        pflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetClientCertificateByID: SetClientCertificateByID::<Identity, OFFSET>,
            SetClientCertificateByName: SetClientCertificateByName::<Identity, OFFSET>,
            RemoveClientCertificate: RemoveClientCertificate::<Identity, OFFSET>,
            GetClientCertificate: GetClientCertificate::<Identity, OFFSET>,
            SetCustomHeaders: SetCustomHeaders::<Identity, OFFSET>,
            GetCustomHeaders: GetCustomHeaders::<Identity, OFFSET>,
            SetSecurityFlags: SetSecurityFlags::<Identity, OFFSET>,
            GetSecurityFlags: GetSecurityFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJobHttpOptions as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_rpcndr")]
impl windows_core::RuntimeName for IBackgroundCopyJobHttpOptions {}
