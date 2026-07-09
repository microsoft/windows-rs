pub const EXITEVENT_CERTDENIED: u32 = 4;
pub const EXITEVENT_CERTIMPORTED: u32 = 512;
pub const EXITEVENT_CERTISSUED: u32 = 1;
pub const EXITEVENT_CERTPENDING: u32 = 2;
pub const EXITEVENT_CERTRETRIEVEPENDING: u32 = 16;
pub const EXITEVENT_CERTREVOKED: u32 = 8;
pub const EXITEVENT_CRLISSUED: u32 = 32;
pub const EXITEVENT_INVALID: u32 = 0;
pub const EXITEVENT_SHUTDOWN: u32 = 64;
pub const EXITEVENT_STARTUP: u32 = 128;
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ICertExit, ICertExit_Vtbl, 0xe19ae1a0_7364_11d0_8816_00a0c903b83c);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ICertExit {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ICertExit, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl ICertExit {
    pub unsafe fn Initialize(&self, strconfig: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strconfig), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Notify(&self, exitevent: i32, context: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), exitevent, context) }
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertExit_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ICertExit_Impl: super::oaidl::IDispatch_Impl {
    fn Initialize(&self, strconfig: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn Notify(&self, exitevent: i32, context: i32) -> windows_core::Result<()>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ICertExit_Vtbl {
    pub const fn new<Identity: ICertExit_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: ICertExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: *mut core::ffi::c_void, peventmask: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertExit_Impl::Initialize(this, core::mem::transmute(&strconfig)) {
                    Ok(ok__) => {
                        peventmask.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Notify<Identity: ICertExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, exitevent: i32, context: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICertExit_Impl::Notify(this, core::mem::transmute_copy(&exitevent), core::mem::transmute_copy(&context)).into()
            }
        }
        unsafe extern "system" fn GetDescription<Identity: ICertExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertExit_Impl::GetDescription(this) {
                    Ok(ok__) => {
                        pstrdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Notify: Notify::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertExit as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ICertExit {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(ICertExit2, ICertExit2_Vtbl, 0x0abf484b_d049_464d_a7ed_552e7529b0ff);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for ICertExit2 {
    type Target = ICertExit;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(ICertExit2, windows_core::IUnknown, super::oaidl::IDispatch, ICertExit);
#[cfg(feature = "Win32_oaidl")]
impl ICertExit2 {
    #[cfg(feature = "Win32_certmod")]
    pub unsafe fn GetManageModule(&self) -> windows_core::Result<super::certmod::ICertManageModule> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetManageModule)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ICertExit2_Vtbl {
    pub base__: ICertExit_Vtbl,
    #[cfg(feature = "Win32_certmod")]
    pub GetManageModule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_certmod"))]
    GetManageModule: usize,
}
#[cfg(all(feature = "Win32_certmod", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ICertExit2_Impl: ICertExit_Impl {
    fn GetManageModule(&self) -> windows_core::Result<super::certmod::ICertManageModule>;
}
#[cfg(all(feature = "Win32_certmod", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ICertExit2_Vtbl {
    pub const fn new<Identity: ICertExit2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetManageModule<Identity: ICertExit2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmanagemodule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertExit2_Impl::GetManageModule(this) {
                    Ok(ok__) => {
                        ppmanagemodule.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ICertExit_Vtbl::new::<Identity, OFFSET>(), GetManageModule: GetManageModule::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertExit2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<ICertExit as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_certmod", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ICertExit2 {}
