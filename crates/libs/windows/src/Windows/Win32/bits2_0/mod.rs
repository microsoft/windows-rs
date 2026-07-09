pub const BG_COPY_FILE_ALL: u32 = 15;
pub const BG_COPY_FILE_DACL: u32 = 4;
pub const BG_COPY_FILE_GROUP: u32 = 2;
pub const BG_COPY_FILE_OWNER: u32 = 1;
pub const BG_COPY_FILE_SACL: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BG_FILE_RANGE {
    pub InitialOffset: u64,
    pub Length: u64,
}
pub const BG_LENGTH_TO_EOF: i32 = -1;
pub const BackgroundCopyManager2_0: windows_core::GUID = windows_core::GUID::from_u128(0x6d18ad12_bde3_4393_b311_099c346e6df9);
#[cfg(feature = "Win32_bits")]
windows_core::imp::define_interface!(IBackgroundCopyFile2, IBackgroundCopyFile2_Vtbl, 0x83e81b93_0873_474d_8a8c_f2018b1a939c);
#[cfg(feature = "Win32_bits")]
impl core::ops::Deref for IBackgroundCopyFile2 {
    type Target = super::bits::IBackgroundCopyFile;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_bits")]
windows_core::imp::interface_hierarchy!(IBackgroundCopyFile2, windows_core::IUnknown, super::bits::IBackgroundCopyFile);
#[cfg(feature = "Win32_bits")]
impl IBackgroundCopyFile2 {
    pub unsafe fn GetFileRanges(&self, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFileRanges)(windows_core::Interface::as_raw(self), rangecount as _, ranges as _) }
    }
    pub unsafe fn SetRemoteName<P0>(&self, val: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRemoteName)(windows_core::Interface::as_raw(self), val.param().abi()) }
    }
}
#[cfg(feature = "Win32_bits")]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile2_Vtbl {
    pub base__: super::bits::IBackgroundCopyFile_Vtbl,
    pub GetFileRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut BG_FILE_RANGE) -> windows_core::HRESULT,
    pub SetRemoteName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_bits")]
pub trait IBackgroundCopyFile2_Impl: super::bits::IBackgroundCopyFile_Impl {
    fn GetFileRanges(&self, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> windows_core::Result<()>;
    fn SetRemoteName(&self, val: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_bits")]
impl IBackgroundCopyFile2_Vtbl {
    pub const fn new<Identity: IBackgroundCopyFile2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFileRanges<Identity: IBackgroundCopyFile2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rangecount: *mut u32, ranges: *mut *mut BG_FILE_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile2_Impl::GetFileRanges(this, core::mem::transmute_copy(&rangecount), core::mem::transmute_copy(&ranges)).into()
            }
        }
        unsafe extern "system" fn SetRemoteName<Identity: IBackgroundCopyFile2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile2_Impl::SetRemoteName(this, core::mem::transmute(&val)).into()
            }
        }
        Self {
            base__: super::bits::IBackgroundCopyFile_Vtbl::new::<Identity, OFFSET>(),
            GetFileRanges: GetFileRanges::<Identity, OFFSET>,
            SetRemoteName: SetRemoteName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyFile2 as windows_core::Interface>::IID || iid == &<super::bits::IBackgroundCopyFile as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_bits")]
impl windows_core::RuntimeName for IBackgroundCopyFile2 {}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5"))]
windows_core::imp::define_interface!(IBackgroundCopyJob3, IBackgroundCopyJob3_Vtbl, 0x443c8934_90ff_48ed_bcde_26f5c7450042);
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5"))]
impl core::ops::Deref for IBackgroundCopyJob3 {
    type Target = super::bits1_5::IBackgroundCopyJob2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5"))]
windows_core::imp::interface_hierarchy!(IBackgroundCopyJob3, windows_core::IUnknown, super::bits::IBackgroundCopyJob, super::bits1_5::IBackgroundCopyJob2);
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5"))]
impl IBackgroundCopyJob3 {
    pub unsafe fn ReplaceRemotePrefix<P0, P1>(&self, oldprefix: P0, newprefix: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReplaceRemotePrefix)(windows_core::Interface::as_raw(self), oldprefix.param().abi(), newprefix.param().abi()) }
    }
    pub unsafe fn AddFileWithRanges<P0, P1>(&self, remoteurl: P0, localname: P1, rangecount: u32, ranges: *const BG_FILE_RANGE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddFileWithRanges)(windows_core::Interface::as_raw(self), remoteurl.param().abi(), localname.param().abi(), rangecount, ranges) }
    }
    pub unsafe fn SetFileACLFlags(&self, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFileACLFlags)(windows_core::Interface::as_raw(self), flags) }
    }
    pub unsafe fn GetFileACLFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileACLFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5"))]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJob3_Vtbl {
    pub base__: super::bits1_5::IBackgroundCopyJob2_Vtbl,
    pub ReplaceRemotePrefix: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub AddFileWithRanges: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32, *const BG_FILE_RANGE) -> windows_core::HRESULT,
    pub SetFileACLFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetFileACLFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5", feature = "Win32_minwindef", feature = "Win32_rpcndr"))]
pub trait IBackgroundCopyJob3_Impl: super::bits1_5::IBackgroundCopyJob2_Impl {
    fn ReplaceRemotePrefix(&self, oldprefix: &windows_core::PCWSTR, newprefix: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddFileWithRanges(&self, remoteurl: &windows_core::PCWSTR, localname: &windows_core::PCWSTR, rangecount: u32, ranges: *const BG_FILE_RANGE) -> windows_core::Result<()>;
    fn SetFileACLFlags(&self, flags: u32) -> windows_core::Result<()>;
    fn GetFileACLFlags(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5", feature = "Win32_minwindef", feature = "Win32_rpcndr"))]
impl IBackgroundCopyJob3_Vtbl {
    pub const fn new<Identity: IBackgroundCopyJob3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReplaceRemotePrefix<Identity: IBackgroundCopyJob3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldprefix: windows_core::PCWSTR, newprefix: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob3_Impl::ReplaceRemotePrefix(this, core::mem::transmute(&oldprefix), core::mem::transmute(&newprefix)).into()
            }
        }
        unsafe extern "system" fn AddFileWithRanges<Identity: IBackgroundCopyJob3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteurl: windows_core::PCWSTR, localname: windows_core::PCWSTR, rangecount: u32, ranges: *const BG_FILE_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob3_Impl::AddFileWithRanges(this, core::mem::transmute(&remoteurl), core::mem::transmute(&localname), core::mem::transmute_copy(&rangecount), core::mem::transmute_copy(&ranges)).into()
            }
        }
        unsafe extern "system" fn SetFileACLFlags<Identity: IBackgroundCopyJob3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob3_Impl::SetFileACLFlags(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetFileACLFlags<Identity: IBackgroundCopyJob3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJob3_Impl::GetFileACLFlags(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::bits1_5::IBackgroundCopyJob2_Vtbl::new::<Identity, OFFSET>(),
            ReplaceRemotePrefix: ReplaceRemotePrefix::<Identity, OFFSET>,
            AddFileWithRanges: AddFileWithRanges::<Identity, OFFSET>,
            SetFileACLFlags: SetFileACLFlags::<Identity, OFFSET>,
            GetFileACLFlags: GetFileACLFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJob3 as windows_core::Interface>::IID || iid == &<super::bits::IBackgroundCopyJob as windows_core::Interface>::IID || iid == &<super::bits1_5::IBackgroundCopyJob2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5", feature = "Win32_minwindef", feature = "Win32_rpcndr"))]
impl windows_core::RuntimeName for IBackgroundCopyJob3 {}
