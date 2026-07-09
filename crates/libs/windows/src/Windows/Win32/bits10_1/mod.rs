pub const BackgroundCopyManager10_1: windows_core::GUID = windows_core::GUID::from_u128(0x4bd3e4e1_7bd4_4a2b_9964_496400de5193);
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits3_0"))]
windows_core::imp::define_interface!(IBackgroundCopyCallback3, IBackgroundCopyCallback3_Vtbl, 0x98c97bd2_e32b_4ad8_a528_95fd8b16bd42);
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits3_0"))]
impl core::ops::Deref for IBackgroundCopyCallback3 {
    type Target = super::bits3_0::IBackgroundCopyCallback2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits3_0"))]
windows_core::imp::interface_hierarchy!(IBackgroundCopyCallback3, windows_core::IUnknown, super::bits::IBackgroundCopyCallback, super::bits3_0::IBackgroundCopyCallback2);
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits3_0"))]
impl IBackgroundCopyCallback3 {
    #[cfg(feature = "Win32_bits2_0")]
    pub unsafe fn FileRangesTransferred<P0, P1>(&self, job: P0, file: P1, rangecount: u32, ranges: *const super::bits2_0::BG_FILE_RANGE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::bits::IBackgroundCopyJob>,
        P1: windows_core::Param<super::bits::IBackgroundCopyFile>,
    {
        unsafe { (windows_core::Interface::vtable(self).FileRangesTransferred)(windows_core::Interface::as_raw(self), job.param().abi(), file.param().abi(), rangecount, ranges) }
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits3_0"))]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyCallback3_Vtbl {
    pub base__: super::bits3_0::IBackgroundCopyCallback2_Vtbl,
    #[cfg(feature = "Win32_bits2_0")]
    pub FileRangesTransferred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const super::bits2_0::BG_FILE_RANGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bits2_0"))]
    FileRangesTransferred: usize,
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0"))]
pub trait IBackgroundCopyCallback3_Impl: super::bits3_0::IBackgroundCopyCallback2_Impl {
    fn FileRangesTransferred(&self, job: windows_core::Ref<super::bits::IBackgroundCopyJob>, file: windows_core::Ref<super::bits::IBackgroundCopyFile>, rangecount: u32, ranges: *const super::bits2_0::BG_FILE_RANGE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0"))]
impl IBackgroundCopyCallback3_Vtbl {
    pub const fn new<Identity: IBackgroundCopyCallback3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FileRangesTransferred<Identity: IBackgroundCopyCallback3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, job: *mut core::ffi::c_void, file: *mut core::ffi::c_void, rangecount: u32, ranges: *const super::bits2_0::BG_FILE_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyCallback3_Impl::FileRangesTransferred(this, core::mem::transmute_copy(&job), core::mem::transmute_copy(&file), core::mem::transmute_copy(&rangecount), core::mem::transmute_copy(&ranges)).into()
            }
        }
        Self {
            base__: super::bits3_0::IBackgroundCopyCallback2_Vtbl::new::<Identity, OFFSET>(),
            FileRangesTransferred: FileRangesTransferred::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyCallback3 as windows_core::Interface>::IID || iid == &<super::bits::IBackgroundCopyCallback as windows_core::Interface>::IID || iid == &<super::bits3_0::IBackgroundCopyCallback2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0"))]
impl windows_core::RuntimeName for IBackgroundCopyCallback3 {}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0", feature = "Win32_bits4_0", feature = "Win32_bits5_0"))]
windows_core::imp::define_interface!(IBackgroundCopyFile6, IBackgroundCopyFile6_Vtbl, 0xcf6784f7_d677_49fd_9368_cb47aee9d1ad);
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0", feature = "Win32_bits4_0", feature = "Win32_bits5_0"))]
impl core::ops::Deref for IBackgroundCopyFile6 {
    type Target = super::bits5_0::IBackgroundCopyFile5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0", feature = "Win32_bits4_0", feature = "Win32_bits5_0"))]
windows_core::imp::interface_hierarchy!(IBackgroundCopyFile6, windows_core::IUnknown, super::bits::IBackgroundCopyFile, super::bits2_0::IBackgroundCopyFile2, super::bits3_0::IBackgroundCopyFile3, super::bits4_0::IBackgroundCopyFile4, super::bits5_0::IBackgroundCopyFile5);
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0", feature = "Win32_bits4_0", feature = "Win32_bits5_0"))]
impl IBackgroundCopyFile6 {
    pub unsafe fn UpdateDownloadPosition(&self, offset: u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateDownloadPosition)(windows_core::Interface::as_raw(self), offset) }
    }
    pub unsafe fn RequestFileRanges(&self, rangecount: u32, ranges: *const super::bits2_0::BG_FILE_RANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RequestFileRanges)(windows_core::Interface::as_raw(self), rangecount, ranges) }
    }
    pub unsafe fn GetFilledFileRanges(&self, rangecount: *mut u32, ranges: *mut *mut super::bits2_0::BG_FILE_RANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFilledFileRanges)(windows_core::Interface::as_raw(self), rangecount as _, ranges as _) }
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0", feature = "Win32_bits4_0", feature = "Win32_bits5_0"))]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile6_Vtbl {
    pub base__: super::bits5_0::IBackgroundCopyFile5_Vtbl,
    pub UpdateDownloadPosition: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub RequestFileRanges: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::bits2_0::BG_FILE_RANGE) -> windows_core::HRESULT,
    pub GetFilledFileRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut super::bits2_0::BG_FILE_RANGE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0", feature = "Win32_bits4_0", feature = "Win32_bits5_0"))]
pub trait IBackgroundCopyFile6_Impl: super::bits5_0::IBackgroundCopyFile5_Impl {
    fn UpdateDownloadPosition(&self, offset: u64) -> windows_core::Result<()>;
    fn RequestFileRanges(&self, rangecount: u32, ranges: *const super::bits2_0::BG_FILE_RANGE) -> windows_core::Result<()>;
    fn GetFilledFileRanges(&self, rangecount: *mut u32, ranges: *mut *mut super::bits2_0::BG_FILE_RANGE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0", feature = "Win32_bits4_0", feature = "Win32_bits5_0"))]
impl IBackgroundCopyFile6_Vtbl {
    pub const fn new<Identity: IBackgroundCopyFile6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UpdateDownloadPosition<Identity: IBackgroundCopyFile6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile6_Impl::UpdateDownloadPosition(this, core::mem::transmute_copy(&offset)).into()
            }
        }
        unsafe extern "system" fn RequestFileRanges<Identity: IBackgroundCopyFile6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rangecount: u32, ranges: *const super::bits2_0::BG_FILE_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile6_Impl::RequestFileRanges(this, core::mem::transmute_copy(&rangecount), core::mem::transmute_copy(&ranges)).into()
            }
        }
        unsafe extern "system" fn GetFilledFileRanges<Identity: IBackgroundCopyFile6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rangecount: *mut u32, ranges: *mut *mut super::bits2_0::BG_FILE_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile6_Impl::GetFilledFileRanges(this, core::mem::transmute_copy(&rangecount), core::mem::transmute_copy(&ranges)).into()
            }
        }
        Self {
            base__: super::bits5_0::IBackgroundCopyFile5_Vtbl::new::<Identity, OFFSET>(),
            UpdateDownloadPosition: UpdateDownloadPosition::<Identity, OFFSET>,
            RequestFileRanges: RequestFileRanges::<Identity, OFFSET>,
            GetFilledFileRanges: GetFilledFileRanges::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyFile6 as windows_core::Interface>::IID || iid == &<super::bits::IBackgroundCopyFile as windows_core::Interface>::IID || iid == &<super::bits2_0::IBackgroundCopyFile2 as windows_core::Interface>::IID || iid == &<super::bits3_0::IBackgroundCopyFile3 as windows_core::Interface>::IID || iid == &<super::bits4_0::IBackgroundCopyFile4 as windows_core::Interface>::IID || iid == &<super::bits5_0::IBackgroundCopyFile5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0", feature = "Win32_bits4_0", feature = "Win32_bits5_0"))]
impl windows_core::RuntimeName for IBackgroundCopyFile6 {}
