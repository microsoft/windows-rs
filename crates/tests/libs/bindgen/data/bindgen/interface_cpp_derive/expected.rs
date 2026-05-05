#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(
    IPersist,
    IPersist_Vtbl,
    0x0000010c_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(IPersist, windows_core::IUnknown);
impl IPersist {
    pub unsafe fn GetClassID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClassID)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersist_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClassID: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::GUID,
    ) -> windows_core::HRESULT,
}
pub trait IPersist_Impl: windows_core::IUnknownImpl {
    fn GetClassID(&self) -> windows_core::Result<windows_core::GUID>;
}
impl IPersist_Vtbl {
    pub const fn new<Identity: IPersist_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClassID<Identity: IPersist_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pclassid: *mut windows_core::GUID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPersist_Impl::GetClassID(this) {
                    Ok(ok__) => {
                        pclassid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClassID: GetClassID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersist as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPersist {}
windows_core::imp::define_interface!(
    IPersistFile,
    IPersistFile_Vtbl,
    0x0000010b_0000_0000_c000_000000000046
);
impl core::ops::Deref for IPersistFile {
    type Target = IPersist;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPersistFile, windows_core::IUnknown, IPersist);
impl IPersistFile {
    pub unsafe fn IsDirty(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn Save<P0>(&self, pszfilename: P0, fremember: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Save)(
                windows_core::Interface::as_raw(self),
                pszfilename.param().abi(),
                fremember.into(),
            )
            .ok()
        }
    }
    pub unsafe fn SaveCompleted<P0>(&self, pszfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SaveCompleted)(
                windows_core::Interface::as_raw(self),
                pszfilename.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn GetCurFile(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurFile)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistFile_Vtbl {
    pub base__: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    Load: usize,
    pub Save: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SaveCompleted: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub GetCurFile: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::PWSTR,
    ) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IPersistFile {}
