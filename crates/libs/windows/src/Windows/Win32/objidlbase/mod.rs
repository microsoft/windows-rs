pub type ACTIVATIONTYPE = i32;
pub const ACTIVATIONTYPE_FROM_DATA: ACTIVATIONTYPE = 2;
pub const ACTIVATIONTYPE_FROM_FILE: ACTIVATIONTYPE = 16;
pub const ACTIVATIONTYPE_FROM_MONIKER: ACTIVATIONTYPE = 1;
pub const ACTIVATIONTYPE_FROM_STORAGE: ACTIVATIONTYPE = 4;
pub const ACTIVATIONTYPE_FROM_STREAM: ACTIVATIONTYPE = 8;
pub const ACTIVATIONTYPE_UNCATEGORIZED: ACTIVATIONTYPE = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct APARTMENTID(pub u32);
pub type APTTYPE = i32;
pub type APTTYPEQUALIFIER = i32;
pub const APTTYPEQUALIFIER_APPLICATION_STA: APTTYPEQUALIFIER = 6;
pub const APTTYPEQUALIFIER_IMPLICIT_MTA: APTTYPEQUALIFIER = 1;
pub const APTTYPEQUALIFIER_NA_ON_IMPLICIT_MTA: APTTYPEQUALIFIER = 4;
pub const APTTYPEQUALIFIER_NA_ON_MAINSTA: APTTYPEQUALIFIER = 5;
pub const APTTYPEQUALIFIER_NA_ON_MTA: APTTYPEQUALIFIER = 2;
pub const APTTYPEQUALIFIER_NA_ON_STA: APTTYPEQUALIFIER = 3;
pub const APTTYPEQUALIFIER_NONE: APTTYPEQUALIFIER = 0;
pub const APTTYPEQUALIFIER_RESERVED_1: APTTYPEQUALIFIER = 7;
pub const APTTYPE_CURRENT: APTTYPE = -1;
pub const APTTYPE_MAINSTA: APTTYPE = 3;
pub const APTTYPE_MTA: APTTYPE = 1;
pub const APTTYPE_NA: APTTYPE = 2;
pub const APTTYPE_STA: APTTYPE = 0;
windows_core::imp::define_interface!(AsyncIMultiQI, AsyncIMultiQI_Vtbl, 0x000e0020_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(AsyncIMultiQI, windows_core::IUnknown);
impl AsyncIMultiQI {
    pub unsafe fn Begin_QueryMultipleInterfaces(&self, pmqis: &mut [MULTI_QI]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Begin_QueryMultipleInterfaces)(windows_core::Interface::as_raw(self), pmqis.len().try_into().unwrap(), core::mem::transmute(pmqis.as_ptr())) }
    }
    pub unsafe fn Finish_QueryMultipleInterfaces(&self, pmqis: *mut MULTI_QI) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finish_QueryMultipleInterfaces)(windows_core::Interface::as_raw(self), core::mem::transmute(pmqis)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIMultiQI_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_QueryMultipleInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MULTI_QI) -> windows_core::HRESULT,
    pub Finish_QueryMultipleInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MULTI_QI) -> windows_core::HRESULT,
}
pub trait AsyncIMultiQI_Impl: windows_core::IUnknownImpl {
    fn Begin_QueryMultipleInterfaces(&self, cmqis: u32, pmqis: *mut MULTI_QI) -> windows_core::Result<()>;
    fn Finish_QueryMultipleInterfaces(&self, pmqis: *mut MULTI_QI) -> windows_core::Result<()>;
}
impl AsyncIMultiQI_Vtbl {
    pub const fn new<Identity: AsyncIMultiQI_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_QueryMultipleInterfaces<Identity: AsyncIMultiQI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cmqis: u32, pmqis: *mut MULTI_QI) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIMultiQI_Impl::Begin_QueryMultipleInterfaces(this, core::mem::transmute_copy(&cmqis), core::mem::transmute_copy(&pmqis)).into()
            }
        }
        unsafe extern "system" fn Finish_QueryMultipleInterfaces<Identity: AsyncIMultiQI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmqis: *mut MULTI_QI) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIMultiQI_Impl::Finish_QueryMultipleInterfaces(this, core::mem::transmute_copy(&pmqis)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_QueryMultipleInterfaces: Begin_QueryMultipleInterfaces::<Identity, OFFSET>,
            Finish_QueryMultipleInterfaces: Finish_QueryMultipleInterfaces::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIMultiQI as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for AsyncIMultiQI {}
windows_core::imp::define_interface!(AsyncIPipeByte, AsyncIPipeByte_Vtbl, 0xdb2f3acb_2f86_11d1_8e04_00c04fb9989a);
windows_core::imp::interface_hierarchy!(AsyncIPipeByte, windows_core::IUnknown);
impl AsyncIPipeByte {
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Begin_Pull)(windows_core::Interface::as_raw(self), crequest) }
    }
    pub unsafe fn Finish_Pull(&self, buf: *mut u8, pcreturned: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finish_Pull)(windows_core::Interface::as_raw(self), buf as _, pcreturned as _) }
    }
    pub unsafe fn Begin_Push(&self, buf: *const u8, csent: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Begin_Push)(windows_core::Interface::as_raw(self), buf, csent) }
    }
    pub unsafe fn Finish_Push(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finish_Push)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeByte_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Finish_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub Begin_Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub Finish_Push: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait AsyncIPipeByte_Impl: windows_core::IUnknownImpl {
    fn Begin_Pull(&self, crequest: u32) -> windows_core::Result<()>;
    fn Finish_Pull(&self, buf: *mut u8, pcreturned: *mut u32) -> windows_core::Result<()>;
    fn Begin_Push(&self, buf: *const u8, csent: u32) -> windows_core::Result<()>;
    fn Finish_Push(&self) -> windows_core::Result<()>;
}
impl AsyncIPipeByte_Vtbl {
    pub const fn new<Identity: AsyncIPipeByte_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_Pull<Identity: AsyncIPipeByte_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crequest: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeByte_Impl::Begin_Pull(this, core::mem::transmute_copy(&crequest)).into()
            }
        }
        unsafe extern "system" fn Finish_Pull<Identity: AsyncIPipeByte_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut u8, pcreturned: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeByte_Impl::Finish_Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&pcreturned)).into()
            }
        }
        unsafe extern "system" fn Begin_Push<Identity: AsyncIPipeByte_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const u8, csent: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeByte_Impl::Begin_Push(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&csent)).into()
            }
        }
        unsafe extern "system" fn Finish_Push<Identity: AsyncIPipeByte_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeByte_Impl::Finish_Push(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_Pull: Begin_Pull::<Identity, OFFSET>,
            Finish_Pull: Finish_Pull::<Identity, OFFSET>,
            Begin_Push: Begin_Push::<Identity, OFFSET>,
            Finish_Push: Finish_Push::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIPipeByte as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for AsyncIPipeByte {}
windows_core::imp::define_interface!(AsyncIPipeDouble, AsyncIPipeDouble_Vtbl, 0xdb2f3acf_2f86_11d1_8e04_00c04fb9989a);
windows_core::imp::interface_hierarchy!(AsyncIPipeDouble, windows_core::IUnknown);
impl AsyncIPipeDouble {
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Begin_Pull)(windows_core::Interface::as_raw(self), crequest) }
    }
    pub unsafe fn Finish_Pull(&self, buf: *mut f64, pcreturned: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finish_Pull)(windows_core::Interface::as_raw(self), buf as _, pcreturned as _) }
    }
    pub unsafe fn Begin_Push(&self, buf: *const f64, csent: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Begin_Push)(windows_core::Interface::as_raw(self), buf, csent) }
    }
    pub unsafe fn Finish_Push(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finish_Push)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeDouble_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Finish_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut u32) -> windows_core::HRESULT,
    pub Begin_Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const f64, u32) -> windows_core::HRESULT,
    pub Finish_Push: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait AsyncIPipeDouble_Impl: windows_core::IUnknownImpl {
    fn Begin_Pull(&self, crequest: u32) -> windows_core::Result<()>;
    fn Finish_Pull(&self, buf: *mut f64, pcreturned: *mut u32) -> windows_core::Result<()>;
    fn Begin_Push(&self, buf: *const f64, csent: u32) -> windows_core::Result<()>;
    fn Finish_Push(&self) -> windows_core::Result<()>;
}
impl AsyncIPipeDouble_Vtbl {
    pub const fn new<Identity: AsyncIPipeDouble_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_Pull<Identity: AsyncIPipeDouble_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crequest: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeDouble_Impl::Begin_Pull(this, core::mem::transmute_copy(&crequest)).into()
            }
        }
        unsafe extern "system" fn Finish_Pull<Identity: AsyncIPipeDouble_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut f64, pcreturned: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeDouble_Impl::Finish_Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&pcreturned)).into()
            }
        }
        unsafe extern "system" fn Begin_Push<Identity: AsyncIPipeDouble_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const f64, csent: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeDouble_Impl::Begin_Push(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&csent)).into()
            }
        }
        unsafe extern "system" fn Finish_Push<Identity: AsyncIPipeDouble_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeDouble_Impl::Finish_Push(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_Pull: Begin_Pull::<Identity, OFFSET>,
            Finish_Pull: Finish_Pull::<Identity, OFFSET>,
            Begin_Push: Begin_Push::<Identity, OFFSET>,
            Finish_Push: Finish_Push::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIPipeDouble as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for AsyncIPipeDouble {}
windows_core::imp::define_interface!(AsyncIPipeLong, AsyncIPipeLong_Vtbl, 0xdb2f3acd_2f86_11d1_8e04_00c04fb9989a);
windows_core::imp::interface_hierarchy!(AsyncIPipeLong, windows_core::IUnknown);
impl AsyncIPipeLong {
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Begin_Pull)(windows_core::Interface::as_raw(self), crequest) }
    }
    pub unsafe fn Finish_Pull(&self, buf: *mut i32, pcreturned: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finish_Pull)(windows_core::Interface::as_raw(self), buf as _, pcreturned as _) }
    }
    pub unsafe fn Begin_Push(&self, buf: *const i32, csent: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Begin_Push)(windows_core::Interface::as_raw(self), buf, csent) }
    }
    pub unsafe fn Finish_Push(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finish_Push)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeLong_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Finish_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut u32) -> windows_core::HRESULT,
    pub Begin_Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32, u32) -> windows_core::HRESULT,
    pub Finish_Push: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait AsyncIPipeLong_Impl: windows_core::IUnknownImpl {
    fn Begin_Pull(&self, crequest: u32) -> windows_core::Result<()>;
    fn Finish_Pull(&self, buf: *mut i32, pcreturned: *mut u32) -> windows_core::Result<()>;
    fn Begin_Push(&self, buf: *const i32, csent: u32) -> windows_core::Result<()>;
    fn Finish_Push(&self) -> windows_core::Result<()>;
}
impl AsyncIPipeLong_Vtbl {
    pub const fn new<Identity: AsyncIPipeLong_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_Pull<Identity: AsyncIPipeLong_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crequest: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeLong_Impl::Begin_Pull(this, core::mem::transmute_copy(&crequest)).into()
            }
        }
        unsafe extern "system" fn Finish_Pull<Identity: AsyncIPipeLong_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut i32, pcreturned: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeLong_Impl::Finish_Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&pcreturned)).into()
            }
        }
        unsafe extern "system" fn Begin_Push<Identity: AsyncIPipeLong_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const i32, csent: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeLong_Impl::Begin_Push(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&csent)).into()
            }
        }
        unsafe extern "system" fn Finish_Push<Identity: AsyncIPipeLong_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeLong_Impl::Finish_Push(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_Pull: Begin_Pull::<Identity, OFFSET>,
            Finish_Pull: Finish_Pull::<Identity, OFFSET>,
            Begin_Push: Begin_Push::<Identity, OFFSET>,
            Finish_Push: Finish_Push::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIPipeLong as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for AsyncIPipeLong {}
pub const COMBND_RESERVED1: RPCOPT_PROPERTIES = 4;
pub const COMBND_RESERVED2: RPCOPT_PROPERTIES = 5;
pub const COMBND_RESERVED3: RPCOPT_PROPERTIES = 8;
pub const COMBND_RESERVED4: RPCOPT_PROPERTIES = 16;
pub const COMBND_RPCTIMEOUT: RPCOPT_PROPERTIES = 1;
pub const COMBND_SERVER_LOCALITY: RPCOPT_PROPERTIES = 2;
pub const COMGLB_APPID: GLOBALOPT_PROPERTIES = 2;
pub const COMGLB_EXCEPTION_DONOT_HANDLE: GLOBALOPT_EH_VALUES = 1;
pub const COMGLB_EXCEPTION_DONOT_HANDLE_ANY: GLOBALOPT_EH_VALUES = 2;
pub const COMGLB_EXCEPTION_DONOT_HANDLE_FATAL: GLOBALOPT_EH_VALUES = 1;
pub const COMGLB_EXCEPTION_HANDLE: GLOBALOPT_EH_VALUES = 0;
pub const COMGLB_EXCEPTION_HANDLING: GLOBALOPT_PROPERTIES = 1;
pub const COMGLB_FAST_RUNDOWN: GLOBALOPT_RO_FLAGS = 8;
pub const COMGLB_PROPERTIES_RESERVED1: GLOBALOPT_PROPERTIES = 6;
pub const COMGLB_PROPERTIES_RESERVED2: GLOBALOPT_PROPERTIES = 7;
pub const COMGLB_PROPERTIES_RESERVED3: GLOBALOPT_PROPERTIES = 8;
pub const COMGLB_RESERVED1: GLOBALOPT_RO_FLAGS = 16;
pub const COMGLB_RESERVED2: GLOBALOPT_RO_FLAGS = 32;
pub const COMGLB_RESERVED3: GLOBALOPT_RO_FLAGS = 64;
pub const COMGLB_RESERVED4: GLOBALOPT_RO_FLAGS = 256;
pub const COMGLB_RESERVED5: GLOBALOPT_RO_FLAGS = 512;
pub const COMGLB_RESERVED6: GLOBALOPT_RO_FLAGS = 1024;
pub const COMGLB_RO_SETTINGS: GLOBALOPT_PROPERTIES = 4;
pub const COMGLB_RPC_THREADPOOL_SETTING: GLOBALOPT_PROPERTIES = 3;
pub const COMGLB_RPC_THREADPOOL_SETTING_DEFAULT_POOL: GLOBALOPT_RPCTP_VALUES = 0;
pub const COMGLB_RPC_THREADPOOL_SETTING_PRIVATE_POOL: GLOBALOPT_RPCTP_VALUES = 1;
pub const COMGLB_STA_MODALLOOP_REMOVE_TOUCH_MESSAGES: GLOBALOPT_RO_FLAGS = 1;
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_DONOT_REMOVE_INPUT_MESSAGES: GLOBALOPT_RO_FLAGS = 4;
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REMOVE_INPUT_MESSAGES: GLOBALOPT_RO_FLAGS = 2;
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REORDER_POINTER_MESSAGES: GLOBALOPT_RO_FLAGS = 128;
pub const COMGLB_UNMARSHALING_POLICY: GLOBALOPT_PROPERTIES = 5;
pub const COMGLB_UNMARSHALING_POLICY_HYBRID: GLOBALOPT_UNMARSHALING_POLICY_VALUES = 2;
pub const COMGLB_UNMARSHALING_POLICY_NORMAL: GLOBALOPT_UNMARSHALING_POLICY_VALUES = 0;
pub const COMGLB_UNMARSHALING_POLICY_STRONG: GLOBALOPT_UNMARSHALING_POLICY_VALUES = 1;
#[repr(C)]
#[cfg(feature = "wtypesbase")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COSERVERINFO {
    pub dwReserved1: u32,
    pub pwszName: windows_core::PWSTR,
    pub pAuthInfo: *mut super::wtypesbase::COAUTHINFO,
    pub dwReserved2: u32,
}
#[cfg(feature = "wtypesbase")]
impl Default for COSERVERINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CO_MARSHALING_CONTEXT_ATTRIBUTES = i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_1: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483648;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_10: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483639;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_11: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483638;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_12: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483637;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_13: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483636;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_14: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483635;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_15: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483634;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_16: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483633;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_17: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483632;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_18: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483631;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_2: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483647;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_3: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483646;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_4: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483645;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_5: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483644;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_6: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483643;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_7: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483642;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_8: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483641;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_9: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483640;
pub const CO_MARSHALING_SOURCE_IS_APP_CONTAINER: CO_MARSHALING_CONTEXT_ATTRIBUTES = 0;
pub const DCOM_CALL_CANCELED: DCOM_CALL_STATE = 2;
pub const DCOM_CALL_COMPLETE: DCOM_CALL_STATE = 1;
pub type DCOM_CALL_STATE = i32;
pub const DCOM_NONE: DCOM_CALL_STATE = 0;
pub const EOAC_ACCESS_CONTROL: EOLE_AUTHENTICATION_CAPABILITIES = 4;
pub const EOAC_ANY_AUTHORITY: EOLE_AUTHENTICATION_CAPABILITIES = 128;
pub const EOAC_APPID: EOLE_AUTHENTICATION_CAPABILITIES = 8;
pub const EOAC_AUTO_IMPERSONATE: EOLE_AUTHENTICATION_CAPABILITIES = 1024;
pub const EOAC_DEFAULT: EOLE_AUTHENTICATION_CAPABILITIES = 2048;
pub const EOAC_DISABLE_AAA: EOLE_AUTHENTICATION_CAPABILITIES = 4096;
pub const EOAC_DYNAMIC: EOLE_AUTHENTICATION_CAPABILITIES = 16;
pub const EOAC_DYNAMIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES = 64;
pub const EOAC_MAKE_FULLSIC: EOLE_AUTHENTICATION_CAPABILITIES = 256;
pub const EOAC_MUTUAL_AUTH: EOLE_AUTHENTICATION_CAPABILITIES = 1;
pub const EOAC_NONE: EOLE_AUTHENTICATION_CAPABILITIES = 0;
pub const EOAC_NO_CUSTOM_MARSHAL: EOLE_AUTHENTICATION_CAPABILITIES = 8192;
pub const EOAC_REQUIRE_FULLSIC: EOLE_AUTHENTICATION_CAPABILITIES = 512;
pub const EOAC_RESERVED1: EOLE_AUTHENTICATION_CAPABILITIES = 16384;
pub const EOAC_SECURE_REFS: EOLE_AUTHENTICATION_CAPABILITIES = 2;
pub const EOAC_STATIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES = 32;
pub type EOLE_AUTHENTICATION_CAPABILITIES = i32;
pub type EXTCONN = i32;
pub const EXTCONN_CALLABLE: EXTCONN = 4;
pub const EXTCONN_STRONG: EXTCONN = 1;
pub const EXTCONN_WEAK: EXTCONN = 2;
pub type GLOBALOPT_EH_VALUES = i32;
pub type GLOBALOPT_PROPERTIES = i32;
pub type GLOBALOPT_RO_FLAGS = i32;
pub type GLOBALOPT_RPCTP_VALUES = i32;
pub type GLOBALOPT_UNMARSHALING_POLICY_VALUES = i32;
windows_core::imp::define_interface!(IActivationFilter, IActivationFilter_Vtbl, 0x00000017_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IActivationFilter, windows_core::IUnknown);
impl IActivationFilter {
    pub unsafe fn HandleActivation(&self, dwactivationtype: u32, rclsid: *const windows_core::GUID) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HandleActivation)(windows_core::Interface::as_raw(self), dwactivationtype, rclsid, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HandleActivation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IActivationFilter_Impl: windows_core::IUnknownImpl {
    fn HandleActivation(&self, dwactivationtype: u32, rclsid: *const windows_core::GUID) -> windows_core::Result<windows_core::GUID>;
}
impl IActivationFilter_Vtbl {
    pub const fn new<Identity: IActivationFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandleActivation<Identity: IActivationFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwactivationtype: u32, rclsid: *const windows_core::GUID, preplacementclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActivationFilter_Impl::HandleActivation(this, core::mem::transmute_copy(&dwactivationtype), core::mem::transmute_copy(&rclsid)) {
                    Ok(ok__) => {
                        preplacementclsid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HandleActivation: HandleActivation::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActivationFilter as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActivationFilter {}
windows_core::imp::define_interface!(IAddrExclusionControl, IAddrExclusionControl_Vtbl, 0x00000148_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IAddrExclusionControl, windows_core::IUnknown);
impl IAddrExclusionControl {
    pub unsafe fn GetCurrentAddrExclusionList<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetCurrentAddrExclusionList)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn UpdateAddrExclusionList<P0>(&self, penumerator: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateAddrExclusionList)(windows_core::Interface::as_raw(self), penumerator.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddrExclusionControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrentAddrExclusionList: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateAddrExclusionList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAddrExclusionControl_Impl: windows_core::IUnknownImpl {
    fn GetCurrentAddrExclusionList(&self, riid: *const windows_core::GUID, ppenumerator: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn UpdateAddrExclusionList(&self, penumerator: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IAddrExclusionControl_Vtbl {
    pub const fn new<Identity: IAddrExclusionControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentAddrExclusionList<Identity: IAddrExclusionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrExclusionControl_Impl::GetCurrentAddrExclusionList(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppenumerator)).into()
            }
        }
        unsafe extern "system" fn UpdateAddrExclusionList<Identity: IAddrExclusionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penumerator: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrExclusionControl_Impl::UpdateAddrExclusionList(this, core::mem::transmute_copy(&penumerator)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentAddrExclusionList: GetCurrentAddrExclusionList::<Identity, OFFSET>,
            UpdateAddrExclusionList: UpdateAddrExclusionList::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAddrExclusionControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAddrExclusionControl {}
windows_core::imp::define_interface!(IAddrTrackingControl, IAddrTrackingControl_Vtbl, 0x00000147_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IAddrTrackingControl, windows_core::IUnknown);
impl IAddrTrackingControl {
    pub unsafe fn EnableCOMDynamicAddrTracking(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnableCOMDynamicAddrTracking)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn DisableCOMDynamicAddrTracking(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DisableCOMDynamicAddrTracking)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddrTrackingControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnableCOMDynamicAddrTracking: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableCOMDynamicAddrTracking: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAddrTrackingControl_Impl: windows_core::IUnknownImpl {
    fn EnableCOMDynamicAddrTracking(&self) -> windows_core::Result<()>;
    fn DisableCOMDynamicAddrTracking(&self) -> windows_core::Result<()>;
}
impl IAddrTrackingControl_Vtbl {
    pub const fn new<Identity: IAddrTrackingControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnableCOMDynamicAddrTracking<Identity: IAddrTrackingControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrTrackingControl_Impl::EnableCOMDynamicAddrTracking(this).into()
            }
        }
        unsafe extern "system" fn DisableCOMDynamicAddrTracking<Identity: IAddrTrackingControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAddrTrackingControl_Impl::DisableCOMDynamicAddrTracking(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnableCOMDynamicAddrTracking: EnableCOMDynamicAddrTracking::<Identity, OFFSET>,
            DisableCOMDynamicAddrTracking: DisableCOMDynamicAddrTracking::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAddrTrackingControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAddrTrackingControl {}
windows_core::imp::define_interface!(IAgileObject, IAgileObject_Vtbl, 0x94ea2b94_e9cc_49e0_c0ff_ee64ca8f5b90);
windows_core::imp::interface_hierarchy!(IAgileObject, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct IAgileObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IAgileObject_Impl: windows_core::IUnknownImpl {}
impl IAgileObject_Vtbl {
    pub const fn new<Identity: IAgileObject_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAgileObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAgileObject {}
windows_core::imp::define_interface!(IAgileReference, IAgileReference_Vtbl, 0xc03f6a43_65a4_9818_987e_e0b810d2a6f2);
windows_core::imp::interface_hierarchy!(IAgileReference, windows_core::IUnknown);
impl IAgileReference {
    pub unsafe fn Resolve<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).Resolve)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgileReference_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Resolve: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAgileReference_Impl: windows_core::IUnknownImpl {
    fn Resolve(&self, riid: *const windows_core::GUID, ppvobjectreference: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IAgileReference_Vtbl {
    pub const fn new<Identity: IAgileReference_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Resolve<Identity: IAgileReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobjectreference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAgileReference_Impl::Resolve(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobjectreference)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Resolve: Resolve::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAgileReference as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAgileReference {}
windows_core::imp::define_interface!(IAsyncManager, IAsyncManager_Vtbl, 0x0000002a_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IAsyncManager, windows_core::IUnknown);
impl IAsyncManager {
    pub unsafe fn CompleteCall(&self, result: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CompleteCall)(windows_core::Interface::as_raw(self), result) }
    }
    pub unsafe fn GetCallContext(&self, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCallContext)(windows_core::Interface::as_raw(self), riid, pinterface as _) }
    }
    pub unsafe fn GetState(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CompleteCall: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetCallContext: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IAsyncManager_Impl: windows_core::IUnknownImpl {
    fn CompleteCall(&self, result: windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetCallContext(&self, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetState(&self) -> windows_core::Result<u32>;
}
impl IAsyncManager_Vtbl {
    pub const fn new<Identity: IAsyncManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CompleteCall<Identity: IAsyncManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncManager_Impl::CompleteCall(this, core::mem::transmute_copy(&result)).into()
            }
        }
        unsafe extern "system" fn GetCallContext<Identity: IAsyncManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncManager_Impl::GetCallContext(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pinterface)).into()
            }
        }
        unsafe extern "system" fn GetState<Identity: IAsyncManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulstateflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncManager_Impl::GetState(this) {
                    Ok(ok__) => {
                        pulstateflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CompleteCall: CompleteCall::<Identity, OFFSET>,
            GetCallContext: GetCallContext::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAsyncManager {}
windows_core::imp::define_interface!(IAsyncRpcChannelBuffer, IAsyncRpcChannelBuffer_Vtbl, 0xa5029fb6_3c34_11d1_9c99_00c04fb998aa);
impl core::ops::Deref for IAsyncRpcChannelBuffer {
    type Target = IRpcChannelBuffer2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAsyncRpcChannelBuffer, windows_core::IUnknown, IRpcChannelBuffer, IRpcChannelBuffer2);
impl IAsyncRpcChannelBuffer {
    pub unsafe fn Send<P1>(&self, pmsg: *mut RPCOLEMESSAGE, psync: P1, pulstatus: *mut u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ISynchronize>,
    {
        unsafe { (windows_core::Interface::vtable(self).Send)(windows_core::Interface::as_raw(self), pmsg as _, psync.param().abi(), pulstatus as _) }
    }
    pub unsafe fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Receive)(windows_core::Interface::as_raw(self), pmsg as _, pulstatus as _) }
    }
    pub unsafe fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDestCtxEx)(windows_core::Interface::as_raw(self), pmsg, pdwdestcontext as _, ppvdestcontext as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncRpcChannelBuffer_Vtbl {
    pub base__: IRpcChannelBuffer2_Vtbl,
    pub Send: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Receive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *mut u32) -> windows_core::HRESULT,
    pub GetDestCtxEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const RPCOLEMESSAGE, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAsyncRpcChannelBuffer_Impl: IRpcChannelBuffer2_Impl {
    fn Send(&self, pmsg: *mut RPCOLEMESSAGE, psync: windows_core::Ref<ISynchronize>, pulstatus: *mut u32) -> windows_core::Result<()>;
    fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> windows_core::Result<()>;
    fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IAsyncRpcChannelBuffer_Vtbl {
    pub const fn new<Identity: IAsyncRpcChannelBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Send<Identity: IAsyncRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, psync: *mut core::ffi::c_void, pulstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncRpcChannelBuffer_Impl::Send(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&psync), core::mem::transmute_copy(&pulstatus)).into()
            }
        }
        unsafe extern "system" fn Receive<Identity: IAsyncRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncRpcChannelBuffer_Impl::Receive(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&pulstatus)).into()
            }
        }
        unsafe extern "system" fn GetDestCtxEx<Identity: IAsyncRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncRpcChannelBuffer_Impl::GetDestCtxEx(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&pdwdestcontext), core::mem::transmute_copy(&ppvdestcontext)).into()
            }
        }
        Self {
            base__: IRpcChannelBuffer2_Vtbl::new::<Identity, OFFSET>(),
            Send: Send::<Identity, OFFSET>,
            Receive: Receive::<Identity, OFFSET>,
            GetDestCtxEx: GetDestCtxEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncRpcChannelBuffer as windows_core::Interface>::IID || iid == &<IRpcChannelBuffer as windows_core::Interface>::IID || iid == &<IRpcChannelBuffer2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAsyncRpcChannelBuffer {}
windows_core::imp::define_interface!(ICallFactory, ICallFactory_Vtbl, 0x1c733a30_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ICallFactory, windows_core::IUnknown);
impl ICallFactory {
    pub unsafe fn CreateCall<P1, T>(&self, riid: *const windows_core::GUID, pctrlunk: P1) -> windows_core::Result<T>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateCall)(windows_core::Interface::as_raw(self), riid, pctrlunk.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateCall: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICallFactory_Impl: windows_core::IUnknownImpl {
    fn CreateCall(&self, riid: *const windows_core::GUID, pctrlunk: windows_core::Ref<windows_core::IUnknown>, riid2: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl ICallFactory_Vtbl {
    pub const fn new<Identity: ICallFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateCall<Identity: ICallFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pctrlunk: *mut core::ffi::c_void, riid2: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICallFactory_Impl::CreateCall(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pctrlunk), core::mem::transmute_copy(&riid2), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateCall: CreateCall::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICallFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICallFactory {}
windows_core::imp::define_interface!(ICancelMethodCalls, ICancelMethodCalls_Vtbl, 0x00000029_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ICancelMethodCalls, windows_core::IUnknown);
impl ICancelMethodCalls {
    pub unsafe fn Cancel(&self, ulseconds: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self), ulseconds) }
    }
    pub unsafe fn TestCancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TestCancel)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICancelMethodCalls_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub TestCancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICancelMethodCalls_Impl: windows_core::IUnknownImpl {
    fn Cancel(&self, ulseconds: u32) -> windows_core::Result<()>;
    fn TestCancel(&self) -> windows_core::Result<()>;
}
impl ICancelMethodCalls_Vtbl {
    pub const fn new<Identity: ICancelMethodCalls_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cancel<Identity: ICancelMethodCalls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulseconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICancelMethodCalls_Impl::Cancel(this, core::mem::transmute_copy(&ulseconds)).into()
            }
        }
        unsafe extern "system" fn TestCancel<Identity: ICancelMethodCalls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICancelMethodCalls_Impl::TestCancel(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Cancel: Cancel::<Identity, OFFSET>, TestCancel: TestCancel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICancelMethodCalls as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICancelMethodCalls {}
windows_core::imp::define_interface!(IChannelHook, IChannelHook_Vtbl, 0x1008c4a0_7613_11cf_9af1_0020af6e72f4);
windows_core::imp::interface_hierarchy!(IChannelHook, windows_core::IUnknown);
impl IChannelHook {
    pub unsafe fn ClientGetSize(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID) -> u32 {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientGetSize)(windows_core::Interface::as_raw(self), uextent, riid, &mut result__);
            result__
        }
    }
    pub unsafe fn ClientFillBuffer(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const core::ffi::c_void) {
        unsafe {
            (windows_core::Interface::vtable(self).ClientFillBuffer)(windows_core::Interface::as_raw(self), uextent, riid, pdatasize as _, pdatabuffer);
        }
    }
    pub unsafe fn ClientNotify(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, cbdatasize: u32, pdatabuffer: *const core::ffi::c_void, ldatarep: u32, hrfault: windows_core::HRESULT) {
        unsafe {
            (windows_core::Interface::vtable(self).ClientNotify)(windows_core::Interface::as_raw(self), uextent, riid, cbdatasize, pdatabuffer, ldatarep, hrfault);
        }
    }
    pub unsafe fn ServerNotify(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, cbdatasize: u32, pdatabuffer: *const core::ffi::c_void, ldatarep: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).ServerNotify)(windows_core::Interface::as_raw(self), uextent, riid, cbdatasize, pdatabuffer, ldatarep);
        }
    }
    pub unsafe fn ServerGetSize(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, hrfault: windows_core::HRESULT) -> u32 {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServerGetSize)(windows_core::Interface::as_raw(self), uextent, riid, hrfault, &mut result__);
            result__
        }
    }
    pub unsafe fn ServerFillBuffer(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const core::ffi::c_void, hrfault: windows_core::HRESULT) {
        unsafe {
            (windows_core::Interface::vtable(self).ServerFillBuffer)(windows_core::Interface::as_raw(self), uextent, riid, pdatasize as _, pdatabuffer, hrfault);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelHook_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ClientGetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut u32),
    pub ClientFillBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut u32, *const core::ffi::c_void),
    pub ClientNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, u32, *const core::ffi::c_void, u32, windows_core::HRESULT),
    pub ServerNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, u32, *const core::ffi::c_void, u32),
    pub ServerGetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, windows_core::HRESULT, *mut u32),
    pub ServerFillBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut u32, *const core::ffi::c_void, windows_core::HRESULT),
}
pub trait IChannelHook_Impl: windows_core::IUnknownImpl {
    fn ClientGetSize(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32);
    fn ClientFillBuffer(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const core::ffi::c_void);
    fn ClientNotify(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, cbdatasize: u32, pdatabuffer: *const core::ffi::c_void, ldatarep: u32, hrfault: windows_core::HRESULT);
    fn ServerNotify(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, cbdatasize: u32, pdatabuffer: *const core::ffi::c_void, ldatarep: u32);
    fn ServerGetSize(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, hrfault: windows_core::HRESULT, pdatasize: *mut u32);
    fn ServerFillBuffer(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const core::ffi::c_void, hrfault: windows_core::HRESULT);
}
impl IChannelHook_Vtbl {
    pub const fn new<Identity: IChannelHook_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ClientGetSize<Identity: IChannelHook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelHook_Impl::ClientGetSize(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pdatasize));
            }
        }
        unsafe extern "system" fn ClientFillBuffer<Identity: IChannelHook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelHook_Impl::ClientFillBuffer(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdatabuffer));
            }
        }
        unsafe extern "system" fn ClientNotify<Identity: IChannelHook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, cbdatasize: u32, pdatabuffer: *const core::ffi::c_void, ldatarep: u32, hrfault: windows_core::HRESULT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelHook_Impl::ClientNotify(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cbdatasize), core::mem::transmute_copy(&pdatabuffer), core::mem::transmute_copy(&ldatarep), core::mem::transmute_copy(&hrfault));
            }
        }
        unsafe extern "system" fn ServerNotify<Identity: IChannelHook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, cbdatasize: u32, pdatabuffer: *const core::ffi::c_void, ldatarep: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelHook_Impl::ServerNotify(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cbdatasize), core::mem::transmute_copy(&pdatabuffer), core::mem::transmute_copy(&ldatarep));
            }
        }
        unsafe extern "system" fn ServerGetSize<Identity: IChannelHook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, hrfault: windows_core::HRESULT, pdatasize: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelHook_Impl::ServerGetSize(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&hrfault), core::mem::transmute_copy(&pdatasize));
            }
        }
        unsafe extern "system" fn ServerFillBuffer<Identity: IChannelHook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const core::ffi::c_void, hrfault: windows_core::HRESULT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelHook_Impl::ServerFillBuffer(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdatabuffer), core::mem::transmute_copy(&hrfault));
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ClientGetSize: ClientGetSize::<Identity, OFFSET>,
            ClientFillBuffer: ClientFillBuffer::<Identity, OFFSET>,
            ClientNotify: ClientNotify::<Identity, OFFSET>,
            ServerNotify: ServerNotify::<Identity, OFFSET>,
            ServerGetSize: ServerGetSize::<Identity, OFFSET>,
            ServerFillBuffer: ServerFillBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IChannelHook as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IChannelHook {}
windows_core::imp::define_interface!(IClientSecurity, IClientSecurity_Vtbl, 0x0000013d_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IClientSecurity, windows_core::IUnknown);
impl IClientSecurity {
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn QueryBlanket<P0>(&self, pproxy: P0, pauthnsvc: *mut u32, pauthzsvc: Option<*mut u32>, pserverprincname: *mut *mut super::wtypesbase::OLECHAR, pauthnlevel: Option<*mut u32>, pimplevel: Option<*mut u32>, pauthinfo: *mut *mut core::ffi::c_void, pcapabilites: Option<*mut u32>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueryBlanket)(windows_core::Interface::as_raw(self), pproxy.param().abi(), pauthnsvc as _, pauthzsvc.unwrap_or(core::mem::zeroed()) as _, pserverprincname as _, pauthnlevel.unwrap_or(core::mem::zeroed()) as _, pimplevel.unwrap_or(core::mem::zeroed()) as _, pauthinfo as _, pcapabilites.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn SetBlanket<P0>(&self, pproxy: P0, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: *const super::wtypesbase::OLECHAR, dwauthnlevel: u32, dwimplevel: u32, pauthinfo: Option<*const core::ffi::c_void>, dwcapabilities: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBlanket)(windows_core::Interface::as_raw(self), pproxy.param().abi(), dwauthnsvc, dwauthzsvc, pserverprincname, dwauthnlevel, dwimplevel, pauthinfo.unwrap_or(core::mem::zeroed()) as _, dwcapabilities) }
    }
    pub unsafe fn CopyProxy<P0>(&self, pproxy: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyProxy)(windows_core::Interface::as_raw(self), pproxy.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClientSecurity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wtypesbase")]
    pub QueryBlanket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut u32, *mut *mut super::wtypesbase::OLECHAR, *mut u32, *mut u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    QueryBlanket: usize,
    #[cfg(feature = "wtypesbase")]
    pub SetBlanket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *const super::wtypesbase::OLECHAR, u32, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    SetBlanket: usize,
    pub CopyProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "wtypesbase")]
pub trait IClientSecurity_Impl: windows_core::IUnknownImpl {
    fn QueryBlanket(&self, pproxy: windows_core::Ref<windows_core::IUnknown>, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut super::wtypesbase::OLECHAR, pauthnlevel: *mut u32, pimplevel: *mut u32, pauthinfo: *mut *mut core::ffi::c_void, pcapabilites: *mut u32) -> windows_core::Result<()>;
    fn SetBlanket(&self, pproxy: windows_core::Ref<windows_core::IUnknown>, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: *const super::wtypesbase::OLECHAR, dwauthnlevel: u32, dwimplevel: u32, pauthinfo: *const core::ffi::c_void, dwcapabilities: u32) -> windows_core::Result<()>;
    fn CopyProxy(&self, pproxy: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "wtypesbase")]
impl IClientSecurity_Vtbl {
    pub const fn new<Identity: IClientSecurity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryBlanket<Identity: IClientSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproxy: *mut core::ffi::c_void, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut super::wtypesbase::OLECHAR, pauthnlevel: *mut u32, pimplevel: *mut u32, pauthinfo: *mut *mut core::ffi::c_void, pcapabilites: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClientSecurity_Impl::QueryBlanket(this, core::mem::transmute_copy(&pproxy), core::mem::transmute_copy(&pauthnsvc), core::mem::transmute_copy(&pauthzsvc), core::mem::transmute_copy(&pserverprincname), core::mem::transmute_copy(&pauthnlevel), core::mem::transmute_copy(&pimplevel), core::mem::transmute_copy(&pauthinfo), core::mem::transmute_copy(&pcapabilites)).into()
            }
        }
        unsafe extern "system" fn SetBlanket<Identity: IClientSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproxy: *mut core::ffi::c_void, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: *const super::wtypesbase::OLECHAR, dwauthnlevel: u32, dwimplevel: u32, pauthinfo: *const core::ffi::c_void, dwcapabilities: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClientSecurity_Impl::SetBlanket(this, core::mem::transmute_copy(&pproxy), core::mem::transmute_copy(&dwauthnsvc), core::mem::transmute_copy(&dwauthzsvc), core::mem::transmute_copy(&pserverprincname), core::mem::transmute_copy(&dwauthnlevel), core::mem::transmute_copy(&dwimplevel), core::mem::transmute_copy(&pauthinfo), core::mem::transmute_copy(&dwcapabilities)).into()
            }
        }
        unsafe extern "system" fn CopyProxy<Identity: IClientSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproxy: *mut core::ffi::c_void, ppcopy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClientSecurity_Impl::CopyProxy(this, core::mem::transmute_copy(&pproxy)) {
                    Ok(ok__) => {
                        ppcopy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryBlanket: QueryBlanket::<Identity, OFFSET>,
            SetBlanket: SetBlanket::<Identity, OFFSET>,
            CopyProxy: CopyProxy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClientSecurity as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wtypesbase")]
impl windows_core::RuntimeName for IClientSecurity {}
windows_core::imp::define_interface!(IComThreadingInfo, IComThreadingInfo_Vtbl, 0x000001ce_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IComThreadingInfo, windows_core::IUnknown);
impl IComThreadingInfo {
    pub unsafe fn GetCurrentApartmentType(&self) -> windows_core::Result<APTTYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentApartmentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrentThreadType(&self) -> windows_core::Result<THDTYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentThreadType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrentLogicalThreadId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentLogicalThreadId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCurrentLogicalThreadId(&self, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentLogicalThreadId)(windows_core::Interface::as_raw(self), rguid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComThreadingInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrentApartmentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut APTTYPE) -> windows_core::HRESULT,
    pub GetCurrentThreadType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut THDTYPE) -> windows_core::HRESULT,
    pub GetCurrentLogicalThreadId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetCurrentLogicalThreadId: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IComThreadingInfo_Impl: windows_core::IUnknownImpl {
    fn GetCurrentApartmentType(&self) -> windows_core::Result<APTTYPE>;
    fn GetCurrentThreadType(&self) -> windows_core::Result<THDTYPE>;
    fn GetCurrentLogicalThreadId(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetCurrentLogicalThreadId(&self, rguid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IComThreadingInfo_Vtbl {
    pub const fn new<Identity: IComThreadingInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentApartmentType<Identity: IComThreadingInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, papttype: *mut APTTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComThreadingInfo_Impl::GetCurrentApartmentType(this) {
                    Ok(ok__) => {
                        papttype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentThreadType<Identity: IComThreadingInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pthreadtype: *mut THDTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComThreadingInfo_Impl::GetCurrentThreadType(this) {
                    Ok(ok__) => {
                        pthreadtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentLogicalThreadId<Identity: IComThreadingInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidlogicalthreadid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComThreadingInfo_Impl::GetCurrentLogicalThreadId(this) {
                    Ok(ok__) => {
                        pguidlogicalthreadid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCurrentLogicalThreadId<Identity: IComThreadingInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComThreadingInfo_Impl::SetCurrentLogicalThreadId(this, core::mem::transmute_copy(&rguid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentApartmentType: GetCurrentApartmentType::<Identity, OFFSET>,
            GetCurrentThreadType: GetCurrentThreadType::<Identity, OFFSET>,
            GetCurrentLogicalThreadId: GetCurrentLogicalThreadId::<Identity, OFFSET>,
            SetCurrentLogicalThreadId: SetCurrentLogicalThreadId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComThreadingInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IComThreadingInfo {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IContext(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IEnumContextProps(pub u8);
windows_core::imp::define_interface!(IEnumString, IEnumString_Vtbl, 0x00000101_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumString, windows_core::IUnknown);
impl IEnumString {
    pub unsafe fn Next(&self, rgelt: &mut [windows_core::PWSTR], pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
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
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumString_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumString_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut windows_core::PWSTR, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumString>;
}
impl IEnumString_Vtbl {
    pub const fn new<Identity: IEnumString_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut windows_core::PWSTR, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumString_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumString_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumString_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumString_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
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
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumString as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumString {}
windows_core::imp::define_interface!(IEnumUnknown, IEnumUnknown_Vtbl, 0x00000100_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumUnknown, windows_core::IUnknown);
impl IEnumUnknown {
    pub unsafe fn Next(&self, rgelt: &mut [Option<windows_core::IUnknown>], pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
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
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumUnknown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumUnknown_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut Option<windows_core::IUnknown>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumUnknown>;
}
impl IEnumUnknown_Vtbl {
    pub const fn new<Identity: IEnumUnknown_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumUnknown_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumUnknown_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumUnknown_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumUnknown_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
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
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumUnknown as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumUnknown {}
windows_core::imp::define_interface!(IExternalConnection, IExternalConnection_Vtbl, 0x00000019_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IExternalConnection, windows_core::IUnknown);
impl IExternalConnection {
    pub unsafe fn AddConnection(&self, extconn: u32, reserved: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddConnection)(windows_core::Interface::as_raw(self), extconn, reserved) }
    }
    pub unsafe fn ReleaseConnection(&self, extconn: u32, reserved: u32, flastreleasecloses: bool) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).ReleaseConnection)(windows_core::Interface::as_raw(self), extconn, reserved, flastreleasecloses.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExternalConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddConnection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> u32,
    pub ReleaseConnection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::BOOL) -> u32,
}
pub trait IExternalConnection_Impl: windows_core::IUnknownImpl {
    fn AddConnection(&self, extconn: u32, reserved: u32) -> u32;
    fn ReleaseConnection(&self, extconn: u32, reserved: u32, flastreleasecloses: windows_core::BOOL) -> u32;
}
impl IExternalConnection_Vtbl {
    pub const fn new<Identity: IExternalConnection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddConnection<Identity: IExternalConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, extconn: u32, reserved: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExternalConnection_Impl::AddConnection(this, core::mem::transmute_copy(&extconn), core::mem::transmute_copy(&reserved))
            }
        }
        unsafe extern "system" fn ReleaseConnection<Identity: IExternalConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, extconn: u32, reserved: u32, flastreleasecloses: windows_core::BOOL) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExternalConnection_Impl::ReleaseConnection(this, core::mem::transmute_copy(&extconn), core::mem::transmute_copy(&reserved), core::mem::transmute_copy(&flastreleasecloses))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddConnection: AddConnection::<Identity, OFFSET>,
            ReleaseConnection: ReleaseConnection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExternalConnection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IExternalConnection {}
windows_core::imp::define_interface!(IFastRundown, IFastRundown_Vtbl, 0x00000040_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IFastRundown, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct IFastRundown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IFastRundown_Impl: windows_core::IUnknownImpl {}
impl IFastRundown_Vtbl {
    pub const fn new<Identity: IFastRundown_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFastRundown as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IFastRundown {}
windows_core::imp::define_interface!(IGlobalInterfaceTable, IGlobalInterfaceTable_Vtbl, 0x00000146_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IGlobalInterfaceTable, windows_core::IUnknown);
impl IGlobalInterfaceTable {
    pub unsafe fn RegisterInterfaceInGlobal<P0>(&self, punk: P0, riid: *const windows_core::GUID) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterInterfaceInGlobal)(windows_core::Interface::as_raw(self), punk.param().abi(), riid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RevokeInterfaceFromGlobal(&self, dwcookie: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RevokeInterfaceFromGlobal)(windows_core::Interface::as_raw(self), dwcookie) }
    }
    pub unsafe fn GetInterfaceFromGlobal<T>(&self, dwcookie: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetInterfaceFromGlobal)(windows_core::Interface::as_raw(self), dwcookie, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalInterfaceTable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterInterfaceInGlobal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub RevokeInterfaceFromGlobal: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetInterfaceFromGlobal: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IGlobalInterfaceTable_Impl: windows_core::IUnknownImpl {
    fn RegisterInterfaceInGlobal(&self, punk: windows_core::Ref<windows_core::IUnknown>, riid: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn RevokeInterfaceFromGlobal(&self, dwcookie: u32) -> windows_core::Result<()>;
    fn GetInterfaceFromGlobal(&self, dwcookie: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IGlobalInterfaceTable_Vtbl {
    pub const fn new<Identity: IGlobalInterfaceTable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterInterfaceInGlobal<Identity: IGlobalInterfaceTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, riid: *const windows_core::GUID, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGlobalInterfaceTable_Impl::RegisterInterfaceInGlobal(this, core::mem::transmute_copy(&punk), core::mem::transmute_copy(&riid)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RevokeInterfaceFromGlobal<Identity: IGlobalInterfaceTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGlobalInterfaceTable_Impl::RevokeInterfaceFromGlobal(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        unsafe extern "system" fn GetInterfaceFromGlobal<Identity: IGlobalInterfaceTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGlobalInterfaceTable_Impl::GetInterfaceFromGlobal(this, core::mem::transmute_copy(&dwcookie), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterInterfaceInGlobal: RegisterInterfaceInGlobal::<Identity, OFFSET>,
            RevokeInterfaceFromGlobal: RevokeInterfaceFromGlobal::<Identity, OFFSET>,
            GetInterfaceFromGlobal: GetInterfaceFromGlobal::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGlobalInterfaceTable as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGlobalInterfaceTable {}
windows_core::imp::define_interface!(IGlobalOptions, IGlobalOptions_Vtbl, 0x0000015b_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IGlobalOptions, windows_core::IUnknown);
impl IGlobalOptions {
    pub unsafe fn Set(&self, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), dwproperty, dwvalue) }
    }
    pub unsafe fn Query(&self, dwproperty: GLOBALOPT_PROPERTIES) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), dwproperty, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, GLOBALOPT_PROPERTIES, usize) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, GLOBALOPT_PROPERTIES, *mut usize) -> windows_core::HRESULT,
}
pub trait IGlobalOptions_Impl: windows_core::IUnknownImpl {
    fn Set(&self, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> windows_core::Result<()>;
    fn Query(&self, dwproperty: GLOBALOPT_PROPERTIES) -> windows_core::Result<usize>;
}
impl IGlobalOptions_Vtbl {
    pub const fn new<Identity: IGlobalOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Set<Identity: IGlobalOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGlobalOptions_Impl::Set(this, core::mem::transmute_copy(&dwproperty), core::mem::transmute_copy(&dwvalue)).into()
            }
        }
        unsafe extern "system" fn Query<Identity: IGlobalOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, pdwvalue: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGlobalOptions_Impl::Query(this, core::mem::transmute_copy(&dwproperty)) {
                    Ok(ok__) => {
                        pdwvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Set: Set::<Identity, OFFSET>, Query: Query::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGlobalOptions as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGlobalOptions {}
windows_core::imp::define_interface!(IInternalUnknown, IInternalUnknown_Vtbl, 0x00000021_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IInternalUnknown, windows_core::IUnknown);
impl IInternalUnknown {
    pub unsafe fn QueryInternalInterface(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInternalInterface)(windows_core::Interface::as_raw(self), riid, ppv as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternalUnknown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInternalInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IInternalUnknown_Impl: windows_core::IUnknownImpl {
    fn QueryInternalInterface(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IInternalUnknown_Vtbl {
    pub const fn new<Identity: IInternalUnknown_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInternalInterface<Identity: IInternalUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternalUnknown_Impl::QueryInternalInterface(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryInternalInterface: QueryInternalInterface::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternalUnknown as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInternalUnknown {}
windows_core::imp::define_interface!(IMachineGlobalObjectTable, IMachineGlobalObjectTable_Vtbl, 0x26d709ac_f70b_4421_a96f_d2878fafb00d);
windows_core::imp::interface_hierarchy!(IMachineGlobalObjectTable, windows_core::IUnknown);
impl IMachineGlobalObjectTable {
    pub unsafe fn RegisterObject<P1, P2>(&self, clsid: *const windows_core::GUID, identifier: P1, object: P2) -> windows_core::Result<MachineGlobalObjectTableRegistrationToken>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterObject)(windows_core::Interface::as_raw(self), clsid, identifier.param().abi(), object.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetObject<P1, T>(&self, clsid: *const windows_core::GUID, identifier: P1) -> windows_core::Result<T>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), clsid, identifier.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn RevokeObject(&self, token: MachineGlobalObjectTableRegistrationToken) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RevokeObject)(windows_core::Interface::as_raw(self), token) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMachineGlobalObjectTable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, *mut core::ffi::c_void, *mut MachineGlobalObjectTableRegistrationToken) -> windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RevokeObject: unsafe extern "system" fn(*mut core::ffi::c_void, MachineGlobalObjectTableRegistrationToken) -> windows_core::HRESULT,
}
pub trait IMachineGlobalObjectTable_Impl: windows_core::IUnknownImpl {
    fn RegisterObject(&self, clsid: *const windows_core::GUID, identifier: &windows_core::PCWSTR, object: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<MachineGlobalObjectTableRegistrationToken>;
    fn GetObject(&self, clsid: *const windows_core::GUID, identifier: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RevokeObject(&self, token: MachineGlobalObjectTableRegistrationToken) -> windows_core::Result<()>;
}
impl IMachineGlobalObjectTable_Vtbl {
    pub const fn new<Identity: IMachineGlobalObjectTable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterObject<Identity: IMachineGlobalObjectTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, identifier: windows_core::PCWSTR, object: *mut core::ffi::c_void, token: *mut MachineGlobalObjectTableRegistrationToken) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMachineGlobalObjectTable_Impl::RegisterObject(this, core::mem::transmute_copy(&clsid), core::mem::transmute(&identifier), core::mem::transmute_copy(&object)) {
                    Ok(ok__) => {
                        token.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObject<Identity: IMachineGlobalObjectTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, identifier: windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMachineGlobalObjectTable_Impl::GetObject(this, core::mem::transmute_copy(&clsid), core::mem::transmute(&identifier), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn RevokeObject<Identity: IMachineGlobalObjectTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: MachineGlobalObjectTableRegistrationToken) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMachineGlobalObjectTable_Impl::RevokeObject(this, core::mem::transmute_copy(&token)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterObject: RegisterObject::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            RevokeObject: RevokeObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMachineGlobalObjectTable as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMachineGlobalObjectTable {}
windows_core::imp::define_interface!(IMalloc, IMalloc_Vtbl, 0x00000002_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IMalloc, windows_core::IUnknown);
impl IMalloc {
    pub unsafe fn Alloc(&self, cb: usize) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).Alloc)(windows_core::Interface::as_raw(self), cb) }
    }
    pub unsafe fn Realloc(&self, pv: Option<*const core::ffi::c_void>, cb: usize) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).Realloc)(windows_core::Interface::as_raw(self), pv.unwrap_or(core::mem::zeroed()) as _, cb) }
    }
    pub unsafe fn Free(&self, pv: Option<*const core::ffi::c_void>) {
        unsafe {
            (windows_core::Interface::vtable(self).Free)(windows_core::Interface::as_raw(self), pv.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn GetSize(&self, pv: Option<*const core::ffi::c_void>) -> usize {
        unsafe { (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), pv.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn DidAlloc(&self, pv: Option<*const core::ffi::c_void>) -> i32 {
        unsafe { (windows_core::Interface::vtable(self).DidAlloc)(windows_core::Interface::as_raw(self), pv.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn HeapMinimize(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).HeapMinimize)(windows_core::Interface::as_raw(self));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMalloc_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Alloc: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> *mut core::ffi::c_void,
    pub Realloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize) -> *mut core::ffi::c_void,
    pub Free: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void),
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> usize,
    pub DidAlloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> i32,
    pub HeapMinimize: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait IMalloc_Impl: windows_core::IUnknownImpl {
    fn Alloc(&self, cb: usize) -> *mut core::ffi::c_void;
    fn Realloc(&self, pv: *const core::ffi::c_void, cb: usize) -> *mut core::ffi::c_void;
    fn Free(&self, pv: *const core::ffi::c_void);
    fn GetSize(&self, pv: *const core::ffi::c_void) -> usize;
    fn DidAlloc(&self, pv: *const core::ffi::c_void) -> i32;
    fn HeapMinimize(&self);
}
impl IMalloc_Vtbl {
    pub const fn new<Identity: IMalloc_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Alloc<Identity: IMalloc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cb: usize) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMalloc_Impl::Alloc(this, core::mem::transmute_copy(&cb))
            }
        }
        unsafe extern "system" fn Realloc<Identity: IMalloc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void, cb: usize) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMalloc_Impl::Realloc(this, core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb))
            }
        }
        unsafe extern "system" fn Free<Identity: IMalloc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMalloc_Impl::Free(this, core::mem::transmute_copy(&pv));
            }
        }
        unsafe extern "system" fn GetSize<Identity: IMalloc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void) -> usize {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMalloc_Impl::GetSize(this, core::mem::transmute_copy(&pv))
            }
        }
        unsafe extern "system" fn DidAlloc<Identity: IMalloc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void) -> i32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMalloc_Impl::DidAlloc(this, core::mem::transmute_copy(&pv))
            }
        }
        unsafe extern "system" fn HeapMinimize<Identity: IMalloc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMalloc_Impl::HeapMinimize(this);
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Alloc: Alloc::<Identity, OFFSET>,
            Realloc: Realloc::<Identity, OFFSET>,
            Free: Free::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            DidAlloc: DidAlloc::<Identity, OFFSET>,
            HeapMinimize: HeapMinimize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMalloc as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMalloc {}
windows_core::imp::define_interface!(IMarshal, IMarshal_Vtbl, 0x00000003_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IMarshal, windows_core::IUnknown);
impl IMarshal {
    pub unsafe fn GetUnmarshalClass(&self, riid: *const windows_core::GUID, pv: Option<*const core::ffi::c_void>, dwdestcontext: u32, pvdestcontext: Option<*const core::ffi::c_void>, mshlflags: u32) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUnmarshalClass)(windows_core::Interface::as_raw(self), riid, pv.unwrap_or(core::mem::zeroed()) as _, dwdestcontext, pvdestcontext.unwrap_or(core::mem::zeroed()) as _, mshlflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMarshalSizeMax(&self, riid: *const windows_core::GUID, pv: Option<*const core::ffi::c_void>, dwdestcontext: u32, pvdestcontext: Option<*const core::ffi::c_void>, mshlflags: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMarshalSizeMax)(windows_core::Interface::as_raw(self), riid, pv.unwrap_or(core::mem::zeroed()) as _, dwdestcontext, pvdestcontext.unwrap_or(core::mem::zeroed()) as _, mshlflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MarshalInterface<P0>(&self, pstm: P0, riid: *const windows_core::GUID, pv: Option<*const core::ffi::c_void>, dwdestcontext: u32, pvdestcontext: Option<*const core::ffi::c_void>, mshlflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).MarshalInterface)(windows_core::Interface::as_raw(self), pstm.param().abi(), riid, pv.unwrap_or(core::mem::zeroed()) as _, dwdestcontext, pvdestcontext.unwrap_or(core::mem::zeroed()) as _, mshlflags) }
    }
    pub unsafe fn UnmarshalInterface<P0>(&self, pstm: P0, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnmarshalInterface)(windows_core::Interface::as_raw(self), pstm.param().abi(), riid, ppv as _) }
    }
    pub unsafe fn ReleaseMarshalData<P0>(&self, pstm: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReleaseMarshalData)(windows_core::Interface::as_raw(self), pstm.param().abi()) }
    }
    pub unsafe fn DisconnectObject(&self, dwreserved: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DisconnectObject)(windows_core::Interface::as_raw(self), dwreserved) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarshal_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUnmarshalClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const core::ffi::c_void, u32, *const core::ffi::c_void, u32, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetMarshalSizeMax: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const core::ffi::c_void, u32, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub MarshalInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *const core::ffi::c_void, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UnmarshalInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseMarshalData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisconnectObject: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IMarshal_Impl: windows_core::IUnknownImpl {
    fn GetUnmarshalClass(&self, riid: *const windows_core::GUID, pv: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, mshlflags: u32) -> windows_core::Result<windows_core::GUID>;
    fn GetMarshalSizeMax(&self, riid: *const windows_core::GUID, pv: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, mshlflags: u32) -> windows_core::Result<u32>;
    fn MarshalInterface(&self, pstm: windows_core::Ref<IStream>, riid: *const windows_core::GUID, pv: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, mshlflags: u32) -> windows_core::Result<()>;
    fn UnmarshalInterface(&self, pstm: windows_core::Ref<IStream>, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ReleaseMarshalData(&self, pstm: windows_core::Ref<IStream>) -> windows_core::Result<()>;
    fn DisconnectObject(&self, dwreserved: u32) -> windows_core::Result<()>;
}
impl IMarshal_Vtbl {
    pub const fn new<Identity: IMarshal_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetUnmarshalClass<Identity: IMarshal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pv: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, mshlflags: u32, pcid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMarshal_Impl::GetUnmarshalClass(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pv), core::mem::transmute_copy(&dwdestcontext), core::mem::transmute_copy(&pvdestcontext), core::mem::transmute_copy(&mshlflags)) {
                    Ok(ok__) => {
                        pcid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMarshalSizeMax<Identity: IMarshal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pv: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, mshlflags: u32, psize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMarshal_Impl::GetMarshalSizeMax(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pv), core::mem::transmute_copy(&dwdestcontext), core::mem::transmute_copy(&pvdestcontext), core::mem::transmute_copy(&mshlflags)) {
                    Ok(ok__) => {
                        psize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MarshalInterface<Identity: IMarshal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void, riid: *const windows_core::GUID, pv: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, mshlflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMarshal_Impl::MarshalInterface(this, core::mem::transmute_copy(&pstm), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pv), core::mem::transmute_copy(&dwdestcontext), core::mem::transmute_copy(&pvdestcontext), core::mem::transmute_copy(&mshlflags)).into()
            }
        }
        unsafe extern "system" fn UnmarshalInterface<Identity: IMarshal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMarshal_Impl::UnmarshalInterface(this, core::mem::transmute_copy(&pstm), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn ReleaseMarshalData<Identity: IMarshal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMarshal_Impl::ReleaseMarshalData(this, core::mem::transmute_copy(&pstm)).into()
            }
        }
        unsafe extern "system" fn DisconnectObject<Identity: IMarshal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMarshal_Impl::DisconnectObject(this, core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUnmarshalClass: GetUnmarshalClass::<Identity, OFFSET>,
            GetMarshalSizeMax: GetMarshalSizeMax::<Identity, OFFSET>,
            MarshalInterface: MarshalInterface::<Identity, OFFSET>,
            UnmarshalInterface: UnmarshalInterface::<Identity, OFFSET>,
            ReleaseMarshalData: ReleaseMarshalData::<Identity, OFFSET>,
            DisconnectObject: DisconnectObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMarshal as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMarshal {}
windows_core::imp::define_interface!(IMarshal2, IMarshal2_Vtbl, 0x000001cf_0000_0000_c000_000000000046);
impl core::ops::Deref for IMarshal2 {
    type Target = IMarshal;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMarshal2, windows_core::IUnknown, IMarshal);
#[repr(C)]
#[doc(hidden)]
pub struct IMarshal2_Vtbl {
    pub base__: IMarshal_Vtbl,
}
pub trait IMarshal2_Impl: IMarshal_Impl {}
impl IMarshal2_Vtbl {
    pub const fn new<Identity: IMarshal2_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IMarshal_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMarshal2 as windows_core::Interface>::IID || iid == &<IMarshal as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMarshal2 {}
windows_core::imp::define_interface!(IMarshalingStream, IMarshalingStream_Vtbl, 0xd8f2f5e6_6102_4863_9f26_389a4676efde);
impl core::ops::Deref for IMarshalingStream {
    type Target = IStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMarshalingStream, windows_core::IUnknown, ISequentialStream, IStream);
impl IMarshalingStream {
    pub unsafe fn GetMarshalingContextAttribute(&self, attribute: CO_MARSHALING_CONTEXT_ATTRIBUTES) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMarshalingContextAttribute)(windows_core::Interface::as_raw(self), attribute, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarshalingStream_Vtbl {
    pub base__: IStream_Vtbl,
    pub GetMarshalingContextAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, CO_MARSHALING_CONTEXT_ATTRIBUTES, *mut usize) -> windows_core::HRESULT,
}
#[cfg(feature = "minwindef")]
pub trait IMarshalingStream_Impl: IStream_Impl {
    fn GetMarshalingContextAttribute(&self, attribute: CO_MARSHALING_CONTEXT_ATTRIBUTES) -> windows_core::Result<usize>;
}
#[cfg(feature = "minwindef")]
impl IMarshalingStream_Vtbl {
    pub const fn new<Identity: IMarshalingStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMarshalingContextAttribute<Identity: IMarshalingStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attribute: CO_MARSHALING_CONTEXT_ATTRIBUTES, pattributevalue: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMarshalingStream_Impl::GetMarshalingContextAttribute(this, core::mem::transmute_copy(&attribute)) {
                    Ok(ok__) => {
                        pattributevalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IStream_Vtbl::new::<Identity, OFFSET>(), GetMarshalingContextAttribute: GetMarshalingContextAttribute::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMarshalingStream as windows_core::Interface>::IID || iid == &<ISequentialStream as windows_core::Interface>::IID || iid == &<IStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "minwindef")]
impl windows_core::RuntimeName for IMarshalingStream {}
windows_core::imp::define_interface!(IMultiQI, IMultiQI_Vtbl, 0x00000020_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IMultiQI, windows_core::IUnknown);
impl IMultiQI {
    pub unsafe fn QueryMultipleInterfaces(&self, pmqis: &mut [MULTI_QI]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryMultipleInterfaces)(windows_core::Interface::as_raw(self), pmqis.len().try_into().unwrap(), core::mem::transmute(pmqis.as_ptr())) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiQI_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryMultipleInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MULTI_QI) -> windows_core::HRESULT,
}
pub trait IMultiQI_Impl: windows_core::IUnknownImpl {
    fn QueryMultipleInterfaces(&self, cmqis: u32, pmqis: *mut MULTI_QI) -> windows_core::Result<()>;
}
impl IMultiQI_Vtbl {
    pub const fn new<Identity: IMultiQI_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryMultipleInterfaces<Identity: IMultiQI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cmqis: u32, pmqis: *mut MULTI_QI) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultiQI_Impl::QueryMultipleInterfaces(this, core::mem::transmute_copy(&cmqis), core::mem::transmute_copy(&pmqis)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryMultipleInterfaces: QueryMultipleInterfaces::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultiQI as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMultiQI {}
windows_core::imp::define_interface!(INoMarshal, INoMarshal_Vtbl, 0xecc8691b_c1db_4dc0_855e_65f6c551af49);
windows_core::imp::interface_hierarchy!(INoMarshal, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct INoMarshal_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait INoMarshal_Impl: windows_core::IUnknownImpl {}
impl INoMarshal_Vtbl {
    pub const fn new<Identity: INoMarshal_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INoMarshal as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for INoMarshal {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IObjContext(pub u8);
windows_core::imp::define_interface!(IPSFactoryBuffer, IPSFactoryBuffer_Vtbl, 0xd5f569d0_593b_101a_b569_08002b2dbf7a);
windows_core::imp::interface_hierarchy!(IPSFactoryBuffer, windows_core::IUnknown);
impl IPSFactoryBuffer {
    pub unsafe fn CreateProxy<P0>(&self, punkouter: P0, riid: *const windows_core::GUID, ppproxy: *mut Option<IRpcProxyBuffer>, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateProxy)(windows_core::Interface::as_raw(self), punkouter.param().abi(), riid, core::mem::transmute(ppproxy), ppv as _) }
    }
    pub unsafe fn CreateStub<P1>(&self, riid: *const windows_core::GUID, punkserver: P1) -> windows_core::Result<IRpcStubBuffer>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStub)(windows_core::Interface::as_raw(self), riid, punkserver.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPSFactoryBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateStub: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPSFactoryBuffer_Impl: windows_core::IUnknownImpl {
    fn CreateProxy(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, riid: *const windows_core::GUID, ppproxy: windows_core::OutRef<IRpcProxyBuffer>, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateStub(&self, riid: *const windows_core::GUID, punkserver: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<IRpcStubBuffer>;
}
impl IPSFactoryBuffer_Vtbl {
    pub const fn new<Identity: IPSFactoryBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateProxy<Identity: IPSFactoryBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppproxy: *mut *mut core::ffi::c_void, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPSFactoryBuffer_Impl::CreateProxy(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppproxy), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn CreateStub<Identity: IPSFactoryBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punkserver: *mut core::ffi::c_void, ppstub: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPSFactoryBuffer_Impl::CreateStub(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&punkserver)) {
                    Ok(ok__) => {
                        ppstub.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateProxy: CreateProxy::<Identity, OFFSET>,
            CreateStub: CreateStub::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPSFactoryBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPSFactoryBuffer {}
windows_core::imp::define_interface!(IPackagedComSyntaxSupport, IPackagedComSyntaxSupport_Vtbl, 0x8f146474_b228_48fb_a58c_105ebb273abc);
windows_core::imp::interface_hierarchy!(IPackagedComSyntaxSupport, windows_core::IUnknown);
impl IPackagedComSyntaxSupport {
    pub unsafe fn GetSupportedVersion(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPackagedComSyntaxSupport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSupportedVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IPackagedComSyntaxSupport_Impl: windows_core::IUnknownImpl {
    fn GetSupportedVersion(&self) -> windows_core::Result<u32>;
}
impl IPackagedComSyntaxSupport_Vtbl {
    pub const fn new<Identity: IPackagedComSyntaxSupport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSupportedVersion<Identity: IPackagedComSyntaxSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedversion: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPackagedComSyntaxSupport_Impl::GetSupportedVersion(this) {
                    Ok(ok__) => {
                        supportedversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSupportedVersion: GetSupportedVersion::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPackagedComSyntaxSupport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPackagedComSyntaxSupport {}
windows_core::imp::define_interface!(IPipeByte, IPipeByte_Vtbl, 0xdb2f3aca_2f86_11d1_8e04_00c04fb9989a);
windows_core::imp::interface_hierarchy!(IPipeByte, windows_core::IUnknown);
impl IPipeByte {
    pub unsafe fn Pull(&self, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Pull)(windows_core::Interface::as_raw(self), buf as _, crequest, pcreturned as _) }
    }
    pub unsafe fn Push(&self, buf: *const u8, csent: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Push)(windows_core::Interface::as_raw(self), buf, csent) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeByte_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
}
pub trait IPipeByte_Impl: windows_core::IUnknownImpl {
    fn Pull(&self, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> windows_core::Result<()>;
    fn Push(&self, buf: *const u8, csent: u32) -> windows_core::Result<()>;
}
impl IPipeByte_Vtbl {
    pub const fn new<Identity: IPipeByte_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Pull<Identity: IPipeByte_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPipeByte_Impl::Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&crequest), core::mem::transmute_copy(&pcreturned)).into()
            }
        }
        unsafe extern "system" fn Push<Identity: IPipeByte_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const u8, csent: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPipeByte_Impl::Push(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&csent)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Pull: Pull::<Identity, OFFSET>, Push: Push::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPipeByte as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPipeByte {}
windows_core::imp::define_interface!(IPipeDouble, IPipeDouble_Vtbl, 0xdb2f3ace_2f86_11d1_8e04_00c04fb9989a);
windows_core::imp::interface_hierarchy!(IPipeDouble, windows_core::IUnknown);
impl IPipeDouble {
    pub unsafe fn Pull(&self, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Pull)(windows_core::Interface::as_raw(self), buf as _, crequest, pcreturned as _) }
    }
    pub unsafe fn Push(&self, buf: *const f64, csent: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Push)(windows_core::Interface::as_raw(self), buf, csent) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeDouble_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, u32, *mut u32) -> windows_core::HRESULT,
    pub Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const f64, u32) -> windows_core::HRESULT,
}
pub trait IPipeDouble_Impl: windows_core::IUnknownImpl {
    fn Pull(&self, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> windows_core::Result<()>;
    fn Push(&self, buf: *const f64, csent: u32) -> windows_core::Result<()>;
}
impl IPipeDouble_Vtbl {
    pub const fn new<Identity: IPipeDouble_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Pull<Identity: IPipeDouble_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPipeDouble_Impl::Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&crequest), core::mem::transmute_copy(&pcreturned)).into()
            }
        }
        unsafe extern "system" fn Push<Identity: IPipeDouble_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const f64, csent: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPipeDouble_Impl::Push(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&csent)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Pull: Pull::<Identity, OFFSET>, Push: Push::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPipeDouble as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPipeDouble {}
windows_core::imp::define_interface!(IPipeLong, IPipeLong_Vtbl, 0xdb2f3acc_2f86_11d1_8e04_00c04fb9989a);
windows_core::imp::interface_hierarchy!(IPipeLong, windows_core::IUnknown);
impl IPipeLong {
    pub unsafe fn Pull(&self, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Pull)(windows_core::Interface::as_raw(self), buf as _, crequest, pcreturned as _) }
    }
    pub unsafe fn Push(&self, buf: *const i32, csent: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Push)(windows_core::Interface::as_raw(self), buf, csent) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeLong_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, u32, *mut u32) -> windows_core::HRESULT,
    pub Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32, u32) -> windows_core::HRESULT,
}
pub trait IPipeLong_Impl: windows_core::IUnknownImpl {
    fn Pull(&self, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> windows_core::Result<()>;
    fn Push(&self, buf: *const i32, csent: u32) -> windows_core::Result<()>;
}
impl IPipeLong_Vtbl {
    pub const fn new<Identity: IPipeLong_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Pull<Identity: IPipeLong_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPipeLong_Impl::Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&crequest), core::mem::transmute_copy(&pcreturned)).into()
            }
        }
        unsafe extern "system" fn Push<Identity: IPipeLong_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const i32, csent: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPipeLong_Impl::Push(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&csent)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Pull: Pull::<Identity, OFFSET>, Push: Push::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPipeLong as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPipeLong {}
windows_core::imp::define_interface!(IProcessInitControl, IProcessInitControl_Vtbl, 0x72380d55_8d2b_43a3_8513_2b6ef31434e9);
windows_core::imp::interface_hierarchy!(IProcessInitControl, windows_core::IUnknown);
impl IProcessInitControl {
    pub unsafe fn ResetInitializerTimeout(&self, dwsecondsremaining: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResetInitializerTimeout)(windows_core::Interface::as_raw(self), dwsecondsremaining) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessInitControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ResetInitializerTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IProcessInitControl_Impl: windows_core::IUnknownImpl {
    fn ResetInitializerTimeout(&self, dwsecondsremaining: u32) -> windows_core::Result<()>;
}
impl IProcessInitControl_Vtbl {
    pub const fn new<Identity: IProcessInitControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ResetInitializerTimeout<Identity: IProcessInitControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsecondsremaining: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProcessInitControl_Impl::ResetInitializerTimeout(this, core::mem::transmute_copy(&dwsecondsremaining)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ResetInitializerTimeout: ResetInitializerTimeout::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProcessInitControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IProcessInitControl {}
windows_core::imp::define_interface!(IReleaseMarshalBuffers, IReleaseMarshalBuffers_Vtbl, 0xeb0cb9e8_7996_11d2_872e_0000f8080859);
windows_core::imp::interface_hierarchy!(IReleaseMarshalBuffers, windows_core::IUnknown);
impl IReleaseMarshalBuffers {
    pub unsafe fn ReleaseMarshalBuffer<P2>(&self, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReleaseMarshalBuffer)(windows_core::Interface::as_raw(self), pmsg as _, dwflags, pchnl.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReleaseMarshalBuffers_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReleaseMarshalBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IReleaseMarshalBuffers_Impl: windows_core::IUnknownImpl {
    fn ReleaseMarshalBuffer(&self, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IReleaseMarshalBuffers_Vtbl {
    pub const fn new<Identity: IReleaseMarshalBuffers_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReleaseMarshalBuffer<Identity: IReleaseMarshalBuffers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReleaseMarshalBuffers_Impl::ReleaseMarshalBuffer(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pchnl)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ReleaseMarshalBuffer: ReleaseMarshalBuffer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReleaseMarshalBuffers as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IReleaseMarshalBuffers {}
windows_core::imp::define_interface!(IRpcChannelBuffer, IRpcChannelBuffer_Vtbl, 0xd5f56b60_593b_101a_b569_08002b2dbf7a);
windows_core::imp::interface_hierarchy!(IRpcChannelBuffer, windows_core::IUnknown);
impl IRpcChannelBuffer {
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), pmessage as _, riid) }
    }
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SendReceive)(windows_core::Interface::as_raw(self), pmessage as _, pstatus.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreeBuffer)(windows_core::Interface::as_raw(self), pmessage as _) }
    }
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDestCtx)(windows_core::Interface::as_raw(self), pdwdestcontext as _, ppvdestcontext as _) }
    }
    pub unsafe fn IsConnected(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsConnected)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SendReceive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *mut u32) -> windows_core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE) -> windows_core::HRESULT,
    pub GetDestCtx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRpcChannelBuffer_Impl: windows_core::IUnknownImpl {
    fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> windows_core::Result<()>;
    fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> windows_core::Result<()>;
    fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn IsConnected(&self) -> windows_core::Result<()>;
}
impl IRpcChannelBuffer_Vtbl {
    pub const fn new<Identity: IRpcChannelBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBuffer<Identity: IRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, riid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer_Impl::GetBuffer(this, core::mem::transmute_copy(&pmessage), core::mem::transmute_copy(&riid)).into()
            }
        }
        unsafe extern "system" fn SendReceive<Identity: IRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer_Impl::SendReceive(this, core::mem::transmute_copy(&pmessage), core::mem::transmute_copy(&pstatus)).into()
            }
        }
        unsafe extern "system" fn FreeBuffer<Identity: IRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer_Impl::FreeBuffer(this, core::mem::transmute_copy(&pmessage)).into()
            }
        }
        unsafe extern "system" fn GetDestCtx<Identity: IRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer_Impl::GetDestCtx(this, core::mem::transmute_copy(&pdwdestcontext), core::mem::transmute_copy(&ppvdestcontext)).into()
            }
        }
        unsafe extern "system" fn IsConnected<Identity: IRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer_Impl::IsConnected(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBuffer: GetBuffer::<Identity, OFFSET>,
            SendReceive: SendReceive::<Identity, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, OFFSET>,
            GetDestCtx: GetDestCtx::<Identity, OFFSET>,
            IsConnected: IsConnected::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcChannelBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRpcChannelBuffer {}
windows_core::imp::define_interface!(IRpcChannelBuffer2, IRpcChannelBuffer2_Vtbl, 0x594f31d0_7f19_11d0_b194_00a0c90dc8bf);
impl core::ops::Deref for IRpcChannelBuffer2 {
    type Target = IRpcChannelBuffer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRpcChannelBuffer2, windows_core::IUnknown, IRpcChannelBuffer);
impl IRpcChannelBuffer2 {
    pub unsafe fn GetProtocolVersion(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProtocolVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBuffer2_Vtbl {
    pub base__: IRpcChannelBuffer_Vtbl,
    pub GetProtocolVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IRpcChannelBuffer2_Impl: IRpcChannelBuffer_Impl {
    fn GetProtocolVersion(&self) -> windows_core::Result<u32>;
}
impl IRpcChannelBuffer2_Vtbl {
    pub const fn new<Identity: IRpcChannelBuffer2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProtocolVersion<Identity: IRpcChannelBuffer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversion: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRpcChannelBuffer2_Impl::GetProtocolVersion(this) {
                    Ok(ok__) => {
                        pdwversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IRpcChannelBuffer_Vtbl::new::<Identity, OFFSET>(), GetProtocolVersion: GetProtocolVersion::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcChannelBuffer2 as windows_core::Interface>::IID || iid == &<IRpcChannelBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRpcChannelBuffer2 {}
windows_core::imp::define_interface!(IRpcChannelBuffer3, IRpcChannelBuffer3_Vtbl, 0x25b15600_0115_11d0_bf0d_00aa00b8dfd2);
impl core::ops::Deref for IRpcChannelBuffer3 {
    type Target = IRpcChannelBuffer2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRpcChannelBuffer3, windows_core::IUnknown, IRpcChannelBuffer, IRpcChannelBuffer2);
impl IRpcChannelBuffer3 {
    pub unsafe fn Send(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Send)(windows_core::Interface::as_raw(self), pmsg as _, pulstatus as _) }
    }
    pub unsafe fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Receive)(windows_core::Interface::as_raw(self), pmsg as _, ulsize, pulstatus as _) }
    }
    pub unsafe fn Cancel(&self, pmsg: *mut RPCOLEMESSAGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self), pmsg as _) }
    }
    pub unsafe fn GetCallContext(&self, pmsg: *const RPCOLEMESSAGE, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCallContext)(windows_core::Interface::as_raw(self), pmsg, riid, pinterface as _) }
    }
    pub unsafe fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDestCtxEx)(windows_core::Interface::as_raw(self), pmsg, pdwdestcontext as _, ppvdestcontext as _) }
    }
    pub unsafe fn GetState(&self, pmsg: *const RPCOLEMESSAGE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), pmsg, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RegisterAsync<P1>(&self, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IAsyncManager>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterAsync)(windows_core::Interface::as_raw(self), pmsg as _, pasyncmgr.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBuffer3_Vtbl {
    pub base__: IRpcChannelBuffer2_Vtbl,
    pub Send: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *mut u32) -> windows_core::HRESULT,
    pub Receive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, u32, *mut u32) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE) -> windows_core::HRESULT,
    pub GetCallContext: unsafe extern "system" fn(*mut core::ffi::c_void, *const RPCOLEMESSAGE, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDestCtxEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const RPCOLEMESSAGE, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *const RPCOLEMESSAGE, *mut u32) -> windows_core::HRESULT,
    pub RegisterAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRpcChannelBuffer3_Impl: IRpcChannelBuffer2_Impl {
    fn Send(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> windows_core::Result<()>;
    fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> windows_core::Result<()>;
    fn Cancel(&self, pmsg: *mut RPCOLEMESSAGE) -> windows_core::Result<()>;
    fn GetCallContext(&self, pmsg: *const RPCOLEMESSAGE, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetState(&self, pmsg: *const RPCOLEMESSAGE) -> windows_core::Result<u32>;
    fn RegisterAsync(&self, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: windows_core::Ref<IAsyncManager>) -> windows_core::Result<()>;
}
impl IRpcChannelBuffer3_Vtbl {
    pub const fn new<Identity: IRpcChannelBuffer3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Send<Identity: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer3_Impl::Send(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&pulstatus)).into()
            }
        }
        unsafe extern "system" fn Receive<Identity: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer3_Impl::Receive(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&ulsize), core::mem::transmute_copy(&pulstatus)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer3_Impl::Cancel(this, core::mem::transmute_copy(&pmsg)).into()
            }
        }
        unsafe extern "system" fn GetCallContext<Identity: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer3_Impl::GetCallContext(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pinterface)).into()
            }
        }
        unsafe extern "system" fn GetDestCtxEx<Identity: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer3_Impl::GetDestCtxEx(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&pdwdestcontext), core::mem::transmute_copy(&ppvdestcontext)).into()
            }
        }
        unsafe extern "system" fn GetState<Identity: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pstate: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRpcChannelBuffer3_Impl::GetState(this, core::mem::transmute_copy(&pmsg)) {
                    Ok(ok__) => {
                        pstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterAsync<Identity: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer3_Impl::RegisterAsync(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&pasyncmgr)).into()
            }
        }
        Self {
            base__: IRpcChannelBuffer2_Vtbl::new::<Identity, OFFSET>(),
            Send: Send::<Identity, OFFSET>,
            Receive: Receive::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            GetCallContext: GetCallContext::<Identity, OFFSET>,
            GetDestCtxEx: GetDestCtxEx::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
            RegisterAsync: RegisterAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcChannelBuffer3 as windows_core::Interface>::IID || iid == &<IRpcChannelBuffer as windows_core::Interface>::IID || iid == &<IRpcChannelBuffer2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRpcChannelBuffer3 {}
windows_core::imp::define_interface!(IRpcHelper, IRpcHelper_Vtbl, 0x00000149_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IRpcHelper, windows_core::IUnknown);
impl IRpcHelper {
    pub unsafe fn GetDCOMProtocolVersion(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDCOMProtocolVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetIIDFromOBJREF(&self, pobjref: *const core::ffi::c_void) -> windows_core::Result<*mut windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIIDFromOBJREF)(windows_core::Interface::as_raw(self), pobjref, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDCOMProtocolVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetIIDFromOBJREF: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IRpcHelper_Impl: windows_core::IUnknownImpl {
    fn GetDCOMProtocolVersion(&self) -> windows_core::Result<u32>;
    fn GetIIDFromOBJREF(&self, pobjref: *const core::ffi::c_void) -> windows_core::Result<*mut windows_core::GUID>;
}
impl IRpcHelper_Vtbl {
    pub const fn new<Identity: IRpcHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDCOMProtocolVersion<Identity: IRpcHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomversion: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRpcHelper_Impl::GetDCOMProtocolVersion(this) {
                    Ok(ok__) => {
                        pcomversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIIDFromOBJREF<Identity: IRpcHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjref: *const core::ffi::c_void, piid: *mut *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRpcHelper_Impl::GetIIDFromOBJREF(this, core::mem::transmute_copy(&pobjref)) {
                    Ok(ok__) => {
                        piid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDCOMProtocolVersion: GetDCOMProtocolVersion::<Identity, OFFSET>,
            GetIIDFromOBJREF: GetIIDFromOBJREF::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcHelper as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRpcHelper {}
windows_core::imp::define_interface!(IRpcOptions, IRpcOptions_Vtbl, 0x00000144_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IRpcOptions, windows_core::IUnknown);
impl IRpcOptions {
    pub unsafe fn Set<P0>(&self, pprx: P0, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), pprx.param().abi(), dwproperty, dwvalue) }
    }
    pub unsafe fn Query<P0>(&self, pprx: P0, dwproperty: RPCOPT_PROPERTIES) -> windows_core::Result<usize>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), pprx.param().abi(), dwproperty, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, RPCOPT_PROPERTIES, usize) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, RPCOPT_PROPERTIES, *mut usize) -> windows_core::HRESULT,
}
pub trait IRpcOptions_Impl: windows_core::IUnknownImpl {
    fn Set(&self, pprx: windows_core::Ref<windows_core::IUnknown>, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> windows_core::Result<()>;
    fn Query(&self, pprx: windows_core::Ref<windows_core::IUnknown>, dwproperty: RPCOPT_PROPERTIES) -> windows_core::Result<usize>;
}
impl IRpcOptions_Vtbl {
    pub const fn new<Identity: IRpcOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Set<Identity: IRpcOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprx: *mut core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcOptions_Impl::Set(this, core::mem::transmute_copy(&pprx), core::mem::transmute_copy(&dwproperty), core::mem::transmute_copy(&dwvalue)).into()
            }
        }
        unsafe extern "system" fn Query<Identity: IRpcOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprx: *mut core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, pdwvalue: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRpcOptions_Impl::Query(this, core::mem::transmute_copy(&pprx), core::mem::transmute_copy(&dwproperty)) {
                    Ok(ok__) => {
                        pdwvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Set: Set::<Identity, OFFSET>, Query: Query::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcOptions as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRpcOptions {}
windows_core::imp::define_interface!(IRpcProxyBuffer, IRpcProxyBuffer_Vtbl, 0xd5f56a34_593b_101a_b569_08002b2dbf7a);
windows_core::imp::interface_hierarchy!(IRpcProxyBuffer, windows_core::IUnknown);
impl IRpcProxyBuffer {
    pub unsafe fn Connect<P0>(&self, prpcchannelbuffer: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IRpcChannelBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), prpcchannelbuffer.param().abi()) }
    }
    pub unsafe fn Disconnect(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcProxyBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait IRpcProxyBuffer_Impl: windows_core::IUnknownImpl {
    fn Connect(&self, prpcchannelbuffer: windows_core::Ref<IRpcChannelBuffer>) -> windows_core::Result<()>;
    fn Disconnect(&self);
}
impl IRpcProxyBuffer_Vtbl {
    pub const fn new<Identity: IRpcProxyBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Connect<Identity: IRpcProxyBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prpcchannelbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcProxyBuffer_Impl::Connect(this, core::mem::transmute_copy(&prpcchannelbuffer)).into()
            }
        }
        unsafe extern "system" fn Disconnect<Identity: IRpcProxyBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcProxyBuffer_Impl::Disconnect(this);
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Connect: Connect::<Identity, OFFSET>, Disconnect: Disconnect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcProxyBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRpcProxyBuffer {}
windows_core::imp::define_interface!(IRpcStubBuffer, IRpcStubBuffer_Vtbl, 0xd5f56afc_593b_101a_b569_08002b2dbf7a);
windows_core::imp::interface_hierarchy!(IRpcStubBuffer, windows_core::IUnknown);
impl IRpcStubBuffer {
    pub unsafe fn Connect<P0>(&self, punkserver: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), punkserver.param().abi()) }
    }
    pub unsafe fn Disconnect(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn Invoke<P1>(&self, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IRpcChannelBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), _prpcmsg as _, _prpcchannelbuffer.param().abi()) }
    }
    pub unsafe fn IsIIDSupported(&self, riid: *const windows_core::GUID) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).IsIIDSupported)(windows_core::Interface::as_raw(self), riid) }
    }
    pub unsafe fn CountRefs(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).CountRefs)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn DebugServerQueryInterface(&self, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DebugServerQueryInterface)(windows_core::Interface::as_raw(self), ppv as _) }
    }
    pub unsafe fn DebugServerRelease(&self, pv: *const core::ffi::c_void) {
        unsafe {
            (windows_core::Interface::vtable(self).DebugServerRelease)(windows_core::Interface::as_raw(self), pv);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcStubBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsIIDSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> Option<IRpcStubBuffer>,
    pub CountRefs: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub DebugServerQueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DebugServerRelease: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void),
}
pub trait IRpcStubBuffer_Impl: windows_core::IUnknownImpl {
    fn Connect(&self, punkserver: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Disconnect(&self);
    fn Invoke(&self, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: windows_core::Ref<IRpcChannelBuffer>) -> windows_core::Result<()>;
    fn IsIIDSupported(&self, riid: *const windows_core::GUID) -> Option<IRpcStubBuffer>;
    fn CountRefs(&self) -> u32;
    fn DebugServerQueryInterface(&self, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn DebugServerRelease(&self, pv: *const core::ffi::c_void);
}
impl IRpcStubBuffer_Vtbl {
    pub const fn new<Identity: IRpcStubBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Connect<Identity: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcStubBuffer_Impl::Connect(this, core::mem::transmute_copy(&punkserver)).into()
            }
        }
        unsafe extern "system" fn Disconnect<Identity: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcStubBuffer_Impl::Disconnect(this);
            }
        }
        unsafe extern "system" fn Invoke<Identity: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcStubBuffer_Impl::Invoke(this, core::mem::transmute_copy(&_prpcmsg), core::mem::transmute_copy(&_prpcchannelbuffer)).into()
            }
        }
        unsafe extern "system" fn IsIIDSupported<Identity: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID) -> Option<IRpcStubBuffer> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcStubBuffer_Impl::IsIIDSupported(this, core::mem::transmute_copy(&riid))
            }
        }
        unsafe extern "system" fn CountRefs<Identity: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcStubBuffer_Impl::CountRefs(this)
            }
        }
        unsafe extern "system" fn DebugServerQueryInterface<Identity: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcStubBuffer_Impl::DebugServerQueryInterface(this, core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn DebugServerRelease<Identity: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcStubBuffer_Impl::DebugServerRelease(this, core::mem::transmute_copy(&pv));
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            Invoke: Invoke::<Identity, OFFSET>,
            IsIIDSupported: IsIIDSupported::<Identity, OFFSET>,
            CountRefs: CountRefs::<Identity, OFFSET>,
            DebugServerQueryInterface: DebugServerQueryInterface::<Identity, OFFSET>,
            DebugServerRelease: DebugServerRelease::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcStubBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRpcStubBuffer {}
windows_core::imp::define_interface!(IRpcSyntaxNegotiate, IRpcSyntaxNegotiate_Vtbl, 0x58a08519_24c8_4935_b482_3fd823333a4f);
windows_core::imp::interface_hierarchy!(IRpcSyntaxNegotiate, windows_core::IUnknown);
impl IRpcSyntaxNegotiate {
    pub unsafe fn NegotiateSyntax(&self, pmsg: *mut RPCOLEMESSAGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NegotiateSyntax)(windows_core::Interface::as_raw(self), pmsg as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcSyntaxNegotiate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NegotiateSyntax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE) -> windows_core::HRESULT,
}
pub trait IRpcSyntaxNegotiate_Impl: windows_core::IUnknownImpl {
    fn NegotiateSyntax(&self, pmsg: *mut RPCOLEMESSAGE) -> windows_core::Result<()>;
}
impl IRpcSyntaxNegotiate_Vtbl {
    pub const fn new<Identity: IRpcSyntaxNegotiate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NegotiateSyntax<Identity: IRpcSyntaxNegotiate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcSyntaxNegotiate_Impl::NegotiateSyntax(this, core::mem::transmute_copy(&pmsg)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NegotiateSyntax: NegotiateSyntax::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRpcSyntaxNegotiate as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRpcSyntaxNegotiate {}
windows_core::imp::define_interface!(ISequentialStream, ISequentialStream_Vtbl, 0x0c733a30_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ISequentialStream, windows_core::IUnknown);
impl ISequentialStream {
    pub unsafe fn Read(&self, pv: *mut core::ffi::c_void, cb: u32, pcbread: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pv as _, cb, pcbread.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Write(&self, pv: *const core::ffi::c_void, cb: u32, pcbwritten: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), pv, cb, pcbwritten.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISequentialStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
}
pub trait ISequentialStream_Impl: windows_core::IUnknownImpl {
    fn Read(&self, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32) -> windows_core::Result<()>;
    fn Write(&self, pv: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> windows_core::Result<()>;
}
impl ISequentialStream_Vtbl {
    pub const fn new<Identity: ISequentialStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: ISequentialStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISequentialStream_Impl::Read(this, core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbread)).into()
            }
        }
        unsafe extern "system" fn Write<Identity: ISequentialStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISequentialStream_Impl::Write(this, core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbwritten)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Read: Read::<Identity, OFFSET>, Write: Write::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISequentialStream as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISequentialStream {}
windows_core::imp::define_interface!(IServerSecurity, IServerSecurity_Vtbl, 0x0000013e_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IServerSecurity, windows_core::IUnknown);
impl IServerSecurity {
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn QueryBlanket(&self, pauthnsvc: Option<*mut u32>, pauthzsvc: Option<*mut u32>, pserverprincname: *mut *mut super::wtypesbase::OLECHAR, pauthnlevel: Option<*mut u32>, pimplevel: Option<*mut u32>, pprivs: *mut *mut core::ffi::c_void, pcapabilities: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryBlanket)(windows_core::Interface::as_raw(self), pauthnsvc.unwrap_or(core::mem::zeroed()) as _, pauthzsvc.unwrap_or(core::mem::zeroed()) as _, pserverprincname as _, pauthnlevel.unwrap_or(core::mem::zeroed()) as _, pimplevel.unwrap_or(core::mem::zeroed()) as _, pprivs as _, pcapabilities.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn ImpersonateClient(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ImpersonateClient)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RevertToSelf(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RevertToSelf)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsImpersonating(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsImpersonating)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServerSecurity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wtypesbase")]
    pub QueryBlanket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut *mut super::wtypesbase::OLECHAR, *mut u32, *mut u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    QueryBlanket: usize,
    pub ImpersonateClient: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RevertToSelf: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsImpersonating: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
#[cfg(feature = "wtypesbase")]
pub trait IServerSecurity_Impl: windows_core::IUnknownImpl {
    fn QueryBlanket(&self, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut super::wtypesbase::OLECHAR, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut core::ffi::c_void, pcapabilities: *mut u32) -> windows_core::Result<()>;
    fn ImpersonateClient(&self) -> windows_core::Result<()>;
    fn RevertToSelf(&self) -> windows_core::Result<()>;
    fn IsImpersonating(&self) -> windows_core::BOOL;
}
#[cfg(feature = "wtypesbase")]
impl IServerSecurity_Vtbl {
    pub const fn new<Identity: IServerSecurity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryBlanket<Identity: IServerSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut super::wtypesbase::OLECHAR, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut core::ffi::c_void, pcapabilities: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServerSecurity_Impl::QueryBlanket(this, core::mem::transmute_copy(&pauthnsvc), core::mem::transmute_copy(&pauthzsvc), core::mem::transmute_copy(&pserverprincname), core::mem::transmute_copy(&pauthnlevel), core::mem::transmute_copy(&pimplevel), core::mem::transmute_copy(&pprivs), core::mem::transmute_copy(&pcapabilities)).into()
            }
        }
        unsafe extern "system" fn ImpersonateClient<Identity: IServerSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServerSecurity_Impl::ImpersonateClient(this).into()
            }
        }
        unsafe extern "system" fn RevertToSelf<Identity: IServerSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServerSecurity_Impl::RevertToSelf(this).into()
            }
        }
        unsafe extern "system" fn IsImpersonating<Identity: IServerSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServerSecurity_Impl::IsImpersonating(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryBlanket: QueryBlanket::<Identity, OFFSET>,
            ImpersonateClient: ImpersonateClient::<Identity, OFFSET>,
            RevertToSelf: RevertToSelf::<Identity, OFFSET>,
            IsImpersonating: IsImpersonating::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServerSecurity as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wtypesbase")]
impl windows_core::RuntimeName for IServerSecurity {}
windows_core::imp::define_interface!(IStdMarshalInfo, IStdMarshalInfo_Vtbl, 0x00000018_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IStdMarshalInfo, windows_core::IUnknown);
impl IStdMarshalInfo {
    pub unsafe fn GetClassForHandler(&self, dwdestcontext: u32, pvdestcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClassForHandler)(windows_core::Interface::as_raw(self), dwdestcontext, pvdestcontext.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStdMarshalInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClassForHandler: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IStdMarshalInfo_Impl: windows_core::IUnknownImpl {
    fn GetClassForHandler(&self, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void) -> windows_core::Result<windows_core::GUID>;
}
impl IStdMarshalInfo_Vtbl {
    pub const fn new<Identity: IStdMarshalInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClassForHandler<Identity: IStdMarshalInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStdMarshalInfo_Impl::GetClassForHandler(this, core::mem::transmute_copy(&dwdestcontext), core::mem::transmute_copy(&pvdestcontext)) {
                    Ok(ok__) => {
                        pclsid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetClassForHandler: GetClassForHandler::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStdMarshalInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IStdMarshalInfo {}
windows_core::imp::define_interface!(IStream, IStream_Vtbl, 0x0000000c_0000_0000_c000_000000000046);
impl core::ops::Deref for IStream {
    type Target = ISequentialStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStream, windows_core::IUnknown, ISequentialStream);
impl IStream {
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: u32, plibnewposition: Option<*mut u64>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Seek)(windows_core::Interface::as_raw(self), dlibmove, dworigin, plibnewposition.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetSize(&self, libnewsize: u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), libnewsize) }
    }
    pub unsafe fn CopyTo<P0>(&self, pstm: P0, cb: u64, pcbread: Option<*mut u64>, pcbwritten: Option<*mut u64>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyTo)(windows_core::Interface::as_raw(self), pstm.param().abi(), cb, pcbread.unwrap_or(core::mem::zeroed()) as _, pcbwritten.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), grfcommitflags) }
    }
    pub unsafe fn Revert(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Revert)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockRegion)(windows_core::Interface::as_raw(self), liboffset, cb, dwlocktype) }
    }
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockRegion)(windows_core::Interface::as_raw(self), liboffset, cb, dwlocktype) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn Stat(&self, pstatstg: *mut STATSTG, grfstatflag: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stat)(windows_core::Interface::as_raw(self), pstatstg as _, grfstatflag) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStream_Vtbl {
    pub base__: ISequentialStream_Vtbl,
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, i64, u32, *mut u64) -> windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub CopyTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, *mut u64, *mut u64) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Revert: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LockRegion: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u32) -> windows_core::HRESULT,
    pub UnlockRegion: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u32) -> windows_core::HRESULT,
    #[cfg(feature = "minwindef")]
    pub Stat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STATSTG, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    Stat: usize,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "minwindef")]
pub trait IStream_Impl: ISequentialStream_Impl {
    fn Seek(&self, dlibmove: i64, dworigin: u32, plibnewposition: *mut u64) -> windows_core::Result<()>;
    fn SetSize(&self, libnewsize: u64) -> windows_core::Result<()>;
    fn CopyTo(&self, pstm: windows_core::Ref<IStream>, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> windows_core::Result<()>;
    fn Commit(&self, grfcommitflags: u32) -> windows_core::Result<()>;
    fn Revert(&self) -> windows_core::Result<()>;
    fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::Result<()>;
    fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::Result<()>;
    fn Stat(&self, pstatstg: *mut STATSTG, grfstatflag: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IStream>;
}
#[cfg(feature = "minwindef")]
impl IStream_Vtbl {
    pub const fn new<Identity: IStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Seek<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dlibmove: i64, dworigin: u32, plibnewposition: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::Seek(this, core::mem::transmute_copy(&dlibmove), core::mem::transmute_copy(&dworigin), core::mem::transmute_copy(&plibnewposition)).into()
            }
        }
        unsafe extern "system" fn SetSize<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, libnewsize: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::SetSize(this, core::mem::transmute_copy(&libnewsize)).into()
            }
        }
        unsafe extern "system" fn CopyTo<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::CopyTo(this, core::mem::transmute_copy(&pstm), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbread), core::mem::transmute_copy(&pcbwritten)).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfcommitflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::Commit(this, core::mem::transmute_copy(&grfcommitflags)).into()
            }
        }
        unsafe extern "system" fn Revert<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::Revert(this).into()
            }
        }
        unsafe extern "system" fn LockRegion<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::LockRegion(this, core::mem::transmute_copy(&liboffset), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&dwlocktype)).into()
            }
        }
        unsafe extern "system" fn UnlockRegion<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::UnlockRegion(this, core::mem::transmute_copy(&liboffset), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&dwlocktype)).into()
            }
        }
        unsafe extern "system" fn Stat<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatstg: *mut STATSTG, grfstatflag: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::Stat(this, core::mem::transmute_copy(&pstatstg), core::mem::transmute_copy(&grfstatflag)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStream_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppstm.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ISequentialStream_Vtbl::new::<Identity, OFFSET>(),
            Seek: Seek::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
            CopyTo: CopyTo::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            Revert: Revert::<Identity, OFFSET>,
            LockRegion: LockRegion::<Identity, OFFSET>,
            UnlockRegion: UnlockRegion::<Identity, OFFSET>,
            Stat: Stat::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStream as windows_core::Interface>::IID || iid == &<ISequentialStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "minwindef")]
impl windows_core::RuntimeName for IStream {}
windows_core::imp::define_interface!(ISupportActivateAsActivatorPackaged, ISupportActivateAsActivatorPackaged_Vtbl, 0x765d1df2_f0af_4ef8_aa50_84789ca330ed);
windows_core::imp::interface_hierarchy!(ISupportActivateAsActivatorPackaged, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct ISupportActivateAsActivatorPackaged_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait ISupportActivateAsActivatorPackaged_Impl: windows_core::IUnknownImpl {}
impl ISupportActivateAsActivatorPackaged_Vtbl {
    pub const fn new<Identity: ISupportActivateAsActivatorPackaged_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISupportActivateAsActivatorPackaged as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISupportActivateAsActivatorPackaged {}
windows_core::imp::define_interface!(ISupportActivationFromPackage, ISupportActivationFromPackage_Vtbl, 0x0a18aae5_5caa_48c5_a9f4_6e46dcd58ad5);
windows_core::imp::interface_hierarchy!(ISupportActivationFromPackage, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct ISupportActivationFromPackage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait ISupportActivationFromPackage_Impl: windows_core::IUnknownImpl {}
impl ISupportActivationFromPackage_Vtbl {
    pub const fn new<Identity: ISupportActivationFromPackage_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISupportActivationFromPackage as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISupportActivationFromPackage {}
windows_core::imp::define_interface!(ISupportAllowLowerTrustActivation, ISupportAllowLowerTrustActivation_Vtbl, 0xe9956ef2_3828_4b4b_8fa9_7db61dee4954);
windows_core::imp::interface_hierarchy!(ISupportAllowLowerTrustActivation, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct ISupportAllowLowerTrustActivation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait ISupportAllowLowerTrustActivation_Impl: windows_core::IUnknownImpl {}
impl ISupportAllowLowerTrustActivation_Vtbl {
    pub const fn new<Identity: ISupportAllowLowerTrustActivation_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISupportAllowLowerTrustActivation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISupportAllowLowerTrustActivation {}
windows_core::imp::define_interface!(ISupportCoAddComDependencyOnPackage, ISupportCoAddComDependencyOnPackage_Vtbl, 0xc8059efc_4e98_4fd0_bfc6_44190b80b823);
windows_core::imp::interface_hierarchy!(ISupportCoAddComDependencyOnPackage, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct ISupportCoAddComDependencyOnPackage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait ISupportCoAddComDependencyOnPackage_Impl: windows_core::IUnknownImpl {}
impl ISupportCoAddComDependencyOnPackage_Vtbl {
    pub const fn new<Identity: ISupportCoAddComDependencyOnPackage_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISupportCoAddComDependencyOnPackage as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISupportCoAddComDependencyOnPackage {}
windows_core::imp::define_interface!(ISupportDoNotElevateServerActivation, ISupportDoNotElevateServerActivation_Vtbl, 0x40aefe22_3ff6_43dc_8108_c8c402d57b5c);
windows_core::imp::interface_hierarchy!(ISupportDoNotElevateServerActivation, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct ISupportDoNotElevateServerActivation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait ISupportDoNotElevateServerActivation_Impl: windows_core::IUnknownImpl {}
impl ISupportDoNotElevateServerActivation_Vtbl {
    pub const fn new<Identity: ISupportDoNotElevateServerActivation_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISupportDoNotElevateServerActivation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISupportDoNotElevateServerActivation {}
windows_core::imp::define_interface!(ISupportPackagedComElevationEnabledClasses, ISupportPackagedComElevationEnabledClasses_Vtbl, 0xb4219019_f712_4d4f_ade7_f468276af0b8);
windows_core::imp::interface_hierarchy!(ISupportPackagedComElevationEnabledClasses, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct ISupportPackagedComElevationEnabledClasses_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait ISupportPackagedComElevationEnabledClasses_Impl: windows_core::IUnknownImpl {}
impl ISupportPackagedComElevationEnabledClasses_Vtbl {
    pub const fn new<Identity: ISupportPackagedComElevationEnabledClasses_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISupportPackagedComElevationEnabledClasses as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISupportPackagedComElevationEnabledClasses {}
windows_core::imp::define_interface!(ISupportPackagedComRegistrationVisibility, ISupportPackagedComRegistrationVisibility_Vtbl, 0x8dc3444e_c7ee_449a_9fb8_b9173988d66a);
windows_core::imp::interface_hierarchy!(ISupportPackagedComRegistrationVisibility, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct ISupportPackagedComRegistrationVisibility_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait ISupportPackagedComRegistrationVisibility_Impl: windows_core::IUnknownImpl {}
impl ISupportPackagedComRegistrationVisibility_Vtbl {
    pub const fn new<Identity: ISupportPackagedComRegistrationVisibility_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISupportPackagedComRegistrationVisibility as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISupportPackagedComRegistrationVisibility {}
windows_core::imp::define_interface!(ISupportServerMustBeEqualOrGreaterPrivilegeActivation, ISupportServerMustBeEqualOrGreaterPrivilegeActivation_Vtbl, 0x5bdb3ee2_46bc_4313_b5fb_801c360ba5f9);
windows_core::imp::interface_hierarchy!(ISupportServerMustBeEqualOrGreaterPrivilegeActivation, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct ISupportServerMustBeEqualOrGreaterPrivilegeActivation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait ISupportServerMustBeEqualOrGreaterPrivilegeActivation_Impl: windows_core::IUnknownImpl {}
impl ISupportServerMustBeEqualOrGreaterPrivilegeActivation_Vtbl {
    pub const fn new<Identity: ISupportServerMustBeEqualOrGreaterPrivilegeActivation_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISupportServerMustBeEqualOrGreaterPrivilegeActivation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISupportServerMustBeEqualOrGreaterPrivilegeActivation {}
windows_core::imp::define_interface!(ISurrogate, ISurrogate_Vtbl, 0x00000022_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ISurrogate, windows_core::IUnknown);
impl ISurrogate {
    pub unsafe fn LoadDllServer(&self, clsid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LoadDllServer)(windows_core::Interface::as_raw(self), clsid) }
    }
    pub unsafe fn FreeSurrogate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreeSurrogate)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurrogate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LoadDllServer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub FreeSurrogate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISurrogate_Impl: windows_core::IUnknownImpl {
    fn LoadDllServer(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn FreeSurrogate(&self) -> windows_core::Result<()>;
}
impl ISurrogate_Vtbl {
    pub const fn new<Identity: ISurrogate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LoadDllServer<Identity: ISurrogate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurrogate_Impl::LoadDllServer(this, core::mem::transmute_copy(&clsid)).into()
            }
        }
        unsafe extern "system" fn FreeSurrogate<Identity: ISurrogate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurrogate_Impl::FreeSurrogate(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LoadDllServer: LoadDllServer::<Identity, OFFSET>,
            FreeSurrogate: FreeSurrogate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISurrogate as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISurrogate {}
windows_core::imp::define_interface!(ISynchronize, ISynchronize_Vtbl, 0x00000030_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ISynchronize, windows_core::IUnknown);
impl ISynchronize {
    pub unsafe fn Wait(&self, dwflags: u32, dwmilliseconds: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Wait)(windows_core::Interface::as_raw(self), dwflags, dwmilliseconds) }
    }
    pub unsafe fn Signal(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Signal)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronize_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Wait: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub Signal: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISynchronize_Impl: windows_core::IUnknownImpl {
    fn Wait(&self, dwflags: u32, dwmilliseconds: u32) -> windows_core::Result<()>;
    fn Signal(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl ISynchronize_Vtbl {
    pub const fn new<Identity: ISynchronize_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Wait<Identity: ISynchronize_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, dwmilliseconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISynchronize_Impl::Wait(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwmilliseconds)).into()
            }
        }
        unsafe extern "system" fn Signal<Identity: ISynchronize_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISynchronize_Impl::Signal(this).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: ISynchronize_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISynchronize_Impl::Reset(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Wait: Wait::<Identity, OFFSET>,
            Signal: Signal::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISynchronize as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISynchronize {}
windows_core::imp::define_interface!(ISynchronizeContainer, ISynchronizeContainer_Vtbl, 0x00000033_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ISynchronizeContainer, windows_core::IUnknown);
impl ISynchronizeContainer {
    pub unsafe fn AddSynchronize<P0>(&self, psync: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISynchronize>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddSynchronize)(windows_core::Interface::as_raw(self), psync.param().abi()) }
    }
    pub unsafe fn WaitMultiple(&self, dwflags: u32, dwtimeout: u32) -> windows_core::Result<ISynchronize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WaitMultiple)(windows_core::Interface::as_raw(self), dwflags, dwtimeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeContainer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddSynchronize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISynchronizeContainer_Impl: windows_core::IUnknownImpl {
    fn AddSynchronize(&self, psync: windows_core::Ref<ISynchronize>) -> windows_core::Result<()>;
    fn WaitMultiple(&self, dwflags: u32, dwtimeout: u32) -> windows_core::Result<ISynchronize>;
}
impl ISynchronizeContainer_Vtbl {
    pub const fn new<Identity: ISynchronizeContainer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddSynchronize<Identity: ISynchronizeContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psync: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISynchronizeContainer_Impl::AddSynchronize(this, core::mem::transmute_copy(&psync)).into()
            }
        }
        unsafe extern "system" fn WaitMultiple<Identity: ISynchronizeContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, dwtimeout: u32, ppsync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISynchronizeContainer_Impl::WaitMultiple(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwtimeout)) {
                    Ok(ok__) => {
                        ppsync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddSynchronize: AddSynchronize::<Identity, OFFSET>,
            WaitMultiple: WaitMultiple::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISynchronizeContainer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISynchronizeContainer {}
windows_core::imp::define_interface!(ISynchronizeEvent, ISynchronizeEvent_Vtbl, 0x00000032_0000_0000_c000_000000000046);
impl core::ops::Deref for ISynchronizeEvent {
    type Target = ISynchronizeHandle;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISynchronizeEvent, windows_core::IUnknown, ISynchronizeHandle);
impl ISynchronizeEvent {
    #[cfg(feature = "winnt")]
    pub unsafe fn SetEventHandle(&self, ph: *const super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEventHandle)(windows_core::Interface::as_raw(self), ph) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeEvent_Vtbl {
    pub base__: ISynchronizeHandle_Vtbl,
    #[cfg(feature = "winnt")]
    pub SetEventHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SetEventHandle: usize,
}
#[cfg(feature = "winnt")]
pub trait ISynchronizeEvent_Impl: ISynchronizeHandle_Impl {
    fn SetEventHandle(&self, ph: *const super::winnt::HANDLE) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl ISynchronizeEvent_Vtbl {
    pub const fn new<Identity: ISynchronizeEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetEventHandle<Identity: ISynchronizeEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ph: *const super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISynchronizeEvent_Impl::SetEventHandle(this, core::mem::transmute_copy(&ph)).into()
            }
        }
        Self { base__: ISynchronizeHandle_Vtbl::new::<Identity, OFFSET>(), SetEventHandle: SetEventHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISynchronizeEvent as windows_core::Interface>::IID || iid == &<ISynchronizeHandle as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for ISynchronizeEvent {}
windows_core::imp::define_interface!(ISynchronizeHandle, ISynchronizeHandle_Vtbl, 0x00000031_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ISynchronizeHandle, windows_core::IUnknown);
impl ISynchronizeHandle {
    #[cfg(feature = "winnt")]
    pub unsafe fn GetHandle(&self) -> windows_core::Result<super::winnt::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeHandle_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub GetHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetHandle: usize,
}
#[cfg(feature = "winnt")]
pub trait ISynchronizeHandle_Impl: windows_core::IUnknownImpl {
    fn GetHandle(&self) -> windows_core::Result<super::winnt::HANDLE>;
}
#[cfg(feature = "winnt")]
impl ISynchronizeHandle_Vtbl {
    pub const fn new<Identity: ISynchronizeHandle_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetHandle<Identity: ISynchronizeHandle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ph: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISynchronizeHandle_Impl::GetHandle(this) {
                    Ok(ok__) => {
                        ph.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetHandle: GetHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISynchronizeHandle as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for ISynchronizeHandle {}
windows_core::imp::define_interface!(ISynchronizeMutex, ISynchronizeMutex_Vtbl, 0x00000025_0000_0000_c000_000000000046);
impl core::ops::Deref for ISynchronizeMutex {
    type Target = ISynchronize;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISynchronizeMutex, windows_core::IUnknown, ISynchronize);
impl ISynchronizeMutex {
    pub unsafe fn ReleaseMutex(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseMutex)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeMutex_Vtbl {
    pub base__: ISynchronize_Vtbl,
    pub ReleaseMutex: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISynchronizeMutex_Impl: ISynchronize_Impl {
    fn ReleaseMutex(&self) -> windows_core::Result<()>;
}
impl ISynchronizeMutex_Vtbl {
    pub const fn new<Identity: ISynchronizeMutex_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReleaseMutex<Identity: ISynchronizeMutex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISynchronizeMutex_Impl::ReleaseMutex(this).into()
            }
        }
        Self { base__: ISynchronize_Vtbl::new::<Identity, OFFSET>(), ReleaseMutex: ReleaseMutex::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISynchronizeMutex as windows_core::Interface>::IID || iid == &<ISynchronize as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISynchronizeMutex {}
windows_core::imp::define_interface!(IWaitMultiple, IWaitMultiple_Vtbl, 0x0000002b_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IWaitMultiple, windows_core::IUnknown);
impl IWaitMultiple {
    pub unsafe fn WaitMultiple(&self, timeout: u32) -> windows_core::Result<ISynchronize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WaitMultiple)(windows_core::Interface::as_raw(self), timeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddSynchronize<P0>(&self, psync: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISynchronize>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddSynchronize)(windows_core::Interface::as_raw(self), psync.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWaitMultiple_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub WaitMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddSynchronize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWaitMultiple_Impl: windows_core::IUnknownImpl {
    fn WaitMultiple(&self, timeout: u32) -> windows_core::Result<ISynchronize>;
    fn AddSynchronize(&self, psync: windows_core::Ref<ISynchronize>) -> windows_core::Result<()>;
}
impl IWaitMultiple_Vtbl {
    pub const fn new<Identity: IWaitMultiple_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WaitMultiple<Identity: IWaitMultiple_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeout: u32, psync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWaitMultiple_Impl::WaitMultiple(this, core::mem::transmute_copy(&timeout)) {
                    Ok(ok__) => {
                        psync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddSynchronize<Identity: IWaitMultiple_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psync: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWaitMultiple_Impl::AddSynchronize(this, core::mem::transmute_copy(&psync)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WaitMultiple: WaitMultiple::<Identity, OFFSET>,
            AddSynchronize: AddSynchronize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWaitMultiple as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWaitMultiple {}
pub type LOCKTYPE = i32;
pub const LOCK_EXCLUSIVE: LOCKTYPE = 2;
pub const LOCK_ONLYONCE: LOCKTYPE = 4;
pub const LOCK_WRITE: LOCKTYPE = 1;
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct MULTI_QI {
    pub pIID: *const windows_core::GUID,
    pub pItf: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub hr: windows_core::HRESULT,
}
impl Default for MULTI_QI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MachineGlobalObjectTableRegistrationToken(pub *mut core::ffi::c_void);
impl MachineGlobalObjectTableRegistrationToken {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for MachineGlobalObjectTableRegistrationToken {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPCOLEMESSAGE(pub *mut RPCOLEMESSAGE);
impl PRPCOLEMESSAGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPCOLEMESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSOLE_AUTHENTICATION_INFO(pub *mut SOLE_AUTHENTICATION_INFO);
impl PSOLE_AUTHENTICATION_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSOLE_AUTHENTICATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSOLE_AUTHENTICATION_LIST(pub *mut SOLE_AUTHENTICATION_LIST);
impl PSOLE_AUTHENTICATION_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSOLE_AUTHENTICATION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "wtypesbase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSOLE_AUTHENTICATION_SERVICE(pub *mut SOLE_AUTHENTICATION_SERVICE);
#[cfg(feature = "wtypesbase")]
impl PSOLE_AUTHENTICATION_SERVICE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "wtypesbase")]
impl Default for PSOLE_AUTHENTICATION_SERVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct RPCOLEDATAREP(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPCOLEMESSAGE {
    pub reserved1: *mut core::ffi::c_void,
    pub dataRepresentation: RPCOLEDATAREP,
    pub Buffer: *mut core::ffi::c_void,
    pub cbBuffer: u32,
    pub iMethod: u32,
    pub reserved2: [*mut core::ffi::c_void; 5],
    pub rpcFlags: u32,
}
impl Default for RPCOLEMESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RPCOPT_PROPERTIES = i32;
pub type RPCOPT_SERVER_LOCALITY_VALUES = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SChannelHookCallInfo {
    pub iid: windows_core::GUID,
    pub cbSize: u32,
    pub uCausality: windows_core::GUID,
    pub dwServerPid: u32,
    pub iMethod: u32,
    pub pObject: *mut core::ffi::c_void,
}
impl Default for SChannelHookCallInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SERVER_LOCALITY_MACHINE_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = 1;
pub const SERVER_LOCALITY_PROCESS_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = 0;
pub const SERVER_LOCALITY_REMOTE: RPCOPT_SERVER_LOCALITY_VALUES = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SOLE_AUTHENTICATION_INFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pAuthInfo: *mut core::ffi::c_void,
}
impl Default for SOLE_AUTHENTICATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SOLE_AUTHENTICATION_LIST {
    pub cAuthInfo: u32,
    pub aAuthInfo: *mut SOLE_AUTHENTICATION_INFO,
}
impl Default for SOLE_AUTHENTICATION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wtypesbase")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SOLE_AUTHENTICATION_SERVICE {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pPrincipalName: *mut super::wtypesbase::OLECHAR,
    pub hr: windows_core::HRESULT,
}
#[cfg(feature = "wtypesbase")]
impl Default for SOLE_AUTHENTICATION_SERVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STATSTG {
    pub pwcsName: windows_core::PWSTR,
    pub r#type: u32,
    pub cbSize: u64,
    pub mtime: super::minwindef::FILETIME,
    pub ctime: super::minwindef::FILETIME,
    pub atime: super::minwindef::FILETIME,
    pub grfMode: u32,
    pub grfLocksSupported: u32,
    pub clsid: windows_core::GUID,
    pub grfStateBits: u32,
    pub reserved: u32,
}
pub type STGTY = i32;
pub const STGTY_LOCKBYTES: STGTY = 3;
pub const STGTY_PROPERTY: STGTY = 4;
pub const STGTY_STORAGE: STGTY = 1;
pub const STGTY_STREAM: STGTY = 2;
pub type STREAM_SEEK = i32;
pub const STREAM_SEEK_CUR: STREAM_SEEK = 1;
pub const STREAM_SEEK_END: STREAM_SEEK = 2;
pub const STREAM_SEEK_SET: STREAM_SEEK = 0;
pub type THDTYPE = i32;
pub const THDTYPE_BLOCKMESSAGES: THDTYPE = 0;
pub const THDTYPE_PROCESSMESSAGES: THDTYPE = 1;
