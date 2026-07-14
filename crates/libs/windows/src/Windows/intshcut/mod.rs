#[cfg(feature = "windef")]
pub type CURLINVOKECOMMANDINFOA = URLINVOKECOMMANDINFOA;
#[cfg(feature = "windef")]
pub type CURLINVOKECOMMANDINFOW = URLINVOKECOMMANDINFOW;
pub const E_FLAGS: i32 = -2147217408;
pub const IS_E_EXEC_FAILED: i32 = -2147213310;
pub type IURL_INVOKECOMMAND_FLAGS = i32;
pub const IURL_INVOKECOMMAND_FL_ALLOW_UI: IURL_INVOKECOMMAND_FLAGS = 1;
pub const IURL_INVOKECOMMAND_FL_ASYNCOK: IURL_INVOKECOMMAND_FLAGS = 8;
pub const IURL_INVOKECOMMAND_FL_DDEWAIT: IURL_INVOKECOMMAND_FLAGS = 4;
pub const IURL_INVOKECOMMAND_FL_LOG_USAGE: IURL_INVOKECOMMAND_FLAGS = 16;
pub const IURL_INVOKECOMMAND_FL_USE_DEFAULT_VERB: IURL_INVOKECOMMAND_FLAGS = 2;
pub type IURL_SETURL_FLAGS = i32;
pub const IURL_SETURL_FL_GUESS_PROTOCOL: IURL_SETURL_FLAGS = 1;
pub const IURL_SETURL_FL_USE_DEFAULT_PROTOCOL: IURL_SETURL_FLAGS = 2;
windows_core::imp::define_interface!(IUniformResourceLocatorA, IUniformResourceLocatorA_Vtbl, 0xfbf23b80_e3f0_101b_8488_00aa003e56f8);
windows_core::imp::interface_hierarchy!(IUniformResourceLocatorA, windows_core::IUnknown);
impl IUniformResourceLocatorA {
    pub unsafe fn SetURL<P0>(&self, pcszurl: P0, dwinflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetURL)(windows_core::Interface::as_raw(self), pcszurl.param().abi(), dwinflags) }
    }
    pub unsafe fn GetURL(&self) -> windows_core::Result<windows_core::PSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn InvokeCommand(&self) -> windows_core::Result<URLINVOKECOMMANDINFOA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InvokeCommand)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUniformResourceLocatorA_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetURL: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32) -> windows_core::HRESULT,
    pub GetURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PSTR) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub InvokeCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut URLINVOKECOMMANDINFOA) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    InvokeCommand: usize,
}
#[cfg(feature = "windef")]
pub trait IUniformResourceLocatorA_Impl: windows_core::IUnknownImpl {
    fn SetURL(&self, pcszurl: &windows_core::PCSTR, dwinflags: u32) -> windows_core::Result<()>;
    fn GetURL(&self) -> windows_core::Result<windows_core::PSTR>;
    fn InvokeCommand(&self) -> windows_core::Result<URLINVOKECOMMANDINFOA>;
}
#[cfg(feature = "windef")]
impl IUniformResourceLocatorA_Vtbl {
    pub const fn new<Identity: IUniformResourceLocatorA_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetURL<Identity: IUniformResourceLocatorA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcszurl: windows_core::PCSTR, dwinflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUniformResourceLocatorA_Impl::SetURL(this, core::mem::transmute(&pcszurl), core::mem::transmute_copy(&dwinflags)).into()
            }
        }
        unsafe extern "system" fn GetURL<Identity: IUniformResourceLocatorA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszurl: *mut windows_core::PSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUniformResourceLocatorA_Impl::GetURL(this) {
                    Ok(ok__) => {
                        ppszurl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InvokeCommand<Identity: IUniformResourceLocatorA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, purlici: *mut URLINVOKECOMMANDINFOA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUniformResourceLocatorA_Impl::InvokeCommand(this) {
                    Ok(ok__) => {
                        purlici.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetURL: SetURL::<Identity, OFFSET>,
            GetURL: GetURL::<Identity, OFFSET>,
            InvokeCommand: InvokeCommand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUniformResourceLocatorA as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IUniformResourceLocatorA {}
windows_core::imp::define_interface!(IUniformResourceLocatorW, IUniformResourceLocatorW_Vtbl, 0xcabb0da0_da57_11cf_9974_0020afd79762);
windows_core::imp::interface_hierarchy!(IUniformResourceLocatorW, windows_core::IUnknown);
impl IUniformResourceLocatorW {
    pub unsafe fn SetURL<P0>(&self, pcszurl: P0, dwinflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetURL)(windows_core::Interface::as_raw(self), pcszurl.param().abi(), dwinflags) }
    }
    pub unsafe fn GetURL(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn InvokeCommand(&self) -> windows_core::Result<URLINVOKECOMMANDINFOW> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InvokeCommand)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUniformResourceLocatorW_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetURL: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub InvokeCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut URLINVOKECOMMANDINFOW) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    InvokeCommand: usize,
}
#[cfg(feature = "windef")]
pub trait IUniformResourceLocatorW_Impl: windows_core::IUnknownImpl {
    fn SetURL(&self, pcszurl: &windows_core::PCWSTR, dwinflags: u32) -> windows_core::Result<()>;
    fn GetURL(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn InvokeCommand(&self) -> windows_core::Result<URLINVOKECOMMANDINFOW>;
}
#[cfg(feature = "windef")]
impl IUniformResourceLocatorW_Vtbl {
    pub const fn new<Identity: IUniformResourceLocatorW_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetURL<Identity: IUniformResourceLocatorW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcszurl: windows_core::PCWSTR, dwinflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUniformResourceLocatorW_Impl::SetURL(this, core::mem::transmute(&pcszurl), core::mem::transmute_copy(&dwinflags)).into()
            }
        }
        unsafe extern "system" fn GetURL<Identity: IUniformResourceLocatorW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszurl: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUniformResourceLocatorW_Impl::GetURL(this) {
                    Ok(ok__) => {
                        ppszurl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InvokeCommand<Identity: IUniformResourceLocatorW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, purlici: *mut URLINVOKECOMMANDINFOW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUniformResourceLocatorW_Impl::InvokeCommand(this) {
                    Ok(ok__) => {
                        purlici.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetURL: SetURL::<Identity, OFFSET>,
            GetURL: GetURL::<Identity, OFFSET>,
            InvokeCommand: InvokeCommand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUniformResourceLocatorW as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IUniformResourceLocatorW {}
pub const MIMEASSOCDLG_FL_REGISTER_ASSOC: MIMEASSOCIATIONDIALOG_IN_FLAGS = 1;
pub type MIMEASSOCIATIONDIALOG_IN_FLAGS = i32;
#[cfg(feature = "windef")]
pub type PCURLINVOKECOMMANDINFOA = *const URLINVOKECOMMANDINFOA;
#[cfg(feature = "windef")]
pub type PCURLINVOKECOMMANDINFOW = *const URLINVOKECOMMANDINFOW;
#[cfg(feature = "windef")]
pub type PURLINVOKECOMMANDINFOA = *mut URLINVOKECOMMANDINFOA;
#[cfg(feature = "windef")]
pub type PURLINVOKECOMMANDINFOW = *mut URLINVOKECOMMANDINFOW;
pub const TRANSLATEURL_FL_GUESS_PROTOCOL: TRANSLATEURL_IN_FLAGS = 1;
pub const TRANSLATEURL_FL_USE_DEFAULT_PROTOCOL: TRANSLATEURL_IN_FLAGS = 2;
pub type TRANSLATEURL_IN_FLAGS = i32;
pub const URLASSOCDLG_FL_REGISTER_ASSOC: URLASSOCIATIONDIALOG_IN_FLAGS = 2;
pub const URLASSOCDLG_FL_USE_DEFAULT_NAME: URLASSOCIATIONDIALOG_IN_FLAGS = 1;
pub type URLASSOCIATIONDIALOG_IN_FLAGS = i32;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct URLINVOKECOMMANDINFOA {
    pub dwcbSize: u32,
    pub dwFlags: u32,
    pub hwndParent: super::windef::HWND,
    pub pcszVerb: windows_core::PCSTR,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct URLINVOKECOMMANDINFOW {
    pub dwcbSize: u32,
    pub dwFlags: u32,
    pub hwndParent: super::windef::HWND,
    pub pcszVerb: windows_core::PCWSTR,
}
pub const URL_E_INVALID_SYNTAX: i32 = -2147217407;
pub const URL_E_UNREGISTERED_PROTOCOL: i32 = -2147217406;
