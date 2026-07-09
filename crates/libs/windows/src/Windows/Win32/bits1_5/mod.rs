#[repr(C)]
#[derive(Clone, Copy)]
pub struct BG_AUTH_CREDENTIALS {
    pub Target: BG_AUTH_TARGET,
    pub Scheme: BG_AUTH_SCHEME,
    pub Credentials: BG_AUTH_CREDENTIALS_UNION,
}
impl Default for BG_AUTH_CREDENTIALS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union BG_AUTH_CREDENTIALS_UNION {
    pub Basic: BG_BASIC_CREDENTIALS,
}
impl Default for BG_AUTH_CREDENTIALS_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type BG_AUTH_SCHEME = i32;
pub const BG_AUTH_SCHEME_BASIC: BG_AUTH_SCHEME = 1;
pub const BG_AUTH_SCHEME_DIGEST: BG_AUTH_SCHEME = 2;
pub const BG_AUTH_SCHEME_NEGOTIATE: BG_AUTH_SCHEME = 4;
pub const BG_AUTH_SCHEME_NTLM: BG_AUTH_SCHEME = 3;
pub const BG_AUTH_SCHEME_PASSPORT: BG_AUTH_SCHEME = 5;
pub type BG_AUTH_TARGET = i32;
pub const BG_AUTH_TARGET_PROXY: BG_AUTH_TARGET = 2;
pub const BG_AUTH_TARGET_SERVER: BG_AUTH_TARGET = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BG_BASIC_CREDENTIALS {
    pub UserName: windows_core::PWSTR,
    pub Password: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BG_JOB_REPLY_PROGRESS {
    pub BytesTotal: u64,
    pub BytesTransferred: u64,
}
pub const BackgroundCopyManager1_5: windows_core::GUID = windows_core::GUID::from_u128(0xf087771f_d74f_4c1a_bb8a_e16aca9124ea);
#[cfg(feature = "Win32_bits")]
windows_core::imp::define_interface!(IBackgroundCopyJob2, IBackgroundCopyJob2_Vtbl, 0x54b50739_686f_45eb_9dff_d6a9a0faa9af);
#[cfg(feature = "Win32_bits")]
impl core::ops::Deref for IBackgroundCopyJob2 {
    type Target = super::bits::IBackgroundCopyJob;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_bits")]
windows_core::imp::interface_hierarchy!(IBackgroundCopyJob2, windows_core::IUnknown, super::bits::IBackgroundCopyJob);
#[cfg(feature = "Win32_bits")]
impl IBackgroundCopyJob2 {
    pub unsafe fn SetNotifyCmdLine<P0, P1>(&self, program: P0, parameters: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetNotifyCmdLine)(windows_core::Interface::as_raw(self), program.param().abi(), parameters.param().abi()) }
    }
    pub unsafe fn GetNotifyCmdLine(&self, pprogram: *mut windows_core::PWSTR, pparameters: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNotifyCmdLine)(windows_core::Interface::as_raw(self), pprogram as _, pparameters as _) }
    }
    pub unsafe fn GetReplyProgress(&self, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetReplyProgress)(windows_core::Interface::as_raw(self), pprogress as _) }
    }
    #[cfg(feature = "Win32_rpcndr")]
    pub unsafe fn GetReplyData(&self, ppbuffer: *mut *mut super::rpcndr::byte, plength: *mut u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetReplyData)(windows_core::Interface::as_raw(self), ppbuffer as _, plength as _) }
    }
    pub unsafe fn SetReplyFileName<P0>(&self, replyfilename: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetReplyFileName)(windows_core::Interface::as_raw(self), replyfilename.param().abi()) }
    }
    pub unsafe fn GetReplyFileName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReplyFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCredentials(&self) -> windows_core::Result<BG_AUTH_CREDENTIALS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetCredentials)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoveCredentials(&self, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveCredentials)(windows_core::Interface::as_raw(self), target, scheme) }
    }
}
#[cfg(feature = "Win32_bits")]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJob2_Vtbl {
    pub base__: super::bits::IBackgroundCopyJob_Vtbl,
    pub SetNotifyCmdLine: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetNotifyCmdLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetReplyProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_JOB_REPLY_PROGRESS) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_rpcndr")]
    pub GetReplyData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::rpcndr::byte, *mut u64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_rpcndr"))]
    GetReplyData: usize,
    pub SetReplyFileName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetReplyFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BG_AUTH_CREDENTIALS) -> windows_core::HRESULT,
    pub RemoveCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, BG_AUTH_TARGET, BG_AUTH_SCHEME) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_minwindef", feature = "Win32_rpcndr"))]
pub trait IBackgroundCopyJob2_Impl: super::bits::IBackgroundCopyJob_Impl {
    fn SetNotifyCmdLine(&self, program: &windows_core::PCWSTR, parameters: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetNotifyCmdLine(&self, pprogram: *mut windows_core::PWSTR, pparameters: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetReplyProgress(&self, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> windows_core::Result<()>;
    fn GetReplyData(&self, ppbuffer: *mut *mut super::rpcndr::byte, plength: *mut u64) -> windows_core::Result<()>;
    fn SetReplyFileName(&self, replyfilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetReplyFileName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetCredentials(&self) -> windows_core::Result<BG_AUTH_CREDENTIALS>;
    fn RemoveCredentials(&self, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_minwindef", feature = "Win32_rpcndr"))]
impl IBackgroundCopyJob2_Vtbl {
    pub const fn new<Identity: IBackgroundCopyJob2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetNotifyCmdLine<Identity: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, program: windows_core::PCWSTR, parameters: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob2_Impl::SetNotifyCmdLine(this, core::mem::transmute(&program), core::mem::transmute(&parameters)).into()
            }
        }
        unsafe extern "system" fn GetNotifyCmdLine<Identity: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprogram: *mut windows_core::PWSTR, pparameters: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob2_Impl::GetNotifyCmdLine(this, core::mem::transmute_copy(&pprogram), core::mem::transmute_copy(&pparameters)).into()
            }
        }
        unsafe extern "system" fn GetReplyProgress<Identity: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprogress: *mut BG_JOB_REPLY_PROGRESS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob2_Impl::GetReplyProgress(this, core::mem::transmute_copy(&pprogress)).into()
            }
        }
        unsafe extern "system" fn GetReplyData<Identity: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbuffer: *mut *mut super::rpcndr::byte, plength: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob2_Impl::GetReplyData(this, core::mem::transmute_copy(&ppbuffer), core::mem::transmute_copy(&plength)).into()
            }
        }
        unsafe extern "system" fn SetReplyFileName<Identity: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, replyfilename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob2_Impl::SetReplyFileName(this, core::mem::transmute(&replyfilename)).into()
            }
        }
        unsafe extern "system" fn GetReplyFileName<Identity: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preplyfilename: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJob2_Impl::GetReplyFileName(this) {
                    Ok(ok__) => {
                        preplyfilename.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCredentials<Identity: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, credentials: *mut BG_AUTH_CREDENTIALS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJob2_Impl::SetCredentials(this) {
                    Ok(ok__) => {
                        credentials.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveCredentials<Identity: IBackgroundCopyJob2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: BG_AUTH_TARGET, scheme: BG_AUTH_SCHEME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob2_Impl::RemoveCredentials(this, core::mem::transmute_copy(&target), core::mem::transmute_copy(&scheme)).into()
            }
        }
        Self {
            base__: super::bits::IBackgroundCopyJob_Vtbl::new::<Identity, OFFSET>(),
            SetNotifyCmdLine: SetNotifyCmdLine::<Identity, OFFSET>,
            GetNotifyCmdLine: GetNotifyCmdLine::<Identity, OFFSET>,
            GetReplyProgress: GetReplyProgress::<Identity, OFFSET>,
            GetReplyData: GetReplyData::<Identity, OFFSET>,
            SetReplyFileName: SetReplyFileName::<Identity, OFFSET>,
            GetReplyFileName: GetReplyFileName::<Identity, OFFSET>,
            SetCredentials: SetCredentials::<Identity, OFFSET>,
            RemoveCredentials: RemoveCredentials::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJob2 as windows_core::Interface>::IID || iid == &<super::bits::IBackgroundCopyJob as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_minwindef", feature = "Win32_rpcndr"))]
impl windows_core::RuntimeName for IBackgroundCopyJob2 {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBG_AUTH_CREDENTIALS(pub *mut BG_AUTH_CREDENTIALS);
impl PBG_AUTH_CREDENTIALS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PBG_AUTH_CREDENTIALS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBG_BASIC_CREDENTIALS(pub *mut BG_BASIC_CREDENTIALS);
impl PBG_BASIC_CREDENTIALS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PBG_BASIC_CREDENTIALS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
