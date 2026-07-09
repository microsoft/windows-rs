pub const BITS_COST_OPTION_IGNORE_CONGESTION: u32 = 2147483648;
pub const BITS_COST_STATE_BELOW_CAP: u32 = 4;
pub const BITS_COST_STATE_CAPPED_USAGE_UNKNOWN: u32 = 2;
pub const BITS_COST_STATE_NEAR_CAP: u32 = 8;
pub const BITS_COST_STATE_OVERCAP_CHARGED: u32 = 16;
pub const BITS_COST_STATE_OVERCAP_THROTTLED: u32 = 32;
pub const BITS_COST_STATE_RESERVED: u32 = 1073741824;
pub const BITS_COST_STATE_ROAMING: u32 = 128;
pub const BITS_COST_STATE_TRANSFER_ALWAYS: i32 = -2147483393;
pub const BITS_COST_STATE_TRANSFER_NOT_ROAMING: i32 = -2147483521;
pub const BITS_COST_STATE_TRANSFER_NO_SURCHARGE: i32 = -2147483537;
pub const BITS_COST_STATE_TRANSFER_STANDARD: i32 = -2147483545;
pub const BITS_COST_STATE_TRANSFER_UNRESTRICTED: i32 = -2147483615;
pub const BITS_COST_STATE_UNRESTRICTED: u32 = 1;
pub const BITS_COST_STATE_USAGE_BASED: u32 = 64;
pub type BITS_FILE_PROPERTY_ID = i32;
pub const BITS_FILE_PROPERTY_ID_HTTP_RESPONSE_HEADERS: BITS_FILE_PROPERTY_ID = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub union BITS_FILE_PROPERTY_VALUE {
    pub String: windows_core::PWSTR,
}
impl Default for BITS_FILE_PROPERTY_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BITS_JOB_PROPERTY_DYNAMIC_CONTENT: BITS_JOB_PROPERTY_ID = 3;
pub const BITS_JOB_PROPERTY_HIGH_PERFORMANCE: BITS_JOB_PROPERTY_ID = 4;
pub type BITS_JOB_PROPERTY_ID = i32;
pub const BITS_JOB_PROPERTY_ID_COST_FLAGS: BITS_JOB_PROPERTY_ID = 1;
pub const BITS_JOB_PROPERTY_MAX_DOWNLOAD_SIZE: BITS_JOB_PROPERTY_ID = 5;
pub const BITS_JOB_PROPERTY_MINIMUM_NOTIFICATION_INTERVAL_MS: BITS_JOB_PROPERTY_ID = 9;
pub const BITS_JOB_PROPERTY_NOTIFICATION_CLSID: BITS_JOB_PROPERTY_ID = 2;
pub const BITS_JOB_PROPERTY_ON_DEMAND_MODE: BITS_JOB_PROPERTY_ID = 10;
pub const BITS_JOB_PROPERTY_USE_STORED_CREDENTIALS: BITS_JOB_PROPERTY_ID = 7;
#[repr(C)]
#[cfg(feature = "Win32_bits1_5")]
#[derive(Clone, Copy)]
pub union BITS_JOB_PROPERTY_VALUE {
    pub Dword: u32,
    pub ClsID: windows_core::GUID,
    pub Enable: windows_core::BOOL,
    pub Uint64: u64,
    pub Target: super::bits1_5::BG_AUTH_TARGET,
}
#[cfg(feature = "Win32_bits1_5")]
impl Default for BITS_JOB_PROPERTY_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type BITS_JOB_TRANSFER_POLICY = i32;
pub const BITS_JOB_TRANSFER_POLICY_ALWAYS: BITS_JOB_TRANSFER_POLICY = -2147483393;
pub const BITS_JOB_TRANSFER_POLICY_NOT_ROAMING: BITS_JOB_TRANSFER_POLICY = -2147483521;
pub const BITS_JOB_TRANSFER_POLICY_NO_SURCHARGE: BITS_JOB_TRANSFER_POLICY = -2147483537;
pub const BITS_JOB_TRANSFER_POLICY_STANDARD: BITS_JOB_TRANSFER_POLICY = -2147483545;
pub const BITS_JOB_TRANSFER_POLICY_UNRESTRICTED: BITS_JOB_TRANSFER_POLICY = -2147483615;
pub const BackgroundCopyManager5_0: windows_core::GUID = windows_core::GUID::from_u128(0x1ecca34c_e88a_44e3_8d6a_8921bde9e452);
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0", feature = "Win32_bits4_0"))]
windows_core::imp::define_interface!(IBackgroundCopyFile5, IBackgroundCopyFile5_Vtbl, 0x85c1657f_dafc_40e8_8834_df18ea25717e);
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0", feature = "Win32_bits4_0"))]
impl core::ops::Deref for IBackgroundCopyFile5 {
    type Target = super::bits4_0::IBackgroundCopyFile4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0", feature = "Win32_bits4_0"))]
windows_core::imp::interface_hierarchy!(IBackgroundCopyFile5, windows_core::IUnknown, super::bits::IBackgroundCopyFile, super::bits2_0::IBackgroundCopyFile2, super::bits3_0::IBackgroundCopyFile3, super::bits4_0::IBackgroundCopyFile4);
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0", feature = "Win32_bits4_0"))]
impl IBackgroundCopyFile5 {
    pub unsafe fn SetProperty(&self, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: BITS_FILE_PROPERTY_VALUE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), propertyid, core::mem::transmute(propertyvalue)) }
    }
    pub unsafe fn GetProperty(&self, propertyid: BITS_FILE_PROPERTY_ID) -> windows_core::Result<BITS_FILE_PROPERTY_VALUE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), propertyid, &mut result__).map(|| result__)
        }
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0", feature = "Win32_bits4_0"))]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyFile5_Vtbl {
    pub base__: super::bits4_0::IBackgroundCopyFile4_Vtbl,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, BITS_FILE_PROPERTY_ID, BITS_FILE_PROPERTY_VALUE) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, BITS_FILE_PROPERTY_ID, *mut BITS_FILE_PROPERTY_VALUE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0", feature = "Win32_bits4_0"))]
pub trait IBackgroundCopyFile5_Impl: super::bits4_0::IBackgroundCopyFile4_Impl {
    fn SetProperty(&self, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: &BITS_FILE_PROPERTY_VALUE) -> windows_core::Result<()>;
    fn GetProperty(&self, propertyid: BITS_FILE_PROPERTY_ID) -> windows_core::Result<BITS_FILE_PROPERTY_VALUE>;
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0", feature = "Win32_bits4_0"))]
impl IBackgroundCopyFile5_Vtbl {
    pub const fn new<Identity: IBackgroundCopyFile5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetProperty<Identity: IBackgroundCopyFile5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: BITS_FILE_PROPERTY_VALUE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyFile5_Impl::SetProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute(&propertyvalue)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IBackgroundCopyFile5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: BITS_FILE_PROPERTY_ID, propertyvalue: *mut BITS_FILE_PROPERTY_VALUE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyFile5_Impl::GetProperty(this, core::mem::transmute_copy(&propertyid)) {
                    Ok(ok__) => {
                        propertyvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::bits4_0::IBackgroundCopyFile4_Vtbl::new::<Identity, OFFSET>(),
            SetProperty: SetProperty::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyFile5 as windows_core::Interface>::IID || iid == &<super::bits::IBackgroundCopyFile as windows_core::Interface>::IID || iid == &<super::bits2_0::IBackgroundCopyFile2 as windows_core::Interface>::IID || iid == &<super::bits3_0::IBackgroundCopyFile3 as windows_core::Interface>::IID || iid == &<super::bits4_0::IBackgroundCopyFile4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits2_0", feature = "Win32_bits3_0", feature = "Win32_bits4_0"))]
impl windows_core::RuntimeName for IBackgroundCopyFile5 {}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5", feature = "Win32_bits2_0", feature = "Win32_bits3_0"))]
windows_core::imp::define_interface!(IBackgroundCopyJob5, IBackgroundCopyJob5_Vtbl, 0xe847030c_bbba_4657_af6d_484aa42bf1fe);
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5", feature = "Win32_bits2_0", feature = "Win32_bits3_0"))]
impl core::ops::Deref for IBackgroundCopyJob5 {
    type Target = super::bits3_0::IBackgroundCopyJob4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5", feature = "Win32_bits2_0", feature = "Win32_bits3_0"))]
windows_core::imp::interface_hierarchy!(IBackgroundCopyJob5, windows_core::IUnknown, super::bits::IBackgroundCopyJob, super::bits1_5::IBackgroundCopyJob2, super::bits2_0::IBackgroundCopyJob3, super::bits3_0::IBackgroundCopyJob4);
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5", feature = "Win32_bits2_0", feature = "Win32_bits3_0"))]
impl IBackgroundCopyJob5 {
    pub unsafe fn SetProperty(&self, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: BITS_JOB_PROPERTY_VALUE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), propertyid, core::mem::transmute(propertyvalue)) }
    }
    pub unsafe fn GetProperty(&self, propertyid: BITS_JOB_PROPERTY_ID) -> windows_core::Result<BITS_JOB_PROPERTY_VALUE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), propertyid, &mut result__).map(|| result__)
        }
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5", feature = "Win32_bits2_0", feature = "Win32_bits3_0"))]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJob5_Vtbl {
    pub base__: super::bits3_0::IBackgroundCopyJob4_Vtbl,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, BITS_JOB_PROPERTY_ID, BITS_JOB_PROPERTY_VALUE) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, BITS_JOB_PROPERTY_ID, *mut BITS_JOB_PROPERTY_VALUE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5", feature = "Win32_bits2_0", feature = "Win32_bits3_0", feature = "Win32_minwindef", feature = "Win32_rpcndr"))]
pub trait IBackgroundCopyJob5_Impl: super::bits3_0::IBackgroundCopyJob4_Impl {
    fn SetProperty(&self, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: &BITS_JOB_PROPERTY_VALUE) -> windows_core::Result<()>;
    fn GetProperty(&self, propertyid: BITS_JOB_PROPERTY_ID) -> windows_core::Result<BITS_JOB_PROPERTY_VALUE>;
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5", feature = "Win32_bits2_0", feature = "Win32_bits3_0", feature = "Win32_minwindef", feature = "Win32_rpcndr"))]
impl IBackgroundCopyJob5_Vtbl {
    pub const fn new<Identity: IBackgroundCopyJob5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetProperty<Identity: IBackgroundCopyJob5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: BITS_JOB_PROPERTY_VALUE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJob5_Impl::SetProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute(&propertyvalue)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IBackgroundCopyJob5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: BITS_JOB_PROPERTY_ID, propertyvalue: *mut BITS_JOB_PROPERTY_VALUE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJob5_Impl::GetProperty(this, core::mem::transmute_copy(&propertyid)) {
                    Ok(ok__) => {
                        propertyvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::bits3_0::IBackgroundCopyJob4_Vtbl::new::<Identity, OFFSET>(),
            SetProperty: SetProperty::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJob5 as windows_core::Interface>::IID || iid == &<super::bits::IBackgroundCopyJob as windows_core::Interface>::IID || iid == &<super::bits1_5::IBackgroundCopyJob2 as windows_core::Interface>::IID || iid == &<super::bits2_0::IBackgroundCopyJob3 as windows_core::Interface>::IID || iid == &<super::bits3_0::IBackgroundCopyJob4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_bits", feature = "Win32_bits1_5", feature = "Win32_bits2_0", feature = "Win32_bits3_0", feature = "Win32_minwindef", feature = "Win32_rpcndr"))]
impl windows_core::RuntimeName for IBackgroundCopyJob5 {}
