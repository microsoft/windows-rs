pub type ACTIVATIONTYPE = i32;
pub const ACTIVATIONTYPE_FROM_DATA: ACTIVATIONTYPE = 2;
pub const ACTIVATIONTYPE_FROM_FILE: ACTIVATIONTYPE = 16;
pub const ACTIVATIONTYPE_FROM_MONIKER: ACTIVATIONTYPE = 1;
pub const ACTIVATIONTYPE_FROM_STORAGE: ACTIVATIONTYPE = 4;
pub const ACTIVATIONTYPE_FROM_STREAM: ACTIVATIONTYPE = 8;
pub const ACTIVATIONTYPE_UNCATEGORIZED: ACTIVATIONTYPE = 0;
pub type APARTMENTID = u32;
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
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn Begin_QueryMultipleInterfaces(&self, pmqis: &mut [MULTI_QI]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Begin_QueryMultipleInterfaces)(windows_core::Interface::as_raw(self), pmqis.len().try_into().unwrap(), pmqis.as_mut_ptr()) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn Finish_QueryMultipleInterfaces(&self, pmqis: *mut MULTI_QI) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finish_QueryMultipleInterfaces)(windows_core::Interface::as_raw(self), pmqis) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIMultiQI_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub Begin_QueryMultipleInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut MULTI_QI) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    Begin_QueryMultipleInterfaces: usize,
    #[cfg(feature = "guiddef")]
    pub Finish_QueryMultipleInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MULTI_QI) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    Finish_QueryMultipleInterfaces: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait AsyncIMultiQI_Impl: windows_core::IUnknownImpl {
    fn Begin_QueryMultipleInterfaces(&self, cmqis: super::ULONG, pmqis: *mut MULTI_QI) -> windows_core::Result<()>;
    fn Finish_QueryMultipleInterfaces(&self, pmqis: *mut MULTI_QI) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl AsyncIMultiQI_Vtbl {
    pub const fn new<Identity: AsyncIMultiQI_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_QueryMultipleInterfaces<Identity: AsyncIMultiQI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cmqis: super::ULONG, pmqis: *mut MULTI_QI) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for AsyncIMultiQI {}
windows_core::imp::define_interface!(AsyncIPipeByte, AsyncIPipeByte_Vtbl, 0xdb2f3acb_2f86_11d1_8e04_00c04fb9989a);
windows_core::imp::interface_hierarchy!(AsyncIPipeByte, windows_core::IUnknown);
impl AsyncIPipeByte {
    #[cfg(feature = "shared")]
    pub unsafe fn Begin_Pull(&self, crequest: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Begin_Pull)(windows_core::Interface::as_raw(self), crequest) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Finish_Pull(&self, buf: *mut super::BYTE, pcreturned: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finish_Pull)(windows_core::Interface::as_raw(self), buf as _, pcreturned as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Begin_Push(&self, buf: &[super::BYTE]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Begin_Push)(windows_core::Interface::as_raw(self), buf.as_ptr(), buf.len().try_into().unwrap()) }
    }
    pub unsafe fn Finish_Push(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finish_Push)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeByte_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Begin_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Begin_Pull: usize,
    #[cfg(feature = "shared")]
    pub Finish_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BYTE, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Finish_Pull: usize,
    #[cfg(feature = "shared")]
    pub Begin_Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Begin_Push: usize,
    pub Finish_Push: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait AsyncIPipeByte_Impl: windows_core::IUnknownImpl {
    fn Begin_Pull(&self, crequest: super::ULONG) -> windows_core::Result<()>;
    fn Finish_Pull(&self, buf: *mut super::BYTE, pcreturned: *mut super::ULONG) -> windows_core::Result<()>;
    fn Begin_Push(&self, buf: *const super::BYTE, csent: super::ULONG) -> windows_core::Result<()>;
    fn Finish_Push(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl AsyncIPipeByte_Vtbl {
    pub const fn new<Identity: AsyncIPipeByte_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_Pull<Identity: AsyncIPipeByte_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crequest: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeByte_Impl::Begin_Pull(this, core::mem::transmute_copy(&crequest)).into()
            }
        }
        unsafe extern "system" fn Finish_Pull<Identity: AsyncIPipeByte_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut super::BYTE, pcreturned: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeByte_Impl::Finish_Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&pcreturned)).into()
            }
        }
        unsafe extern "system" fn Begin_Push<Identity: AsyncIPipeByte_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const super::BYTE, csent: super::ULONG) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for AsyncIPipeByte {}
windows_core::imp::define_interface!(AsyncIPipeDouble, AsyncIPipeDouble_Vtbl, 0xdb2f3acf_2f86_11d1_8e04_00c04fb9989a);
windows_core::imp::interface_hierarchy!(AsyncIPipeDouble, windows_core::IUnknown);
impl AsyncIPipeDouble {
    #[cfg(feature = "shared")]
    pub unsafe fn Begin_Pull(&self, crequest: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Begin_Pull)(windows_core::Interface::as_raw(self), crequest) }
    }
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn Finish_Pull(&self, buf: *mut super::DOUBLE, pcreturned: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finish_Pull)(windows_core::Interface::as_raw(self), buf as _, pcreturned as _) }
    }
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn Begin_Push(&self, buf: &[super::DOUBLE]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Begin_Push)(windows_core::Interface::as_raw(self), buf.as_ptr(), buf.len().try_into().unwrap()) }
    }
    pub unsafe fn Finish_Push(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finish_Push)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeDouble_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Begin_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Begin_Pull: usize,
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub Finish_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DOUBLE, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypesbase")))]
    Finish_Pull: usize,
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub Begin_Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::DOUBLE, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypesbase")))]
    Begin_Push: usize,
    pub Finish_Push: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub trait AsyncIPipeDouble_Impl: windows_core::IUnknownImpl {
    fn Begin_Pull(&self, crequest: super::ULONG) -> windows_core::Result<()>;
    fn Finish_Pull(&self, buf: *mut super::DOUBLE, pcreturned: *mut super::ULONG) -> windows_core::Result<()>;
    fn Begin_Push(&self, buf: *const super::DOUBLE, csent: super::ULONG) -> windows_core::Result<()>;
    fn Finish_Push(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
impl AsyncIPipeDouble_Vtbl {
    pub const fn new<Identity: AsyncIPipeDouble_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_Pull<Identity: AsyncIPipeDouble_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crequest: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeDouble_Impl::Begin_Pull(this, core::mem::transmute_copy(&crequest)).into()
            }
        }
        unsafe extern "system" fn Finish_Pull<Identity: AsyncIPipeDouble_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut super::DOUBLE, pcreturned: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeDouble_Impl::Finish_Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&pcreturned)).into()
            }
        }
        unsafe extern "system" fn Begin_Push<Identity: AsyncIPipeDouble_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const super::DOUBLE, csent: super::ULONG) -> windows_core::HRESULT {
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
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
impl windows_core::RuntimeName for AsyncIPipeDouble {}
windows_core::imp::define_interface!(AsyncIPipeLong, AsyncIPipeLong_Vtbl, 0xdb2f3acd_2f86_11d1_8e04_00c04fb9989a);
windows_core::imp::interface_hierarchy!(AsyncIPipeLong, windows_core::IUnknown);
impl AsyncIPipeLong {
    #[cfg(feature = "shared")]
    pub unsafe fn Begin_Pull(&self, crequest: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Begin_Pull)(windows_core::Interface::as_raw(self), crequest) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Finish_Pull(&self, buf: *mut super::LONG, pcreturned: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finish_Pull)(windows_core::Interface::as_raw(self), buf as _, pcreturned as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Begin_Push(&self, buf: &[super::LONG]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Begin_Push)(windows_core::Interface::as_raw(self), buf.as_ptr(), buf.len().try_into().unwrap()) }
    }
    pub unsafe fn Finish_Push(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Finish_Push)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeLong_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Begin_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Begin_Pull: usize,
    #[cfg(feature = "shared")]
    pub Finish_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Finish_Pull: usize,
    #[cfg(feature = "shared")]
    pub Begin_Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::LONG, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Begin_Push: usize,
    pub Finish_Push: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait AsyncIPipeLong_Impl: windows_core::IUnknownImpl {
    fn Begin_Pull(&self, crequest: super::ULONG) -> windows_core::Result<()>;
    fn Finish_Pull(&self, buf: *mut super::LONG, pcreturned: *mut super::ULONG) -> windows_core::Result<()>;
    fn Begin_Push(&self, buf: *const super::LONG, csent: super::ULONG) -> windows_core::Result<()>;
    fn Finish_Push(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl AsyncIPipeLong_Vtbl {
    pub const fn new<Identity: AsyncIPipeLong_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_Pull<Identity: AsyncIPipeLong_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crequest: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeLong_Impl::Begin_Pull(this, core::mem::transmute_copy(&crequest)).into()
            }
        }
        unsafe extern "system" fn Finish_Pull<Identity: AsyncIPipeLong_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut super::LONG, pcreturned: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIPipeLong_Impl::Finish_Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&pcreturned)).into()
            }
        }
        unsafe extern "system" fn Begin_Push<Identity: AsyncIPipeLong_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const super::LONG, csent: super::ULONG) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for AsyncIPipeLong {}
pub const COLE_DEFAULT_AUTHINFO: *mut core::ffi::c_void = core::ptr::without_provenance_mut::<core::ffi::c_void>((-1i32) as usize);
#[cfg(feature = "wtypesbase")]
pub const COLE_DEFAULT_PRINCIPAL: *mut super::OLECHAR = core::ptr::without_provenance_mut::<super::OLECHAR>((-1i32) as usize);
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
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct COSERVERINFO {
    pub dwReserved1: super::DWORD,
    pub pwszName: windows_core::PWSTR,
    pub pAuthInfo: *mut super::COAUTHINFO,
    pub dwReserved2: super::DWORD,
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
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn HandleActivation(&self, dwactivationtype: super::DWORD, rclsid: *const super::IID) -> windows_core::Result<super::CLSID> {
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
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub HandleActivation: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const super::IID, *mut super::CLSID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    HandleActivation: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IActivationFilter_Impl: windows_core::IUnknownImpl {
    fn HandleActivation(&self, dwactivationtype: super::DWORD, rclsid: *const super::IID) -> windows_core::Result<super::CLSID>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IActivationFilter_Vtbl {
    pub const fn new<Identity: IActivationFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandleActivation<Identity: IActivationFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwactivationtype: super::DWORD, rclsid: *const super::IID, preplacementclsid: *mut super::CLSID) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IActivationFilter {}
windows_core::imp::define_interface!(IAddrExclusionControl, IAddrExclusionControl_Vtbl, 0x00000148_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IAddrExclusionControl, windows_core::IUnknown);
impl IAddrExclusionControl {
    #[cfg(feature = "guiddef")]
    pub unsafe fn GetCurrentAddrExclusionList<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetCurrentAddrExclusionList)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
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
    #[cfg(feature = "guiddef")]
    pub GetCurrentAddrExclusionList: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    GetCurrentAddrExclusionList: usize,
    pub UpdateAddrExclusionList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "guiddef")]
pub trait IAddrExclusionControl_Impl: windows_core::IUnknownImpl {
    fn GetCurrentAddrExclusionList(&self, riid: *const super::IID, ppenumerator: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn UpdateAddrExclusionList(&self, penumerator: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "guiddef")]
impl IAddrExclusionControl_Vtbl {
    pub const fn new<Identity: IAddrExclusionControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentAddrExclusionList<Identity: IAddrExclusionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, ppenumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(feature = "guiddef")]
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
    #[cfg(feature = "guiddef")]
    pub unsafe fn Resolve<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).Resolve)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgileReference_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "guiddef")]
    pub Resolve: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    Resolve: usize,
}
#[cfg(feature = "guiddef")]
pub trait IAgileReference_Impl: windows_core::IUnknownImpl {
    fn Resolve(&self, riid: *const super::IID, ppvobjectreference: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "guiddef")]
impl IAgileReference_Vtbl {
    pub const fn new<Identity: IAgileReference_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Resolve<Identity: IAgileReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, ppvobjectreference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(feature = "guiddef")]
impl windows_core::RuntimeName for IAgileReference {}
windows_core::imp::define_interface!(IAsyncManager, IAsyncManager_Vtbl, 0x0000002a_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IAsyncManager, windows_core::IUnknown);
impl IAsyncManager {
    pub unsafe fn CompleteCall(&self, result: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CompleteCall)(windows_core::Interface::as_raw(self), result) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn GetCallContext(&self, riid: *const super::IID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCallContext)(windows_core::Interface::as_raw(self), riid, pinterface as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetState(&self) -> windows_core::Result<super::ULONG> {
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
    #[cfg(feature = "guiddef")]
    pub GetCallContext: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    GetCallContext: usize,
    #[cfg(feature = "shared")]
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetState: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IAsyncManager_Impl: windows_core::IUnknownImpl {
    fn CompleteCall(&self, result: windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetCallContext(&self, riid: *const super::IID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetState(&self) -> windows_core::Result<super::ULONG>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IAsyncManager_Vtbl {
    pub const fn new<Identity: IAsyncManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CompleteCall<Identity: IAsyncManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncManager_Impl::CompleteCall(this, core::mem::transmute_copy(&result)).into()
            }
        }
        unsafe extern "system" fn GetCallContext<Identity: IAsyncManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncManager_Impl::GetCallContext(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pinterface)).into()
            }
        }
        unsafe extern "system" fn GetState<Identity: IAsyncManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulstateflags: *mut super::ULONG) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "shared"))]
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
    #[cfg(feature = "shared")]
    pub unsafe fn Send<P1>(&self, pmsg: *mut RPCOLEMESSAGE, psync: P1, pulstatus: *mut super::ULONG) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ISynchronize>,
    {
        unsafe { (windows_core::Interface::vtable(self).Send)(windows_core::Interface::as_raw(self), pmsg as _, psync.param().abi(), pulstatus as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Receive)(windows_core::Interface::as_raw(self), pmsg as _, pulstatus as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut super::DWORD, ppvdestcontext: Option<*mut *mut core::ffi::c_void>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDestCtxEx)(windows_core::Interface::as_raw(self), pmsg, pdwdestcontext as _, ppvdestcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncRpcChannelBuffer_Vtbl {
    pub base__: IRpcChannelBuffer2_Vtbl,
    #[cfg(feature = "shared")]
    pub Send: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Send: usize,
    #[cfg(feature = "shared")]
    pub Receive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Receive: usize,
    #[cfg(feature = "shared")]
    pub GetDestCtxEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const RPCOLEMESSAGE, *mut super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDestCtxEx: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IAsyncRpcChannelBuffer_Impl: IRpcChannelBuffer2_Impl {
    fn Send(&self, pmsg: *mut RPCOLEMESSAGE, psync: windows_core::Ref<ISynchronize>, pulstatus: *mut super::ULONG) -> windows_core::Result<()>;
    fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut super::ULONG) -> windows_core::Result<()>;
    fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut super::DWORD, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IAsyncRpcChannelBuffer_Vtbl {
    pub const fn new<Identity: IAsyncRpcChannelBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Send<Identity: IAsyncRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, psync: *mut core::ffi::c_void, pulstatus: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncRpcChannelBuffer_Impl::Send(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&psync), core::mem::transmute_copy(&pulstatus)).into()
            }
        }
        unsafe extern "system" fn Receive<Identity: IAsyncRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncRpcChannelBuffer_Impl::Receive(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&pulstatus)).into()
            }
        }
        unsafe extern "system" fn GetDestCtxEx<Identity: IAsyncRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut super::DWORD, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IAsyncRpcChannelBuffer {}
windows_core::imp::define_interface!(ICallFactory, ICallFactory_Vtbl, 0x1c733a30_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ICallFactory, windows_core::IUnknown);
impl ICallFactory {
    #[cfg(feature = "guiddef")]
    pub unsafe fn CreateCall<P1>(&self, riid: *const super::IID, pctrlunk: P1, riid2: *const super::IID) -> windows_core::Result<windows_core::IUnknown>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCall)(windows_core::Interface::as_raw(self), riid, pctrlunk.param().abi(), riid2, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "guiddef")]
    pub CreateCall: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut core::ffi::c_void, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    CreateCall: usize,
}
#[cfg(feature = "guiddef")]
pub trait ICallFactory_Impl: windows_core::IUnknownImpl {
    fn CreateCall(&self, riid: *const super::IID, pctrlunk: windows_core::Ref<windows_core::IUnknown>, riid2: *const super::IID) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "guiddef")]
impl ICallFactory_Vtbl {
    pub const fn new<Identity: ICallFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateCall<Identity: ICallFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, pctrlunk: *mut core::ffi::c_void, riid2: *const super::IID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICallFactory_Impl::CreateCall(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pctrlunk), core::mem::transmute_copy(&riid2)) {
                    Ok(ok__) => {
                        ppv.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateCall: CreateCall::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICallFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "guiddef")]
impl windows_core::RuntimeName for ICallFactory {}
windows_core::imp::define_interface!(ICancelMethodCalls, ICancelMethodCalls_Vtbl, 0x00000029_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ICancelMethodCalls, windows_core::IUnknown);
impl ICancelMethodCalls {
    #[cfg(feature = "shared")]
    pub unsafe fn Cancel(&self, ulseconds: super::ULONG) -> windows_core::HRESULT {
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
    #[cfg(feature = "shared")]
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Cancel: usize,
    pub TestCancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait ICancelMethodCalls_Impl: windows_core::IUnknownImpl {
    fn Cancel(&self, ulseconds: super::ULONG) -> windows_core::Result<()>;
    fn TestCancel(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl ICancelMethodCalls_Vtbl {
    pub const fn new<Identity: ICancelMethodCalls_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cancel<Identity: ICancelMethodCalls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulseconds: super::ULONG) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for ICancelMethodCalls {}
windows_core::imp::define_interface!(IChannelHook, IChannelHook_Vtbl, 0x1008c4a0_7613_11cf_9af1_0020af6e72f4);
windows_core::imp::interface_hierarchy!(IChannelHook, windows_core::IUnknown);
impl IChannelHook {
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn ClientGetSize(&self, uextent: *const windows_core::GUID, riid: *const super::IID) -> super::ULONG {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientGetSize)(windows_core::Interface::as_raw(self), uextent, riid, &mut result__);
            result__
        }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn ClientFillBuffer(&self, uextent: *const windows_core::GUID, riid: *const super::IID, pdatasize: *mut super::ULONG, pdatabuffer: *const core::ffi::c_void) {
        unsafe {
            (windows_core::Interface::vtable(self).ClientFillBuffer)(windows_core::Interface::as_raw(self), uextent, riid, pdatasize as _, pdatabuffer);
        }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn ClientNotify(&self, uextent: *const windows_core::GUID, riid: *const super::IID, cbdatasize: super::ULONG, pdatabuffer: *const core::ffi::c_void, ldatarep: super::DWORD, hrfault: windows_core::HRESULT) {
        unsafe {
            (windows_core::Interface::vtable(self).ClientNotify)(windows_core::Interface::as_raw(self), uextent, riid, cbdatasize, pdatabuffer, ldatarep, hrfault);
        }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn ServerNotify(&self, uextent: *const windows_core::GUID, riid: *const super::IID, cbdatasize: super::ULONG, pdatabuffer: *const core::ffi::c_void, ldatarep: super::DWORD) {
        unsafe {
            (windows_core::Interface::vtable(self).ServerNotify)(windows_core::Interface::as_raw(self), uextent, riid, cbdatasize, pdatabuffer, ldatarep);
        }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn ServerGetSize(&self, uextent: *const windows_core::GUID, riid: *const super::IID, hrfault: windows_core::HRESULT) -> super::ULONG {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServerGetSize)(windows_core::Interface::as_raw(self), uextent, riid, hrfault, &mut result__);
            result__
        }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn ServerFillBuffer(&self, uextent: *const windows_core::GUID, riid: *const super::IID, pdatasize: *mut super::ULONG, pdatabuffer: *const core::ffi::c_void, hrfault: windows_core::HRESULT) {
        unsafe {
            (windows_core::Interface::vtable(self).ServerFillBuffer)(windows_core::Interface::as_raw(self), uextent, riid, pdatasize as _, pdatabuffer, hrfault);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelHook_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub ClientGetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::IID, *mut super::ULONG),
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    ClientGetSize: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub ClientFillBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::IID, *mut super::ULONG, *const core::ffi::c_void),
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    ClientFillBuffer: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub ClientNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::IID, super::ULONG, *const core::ffi::c_void, super::DWORD, windows_core::HRESULT),
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    ClientNotify: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub ServerNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::IID, super::ULONG, *const core::ffi::c_void, super::DWORD),
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    ServerNotify: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub ServerGetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::IID, windows_core::HRESULT, *mut super::ULONG),
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    ServerGetSize: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub ServerFillBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::IID, *mut super::ULONG, *const core::ffi::c_void, windows_core::HRESULT),
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    ServerFillBuffer: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IChannelHook_Impl: windows_core::IUnknownImpl {
    fn ClientGetSize(&self, uextent: *const windows_core::GUID, riid: *const super::IID, pdatasize: *mut super::ULONG);
    fn ClientFillBuffer(&self, uextent: *const windows_core::GUID, riid: *const super::IID, pdatasize: *mut super::ULONG, pdatabuffer: *const core::ffi::c_void);
    fn ClientNotify(&self, uextent: *const windows_core::GUID, riid: *const super::IID, cbdatasize: super::ULONG, pdatabuffer: *const core::ffi::c_void, ldatarep: super::DWORD, hrfault: windows_core::HRESULT);
    fn ServerNotify(&self, uextent: *const windows_core::GUID, riid: *const super::IID, cbdatasize: super::ULONG, pdatabuffer: *const core::ffi::c_void, ldatarep: super::DWORD);
    fn ServerGetSize(&self, uextent: *const windows_core::GUID, riid: *const super::IID, hrfault: windows_core::HRESULT, pdatasize: *mut super::ULONG);
    fn ServerFillBuffer(&self, uextent: *const windows_core::GUID, riid: *const super::IID, pdatasize: *mut super::ULONG, pdatabuffer: *const core::ffi::c_void, hrfault: windows_core::HRESULT);
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IChannelHook_Vtbl {
    pub const fn new<Identity: IChannelHook_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ClientGetSize<Identity: IChannelHook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const super::IID, pdatasize: *mut super::ULONG) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelHook_Impl::ClientGetSize(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pdatasize));
            }
        }
        unsafe extern "system" fn ClientFillBuffer<Identity: IChannelHook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const super::IID, pdatasize: *mut super::ULONG, pdatabuffer: *const core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelHook_Impl::ClientFillBuffer(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdatabuffer));
            }
        }
        unsafe extern "system" fn ClientNotify<Identity: IChannelHook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const super::IID, cbdatasize: super::ULONG, pdatabuffer: *const core::ffi::c_void, ldatarep: super::DWORD, hrfault: windows_core::HRESULT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelHook_Impl::ClientNotify(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cbdatasize), core::mem::transmute_copy(&pdatabuffer), core::mem::transmute_copy(&ldatarep), core::mem::transmute_copy(&hrfault));
            }
        }
        unsafe extern "system" fn ServerNotify<Identity: IChannelHook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const super::IID, cbdatasize: super::ULONG, pdatabuffer: *const core::ffi::c_void, ldatarep: super::DWORD) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelHook_Impl::ServerNotify(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cbdatasize), core::mem::transmute_copy(&pdatabuffer), core::mem::transmute_copy(&ldatarep));
            }
        }
        unsafe extern "system" fn ServerGetSize<Identity: IChannelHook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const super::IID, hrfault: windows_core::HRESULT, pdatasize: *mut super::ULONG) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelHook_Impl::ServerGetSize(this, core::mem::transmute_copy(&uextent), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&hrfault), core::mem::transmute_copy(&pdatasize));
            }
        }
        unsafe extern "system" fn ServerFillBuffer<Identity: IChannelHook_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uextent: *const windows_core::GUID, riid: *const super::IID, pdatasize: *mut super::ULONG, pdatabuffer: *const core::ffi::c_void, hrfault: windows_core::HRESULT) {
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
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IChannelHook {}
windows_core::imp::define_interface!(IClientSecurity, IClientSecurity_Vtbl, 0x0000013d_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IClientSecurity, windows_core::IUnknown);
impl IClientSecurity {
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn QueryBlanket<P0>(&self, pproxy: P0, pauthnsvc: *mut super::DWORD, pauthzsvc: Option<*mut super::DWORD>, pserverprincname: *mut *mut super::OLECHAR, pauthnlevel: Option<*mut super::DWORD>, pimplevel: Option<*mut super::DWORD>, pauthinfo: *mut *mut core::ffi::c_void, pcapabilites: Option<*mut super::DWORD>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueryBlanket)(windows_core::Interface::as_raw(self), pproxy.param().abi(), pauthnsvc as _, pauthzsvc.unwrap_or(core::mem::zeroed()) as _, pserverprincname as _, pauthnlevel.unwrap_or(core::mem::zeroed()) as _, pimplevel.unwrap_or(core::mem::zeroed()) as _, pauthinfo as _, pcapabilites.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn SetBlanket<P0>(&self, pproxy: P0, dwauthnsvc: super::DWORD, dwauthzsvc: super::DWORD, pserverprincname: Option<*const super::OLECHAR>, dwauthnlevel: super::DWORD, dwimplevel: super::DWORD, pauthinfo: Option<*const core::ffi::c_void>, dwcapabilities: super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBlanket)(windows_core::Interface::as_raw(self), pproxy.param().abi(), dwauthnsvc, dwauthzsvc, pserverprincname.unwrap_or(core::mem::zeroed()) as _, dwauthnlevel, dwimplevel, pauthinfo.unwrap_or(core::mem::zeroed()) as _, dwcapabilities) }
    }
    pub unsafe fn CopyProxy<P0>(&self, pproxy: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyProxy)(windows_core::Interface::as_raw(self), pproxy.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClientSecurity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub QueryBlanket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::DWORD, *mut super::DWORD, *mut *mut super::OLECHAR, *mut super::DWORD, *mut super::DWORD, *mut *mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypesbase")))]
    QueryBlanket: usize,
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub SetBlanket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::DWORD, super::DWORD, *const super::OLECHAR, super::DWORD, super::DWORD, *const core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypesbase")))]
    SetBlanket: usize,
    pub CopyProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub trait IClientSecurity_Impl: windows_core::IUnknownImpl {
    fn QueryBlanket(&self, pproxy: windows_core::Ref<windows_core::IUnknown>, pauthnsvc: *mut super::DWORD, pauthzsvc: *mut super::DWORD, pserverprincname: *mut *mut super::OLECHAR, pauthnlevel: *mut super::DWORD, pimplevel: *mut super::DWORD, pauthinfo: *mut *mut core::ffi::c_void, pcapabilites: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetBlanket(&self, pproxy: windows_core::Ref<windows_core::IUnknown>, dwauthnsvc: super::DWORD, dwauthzsvc: super::DWORD, pserverprincname: *const super::OLECHAR, dwauthnlevel: super::DWORD, dwimplevel: super::DWORD, pauthinfo: *const core::ffi::c_void, dwcapabilities: super::DWORD) -> windows_core::Result<()>;
    fn CopyProxy(&self, pproxy: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
impl IClientSecurity_Vtbl {
    pub const fn new<Identity: IClientSecurity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryBlanket<Identity: IClientSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproxy: *mut core::ffi::c_void, pauthnsvc: *mut super::DWORD, pauthzsvc: *mut super::DWORD, pserverprincname: *mut *mut super::OLECHAR, pauthnlevel: *mut super::DWORD, pimplevel: *mut super::DWORD, pauthinfo: *mut *mut core::ffi::c_void, pcapabilites: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClientSecurity_Impl::QueryBlanket(this, core::mem::transmute_copy(&pproxy), core::mem::transmute_copy(&pauthnsvc), core::mem::transmute_copy(&pauthzsvc), core::mem::transmute_copy(&pserverprincname), core::mem::transmute_copy(&pauthnlevel), core::mem::transmute_copy(&pimplevel), core::mem::transmute_copy(&pauthinfo), core::mem::transmute_copy(&pcapabilites)).into()
            }
        }
        unsafe extern "system" fn SetBlanket<Identity: IClientSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproxy: *mut core::ffi::c_void, dwauthnsvc: super::DWORD, dwauthzsvc: super::DWORD, pserverprincname: *const super::OLECHAR, dwauthnlevel: super::DWORD, dwimplevel: super::DWORD, pauthinfo: *const core::ffi::c_void, dwcapabilities: super::DWORD) -> windows_core::HRESULT {
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
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IContext(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IEnumContextProps(pub u8);
windows_core::imp::define_interface!(IEnumString, IEnumString_Vtbl, 0x00000101_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumString, windows_core::IUnknown);
impl IEnumString {
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn Next(&self, celt: super::ULONG, rgelt: *mut super::LPOLESTR, pceltfetched: Option<*mut super::ULONG>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt as _, pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Skip(&self, celt: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumString_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut super::LPOLESTR, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypesbase")))]
    Next: usize,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub trait IEnumString_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: super::ULONG, rgelt: *mut super::LPOLESTR, pceltfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Skip(&self, celt: super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumString>;
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
impl IEnumString_Vtbl {
    pub const fn new<Identity: IEnumString_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG, rgelt: *mut super::LPOLESTR, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumString_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG) -> windows_core::HRESULT {
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
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IEnumString {}
windows_core::imp::define_interface!(IEnumUnknown, IEnumUnknown_Vtbl, 0x00000100_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumUnknown, windows_core::IUnknown);
impl IEnumUnknown {
    #[cfg(feature = "shared")]
    pub unsafe fn Next(&self, celt: super::ULONG, rgelt: *mut Option<windows_core::IUnknown>, pceltfetched: Option<*mut super::ULONG>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgelt), pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Skip(&self, celt: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumUnknown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Next: usize,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IEnumUnknown_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: super::ULONG, rgelt: *mut Option<windows_core::IUnknown>, pceltfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Skip(&self, celt: super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumUnknown>;
}
#[cfg(feature = "shared")]
impl IEnumUnknown_Vtbl {
    pub const fn new<Identity: IEnumUnknown_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumUnknown_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IEnumUnknown {}
windows_core::imp::define_interface!(IExternalConnection, IExternalConnection_Vtbl, 0x00000019_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IExternalConnection, windows_core::IUnknown);
impl IExternalConnection {
    #[cfg(feature = "shared")]
    pub unsafe fn AddConnection(&self, extconn: super::DWORD, reserved: super::DWORD) -> super::DWORD {
        unsafe { (windows_core::Interface::vtable(self).AddConnection)(windows_core::Interface::as_raw(self), extconn, reserved) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ReleaseConnection(&self, extconn: super::DWORD, reserved: super::DWORD, flastreleasecloses: bool) -> super::DWORD {
        unsafe { (windows_core::Interface::vtable(self).ReleaseConnection)(windows_core::Interface::as_raw(self), extconn, reserved, flastreleasecloses.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExternalConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub AddConnection: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD) -> super::DWORD,
    #[cfg(not(feature = "shared"))]
    AddConnection: usize,
    #[cfg(feature = "shared")]
    pub ReleaseConnection: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, windows_core::BOOL) -> super::DWORD,
    #[cfg(not(feature = "shared"))]
    ReleaseConnection: usize,
}
#[cfg(feature = "shared")]
pub trait IExternalConnection_Impl: windows_core::IUnknownImpl {
    fn AddConnection(&self, extconn: super::DWORD, reserved: super::DWORD) -> super::DWORD;
    fn ReleaseConnection(&self, extconn: super::DWORD, reserved: super::DWORD, flastreleasecloses: windows_core::BOOL) -> super::DWORD;
}
#[cfg(feature = "shared")]
impl IExternalConnection_Vtbl {
    pub const fn new<Identity: IExternalConnection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddConnection<Identity: IExternalConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, extconn: super::DWORD, reserved: super::DWORD) -> super::DWORD {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExternalConnection_Impl::AddConnection(this, core::mem::transmute_copy(&extconn), core::mem::transmute_copy(&reserved))
            }
        }
        unsafe extern "system" fn ReleaseConnection<Identity: IExternalConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, extconn: super::DWORD, reserved: super::DWORD, flastreleasecloses: windows_core::BOOL) -> super::DWORD {
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
#[cfg(feature = "shared")]
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
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn RegisterInterfaceInGlobal<P0>(&self, punk: P0, riid: *const super::IID) -> windows_core::Result<super::DWORD>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterInterfaceInGlobal)(windows_core::Interface::as_raw(self), punk.param().abi(), riid, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RevokeInterfaceFromGlobal(&self, dwcookie: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RevokeInterfaceFromGlobal)(windows_core::Interface::as_raw(self), dwcookie) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn GetInterfaceFromGlobal<T>(&self, dwcookie: super::DWORD) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetInterfaceFromGlobal)(windows_core::Interface::as_raw(self), dwcookie, &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalInterfaceTable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub RegisterInterfaceInGlobal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::IID, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    RegisterInterfaceInGlobal: usize,
    #[cfg(feature = "shared")]
    pub RevokeInterfaceFromGlobal: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RevokeInterfaceFromGlobal: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub GetInterfaceFromGlobal: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    GetInterfaceFromGlobal: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IGlobalInterfaceTable_Impl: windows_core::IUnknownImpl {
    fn RegisterInterfaceInGlobal(&self, punk: windows_core::Ref<windows_core::IUnknown>, riid: *const super::IID) -> windows_core::Result<super::DWORD>;
    fn RevokeInterfaceFromGlobal(&self, dwcookie: super::DWORD) -> windows_core::Result<()>;
    fn GetInterfaceFromGlobal(&self, dwcookie: super::DWORD, riid: *const super::IID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IGlobalInterfaceTable_Vtbl {
    pub const fn new<Identity: IGlobalInterfaceTable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterInterfaceInGlobal<Identity: IGlobalInterfaceTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, riid: *const super::IID, pdwcookie: *mut super::DWORD) -> windows_core::HRESULT {
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
        unsafe extern "system" fn RevokeInterfaceFromGlobal<Identity: IGlobalInterfaceTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGlobalInterfaceTable_Impl::RevokeInterfaceFromGlobal(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        unsafe extern "system" fn GetInterfaceFromGlobal<Identity: IGlobalInterfaceTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: super::DWORD, riid: *const super::IID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IGlobalInterfaceTable {}
windows_core::imp::define_interface!(IGlobalOptions, IGlobalOptions_Vtbl, 0x0000015b_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IGlobalOptions, windows_core::IUnknown);
impl IGlobalOptions {
    #[cfg(feature = "shared")]
    pub unsafe fn Set(&self, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: super::ULONG_PTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), dwproperty, dwvalue) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Query(&self, dwproperty: GLOBALOPT_PROPERTIES) -> windows_core::Result<super::ULONG_PTR> {
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
    #[cfg(feature = "shared")]
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, GLOBALOPT_PROPERTIES, super::ULONG_PTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Set: usize,
    #[cfg(feature = "shared")]
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, GLOBALOPT_PROPERTIES, *mut super::ULONG_PTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Query: usize,
}
#[cfg(feature = "shared")]
pub trait IGlobalOptions_Impl: windows_core::IUnknownImpl {
    fn Set(&self, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: super::ULONG_PTR) -> windows_core::Result<()>;
    fn Query(&self, dwproperty: GLOBALOPT_PROPERTIES) -> windows_core::Result<super::ULONG_PTR>;
}
#[cfg(feature = "shared")]
impl IGlobalOptions_Vtbl {
    pub const fn new<Identity: IGlobalOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Set<Identity: IGlobalOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: super::ULONG_PTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGlobalOptions_Impl::Set(this, core::mem::transmute_copy(&dwproperty), core::mem::transmute_copy(&dwvalue)).into()
            }
        }
        unsafe extern "system" fn Query<Identity: IGlobalOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, pdwvalue: *mut super::ULONG_PTR) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IGlobalOptions {}
windows_core::imp::define_interface!(IInternalUnknown, IInternalUnknown_Vtbl, 0x00000021_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IInternalUnknown, windows_core::IUnknown);
impl IInternalUnknown {
    #[cfg(feature = "guiddef")]
    pub unsafe fn QueryInternalInterface(&self, riid: *const super::IID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryInternalInterface)(windows_core::Interface::as_raw(self), riid, ppv as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternalUnknown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "guiddef")]
    pub QueryInternalInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    QueryInternalInterface: usize,
}
#[cfg(feature = "guiddef")]
pub trait IInternalUnknown_Impl: windows_core::IUnknownImpl {
    fn QueryInternalInterface(&self, riid: *const super::IID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "guiddef")]
impl IInternalUnknown_Vtbl {
    pub const fn new<Identity: IInternalUnknown_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInternalInterface<Identity: IInternalUnknown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(feature = "guiddef")]
impl windows_core::RuntimeName for IInternalUnknown {}
windows_core::imp::define_interface!(IMachineGlobalObjectTable, IMachineGlobalObjectTable_Vtbl, 0x26d709ac_f70b_4421_a96f_d2878fafb00d);
windows_core::imp::interface_hierarchy!(IMachineGlobalObjectTable, windows_core::IUnknown);
impl IMachineGlobalObjectTable {
    #[cfg(feature = "guiddef")]
    pub unsafe fn RegisterObject<P1, P2>(&self, clsid: *const super::IID, identifier: P1, object: P2) -> windows_core::Result<MachineGlobalObjectTableRegistrationToken>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterObject)(windows_core::Interface::as_raw(self), clsid, identifier.param().abi(), object.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn GetObject<P1, T>(&self, clsid: *const super::IID, identifier: P1) -> windows_core::Result<T>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), clsid, identifier.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
    pub unsafe fn RevokeObject(&self, token: MachineGlobalObjectTableRegistrationToken) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RevokeObject)(windows_core::Interface::as_raw(self), token) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMachineGlobalObjectTable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "guiddef")]
    pub RegisterObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, windows_core::PCWSTR, *mut core::ffi::c_void, *mut MachineGlobalObjectTableRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    RegisterObject: usize,
    #[cfg(feature = "guiddef")]
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, windows_core::PCWSTR, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    GetObject: usize,
    pub RevokeObject: unsafe extern "system" fn(*mut core::ffi::c_void, MachineGlobalObjectTableRegistrationToken) -> windows_core::HRESULT,
}
#[cfg(feature = "guiddef")]
pub trait IMachineGlobalObjectTable_Impl: windows_core::IUnknownImpl {
    fn RegisterObject(&self, clsid: *const super::IID, identifier: &windows_core::PCWSTR, object: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<MachineGlobalObjectTableRegistrationToken>;
    fn GetObject(&self, clsid: *const super::IID, identifier: &windows_core::PCWSTR, riid: *const super::IID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RevokeObject(&self, token: MachineGlobalObjectTableRegistrationToken) -> windows_core::Result<()>;
}
#[cfg(feature = "guiddef")]
impl IMachineGlobalObjectTable_Vtbl {
    pub const fn new<Identity: IMachineGlobalObjectTable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterObject<Identity: IMachineGlobalObjectTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const super::IID, identifier: windows_core::PCWSTR, object: *mut core::ffi::c_void, token: *mut MachineGlobalObjectTableRegistrationToken) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetObject<Identity: IMachineGlobalObjectTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const super::IID, identifier: windows_core::PCWSTR, riid: *const super::IID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(feature = "guiddef")]
impl windows_core::RuntimeName for IMachineGlobalObjectTable {}
windows_core::imp::define_interface!(IMalloc, IMalloc_Vtbl, 0x00000002_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IMalloc, windows_core::IUnknown);
impl IMalloc {
    #[cfg(feature = "shared")]
    pub unsafe fn Alloc(&self, cb: super::SIZE_T) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).Alloc)(windows_core::Interface::as_raw(self), cb) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Realloc(&self, pv: Option<*const core::ffi::c_void>, cb: super::SIZE_T) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).Realloc)(windows_core::Interface::as_raw(self), pv.unwrap_or(core::mem::zeroed()) as _, cb) }
    }
    pub unsafe fn Free(&self, pv: Option<*const core::ffi::c_void>) {
        unsafe {
            (windows_core::Interface::vtable(self).Free)(windows_core::Interface::as_raw(self), pv.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSize(&self, pv: Option<*const core::ffi::c_void>) -> super::SIZE_T {
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
    #[cfg(feature = "shared")]
    pub Alloc: unsafe extern "system" fn(*mut core::ffi::c_void, super::SIZE_T) -> *mut core::ffi::c_void,
    #[cfg(not(feature = "shared"))]
    Alloc: usize,
    #[cfg(feature = "shared")]
    pub Realloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, super::SIZE_T) -> *mut core::ffi::c_void,
    #[cfg(not(feature = "shared"))]
    Realloc: usize,
    pub Free: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void),
    #[cfg(feature = "shared")]
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> super::SIZE_T,
    #[cfg(not(feature = "shared"))]
    GetSize: usize,
    pub DidAlloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> i32,
    pub HeapMinimize: unsafe extern "system" fn(*mut core::ffi::c_void),
}
#[cfg(feature = "shared")]
pub trait IMalloc_Impl: windows_core::IUnknownImpl {
    fn Alloc(&self, cb: super::SIZE_T) -> *mut core::ffi::c_void;
    fn Realloc(&self, pv: *const core::ffi::c_void, cb: super::SIZE_T) -> *mut core::ffi::c_void;
    fn Free(&self, pv: *const core::ffi::c_void);
    fn GetSize(&self, pv: *const core::ffi::c_void) -> super::SIZE_T;
    fn DidAlloc(&self, pv: *const core::ffi::c_void) -> i32;
    fn HeapMinimize(&self);
}
#[cfg(feature = "shared")]
impl IMalloc_Vtbl {
    pub const fn new<Identity: IMalloc_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Alloc<Identity: IMalloc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cb: super::SIZE_T) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMalloc_Impl::Alloc(this, core::mem::transmute_copy(&cb))
            }
        }
        unsafe extern "system" fn Realloc<Identity: IMalloc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void, cb: super::SIZE_T) -> *mut core::ffi::c_void {
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
        unsafe extern "system" fn GetSize<Identity: IMalloc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void) -> super::SIZE_T {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IMalloc {}
windows_core::imp::define_interface!(IMarshal, IMarshal_Vtbl, 0x00000003_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IMarshal, windows_core::IUnknown);
impl IMarshal {
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn GetUnmarshalClass(&self, riid: *const super::IID, pv: Option<*const core::ffi::c_void>, dwdestcontext: super::DWORD, pvdestcontext: Option<*const core::ffi::c_void>, mshlflags: super::DWORD) -> windows_core::Result<super::CLSID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUnmarshalClass)(windows_core::Interface::as_raw(self), riid, pv.unwrap_or(core::mem::zeroed()) as _, dwdestcontext, pvdestcontext.unwrap_or(core::mem::zeroed()) as _, mshlflags, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn GetMarshalSizeMax(&self, riid: *const super::IID, pv: Option<*const core::ffi::c_void>, dwdestcontext: super::DWORD, pvdestcontext: Option<*const core::ffi::c_void>, mshlflags: super::DWORD) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMarshalSizeMax)(windows_core::Interface::as_raw(self), riid, pv.unwrap_or(core::mem::zeroed()) as _, dwdestcontext, pvdestcontext.unwrap_or(core::mem::zeroed()) as _, mshlflags, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn MarshalInterface<P0>(&self, pstm: P0, riid: *const super::IID, pv: Option<*const core::ffi::c_void>, dwdestcontext: super::DWORD, pvdestcontext: Option<*const core::ffi::c_void>, mshlflags: super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).MarshalInterface)(windows_core::Interface::as_raw(self), pstm.param().abi(), riid, pv.unwrap_or(core::mem::zeroed()) as _, dwdestcontext, pvdestcontext.unwrap_or(core::mem::zeroed()) as _, mshlflags) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn UnmarshalInterface<P0>(&self, pstm: P0, riid: *const super::IID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
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
    #[cfg(feature = "shared")]
    pub unsafe fn DisconnectObject(&self, dwreserved: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DisconnectObject)(windows_core::Interface::as_raw(self), dwreserved) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarshal_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub GetUnmarshalClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *const core::ffi::c_void, super::DWORD, *const core::ffi::c_void, super::DWORD, *mut super::CLSID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    GetUnmarshalClass: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub GetMarshalSizeMax: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *const core::ffi::c_void, super::DWORD, *const core::ffi::c_void, super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    GetMarshalSizeMax: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub MarshalInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::IID, *const core::ffi::c_void, super::DWORD, *const core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    MarshalInterface: usize,
    #[cfg(feature = "guiddef")]
    pub UnmarshalInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    UnmarshalInterface: usize,
    pub ReleaseMarshalData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub DisconnectObject: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DisconnectObject: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IMarshal_Impl: windows_core::IUnknownImpl {
    fn GetUnmarshalClass(&self, riid: *const super::IID, pv: *const core::ffi::c_void, dwdestcontext: super::DWORD, pvdestcontext: *const core::ffi::c_void, mshlflags: super::DWORD) -> windows_core::Result<super::CLSID>;
    fn GetMarshalSizeMax(&self, riid: *const super::IID, pv: *const core::ffi::c_void, dwdestcontext: super::DWORD, pvdestcontext: *const core::ffi::c_void, mshlflags: super::DWORD) -> windows_core::Result<super::DWORD>;
    fn MarshalInterface(&self, pstm: windows_core::Ref<IStream>, riid: *const super::IID, pv: *const core::ffi::c_void, dwdestcontext: super::DWORD, pvdestcontext: *const core::ffi::c_void, mshlflags: super::DWORD) -> windows_core::Result<()>;
    fn UnmarshalInterface(&self, pstm: windows_core::Ref<IStream>, riid: *const super::IID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ReleaseMarshalData(&self, pstm: windows_core::Ref<IStream>) -> windows_core::Result<()>;
    fn DisconnectObject(&self, dwreserved: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IMarshal_Vtbl {
    pub const fn new<Identity: IMarshal_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetUnmarshalClass<Identity: IMarshal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, pv: *const core::ffi::c_void, dwdestcontext: super::DWORD, pvdestcontext: *const core::ffi::c_void, mshlflags: super::DWORD, pcid: *mut super::CLSID) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetMarshalSizeMax<Identity: IMarshal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, pv: *const core::ffi::c_void, dwdestcontext: super::DWORD, pvdestcontext: *const core::ffi::c_void, mshlflags: super::DWORD, psize: *mut super::DWORD) -> windows_core::HRESULT {
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
        unsafe extern "system" fn MarshalInterface<Identity: IMarshal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void, riid: *const super::IID, pv: *const core::ffi::c_void, dwdestcontext: super::DWORD, pvdestcontext: *const core::ffi::c_void, mshlflags: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMarshal_Impl::MarshalInterface(this, core::mem::transmute_copy(&pstm), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pv), core::mem::transmute_copy(&dwdestcontext), core::mem::transmute_copy(&pvdestcontext), core::mem::transmute_copy(&mshlflags)).into()
            }
        }
        unsafe extern "system" fn UnmarshalInterface<Identity: IMarshal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void, riid: *const super::IID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn DisconnectObject<Identity: IMarshal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: super::DWORD) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "shared"))]
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
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IMarshal2_Impl: IMarshal_Impl {}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IMarshal2_Vtbl {
    pub const fn new<Identity: IMarshal2_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IMarshal_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMarshal2 as windows_core::Interface>::IID || iid == &<IMarshal as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
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
    #[cfg(feature = "shared")]
    pub unsafe fn GetMarshalingContextAttribute(&self, attribute: CO_MARSHALING_CONTEXT_ATTRIBUTES) -> windows_core::Result<super::ULONG_PTR> {
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
    #[cfg(feature = "shared")]
    pub GetMarshalingContextAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, CO_MARSHALING_CONTEXT_ATTRIBUTES, *mut super::ULONG_PTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMarshalingContextAttribute: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
pub trait IMarshalingStream_Impl: IStream_Impl {
    fn GetMarshalingContextAttribute(&self, attribute: CO_MARSHALING_CONTEXT_ATTRIBUTES) -> windows_core::Result<super::ULONG_PTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
impl IMarshalingStream_Vtbl {
    pub const fn new<Identity: IMarshalingStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMarshalingContextAttribute<Identity: IMarshalingStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attribute: CO_MARSHALING_CONTEXT_ATTRIBUTES, pattributevalue: *mut super::ULONG_PTR) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMarshalingStream {}
windows_core::imp::define_interface!(IMultiQI, IMultiQI_Vtbl, 0x00000020_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IMultiQI, windows_core::IUnknown);
impl IMultiQI {
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn QueryMultipleInterfaces(&self, pmqis: &mut [MULTI_QI]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryMultipleInterfaces)(windows_core::Interface::as_raw(self), pmqis.len().try_into().unwrap(), pmqis.as_mut_ptr()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiQI_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub QueryMultipleInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut MULTI_QI) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    QueryMultipleInterfaces: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IMultiQI_Impl: windows_core::IUnknownImpl {
    fn QueryMultipleInterfaces(&self, cmqis: super::ULONG, pmqis: *mut MULTI_QI) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IMultiQI_Vtbl {
    pub const fn new<Identity: IMultiQI_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryMultipleInterfaces<Identity: IMultiQI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cmqis: super::ULONG, pmqis: *mut MULTI_QI) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "shared"))]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IObjContext(pub u8);
windows_core::imp::define_interface!(IPSFactoryBuffer, IPSFactoryBuffer_Vtbl, 0xd5f569d0_593b_101a_b569_08002b2dbf7a);
windows_core::imp::interface_hierarchy!(IPSFactoryBuffer, windows_core::IUnknown);
impl IPSFactoryBuffer {
    #[cfg(feature = "guiddef")]
    pub unsafe fn CreateProxy<P0>(&self, punkouter: P0, riid: *const super::IID, ppproxy: *mut Option<IRpcProxyBuffer>, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateProxy)(windows_core::Interface::as_raw(self), punkouter.param().abi(), riid, core::mem::transmute(ppproxy), ppv as _) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn CreateStub<P1>(&self, riid: *const super::IID, punkserver: P1) -> windows_core::Result<IRpcStubBuffer>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStub)(windows_core::Interface::as_raw(self), riid, punkserver.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPSFactoryBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "guiddef")]
    pub CreateProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::IID, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    CreateProxy: usize,
    #[cfg(feature = "guiddef")]
    pub CreateStub: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    CreateStub: usize,
}
#[cfg(feature = "guiddef")]
pub trait IPSFactoryBuffer_Impl: windows_core::IUnknownImpl {
    fn CreateProxy(&self, punkouter: windows_core::Ref<windows_core::IUnknown>, riid: *const super::IID, ppproxy: windows_core::OutRef<IRpcProxyBuffer>, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateStub(&self, riid: *const super::IID, punkserver: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<IRpcStubBuffer>;
}
#[cfg(feature = "guiddef")]
impl IPSFactoryBuffer_Vtbl {
    pub const fn new<Identity: IPSFactoryBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateProxy<Identity: IPSFactoryBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const super::IID, ppproxy: *mut *mut core::ffi::c_void, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPSFactoryBuffer_Impl::CreateProxy(this, core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppproxy), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn CreateStub<Identity: IPSFactoryBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID, punkserver: *mut core::ffi::c_void, ppstub: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(feature = "guiddef")]
impl windows_core::RuntimeName for IPSFactoryBuffer {}
windows_core::imp::define_interface!(IPackagedComSyntaxSupport, IPackagedComSyntaxSupport_Vtbl, 0x8f146474_b228_48fb_a58c_105ebb273abc);
windows_core::imp::interface_hierarchy!(IPackagedComSyntaxSupport, windows_core::IUnknown);
impl IPackagedComSyntaxSupport {
    #[cfg(feature = "shared")]
    pub unsafe fn GetSupportedVersion(&self) -> windows_core::Result<super::UINT32> {
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
    #[cfg(feature = "shared")]
    pub GetSupportedVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSupportedVersion: usize,
}
#[cfg(feature = "shared")]
pub trait IPackagedComSyntaxSupport_Impl: windows_core::IUnknownImpl {
    fn GetSupportedVersion(&self) -> windows_core::Result<super::UINT32>;
}
#[cfg(feature = "shared")]
impl IPackagedComSyntaxSupport_Vtbl {
    pub const fn new<Identity: IPackagedComSyntaxSupport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSupportedVersion<Identity: IPackagedComSyntaxSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedversion: *mut super::UINT32) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IPackagedComSyntaxSupport {}
windows_core::imp::define_interface!(IPipeByte, IPipeByte_Vtbl, 0xdb2f3aca_2f86_11d1_8e04_00c04fb9989a);
windows_core::imp::interface_hierarchy!(IPipeByte, windows_core::IUnknown);
impl IPipeByte {
    #[cfg(feature = "shared")]
    pub unsafe fn Pull(&self, buf: *mut super::BYTE, crequest: super::ULONG, pcreturned: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Pull)(windows_core::Interface::as_raw(self), buf as _, crequest, pcreturned as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Push(&self, buf: &[super::BYTE]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Push)(windows_core::Interface::as_raw(self), buf.as_ptr(), buf.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeByte_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BYTE, super::ULONG, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Pull: usize,
    #[cfg(feature = "shared")]
    pub Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Push: usize,
}
#[cfg(feature = "shared")]
pub trait IPipeByte_Impl: windows_core::IUnknownImpl {
    fn Pull(&self, buf: *mut super::BYTE, crequest: super::ULONG, pcreturned: *mut super::ULONG) -> windows_core::Result<()>;
    fn Push(&self, buf: *const super::BYTE, csent: super::ULONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IPipeByte_Vtbl {
    pub const fn new<Identity: IPipeByte_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Pull<Identity: IPipeByte_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut super::BYTE, crequest: super::ULONG, pcreturned: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPipeByte_Impl::Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&crequest), core::mem::transmute_copy(&pcreturned)).into()
            }
        }
        unsafe extern "system" fn Push<Identity: IPipeByte_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const super::BYTE, csent: super::ULONG) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IPipeByte {}
windows_core::imp::define_interface!(IPipeDouble, IPipeDouble_Vtbl, 0xdb2f3ace_2f86_11d1_8e04_00c04fb9989a);
windows_core::imp::interface_hierarchy!(IPipeDouble, windows_core::IUnknown);
impl IPipeDouble {
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn Pull(&self, buf: *mut super::DOUBLE, crequest: super::ULONG, pcreturned: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Pull)(windows_core::Interface::as_raw(self), buf as _, crequest, pcreturned as _) }
    }
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn Push(&self, buf: &[super::DOUBLE]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Push)(windows_core::Interface::as_raw(self), buf.as_ptr(), buf.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeDouble_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DOUBLE, super::ULONG, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypesbase")))]
    Pull: usize,
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::DOUBLE, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypesbase")))]
    Push: usize,
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub trait IPipeDouble_Impl: windows_core::IUnknownImpl {
    fn Pull(&self, buf: *mut super::DOUBLE, crequest: super::ULONG, pcreturned: *mut super::ULONG) -> windows_core::Result<()>;
    fn Push(&self, buf: *const super::DOUBLE, csent: super::ULONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
impl IPipeDouble_Vtbl {
    pub const fn new<Identity: IPipeDouble_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Pull<Identity: IPipeDouble_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut super::DOUBLE, crequest: super::ULONG, pcreturned: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPipeDouble_Impl::Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&crequest), core::mem::transmute_copy(&pcreturned)).into()
            }
        }
        unsafe extern "system" fn Push<Identity: IPipeDouble_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const super::DOUBLE, csent: super::ULONG) -> windows_core::HRESULT {
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
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPipeDouble {}
windows_core::imp::define_interface!(IPipeLong, IPipeLong_Vtbl, 0xdb2f3acc_2f86_11d1_8e04_00c04fb9989a);
windows_core::imp::interface_hierarchy!(IPipeLong, windows_core::IUnknown);
impl IPipeLong {
    #[cfg(feature = "shared")]
    pub unsafe fn Pull(&self, buf: *mut super::LONG, crequest: super::ULONG, pcreturned: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Pull)(windows_core::Interface::as_raw(self), buf as _, crequest, pcreturned as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Push(&self, buf: &[super::LONG]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Push)(windows_core::Interface::as_raw(self), buf.as_ptr(), buf.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeLong_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG, super::ULONG, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Pull: usize,
    #[cfg(feature = "shared")]
    pub Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::LONG, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Push: usize,
}
#[cfg(feature = "shared")]
pub trait IPipeLong_Impl: windows_core::IUnknownImpl {
    fn Pull(&self, buf: *mut super::LONG, crequest: super::ULONG, pcreturned: *mut super::ULONG) -> windows_core::Result<()>;
    fn Push(&self, buf: *const super::LONG, csent: super::ULONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IPipeLong_Vtbl {
    pub const fn new<Identity: IPipeLong_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Pull<Identity: IPipeLong_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *mut super::LONG, crequest: super::ULONG, pcreturned: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPipeLong_Impl::Pull(this, core::mem::transmute_copy(&buf), core::mem::transmute_copy(&crequest), core::mem::transmute_copy(&pcreturned)).into()
            }
        }
        unsafe extern "system" fn Push<Identity: IPipeLong_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buf: *const super::LONG, csent: super::ULONG) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IPipeLong {}
windows_core::imp::define_interface!(IProcessInitControl, IProcessInitControl_Vtbl, 0x72380d55_8d2b_43a3_8513_2b6ef31434e9);
windows_core::imp::interface_hierarchy!(IProcessInitControl, windows_core::IUnknown);
impl IProcessInitControl {
    #[cfg(feature = "shared")]
    pub unsafe fn ResetInitializerTimeout(&self, dwsecondsremaining: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResetInitializerTimeout)(windows_core::Interface::as_raw(self), dwsecondsremaining) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessInitControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub ResetInitializerTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ResetInitializerTimeout: usize,
}
#[cfg(feature = "shared")]
pub trait IProcessInitControl_Impl: windows_core::IUnknownImpl {
    fn ResetInitializerTimeout(&self, dwsecondsremaining: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IProcessInitControl_Vtbl {
    pub const fn new<Identity: IProcessInitControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ResetInitializerTimeout<Identity: IProcessInitControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsecondsremaining: super::DWORD) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IProcessInitControl {}
windows_core::imp::define_interface!(IReleaseMarshalBuffers, IReleaseMarshalBuffers_Vtbl, 0xeb0cb9e8_7996_11d2_872e_0000f8080859);
windows_core::imp::interface_hierarchy!(IReleaseMarshalBuffers, windows_core::IUnknown);
impl IReleaseMarshalBuffers {
    #[cfg(feature = "shared")]
    pub unsafe fn ReleaseMarshalBuffer<P2>(&self, pmsg: *mut RPCOLEMESSAGE, dwflags: super::DWORD, pchnl: P2) -> windows_core::HRESULT
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
    #[cfg(feature = "shared")]
    pub ReleaseMarshalBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, super::DWORD, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ReleaseMarshalBuffer: usize,
}
#[cfg(feature = "shared")]
pub trait IReleaseMarshalBuffers_Impl: windows_core::IUnknownImpl {
    fn ReleaseMarshalBuffer(&self, pmsg: *mut RPCOLEMESSAGE, dwflags: super::DWORD, pchnl: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IReleaseMarshalBuffers_Vtbl {
    pub const fn new<Identity: IReleaseMarshalBuffers_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReleaseMarshalBuffer<Identity: IReleaseMarshalBuffers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, dwflags: super::DWORD, pchnl: *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IReleaseMarshalBuffers {}
windows_core::imp::define_interface!(IRpcChannelBuffer, IRpcChannelBuffer_Vtbl, 0xd5f56b60_593b_101a_b569_08002b2dbf7a);
windows_core::imp::interface_hierarchy!(IRpcChannelBuffer, windows_core::IUnknown);
impl IRpcChannelBuffer {
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const super::IID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), pmessage as _, riid) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: Option<*mut super::ULONG>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SendReceive)(windows_core::Interface::as_raw(self), pmessage as _, pstatus.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreeBuffer)(windows_core::Interface::as_raw(self), pmessage as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut super::DWORD, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *const super::IID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    GetBuffer: usize,
    #[cfg(feature = "shared")]
    pub SendReceive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SendReceive: usize,
    #[cfg(feature = "shared")]
    pub FreeBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    FreeBuffer: usize,
    #[cfg(feature = "shared")]
    pub GetDestCtx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDestCtx: usize,
    pub IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IRpcChannelBuffer_Impl: windows_core::IUnknownImpl {
    fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const super::IID) -> windows_core::Result<()>;
    fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut super::ULONG) -> windows_core::Result<()>;
    fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> windows_core::Result<()>;
    fn GetDestCtx(&self, pdwdestcontext: *mut super::DWORD, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn IsConnected(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IRpcChannelBuffer_Vtbl {
    pub const fn new<Identity: IRpcChannelBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBuffer<Identity: IRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, riid: *const super::IID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer_Impl::GetBuffer(this, core::mem::transmute_copy(&pmessage), core::mem::transmute_copy(&riid)).into()
            }
        }
        unsafe extern "system" fn SendReceive<Identity: IRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut super::ULONG) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetDestCtx<Identity: IRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwdestcontext: *mut super::DWORD, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "shared"))]
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
    #[cfg(feature = "shared")]
    pub unsafe fn GetProtocolVersion(&self) -> windows_core::Result<super::DWORD> {
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
    #[cfg(feature = "shared")]
    pub GetProtocolVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetProtocolVersion: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IRpcChannelBuffer2_Impl: IRpcChannelBuffer_Impl {
    fn GetProtocolVersion(&self) -> windows_core::Result<super::DWORD>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IRpcChannelBuffer2_Vtbl {
    pub const fn new<Identity: IRpcChannelBuffer2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProtocolVersion<Identity: IRpcChannelBuffer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversion: *mut super::DWORD) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "shared"))]
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
    #[cfg(feature = "shared")]
    pub unsafe fn Send(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Send)(windows_core::Interface::as_raw(self), pmsg as _, pulstatus as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, ulsize: super::ULONG, pulstatus: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Receive)(windows_core::Interface::as_raw(self), pmsg as _, ulsize, pulstatus as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Cancel(&self, pmsg: *mut RPCOLEMESSAGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self), pmsg as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn GetCallContext(&self, pmsg: *const RPCOLEMESSAGE, riid: *const super::IID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCallContext)(windows_core::Interface::as_raw(self), pmsg, riid, pinterface as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut super::DWORD, ppvdestcontext: Option<*mut *mut core::ffi::c_void>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDestCtxEx)(windows_core::Interface::as_raw(self), pmsg, pdwdestcontext as _, ppvdestcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetState(&self, pmsg: *const RPCOLEMESSAGE) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), pmsg, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
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
    #[cfg(feature = "shared")]
    pub Send: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Send: usize,
    #[cfg(feature = "shared")]
    pub Receive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, super::ULONG, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Receive: usize,
    #[cfg(feature = "shared")]
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Cancel: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub GetCallContext: unsafe extern "system" fn(*mut core::ffi::c_void, *const RPCOLEMESSAGE, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    GetCallContext: usize,
    #[cfg(feature = "shared")]
    pub GetDestCtxEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const RPCOLEMESSAGE, *mut super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDestCtxEx: usize,
    #[cfg(feature = "shared")]
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *const RPCOLEMESSAGE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetState: usize,
    #[cfg(feature = "shared")]
    pub RegisterAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RegisterAsync: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IRpcChannelBuffer3_Impl: IRpcChannelBuffer2_Impl {
    fn Send(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut super::ULONG) -> windows_core::Result<()>;
    fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, ulsize: super::ULONG, pulstatus: *mut super::ULONG) -> windows_core::Result<()>;
    fn Cancel(&self, pmsg: *mut RPCOLEMESSAGE) -> windows_core::Result<()>;
    fn GetCallContext(&self, pmsg: *const RPCOLEMESSAGE, riid: *const super::IID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut super::DWORD, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetState(&self, pmsg: *const RPCOLEMESSAGE) -> windows_core::Result<super::DWORD>;
    fn RegisterAsync(&self, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: windows_core::Ref<IAsyncManager>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IRpcChannelBuffer3_Vtbl {
    pub const fn new<Identity: IRpcChannelBuffer3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Send<Identity: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer3_Impl::Send(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&pulstatus)).into()
            }
        }
        unsafe extern "system" fn Receive<Identity: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, ulsize: super::ULONG, pulstatus: *mut super::ULONG) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetCallContext<Identity: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, riid: *const super::IID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer3_Impl::GetCallContext(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pinterface)).into()
            }
        }
        unsafe extern "system" fn GetDestCtxEx<Identity: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut super::DWORD, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcChannelBuffer3_Impl::GetDestCtxEx(this, core::mem::transmute_copy(&pmsg), core::mem::transmute_copy(&pdwdestcontext), core::mem::transmute_copy(&ppvdestcontext)).into()
            }
        }
        unsafe extern "system" fn GetState<Identity: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pstate: *mut super::DWORD) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IRpcChannelBuffer3 {}
windows_core::imp::define_interface!(IRpcHelper, IRpcHelper_Vtbl, 0x00000149_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IRpcHelper, windows_core::IUnknown);
impl IRpcHelper {
    #[cfg(feature = "shared")]
    pub unsafe fn GetDCOMProtocolVersion(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDCOMProtocolVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn GetIIDFromOBJREF(&self, pobjref: *const core::ffi::c_void) -> windows_core::Result<*mut super::IID> {
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
    #[cfg(feature = "shared")]
    pub GetDCOMProtocolVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDCOMProtocolVersion: usize,
    #[cfg(feature = "guiddef")]
    pub GetIIDFromOBJREF: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut *mut super::IID) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    GetIIDFromOBJREF: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IRpcHelper_Impl: windows_core::IUnknownImpl {
    fn GetDCOMProtocolVersion(&self) -> windows_core::Result<super::DWORD>;
    fn GetIIDFromOBJREF(&self, pobjref: *const core::ffi::c_void) -> windows_core::Result<*mut super::IID>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IRpcHelper_Vtbl {
    pub const fn new<Identity: IRpcHelper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDCOMProtocolVersion<Identity: IRpcHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomversion: *mut super::DWORD) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetIIDFromOBJREF<Identity: IRpcHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjref: *const core::ffi::c_void, piid: *mut *mut super::IID) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IRpcHelper {}
windows_core::imp::define_interface!(IRpcOptions, IRpcOptions_Vtbl, 0x00000144_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IRpcOptions, windows_core::IUnknown);
impl IRpcOptions {
    #[cfg(feature = "shared")]
    pub unsafe fn Set<P0>(&self, pprx: P0, dwproperty: RPCOPT_PROPERTIES, dwvalue: super::ULONG_PTR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), pprx.param().abi(), dwproperty, dwvalue) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Query<P0>(&self, pprx: P0, dwproperty: RPCOPT_PROPERTIES) -> windows_core::Result<super::ULONG_PTR>
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
    #[cfg(feature = "shared")]
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, RPCOPT_PROPERTIES, super::ULONG_PTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Set: usize,
    #[cfg(feature = "shared")]
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, RPCOPT_PROPERTIES, *mut super::ULONG_PTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Query: usize,
}
#[cfg(feature = "shared")]
pub trait IRpcOptions_Impl: windows_core::IUnknownImpl {
    fn Set(&self, pprx: windows_core::Ref<windows_core::IUnknown>, dwproperty: RPCOPT_PROPERTIES, dwvalue: super::ULONG_PTR) -> windows_core::Result<()>;
    fn Query(&self, pprx: windows_core::Ref<windows_core::IUnknown>, dwproperty: RPCOPT_PROPERTIES) -> windows_core::Result<super::ULONG_PTR>;
}
#[cfg(feature = "shared")]
impl IRpcOptions_Vtbl {
    pub const fn new<Identity: IRpcOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Set<Identity: IRpcOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprx: *mut core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, dwvalue: super::ULONG_PTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcOptions_Impl::Set(this, core::mem::transmute_copy(&pprx), core::mem::transmute_copy(&dwproperty), core::mem::transmute_copy(&dwvalue)).into()
            }
        }
        unsafe extern "system" fn Query<Identity: IRpcOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprx: *mut core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, pdwvalue: *mut super::ULONG_PTR) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
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
    #[cfg(feature = "shared")]
    pub unsafe fn Invoke<P1>(&self, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IRpcChannelBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), _prpcmsg as _, _prpcchannelbuffer.param().abi()) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn IsIIDSupported(&self, riid: *const super::IID) -> Option<Self> {
        unsafe { (windows_core::Interface::vtable(self).IsIIDSupported)(windows_core::Interface::as_raw(self), riid) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CountRefs(&self) -> super::ULONG {
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
    #[cfg(feature = "shared")]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Invoke: usize,
    #[cfg(feature = "guiddef")]
    pub IsIIDSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID) -> Option<IRpcStubBuffer>,
    #[cfg(not(feature = "guiddef"))]
    IsIIDSupported: usize,
    #[cfg(feature = "shared")]
    pub CountRefs: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::ULONG,
    #[cfg(not(feature = "shared"))]
    CountRefs: usize,
    pub DebugServerQueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DebugServerRelease: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void),
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IRpcStubBuffer_Impl: windows_core::IUnknownImpl {
    fn Connect(&self, punkserver: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Disconnect(&self);
    fn Invoke(&self, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: windows_core::Ref<IRpcChannelBuffer>) -> windows_core::Result<()>;
    fn IsIIDSupported(&self, riid: *const super::IID) -> Option<IRpcStubBuffer>;
    fn CountRefs(&self) -> super::ULONG;
    fn DebugServerQueryInterface(&self, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn DebugServerRelease(&self, pv: *const core::ffi::c_void);
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
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
        unsafe extern "system" fn IsIIDSupported<Identity: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const super::IID) -> Option<IRpcStubBuffer> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRpcStubBuffer_Impl::IsIIDSupported(this, core::mem::transmute_copy(&riid))
            }
        }
        unsafe extern "system" fn CountRefs<Identity: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::ULONG {
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
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IRpcStubBuffer {}
windows_core::imp::define_interface!(IRpcSyntaxNegotiate, IRpcSyntaxNegotiate_Vtbl, 0x58a08519_24c8_4935_b482_3fd823333a4f);
windows_core::imp::interface_hierarchy!(IRpcSyntaxNegotiate, windows_core::IUnknown);
impl IRpcSyntaxNegotiate {
    #[cfg(feature = "shared")]
    pub unsafe fn NegotiateSyntax(&self, pmsg: *mut RPCOLEMESSAGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NegotiateSyntax)(windows_core::Interface::as_raw(self), pmsg as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcSyntaxNegotiate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub NegotiateSyntax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    NegotiateSyntax: usize,
}
#[cfg(feature = "shared")]
pub trait IRpcSyntaxNegotiate_Impl: windows_core::IUnknownImpl {
    fn NegotiateSyntax(&self, pmsg: *mut RPCOLEMESSAGE) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IRpcSyntaxNegotiate {}
windows_core::imp::define_interface!(ISequentialStream, ISequentialStream_Vtbl, 0x0c733a30_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ISequentialStream, windows_core::IUnknown);
impl ISequentialStream {
    #[cfg(feature = "shared")]
    pub unsafe fn Read(&self, pv: *mut core::ffi::c_void, cb: super::ULONG, pcbread: Option<*mut super::ULONG>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pv as _, cb, pcbread.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Write(&self, pv: *const core::ffi::c_void, cb: super::ULONG, pcbwritten: Option<*mut super::ULONG>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), pv, cb, pcbwritten.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISequentialStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::ULONG, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Read: usize,
    #[cfg(feature = "shared")]
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, super::ULONG, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Write: usize,
}
#[cfg(feature = "shared")]
pub trait ISequentialStream_Impl: windows_core::IUnknownImpl {
    fn Read(&self, pv: *mut core::ffi::c_void, cb: super::ULONG, pcbread: *mut super::ULONG) -> windows_core::Result<()>;
    fn Write(&self, pv: *const core::ffi::c_void, cb: super::ULONG, pcbwritten: *mut super::ULONG) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl ISequentialStream_Vtbl {
    pub const fn new<Identity: ISequentialStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: ISequentialStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *mut core::ffi::c_void, cb: super::ULONG, pcbread: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISequentialStream_Impl::Read(this, core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbread)).into()
            }
        }
        unsafe extern "system" fn Write<Identity: ISequentialStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void, cb: super::ULONG, pcbwritten: *mut super::ULONG) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for ISequentialStream {}
windows_core::imp::define_interface!(IServerSecurity, IServerSecurity_Vtbl, 0x0000013e_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IServerSecurity, windows_core::IUnknown);
impl IServerSecurity {
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn QueryBlanket(&self, pauthnsvc: Option<*mut super::DWORD>, pauthzsvc: Option<*mut super::DWORD>, pserverprincname: *mut *mut super::OLECHAR, pauthnlevel: Option<*mut super::DWORD>, pimplevel: Option<*mut super::DWORD>, pprivs: *mut *mut core::ffi::c_void, pcapabilities: Option<*mut super::DWORD>) -> windows_core::HRESULT {
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
    #[cfg(all(feature = "shared", feature = "wtypesbase"))]
    pub QueryBlanket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD, *mut super::DWORD, *mut *mut super::OLECHAR, *mut super::DWORD, *mut super::DWORD, *mut *mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypesbase")))]
    QueryBlanket: usize,
    pub ImpersonateClient: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RevertToSelf: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsImpersonating: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub trait IServerSecurity_Impl: windows_core::IUnknownImpl {
    fn QueryBlanket(&self, pauthnsvc: *mut super::DWORD, pauthzsvc: *mut super::DWORD, pserverprincname: *mut *mut super::OLECHAR, pauthnlevel: *mut super::DWORD, pimplevel: *mut super::DWORD, pprivs: *mut *mut core::ffi::c_void, pcapabilities: *mut super::DWORD) -> windows_core::Result<()>;
    fn ImpersonateClient(&self) -> windows_core::Result<()>;
    fn RevertToSelf(&self) -> windows_core::Result<()>;
    fn IsImpersonating(&self) -> windows_core::BOOL;
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
impl IServerSecurity_Vtbl {
    pub const fn new<Identity: IServerSecurity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryBlanket<Identity: IServerSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauthnsvc: *mut super::DWORD, pauthzsvc: *mut super::DWORD, pserverprincname: *mut *mut super::OLECHAR, pauthnlevel: *mut super::DWORD, pimplevel: *mut super::DWORD, pprivs: *mut *mut core::ffi::c_void, pcapabilities: *mut super::DWORD) -> windows_core::HRESULT {
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
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IServerSecurity {}
windows_core::imp::define_interface!(IStdMarshalInfo, IStdMarshalInfo_Vtbl, 0x00000018_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IStdMarshalInfo, windows_core::IUnknown);
impl IStdMarshalInfo {
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn GetClassForHandler(&self, dwdestcontext: super::DWORD, pvdestcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<super::CLSID> {
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
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub GetClassForHandler: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const core::ffi::c_void, *mut super::CLSID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    GetClassForHandler: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IStdMarshalInfo_Impl: windows_core::IUnknownImpl {
    fn GetClassForHandler(&self, dwdestcontext: super::DWORD, pvdestcontext: *const core::ffi::c_void) -> windows_core::Result<super::CLSID>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IStdMarshalInfo_Vtbl {
    pub const fn new<Identity: IStdMarshalInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClassForHandler<Identity: IStdMarshalInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdestcontext: super::DWORD, pvdestcontext: *const core::ffi::c_void, pclsid: *mut super::CLSID) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "shared"))]
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
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn Seek(&self, dlibmove: super::LARGE_INTEGER, dworigin: super::DWORD, plibnewposition: Option<*mut super::ULARGE_INTEGER>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Seek)(windows_core::Interface::as_raw(self), dlibmove, dworigin, plibnewposition.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SetSize(&self, libnewsize: super::ULARGE_INTEGER) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), libnewsize) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn CopyTo<P0>(&self, pstm: P0, cb: super::ULARGE_INTEGER, pcbread: Option<*mut super::ULARGE_INTEGER>, pcbwritten: Option<*mut super::ULARGE_INTEGER>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyTo)(windows_core::Interface::as_raw(self), pstm.param().abi(), cb, pcbread.unwrap_or(core::mem::zeroed()) as _, pcbwritten.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Commit(&self, grfcommitflags: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), grfcommitflags) }
    }
    pub unsafe fn Revert(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Revert)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn LockRegion(&self, liboffset: super::ULARGE_INTEGER, cb: super::ULARGE_INTEGER, dwlocktype: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockRegion)(windows_core::Interface::as_raw(self), liboffset, cb, dwlocktype) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn UnlockRegion(&self, liboffset: super::ULARGE_INTEGER, cb: super::ULARGE_INTEGER, dwlocktype: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockRegion)(windows_core::Interface::as_raw(self), liboffset, cb, dwlocktype) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
    pub unsafe fn Stat(&self, pstatstg: *mut STATSTG, grfstatflag: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stat)(windows_core::Interface::as_raw(self), pstatstg as _, grfstatflag) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStream_Vtbl {
    pub base__: ISequentialStream_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, super::LARGE_INTEGER, super::DWORD, *mut super::ULARGE_INTEGER) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    Seek: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULARGE_INTEGER) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SetSize: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub CopyTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::ULARGE_INTEGER, *mut super::ULARGE_INTEGER, *mut super::ULARGE_INTEGER) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    CopyTo: usize,
    #[cfg(feature = "shared")]
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Commit: usize,
    pub Revert: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub LockRegion: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULARGE_INTEGER, super::ULARGE_INTEGER, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    LockRegion: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub UnlockRegion: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULARGE_INTEGER, super::ULARGE_INTEGER, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    UnlockRegion: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
    pub Stat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STATSTG, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypesbase")))]
    Stat: usize,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
pub trait IStream_Impl: ISequentialStream_Impl {
    fn Seek(&self, dlibmove: &super::LARGE_INTEGER, dworigin: super::DWORD, plibnewposition: *mut super::ULARGE_INTEGER) -> windows_core::Result<()>;
    fn SetSize(&self, libnewsize: &super::ULARGE_INTEGER) -> windows_core::Result<()>;
    fn CopyTo(&self, pstm: windows_core::Ref<IStream>, cb: &super::ULARGE_INTEGER, pcbread: *mut super::ULARGE_INTEGER, pcbwritten: *mut super::ULARGE_INTEGER) -> windows_core::Result<()>;
    fn Commit(&self, grfcommitflags: super::DWORD) -> windows_core::Result<()>;
    fn Revert(&self) -> windows_core::Result<()>;
    fn LockRegion(&self, liboffset: &super::ULARGE_INTEGER, cb: &super::ULARGE_INTEGER, dwlocktype: super::DWORD) -> windows_core::Result<()>;
    fn UnlockRegion(&self, liboffset: &super::ULARGE_INTEGER, cb: &super::ULARGE_INTEGER, dwlocktype: super::DWORD) -> windows_core::Result<()>;
    fn Stat(&self, pstatstg: *mut STATSTG, grfstatflag: super::DWORD) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IStream>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
impl IStream_Vtbl {
    pub const fn new<Identity: IStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Seek<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dlibmove: super::LARGE_INTEGER, dworigin: super::DWORD, plibnewposition: *mut super::ULARGE_INTEGER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::Seek(this, core::mem::transmute(&dlibmove), core::mem::transmute_copy(&dworigin), core::mem::transmute_copy(&plibnewposition)).into()
            }
        }
        unsafe extern "system" fn SetSize<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, libnewsize: super::ULARGE_INTEGER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::SetSize(this, core::mem::transmute(&libnewsize)).into()
            }
        }
        unsafe extern "system" fn CopyTo<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void, cb: super::ULARGE_INTEGER, pcbread: *mut super::ULARGE_INTEGER, pcbwritten: *mut super::ULARGE_INTEGER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::CopyTo(this, core::mem::transmute_copy(&pstm), core::mem::transmute(&cb), core::mem::transmute_copy(&pcbread), core::mem::transmute_copy(&pcbwritten)).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfcommitflags: super::DWORD) -> windows_core::HRESULT {
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
        unsafe extern "system" fn LockRegion<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, liboffset: super::ULARGE_INTEGER, cb: super::ULARGE_INTEGER, dwlocktype: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::LockRegion(this, core::mem::transmute(&liboffset), core::mem::transmute(&cb), core::mem::transmute_copy(&dwlocktype)).into()
            }
        }
        unsafe extern "system" fn UnlockRegion<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, liboffset: super::ULARGE_INTEGER, cb: super::ULARGE_INTEGER, dwlocktype: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStream_Impl::UnlockRegion(this, core::mem::transmute(&liboffset), core::mem::transmute(&cb), core::mem::transmute_copy(&dwlocktype)).into()
            }
        }
        unsafe extern "system" fn Stat<Identity: IStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatstg: *mut STATSTG, grfstatflag: super::DWORD) -> windows_core::HRESULT {
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
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
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
    #[cfg(feature = "guiddef")]
    pub unsafe fn LoadDllServer(&self, clsid: *const super::IID) -> windows_core::HRESULT {
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
    #[cfg(feature = "guiddef")]
    pub LoadDllServer: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::IID) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    LoadDllServer: usize,
    pub FreeSurrogate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "guiddef")]
pub trait ISurrogate_Impl: windows_core::IUnknownImpl {
    fn LoadDllServer(&self, clsid: *const super::IID) -> windows_core::Result<()>;
    fn FreeSurrogate(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "guiddef")]
impl ISurrogate_Vtbl {
    pub const fn new<Identity: ISurrogate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LoadDllServer<Identity: ISurrogate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const super::IID) -> windows_core::HRESULT {
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
#[cfg(feature = "guiddef")]
impl windows_core::RuntimeName for ISurrogate {}
windows_core::imp::define_interface!(ISynchronize, ISynchronize_Vtbl, 0x00000030_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ISynchronize, windows_core::IUnknown);
impl ISynchronize {
    #[cfg(feature = "shared")]
    pub unsafe fn Wait(&self, dwflags: super::DWORD, dwmilliseconds: super::DWORD) -> windows_core::HRESULT {
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
    #[cfg(feature = "shared")]
    pub Wait: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Wait: usize,
    pub Signal: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait ISynchronize_Impl: windows_core::IUnknownImpl {
    fn Wait(&self, dwflags: super::DWORD, dwmilliseconds: super::DWORD) -> windows_core::Result<()>;
    fn Signal(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl ISynchronize_Vtbl {
    pub const fn new<Identity: ISynchronize_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Wait<Identity: ISynchronize_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, dwmilliseconds: super::DWORD) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
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
    #[cfg(feature = "shared")]
    pub unsafe fn WaitMultiple(&self, dwflags: super::DWORD, dwtimeout: super::DWORD) -> windows_core::Result<ISynchronize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WaitMultiple)(windows_core::Interface::as_raw(self), dwflags, dwtimeout, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeContainer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddSynchronize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub WaitMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    WaitMultiple: usize,
}
#[cfg(feature = "shared")]
pub trait ISynchronizeContainer_Impl: windows_core::IUnknownImpl {
    fn AddSynchronize(&self, psync: windows_core::Ref<ISynchronize>) -> windows_core::Result<()>;
    fn WaitMultiple(&self, dwflags: super::DWORD, dwtimeout: super::DWORD) -> windows_core::Result<ISynchronize>;
}
#[cfg(feature = "shared")]
impl ISynchronizeContainer_Vtbl {
    pub const fn new<Identity: ISynchronizeContainer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddSynchronize<Identity: ISynchronizeContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psync: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISynchronizeContainer_Impl::AddSynchronize(this, core::mem::transmute_copy(&psync)).into()
            }
        }
        unsafe extern "system" fn WaitMultiple<Identity: ISynchronizeContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, dwtimeout: super::DWORD, ppsync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
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
    pub unsafe fn SetEventHandle(&self, ph: *const super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEventHandle)(windows_core::Interface::as_raw(self), ph) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeEvent_Vtbl {
    pub base__: ISynchronizeHandle_Vtbl,
    #[cfg(feature = "winnt")]
    pub SetEventHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SetEventHandle: usize,
}
#[cfg(feature = "winnt")]
pub trait ISynchronizeEvent_Impl: ISynchronizeHandle_Impl {
    fn SetEventHandle(&self, ph: *const super::HANDLE) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl ISynchronizeEvent_Vtbl {
    pub const fn new<Identity: ISynchronizeEvent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetEventHandle<Identity: ISynchronizeEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ph: *const super::HANDLE) -> windows_core::HRESULT {
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
    pub unsafe fn GetHandle(&self, ph: *mut super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetHandle)(windows_core::Interface::as_raw(self), ph as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeHandle_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub GetHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetHandle: usize,
}
#[cfg(feature = "winnt")]
pub trait ISynchronizeHandle_Impl: windows_core::IUnknownImpl {
    fn GetHandle(&self, ph: *mut super::HANDLE) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl ISynchronizeHandle_Vtbl {
    pub const fn new<Identity: ISynchronizeHandle_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetHandle<Identity: ISynchronizeHandle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ph: *mut super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISynchronizeHandle_Impl::GetHandle(this, core::mem::transmute_copy(&ph)).into()
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
#[cfg(feature = "shared")]
pub trait ISynchronizeMutex_Impl: ISynchronize_Impl {
    fn ReleaseMutex(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for ISynchronizeMutex {}
windows_core::imp::define_interface!(IWaitMultiple, IWaitMultiple_Vtbl, 0x0000002b_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IWaitMultiple, windows_core::IUnknown);
impl IWaitMultiple {
    #[cfg(feature = "shared")]
    pub unsafe fn WaitMultiple(&self, timeout: super::DWORD) -> windows_core::Result<ISynchronize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WaitMultiple)(windows_core::Interface::as_raw(self), timeout, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
    #[cfg(feature = "shared")]
    pub WaitMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    WaitMultiple: usize,
    pub AddSynchronize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IWaitMultiple_Impl: windows_core::IUnknownImpl {
    fn WaitMultiple(&self, timeout: super::DWORD) -> windows_core::Result<ISynchronize>;
    fn AddSynchronize(&self, psync: windows_core::Ref<ISynchronize>) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWaitMultiple_Vtbl {
    pub const fn new<Identity: IWaitMultiple_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WaitMultiple<Identity: IWaitMultiple_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeout: super::DWORD, psync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWaitMultiple {}
pub type LOCKTYPE = i32;
pub const LOCK_EXCLUSIVE: LOCKTYPE = 2;
pub const LOCK_ONLYONCE: LOCKTYPE = 4;
pub const LOCK_WRITE: LOCKTYPE = 1;
pub type LPADDREXCLUSIONCONTROL = IAddrExclusionControl;
pub type LPADDRTRACKINGCONTROL = IAddrTrackingControl;
pub type LPCANCELMETHODCALLS = ICancelMethodCalls;
pub type LPENUMSTRING = IEnumString;
pub type LPENUMUNKNOWN = IEnumUnknown;
pub type LPEXTERNALCONNECTION = IExternalConnection;
pub type LPGLOBALINTERFACETABLE = IGlobalInterfaceTable;
pub type LPMALLOC = IMalloc;
pub type LPMARSHAL = IMarshal;
pub type LPMARSHAL2 = IMarshal2;
pub type LPMULTIQI = IMultiQI;
pub type LPSTDMARSHALINFO = IStdMarshalInfo;
pub type LPSTREAM = IStream;
pub type LPSURROGATE = ISurrogate;
#[repr(C, packed(8))]
#[cfg(feature = "guiddef")]
#[derive(Default)]
pub struct MULTI_QI {
    pub pIID: *const super::IID,
    pub pItf: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub hr: windows_core::HRESULT,
}
pub type MachineGlobalObjectTableRegistrationToken = *mut MachineGlobalObjectTableRegistrationToken__;
#[repr(C, packed(8))]
#[derive(Clone, Copy, Default)]
pub struct MachineGlobalObjectTableRegistrationToken__ {
    pub unused: i32,
}
#[cfg(feature = "shared")]
pub type PRPCOLEMESSAGE = *mut RPCOLEMESSAGE;
#[cfg(feature = "shared")]
pub type PSOLE_AUTHENTICATION_INFO = *mut SOLE_AUTHENTICATION_INFO;
#[cfg(feature = "shared")]
pub type PSOLE_AUTHENTICATION_LIST = *mut SOLE_AUTHENTICATION_LIST;
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub type PSOLE_AUTHENTICATION_SERVICE = *mut SOLE_AUTHENTICATION_SERVICE;
pub type RPCOLEDATAREP = u32;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct RPCOLEMESSAGE {
    pub reserved1: *mut core::ffi::c_void,
    pub dataRepresentation: RPCOLEDATAREP,
    pub Buffer: *mut core::ffi::c_void,
    pub cbBuffer: super::ULONG,
    pub iMethod: super::ULONG,
    pub reserved2: [*mut core::ffi::c_void; 5],
    pub rpcFlags: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for RPCOLEMESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RPCOPT_PROPERTIES = i32;
pub type RPCOPT_SERVER_LOCALITY_VALUES = i32;
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct SChannelHookCallInfo {
    pub iid: super::IID,
    pub cbSize: super::DWORD,
    pub uCausality: windows_core::GUID,
    pub dwServerPid: super::DWORD,
    pub iMethod: super::DWORD,
    pub pObject: *mut core::ffi::c_void,
}
pub const SERVER_LOCALITY_MACHINE_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = 1;
pub const SERVER_LOCALITY_PROCESS_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = 0;
pub const SERVER_LOCALITY_REMOTE: RPCOPT_SERVER_LOCALITY_VALUES = 2;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SOLE_AUTHENTICATION_INFO {
    pub dwAuthnSvc: super::DWORD,
    pub dwAuthzSvc: super::DWORD,
    pub pAuthInfo: *mut core::ffi::c_void,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SOLE_AUTHENTICATION_LIST {
    pub cAuthInfo: super::DWORD,
    pub aAuthInfo: *mut SOLE_AUTHENTICATION_INFO,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct SOLE_AUTHENTICATION_SERVICE {
    pub dwAuthnSvc: super::DWORD,
    pub dwAuthzSvc: super::DWORD,
    pub pPrincipalName: *mut super::OLECHAR,
    pub hr: windows_core::HRESULT,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct STATSTG {
    pub pwcsName: super::LPOLESTR,
    pub r#type: super::DWORD,
    pub cbSize: super::ULARGE_INTEGER,
    pub mtime: super::FILETIME,
    pub ctime: super::FILETIME,
    pub atime: super::FILETIME,
    pub grfMode: super::DWORD,
    pub grfLocksSupported: super::DWORD,
    pub clsid: super::CLSID,
    pub grfStateBits: super::DWORD,
    pub reserved: super::DWORD,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
impl Default for STATSTG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
