pub const BG_TOKEN_LOCAL_FILE: u32 = 1;
pub const BG_TOKEN_NETWORK: u32 = 2;
pub const BackgroundCopyManager4_0: windows_core::GUID = windows_core::GUID::from_u128(0xbb6df56b_cace_11dc_9992_0019b93a3a84);
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0"))]
windows_core::imp::define_interface!(IBackgroundCopyFile4, IBackgroundCopyFile4_Vtbl, 0xef7e0655_7888_4960_b0e5_730846e03492);
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0"))]
impl core::ops::Deref for IBackgroundCopyFile4 {
    type Target = super::bits3_0::IBackgroundCopyFile3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0"))]
windows_core::imp::interface_hierarchy!(IBackgroundCopyFile4, windows_core::IUnknown, super::bits::IBackgroundCopyFile, super::bits2_0::IBackgroundCopyFile2, super::bits3_0::IBackgroundCopyFile3);
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0"))]
impl IBackgroundCopyFile4 {
    pub unsafe fn GetPeerDownloadStats(&self, pfromorigin: *mut u64, pfrompeers: *mut u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPeerDownloadStats)(windows_core::Interface::as_raw(self), pfromorigin as _, pfrompeers as _) }
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0"))]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile4_Vtbl {
    pub base__: super::bits3_0::IBackgroundCopyFile3_Vtbl,
    pub GetPeerDownloadStats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0"))]
pub trait IBackgroundCopyFile4_Impl: super::bits3_0::IBackgroundCopyFile3_Impl {
    fn GetPeerDownloadStats(&self, pfromorigin: *mut u64, pfrompeers: *mut u64) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0"))]
impl IBackgroundCopyFile4_Vtbl {
    pub const fn new<Identity: IBackgroundCopyFile4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPeerDownloadStats<Identity: IBackgroundCopyFile4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfromorigin: *mut u64, pfrompeers: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile4_Impl::GetPeerDownloadStats(this, core::mem::transmute_copy(&pfromorigin), core::mem::transmute_copy(&pfrompeers)).into()
            }
        }
        Self { base__: super::bits3_0::IBackgroundCopyFile3_Vtbl::new::<Identity, OFFSET>(), GetPeerDownloadStats: GetPeerDownloadStats::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyFile4 as windows_core::Interface>::IID || iid == &<super::bits::IBackgroundCopyFile as windows_core::Interface>::IID || iid == &<super::bits2_0::IBackgroundCopyFile2 as windows_core::Interface>::IID || iid == &<super::bits3_0::IBackgroundCopyFile3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0"))]
impl windows_core::RuntimeName for IBackgroundCopyFile4 {}
windows_core::imp::define_interface!(IBitsTokenOptions, IBitsTokenOptions_Vtbl, 0x9a2584c3_f7d2_457a_9a5e_22b67bffc7d2);
windows_core::imp::interface_hierarchy!(IBitsTokenOptions, windows_core::IUnknown);
impl IBitsTokenOptions {
    pub unsafe fn SetHelperTokenFlags(&self, usageflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHelperTokenFlags)(windows_core::Interface::as_raw(self), usageflags) }
    }
    pub unsafe fn GetHelperTokenFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHelperTokenFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHelperToken(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHelperToken)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ClearHelperToken(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearHelperToken)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetHelperTokenSid(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHelperTokenSid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitsTokenOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetHelperTokenFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetHelperTokenFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetHelperToken: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearHelperToken: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHelperTokenSid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IBitsTokenOptions_Impl: windows_core::IUnknownImpl {
    fn SetHelperTokenFlags(&self, usageflags: u32) -> windows_core::Result<()>;
    fn GetHelperTokenFlags(&self) -> windows_core::Result<u32>;
    fn SetHelperToken(&self) -> windows_core::Result<()>;
    fn ClearHelperToken(&self) -> windows_core::Result<()>;
    fn GetHelperTokenSid(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IBitsTokenOptions_Vtbl {
    pub const fn new<Identity: IBitsTokenOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetHelperTokenFlags<Identity: IBitsTokenOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usageflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsTokenOptions_Impl::SetHelperTokenFlags(this, core::mem::transmute_copy(&usageflags)).into()
            }
        }
        unsafe extern "system" fn GetHelperTokenFlags<Identity: IBitsTokenOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitsTokenOptions_Impl::GetHelperTokenFlags(this) {
                    Ok(ok__) => {
                        pflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHelperToken<Identity: IBitsTokenOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsTokenOptions_Impl::SetHelperToken(this).into()
            }
        }
        unsafe extern "system" fn ClearHelperToken<Identity: IBitsTokenOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsTokenOptions_Impl::ClearHelperToken(this).into()
            }
        }
        unsafe extern "system" fn GetHelperTokenSid<Identity: IBitsTokenOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitsTokenOptions_Impl::GetHelperTokenSid(this) {
                    Ok(ok__) => {
                        psid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetHelperTokenFlags: SetHelperTokenFlags::<Identity, OFFSET>,
            GetHelperTokenFlags: GetHelperTokenFlags::<Identity, OFFSET>,
            SetHelperToken: SetHelperToken::<Identity, OFFSET>,
            ClearHelperToken: ClearHelperToken::<Identity, OFFSET>,
            GetHelperTokenSid: GetHelperTokenSid::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBitsTokenOptions as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBitsTokenOptions {}
