pub const BG_DISABLE_BRANCH_CACHE: u32 = 4;
pub const BG_ENABLE_PEERCACHING_CLIENT: u32 = 1;
pub const BG_ENABLE_PEERCACHING_SERVER: u32 = 2;
pub const BG_JOB_DISABLE_BRANCH_CACHE: u32 = 4;
pub const BG_JOB_ENABLE_PEERCACHING_CLIENT: u32 = 1;
pub const BG_JOB_ENABLE_PEERCACHING_SERVER: u32 = 2;
pub const BackgroundCopyManager3_0: windows_core::GUID = windows_core::GUID::from_u128(0x659cdea7_489e_11d9_a9cd_000d56965251);
#[cfg(feature = "Win32_bits")]
windows_core::imp::define_interface!(IBackgroundCopyCallback2, IBackgroundCopyCallback2_Vtbl, 0x659cdeac_489e_11d9_a9cd_000d56965251);
#[cfg(feature = "Win32_bits")]
impl core::ops::Deref for IBackgroundCopyCallback2 {
    type Target = super::bits::IBackgroundCopyCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_bits")]
windows_core::imp::interface_hierarchy!(IBackgroundCopyCallback2, windows_core::IUnknown, super::bits::IBackgroundCopyCallback);
#[cfg(feature = "Win32_bits")]
impl IBackgroundCopyCallback2 {
    pub unsafe fn FileTransferred<P0, P1>(&self, pjob: P0, pfile: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::bits::IBackgroundCopyJob>,
        P1: windows_core::Param<super::bits::IBackgroundCopyFile>,
    {
        unsafe { (windows_core::Interface::vtable(self).FileTransferred)(windows_core::Interface::as_raw(self), pjob.param().abi(), pfile.param().abi()) }
    }
}
#[cfg(feature = "Win32_bits")]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyCallback2_Vtbl {
    pub base__: super::bits::IBackgroundCopyCallback_Vtbl,
    pub FileTransferred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_bits")]
pub trait IBackgroundCopyCallback2_Impl: super::bits::IBackgroundCopyCallback_Impl {
    fn FileTransferred(&self, pjob: windows_core::Ref<super::bits::IBackgroundCopyJob>, pfile: windows_core::Ref<super::bits::IBackgroundCopyFile>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_bits")]
impl IBackgroundCopyCallback2_Vtbl {
    pub const fn new<Identity: IBackgroundCopyCallback2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FileTransferred<Identity: IBackgroundCopyCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjob: *mut core::ffi::c_void, pfile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyCallback2_Impl::FileTransferred(this, core::mem::transmute_copy(&pjob), core::mem::transmute_copy(&pfile)).into()
            }
        }
        Self { base__: super::bits::IBackgroundCopyCallback_Vtbl::new::<Identity, OFFSET>(), FileTransferred: FileTransferred::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyCallback2 as windows_core::Interface>::IID || iid == &<super::bits::IBackgroundCopyCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_bits")]
impl windows_core::RuntimeName for IBackgroundCopyCallback2 {}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0"))]
windows_core::imp::define_interface!(IBackgroundCopyFile3, IBackgroundCopyFile3_Vtbl, 0x659cdeaa_489e_11d9_a9cd_000d56965251);
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0"))]
impl core::ops::Deref for IBackgroundCopyFile3 {
    type Target = super::bits2_0::IBackgroundCopyFile2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0"))]
windows_core::imp::interface_hierarchy!(IBackgroundCopyFile3, windows_core::IUnknown, super::bits::IBackgroundCopyFile, super::bits2_0::IBackgroundCopyFile2);
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0"))]
impl IBackgroundCopyFile3 {
    pub unsafe fn GetTemporaryName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTemporaryName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetValidationState(&self, state: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValidationState)(windows_core::Interface::as_raw(self), state.into()) }
    }
    pub unsafe fn GetValidationState(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValidationState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsDownloadedFromPeer(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsDownloadedFromPeer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0"))]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile3_Vtbl {
    pub base__: super::bits2_0::IBackgroundCopyFile2_Vtbl,
    pub GetTemporaryName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetValidationState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetValidationState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsDownloadedFromPeer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0"))]
pub trait IBackgroundCopyFile3_Impl: super::bits2_0::IBackgroundCopyFile2_Impl {
    fn GetTemporaryName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetValidationState(&self, state: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetValidationState(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsDownloadedFromPeer(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0"))]
impl IBackgroundCopyFile3_Vtbl {
    pub const fn new<Identity: IBackgroundCopyFile3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTemporaryName<Identity: IBackgroundCopyFile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilename: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyFile3_Impl::GetTemporaryName(this) {
                    Ok(ok__) => {
                        pfilename.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValidationState<Identity: IBackgroundCopyFile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile3_Impl::SetValidationState(this, core::mem::transmute_copy(&state)).into()
            }
        }
        unsafe extern "system" fn GetValidationState<Identity: IBackgroundCopyFile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyFile3_Impl::GetValidationState(this) {
                    Ok(ok__) => {
                        pstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsDownloadedFromPeer<Identity: IBackgroundCopyFile3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyFile3_Impl::IsDownloadedFromPeer(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::bits2_0::IBackgroundCopyFile2_Vtbl::new::<Identity, OFFSET>(),
            GetTemporaryName: GetTemporaryName::<Identity, OFFSET>,
            SetValidationState: SetValidationState::<Identity, OFFSET>,
            GetValidationState: GetValidationState::<Identity, OFFSET>,
            IsDownloadedFromPeer: IsDownloadedFromPeer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyFile3 as windows_core::Interface>::IID || iid == &<super::bits::IBackgroundCopyFile as windows_core::Interface>::IID || iid == &<super::bits2_0::IBackgroundCopyFile2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0"))]
impl windows_core::RuntimeName for IBackgroundCopyFile3 {}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5", feature = "Win32_bits2_0"))]
windows_core::imp::define_interface!(IBackgroundCopyJob4, IBackgroundCopyJob4_Vtbl, 0x659cdeae_489e_11d9_a9cd_000d56965251);
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5", feature = "Win32_bits2_0"))]
impl core::ops::Deref for IBackgroundCopyJob4 {
    type Target = super::bits2_0::IBackgroundCopyJob3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5", feature = "Win32_bits2_0"))]
windows_core::imp::interface_hierarchy!(IBackgroundCopyJob4, windows_core::IUnknown, super::bits::IBackgroundCopyJob, super::bits1_5::IBackgroundCopyJob2, super::bits2_0::IBackgroundCopyJob3);
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5", feature = "Win32_bits2_0"))]
impl IBackgroundCopyJob4 {
    pub unsafe fn SetPeerCachingFlags(&self, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPeerCachingFlags)(windows_core::Interface::as_raw(self), flags) }
    }
    pub unsafe fn GetPeerCachingFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPeerCachingFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOwnerIntegrityLevel(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOwnerIntegrityLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOwnerElevationState(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOwnerElevationState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaximumDownloadTime(&self, timeout: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaximumDownloadTime)(windows_core::Interface::as_raw(self), timeout) }
    }
    pub unsafe fn GetMaximumDownloadTime(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaximumDownloadTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5", feature = "Win32_bits2_0"))]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJob4_Vtbl {
    pub base__: super::bits2_0::IBackgroundCopyJob3_Vtbl,
    pub SetPeerCachingFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetPeerCachingFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetOwnerIntegrityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetOwnerElevationState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetMaximumDownloadTime: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaximumDownloadTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5", feature = "Win32_bits2_0", feature = "Win32_minwindef", feature = "Win32_rpcndr"))]
pub trait IBackgroundCopyJob4_Impl: super::bits2_0::IBackgroundCopyJob3_Impl {
    fn SetPeerCachingFlags(&self, flags: u32) -> windows_core::Result<()>;
    fn GetPeerCachingFlags(&self) -> windows_core::Result<u32>;
    fn GetOwnerIntegrityLevel(&self) -> windows_core::Result<u32>;
    fn GetOwnerElevationState(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetMaximumDownloadTime(&self, timeout: u32) -> windows_core::Result<()>;
    fn GetMaximumDownloadTime(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5", feature = "Win32_bits2_0", feature = "Win32_minwindef", feature = "Win32_rpcndr"))]
impl IBackgroundCopyJob4_Vtbl {
    pub const fn new<Identity: IBackgroundCopyJob4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPeerCachingFlags<Identity: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob4_Impl::SetPeerCachingFlags(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetPeerCachingFlags<Identity: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJob4_Impl::GetPeerCachingFlags(this) {
                    Ok(ok__) => {
                        pflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOwnerIntegrityLevel<Identity: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plevel: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJob4_Impl::GetOwnerIntegrityLevel(this) {
                    Ok(ok__) => {
                        plevel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOwnerElevationState<Identity: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pelevated: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJob4_Impl::GetOwnerElevationState(this) {
                    Ok(ok__) => {
                        pelevated.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaximumDownloadTime<Identity: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeout: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob4_Impl::SetMaximumDownloadTime(this, core::mem::transmute_copy(&timeout)).into()
            }
        }
        unsafe extern "system" fn GetMaximumDownloadTime<Identity: IBackgroundCopyJob4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptimeout: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJob4_Impl::GetMaximumDownloadTime(this) {
                    Ok(ok__) => {
                        ptimeout.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::bits2_0::IBackgroundCopyJob3_Vtbl::new::<Identity, OFFSET>(),
            SetPeerCachingFlags: SetPeerCachingFlags::<Identity, OFFSET>,
            GetPeerCachingFlags: GetPeerCachingFlags::<Identity, OFFSET>,
            GetOwnerIntegrityLevel: GetOwnerIntegrityLevel::<Identity, OFFSET>,
            GetOwnerElevationState: GetOwnerElevationState::<Identity, OFFSET>,
            SetMaximumDownloadTime: SetMaximumDownloadTime::<Identity, OFFSET>,
            GetMaximumDownloadTime: GetMaximumDownloadTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJob4 as windows_core::Interface>::IID || iid == &<super::bits::IBackgroundCopyJob as windows_core::Interface>::IID || iid == &<super::bits1_5::IBackgroundCopyJob2 as windows_core::Interface>::IID || iid == &<super::bits2_0::IBackgroundCopyJob3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5", feature = "Win32_bits2_0", feature = "Win32_minwindef", feature = "Win32_rpcndr"))]
impl windows_core::RuntimeName for IBackgroundCopyJob4 {}
windows_core::imp::define_interface!(IBitsPeer, IBitsPeer_Vtbl, 0x659cdea2_489e_11d9_a9cd_000d56965251);
windows_core::imp::interface_hierarchy!(IBitsPeer, windows_core::IUnknown);
impl IBitsPeer {
    pub unsafe fn GetPeerName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPeerName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsAuthenticated(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsAuthenticated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsAvailable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsAvailable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitsPeer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPeerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub IsAuthenticated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IBitsPeer_Impl: windows_core::IUnknownImpl {
    fn GetPeerName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn IsAuthenticated(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsAvailable(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IBitsPeer_Vtbl {
    pub const fn new<Identity: IBitsPeer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPeerName<Identity: IBitsPeer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitsPeer_Impl::GetPeerName(this) {
                    Ok(ok__) => {
                        pname.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsAuthenticated<Identity: IBitsPeer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauth: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitsPeer_Impl::IsAuthenticated(this) {
                    Ok(ok__) => {
                        pauth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsAvailable<Identity: IBitsPeer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ponline: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitsPeer_Impl::IsAvailable(this) {
                    Ok(ok__) => {
                        ponline.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPeerName: GetPeerName::<Identity, OFFSET>,
            IsAuthenticated: IsAuthenticated::<Identity, OFFSET>,
            IsAvailable: IsAvailable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBitsPeer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBitsPeer {}
windows_core::imp::define_interface!(IBitsPeerCacheAdministration, IBitsPeerCacheAdministration_Vtbl, 0x659cdead_489e_11d9_a9cd_000d56965251);
windows_core::imp::interface_hierarchy!(IBitsPeerCacheAdministration, windows_core::IUnknown);
impl IBitsPeerCacheAdministration {
    pub unsafe fn GetMaximumCacheSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaximumCacheSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaximumCacheSize(&self, bytes: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaximumCacheSize)(windows_core::Interface::as_raw(self), bytes) }
    }
    pub unsafe fn GetMaximumContentAge(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaximumContentAge)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaximumContentAge(&self, seconds: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaximumContentAge)(windows_core::Interface::as_raw(self), seconds) }
    }
    pub unsafe fn GetConfigurationFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConfigurationFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetConfigurationFlags(&self, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConfigurationFlags)(windows_core::Interface::as_raw(self), flags) }
    }
    pub unsafe fn EnumRecords(&self) -> windows_core::Result<IEnumBitsPeerCacheRecords> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumRecords)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRecord(&self, id: *const windows_core::GUID) -> windows_core::Result<IBitsPeerCacheRecord> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRecord)(windows_core::Interface::as_raw(self), id, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ClearRecords(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearRecords)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn DeleteRecord(&self, id: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteRecord)(windows_core::Interface::as_raw(self), id) }
    }
    pub unsafe fn DeleteUrl<P0>(&self, url: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteUrl)(windows_core::Interface::as_raw(self), url.param().abi()) }
    }
    pub unsafe fn EnumPeers(&self) -> windows_core::Result<IEnumBitsPeers> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumPeers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ClearPeers(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearPeers)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn DiscoverPeers(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DiscoverPeers)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitsPeerCacheAdministration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMaximumCacheSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaximumCacheSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaximumContentAge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaximumContentAge: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetConfigurationFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetConfigurationFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnumRecords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRecord: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearRecords: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteRecord: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub DeleteUrl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub EnumPeers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearPeers: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DiscoverPeers: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IBitsPeerCacheAdministration_Impl: windows_core::IUnknownImpl {
    fn GetMaximumCacheSize(&self) -> windows_core::Result<u32>;
    fn SetMaximumCacheSize(&self, bytes: u32) -> windows_core::Result<()>;
    fn GetMaximumContentAge(&self) -> windows_core::Result<u32>;
    fn SetMaximumContentAge(&self, seconds: u32) -> windows_core::Result<()>;
    fn GetConfigurationFlags(&self) -> windows_core::Result<u32>;
    fn SetConfigurationFlags(&self, flags: u32) -> windows_core::Result<()>;
    fn EnumRecords(&self) -> windows_core::Result<IEnumBitsPeerCacheRecords>;
    fn GetRecord(&self, id: *const windows_core::GUID) -> windows_core::Result<IBitsPeerCacheRecord>;
    fn ClearRecords(&self) -> windows_core::Result<()>;
    fn DeleteRecord(&self, id: *const windows_core::GUID) -> windows_core::Result<()>;
    fn DeleteUrl(&self, url: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnumPeers(&self) -> windows_core::Result<IEnumBitsPeers>;
    fn ClearPeers(&self) -> windows_core::Result<()>;
    fn DiscoverPeers(&self) -> windows_core::Result<()>;
}
impl IBitsPeerCacheAdministration_Vtbl {
    pub const fn new<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMaximumCacheSize<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbytes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitsPeerCacheAdministration_Impl::GetMaximumCacheSize(this) {
                    Ok(ok__) => {
                        pbytes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaximumCacheSize<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bytes: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheAdministration_Impl::SetMaximumCacheSize(this, core::mem::transmute_copy(&bytes)).into()
            }
        }
        unsafe extern "system" fn GetMaximumContentAge<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pseconds: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitsPeerCacheAdministration_Impl::GetMaximumContentAge(this) {
                    Ok(ok__) => {
                        pseconds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaximumContentAge<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheAdministration_Impl::SetMaximumContentAge(this, core::mem::transmute_copy(&seconds)).into()
            }
        }
        unsafe extern "system" fn GetConfigurationFlags<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitsPeerCacheAdministration_Impl::GetConfigurationFlags(this) {
                    Ok(ok__) => {
                        pflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetConfigurationFlags<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheAdministration_Impl::SetConfigurationFlags(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn EnumRecords<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitsPeerCacheAdministration_Impl::EnumRecords(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRecord<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *const windows_core::GUID, pprecord: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitsPeerCacheAdministration_Impl::GetRecord(this, core::mem::transmute_copy(&id)) {
                    Ok(ok__) => {
                        pprecord.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClearRecords<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheAdministration_Impl::ClearRecords(this).into()
            }
        }
        unsafe extern "system" fn DeleteRecord<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheAdministration_Impl::DeleteRecord(this, core::mem::transmute_copy(&id)).into()
            }
        }
        unsafe extern "system" fn DeleteUrl<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, url: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheAdministration_Impl::DeleteUrl(this, core::mem::transmute(&url)).into()
            }
        }
        unsafe extern "system" fn EnumPeers<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitsPeerCacheAdministration_Impl::EnumPeers(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClearPeers<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheAdministration_Impl::ClearPeers(this).into()
            }
        }
        unsafe extern "system" fn DiscoverPeers<Identity: IBitsPeerCacheAdministration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheAdministration_Impl::DiscoverPeers(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMaximumCacheSize: GetMaximumCacheSize::<Identity, OFFSET>,
            SetMaximumCacheSize: SetMaximumCacheSize::<Identity, OFFSET>,
            GetMaximumContentAge: GetMaximumContentAge::<Identity, OFFSET>,
            SetMaximumContentAge: SetMaximumContentAge::<Identity, OFFSET>,
            GetConfigurationFlags: GetConfigurationFlags::<Identity, OFFSET>,
            SetConfigurationFlags: SetConfigurationFlags::<Identity, OFFSET>,
            EnumRecords: EnumRecords::<Identity, OFFSET>,
            GetRecord: GetRecord::<Identity, OFFSET>,
            ClearRecords: ClearRecords::<Identity, OFFSET>,
            DeleteRecord: DeleteRecord::<Identity, OFFSET>,
            DeleteUrl: DeleteUrl::<Identity, OFFSET>,
            EnumPeers: EnumPeers::<Identity, OFFSET>,
            ClearPeers: ClearPeers::<Identity, OFFSET>,
            DiscoverPeers: DiscoverPeers::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBitsPeerCacheAdministration as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBitsPeerCacheAdministration {}
windows_core::imp::define_interface!(IBitsPeerCacheRecord, IBitsPeerCacheRecord_Vtbl, 0x659cdeaf_489e_11d9_a9cd_000d56965251);
windows_core::imp::interface_hierarchy!(IBitsPeerCacheRecord, windows_core::IUnknown);
impl IBitsPeerCacheRecord {
    pub unsafe fn GetId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOriginUrl(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOriginUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFileSize(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_minwindef")]
    pub unsafe fn GetFileModificationTime(&self) -> windows_core::Result<super::minwindef::FILETIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileModificationTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_minwindef")]
    pub unsafe fn GetLastAccessTime(&self) -> windows_core::Result<super::minwindef::FILETIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastAccessTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsFileValidated(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsFileValidated)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_bits2_0")]
    pub unsafe fn GetFileRanges(&self, prangecount: *mut u32, ppranges: *mut *mut super::bits2_0::BG_FILE_RANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFileRanges)(windows_core::Interface::as_raw(self), prangecount as _, ppranges as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitsPeerCacheRecord_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetOriginUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetFileSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_minwindef")]
    pub GetFileModificationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::minwindef::FILETIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_minwindef"))]
    GetFileModificationTime: usize,
    #[cfg(feature = "Win32_minwindef")]
    pub GetLastAccessTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::minwindef::FILETIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_minwindef"))]
    GetLastAccessTime: usize,
    pub IsFileValidated: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_bits2_0")]
    pub GetFileRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut super::bits2_0::BG_FILE_RANGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bits2_0"))]
    GetFileRanges: usize,
}
#[cfg(all(feature = "Win32_bits2_0", feature = "Win32_minwindef"))]
pub trait IBitsPeerCacheRecord_Impl: windows_core::IUnknownImpl {
    fn GetId(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetOriginUrl(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetFileSize(&self) -> windows_core::Result<u64>;
    fn GetFileModificationTime(&self) -> windows_core::Result<super::minwindef::FILETIME>;
    fn GetLastAccessTime(&self) -> windows_core::Result<super::minwindef::FILETIME>;
    fn IsFileValidated(&self) -> windows_core::Result<()>;
    fn GetFileRanges(&self, prangecount: *mut u32, ppranges: *mut *mut super::bits2_0::BG_FILE_RANGE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_bits2_0", feature = "Win32_minwindef"))]
impl IBitsPeerCacheRecord_Vtbl {
    pub const fn new<Identity: IBitsPeerCacheRecord_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetId<Identity: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitsPeerCacheRecord_Impl::GetId(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOriginUrl<Identity: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitsPeerCacheRecord_Impl::GetOriginUrl(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFileSize<Identity: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitsPeerCacheRecord_Impl::GetFileSize(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFileModificationTime<Identity: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::minwindef::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitsPeerCacheRecord_Impl::GetFileModificationTime(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLastAccessTime<Identity: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::minwindef::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitsPeerCacheRecord_Impl::GetLastAccessTime(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsFileValidated<Identity: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheRecord_Impl::IsFileValidated(this).into()
            }
        }
        unsafe extern "system" fn GetFileRanges<Identity: IBitsPeerCacheRecord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prangecount: *mut u32, ppranges: *mut *mut super::bits2_0::BG_FILE_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitsPeerCacheRecord_Impl::GetFileRanges(this, core::mem::transmute_copy(&prangecount), core::mem::transmute_copy(&ppranges)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetId: GetId::<Identity, OFFSET>,
            GetOriginUrl: GetOriginUrl::<Identity, OFFSET>,
            GetFileSize: GetFileSize::<Identity, OFFSET>,
            GetFileModificationTime: GetFileModificationTime::<Identity, OFFSET>,
            GetLastAccessTime: GetLastAccessTime::<Identity, OFFSET>,
            IsFileValidated: IsFileValidated::<Identity, OFFSET>,
            GetFileRanges: GetFileRanges::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBitsPeerCacheRecord as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_bits2_0", feature = "Win32_minwindef"))]
impl windows_core::RuntimeName for IBitsPeerCacheRecord {}
windows_core::imp::define_interface!(IEnumBitsPeerCacheRecords, IEnumBitsPeerCacheRecords_Vtbl, 0x659cdea4_489e_11d9_a9cd_000d56965251);
windows_core::imp::interface_hierarchy!(IEnumBitsPeerCacheRecords, windows_core::IUnknown);
impl IEnumBitsPeerCacheRecords {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut Option<IBitsPeerCacheRecord>, pceltfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgelt), pceltfetched as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumBitsPeerCacheRecords_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IEnumBitsPeerCacheRecords_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: windows_core::OutRef<IBitsPeerCacheRecord>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumBitsPeerCacheRecords>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl IEnumBitsPeerCacheRecords_Vtbl {
    pub const fn new<Identity: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBitsPeerCacheRecords_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBitsPeerCacheRecords_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBitsPeerCacheRecords_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumBitsPeerCacheRecords_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumBitsPeerCacheRecords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumBitsPeerCacheRecords_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pucount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumBitsPeerCacheRecords as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumBitsPeerCacheRecords {}
windows_core::imp::define_interface!(IEnumBitsPeers, IEnumBitsPeers_Vtbl, 0x659cdea5_489e_11d9_a9cd_000d56965251);
windows_core::imp::interface_hierarchy!(IEnumBitsPeers, windows_core::IUnknown);
impl IEnumBitsPeers {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut Option<IBitsPeer>, pceltfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgelt), pceltfetched as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumBitsPeers_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IEnumBitsPeers_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: windows_core::OutRef<IBitsPeer>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumBitsPeers>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl IEnumBitsPeers_Vtbl {
    pub const fn new<Identity: IEnumBitsPeers_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumBitsPeers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBitsPeers_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumBitsPeers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBitsPeers_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumBitsPeers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumBitsPeers_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumBitsPeers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumBitsPeers_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumBitsPeers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumBitsPeers_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pucount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumBitsPeers as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumBitsPeers {}
